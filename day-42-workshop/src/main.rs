fn sandbox() {
    let list = vec![1, 2, 3];
    let double_list: Vec<i32> = list.iter().map(|item| item * 2).collect();
    println!("{:?}", double_list);
}

fn read_file() {
    let lines = include_str!("txt.txt");
    let _line_count = lines
        .lines()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .skip(2)
        .take(2)
        .for_each(|(_, line)| println!("{}", line));
}

enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Color {
    fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        }
        return false;
    }

    fn has_green_parts(&self) -> bool {
        match Color::Green {
            Color::Red => false,
            Color::Green => false,
            Color::Blue => true,
            Color::Yellow => true,
        }
    }
}

fn print_colour(color: Color) {
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::Yellow => println!("Yellow"),
    }
}

fn main() {
    let foo = Color::Green;
    println!("green is green - {}", foo.is_green());
    print_colour(Color::Red)
}
