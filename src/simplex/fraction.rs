fn gcd(a: i64, b:i64) -> i64 {
    // Euclidean algorithm
    if b == 0{
        if a >= 0 {
            a
        }
        else {
            -a
        }
    }
    else {
        gcd(b, a%b)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fraction {
    pub numerator: i64,
    pub denominator: i64,
}

impl Fraction {
    pub fn new(n: i64, d: i64) -> Fraction {
        Fraction {
            numerator: n,
            denominator: d,
        }
    }

    pub fn reduce(&mut self){
        let divisor = gcd(self.numerator, self.denominator);
        self.numerator /= divisor;
        self.denominator /= divisor;
    }
}

impl std::fmt::Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl Mul for Fraction {
    fn mul(self, other: Fraction) -> Fraction {
        Fraction {
            let mut res = Fraction {
                numerator: self.numerator * other.numerator,
                denominator: self.denominator * other.denominator,
            }
            res.reduce()
        }
    }
}

impl Div for Fraction {
    fn div(self, other: Fraction) -> Fraction {
            let mut res = Fraction {
                numerator: self.numerator * other.denominator,
                denominator: self.denominator * other.numerator,
            }
            res.reduce()
    }
}

impl From<i64> for Fraction {
    fn from(i: i64) -> Fraction {
        Fraction {
            numerator: i,
            denominator: 1i64,
        }
    }
}

impl From<f64> for Fraction {
    fn from(f: f64) -> Fraction {
        let mut power: usize = 0;
        while(f != math::round::floor(f)){
            f *= 10;
            power++;
        }
        let mut res = Fraction {
            numerator: f as i64,
            denominator: 10i32.pow(power) as i64,
        }
        res.reduce()
    }
}

impl From<Fraction> for f64 {
    fn from(frac: Fraction) -> f64 {
        frac.numerator as f64 / frac.denominator as f64 
    }
}
