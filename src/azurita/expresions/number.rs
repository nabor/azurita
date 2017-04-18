use std::fmt;

pub struct AzInteger {
    value: i64,
}

impl fmt::Display for AzInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl AzInteger {
    pub fn new(s: String) -> AzInteger {
        return AzInteger{ value: s.parse().unwrap() };
    }

    pub fn sum(self, i: AzInteger) -> AzInteger {
        return AzInteger{ value: self.value+i.value };
    }

    pub fn dif(self, i: AzInteger) -> AzInteger {
        return AzInteger{ value: self.value-i.value };
    }

    pub fn div(self, i: AzInteger) -> AzInteger {
        return AzInteger{ value: self.value/i.value };
    }

    pub fn pro(self, i: AzInteger) -> AzInteger {
        return AzInteger{ value: self.value*i.value };
    }
}
