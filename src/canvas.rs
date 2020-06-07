use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d};


pub struct Canvas {
    pub canvas: CanvasElement,
    pub ctx: CanvasRenderingContext2d,
    scaled_width: u32,
    scaled_height: u32,
    width:  u32,
    height: u32,
}

impl Canvas{
    pub fn new(attr_id: &str, width: u32, height: u32) -> Canvas {
        let canvas : CanvasElement = document()
            .query_selector(attr_id)
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

    }
}