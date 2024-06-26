use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let res = a
        .iter()
        .tuple_combinations()
        .any(|(a, b, c)| a + b + c == 1000);

    println!("{}", if res { "Yes" } else { "No" });
}
