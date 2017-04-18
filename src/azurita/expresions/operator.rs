use std::fmt;

pub enum AzOperator {
    AzSum,
    AzDif,
    AzDiv,
    AzPro,
}

impl fmt::Display for AzOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AzOperator::AzSum => write!(f, "+"),
            AzOperator::AzDif => write!(f, "-"),
            AzOperator::AzDiv => write!(f, "/"),
            AzOperator::AzPro => write!(f, "*"),
        }
    }
}

impl AzOperator {
    pub fn new(s: String) -> Result<AzOperator, &'static str> {
        match s.as_str() {
            "+" => return Ok(AzOperator::AzSum),
            "-" => return Ok(AzOperator::AzDif),
            "/" => return Ok(AzOperator::AzDiv),
            "*" => return Ok(AzOperator::AzPro),
            _ => return Err("Invalid operator"),
        }
    }
}
