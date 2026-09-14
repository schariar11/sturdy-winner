/// Tipos de token reconhecidos pela calculadora
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Numero(f64),
    Identificador(String),
    Mais,         // +
    Menos,        // -
    Asterisco,    // *
    Barra,        // /
    AbreParentese,  // (
    FechaParentese, // )
    Igual,        // =
    Virgula,      // ,
    FimDaEntrada,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Numero(n) => write!(f, "{}", n),
            Token::Identificador(nome) => write!(f, "{}", nome),
            Token::Mais => write!(f, "+"),
            Token::Menos => write!(f, "-"),
            Token::Asterisco => write!(f, "*"),
            Token::Barra => write!(f, "/"),
            Token::AbreParentese => write!(f, "("),
            Token::FechaParentese => write!(f, ")"),
            Token::Igual => write!(f, "="),
            Token::Virgula => write!(f, ","),
            Token::FimDaEntrada => write!(f, "EOF"),
        }
    }
}

/// Transforma uma string em uma sequência de tokens
pub fn tokenizar(entrada: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let caracteres: Vec<char> = entrada.chars().collect();
    let mut pos = 0;

    while pos < caracteres.len() {
        let c = caracteres[pos];

        // Pular espaços em branco
        if c.is_whitespace() {
            pos += 1;
            continue;
        }

        // Números (inteiros e decimais)
        if c.is_ascii_digit() || (c == '.' && pos + 1 < caracteres.len()
            && caracteres[pos + 1].is_ascii_digit())
        {
            let inicio = pos;
            while pos < caracteres.len()
                && (caracteres[pos].is_ascii_digit() || caracteres[pos] == '.')
            {
                pos += 1;
            }
            let texto: String = caracteres[inicio..pos].iter().collect();
            let numero: f64 = texto
                .parse()
                .map_err(|_| format!("Número inválido: '{}'", texto))?;
            tokens.push(Token::Numero(numero));
            continue;
        }

        // Identificadores (variáveis e funções)
        if c.is_ascii_alphabetic() || c == '_' {
            let inicio = pos;
            while pos < caracteres.len()
                && (caracteres[pos].is_ascii_alphanumeric() || caracteres[pos] == '_')
            {
                pos += 1;
            }
            let nome: String = caracteres[inicio..pos].iter().collect();
            tokens.push(Token::Identificador(nome));
            continue;
        }

        // Operadores e pontuação
        let token = match c {
            '+' => Token::Mais,
            '-' => Token::Menos,
            '*' => Token::Asterisco,
            '/' => Token::Barra,
            '(' => Token::AbreParentese,
            ')' => Token::FechaParentese,
            '=' => Token::Igual,
            ',' => Token::Virgula,
            _ => return Err(format!("Caractere inesperado: '{}'", c)),
        };
        tokens.push(token);
        pos += 1;
    }

    tokens.push(Token::FimDaEntrada);
    Ok(tokens)
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn testar_expressao_simples() {
        let tokens = tokenizar("3 + 4.5 * 2").unwrap();
        assert_eq!(tokens[0], Token::Numero(3.0));
        assert_eq!(tokens[1], Token::Mais);
        assert_eq!(tokens[2], Token::Numero(4.5));
        assert_eq!(tokens[3], Token::Asterisco);
        assert_eq!(tokens[4], Token::Numero(2.0));
    }

    #[test]
    fn testar_atribuicao() {
        let tokens = tokenizar("x = 10").unwrap();
        assert_eq!(tokens[0], Token::Identificador("x".to_string()));
        assert_eq!(tokens[1], Token::Igual);
        assert_eq!(tokens[2], Token::Numero(10.0));
    }
}