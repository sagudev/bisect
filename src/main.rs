use bisect::m;
use std::io::Read;
use std::io::Write;

fn pause() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press enter to exit").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
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
        break;
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
        break;
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
        break;
    }
    let mut int = m.interval(a, b);
    println!("x ≈ {}", m.mach(&mut int, precision));
    pause();
}
