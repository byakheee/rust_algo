use proconio::input;

fn main() {
    input! {
        s: String
    }
    solve(s, vec![0]);
}

fn solve(remain: String, current: Vec<i32>) {
    // remain が 0 なら　sum して出力して終わり

    // remainの先頭から一文字抜く
    // それを i32 に変換して current の末尾の数字に足すパターン
    // それを i32 に変換して current の末尾にpushするパターン
}
