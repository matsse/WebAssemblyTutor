#[macro_use]
extern crate  stdweb;

mod canvas;
mod direction;


use canvas::Canvas;

fn main() {
    stdweb::initialize();

    let canvas = Canvas::new("#canvas", 20, 20);

    canvas.draw(5,5,"red");
    canvas.draw(10,10,"Orange");
    canvas.draw(15,15, "blue");

    canvas.draw(15,12, "purple");
    stdweb::event_loop();
}
