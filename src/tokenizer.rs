use std::{
    char as stdchar,
    iter::Peekable,
    result::Result as StdResult
};
use types::{Interpolate, Value};
use unic_ucd_name::Name as UnicName;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "invalid character in identifier: {}", _0)]
    InvalidIdent(char),
    #[fail(display = "invalid characters in interpolation: {:?}", _0)]
    InvalidInterpolation(String),
    #[fail(display = "invalid number: {:?}", _0)]
    InvalidNumber(String),
    #[fail(display = "invalid unicode character: {}", _0)]
    InvalidUnicode(String),
    #[fail(display = "unclosed comment")]
    UnclosedComment,
    #[fail(display = "unclosed interpolation in string")]
    UnclosedInterpolation,
    #[fail(display = "unclosed string")]
    UnclosedString,
    #[fail(display = "unexpected end of file")]
    UnexpectedEOF,
    #[fail(display = "unknown escape character: {}", _0)]
    UnknownEscape(char),
    #[fail(display = "unknown token")]
    UnknownToken,
}

type Result<T> = StdResult<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    It,
    Ident(String),
    Value(Value),

    Hai,
    KThxBye,
    Separator,

    IHasA,
    Itz,
    R,

    SumOf,
    DiffOf,
    ProduktOf,
    QuoshuntOf,
    ModOf,
    BiggrOf,
    SmallrOf,

    BothOf,
    EitherOf,
    WonOf,
    Not,
    AllOf,
    AnyOf,
    BothSaem,
    Diffrint,

    Smoosh,
    An,
    Mkay,

    ORly,
    YaRly,
    Mebbe,
    NoWai,
    Oic,

    Wtf,
    Omg,
    OmgWtf,
    Gtfo,

    ImInYr,
    Uppin,
    Nerfin,
    Til,
    Wile,
    ImOuttaYr,

    HowIzI,
    Yr,
    IfUSaySo,
    FoundYr,
    IIz,

    Visible,
    Exclamation,
    Gimmeh
}

#[derive(Clone)]
pub struct Tokenizer<I: Iterator<Item = char> + Clone> {
    pub iter: Peekable<I>
}

fn is_space(c: char) -> bool {
    c == ' ' || c == '\t'
}

