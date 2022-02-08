use seed::{prelude::*, *};

use serde::{Deserialize, Serialize};
use tinyrenderer::{Color, Render, Xy};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(|_| Msg::TestRender);
    let model = Model::default();
    // model.renderer.load_file("assets/cessna.obj");
    model
}

const IMAGE_SIZE: usize = 400;
struct Model {
    x: f64,
    y: f64,
    zoom: f64,
    canvas: ElRef<HtmlCanvasElement>,
    renderer: Render,
    loaded: bool,
    filename: String,
}

enum Msg {
    TestRender,
    Render,
    DownloadFile,
    Fetched(fetch::Result<String>),
    Test,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            zoom: 1.,
            canvas: ElRef::<HtmlCanvasElement>::default(),
            renderer: Render::default(),
            filename: "ant.obj".to_string(),
            loaded: false,
        }
    }
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::TestRender => {
            if model.loaded {
                draw(&model.canvas, &model)
            } else {
                draw_test(&model.canvas, &model);
            }
            orders.after_next_render(|_| Msg::TestRender).skip();
        }
        Msg::Render => {}
        Msg::Fetched(Ok(response_data)) => {
            log("got it result ok");
            model.renderer.load_from_string(response_data.as_str());
            // log(response_data);
            log(format!("file_data {:?}", model.renderer.file_data()));
            model.renderer.reload();
            model.renderer.update_file_render();
            model.loaded = true;
        }
        Msg::Fetched(Err(response_data)) => {
            log("got it");
        }
        Msg::DownloadFile => {
            log("outside downloading file");

            orders
                .skip()
                .perform_cmd(async { Msg::Fetched(download_file("assets/ant.obj").await) });
            // .perform_cmd(async { Msg::Fetched(download_file(model.filenameo).await) });
        }
        Msg::Test => {
            log("testing 1 2 3".to_string());
        }
    }
}
//TODO do we really need serde? just got it from the example
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendMessageRequestBody {
    pub text: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendMessageResponseBody {
    pub ordinal_number: u32,
    pub text: String,
}

async fn send_message(new_message: String) -> fetch::Result<String> {
    Request::new("asdf.html")
        .method(Method::Post)
        .json(&SendMessageRequestBody { text: new_message })?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

async fn download_file(filename: &str) -> fetch::Result<String> {
    log("inside downloading file");
    Request::new(filename)
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .text()
        .await
}

fn draw_test(canvas: &ElRef<HtmlCanvasElement>, model: &Model) {
    let canvas = canvas.get().expect("get canvas element");
    let mut ctx = seed::canvas_context_2d(&canvas);

    let height: f64 = 400.;
    let width: f64 = 400.;
    ctx.begin_path();
    ctx.clear_rect(0., 0., height, width);
    ctx.rect(0., 0., height, width);
    // DO remove me if below works outk
    ctx.set_fill_style(&JsValue::from_str("green"));
    ctx.fill();
    ctx.move_to(0., 0.);
    ctx.line_to(200., 250.);
    ctx.stroke();
    set_pixel(
        &mut ctx,
        100,
        40,
        Color {
            r: 200,
            b: 100,
            g: 0,
        },
    );
    draw_test_buffer(ctx, model);
}

fn draw_test_buffer(mut ctx: CanvasRenderingContext2d, model: &Model) {
    for x in 0..=model.renderer.width() as u32 {
        for y in 0..=model.renderer.height() as u32 {
            let color: Color;
            if y < 80 {
                color = Color { r: 240, g: 0, b: 0 };
            } else {
                color = Color { r: 0, g: 200, b: 0 };
            }
            set_pixel(&mut ctx, x, y, color);
        }
    }
}

fn draw(canvas: &ElRef<HtmlCanvasElement>, model: &Model) {
    let canvas = canvas.get().expect("get canvas element");
    let mut ctx = seed::canvas_context_2d(&canvas);

    let height: f64 = 400.;
    let width: f64 = 400.;
    ctx.begin_path();
    ctx.clear_rect(0., 0., height, width);
    ctx.rect(0., 0., height, width);
    // DO remove me if below works outk
    ctx.set_fill_style(&JsValue::from_str("green"));
    ctx.fill();
    ctx.move_to(0., 0.);
    ctx.line_to(200., 250.);
    ctx.stroke();
    set_pixel(
        &mut ctx,
        100,
        40,
        Color {
            r: 200,
            b: 100,
            g: 0,
        },
    );
    draw_buffer(ctx, model);
}

fn draw_buffer(mut ctx: CanvasRenderingContext2d, model: &Model) {
    let image_buffer = model.renderer.image_buffer();
    for x in 0..model.renderer.width() as u32 {
        for y in 0..model.renderer.height() as u32 {
            let color: Color;
            let color = image_buffer[&Xy(x as i32, y as i32)];
            // model.renderer.image.data[(y as usize) * model.renderer.width() + (x as usize)];

            set_pixel(&mut ctx, x, y, color);
        }
    }
}
fn view(model: &Model) -> impl IntoNodes<Msg> {
    div![
        style! { St::Display => "flex"},
        canvas![
            el_ref(&model.canvas),
            attrs![
              At::Width => px(1000), //model.render.width()),
              At::Height => px(1000), //model.render.height()),
            ],
            style![
              St::Border => "1px solid black"
            ],
        ],
        button!["Load File", ev(Ev::Click, |_| Msg::DownloadFile)],
        button!["Render", ev(Ev::Click, |_| Msg::Render)]
    ]
}
trait DrawBuffer {
    fn draw_buffer(ctx: CanvasRenderingContext2d);
}

// trait SetPixel{
fn set_pixel(ctx: &mut CanvasRenderingContext2d, x: u32, y: u32, color: Color) {
    let color_str = format!("rgba({},{},{},{})", color.r, color.g, color.b, 1);
    ctx.set_fill_style(&JsValue::from_str(color_str.as_str()));
    ctx.fill_rect(x as f64, y as f64, 1., 1.)
}
// }
// impl SetPixel for CanvasRenderingContext2d{};

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
