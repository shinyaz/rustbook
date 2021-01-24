fn main() {
    // 変数 x に対して可変で 5 を束縛する。
    let mut x = 5;
    // println! は標準出力を意味する。
    println!("number is: {}", x);
    // 6という値を再代入できる。
    x = 6;
    println!("number is: {}", x);
}
