use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::regs::{Flag, Register, RegisterFile};

use crate::tickable::{Tickable, Ticks};

pub type GsuAddress = u32;
pub const GSU_ADDRESS_MASK: GsuAddress = 0xFFFFFF;
fn gsu_addr_add(addr: GsuAddress, i: GsuAddress) -> GsuAddress {
    (addr & 0xFF0000) | (addr.wrapping_add(i) & 0xFFFF)
}

#[derive(Serialize, Deserialize)]
pub enum GsuBus {
    ROM,
    RAM,
    Cache,
}

/// SuperFX CPU (GSU)
#[derive(Serialize, Deserialize)]
pub struct CpuGsu {
    pub regs: RegisterFile,
    pub cycles: Ticks,
    pub cache: Vec<u8>,
    pub rom: Vec<u8>,
    pub ram: Vec<u8>,

    sreg: usize,
    dreg: usize,
}

impl CpuGsu {
    pub fn new(rom: &[u8]) -> Self {
        let mut c = Self {
            regs: RegisterFile::new(),
            cycles: 0,
            cache: vec![0; 512],
            rom: vec![0xFF; 8 * 1024 * 1024],
            ram: vec![0xFF; 256 * 1024],
            sreg: 0,
            dreg: 0,
        };

        c.rom[0..rom.len()].copy_from_slice(rom);
        c
    }

    pub fn determine_bus(&self, fulladdr: GsuAddress) -> GsuBus {
        let (bank, addr) = ((fulladdr >> 16) as usize, (fulladdr & 0xFFFF) as usize);

        // TODO cache
        match (bank & !0x80, addr) {
            (0x00..=0x3F, 0x8000..=0xFFFF) => GsuBus::ROM,
            (0x40..=0x5F, _) => GsuBus::ROM,
            (0x70..=0x71, _) => GsuBus::RAM,
            _ => panic!("Unmapped address"),
        }
    }

    pub fn read_bus(&self, fulladdr: GsuAddress) -> u8 {
        let (bank, addr) = ((fulladdr >> 16) as usize, (fulladdr & 0xFFFF) as usize);

        // TODO bus access clear check

        // TODO cache
        match (bank & !0x80, addr) {
            (0x00..=0x3F, 0x8000..=0xFFFF) => self.rom[addr - 0x8000 + bank * 0x8000],
            (0x40..=0x5F, _) => self.rom[(bank - 0x40) * 0x10000 + addr],
            (0x70..=0x71, _) => self.ram[(bank - 0x70) * 0x10000 + addr],
            _ => panic!("Unmapped address"),
        }
    }

    pub fn read16_bus(&self, fulladdr: GsuAddress) -> u16 {
        (self.read_bus(fulladdr) as u16) | ((self.read_bus(gsu_addr_add(fulladdr, 1)) as u16) << 8)
    }

    fn fetch(&mut self) -> u8 {
        let pc_bank = GsuAddress::from(self.regs.read(Register::PBR)) << 16;
        let pc = pc_bank | GsuAddress::from(self.regs.read_inc(Register::R15));
        self.read_bus(pc)
    }

    fn fetch16(&mut self) -> u16 {
        let lo = self.fetch() as u16;
        let hi = self.fetch() as u16;
        lo | (hi << 8)
    }

