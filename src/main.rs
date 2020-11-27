type F = fraction::Fraction;

#[derive(Debug, Clone, Copy)]
struct num {
    /// num
    x: f64,
    /// of result
    sign: Option<bool>,
    /// res
    p: f64,
}

#[derive(Debug, Clone, Copy)]
struct interval(num, num);

impl interval {
    fn get_c(&self) -> f64 {
        (self.0.x + self.1.x) / 2.0
    }
    fn get_interval(self, n: num) -> Self {
        if self.0.sign.unwrap() != n.sign.unwrap() {
            interval(self.0, n)
        } else if self.1.sign.unwrap() != n.sign.unwrap() {
            interval(self.1, n)
        } else {
            panic!("se ne zgodi")
        }
    }
}

struct m {
    expr: String,
}

impl m {
    fn new(expr: String) -> Self {
        meval::eval_str(expr.replace("x", &"(1)")).unwrap();
        Self { expr }
    }
    fn eval(&self, x: f64) -> f64 {
        //println!("{}", self.expr.replace("x", &(format!("({})", x))));
        meval::eval_str(self.expr.replace("x", &(format!("({})", x)))).unwrap()
    }
    fn sign(&self, x: f64) -> Option<bool> {
        let r = self.eval(x);
        if r == 0.0 || r == -0.0 {
            None
        } else {
            Some(r.is_sign_positive())
        }
    }
    fn get(&self, x: f64) -> num {
        let p = self.eval(x);
        num {
            x,
            sign: if p == 0.0 || p == -0.0 {
                None
            } else {
                Some(p.is_sign_positive())
            },
            p,
        }
    }
    fn interval(&self, a: f64, b: f64) -> interval {
        interval(self.get(a), self.get(b))
    }
    fn c(&self, int: interval) -> num {
        self.get(int.get_c())
    }
    fn mach(&self, int: &mut interval, precision: usize) -> f64 {
        let mut i = 0;
        loop {
            let a = &int.0;
            let b = &int.1;
            if round(a.x, precision) == round(b.x, precision) {
                println!("Končni interval ({}, {})", a.x, b.x);
                return round(a.x, precision);
            }
            println!("Interval iskanja ({}, {})", a.x, b.x);
            if a.sign.is_none() {
                println!("Dobili smo točno ničlo: x={}", a.x);
                return a.x;
            } else if b.sign.is_none() {
                println!("Dobili smo točno ničlo: x={}", b.x);
                return b.x;
            } else {
                if a.sign.unwrap() == b.sign.unwrap() {
                    println!("Ne znam določiti");
                } else {
                    let c = self.c(*int);
                    println!("c{} = {} = {}", i, F::from(c.x), c.x);
                    if c.sign.is_none() {
                        println!("c{} je ničla", i);
                        return c.x;
                    } else {
                        println!(
                            "p(c{}) = {} kar je {}",
                            i,
                            c.p,
                            display_bool(c.sign.unwrap())
                        );
                        *int = int.get_interval(c);
                    }
                }
            }
            i += 1;
        }
    }
}

fn display_bool(b: bool) -> &'static str {
    if b {
        "poz"
    } else {
        "neg"
    }
}

fn round(f: f64, precision: usize) -> f64 {
    (f * 10_f64.powf(precision as f64)).round() / 10_f64.powf(precision as f64)
}

fn main() {
    let m;
    loop {
        let mut line = String::new();
        println!("Vpiši polinom (x^3-3*x+1):");
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if !line.is_empty() {
            m = m::new(line);
            break;
        }
    }

    let precision: usize;
    loop {
        let mut line = String::new();
        println!("Kakšna natančnost:");
        std::io::stdin().read_line(&mut line).unwrap();
        precision = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break
    }

    let a: f64;
    loop {
        let mut line = String::new();
        println!("Interval (x, _):");
        std::io::stdin().read_line(&mut line).unwrap();
        a = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break
    }

    let b: f64;
    loop {
        let mut line = String::new();
        println!("Interval (_, x):");
        std::io::stdin().read_line(&mut line).unwrap();
        b = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break
    }
    let mut int = m.interval(a, b);
    println!("x ≈ {}", m.mach(&mut int, precision));
}

#[test]
fn it_works() {
    let m = m::new("x^3-3*x+1".to_string());
    let mut int = m.interval(0.0, 1.0);
    let precision: usize = 2;
    assert_eq!(0.35, m.mach(&mut int, precision))
}

#[test]
fn round_test() {
    assert_eq!(0.05, round(0.05, 2));
    assert_eq!(0.1, round(0.05, 1));
}
