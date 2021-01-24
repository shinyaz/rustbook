fn main() {
    // Some(1) を代入すると、値は1で奇数なので「奇数です: 1」と出力される。
    // Some(2) を代入すると、値は2で偶数なので「偶数です: 2」と出力される。
    // None を代入すると、「値はありません」と出力される。
    // i32 は符号付き整数型を示す。
    let objective: Option<i32> = Some(1);
    match objective {
        Some(x) if x % 2 == 0 => println!("偶数です: {}", x),
        Some(x) => println!("奇数です: {}", x),
        None => println!("値がありません"),
    }
}
