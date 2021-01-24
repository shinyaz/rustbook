fn main() {
    // 1〜5 までの数値を持っているベクタを用意する。
    // vec![] とすると Rust ではベクタを生成できる。
    let source = vec![1, 2, 3, 4, 5];
    let result = source
        .into_iter()
        // 偶数か判定し、偶数なら残す
        .filter(|n| n % 2 == 0)
        // 数値を文字列型に変換する。
        .map(|n| n.to_string())
        //結果をリストに詰める。
        .collect::<Vec<String>>();

    // なにか後続の処理…
}
