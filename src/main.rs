const COUNT: usize = 100;

fn main() {
    let mut res: Vec<String> = Vec::with_capacity(COUNT);
    for i in 1..=COUNT {
        let s: String = match (i % 3, i % 5) {
            (0, 0) => String::from("FizzBuzz"),
            (0, _) => String::from("Fizz"),
            (_, 0) => String::from("Buzz"),
            (_, _) => format!("{}", i),
        };
        res.push(s);
    }
    print_result(&res);
}

fn print_result(result: &Vec<String>) {
    for r in result {
        println!("{}", r)
    }
}
