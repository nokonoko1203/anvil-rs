use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

struct Region {
    data: Vec<u8>,
}

impl Region {
    fn new(data: Vec<u8>) -> Region {
        Region { data }
    }

    fn header_offset(chunk_x: i32, chunk_z: i32) -> usize {
        4 * ((chunk_x % 32) as usize + (chunk_z % 32) as usize * 32)
    }

    fn chunk_location(&self, chunk_x: i32, chunk_z: i32) -> (usize, u8) {
        let b_off = Region::header_offset(chunk_x, chunk_z);
        let off = u32::from_be_bytes([
            self.data[b_off],
            self.data[b_off + 1],
            self.data[b_off + 2],
            0,
        ]) as usize;
        let sectors = self.data[b_off + 3];
        (off, sectors)
    }
}