    pub fn step(&mut self) -> Result<()> {
        let instr = self.fetch();

        // Note: ALTx is ignored if the opcode following does not
        // require ALTx.
        let alt1 = self.regs.test_flag(Flag::ALT1);
        let alt2 = self.regs.test_flag(Flag::ALT2);
        self.regs
            .write_flags(&[(Flag::ALT1, false), (Flag::ALT2, false)]);

        let sreg = self.sreg;
        let dreg = self.dreg;
        // SREG/DREG are reset after execution, but should persist
        // for branch instructions.
        self.sreg = 0;
        self.dreg = 0;

        match (instr, alt1, alt2) {
            (0x00, _, _) => {
                // STOP
                self.regs.write_flags(&[(Flag::G, false)]);
                self.cycles(3, 3, 1)?;
            }
            (0x01, _, _) => {
                // NOP
                self.cycles(3, 3, 1)?;
            }
            (0x10..=0x1F, _, _) => {
                // TO
                let reg = (instr & 0x0F) as usize;
                self.dreg = reg;
                self.cycles(3, 3, 1)?;
            }
            (0x20..=0x2F, _, _) => {
                // WITH
                let reg = (instr & 0x0F) as usize;
                self.sreg = reg;
                self.dreg = reg;
                // cycles unknown, assumed 3/3/1
                self.cycles(3, 3, 1)?;
            }
            (0x3D, _, _) => {
                // ALT1
                self.regs.write_flags(&[(Flag::ALT1, true)]);

                // Prefix opcodes preserve SReg/DReg
                self.sreg = sreg;
                self.dreg = dreg;

                self.cycles(3, 3, 1)?;
            }
            (0x3E, _, _) => {
                // ALT2
                self.regs.write_flags(&[(Flag::ALT2, true)]);

                // Prefix opcodes preserve SReg/DReg
                self.sreg = sreg;
                self.dreg = dreg;

                self.cycles(3, 3, 1)?;
            }
            (0x3F, _, _) => {
                // ALT3
                self.regs
                    .write_flags(&[(Flag::ALT1, true), (Flag::ALT2, true)]);

                // Prefix opcodes preserve SReg/DReg
                self.sreg = sreg;
                self.dreg = dreg;

                self.cycles(3, 3, 1)?;
            }
            (0x4F, _, _) => {
                // NOT
                let result = !self.regs.read_r(sreg);
                self.regs.write_r(dreg, result);
                self.regs
                    .write_flags(&[(Flag::Z, result == 0), (Flag::S, result & 0x8000 != 0)]);
                self.cycles(3, 3, 1)?;
            }
            (0x70, _, _) => {
                // MERGE
                let result = (self.regs.read(Register::R7) & 0xFF00)
                    .wrapping_add(self.regs.read(Register::R8) / 0x0100);
                self.regs.write_r(dreg, result);
                self.regs.write_flags(&[
                    (Flag::S, result & 0x8080 != 0),
                    (Flag::V, result & 0xC0C0 != 0),
                    (Flag::C, result & 0xE0E0 != 0),
                    (Flag::Z, result & 0xF0F0 != 0),
                ]);
                self.cycles(3, 3, 1)?;
            }
            (0x71..=0x7F, false, false) => {
                // AND r#
                let s2reg = (instr & 0x0F) as usize;
                let s1 = self.regs.read_r(sreg);
                let s2 = self.regs.read_r(s2reg);

                let result = s1 & s2;
                self.regs.write_r(dreg, result);
                self.regs
                    .write_flags(&[(Flag::Z, result == 0), (Flag::S, result & 0x8000 != 0)]);
                self.cycles(3, 3, 1)?;
            }
            (0x71..=0x7F, false, true) => {
                // AND #
                let s1 = self.regs.read_r(sreg);
                let s2 = (instr & 0x0F) as u16;

                let result = s1 & s2;
                self.regs.write_r(dreg, result);
                self.regs
                    .write_flags(&[(Flag::Z, result == 0), (Flag::S, result & 0x8000 != 0)]);
                self.cycles(6, 6, 2)?;
            }
            (0x95, _, _) => {
                // SEX
                let s = self.regs.read_r(sreg) & 0xFF;
                let sgn = if s & 0x80 != 0 { 0xFF00_u16 } else { 0_u16 };
                let result = sgn | s;

                self.regs.write_r(dreg, result);
                self.regs
                    .write_flags(&[(Flag::Z, result == 0), (Flag::S, result & 0x8000 != 0)]);
                self.cycles(3, 3, 1)?;
            }
            (0xB0..=0xBF, _, _) => {
                // FROM
                let reg = (instr & 0x0F) as usize;
                self.sreg = reg;
                self.cycles(3, 3, 1)?;
            }
            (0xC0, _, _) => {
                // HIB
                let result = self.regs.read_r(sreg) >> 8;
                self.regs.write_r(dreg, result);
                self.regs
                    .write_flags(&[(Flag::Z, result == 0), (Flag::S, result & 0x80 != 0)]);
                self.cycles(3, 3, 1)?;
            }
            (0xC1..=0xCF, true, false) => {
                // XOR r#
                let s2reg = (instr & 0x0F) as usize;
                let s1 = self.regs.read_r(sreg);
                let s2 = self.regs.read_r(s2reg);

                let result = s1 ^ s2;
                self.regs.write_r(dreg, result);
                self.regs
                    .write_flags(&[(Flag::Z, result == 0), (Flag::S, result & 0x8000 != 0)]);
                self.cycles(3, 3, 1)?;
            }
            (0xC1..=0xCF, true, true) => {
                // XOR #
                let s1 = self.regs.read_r(sreg);
                let s2 = (instr & 0x0F) as u16;

                let result = s1 ^ s2;
                self.regs.write_r(dreg, result);
                self.regs
                    .write_flags(&[(Flag::Z, result == 0), (Flag::S, result & 0x8000 != 0)]);
                self.cycles(6, 6, 2)?;
            }
            (0xF0..=0xFF, _, _) => {
                // IWT
                let reg = (instr & 0x0F) as usize;
                let imm = self.fetch16();
                self.regs.write_r(reg, imm);
                self.cycles(9, 9, 3)?;
            }
            _ => panic!(
                "Unimplemented instruction {:02X} alt1 = {} alt2 = {}",
                instr, alt1, alt2
            ),
        }
        Ok(())
    }

    fn cycles(&mut self, _cy_rom: Ticks, _cy_ram: usize, _cy_cache: usize) -> Result<()> {
        // TODO cycles

        Ok(())
    }
}

impl Tickable for CpuGsu {
    fn tick(&mut self, _ticks: Ticks) -> Result<()> {
        if !self.regs.test_flag(Flag::G) {
            // GSU stopped
            return Ok(());
        }

        // TODO credits
        self.step()
    }
}
