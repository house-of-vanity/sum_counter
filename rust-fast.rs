use std::env;

fn sum_digits_upto(n: u128, depth: usize) -> u128 {
    let indent = "| ".repeat(depth);
    eprintln!("{}-> Вызов sum_digits_upto(n={}, depth={})", indent, n, depth);

    if n < 10 {
        let result = n * (n + 1) / 2;
        eprintln!("{}   Базовый случай. Сумма цифр от 1 до {} = {}.", indent, n, result);
        eprintln!("{}<- Возврат {} из sum_digits_upto(n={})", indent, result, n);
        return result;
    }

    let mut p = 1;
    while p * 10 <= n {
        p *= 10;
    }
    let msd = n / p; // Самая значащая цифра
    eprintln!("{}   Определили: p={}, msd={}", indent, p, msd);

    // --- Часть 1: sum_before ---
    eprintln!("{}   [1] Считаем sum_before (хвосты полных блоков): msd * sum_digits_upto(p - 1)", indent);
    let sum_upto_p_minus_1 = sum_digits_upto(p - 1, depth + 1);
    let sum_before = msd * sum_upto_p_minus_1;
    eprintln!("{}   [1] sum_before = {} * {} = {}", indent, msd, sum_upto_p_minus_1, sum_before);

    // --- Часть 2: sum_msd ---
    let sum_msd = msd * (msd - 1) / 2 * p;
    eprintln!("{}   [2] sum_msd (головы полных блоков) = ({} * {} / 2) * {} = {}", indent, msd, msd - 1, p, sum_msd);

    // --- Часть 3: sum_rest ---
    eprintln!("{}   [3] Считаем sum_rest (остаток от {} до {})", indent, msd * p, n);
    let remainder = n % p;
    let sum_heads_rest = msd * (1 + remainder);
    eprintln!("{}   [3a] Головы в остатке: {} * (1 + {}) = {}", indent, msd, remainder, sum_heads_rest);
    eprintln!("{}   [3b] Хвосты в остатке: sum_digits_upto({})", indent, remainder);
    let sum_tails_rest = sum_digits_upto(remainder, depth + 1);
    let sum_rest = sum_heads_rest + sum_tails_rest;
    eprintln!("{}   [3] sum_rest = {} + {} = {}", indent, sum_heads_rest, sum_tails_rest, sum_rest);

    let total = sum_before + sum_msd + sum_rest;
    eprintln!("{}   Итого для n={}: {} + {} + {} = {}", indent, n, sum_before, sum_msd, sum_rest, total);
    eprintln!("{}<- Возврат {} из sum_digits_upto(n={})", indent, total, n);
    total
}

fn sum_digits_range(a: u128, b: u128) -> u128 {
    if a > 1 {
        eprintln!("\nСчитаем сумму в диапазоне [{}, {}]", a, b);
        eprintln!("Это (сумма до {}) - (сумма до {})", b, a - 1);
        sum_digits_upto(b, 0) - sum_digits_upto(a - 1, 0)
    } else {
        eprintln!("\nСчитаем сумму в диапазоне [1, {}]", b);
        sum_digits_upto(b, 0)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <start> <end>", args[0]);
        std::process::exit(1);
    }
    let start: u128 = args[1].parse().expect("Invalid start");
    let end: u128 = args[2].parse().expect("Invalid end");
    let result = sum_digits_range(start, end);
    println!("{}", result);
}
