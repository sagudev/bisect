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
    fn get_s(&self) -> f64 {
        self.0.x
    }
    fn get_e(&self) -> f64 {
        self.1.x
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
        Self { expr }
    }
    fn eval(&self, x: f64) -> f64 {
        println!("{}", self.expr.replace("x", &(format!("({})", x))));
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
}

fn display_bool(b: bool) -> &'static str {
    if b {
        "+"
    } else {
        "-"
    }
}

fn main() {
    //let mut line = String::new();
    //println!("Enter expr :");
    //std::io::stdin().read_line(&mut line).unwrap();
    let m = m::new("x^3-3*x+1".to_string());
    //println!("Result: {}", m.eval(0.0));
    let int = m.interval(0.0, 1.0);
    let a = &int.0;
    let b = &int.1;
    if a.sign.is_none() {
        println!("Dobili smo točno ničlo {:?}", a)
    } else if b.sign.is_none() {
        println!("Dobili smo točno ničlo {:?}", b)
    } else {
        if a.sign.unwrap() == b.sign.unwrap() {
            println!("Ne znam določiti");
        } else {
            println!("Določujem");
            let c = m.c(int);
            println!("c = {:?}", c);
            if c.sign.is_none() {
                println!("c je ničla")
            } else {
                println!("nov interval {:?}", int.get_interval(c));
            }
        }
    }
}
