use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
        rects: [i64; 3],
    }
    if dfs(&mut rects.clone(), x, y) {
        println!("Yes");
        return;
    }
    println!("No");
}

fn dfs(rects: &mut Vec<i64>, x: i64, y: i64) -> bool {
    // GOODな領域がなかったら false
    if 0 > x || 0 > y {
        return false;
    }
    // rects の合計が x*y より大きかったら false
    if rects.iter().sum::<i64>() > x * y {
        return false;
    }
    // rects を使い切ったら成功
    if rects.is_empty() {
        return true;
    }
    for i in 0..rects.len() {
        let now = rects[i];
        let mut new_y = y - now / x;
        if now % x != 0 {
            new_y -= 1;
        }
        let mut new_x = x - now / y;
        if now % y != 0 {
            new_x -= 1;
        }
        let mut new_rects = rects.clone();
        new_rects.remove(i);
        if dfs(&mut new_rects, x, new_y) || dfs(&mut new_rects, new_x, y) {
            return true;
        }
    }
    return false;
}
