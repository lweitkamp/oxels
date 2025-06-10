use oxels::{Image, AnyImage, load_meta_image, save_meta_image};
use std::fs::remove_file;


fn assert_sum7203(path: &str) {
    let image: Box<dyn AnyImage> = load_meta_image(path);
    let sum: f64 = image.iter_f64().sum();
    assert_eq!(sum, 7203.0);
}


#[test]
fn assert_sum7203_uint8_uncompressed() {
    assert_sum7203("assets/tensors/uint8_uncompressed.mhd");
}

#[test]
fn assert_sum7203_int8_uncompressed() {
    assert_sum7203("assets/tensors/int8_uncompressed.mhd");
}

#[test]
fn assert_sum7203_uint16_uncompressed() {
    assert_sum7203("assets/tensors/uint16_uncompressed.mhd");
}

#[test]
fn assert_sum7203_int16_uncompressed() {
    assert_sum7203("assets/tensors/int16_uncompressed.mhd");
}

#[test]
fn assert_sum7203_uint32_uncompressed() {
    assert_sum7203("assets/tensors/uint32_uncompressed.mhd");
}

#[test]
fn assert_sum7203_int32_uncompressed() {
    assert_sum7203("assets/tensors/int32_uncompressed.mhd");
}

#[test]
fn assert_sum7203_float32_uncompressed() {
    assert_sum7203("assets/tensors/float32_uncompressed.mhd");
}

#[test]
fn assert_sum7203_uint64_uncompressed() {
    assert_sum7203("assets/tensors/uint64_uncompressed.mhd");
}

#[test]
fn assert_sum7203_int64_uncompressed() {
    assert_sum7203("assets/tensors/int64_uncompressed.mhd");
}

#[test]
fn assert_sum7203_float64_uncompressed() {
    assert_sum7203("assets/tensors/float64_uncompressed.mhd");
}

#[test]
fn assert_equal_with_compression() {
    let compressed: Box<dyn AnyImage> = load_meta_image("assets/tensors/uint16_compressed.mhd");
    let uncompressed: Box<dyn AnyImage> = load_meta_image("assets/tensors/uint16_uncompressed.mhd");

        for (a, b) in compressed.iter_f64().zip(uncompressed.iter_f64()) {
            assert_eq!(a, b);
        }
}

#[test]
fn roundtrip_save_and_load_uint16() {
    // Create a simple 2x2x2 image with known values
    let voxels = vec![1u16, 2, 3, 4, 5, 6, 7, 8];
    let img = Image {
        voxels: voxels.clone(),
        width: 2,
        height: 2,
        depth: 2,
        spacing: (1.0, 1.0, 1.0),
        origin: (0.0, 0.0, 0.0),
        direction: (1,0,0,0,1,0,0,0,1),
    };
    let path = "test_uint16.mhd";
    save_meta_image(&img, path).unwrap();

    // Load it back
    let loaded: Box<dyn AnyImage> = oxels::load_meta_image(path);

    // Check metadata
    assert_eq!(loaded.width(), 2);
    assert_eq!(loaded.height(), 2);
    assert_eq!(loaded.depth(), 2);
    assert_eq!(loaded.spacing(), (1.0, 1.0, 1.0));
    assert_eq!(loaded.origin(), (0.0, 0.0, 0.0));
    assert_eq!(loaded.direction(), (1,0,0,0,1,0,0,0,1));

    // Check voxel values
    let loaded_voxels: Vec<u16> = loaded.iter_f64().map(|v| v as u16).collect();
    assert_eq!(loaded_voxels, voxels);

    // Clean up
    remove_file(path).unwrap();
    remove_file(path.replace(".mhd", ".zraw")).unwrap();
}