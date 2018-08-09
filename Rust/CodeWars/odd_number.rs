fn main() {
    print!("{}\n", row_sum_odd_numbers(42));
}

fn ari(n: i64) -> usize {
    let mut sum = 0;
    for i in 1..n + 1 {
        sum += i as usize;
    }
    sum
}

fn row_sum_odd_numbers(n: i64) -> i64 {
    let ari_n: usize = ari(n - 1);
    let ari_n1: usize = ari(n);

    (1..1000000_i64)
        .filter(|x| x % 2 == 1)
        .enumerate()
        .filter(|(i, _)| i >= &ari_n && i < &ari_n1)
        .map(|(_, x)| x)
        .sum()
}
