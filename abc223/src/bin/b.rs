use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut cases = Vec::new();
    let mut current = s;
    cases.push(format!("{}", current));
    for _i in 1..current.len() {
        current = format!("{}{}", &current[1..current.len()], &current[..1]);
        cases.push(format!("{}", current));
    }
    cases.sort();
    println!("{}", cases[0]);
    println!("{}", cases[cases.len() - 1]);
}
