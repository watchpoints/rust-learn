#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    // TODO: Define the different variants used below.
    Quit,                           // 无数据
    Move(Point),                    // 包含一个 `Point` 结构体
    Echo(String),                   // 包含一个字符串
    ChangeColor(u8, u8, u8),        // 包含 3 个 `u8` 代表 RGB 颜色
    Resize { width: u64, height: u64 }, // 具名结构体（结构体字段语法）
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
