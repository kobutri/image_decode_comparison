use std::hint::black_box;
use std::io::Cursor;
use std::time::Duration;
use criterion::{criterion_group, criterion_main, Criterion};

const JXL_FAST: &[u8] = include_bytes!("../data/fast.jxl");
const JXL_SLOW: &[u8] = include_bytes!("../data/slow.jxl");
const JPEG: &[u8] = include_bytes!("../data/photo.jpg");
const AVIF: &[u8] = include_bytes!("../data/photo.avif");
const PNG: &[u8] = include_bytes!("../data/photo.png");
const WEBP: &[u8] = include_bytes!("../data/photo.webp");

fn decode_zune_jpg(buf: &[u8]) -> Vec<u8> {
    let mut d = zune_jpeg::JpegDecoder::new(buf);
    d.decode().unwrap()
}

fn decode_png(data: &[u8]) -> Vec<u8> {
    let mut decoder = png::Decoder::new(data);
    decoder.set_transformations(png::Transformations::EXPAND);

    let mut reader = decoder.read_info().unwrap();

    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let _ = reader.next_frame(&mut buf).unwrap();

    buf
}

fn decode_zune_png(data: &[u8]) -> Vec<u8> {
    zune_png::PngDecoder::new(data).decode_raw().unwrap()
}

fn decode_jpeg_mozjpeg(buf: &[u8]) -> Vec<[u8; 3]> {
    let p = std::panic::catch_unwind(|| {
        let d = mozjpeg::Decompress::with_markers(mozjpeg::ALL_MARKERS)
            .from_mem(buf)
            .unwrap();

        // rgba() enables conversion
        let mut image = d.rgb().unwrap();

        let pixels: Vec<[u8; 3]> = image.read_scanlines().unwrap();

        image.finish().unwrap();
        pixels
    })
        .unwrap();

    p
}

fn decode_jpeg_image_rs(buf: &[u8]) -> Vec<u8> {
    let mut decoder = jpeg_decoder::Decoder::new(buf);

    decoder.decode().unwrap()
}

fn decode_jxl_oxide(buf: &[u8]) -> Vec<f32> {
    let image = jxl_oxide::JxlImage::builder().read(Cursor::new(buf)).unwrap();
    image.render_frame(0).unwrap().image().buf().to_vec()
}

fn decode_libjxl(buf: &[u8]) -> jpegxl_rs::decode::Pixels {
    jpegxl_rs::decoder_builder().build().unwrap().decode(buf).unwrap().1
}

fn decode_libavif(buf: &[u8]) -> Vec<u8> {
    libavif::decode_rgb(buf).unwrap().to_vec()
}

fn decode_webp(buf: &[u8]) -> Vec<u8> {
    webp::Decoder::new(buf).decode().unwrap().to_vec()
}

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("image decoding");
    group.bench_function("zune jpg", |b| {
        b.iter(|| black_box(decode_zune_jpg(black_box(JPEG))))
    });
    group.bench_function("png", |b| b.iter(|| black_box(decode_png(black_box(PNG)))));
    group.bench_function("zune PNG", |b| {
        b.iter(|| black_box(decode_zune_png(black_box(PNG))))
    });
    group.bench_function("mozjpg", |b| {
        b.iter(|| black_box(decode_jpeg_mozjpeg(black_box(JPEG))))
    });
    group.bench_function("jpeg-decoder", |b| {
        b.iter(|| black_box(decode_jpeg_image_rs(black_box(JPEG))))
    });
    group.bench_function("jxl-oxide slow", |b| {
        b.iter(|| black_box(decode_jxl_oxide(black_box(JXL_SLOW))))
    });
    group.bench_function("libjxl slow", |b| {
        b.iter(|| black_box(decode_libjxl(black_box(JXL_SLOW))))
    });
    group.bench_function("jxl-oxide fast", |b| {
        b.iter(|| black_box(decode_jxl_oxide(black_box(JXL_FAST))))
    });
    group.bench_function("libjxl fast", |b| {
        b.iter(|| black_box(decode_libjxl(black_box(JXL_FAST))))
    });
    group.bench_function("avif", |b| {
        b.iter(|| black_box(decode_libavif(black_box(AVIF))))
    });
    group.bench_function("webp", |b| {
        b.iter(|| black_box(decode_webp(black_box(WEBP))))
    });

    group.finish();

}

criterion_group!(name=benches;
      config={
      let c = Criterion::default();
        c.measurement_time(Duration::from_secs(20))
      };
    targets = benchmark
    );

criterion_main!(benches);
