use seed::{prelude::*, *};
use tinyrenderer::Image;
use web_sys::HtmlCanvasElement;

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(|_| Msg::Rendered);
    Model::default()
}
const IMAGE_SIZE: usize = 400;
struct Model {
    x: f64,
    y: f64,
    zoom: f64,
    image: Image<IMAGE_SIZE, IMAGE_SIZE>,
    canvas: ElRef<HtmlCanvasElement>,
}

enum Msg {
    Rendered,
    Action,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            zoom: 1.,
            image: Image::new(),
            canvas: ElRef::<HtmlCanvasElement>::default(),
        }
    }
}

fn view(model: &Model) -> impl IntoNodes<Msg> {
    div![
        style! { St::Display => "flex"},
        canvas![
            el_ref(&model.canvas),
            attrs![
              At::Width => px(model.image.width),
              At::Height => px(model.image.height),
            ],
            style![
              St::Border => "1px solid black"
            ],
        ],
        button!["Action", ev(Ev::Click, |_| Msg::Action)]
    ]
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Rendered => {
            orders.after_next_render(|_| Msg::Rendered).skip();
            draw(&model.canvas, &model)
        }
        Msg::Action => todo!(),
    }
}

fn draw(canvas: &ElRef<HtmlCanvasElement>, model: &Model) {
    let canvas = canvas.get().expect("get canvas element");
    let ctx = seed::canvas_context_2d(&canvas);

    ctx.begin_path();
    ctx.clear_rect(0., 0., model.image.height as f64, model.image.width as f64);
    ctx.rect(0., 0., model.image.height as f64, model.image.width as f64);
    ctx.set_fill_style(&JsValue::from_str("green"));
}
#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
