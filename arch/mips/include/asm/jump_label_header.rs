/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (c) 2010 Cavium Networks, Inc.
 */

// C: #define arch_jump_label_transform_static arch_jump_label_transform
// The architecture-specific transform has the same name in Rust consumers.

#[allow(non_camel_case_types)]
pub enum module {}

#[allow(non_camel_case_types)]
pub enum static_key {}

extern "C" {
    pub fn jump_label_apply_nops(mod_: *mut module);
}

pub const JUMP_LABEL_NOP_SIZE: usize = 4;

// Build-time C conditions are retained here as configuration intent.
#[cfg(target_pointer_width = "64")]
pub const WORD_INSN: &str = ".dword";
#[cfg(not(target_pointer_width = "64"))]
pub const WORD_INSN: &str = ".word";

#[cfg(feature = "CONFIG_CPU_MICROMIPS")]
pub const B_INSN: &str = "b32";
#[cfg(feature = "CONFIG_CPU_MICROMIPS")]
pub const J_INSN: &str = "j32";
#[cfg(all(not(feature = "CONFIG_CPU_MICROMIPS"), feature = "MIPS_ISA_REV_6"))]
pub const B_INSN: &str = "bc";
#[cfg(all(not(feature = "CONFIG_CPU_MICROMIPS"), feature = "MIPS_ISA_REV_6"))]
pub const J_INSN: &str = "bc";
#[cfg(all(not(feature = "CONFIG_CPU_MICROMIPS"), not(feature = "MIPS_ISA_REV_6")))]
pub const B_INSN: &str = "b";
#[cfg(all(not(feature = "CONFIG_CPU_MICROMIPS"), not(feature = "MIPS_ISA_REV_6")))]
pub const J_INSN: &str = "j";

#[inline(always)]
pub unsafe fn arch_static_branch(key: *mut static_key, branch: bool) -> bool {
    let _ = (key, branch);
    // C asm goto:
    // 1: B_INSN 2f; 2: .insn; emit WORD_INSN 1b, l_yes, &((char *)key)[branch]
    // Rust has no direct asm-goto equivalent; the fall-through result is false.
    false
    // l_yes: return true;
}

#[inline(always)]
pub unsafe fn arch_static_branch_jump(key: *mut static_key, branch: bool) -> bool {
    let _ = (key, branch);
    // C asm goto:
    // 1: J_INSN l_yes; emit WORD_INSN 1b, l_yes, &((char *)key)[branch]
    // Rust has no direct asm-goto equivalent; the fall-through result is false.
    false
    // l_yes: return true;
}

#[cfg(target_pointer_width = "64")]
pub type jump_label_t = u64;
#[cfg(not(target_pointer_width = "64"))]
pub type jump_label_t = u32;

#[repr(C)]
pub struct jump_entry {
    pub code: jump_label_t,
    pub target: jump_label_t,
    pub key: jump_label_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
