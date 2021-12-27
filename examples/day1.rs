const DATA: &str = include_str!("./day1.txt");

fn main() {
    let lines: Vec<i32> = DATA.split("\n").filter_map(|n| n.parse().ok()).collect();
    // star 1
    let count = |lines: &Vec<i32>| {
        lines
            .iter()
            .zip(lines.iter().skip(1))
            // .inspect(|(prev, next)| println!("{} < {} :: {}", prev, next, prev < next))
            .filter(|(prev, next)| prev < next)
            .count()
    };
    println!(
        "{} measurements are larger than the previous one",
        count(&lines)
    );

    // star 2
    let windows: Vec<_> = lines
        .iter()
        .zip(lines.iter().skip(1))
        .zip(lines.iter().skip(2))
        .map(|((a, b), c)| a + b + c)
        .collect();
    println!(
        "{} windowed measurements are larger than the previous one",
        count(&windows)
    );
}
