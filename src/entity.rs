use std::time::SystemTime;

#[derive(Debug)]
pub struct Entity {
    pub version: String,
    pub header_chunk: HeaderChunk,
    pub data_chunks: Vec<DataChunk>,
    pub other_chunks: Vec<Chunk>,
}

#[derive(Debug)]
pub struct Chunk {
    pub code: String,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct HeaderAttibute {
    pub key: String,
    pub val: Vec<u8>,
}

#[derive(Debug)]
pub struct HeaderChunk {
    pub creation_date: Option<SystemTime>,
    pub width: u16,
    pub height: u16,
    pub other_attributes: Vec<HeaderAttibute>,
}

#[derive(Debug)]
pub struct PathObject {
    pub color: [u8; 3],
    pub points: Vec<[u16; 2]>,
}

#[derive(Debug)]
pub enum Object {
    Path(PathObject),
}

#[derive(Debug)]
pub struct DataChunk {
    pub objects: Vec<Object>,
}
