use std::io::stdin;

fn main() {
    let mut sum = 0;

    // line 1, "n"
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<usize>().unwrap();

    // line 2, numbers to handle
    buf.clear();
    stdin().read_line(&mut buf).unwrap();
    let mut nums = buf
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    // descending order
    nums.sort();
    nums.reverse();

    // since the numbers are sorted, we only take
    // the largest and necessary numbers with this
    for num in nums.iter().take((n / 2) + (n % 2)) {
        sum += num
    }

    println!("{sum}")
}
