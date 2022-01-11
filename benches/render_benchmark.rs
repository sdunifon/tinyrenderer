use criterion::Bencher;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tinyrenderer::fillable::Fillable;
use tinyrenderer::make_image;
// use tinyrenderer::test_helper::tests::assert_file_creation;
use tinyrenderer::*;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn triangle() -> Triangle {
    Triangle {
        vertices: [
            Vertex {
                x: 50.,
                y: 50.,
                z: 0.,
            },
            Vertex {
                x: 75.,
                y: 100.,
                z: 0.,
            },
            Vertex {
                x: 100.,
                y: 50.,
                z: 0.,
            },
        ],
    }
}

fn bench_render_only(b: &mut Bencher) {
    todo!("fix assert_file_creation importing");
    // const IMAGE_SIZE: usize = 1000;
    // let mut render = Render::default();
    // render.load_file("assets/head.obj");
    // render.update();
    // assert_file_creation("test_render.tga", |filename: &str| {
    //     render.update();
    // })
}

fn bench_render_triangle(b: &mut Bencher) {
    todo!();
    // const IMAGE_SIZE: usize = 250;
    // let mut image = Image::<IMAGE_SIZE, IMAGE_SIZE>::new();

    // let triangle = triangle();
    // b.iter(|| {
    //     image.draw(&triangle);
    //     triangle.fill(&mut image);
    // });
}

fn bench_render_filled_triangle(b: &mut Bencher) {
    todo!();
    //const IMAGE_SIZE: usize = 250;
    //const IMAGE_SIZE: usize = 500;
    // let mut image = Image::<IMAGE_SIZE, IMAGE_SIZE>::new();

    // let triangle = triangle();
    // b.iter(|| {
    //     image.draw(&triangle);
    //     triangle.fill(&mut image)
    // });
}

fn bench_render_hi_res_with_load(b: &mut Bencher) {
    b.iter(|| {
        let mut render = Render::new(RenderOptions {
            wireframe: false,
            height: 1024,
            width: 1024,
            ..Default::default()
        });

        render.load_file("assets/cessna.obj").unwrap();
        render.update_file_render().unwrap();
    })
}

fn bench_render_hi_res_just_render(b: &mut Bencher) {
    let mut render = Render::new(RenderOptions {
        wireframe: false,
        height: 1024,
        width: 1024,
        ..Default::default()
    });

    render.load_file("assets/cessna.obj").unwrap();
    b.iter(|| {
        render.update_file_render().unwrap();
    })
}
fn image_creation(b: &mut Bencher) {
    todo!("fix assert_file_creation importing");
    // assert_file_creation("test_render.tga", |filename: &str| {
    //     b.iter(|| make_image().unwrap().render(filename));
    // });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    // c.bench_function("render_only", bench_render_only);
    // c.bench_function("make_image", image_creation);
    // c.bench_function("draw wirefram triangle", bench_render_triangle);
    // c.bench_function("draw filled triangle", bench_render_filled_triangle);
    c.bench_function("render high res torus load", bench_render_hi_res_with_load);
    c.bench_function(
        "render high res torus render only",
        bench_render_hi_res_just_render,
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
