use oxels::{AnyImage, load_meta_image};


fn assert_sum7203(path: &str) {
    let image: Box<dyn AnyImage> = load_meta_image(path);
    let sum: f64 = image.iter_f64().sum();
    assert!(
        (sum - 7203.0).abs() < 1e-6,
        "Expected sum 7203.0 for {}, got {}",
        path,
        sum
    );
}


#[test]
fn assert_sum7203_uint8_uncompressed() {
    assert_sum7203("assets/tensors/uint8_uncompressed.mha");
}

#[test]
fn assert_sum7203_int8_uncompressed() {
    assert_sum7203("assets/tensors/int8_uncompressed.mha");
}

#[test]
fn assert_sum7203_uint16_uncompressed() {
    assert_sum7203("assets/tensors/uint16_uncompressed.mha");
}

#[test]
fn assert_sum7203_int16_uncompressed() {
    assert_sum7203("assets/tensors/int16_uncompressed.mha");
}

#[test]
fn assert_sum7203_uint32_uncompressed() {
    assert_sum7203("assets/tensors/uint32_uncompressed.mha");
}

#[test]
fn assert_sum7203_int32_uncompressed() {
    assert_sum7203("assets/tensors/int32_uncompressed.mha");
}

#[test]
fn assert_sum7203_float32_uncompressed() {
    assert_sum7203("assets/tensors/float32_uncompressed.mha");
}

#[test]
fn assert_sum7203_uint64_uncompressed() {
    assert_sum7203("assets/tensors/uint64_uncompressed.mha");
}

#[test]
fn assert_sum7203_int64_uncompressed() {
    assert_sum7203("assets/tensors/int64_uncompressed.mha");
}

#[test]
fn assert_sum7203_float64_uncompressed() {
    assert_sum7203("assets/tensors/float64_uncompressed.mha");
}
