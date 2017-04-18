use std::fmt;
use azurita::expresions::{AzInteger, AzOperator};

pub enum Expresion {
    Void,
    Number(AzInteger),
    Operation{ operator: AzOperator, left: Box<Expresion>, right: Box<Expresion>},
}

impl fmt::Display for Expresion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expresion::Void => write!(f, "Void"),
            Expresion::Number(ref n) => write!(f, "{}", n),
            Expresion::Operation{ operator: ref o, left: ref l, right: ref r } => {
                    write!(f, "{} {} {}", l, o, r)
            },
        }
    }
}

impl Expresion {
    pub fn eval(self) -> Result<Expresion, &'static str> {
        match self {
            Expresion::Number(_) => return Ok(self),
            Expresion::Operation{ operator: o, left: l, right: r } => {
                match *l {
                    Expresion::Number(ln) => {
                        match *r {
                            Expresion::Number(rn) => {
                                match o {
                                    AzOperator::AzSum => return Ok(Expresion::Number(ln.sum(rn))),
                                    AzOperator::AzDif => return Ok(Expresion::Number(ln.dif(rn))),
                                    AzOperator::AzDiv => return Ok(Expresion::Number(ln.div(rn))),
                                    AzOperator::AzPro => return Ok(Expresion::Number(ln.pro(rn))),
                                }
                            },
                            _ => return Err("Invalix syntax"),
                        }
                    },
                    _ => return Err("Invalid syntax"),
                }
            },
            _ => return Err("Invalid Syntax"),
        }
    }
}
