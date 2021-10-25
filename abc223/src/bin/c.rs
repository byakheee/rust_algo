use proconio::input;

fn main() {
    input! {
        n: usize,
        fuse: [[f32; 2]; n],
    }
    let mut times: Vec<f32> = Vec::new();
    for i in 0..n {
        // それぞれの導火線の所要時間を出す
        times.push(fuse[i][0] / fuse[i][1]);
    }
    // 所要時間の合計を半分にして、順番に所要時間を引いて言って m 本目で到達するのを見つける
    let mut time_arrival_centar: f32 = times.iter().cloned().sum::<f32>() / 2.0;
    for m in 0..n {
        if time_arrival_centar < times[m] {
            // m 本目が何cmで消えるか算出する
            let mut length: f32 = fuse[m][1] * time_arrival_centar;
            // m本目までの長さを全部足す
            if m != 0 {
                length += fuse[0..m].iter().map(|x| x[0]).sum::<f32>();
            }
            println!("{}", length);
            return;
        }
        time_arrival_centar -= times[m];
    }
}
