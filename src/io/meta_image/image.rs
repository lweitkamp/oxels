use crate::image::{Image, AnyImage};

use super::header::{HeaderError, parse_header};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use flate2::read::ZlibDecoder;

use bytemuck::{cast_slice, Pod};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ElementType { // TODO: bool support?
    I8, U8,
    I16, U16,
    I32, U32, F32,
    I64, U64, F64,
}

impl std::str::FromStr for ElementType {
    type Err = HeaderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ElementType::*;
        match s {
            "MET_UCHAR"  => Ok(U8),
            "MET_CHAR"   => Ok(I8),
            "MET_USHORT" => Ok(U16),
            "MET_SHORT"  => Ok(I16),
            "MET_UINT" => Ok(U32),
            "MET_INT" => Ok(I32),
            "MET_FLOAT"  => Ok(F32),
            "MET_ULONG_LONG" => Ok(U64),
            "MET_LONG_LONG" => Ok(I64),
            "MET_DOUBLE" => Ok(F64),
            _ => Err(HeaderError::UnsupportedElementType(s.into())),
        }
    }
}

impl ElementType {
    fn bytes(self) -> usize {
        use ElementType::*;
        match self {
            U8 | I8 => 1,
            U16 | I16 => 2,
            U32 | I32 | F32 => 4,
            U64 | I64 | F64 => 8,
        }
    }
}

// Read raw byte vector and covert to type T.
fn bytes_to_vec<T: Pod>(raw: Vec<u8>) -> Result<Vec<T>, &'static str> {
    if raw.len() % std::mem::size_of::<T>() != 0 {
        return Err("byte count not divisible by element size");
    }
    let slice_t: &[T] = cast_slice(&raw);
    Ok(slice_t.to_vec())
}


