pub mod bus;

#[cfg(test)]
pub mod tests;

use std::cell::Cell;

type VramWord = u16;
const VRAM_WORDS: usize = 32 * 1024;
const VRAM_WORDSIZE: usize = 2;
// 32K-words addressable (64KB)
const VRAM_ADDRMASK: usize = 0x7FFF;

// VMAIN bits
const VMAIN_HIGH: u8 = 1 << 7;
const VMAIN_INC_MASK: u8 = 0x03;
const VMAIN_TRANSLATE_MASK: u8 = 0x03;
const VMAIN_TRANSLATE_SHIFT: u8 = 2;

pub struct PPU {
    vram: Vec<VramWord>,
    vmadd: Cell<u16>,
    vmain: u8,

    bgmode: u8,
    bgxsc: [u8; 4],
}

impl PPU {
    pub fn new() -> Self {
        Self {
            vram: vec![0; VRAM_WORDS * VRAM_WORDSIZE],
            vmadd: Cell::new(0),
            vmain: 0,
            bgmode: 0,
            bgxsc: [0, 0, 0, 0],
        }
    }

    fn vram_autoinc(&self, upper: bool) {
        let inc_on_upper = self.vmain & VMAIN_HIGH != 0;
        if upper != inc_on_upper {
            return;
        }

        // TODO prefetch BEFORE increment
        let inc = match self.vmain & VMAIN_INC_MASK {
            0 => 1,
            1 => 32,
            2..=3 => 128,
            _ => unreachable!(),
        };
        self.vmadd.set(self.vmadd.get().wrapping_add(inc));
    }

    fn vram_addr_translate(&self, addr: u16) -> u16 {
        // Translation  Bitmap Type              Port [2116h/17h]    VRAM Word-Address
        //  8bit rotate  4-color; 1 word/plane    aaaaaaaaYYYxxxxx --> aaaaaaaaxxxxxYYY
        //  9bit rotate  16-color; 2 words/plane  aaaaaaaYYYxxxxxP --> aaaaaaaxxxxxPYYY
        // 10bit rotate 256-color; 4 words/plane aaaaaaYYYxxxxxPP --> aaaaaaxxxxxPPYYY
        match (self.vmain & VMAIN_TRANSLATE_MASK) >> VMAIN_TRANSLATE_SHIFT {
            0 => addr,
            1..=3 => todo!(),
            _ => unreachable!(),
        }
    }

    fn get_bg_map_addr(&self, bg: usize) -> usize {
        assert!((0..4).contains(&bg));
        (self.bgxsc[bg] >> 2) as usize * 2
    }
}
