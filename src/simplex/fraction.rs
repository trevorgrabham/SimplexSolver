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
        assert_ne!(d,0);
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
        if self.denominator == 1 {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

impl std::ops::Sub for Fraction {
    type Output = Fraction;

    fn sub(self, other: Fraction) -> Fraction {
        let mut res = Fraction {
            numerator: self.numerator*other.denominator - other.numerator*self.denominator,
            denominator: self.denominator*other.denominator,
        };
        res.reduce();
        res
    }
}

impl std::ops::Add for Fraction {
    type Output = Fraction;

    fn add(self, other: Fraction) -> Fraction {
        let mut res = Fraction {
            numerator: self.numerator*other.denominator + other.numerator*self.denominator,
            denominator: self.denominator*other.denominator,
        };
        res.reduce();
        res
    }
}

impl std::ops::Mul for Fraction {
    type Output = Fraction;

    fn mul(self, other: Fraction) -> Fraction {
        let mut res = Fraction {
        numerator: self.numerator * other.numerator,
        denominator: self.denominator * other.denominator,
        };
        res.reduce();
        res
    }
}

impl std::ops::Div for Fraction {
    type Output = Fraction;

    fn div(self, other: Fraction) -> Fraction {
            let mut res = Fraction {
                numerator: self.numerator * other.denominator,
                denominator: self.denominator * other.numerator,
            };
            res.reduce();
            res
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Fraction) -> Option<std::cmp::Ordering> {
        if self.numerator == i64::MAX || self.numerator == -i64::MAX || other.numerator == i64::MAX || other.numerator == -i64::MAX {
            Some(self.numerator.cmp(&other.numerator))
        } else {
            Some((self.numerator*other.denominator).cmp(&(other.numerator*self.denominator)))
        }
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
        let mut power: u32 = 0;
        let mut n: f64 = f;
        while n != n.floor() {
            n *= 10f64;
            power += 1;
        }
        let mut res = Fraction {
            numerator: n as i64,
            denominator: 10i32.pow(power) as i64,
        };
        res.reduce();
        res
    }
}

impl From<Fraction> for f64 {
    fn from(frac: Fraction) -> f64 {
        frac.numerator as f64 / frac.denominator as f64 
    }
}