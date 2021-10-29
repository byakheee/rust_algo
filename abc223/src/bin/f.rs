use proconio::input;

fn main() {
    input! {
        _n: usize,  // Sの長さ
        q: usize,  // Queryの数
        s: String, // 文字列S
        queries: [[usize; 3]; q],
    }
    let mut _target = String::new();
    _target = s;
    for query in queries {
        match query[0] {
            1 => {
                // 文字列操作
                let mut v: Vec<&str> = _target.split("").collect();
                let cache = v[(query[1] - 1)];
                v[(query[1] - 1)] = v[(query[2])];
                v[(query[2])] = cache;
                _target = v.join("");
            }
            2 => {
                // 部分をcheckする
                if check((&_target[(query[1] - 1)..(query[2])]).to_string()) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
            _ => {}
        }
    }
}

fn check(s: String) -> bool {
    if s.len() % 2 != 0 {
        return false;
    }
    // "(" なら+1 ")"なら-1する
    let mut indicator = 0;
    for c in s.split("") {
        if c == "(" {
            indicator += 1;
        }
        if c == ")" {
            indicator -= 1;
        }
        if indicator < 0 {
            return false;
        }
    }
    if indicator == 0 {
        return true;
    }
    return false;
}
