fn main() {
    let mut rl = rustyline::Editor::<()>::new();
    loop {
        match rl.readline("user> ") {
            Ok(line) => {
                let output = rep(line);
                println!("{}", output);
            }
            Err(_) => break,
        }
    }
}

fn read(input: String) -> String {
    input
}

fn eval(expr: String) -> String {
    expr
}
fn print(result: String) -> String {
    result
}

fn rep(input: String) -> String {
    let expr = read(input);
    let result = eval(expr);
    let output = print(result);
    output
}
