use std::io::{Cursor, Read};

use fnv::FnvHashMap;
use nibble_vec::NibbleVec;

use crate::render::chunk::ChunkRenderData;

pub const CHUNK_SECTION_SIZE: usize = 16 * 16 * 16;
pub const CHUNK_SIZE: usize = CHUNK_SECTION_SIZE * 16;
pub const CHUNK_SIZE_2D: usize = 16 * 16;

macro_rules! chunk_coord {
    ($block_x:expr, $block_y:expr, $block_z:expr) => {
        // ($block_x / 16, $block_y / 16, $block_z / 16)
        ($block_x >> 4, $block_y >> 4, $block_z >> 4)
    };
}

pub struct ChunkColumn {
    pub sections: [Option<ChunkSectionData>; 16],
    pub biomes: [u8; CHUNK_SIZE_2D],
}

impl ChunkColumn {
    pub fn empty() -> Self {
        const INIT: Option<ChunkSectionData> = None;
        Self {
            sections: [INIT; 16],
            biomes: [0; CHUNK_SIZE_2D],
        }
    }

    pub fn get_section_mut_or_insert(&mut self, y: u8) -> &mut ChunkSectionData {
        assert!(y < 16);

        if self.sections[y as usize].is_none() {
            self.sections[y as usize] = Some(ChunkSectionData::empty());
        }

        self.sections[y as usize].as_mut().unwrap()
    }

    pub fn get_section_mut(&mut self, y: u8) -> Option<&mut ChunkSectionData> {
        self.sections[y as usize].as_mut()
    }

    pub fn get_section(&self, y: u8) -> Option<&ChunkSectionData> {
        self.sections[y as usize].as_ref()
    }
}

pub struct ChunkSectionData {
    pub dirty: bool,
    pub renderdata: Option<ChunkRenderData>,
    pub blocks: [u8; CHUNK_SECTION_SIZE],
    // pub metadata: NibbleVec<[u8; CHUNK_SIZE]>,
    pub light: NibbleVec<[u8; CHUNK_SIZE / 2]>,
    // pub skylight: NibbleVec<[u8; CHUNK_SIZE]>,
    // pub add: NibbleVec<[u8; CHUNK_SIZE]>,
}

impl ChunkSectionData {
    pub fn empty() -> ChunkSectionData {
        ChunkSectionData {
            dirty: true,
            renderdata: None,
            blocks: [0; CHUNK_SECTION_SIZE],
            // metadata:(),
            light: NibbleVec::new(),
            // skylight: (),
            // add: (),
        }
    }

    // TODO: Dedicated position type

    /// ! This function does not check if the coordinates are inside of the chunk
    pub fn get_block(&self, x: i32, y: i32, z: i32) -> u8 {
        if y < 0 {
            return 0;
        }

        return self.blocks[(((y & 0x0f) << 8) | ((z & 0x0f) << 4) | (x & 0x0f)) as usize];
    }

    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: u8) {
        if y < 0 {
            return;
        }

        self.blocks[(((y & 0x0f) << 8) | ((z & 0x0f) << 4) | (x & 0x0f)) as usize] = block;
        self.dirty = true;
    }

    pub fn get_block_light(&self, x: i32, y: i32, z: i32) -> u8 {
        if y < 0 {
            return 0;
        }

        return self
            .light
            .get((((y & 0x0f) << 8) | ((z & 0x0f) << 4) | (x & 0x0f)) as usize);
    }

    /// Returns (up, down, left, right, front, back)
    pub fn get_neighbors(&self, x: i32, y: i32, z: i32) -> (bool, bool, bool, bool, bool, bool) {
        macro_rules! nb {
            ($x:expr, $y:expr, $z:expr) => {{
                let (tx, ty, tz) = (x + $x, y + $y, z + $z);

                if tx < 0 || ty < 0 || tz < 0 {
                    false
                } else if tx > 15 || ty > 15 || tz > 15 {
                    false
                } else {
                    self.get_block(tx, ty, tz) != 0
                }
            }};
        }

        (
            nb!(0, 1, 0),  // Up
            nb!(0, -1, 0), // Down
            nb!(-1, 0, 0), // Left
            nb!(1, 0, 0),  // Right
            nb!(0, 0, -1), // Front
            nb!(0, 0, 1),  // Back
        )
        // (false, false, false, false, false, false)
    }
}

pub struct ChunkManager {
    pub chunks: FnvHashMap<(i32, i32), ChunkColumn>,
}

impl ChunkManager {
    pub fn new() -> Self {
        Self {
            chunks: FnvHashMap::default(),
        }
    }