impl<I: Iterator<Item = char> + Clone> Tokenizer<I> {
    fn trim(&mut self) {
        loop {
            match self.iter.peek().cloned() {
                Some(c) if is_space(c) => { self.iter.next(); },
                _ => break
            }
        }
    }
    fn peek(&mut self) -> Option<char> {
        self.trim();
        self.iter.peek().cloned()
    }
    fn word(&mut self) -> String {
        let mut word = String::new();
        loop {
            match self.iter.peek().cloned() {
                Some(c) if is_space(c) => {
                    self.trim();
                    return word;
                },
                None | Some('\n') | Some(',') => return word,
                Some(c) => {
                    self.iter.next();
                    word.push(c);
                }
            }
        }
    }
    /// Read one token from the input
    pub fn next(&mut self) -> Result<Option<Token>> {
        let c = match self.peek() {
            Some(c) => c,
            None => return Ok(None)
        };
        if c == '"' {
            fn read_until<I: Iterator<Item = char>>(iter: &mut I, c: char) -> Result<String> {
                let mut string = String::new();
                loop {
                    match iter.next() {
                        Some('"') => return Err(Error::UnclosedInterpolation),
                        None => return Err(Error::UnclosedString),

                        Some(c2) if c == c2 => break,
                        Some(c) => string.push(c)
                    }
                }
                Ok(string)
            }
            self.iter.next(); // leading "
            let mut interpolated = Vec::new();
            let mut string = String::new();
            while let Some(c) = self.iter.next() {
                if c == '|' {
                    match self.iter.next() {
                        Some('n') => string.push('\n'),
                        Some('t') => string.push('\t'),
                        Some('o') => string.push('\x07'),
                        Some('"') => string.push('"'),
                        Some(':') => string.push(':'),
                        Some('(') => {
                            let hex = read_until(&mut self.iter, ')')?;
                            let num = match u32::from_str_radix(&hex, 16) {
                                Ok(num) => num,
                                Err(_) => return Err(Error::InvalidNumber(hex))
                            };
                            match stdchar::from_u32(num) {
                                Some(c) => string.push(c),
                                None => return Err(Error::InvalidUnicode(hex))
                            }
                        },
                        Some('{') => {
                            let var = read_until(&mut self.iter, '}')?;
                            match var.chars().next() {
                                None |
                                Some('0'..='9') => return Err(Error::InvalidInterpolation(var)),
                                _ => ()
                            }
                            if !var.chars().all(|c|
                                    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') ||
                                    (c >= '0' && c <= '9') || c == '_') {
                                return Err(Error::InvalidInterpolation(var));
                            }
                            if !string.is_empty() {
                                interpolated.push(Interpolate::Str(string));
                            }
                            interpolated.push(Interpolate::Var(var));
                            string = String::new();
                        },
                        Some('[') => {
                            let name = read_until(&mut self.iter, ']')?.to_uppercase();
                            let mut unicode = None;
                            for c in chars!(..) {
                                if UnicName::of(c)
                                        .map(|n| n.to_string().to_uppercase() == name)
                                        .unwrap_or(false) {
                                    unicode = Some(c);
                                    break;
                                }
                            }
                            match unicode {
                                Some(c) => string.push(c),
                                None => return Err(Error::InvalidUnicode(name))
                            }
                        },
                        Some(c) => return Err(Error::UnknownEscape(c)),
                        None => return Err(Error::UnclosedString)
                    };
                    continue;
                } else if c == '"' {
                    break;
                }
                string.push(c);
            }
            if interpolated.is_empty() {
                return Ok(Some(Token::Value(Value::Yarn(string))));
            } else {
                if !string.is_empty() {
                    interpolated.push(Interpolate::Str(string));
                }
                return Ok(Some(Token::Value(Value::YarnRaw(interpolated))));
            }
        } else if c == '\n' || c == ',' {
            self.iter.next();
            return Ok(Some(Token::Separator));
        }

        let word = self.word();
        match &*word {
            "oi" => return Ok(Some(Token::Hai)),
            "adeus" => return Ok(Some(Token::KThxBye)),
            "comen" => {
                loop {
                    match self.iter.next() {
                        Some('\n') | None => break,
                        _ => ()
                    }
                }
                return self.next();
            },
//TODO: multiline comments
            "comentário" => {
                loop {
                    match self.peek() {
                        None => return Err(Error::UnclosedComment),
                        Some('f') => {
                            if self.word() == "fim" {
                                return self.next();
                            } else {
                                self.iter.next();
                            }
                        },
                        _ => { self.iter.next(); },
                    }
                }
            },
            "verdadeiro" => return Ok(Some(Token::Value(Value::Troof(true)))),
            "falso" => return Ok(Some(Token::Value(Value::Troof(false)))),
            "IT" => return Ok(Some(Token::It)),
            "eu" => {
                let mut clone = self.clone();
                match &*clone.word() {
                    "tenho" => if clone.word() == "um" {
                        *self = clone;
                        return Ok(Some(Token::IHasA));
                    },
                    // "IZ" => {
                    //     *self = clone;
                    //     return Ok(Some(Token::IIz));
                    // },
                    _ => ()
                }
            },
           "chame" => {
                let mut clone = self.clone();
                match &*clone.word() {
                    "a" => {
                        match &*clone.word() {
                            "função" => {
                               *self = clone;
                        return Ok(Some(Token::IIz));
                            },
                            _ => ()
                        }
                    },
                    _ => ()
                }
           },
           "que" => {
                let mut clone = self.clone();
                if clone.word() == "vale" {
                    *self = clone;
                    return Ok(Some(Token::Itz));
                }
            },
            //"que vale" => return Ok(Some(Token::Itz)),
            "é" => return Ok(Some(Token::R)),
            "soma" | "diferença" | "produto" | "quociente" | "módulo" | "maior" | "menor" |
            "ambos" | "qualquer" | "WON" | "todos" | "algum" => {
                let mut clone = self.clone();
                match &*clone.word() {
                    "de" => {
                        *self = clone;
                        return Ok(Some(match &*word {
                            "soma" => Token::SumOf,
                            "diferença" => Token::DiffOf,
                            "produto" => Token::ProduktOf,
                            "quociente" => Token::QuoshuntOf,
                            "módulo" => Token::ModOf,
                            "maior" => Token::BiggrOf,
                            "menor" => Token::SmallrOf,

                            "ambos" => Token::BothOf,
                            "qualquer" => Token::EitherOf,
                            "WON" => Token::WonOf,
                            "todos" => Token::AllOf,
                            "algum" => Token::AnyOf,

                            _ => unreachable!()
                        }));
                    },
                    "iguais" if word == "ambos" => {
                        *self = clone;
                        return Ok(Some(Token::BothSaem));
                    },
                    _ => ()
                }
            },
            "não" => return Ok(Some(Token::Not)),
            "diferente" => return Ok(Some(Token::Diffrint)),
            "SMOOSH" => return Ok(Some(Token::Smoosh)),
            "e" => return Ok(Some(Token::An)),
            "MKAY" => return Ok(Some(Token::Mkay)),
            "tem" => {
                let mut clone = self.clone();
                if clone.word() == "certeza?" {
                    *self = clone;
                    return Ok(Some(Token::ORly));
                }
            },
           "acho" => {
                let mut clone = self.clone();
                match &*clone.word() {
                    "que" => {
                        match &*clone.word() {
                            "sim" => {
                               *self = clone;
                        return Ok(Some(Token::YaRly));
                            },
                            "não" => {
                               *self = clone;
                        return Ok(Some(Token::NoWai));
                            },
                            _ => ()
                        }
                    },


                    _ => ()
                }
           },

            "talvez" => return Ok(Some(Token::Mebbe)),
            "resolvido" => return Ok(Some(Token::Oic)),
            "chave" => return Ok(Some(Token::Wtf)),
            "caso" => return Ok(Some(Token::Omg)),
            "padrão" => return Ok(Some(Token::OmgWtf)),
            "quebra" => return Ok(Some(Token::Gtfo)),
            "repita" => return Ok(Some(Token::ImInYr)),//{
               // let mut clone = self.clone();
               // match &*clone.word() {
                  //  "IN" => if clone.word() == "YR" {
                   //     *self = clone;
                   //     return Ok(Some(Token::ImInYr));
                  //  },
                  //  "OUTTA" => if clone.word() == "YR" {
                     //   *self = clone;
                   //     return Ok(Some(Token::ImOuttaYr));
                 //   },
               //     _ => ()
             //   }
           // },
            //"pare de repetir" => return Ok(Some(Token::ImOuttaYr)),
           "pare" => {
                let mut clone = self.clone();
                match &*clone.word() {
                    "de" => {
                        match &*clone.word() {
                            "repetir" => {
                           
                               *self = clone;
                        return Ok(Some(Token::ImOuttaYr));
                            },
                            _ => ()
                        }
                    },
                    _ => ()
                }
           },
            "incrementando" => return Ok(Some(Token::Uppin)),
            "decrementando" => return Ok(Some(Token::Nerfin)),
            "com" => return Ok(Some(Token::Yr)),
            "até" => return Ok(Some(Token::Til)),
            "enquanto" => return Ok(Some(Token::Wile)),
            "função" => return Ok(Some(Token::HowIzI)),// {
            //     let mut clone = self.clone();
            //     if clone.word() == "IZ" {
            //         if clone.word() == "I" {
            //             *self = clone;
            //             return Ok(Some(Token::HowIzI));
            //         }
            //     }
            // },
            "final" => {
                let mut clone = self.clone();
                if clone.word() == "da" {
                    if clone.word() == "função" {
                        //if clone.word() == "SO" {
                            *self = clone;
                            return Ok(Some(Token::IfUSaySo));
                        //}
                    }
                }
            },
            "retorne" => return Ok(Some(Token::FoundYr)),// {
            //     let mut clone = self.clone();
            //     if clone.word() == "YR" {
            //         *self = clone;
            //         return Ok(Some(Token::FoundYr));
            //     }
            // },
            "mostre" => return Ok(Some(Token::Visible)),
            "!" => return Ok(Some(Token::Exclamation)),
            "entrada" => return Ok(Some(Token::Gimmeh)),
            _ => ()
        }

        match c {
            'a'..='z' |
            'A'..='Z' |
            '_' | '?' => {
                for c in word.chars() {
                    match c {
                        'a'..='z' |
                        'A'..='Z' |
                        '0'..='9' |
                        '_' | '?' => (),
                        c => return Err(Error::InvalidIdent(c))
                    }
                }
                return Ok(Some(Token::Ident(word)));
            },
            '-' | '0'..='9' => {
                if let Ok(num) = word.parse::<i64>() {
                    return Ok(Some(Token::Value(Value::Numbr(num))));
                } else if let Ok(num) = word.parse::<f64>() {
                    return Ok(Some(Token::Value(Value::Numbar(num))));
                }
                return Err(Error::InvalidNumber(word));
            },
            _ => ()
        }

        Err(Error::UnknownToken)
    }
}

