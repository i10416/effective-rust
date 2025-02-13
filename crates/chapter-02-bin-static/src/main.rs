use chapter_02::{static_dispatch, Rect, Square};
fn main() {
    let rect = Rect {
        width: 10,
        height: 10,
    };
    let square = Square { size: 10 };
    println!("{}", static_dispatch(&rect) + static_dispatch(&square))
}
