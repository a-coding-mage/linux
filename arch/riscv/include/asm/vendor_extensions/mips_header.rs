/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2025 MIPS.
 */

// Dependency supplied by the surrounding kernel code: linux/types.h

pub const RISCV_ISA_VENDOR_EXT_XMIPSEXECTL: u32 = 0;

// This declaration is excluded by the C header when building assembler.
#[repr(C)]
pub struct riscv_isa_vendor_ext_data_list {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut riscv_isa_vendor_ext_list_mips: riscv_isa_vendor_ext_data_list;
}

/* Extension specific instructions */

/*
 * All of the xmipsexectl extension instructions are
 * ‘hint’ encodings of the SLLI instruction,
 * with rd = 0, rs1 = 0 and imm = 1 for IHB, imm = 3 for EHB,
 * and imm = 5 for PAUSE.
 * MIPS.PAUSE is an alternative opcode which is implemented to have
 * the same behavior as PAUSE on some MIPS RISCV cores.
 * MIPS.EHB clears all execution hazards before allowing
 * any subsequent instructions to execute.
 * MIPS.IHB clears all instruction hazards before
 * allowing any subsequent instructions to fetch.
 */

// ASM_INSN_I(...) supplies the corresponding inline-assembly instruction
// string in the original C header.
pub const MIPS_PAUSE: &str = "0x00501013\n\t";
pub const MIPS_EHB: &str = "0x00301013\n\t";
pub const MIPS_IHB: &str = "0x00101013\n\t";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
