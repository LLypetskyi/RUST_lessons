use std::result;

enum Say {
    Hi(String),
    Bye(String),
    GM(String),
    GN(String),
}

fn main() {
    let say = Say::Bye("Bye".to_string());

    match say {
        Say::Hi(message) => println!("U say - {}", message),
        Say::Bye(message) => println!("U say - {}", message),
        Say::GM(message) => println!("U say - {}", message),
        Say::GN(message) => println!("U say - {}", message),
        // (_)=> {}
    }

    enum MathOperation {
        Add(f64, f64),
        Sub(f64, f64),
        Mul(f64, f64),
        Div(f64, f64),
    }

    impl MathOperation {
        fn calc(&self) -> f64 {
            match self {
                MathOperation::Add(a, b) => a + b,
                MathOperation::Sub(a, b) => a - b,
                MathOperation::Mul(a, b) => a * b,
                MathOperation::Div(a, b) => a / b,
            }
        }
    }
    // let mo = MathOperation::Mul((18.0), (9.0));
    // let result = mo.calc();
    let result = MathOperation::Mul((18.0), (9.0)).calc();

    println!("Result is => {}", result);
}
