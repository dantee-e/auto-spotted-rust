use fltk::{app, prelude::*, window::Window};
use crate::definitions::{SpottedStruct, get_spotted_nmr};
use crate::imagens;


#[allow(dead_code)]
pub fn window_spotted(mut spotted_contents: SpottedStruct){
    spotted_contents.spt_num = get_spotted_nmr();
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    wind.end();
    wind.show();
    app.run().unwrap();
}
