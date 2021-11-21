use criterion::Bencher;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tinyrenderer::fillable::Fillable;
use tinyrenderer::test_helper::assert_file_creation;
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
            Vertex { x: 50, y: 50, z: 0 },
            Vertex {
                x: 75,
                y: 100,
                z: 0,
            },
            Vertex {
                x: 100,
                y: 50,
                z: 0,
            },
        ],
    }
}

fn bench_render_only(b: &mut Bencher) {
    const IMAGE_SIZE: usize = 1000;
    let mut i = Image::<IMAGE_SIZE, IMAGE_SIZE>::new();

    let file = ModelFile::open("head.obj");

    let verticies = file.vertex_parse(IMAGE_SIZE, IMAGE_SIZE);

    let faces = file.face_parse(&verticies);

    b.iter(|| {
        for face in &faces {
            i.draw(face)
        }
    });

    assert_file_creation("test_render.tga", |filename: &str| {
        i.render(filename);
    })
}

fn bench_render_triangle(b: &mut Bencher) {
    const IMAGE_SIZE: usize = 250;
    let mut image = Image::<IMAGE_SIZE, IMAGE_SIZE>::new();

    let triangle = triangle();
    b.iter(|| {
        image.draw(&triangle);
        triangle.fill(&mut image);
    });
}

fn bench_render_filled_triangle(b: &mut Bencher) {
    const IMAGE_SIZE: usize = 500;
    let mut image = Image::<IMAGE_SIZE, IMAGE_SIZE>::new();

    let triangle = triangle();
    b.iter(|| {
        image.draw(&triangle);
        triangle.fill(&mut image)
    });
}

fn image_creation(b: &mut Bencher) {
    assert_file_creation("test_render.tga", |filename: &str| {
        b.iter(|| make_image().render(filename));
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    c.bench_function("render_only", bench_render_only);
    c.bench_function("make_image", image_creation);
    c.bench_function("draw wirefram triangle", bench_render_triangle);
    c.bench_function("draw filled triangle", bench_render_filled_triangle);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