pub fn load_meta_image(filename: &str) -> Box<dyn AnyImage> {
    let header = parse_header(filename).expect("Invalid Meta Image");

    // First, get the voxel count in the image.
    let (width, height, depth) = header.dim_size;
    let voxel_count = (width as usize) * (height as usize) * (depth as usize);

    // Next, init a vector with total byte size (voxels * data type)
    let etype: ElementType = header.element_type.parse().expect("Unsupported Element type!");
    let total_bytes = etype.bytes() * voxel_count;
    let mut buffer = vec![0u8; total_bytes];

    // EDF LOCAL implies next filepath bytes is data, otherwise data is
    // found in a different file pointed to by EDF.
    let data_filename: String;
    let data_offset: u64;
    if header.element_data_file == "LOCAL" {
        data_offset = header.data_offset;
        data_filename = filename.to_string();
    } else {
        data_offset = 0;
        let base_path = Path::new(filename)
            .parent()
            .expect("Failed to get parent directory of filename");
        data_filename = base_path.join(&header.element_data_file).to_str().unwrap().to_string();
    }

    // Now just load the file and get bytes from the offset.
    let mut f = File::open(&data_filename).expect("Failed to open MetaImage file");
    f.seek(SeekFrom::Start(data_offset))
        .expect("Failed to seek to data offset");

    // If file is compressed we have to read _all_ bytes in file, otherwise we can do exact reads.
    if header.compressed_data {
        let mut compressed = Vec::new();
        f.read_to_end(&mut compressed).expect("Failed to read compressed voxel data");
        let mut decoder = ZlibDecoder::new(&compressed[..]);
        let mut decompressed = Vec::with_capacity(total_bytes);
        decoder
            .read_to_end(&mut decompressed)
            .expect("Failed to decompress voxel data");
        buffer = decompressed;
    } else {
        f.read_exact(&mut buffer).expect("Failed to read voxel data");
    }

    match etype {
        ElementType::U8 => {
            let vox_vec: Vec<u8> = buffer; // already Vec<u8>
            let image = Image::<u8> {
                voxels: vox_vec,
                width: width as u32,
                height: height as u32,
                depth: depth as u32,
                spacing: header.element_spacing,
                origin: header.offset,
                direction: header.transform_matrix,
            };
            Box::new(image)
        }
        ElementType::I8 => {
            let vox_vec: Vec<i8> = buffer.iter().map(|&b| b as i8).collect();
            let image = Image::<i8> {
                voxels: vox_vec,
                width: width as u32,
                height: height as u32,
                depth: depth as u32,
                spacing: header.element_spacing,
                origin: header.offset,
                direction: header.transform_matrix,
            };
            Box::new(image)
        }
        ElementType::I16 => {
            let vox_vec: Vec<i16> = bytes_to_vec::<i16>(buffer).expect("Bad byte count");
            let image = Image::<i16> {
                voxels: vox_vec,
                width: width as u32,
                height: height as u32,
                depth: depth as u32,
                spacing: header.element_spacing,
                origin: header.offset,
                direction: header.transform_matrix,
            };
            Box::new(image)
        }
        ElementType::U16 => {
            let vox_vec: Vec<u16> = bytes_to_vec::<u16>(buffer).expect("Bad byte count");
            let image = Image::<u16> {
                voxels: vox_vec,
                width: width as u32,
                height: height as u32,
                depth: depth as u32,
                spacing: header.element_spacing,
                origin: header.offset,
                direction: header.transform_matrix,
            };
            Box::new(image)
        }
        ElementType::I32 => {
            let vox_vec: Vec<i32> = bytes_to_vec::<i32>(buffer).expect("Bad byte count");
            let image = Image::<i32> {
                voxels: vox_vec,
                width: width as u32,
                height: height as u32,
                depth: depth as u32,
                spacing: header.element_spacing,
                origin: header.offset,
                direction: header.transform_matrix,
            };
            Box::new(image)
        }
        ElementType::U32 => {
            let vox_vec: Vec<u32> = bytes_to_vec::<u32>(buffer).expect("Bad byte count");
            let image = Image::<u32> {
                voxels: vox_vec,
                width: width as u32,
                height: height as u32,
                depth: depth as u32,
                spacing: header.element_spacing,
                origin: header.offset,
                direction: header.transform_matrix,
            };
            Box::new(image)
        }
        ElementType::I64 => {
            let vox_vec: Vec<i64> = bytes_to_vec::<i64>(buffer).expect("Bad byte count");
            let image = Image::<i64> {
                voxels: vox_vec,
                width: width as u32,
                height: height as u32,
                depth: depth as u32,
                spacing: header.element_spacing,
                origin: header.offset,
                direction: header.transform_matrix,
            };
            Box::new(image)
        }
        ElementType::U64 => {
            let vox_vec: Vec<u64> = bytes_to_vec::<u64>(buffer).expect("Bad byte count");
            let image = Image::<u64> {
                voxels: vox_vec,
                width: width as u32,
                height: height as u32,
                depth: depth as u32,
                spacing: header.element_spacing,
                origin: header.offset,
                direction: header.transform_matrix,
            };
            Box::new(image)
        }
        ElementType::F32 => {
            let vox_vec: Vec<f32> = bytes_to_vec::<f32>(buffer).expect("Bad byte count");
            let image = Image::<f32> {
                voxels: vox_vec,
                width: width as u32,
                height: height as u32,
                depth: depth as u32,
                spacing: header.element_spacing,
                origin: header.offset,
                direction: header.transform_matrix,
            };
            Box::new(image)
        }
        ElementType::F64 => {
            let vox_vec: Vec<f64> = bytes_to_vec::<f64>(buffer).expect("Bad byte count");
            let image = Image::<f64> {
                voxels: vox_vec,
                width: width as u32,
                height: height as u32,
                depth: depth as u32,
                spacing: header.element_spacing,
                origin: header.offset,
                direction: header.transform_matrix,
            };
            Box::new(image)
        }
    }
}