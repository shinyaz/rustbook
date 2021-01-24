fn main() {
    let dog = Dog{};
    let cat = Cat{};
    show_animal_data(dog);
    show_animal_data(cat);
}

trait Animal {
    // 動物の寿命を返す。
    fn lifespan(&self) -> u32;
    // 動物の学術名を返す。
    fn scientific_name(&self) -> String;
}

// 犬の構造体を用意する。
struct Dog;

// 犬の構造体に対する`lifespan()` 関数と `scientific_name()` 関数の定義をする。
impl Animal for Dog {
    fn lifespan(&self) -> u32 {
        13
    }

    fn scientific_name(&self) -> String {
        "Canis lupus familiaris.".to_string()
    }
}

// 猫の構造体を用意する。
struct Cat;

// 猫の構造体に対する`lifespan()` 関数と `scientific_name()`　関数の定義をする。
impl Animal for Cat {
    fn lifespan(&self) -> u32 {
        16
    }

    fn scientific_name(&self) -> String {
        "Felis Catus".to_string()
    }
}

// 動物の寿命と学術名を標準出力する関数。
// `Animal` トレイとに定義されている `lifespan()` 関数と `scientific_name()` 関数を内部で呼び出す。
// `T: Animal` というのは `Animal` トレイとを境界として持つものという意味。
// これにより、`Animal` を実装した方のみこの `T` という型引数に入れることができる。
fn show_animal_data<T: Animal>(animal: T) {
    println!("Lifespan: {}", animal.lifespan());
    println!("Scientific Name: {}", animal.scientific_name());
}