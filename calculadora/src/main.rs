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

        // Tokenizar
        let tokens = match lexer::tokenizar(entrada) {
            Ok(t) => t,
            Err(e) => {
                println!("Erro léxico: {}", e);
                continue;
            }
        };

        // Analisar (parser)
        let mut analisador = parser::Parser::new(tokens);
        let expressao = match analisador.analisar() {
            Ok(expr) => expr,
            Err(e) => {
                println!("Erro sintático: {}", e);
                continue;
            }
        };

        // Avaliar
        match ambiente.avaliar(&expressao) {
            Ok(resultado) => {
                // Formata o resultado de forma legível
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