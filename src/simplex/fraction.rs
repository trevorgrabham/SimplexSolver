use std::{fmt,ops,cmp};

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
        assert_ne!(d,0, "Cannot create a fraction with a denominator of 0.");
        Fraction {
            numerator: n,
            denominator: d,
        }
    }

    fn reduce(&mut self){
        let divisor = gcd(self.numerator, self.denominator);
        self.numerator /= divisor;
        self.denominator /= divisor;
        if self.denominator < 0 {
            self.numerator *= -1;
            self.denominator *= -1;
        }
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.denominator == 1 {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

impl ops::Sub for Fraction {
    type Output = Fraction;

    fn sub(self, other: Fraction) -> Fraction {
        // INF - INF = 0
        if self.numerator.abs() == i64::MAX && other.numerator == self.numerator {
            if self.numerator > 0 {
        // INF - INF
                if other.numerator > 0 {
                    Fraction::from(0)
                } else {
        // INF - -INF
                    Fraction::from(i64::MAX)
                }
            } else {
        // -INF - INF
                if other.numerator > 0 {
                    Fraction::from(-i64::MAX)
                } else {
        // -INF - -INF
                    Fraction::from(0)
                }
            }
        } else if self.numerator.abs() == i64::MAX {
        // INF - other = INF
            Fraction::from(self.numerator)
        } else if other.numerator.abs() == i64::MAX {
        // self - INF = -INF
            Fraction::from(-other.numerator)
        } else if self.denominator.abs() == i64::MAX {
        // 0 - other = -other
            Fraction {
                numerator: -other.numerator,
                denominator: other.denominator,
            }
        } else if other.denominator.abs() == i64::MAX {
        // self - 0 = self
            Fraction {
                numerator: self.numerator,
                denominator: self.denominator,
            }
        } else {
            let mut res = Fraction {
                numerator: self.numerator*other.denominator - other.numerator*self.denominator,
                denominator: self.denominator*other.denominator,
            };
            res.reduce();
            res
        }
    }
}

impl ops::Add for Fraction {
    type Output = Fraction;

    fn add(self, other: Fraction) -> Fraction {
        if self.numerator.abs() == i64::MAX && other.numerator.abs() == i64::MAX {
            if self.numerator == i64::MAX {
        // INF + INF
                if other.numerator == i64::MAX {
                    Fraction::from(i64::MAX)
                } else {
        // INF + -INF
                    Fraction::from(0)
                }
            } else {
        // -INF + INF
                if other.numerator == i64::MAX {
                    Fraction::from(0)
                } else {
        // -INF + -INF
                    Fraction::from(-i64::MAX)
                }
            }
        } else if self.numerator.abs() == i64::MAX {
        // +-INF + other = +-INF 
            Fraction::from(self.numerator)
        } else if other.numerator.abs() == i64::MAX {
        // self +- INF = +-INF
            Fraction::from(other.numerator)
        } else if self.denominator.abs() == i64::MAX {
        // 0 + other = other
            Fraction {
                numerator: other.numerator,
                denominator: other.denominator,
            }
        } else if other.denominator.abs() == i64::MAX {
        // self + 0 = self
            Fraction {
                numerator: self.numerator,
                denominator: self.denominator,
            }
        } else {
            let mut res = Fraction {
                numerator: self.numerator*other.denominator + other.numerator*self.denominator,
                denominator: self.denominator*other.denominator,
            };
            res.reduce();
            res
        }
    }
}

impl ops::Mul for Fraction {
    type Output = Fraction;

    fn mul(self, other: Fraction) -> Fraction {
        // INF * INF = INF
        if self.numerator.abs() == i64::MAX && other.numerator.abs() == i64::MAX {
            if (self.numerator > 0 && other.numerator < 0) || (self.numerator < 0 && other.numerator > 0) {
                Fraction::from(-i64::MAX)
            } else {
                Fraction::from(i64::MAX)
            }
        } else if self.numerator.abs() == i64::MAX {
            if other.denominator.abs() == i64::MAX {
                if self.numerator > 0 {
                    if other.numerator > 0 {
        // INF * 0 = other.numerator/self.denominator
                        let mut res = Fraction::new(other.numerator, self.denominator);
                        res.reduce();
                        res
                    } else {
        // INF * -0 = -other.numerator/self.denominator
                        let mut res = Fraction::new(other.numerator, self.denominator);
                        res.reduce();
                        res
                    }
                } else {
                    if other.numerator > 0 {
        // -INF * 0 = -other.numerator/self.denominator
                        let mut res = Fraction::new(-other.numerator, self.denominator);
                        res.reduce();
                        res
                    } else {
        // -INF * -0 = other.numerator/self.denominator
                        let mut res = Fraction::new(-other.numerator, self.denominator);
                        res.reduce();
                        res
                    }
                }
            } else {
                if other.numerator > 0 {
        // INF * other = -INF || -INf * other = INF
                    Fraction::from(self.numerator)
                } else {
        // INF * -other = INF || -INF * -other = -INF
                    Fraction::from(-self.numerator)
                }
            }
        } else if self.denominator.abs() == i64::MAX {
            if other.numerator.abs() == i64::MAX {
                if self.numerator > 0 {
                    if other.numerator > 0 {
        // 0 * INF = self.numerator/other.denominator
                        let mut res = Fraction::new(self.numerator, other.denominator);
                        res.reduce();
                        res
                    } else {
        // 0 * -INF = -self.numerator/other.denominator
                        let mut res = Fraction::new(-self.numerator, other.denominator);
                        res.reduce();
                        res
                    }
                } else {
                    if other.numerator > 0 {
        // -0 * INF = -self.numerator/other.denominator
                        let mut res = Fraction::new(self.numerator, other.denominator);
                        res.reduce();
                        res
                    } else {
        // -0 * -INF = self.numerator/other.denominator
                        let mut res = Fraction::new(-self.numerator, other.denominator);
                        res.reduce();
                        res
                    }
                }
            } else {
        // 0 * other = 0
                Fraction::from(0) 
            }
        } else if other.numerator.abs() == i64::MAX {
            if self.numerator > 0 {
        // self * INF = INF || self * -INF = -INF 
                Fraction::from(other.numerator)
            } else {
        // -self * INF = -INF || -self * -INF = INF
                Fraction::from(-other.numerator)
            }
        } else if other.denominator.abs() == i64::MAX {
            Fraction::from(0)
        } else {
            let mut res = Fraction {
            numerator: self.numerator * other.numerator,
            denominator: self.denominator * other.denominator,
            };
            res.reduce();
            res
        }
    }
}

impl ops::Div for Fraction {
    type Output = Fraction;

    fn div(self, other: Fraction) -> Fraction {
        if self.numerator.abs() == i64::MAX && other.numerator.abs() == i64::MAX {
            if (self.numerator < 0 && other.numerator > 0) || (self.numerator > 0 && other.numerator < 0) {
        // INF / -INF  || -INF / INF = -other.denominator/self.denominator
                let mut res = Fraction::new(-other.denominator, self.denominator);
                res.reduce();
                res
            } else {
        // INF / INF  || -INF / -INF = other.denominator/self.denominator
                let mut res = Fraction::new(other.denominator, self.denominator);
                res.reduce();
                res
            }
        } else if self.numerator.abs() == i64::MAX {
            if other.numerator < 0 {
        // INF / -other  = INF  || -INF / -other = INF
                Fraction::from(-self.numerator)
            } else {
        // INF / other = INF  || -INF / other = -INF
                Fraction::from(self.numerator)
            }
        } else if other.numerator.abs() == i64::MAX {
        // self / INF = 0
            Fraction::from(0)
        } else if self.denominator.abs() == i64::MAX && other.denominator.abs() == i64::MAX {
        // 0 / 0 = self.numerator/other.numerator
            let mut res = Fraction::new(self.numerator, other.numerator);
            res.reduce();
            res
        } else if other.denominator.abs() == i64::MAX {
            if self.numerator > 0 {
                if other.numerator > 0 {
        // self / 0 = INF
                    Fraction::from(i64::MAX)
                } else {
        // self / -0 = -INF
                    Fraction::from(-i64::MAX)
                }
            } else {
                if other.numerator > 0 {
        // -self / 0 = INF
                    Fraction::from(-i64::MAX)
                } else {
        // -self / -0 = INF
                    Fraction::from(i64::MAX)
                }
            }
        } else if self.denominator.abs() == i64::MAX {
        // 0 / other = 0
            Fraction::from(0)
        } else {
            let mut res = Fraction {
                numerator: self.numerator * other.denominator,
                denominator: self.denominator * other.numerator,
            };
            res.reduce();
            res
        }
    }
}

impl ops::Neg for Fraction {
    type Output = Fraction;

    fn neg(self) -> Fraction {
        Fraction {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Fraction) -> Option<cmp::Ordering> {
        if self.numerator.abs() == i64::MAX || other.numerator.abs() == i64::MAX {
            Some(self.numerator.cmp(&other.numerator))
        } else if self.denominator.abs() == i64::MAX || other.denominator.abs() == i64::MAX {
            Some(other.denominator.cmp(&self.denominator))
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

mod fraction_test {
    use super::Fraction;
    #[test]
    fn add_inf2(){
        assert_eq!(Fraction::from(0), Fraction::from(i64::MAX) + Fraction::from(-i64::MAX), "Failed INF + -INF.");
        assert_eq!(Fraction::from(0), Fraction::from(-i64::MAX) + Fraction::from(i64::MAX), "Failed -INF + INF.");
        assert_eq!(Fraction::from(i64::MAX), Fraction::from(i64::MAX) + Fraction::from(i64::MAX), "Failed INF + INF.");
        assert_eq!(Fraction::from(-i64::MAX), Fraction::from(-i64::MAX) + Fraction::from(-i64::MAX), "Failed -INF + -INF.");
    }
    #[test]
    fn add_inf(){
        assert_eq!(Fraction::from(i64::MAX), Fraction::from(i64::MAX) + Fraction::from(10), "Failed INF + other.");
        assert_eq!(Fraction::from(-i64::MAX), Fraction::from(-i64::MAX) + Fraction::from(10), "Failed -INF + other.");
        assert_eq!(Fraction::from(i64::MAX), Fraction::from(10) + Fraction::from(i64::MAX), "Failed self + INF.");
        assert_eq!(Fraction::from(-i64::MAX), Fraction::from(10) + Fraction::from(-i64::MAX), "Failed self + -INF.");
    }
    #[test]
    fn add_0(){
        assert_eq!(Fraction::from(8), Fraction::new(1,i64::MAX) + Fraction::from(8), "Failed 0 + other.");
        assert_eq!(Fraction::from(-8), Fraction::new(1,i64::MAX) + Fraction::from(-8), "Failed 0 + -other.");
        assert_eq!(Fraction::from(7), Fraction::from(7) + Fraction::new(1,i64::MAX), "Failed self + 0.");
        assert_eq!(Fraction::from(-7), Fraction::from(-7) + Fraction::new(1,i64::MAX), "Failed -self + 0.");
    }
    #[test]
    fn add_standard(){
        assert_eq!(Fraction::from(17), Fraction::from(7) + Fraction::from(10), "Failed regular addition.");
        assert_eq!(Fraction::from(-3), Fraction::from(7) + Fraction::from(-10), "Failed regular addition.");
        assert_eq!(Fraction::from(3), Fraction::from(-7) + Fraction::from(10), "Failed regular addition.");
        assert_eq!(Fraction::from(-17), Fraction::from(-7) + Fraction::from(-10), "Failed regular addition.");
    }
    #[test]
    fn sub_inf2(){
        assert_eq!(Fraction::from(0), Fraction::from(i64::MAX) - Fraction::from(i64::MAX), "Failed INF - INF.");
        assert_eq!(Fraction::from(i64::MAX), Fraction::from(i64::MAX) - Fraction::from(-i64::MAX), "Failed INF - -INF.");
        assert_eq!(Fraction::from(-i64::MAX), Fraction::from(-i64::MAX) - Fraction::from(i64::MAX), "Failed -INF - INF.");
        assert_eq!(Fraction::from(0), Fraction::from(-i64::MAX) - Fraction::from(-i64::MAX), "Failed -INF - -INF.");
    }
    #[test]
    fn sub_inf(){
        assert_eq!(Fraction::from(i64::MAX), Fraction::from(i64::MAX) - Fraction::from(10), "Failed INF - other.");
        assert_eq!(Fraction::from(-i64::MAX), Fraction::from(-i64::MAX) - Fraction::from(10), "Failed -INF - other.");
        assert_eq!(Fraction::from(-i64::MAX), Fraction::from(10) - Fraction::from(i64::MAX), "Failed self - INF.");
        assert_eq!(Fraction::from(i64::MAX), Fraction::from(10) - Fraction::from(-i64::MAX), "Failed self - -INF.");
    }
    #[test]
    fn sub_0(){
        assert_eq!(Fraction::from(-8), Fraction::new(1,i64::MAX) - Fraction::from(8), "Failed 0 - other.");
        assert_eq!(Fraction::from(8), Fraction::new(1,i64::MAX) - Fraction::from(-8), "Failed 0 - -other.");
        assert_eq!(Fraction::from(7), Fraction::from(7) - Fraction::new(1,i64::MAX), "Failed self - 0.");
        assert_eq!(Fraction::from(-7), Fraction::from(-7) - Fraction::new(1,i64::MAX), "Failed -self - 0.");
    }
    #[test]
    fn sub_standard(){
        assert_eq!(Fraction::from(-3), Fraction::from(7) - Fraction::from(10), "Failed regular addition.");
        assert_eq!(Fraction::from(3), Fraction::from(-7) - Fraction::from(-10), "Failed regular addition.");
        assert_eq!(Fraction::from(17), Fraction::from(7) - Fraction::from(-10), "Failed regular addition.");
        assert_eq!(Fraction::from(-17), Fraction::from(-7) - Fraction::from(10), "Failed regular addition.");
    }
    #[test]
    fn mul_inf2(){
        assert_eq!(Fraction::from(i64::MAX), Fraction::from(i64::MAX) * Fraction::from(i64::MAX), "Failed INF * INF.");
        assert_eq!(Fraction::from(-i64::MAX), Fraction::from(i64::MAX) * Fraction::from(-i64::MAX), "Failed INF * -INF.");
        assert_eq!(Fraction::from(-i64::MAX), Fraction::from(-i64::MAX) * Fraction::from(i64::MAX), "Failed -INF * INF.");
        assert_eq!(Fraction::from(i64::MAX), Fraction::from(-i64::MAX) * Fraction::from(-i64::MAX), "Failed -INF * -INF.");
    }
    #[test]
    fn mul_inf0(){
        assert_eq!(Fraction::new(3,7), Fraction::new(i64::MAX, 7) * Fraction::new(3,i64::MAX), "Failed INF * 0.");
        assert_eq!(Fraction::new(-3,7), Fraction::new(-i64::MAX, 7) * Fraction::new(3,i64::MAX), "Failed -INF * 0.");
        assert_eq!(Fraction::new(-3,7), Fraction::new(i64::MAX, 7) * Fraction::new(-3,i64::MAX), "Failed INF * -0.");
        assert_eq!(Fraction::new(3,7), Fraction::new(-i64::MAX, 7) * Fraction::new(-3,i64::MAX), "Failed -INF * -0.");
    }
    #[test]
    fn mul_0inf(){
        assert_eq!(Fraction::new(7,3), Fraction::new(7,i64::MAX) * Fraction::new(i64::MAX,3), "Failed 0 * INF.");
        assert_eq!(Fraction::new(-7,3), Fraction::new(-7,i64::MAX) * Fraction::new(i64::MAX,3), "Failed -0 * INF.");
        assert_eq!(Fraction::new(-7,3), Fraction::new(7,i64::MAX) * Fraction::new(-i64::MAX,3), "Failed 0 * -INF.");
        assert_eq!(Fraction::new(7,3), Fraction::new(-7,i64::MAX) * Fraction::new(-i64::MAX,3), "Failed -0 * -INF.");
    }
    #[test]
    fn mul_inf(){
        assert_eq!(Fraction::from(-i64::MAX), Fraction::from(i64::MAX) * Fraction::from(-2), "Failed INF * -other.");
        assert_eq!(Fraction::from(-i64::MAX), Fraction::from(-i64::MAX) * Fraction::from(2), "Failed -INF * other.");
        assert_eq!(Fraction::from(i64::MAX), Fraction::from(-i64::MAX) * Fraction::from(-2), "Failed -INF * -other.");
        assert_eq!(Fraction::from(i64::MAX), Fraction::from(i64::MAX) * Fraction::from(2), "Failed INF * other.");
        assert_eq!(Fraction::from(-i64::MAX), Fraction::from(-10) * Fraction::from(i64::MAX), "Failed -self * INF.");
        assert_eq!(Fraction::from(i64::MAX), Fraction::from(-10) * Fraction::from(-i64::MAX), "Failed -self * -INF.");
        assert_eq!(Fraction::from(i64::MAX), Fraction::from(10) * Fraction::from(i64::MAX), "Failed self * INF.");
        assert_eq!(Fraction::from(-i64::MAX), Fraction::from(10) * Fraction::from(-i64::MAX), "Failed self * -INF.");
    }
    #[test]
    fn mul_0(){
        assert_eq!(Fraction::from(0), Fraction::new(1,i64::MAX) * Fraction::from(8), "Failed 0 * other.");
        assert_eq!(Fraction::from(0), Fraction::from(7) * Fraction::new(1,i64::MAX), "Failed self * 0.");
    }
    #[test]
    fn mul_standard(){
        assert_eq!(Fraction::from(21), Fraction::from(7) * Fraction::from(3), "Failed regular addition.");
        assert_eq!(Fraction::from(-21), Fraction::from(-7) * Fraction::from(3), "Failed regular addition.");
        assert_eq!(Fraction::from(-21), Fraction::from(7) * Fraction::from(-3), "Failed regular addition.");
        assert_eq!(Fraction::from(21), Fraction::from(-7) * Fraction::from(-3), "Failed regular addition.");
    }
    #[test]
    fn div_inf2(){
        assert_eq!(Fraction::new(3,7), Fraction::new(i64::MAX, 7) / Fraction::new(i64::MAX, 3), "Failed INF / INF.");
        assert_eq!(Fraction::new(-3,7), Fraction::new(-i64::MAX, 7) / Fraction::new(i64::MAX, 3), "Failed -INF / INF.");
        assert_eq!(Fraction::new(-3,7), Fraction::new(i64::MAX, 7) / Fraction::new(-i64::MAX, 3), "Failed INF / -INF.");
        assert_eq!(Fraction::new(3,7), Fraction::new(-i64::MAX, 7) / Fraction::new(-i64::MAX, 3), "Failed -INF / -INF.");
    }
    #[test]
    fn div_inf0(){
        assert_eq!(Fraction::from(i64::MAX), Fraction::new(i64::MAX, 7) / Fraction::new(3,i64::MAX), "Failed INF / 0.");
        assert_eq!(Fraction::from(-i64::MAX), Fraction::new(-i64::MAX, 7) / Fraction::new(3,i64::MAX), "Failed -INF / 0.");
        assert_eq!(Fraction::from(-i64::MAX), Fraction::new(i64::MAX, 7) / Fraction::new(-3,i64::MAX), "Failed INF / -0.");
        assert_eq!(Fraction::from(i64::MAX), Fraction::new(-i64::MAX, 7) / Fraction::new(-3,i64::MAX), "Failed -INF / -0.");
    }
    #[test]
    fn div_0inf(){
        assert_eq!(Fraction::from(0), Fraction::new(7,i64::MAX) / Fraction::new(i64::MAX,3), "Failed 0 / INF.");
    }
    #[test]
    fn div_inf(){
        assert_eq!(Fraction::from(-i64::MAX), Fraction::from(i64::MAX) / Fraction::from(-2), "Failed INF / -other.");
        assert_eq!(Fraction::from(i64::MAX), Fraction::from(-i64::MAX) / Fraction::from(-2), "Failed -INF / -other.");
        assert_eq!(Fraction::from(i64::MAX), Fraction::from(i64::MAX) / Fraction::from(2), "Failed INF / other.");
        assert_eq!(Fraction::from(-i64::MAX), Fraction::from(-i64::MAX) / Fraction::from(2), "Failed -INF / other.");
        assert_eq!(Fraction::from(0), Fraction::from(-10) / Fraction::from(i64::MAX), "Failed -self / INF.");
        assert_eq!(Fraction::from(0), Fraction::from(10) / Fraction::from(i64::MAX), "Failed self / INF.");
    }
    #[test]
    fn div_0(){
        assert_eq!(Fraction::from(0), Fraction::new(1,i64::MAX) / Fraction::from(8), "Failed 0 / other.");
        assert_eq!(Fraction::from(i64::MAX), Fraction::from(7) / Fraction::new(1,i64::MAX), "Failed self / 0.");
        assert_eq!(Fraction::from(-i64::MAX), Fraction::from(-7) / Fraction::new(1,i64::MAX), "Failed -self / 0.");
        assert_eq!(Fraction::from(-i64::MAX), Fraction::from(7) / Fraction::new(-1,i64::MAX), "Failed self / -0.");
        assert_eq!(Fraction::from(i64::MAX), Fraction::from(-7) / Fraction::new(-1,i64::MAX), "Failed -self / -0.");
    }
    #[test]
    fn div_standard(){
        assert_eq!(Fraction::new(7,3), Fraction::from(7) / Fraction::from(3), "Failed regular addition.");
        assert_eq!(Fraction::from(-21), Fraction::from(-7) / Fraction::new(1,3), "Failed regular addition.");
        assert_eq!(Fraction::new(-7,3), Fraction::from(7) / Fraction::from(-3), "Failed regular addition.");
        assert_eq!(Fraction::new(7,3), Fraction::from(-7) / Fraction::from(-3), "Failed regular addition.");
    }
    #[test]
    fn comparisons() {
        assert!(Fraction::new(1,3) > Fraction::new(1, 4));
        assert!(Fraction::new(1,6) > Fraction::new(1, 10));
        assert!(Fraction::new(1,i64::MAX) < Fraction::new(1, 4));
        assert!(Fraction::from(i64::MAX) > Fraction::from(4));
        assert!(Fraction::from(-i64::MAX) < Fraction::new(1, 4));
        assert!(Fraction::from(-i64::MAX) == Fraction::from(-i64::MAX));
        assert!(Fraction::from(1) > Fraction::from(-i64::MAX));
        assert!(Fraction::from(-i64::MAX) < Fraction::new(1,-i64::MAX));
        assert!(Fraction::from(i64::MAX) > Fraction::new(1,i64::MAX));
    }
    #[test]
    fn negation() {
        assert_eq!(Fraction::from(1), -Fraction::from(-1));
        assert_eq!(Fraction::from(-7), -Fraction::from(7));
    }
}