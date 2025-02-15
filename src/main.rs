use tcod::colors::*;
use tcod::console::*;
struct Tcod {
    root : Root
}
fn main() {
    tcod::system ::set_fps(LIMIT_FPS);
    const SCREEN_WIDTH : i32 = 80;
    const SCREEN_HEIGHT :i32 = 50;
    const LIMIT_FPS : i32 = 20;

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/Libcot tutorial")
        .init();

    let mut tcod = Tcod{root};

    while !tcod.root.window_closed() {
        tcod.root.set_default_foreground(WHITE);
        tcod.root.clear();
        tcod.root.put_char(1,1, '@' , BackgroundFlag::None);
        tcod.root.flush();
        tcod.root.wait_for_keypress(true);
    }



}