/// Convenience function for reading all tokens from `input`
pub fn tokenize<I: Iterator<Item = char> + Clone>(input: I) -> Result<Vec<Token>> {
    let mut tokenizer = Tokenizer { iter: input.peekable() };
    let mut tokens = Vec::new();
    while let Some(token) = tokenizer.next()? {
        tokens.push(token);
    }
    Ok(tokens)
}
/// Convenience function for reading all tokens from `input` from a string
pub fn tokenize_str(input: &str) -> Result<Vec<Token>> {
    tokenize(input.chars())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn yarns() {
        assert_eq!(
            tokenize_str(r#" "Hello World :) How are you :>? I'm:: :"fine:"" "#).unwrap(),
            &[Token::Value(Value::Yarn("Hello World \n How are you \t? I'm: \"fine\"".to_string()))]
        );
    }
    #[test]
    fn interpolation() {
        assert_eq!(
            tokenize_str(r#" ":[SNOWMAN] is :(1F60A). He says:: :{something}" "#).unwrap(),
            &[Token::Value(Value::YarnRaw(vec![
                Interpolate::Str("☃ is 😊. He says: ".to_string()),
                Interpolate::Var("something".to_string())
            ]))]
        );
    }
    #[test]
    fn primitives() {
        assert_eq!(
            tokenize_str("1, -5, 2.3, WIN, FAIL").unwrap(),
            &[
                Token::Value(Value::Numbr(1)), Token::Separator,
                Token::Value(Value::Numbr(-5)), Token::Separator,
                Token::Value(Value::Numbar(2.3)), Token::Separator,
                Token::Value(Value::Troof(true)), Token::Separator,
                Token::Value(Value::Troof(false))
            ]
        );
    }
    #[test]
    fn assign() {
        assert_eq!(
            tokenize_str("I HAS A VAR ITZ 12           BTW this is a comment").unwrap(),
            &[Token::IHasA, Token::Ident("VAR".to_string()), Token::Itz, Token::Value(Value::Numbr(12))]
        );
        assert_eq!(
            tokenize_str("VAR R 12").unwrap(),
            &[Token::Ident("VAR".to_string()), Token::R, Token::Value(Value::Numbr(12))]
        );
    }
    #[test]
    fn sum_of() {
        assert_eq!(
            tokenize_str("SUM OF OBTW hi TLDR 2 AN 4").unwrap(),
            &[Token::SumOf, Token::Value(Value::Numbr(2)), Token::An, Token::Value(Value::Numbr(4))]
        );
    }
    #[test]
    fn orly() {
        assert_eq!(
            tokenize_str("\
                BOTH SAEM 1 AN 1, O RLY?
                    YA RLY, RESULT R \"YES\"
                    MEBBE BOTH SAEM 1 AN 2, RESULT R \"CLOSE\"
                    NO WAI, RESULT R \"NO\"
                OIC\
            ").unwrap(),
            &[
                Token::BothSaem, Token::Value(Value::Numbr(1)), Token::An, Token::Value(Value::Numbr(1)), Token::Separator,
                Token::ORly, Token::Separator,
                    Token::YaRly, Token::Separator,
                        Token::Ident("RESULT".to_string()), Token::R, Token::Value(Value::Yarn("YES".to_string())),
                        Token::Separator,
                    Token::Mebbe,
                        Token::BothSaem, Token::Value(Value::Numbr(1)), Token::An, Token::Value(Value::Numbr(2)),
                        Token::Separator,
                        Token::Ident("RESULT".to_string()), Token::R, Token::Value(Value::Yarn("CLOSE".to_string())),
                        Token::Separator,
                    Token::NoWai, Token::Separator,
                        Token::Ident("RESULT".to_string()), Token::R, Token::Value(Value::Yarn("NO".to_string())),
                        Token::Separator,
                Token::Oic
            ]
        )
    }
    #[test]
    fn wtf() {
        assert_eq!(
            tokenize_str("\
                SUM OF 1 AN 3
                WTF?
                OMG 1
                    VISIBLE \"WHAT, NO\"
                OMG 2
                OMG 3
                    VISIBLE \"R U STUPID?\"
                    GTFO
                OMG 4
                    VISIBLE \"CORREC!\"
                    GTFO
                OMGWTF
                    VISIBLE \"IDFK\"
                    GTFO
                OIC\
            ").unwrap(),
            vec![
                Token::SumOf, Token::Value(Value::Numbr(1)), Token::An, Token::Value(Value::Numbr(3)), Token::Separator,
                Token::Wtf, Token::Separator,
                Token::Omg, Token::Value(Value::Numbr(1)), Token::Separator,
                    Token::Visible, Token::Value(Value::Yarn("WHAT, NO".to_string())), Token::Separator,
                Token::Omg, Token::Value(Value::Numbr(2)), Token::Separator,
                Token::Omg, Token::Value(Value::Numbr(3)), Token::Separator,
                    Token::Visible, Token::Value(Value::Yarn("R U STUPID?".to_string())), Token::Separator,
                    Token::Gtfo, Token::Separator,
                Token::Omg, Token::Value(Value::Numbr(4)), Token::Separator,
                    Token::Visible, Token::Value(Value::Yarn("CORREC!".to_string())), Token::Separator,
                    Token::Gtfo, Token::Separator,
                Token::OmgWtf, Token::Separator,
                    Token::Visible, Token::Value(Value::Yarn("IDFK".to_string())), Token::Separator,
                    Token::Gtfo, Token::Separator,
                Token::Oic
            ]
        );
    }
    #[test]
    fn loops() {
        assert_eq!(
            tokenize_str("\
                IM IN YR LOOP UPPIN YR VAR TIL BOTH SAEM VAR AN 5
                    VISIBLE VAR
                IM OUTTA YR LOOP\
            ").unwrap(),
            &[Token::ImInYr, Token::Ident("LOOP".to_string()), Token::Uppin, Token::Yr, Token::Ident("VAR".to_string()),
              Token::Til, Token::BothSaem, Token::Ident("VAR".to_string()), Token::An, Token::Value(Value::Numbr(5)),
              Token::Separator,
              Token::Visible, Token::Ident("VAR".to_string()), Token::Separator,
              Token::ImOuttaYr, Token::Ident("LOOP".to_string())]
        );
    }
    #[test]
    fn functions() {
        assert_eq!(
            tokenize_str("\
                HOW IZ I SCREAMING YR STUFF
                    VISIBLE STUFF \"!\"
                IF U SAY SO

                I IZ SCREAMING \"STUFF\" MKAY\
            ").unwrap(),
            &[Token::HowIzI, Token::Ident("SCREAMING".to_string()), Token::Yr, Token::Ident("STUFF".to_string()),
              Token::Separator,
              Token::Visible, Token::Ident("STUFF".to_string()), Token::Value(Value::Yarn("!".to_string())),
              Token::Separator,
              Token::IfUSaySo, Token::Separator,
              Token::Separator,
              Token::IIz, Token::Ident("SCREAMING".to_string()), Token::Value(Value::Yarn("STUFF".to_string())), Token::Mkay]
        );
    }
}
