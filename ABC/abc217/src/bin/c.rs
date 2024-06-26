use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut q = p.clone();
    for i in 0..n {
        q[p[i] - 1] = i + 1;
    }

    for v in q {
        print!("{} ", v);
    }
}
