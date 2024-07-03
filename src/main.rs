fn main() {
    let x = (1, "hello", 3.4);
    let (a, b, c) = x;
    println!("{} {} {}", a, b, c);
    println!("{}", x.1);

    // struct
    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 0, y: 0 };
    println!("{} {}", origin.x, origin.y);

    // tuple struct
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("{} {} {}", black.0, black.1, black.2);

    // enum
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let m = Message::Write(String::from("hello"));
}
