use std::collections::HashMap;
use crate::parser::{Expressao, Operador};

/// Ambiente de execução com variáveis e funções
pub struct Ambiente {
    variaveis: HashMap<String, f64>,
}

impl Ambiente {
    pub fn new() -> Self {
        let mut variaveis = HashMap::new();
        // Constantes pré-definidas
        variaveis.insert("pi".to_string(), std::f64::consts::PI);
        variaveis.insert("e".to_string(), std::f64::consts::E);

        Self { variaveis }
    }

    /// Avalia uma expressão e retorna o resultado
    pub fn avaliar(&mut self, expr: &Expressao) -> Result<f64, String> {
        match expr {
            Expressao::Numero(valor) => Ok(*valor),

            Expressao::Variavel(nome) => {
                self.variaveis
                    .get(nome)
                    .copied()
                    .ok_or_else(|| format!("Variável não definida: '{}'", nome))
            }

            Expressao::Negacao(operando) => {
                let valor = self.avaliar(operando)?;
                Ok(-valor)
            }

            Expressao::Operacao {
                esquerda,
                operador,
                direita,
            } => {
                let val_esq = self.avaliar(esquerda)?;
                let val_dir = self.avaliar(direita)?;

                match operador {
                    Operador::Somar => Ok(val_esq + val_dir),
                    Operador::Subtrair => Ok(val_esq - val_dir),
                    Operador::Multiplicar => Ok(val_esq * val_dir),
                    Operador::Dividir => {
                        if val_dir == 0.0 {
                            Err("Erro: divisão por zero".to_string())
                        } else {
                            Ok(val_esq / val_dir)
                        }
                    }
                }
            }

            Expressao::Atribuicao { nome, valor } => {
                let resultado = self.avaliar(valor)?;
                self.variaveis.insert(nome.clone(), resultado);
                Ok(resultado)
            }

            Expressao::ChamadaFuncao { nome, argumentos } => {
                self.chamar_funcao(nome, argumentos)
            }
        }
    }

    fn chamar_funcao(
        &mut self,
        nome: &str,
        argumentos: &[Expressao],
    ) -> Result<f64, String> {
        // Funções de um argumento
        if argumentos.len() == 1 {
            let arg = self.avaliar(&argumentos[0])?;
            return match nome {
                "sin" | "sen" => Ok(arg.sin()),
                "cos" => Ok(arg.cos()),
                "tan" => Ok(arg.tan()),
                "sqrt" | "raiz" => {
                    if arg < 0.0 {
                        Err("Erro: raiz quadrada de número negativo".to_string())
                    } else {
                        Ok(arg.sqrt())
                    }
                }
                "abs" => Ok(arg.abs()),
                "ln" => {
                    if arg <= 0.0 {
                        Err("Erro: ln de valor não positivo".to_string())
                    } else {
                        Ok(arg.ln())
                    }
                }
                "log" => {
                    if arg <= 0.0 {
                        Err("Erro: log de valor não positivo".to_string())
                    } else {
                        Ok(arg.log10())
                    }
                }
                "ceil" => Ok(arg.ceil()),
                "floor" => Ok(arg.floor()),
                "round" => Ok(arg.round()),
                _ => Err(format!("Função desconhecida: '{}'", nome)),
            };
        }

        // Funções de dois argumentos
        if argumentos.len() == 2 {
            let arg1 = self.avaliar(&argumentos[0])?;
            let arg2 = self.avaliar(&argumentos[1])?;
            return match nome {
                "pow" => Ok(arg1.powf(arg2)),
                "max" => Ok(arg1.max(arg2)),
                "min" => Ok(arg1.min(arg2)),
                _ => Err(format!("Função '{}' não aceita 2 argumentos", nome)),
            };
        }

        Err(format!(
            "Função '{}' chamada com {} argumentos (esperado 1 ou 2)",
            nome,
            argumentos.len()
        ))
    }
}