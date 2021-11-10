use criterion::Bencher;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tinyrenderer::test_helper::assert_file_creation;
use tinyrenderer::*;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn bench_render_only(b: &mut Bencher) {
    const IMAGE_SIZE: usize = 1000;
    let mut i = Image::<IMAGE_SIZE, IMAGE_SIZE>::new();

    let file = ModelFile {
        filename: "head.obj",
    };

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

fn image_creation(b: &mut Bencher) {
    assert_file_creation("test_render.tga", |filename: &str| {
        b.iter(|| make_image(filename));
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    c.bench_function("render_only", bench_render_only);
    c.bench_function("make_image", image_creation);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
