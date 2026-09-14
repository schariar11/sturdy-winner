use crate::lexer::Token;

/// Nó da árvore sintática abstrata
#[derive(Debug, Clone)]
pub enum Expressao {
    Numero(f64),
    Variavel(String),
    Negacao(Box<Expressao>),
    Operacao {
        esquerda: Box<Expressao>,
        operador: Operador,
        direita: Box<Expressao>,
    },
    Atribuicao {
        nome: String,
        valor: Box<Expressao>,
    },
    ChamadaFuncao {
        nome: String,
        argumentos: Vec<Expressao>,
    },
}

#[derive(Debug, Clone)]
pub enum Operador {
    Somar,
    Subtrair,
    Multiplicar,
    Dividir,
}

pub struct Parser {
    tokens: Vec<Token>,
    posicao: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, posicao: 0 }
    }

    fn atual(&self) -> &Token {
        &self.tokens[self.posicao]
    }

    fn avancar(&mut self) -> Token {
        let token = self.tokens[self.posicao].clone();
        if self.posicao < self.tokens.len() - 1 {
            self.posicao += 1;
        }
        token
    }

    fn esperar(&mut self, esperado: &Token) -> Result<(), String> {
        if self.atual() == esperado {
            self.avancar();
            Ok(())
        } else {
            Err(format!(
                "Esperado '{}', encontrado '{}'",
                esperado,
                self.atual()
            ))
        }
    }

    pub fn analisar(&mut self) -> Result<Expressao, String> {
        let resultado = self.expressao()?;
        if *self.atual() != Token::FimDaEntrada {
            return Err(format!("Token inesperado: '{}'", self.atual()));
        }
        Ok(resultado)
    }

    fn expressao(&mut self) -> Result<Expressao, String> {
        self.atribuicao()
    }

    fn atribuicao(&mut self) -> Result<Expressao, String> {
        // Verifica se é uma atribuição: identificador = expressao
        if let Token::Identificador(nome) = self.atual().clone() {
            if self.posicao + 1 < self.tokens.len()
                && self.tokens[self.posicao + 1] == Token::Igual
            {
                self.avancar(); // consome identificador
                self.avancar(); // consome '='
                let valor = self.expressao()?;
                return Ok(Expressao::Atribuicao {
                    nome,
                    valor: Box::new(valor),
                });
            }
        }
        self.soma()
    }

    fn soma(&mut self) -> Result<Expressao, String> {
        let mut esquerda = self.multiplicacao()?;

        loop {
            let operador = match self.atual() {
                Token::Mais => Operador::Somar,
                Token::Menos => Operador::Subtrair,
                _ => break,
            };
            self.avancar();
            let direita = self.multiplicacao()?;
            esquerda = Expressao::Operacao {
                esquerda: Box::new(esquerda),
                operador,
                direita: Box::new(direita),
            };
        }

        Ok(esquerda)
    }

    fn multiplicacao(&mut self) -> Result<Expressao, String> {
        let mut esquerda = self.unario()?;

        loop {
            let operador = match self.atual() {
                Token::Asterisco => Operador::Multiplicar,
                Token::Barra => Operador::Dividir,
                _ => break,
            };
            self.avancar();
            let direita = self.unario()?;
            esquerda = Expressao::Operacao {
                esquerda: Box::new(esquerda),
                operador,
                direita: Box::new(direita),
            };
        }

        Ok(esquerda)
    }

    fn unario(&mut self) -> Result<Expressao, String> {
        if *self.atual() == Token::Menos {
            self.avancar();
            let operando = self.unario()?;
            return Ok(Expressao::Negacao(Box::new(operando)));
        }
        self.chamada()
    }

    fn chamada(&mut self) -> Result<Expressao, String> {
        if let Token::Identificador(nome) = self.atual().clone() {
            if self.posicao + 1 < self.tokens.len()
                && self.tokens[self.posicao + 1] == Token::AbreParentese
            {
                self.avancar(); // consome identificador
                self.avancar(); // consome '('
                let mut argumentos = Vec::new();

                if *self.atual() != Token::FechaParentese {
                    argumentos.push(self.expressao()?);
                    while *self.atual() == Token::Virgula {
                        self.avancar();
                        argumentos.push(self.expressao()?);
                    }
                }

                self.esperar(&Token::FechaParentese)?;
                return Ok(Expressao::ChamadaFuncao { nome, argumentos });
            }
        }
        self.primario()
    }

    fn primario(&mut self) -> Result<Expressao, String> {
        match self.atual().clone() {
            Token::Numero(valor) => {
                self.avancar();
                Ok(Expressao::Numero(valor))
            }
            Token::Identificador(nome) => {
                self.avancar();
                Ok(Expressao::Variavel(nome))
            }
            Token::AbreParentese => {
                self.avancar();
                let expr = self.expressao()?;
                self.esperar(&Token::FechaParentese)?;
                Ok(expr)
            }
            outro => Err(format!("Expressão esperada, encontrado '{}'", outro)),
        }
    }
}