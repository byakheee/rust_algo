use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        m: usize,
        edge: [[i32; 2]; m],
    }
    let mut ans: Vec<i32> = Vec::new();
    let mut graph: Vec<Vec<i32>> = vec![Vec::new(); n];
    // すべての頂点の入次数を配列にする
    let mut in_degrees: Vec<i32> = vec![0; n];
    for i in 0..m {
        graph[(edge[i][0] - 1) as usize].push(edge[i][1] - 1);
        in_degrees[(edge[i][1] - 1) as usize] += 1;
    }
    // 小さい頂点から順に入次数 0 の頂点iを探し、キューに追加する
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    for i in 0..n {
        if in_degrees[i] == 0 {
            heap.push(Reverse(i as i32));
        }
    }
    // キューが空になるまで、結果に追加し、nowから入次している頂点の入次数を1減らす。この時入次数が0になったらキューに追加する
    while let Some(Reverse(now)) = heap.pop() {
        ans.push(now + 1);
        for edge_to in &graph[now as usize] {
            in_degrees[*edge_to as usize] -= 1;
            if in_degrees[*edge_to as usize] == 0 {
                heap.push(Reverse(*edge_to));
            }
        }
    }
    // キューが空になった時、結果の数がnになっていなければ、途中終了してるので-1
    if ans.len() != n {
        println!("-1");
        return;
    }
    let dst: Vec<String> = ans.iter().map(|x| x.to_string()).collect();
    println!("{}", dst.join(" "));
}

// 条件が何もないなら 1,2,...N が正解
// [1,2,3,4]で、例えば (3, 1) のような条件があれば 2, 3, 1, 4 が正解
// 1. 入次数0の中で一番小さいxを選ぶ -> なければ -1
// 2. xから出ている条件をすべて削除する
// 3. すべての数字を選ぶまで 1. に戻る
// 計算量いっぱいだった

// 1. すべての頂点の入次数を配列にする o(m)
// 2. 小さい頂点から順に入次数 0 の頂点iを探し、頂点から抜いて結果に追加する -> なければ-1 o(n)
// 3. iから入次している頂点の入次数から -1 する o(m)
// 4. 2.3.を n 回繰り返し、頂点が空になったら終了する
// これでも o(n^2)+o(m(n+1))

// 1. すべての頂点の入次数を配列indegreesにする o(m)
// 2. 小さい頂点から順に入次数 0 の頂点iを探し、キューに追加する -> なければ-1
// 3. キューが空になるまで、頂点から抜いて結果に追加し、iから入次している頂点の入次数を1減らす。この時入次数が0になったらキューに追加する
// 4. キューが空になった時、頂点が空になっていなければ-1 2-4でo(n)
// o(n)+o(m)
