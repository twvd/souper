use crate::frontend::NullRenderer;
use crate::snes::bus::BusMember;

use super::*;

fn ppu() -> PPU<NullRenderer> {
    PPU::<NullRenderer>::new(NullRenderer::new(0, 0).unwrap())
}

#[test]
fn vram_write_inc_low() {
    let mut p = ppu();
    p.write(0x2116, 0x34); // VMADDH
    p.write(0x2117, 0x12); // VMADDL
    p.write(0x2115, 0x00); // VMAIN - low 1 word

    assert_eq!(p.vmadd.get(), 0x1234);
    p.write(0x2119, 0xAA); // VMDATAH
    assert_eq!(p.vmadd.get(), 0x1234);
    p.write(0x2118, 0xAA); // VMDATAL
    assert_eq!(p.vmadd.get(), 0x1235);
    p.write(0x2119, 0xBB); // VMDATAH
    assert_eq!(p.vmadd.get(), 0x1235);
    p.write(0x2118, 0xBB); // VMDATAL
    assert_eq!(p.vmadd.get(), 0x1236);

    assert_eq!(p.vram[0x1233], 0x0000);
    assert_eq!(p.vram[0x1234], 0xAAAA);
    assert_eq!(p.vram[0x1235], 0xBBBB);
    assert_eq!(p.vram[0x1236], 0x0000);
}

#[test]
fn vram_write_inc_high() {
    let mut p = ppu();
    p.write(0x2116, 0x34); // VMADDH
    p.write(0x2117, 0x12); // VMADDL
    p.write(0x2115, 0x80); // VMAIN - high 1 word

    assert_eq!(p.vmadd.get(), 0x1234);
    p.write(0x2118, 0xAA); // VMDATAL
    assert_eq!(p.vmadd.get(), 0x1234);
    p.write(0x2119, 0xAA); // VMDATAH
    assert_eq!(p.vmadd.get(), 0x1235);
    p.write(0x2118, 0xBB); // VMDATAL
    assert_eq!(p.vmadd.get(), 0x1235);
    p.write(0x2119, 0xBB); // VMDATAH
    assert_eq!(p.vmadd.get(), 0x1236);

    assert_eq!(p.vram[0x1233], 0x0000);
    assert_eq!(p.vram[0x1234], 0xAAAA);
    assert_eq!(p.vram[0x1235], 0xBBBB);
    assert_eq!(p.vram[0x1236], 0x0000);
}

#[test]
fn vram_write_inc_thirtytwo() {
    let mut p = ppu();
    p.write(0x2116, 0x34); // VMADDH
    p.write(0x2117, 0x12); // VMADDL
    p.write(0x2115, 0x01); // VMAIN - low 32 words

    assert_eq!(p.vmadd.get(), 0x1234);
    p.write(0x2119, 0xAA); // VMDATAH
    assert_eq!(p.vmadd.get(), 0x1234);
    p.write(0x2118, 0xAA); // VMDATAL
    assert_eq!(p.vmadd.get(), 0x1254);
    p.write(0x2119, 0xBB); // VMDATAH
    assert_eq!(p.vmadd.get(), 0x1254);
    p.write(0x2118, 0xBB); // VMDATAL
    assert_eq!(p.vmadd.get(), 0x1274);

    assert_eq!(p.vram[0x1233], 0x0000);
    assert_eq!(p.vram[0x1234], 0xAAAA);
    assert_eq!(p.vram[0x1235], 0x0000);
    assert_eq!(p.vram[0x1253], 0x0000);
    assert_eq!(p.vram[0x1254], 0xBBBB);
    assert_eq!(p.vram[0x1255], 0x0000);
    assert_eq!(p.vram[0x1274], 0x0000);
}

#[test]
fn vram_write_inc_hundredtwentyeight() {
    let mut p = ppu();
    p.write(0x2116, 0x34); // VMADDH
    p.write(0x2117, 0x12); // VMADDL
    p.write(0x2115, 0x02); // VMAIN - low 128 words

    assert_eq!(p.vmadd.get(), 0x1234);
    p.write(0x2119, 0xAA); // VMDATAH
    assert_eq!(p.vmadd.get(), 0x1234);
    p.write(0x2118, 0xAA); // VMDATAL
    assert_eq!(p.vmadd.get(), 0x12B4);
    p.write(0x2119, 0xBB); // VMDATAH
    assert_eq!(p.vmadd.get(), 0x12B4);
    p.write(0x2118, 0xBB); // VMDATAL
    assert_eq!(p.vmadd.get(), 0x1334);

    assert_eq!(p.vram[0x1233], 0x0000);
    assert_eq!(p.vram[0x1234], 0xAAAA);
    assert_eq!(p.vram[0x1235], 0x0000);
    assert_eq!(p.vram[0x12B3], 0x0000);
    assert_eq!(p.vram[0x12B4], 0xBBBB);
    assert_eq!(p.vram[0x12B5], 0x0000);
    assert_eq!(p.vram[0x1334], 0x0000);
}

#[test]
fn cgram_write() {
    let mut p = ppu();
    p.write(0x2121, 0); // CGADD
    p.write(0x2122, 0xAA); // CGDATA
    assert_eq!(p.cgadd.get(), 0);
    p.write(0x2122, 0xBB); // CGDATA
    assert_eq!(p.cgadd.get(), 1);
    assert_eq!(p.cgram[0], 0xBBAA);
}

#[test]
fn cgram_addr_reset_flipflop() {
    let mut p = ppu();
    p.write(0x2121, 0); // CGADD
    p.write(0x2122, 0xAA); // CGDATA
    p.write(0x2121, 0); // CGADD
    p.write(0x2122, 0xBB); // CGDATA
    assert_eq!(p.cgram[0], 0xBB);
}

#[test]
fn cgram_write_overflow() {
    let mut p = ppu();
    p.write(0x2121, 0xFF); // CGADD
    p.write(0x2122, 0xAA); // CGDATA
    p.write(0x2122, 0xBB); // CGDATA
    p.write(0x2122, 0xCC); // CGDATA
    p.write(0x2122, 0xDD); // CGDATA
    assert_eq!(p.cgram[0], 0xDDCC);
    assert_eq!(p.cgram[0xFF], 0xBBAA);
}

#[test]
fn cgram_read() {
    let mut p = ppu();
    p.cgram[0] = 0xBBAA;
    p.cgram[1] = 0xDDCC;
    p.write(0x2121, 0); // CGADD
    assert_eq!(p.read(0x213B), Some(0xAA)); // RDCGRAM
    assert_eq!(p.read(0x213B), Some(0xBB)); // RDCGRAM
    assert_eq!(p.read(0x213B), Some(0xCC)); // RDCGRAM
    assert_eq!(p.read(0x213B), Some(0xDD)); // RDCGRAM
}

#[test]
fn cgram_read_overflow() {
    let mut p = ppu();
    p.cgram[255] = 0xBBAA;
    p.cgram[0] = 0xDDCC;
    p.write(0x2121, 0xFF); // CGADD
    assert_eq!(p.read(0x213B), Some(0xAA)); // RDCGRAM
    assert_eq!(p.read(0x213B), Some(0xBB)); // RDCGRAM
    assert_eq!(p.read(0x213B), Some(0xCC)); // RDCGRAM
    assert_eq!(p.read(0x213B), Some(0xDD)); // RDCGRAM
}
