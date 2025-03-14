use super::regs::Reg;
use super::immediate::{ImmediateInstruction, immediate_to_endian};

pub fn jarl(dist: Reg, rs1: Reg, offset: i64) -> Vec<u8> {
    immediate_to_endian(ImmediateInstruction {
        rs1,
        rd: dist,
        imm: offset,
        opcode: 0x67,
    })
}

pub fn jal(target_pc: u64, current_pc: u64, rd: Reg) -> Vec<u8> {
    let offset = target_pc.wrapping_sub(current_pc) as i32;
    let rd: u64 = rd.into();

    if offset < -1048576 || offset > 1048574 {
        panic!("invalid offset");
    }

    if (offset % 2) != 0 {
        panic!("invalid offset");
    }

    let offset_in_units = offset / 2;

    let imm = ((offset_in_units & 0x80000) << 12)
        | ((offset_in_units & 0x3FF) << 21)
        | ((offset_in_units & 0x400) << 10)
        | ((offset_in_units & 0x7F800) >> 11);

    let instruction = imm as u32
        | (rd as u32) << 7  
        | 0x6F;             

    instruction.to_le_bytes().to_vec()
}
