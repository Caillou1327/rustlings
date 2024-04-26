// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

/// Changer Some(p) en Some(ref p) pour pouvoir faire en sorte que y ce réfère au pointeur p

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y), 
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
