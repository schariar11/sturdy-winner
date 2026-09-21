mod avaliador;
mod lexer;
mod parser;

use std::io::{self, BufRead, Write};

fn main() {
    println!("=== Calculadora Rust ===");
    println!("Digite expressões matemáticas para avaliar.");
    println!("Exemplos: 2 + 3 * 4, x = 10, sqrt(x), sin(pi/2)");
    println!("Constantes disponíveis: pi, e");
    println!("Funções: sin, cos, tan, sqrt, abs, ln, log, ceil, floor, round, pow, max, min");
    println!("Digite 'sair' para encerrar.\n");

    let stdin = io::stdin();
    let mut ambiente = avaliador::Ambiente::new();

    loop {
        print!("calc> ");
        io::stdout().flush().unwrap();

        let mut linha = String::new();
        if stdin.lock().read_line(&mut linha).unwrap() == 0 {
            break;
        }
        let entrada = linha.trim();

        if entrada.is_empty() {
            continue;
        }

        if entrada == "sair" {
            println!("Até logo!");
            break;
        }

        let tokens = match lexer::tokenizar(entrada) {
            Ok(t) => t,
            Err(e) => {
                println!("Erro léxico: {}", e);
                continue;
            }
        };


        let mut analisador = parser::Parser::new(tokens);
        let expressao = match analisador.analisar() {
            Ok(expr) => expr,
            Err(e) => {
                println!("Erro sintático: {}", e);
                continue;
            }
        };

        match ambiente.avaliar(&expressao) {
            Ok(resultado) => {
                if resultado == resultado.floor() && resultado.abs() < 1e15 {
                    println!("= {}", resultado as i64);
                } else {
                    println!("= {:.6}", resultado);
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}

// =====================================================================
// Testes Unitários
// =====================================================================
#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-6;

    fn quase_igual(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    // Helper interno 
    fn avaliar_teste(entrada: &str, ambiente: &mut avaliador::Ambiente) -> Result<f64, String> {
        let tokens = lexer::tokenizar(entrada).map_err(|e| e.to_string())?;
        let mut analisador = parser::Parser::new(tokens);
        let expressao = analisador.analisar().map_err(|e| e.to_string())?;
        ambiente.avaliar(&expressao).map_err(|e| e.to_string())
    }

    #[test]
    fn deve_calcular_operacoes_com_precedencia() {
        let mut env = avaliador::Ambiente::new();

        let res = avaliar_teste("2 + 3 * 4", &mut env).unwrap();
        assert_eq!(res, 14.0);

        let res_parenteses = avaliar_teste("(2 + 3) * 4", &mut env).unwrap();
        assert_eq!(res_parenteses, 20.0);
    }

    #[test]
    fn deve_atribuir_e_persistir_variaveis() {
        let mut env = avaliador::Ambiente::new();

        let atribuicao = avaliar_teste("x = 10", &mut env).unwrap();
        assert_eq!(atribuicao, 10.0);

        let calculo = avaliar_teste("x * 2", &mut env).unwrap();
        assert_eq!(calculo, 20.0);
    }

    #[test]
    fn deve_avaliar_funcoes_e_constantes() {
        let mut env = avaliador::Ambiente::new();

        let raiz = avaliar_teste("sqrt(16)", &mut env).unwrap();
        assert_eq!(raiz, 4.0);

        let seno = avaliar_teste("sin(pi / 2)", &mut env).unwrap();
        assert!(quase_igual(seno, 1.0));
    }

    #[test]
    fn deve_capturar_erros_de_sintaxe() {
        let mut env = avaliador::Ambiente::new();

        assert!(avaliar_teste("2 + * 3", &mut env).is_err());
        assert!(avaliar_teste("sqrt(", &mut env).is_err());
    }

    #[test]
    fn deve_falhar_com_variavel_nao_declarada() {
        let mut env = avaliador::Ambiente::new();
        assert!(avaliar_teste("variavel_inexistente + 1", &mut env).is_err());
    }
}
