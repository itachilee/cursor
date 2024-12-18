use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::prelude::*;
fn compress_text(text: &str) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(text.as_bytes()).unwrap();
    encoder.finish().unwrap()
}

fn decompress_text(compressed: &[u8]) -> String {
    let mut decoder = flate2::read::GzDecoder::new(compressed);
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed).unwrap();
    decompressed
}

fn test_gzip() {
    let text = "";
    let compressed = compress_text(text);
    println!("原始文本长度: {} 字节", text.len());
    println!("压缩后长度: {} 字节", compressed.len());

    let decompressed = decompress_text(&compressed);
    println!("解压后文本: {}", decompressed);
}
