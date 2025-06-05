use std::fs::File;
use std::io::{BufRead, BufReader};

// Raw header for parsing
#[derive(Debug, Default)]
struct RawHeader {
    object_type: Option<String>,
    n_dims: Option<u8>,
    binary_data: Option<bool>,
    binary_data_byte_order_msb: Option<bool>,
    compressed_data: Option<bool>,
    transform_matrix: Option<(u8, u8, u8, u8, u8, u8, u8, u8, u8)>,
    offset: Option<(f64, f64, f64)>,
    center_of_rotation: Option<(f64, f64, f64)>,
    anatomical_orientation: Option<String>,
    element_spacing: Option<(f64, f64, f64)>,
    dim_size: Option<(u32, u32, u32)>,
    element_type: Option<String>,
    element_data_file: Option<String>,

    // This one is not part of the Standard but we need it for .mha files.
    data_offset: Option<u64>,
}

// filled header for image bytes retrieval
#[derive(Debug)]
pub(super) struct Header {
    // pub object_type: String,
    // pub n_dims: u8,
    // pub binary_data: bool,
    // pub binary_data_byte_order_msb: bool,
    pub compressed_data: bool,
    pub transform_matrix: (u8, u8, u8, u8, u8, u8, u8, u8, u8),
    pub offset: (f64, f64, f64),
    // pub center_of_rotation: (f64, f64, f64),
    // pub anatomical_orientation: String,
    pub element_spacing: (f64, f64, f64),
    pub dim_size: (u32, u32, u32),
    pub element_type: String,
    pub element_data_file: String,
    pub data_offset: u64,
}

#[derive(Debug)]
pub(super) enum HeaderError {
    Missing(&'static str),  // Missing key-values that we need
    Parse(std::io::Error),  // Generic Parsing error (file issue)
    UnsupportedElementType(String),
}

impl std::fmt::Display for HeaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeaderError::Missing(key) => write!(f, "Missing header key: {}", key),
            HeaderError::Parse(e) => write!(f, "Parse error: {}", e),
            HeaderError::UnsupportedElementType(e) => write!(f, "Unsupported element type: {}", e),
        }
    }
}
impl std::error::Error for HeaderError {}


impl From<std::io::Error> for HeaderError {
    fn from(e: std::io::Error) -> Self { HeaderError::Parse(e) }
}

impl TryFrom<RawHeader> for Header {
    type Error = HeaderError;

    fn try_from(r: RawHeader) -> Result<Self, Self::Error> {
        Ok(Header {
            // object_type: r.object_type.ok_or(HeaderError::Missing("ObjectType"))?,
            // n_dims: r.n_dims.ok_or(HeaderError::Missing("NDims"))?,
            // binary_data: r.binary_data.ok_or(HeaderError::Missing("BinaryData"))?,
            // binary_data_byte_order_msb: r.binary_data_byte_order_msb.ok_or(HeaderError::Missing("BinaryDataByteOrderMSB"))?,
            compressed_data: r.compressed_data.ok_or(HeaderError::Missing("CompressedData"))?,
            transform_matrix: r.transform_matrix.ok_or(HeaderError::Missing("TransformMatrix"))?,
            offset: r.offset.ok_or(HeaderError::Missing("Offset"))?,
            // center_of_rotation: r.center_of_rotation.ok_or(HeaderError::Missing("CenterOfRotation"))?,
            // anatomical_orientation: r.anatomical_orientation.ok_or(HeaderError::Missing("AnatomicalOrientation"))?,
            element_spacing: r.element_spacing.ok_or(HeaderError::Missing("ElementSpacing"))?,
            dim_size: r.dim_size.ok_or(HeaderError::Missing("DimSize"))?,
            element_type: r.element_type.ok_or(HeaderError::Missing("ElementType"))?,
            element_data_file: r.element_data_file.ok_or(HeaderError::Missing("ElementDataFile"))?,
            data_offset: r.data_offset.ok_or(HeaderError::Missing("Data Offset? weird!"))?,
        })
    }
}


fn parse_bool(value: &str) -> Option<bool> {
    match value.to_lowercase().as_str() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}


pub(super) fn parse_header(filename: &str) -> Result<Header, HeaderError> {
    let mut raw = RawHeader::default();
    let mut data_offset: u64 = 0;

    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut line_buffer = String::new();

    loop {
        line_buffer.clear();
        let bytes_this_line = reader.read_line(&mut line_buffer)?;

        if let Some((key, value)) = line_buffer.trim().split_once('=') {
            let key = key.trim();
            let value = value.trim();

            match key {
                "ObjectType" => raw.object_type = Some(value.to_string()),
                "NDims" => raw.n_dims = value.parse().ok(),
                "BinaryData" => raw.binary_data = parse_bool(value),
                "BinaryDataByteOrderMSB" => raw.binary_data_byte_order_msb = parse_bool(value),
                "CompressedData" => raw.compressed_data = parse_bool(value),
                "TransformMatrix" => {
                    let mut iter = value.split_whitespace().map(|s| s.parse::<u8>().unwrap());
                    raw.transform_matrix = Some((
                        iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(),
                        iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(),
                        iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(),
                    ));
                },
                "Offset" => {
                    let mut iter = value.split_whitespace().map(|s| s.parse::<f64>().unwrap());
                    raw.offset = Some((
                        iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(),
                    ));
                },
                "CenterOfRotation" => {
                    let mut iter = value.split_whitespace().map(|s| s.parse::<f64>().unwrap());
                    raw.center_of_rotation = Some((
                        iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(),
                    ));
                },
                "AnatomicalOrientation" => raw.anatomical_orientation = Some(value.to_string()),
                "ElementSpacing" => {
                    let mut iter = value.split_whitespace().map(|s| s.parse::<f64>().unwrap());
                    raw.element_spacing = Some((
                        iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(),
                    ));
                },
                "DimSize" => {
                    let mut iter = value.split_whitespace().map(|s| s.parse::<u32>().unwrap());
                    raw.dim_size = Some((
                        iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(),
                    ));
                },
                "ElementType" => raw.element_type = Some(value.to_string()),
                "ElementDataFile" => raw.element_data_file = Some(value.to_string()),
                _ => {
                    // Unknown key. Shouldn't happen?
                }
            }
            data_offset += bytes_this_line as u64;
        } else {
            break; // Start of binary data.
        }
    }

    raw.data_offset = Some(data_offset);

    Header::try_from(raw)
}
