use std::env;

/// Excel の列名文字をインデックスに変換する。インデックスは 1 から始まる
/// 
/// # Examples
/// excol A -> 1
/// excol AX -> 50
fn main() {
    let args: Vec<String> = env::args().collect();
    let col = &args[1];
    let col_normalize = col.to_ascii_uppercase();
    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut sum = 0;
    let mut index = col_normalize.len() as u32;
    for ch in col_normalize.chars() {
        let pos = letters.chars().position(|c| c == ch).unwrap() as i32;
        sum += 26_i32.pow(index - 1) * (pos + 1);
        index = index.wrapping_sub(1);
    }
    println!("{}", sum);
}
