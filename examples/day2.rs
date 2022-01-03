const DATA: &str = include_str!("./day2.txt");

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Forward,
}

type Command = (Direction, i32);
fn parse_command(line: &str) -> Option<Command> {
    line.split_once(" ").map(|(direction, amount)| {
        let amount = amount.parse().expect("bad amount");
        match direction {
            "forward" => (Direction::Forward, amount),
            "up" => (Direction::Up, amount),
            "down" => (Direction::Down, amount),
            _ => panic!("bad direction"),
        }
    })
}

fn main() {
    let commands: Vec<Command> = DATA.split("\n").filter_map(|n| parse_command(n)).collect();

    // star 1
    type Position = (i32, i32);
    let diffs: Vec<Position> = commands
        .iter()
        .cloned()
        .map(|(command, amount)| match command {
            Direction::Up => (0, -amount),
            Direction::Down => (0, amount),
            Direction::Forward => (amount, 0),
        })
        .collect();
    let (horizontal, vertical) = diffs
        .into_iter()
        .reduce(|(ph, pv), (h, v)| (ph + h, pv + v))
        .expect("no data");
    println!(
        "[star 1] final position is ({}, {}), answer is [{}]",
        horizontal,
        vertical,
        horizontal * vertical
    );

    // star 2
    type PositionWithAim = (i32, i32, i32);
    let (horizontal, vertical, _): PositionWithAim = commands.into_iter().fold(
        (0, 0, 0),
        |(previous_horizontal, previous_vertical, previous_aim), (command, amount)| match command {
            Direction::Up => (
                previous_horizontal,
                previous_vertical,
                previous_aim - amount,
            ),
            Direction::Down => (
                previous_horizontal,
                previous_vertical,
                previous_aim + amount,
            ),
            Direction::Forward => (
                previous_horizontal + amount,
                previous_vertical + (amount * previous_aim),
                previous_aim,
            ),
        },
    );
    println!(
        "[star 2] final position is actually ({}, {}), the answer is [{}]",
        horizontal,
        vertical,
        horizontal * vertical
    );
}
