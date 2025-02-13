use chapter_02::{dynamic_dispatch, Rect, Square};
fn main() {
    let rect = Rect {
        width: 10,
        height: 10,
    };
    let square = Square { size: 10 };
    println!("{}", dynamic_dispatch(&rect) + dynamic_dispatch(&square))
}
