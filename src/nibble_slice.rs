// use std::{ops::Index, slice::SliceIndex};

// pub struct NibbleSlice<const COUNT_HALF: usize> {
//     pub data: [u8; COUNT_HALF],
// }

// impl<const COUNT_HALF: usize> NibbleSlice<COUNT_HALF> {
//     fn get(&self, index: usize) -> u8 {
//         let base = self.data[index >> 1];

//         if (index & 1) == 0 {
//             // Even indices
//             &(base & 0x0f)
//         } else {
//             // Odd indices
//             &(base >> 4)
//         }
//     }
// }
