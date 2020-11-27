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
    fn mach(&self, int: &mut interval) {
        for i in 0..10 {
            let a = &int.0;
            let b = &int.1;
            println!("Interval iskanja ({}, {})", a.x, b.x);
            if a.sign.is_none() {
                println!("Dobili smo točno ničlo: x={}", a.x);
            } else if b.sign.is_none() {
                println!("Dobili smo točno ničlo: x={}", b.x);
            } else {
                if a.sign.unwrap() == b.sign.unwrap() {
                    println!("Ne znam določiti");
                } else {
                    let c = self.c(*int);
                    println!("c{} = {} = {}", i, F::from(c.x), c.x);
                    if c.sign.is_none() {
                        println!("c{} je ničla", i)
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

fn main() {
    //let mut line = String::new();
    //println!("Enter expr :");
    //std::io::stdin().read_line(&mut line).unwrap();
    let m = m::new("x^3-3*x+1".to_string());
    //println!("Result: {}", m.eval(0.0));
    // {:#.4}
    let mut int = m.interval(0.0, 1.0);
    m.mach(&mut int);
    let a = &int.0;
    let b = &int.1;
    println!("Končni interval ({}, {})", a.x, b.x);
}

#[test]
fn it_works() {
    let m = m::new("x^3-3*x+1".to_string());
    let mut int = m.interval(0.0, 1.0);
    m.mach(&mut int);
    let a = &int.0;
    let b = &int.1;
    println!("Končni interval ({}, {})", a.x, b.x);
}