    // @return the amount of bytes read from the data buffer
    pub fn load_chunk(
        &mut self,
        coords: (i32, i32),
        bitmask: u16,
        bitmask_add: u16,
        skylight: bool,
        ground_up_continuous: bool,
        data: &[u8],
    ) -> anyhow::Result<u64> {
        let chunk = if let Some(c) = self.chunks.get_mut(&coords) {
            c
        } else {
            self.chunks.insert(coords, ChunkColumn::empty());
            self.chunks.get_mut(&coords).unwrap()
        };

        let mut cur = Cursor::new(data);
        for i in 0..16 {
            if bitmask & (1 << i) == 0 {
                continue;
            }

            let mut block_types = vec![0u8; CHUNK_SECTION_SIZE];
            cur.read_exact(&mut block_types)?;

            let s = chunk.get_section_mut_or_insert(i as u8);
            s.dirty = true;
            s.blocks[..].copy_from_slice(&block_types);
        }

        for i in 0..16 {
            if bitmask & (1 << i) == 0 {
                continue;
            }
            // TODO: Nibble array type
            let mut block_metadata = vec![0u8; CHUNK_SECTION_SIZE / 2];
            cur.read_exact(&mut block_metadata)?;
        }

        for i in 0..16 {
            if bitmask & (1 << i) == 0 {
                continue;
            }
            // TODO: Nibble array type
            let mut block_light = vec![0u8; CHUNK_SECTION_SIZE / 2];
            cur.read_exact(&mut block_light)?;

            let s = chunk.get_section_mut_or_insert(i as u8);
            s.dirty = true;
            s.light = NibbleVec::from(block_light);
        }

        if skylight {
            for i in 0..16 {
                if bitmask & (1 << i) == 0 {
                    continue;
                }
                // TODO: Nibble array type
                let mut sky_light = vec![0u8; CHUNK_SECTION_SIZE / 2];
                cur.read_exact(&mut sky_light)?;
            }
        }

        for i in 0..16 {
            if bitmask_add & (1 << i) == 0 {
                continue;
            }
            // TODO: Nibble array type
            let mut block_add = vec![0u8; CHUNK_SECTION_SIZE / 2];
            cur.read_exact(&mut block_add)?;
        }

        if ground_up_continuous {
            let mut biomes = vec![0u8; CHUNK_SIZE_2D];
            cur.read_exact(&mut biomes)?;
            chunk.biomes[..].copy_from_slice(&biomes);
        }

        Ok(cur.position())
    }

    pub fn get(&self, coords: &(i32, i32)) -> Option<&ChunkColumn> {
        self.chunks.get(coords)
    }

    pub fn get_mut(&mut self, coords: &(i32, i32)) -> Option<&mut ChunkColumn> {
        self.chunks.get_mut(coords)
    }

    pub fn get_block(&self, bx: i32, by: i32, bz: i32) -> u8 {
        let ccoord = chunk_coord!(bx, by, bz);
        if let Some(chunk) = self.get(&(ccoord.0, ccoord.2)) {
            if let Some(Some(section)) = chunk.sections.get(ccoord.1 as usize) {
                section.get_block(bx, by, bz)
            } else {
                0
            }
        } else {
            0
        }
    }

    pub fn set_block(&mut self, bx: i32, by: i32, bz: i32, block: u8) {
        let ccoord = chunk_coord!(bx, by, bz);
        let (rx, ry, rz) = (bx % 16, by % 16, bz % 16);
        if let Some(chunk) = self.get_mut(&(ccoord.0, ccoord.2)) {
            if let Some(section) = chunk.get_section_mut(ccoord.1 as u8) {
                section.set_block(bx, by, bz, block);
            }

            if ry == 0 {
                chunk
                    .get_section_mut((ccoord.1 - 1) as u8)
                    .map(|c| c.dirty = true);
            }

            if ry == 15 {
                chunk
                    .get_section_mut((ccoord.1 + 1) as u8)
                    .map(|c| c.dirty = true);
            }
        }

        if rx == 0 {
            self.get_mut(&(ccoord.0 - 1, ccoord.2))
                .map(|c| c.get_section_mut(ccoord.1 as u8).map(|c| c.dirty = true));
        }

        if rx == 15 {
            self.get_mut(&(ccoord.0 + 1, ccoord.2))
                .map(|c| c.get_section_mut(ccoord.1 as u8).map(|c| c.dirty = true));
        }

        if rz == 0 {
            self.get_mut(&(ccoord.0, ccoord.2 - 1))
                .map(|c| c.get_section_mut(ccoord.1 as u8).map(|c| c.dirty = true));
        }

        if rz == 15 {
            self.get_mut(&(ccoord.0, ccoord.2 + 1))
                .map(|c| c.get_section_mut(ccoord.1 as u8).map(|c| c.dirty = true));
        }
    }

    pub fn get_block_light(&self, coords: (i32, i32, i32)) -> u8 {
        let ccoord = chunk_coord!(coords.0, coords.1, coords.2);
        if let Some(chunk) = self.get(&(ccoord.0, ccoord.2)) {
            if let Some(Some(section)) = chunk.sections.get(ccoord.1 as usize) {
                section.get_block_light(coords.0, coords.1, coords.2)
            } else {
                0
            }
        } else {
            0
        }
    }

    /// Returns (up, down, left, right, front, back)
    pub fn get_neighbors(&self, x: i32, y: i32, z: i32) -> (bool, bool, bool, bool, bool, bool) {
        macro_rules! nb {
            ($x:expr, $y:expr, $z:expr) => {{
                let (tx, ty, tz) = (x + $x, y + $y, z + $z);

                self.get_block(tx, ty, tz) != 0
            }};
        }

        (
            nb!(0, 1, 0),  // Up
            nb!(0, -1, 0), // Down
            nb!(-1, 0, 0), // Left
            nb!(1, 0, 0),  // Right
            nb!(0, 0, -1), // Front
            nb!(0, 0, 1),  // Back
        )
        // (false, false, false, false, false, false)
    }
}
