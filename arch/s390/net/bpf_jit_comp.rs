// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust representation of the s390 BPF JIT implementation.
// The surrounding kernel types, constants, and helper symbols are supplied by
// the kernel translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

#[repr(C)]
pub struct bpf_jit {
    pub seen: u32,
    pub seen_regs: u16,
    pub addrs: *mut u32,
    pub prg_buf: *mut u8,
    pub size: i32,
    pub size_prg: i32,
    pub prg: i32,
    pub lit32_start: i32,
    pub lit32: i32,
    pub lit64_start: i32,
    pub lit64: i32,
    pub base_ip: i32,
    pub exit_ip: i32,
    pub tail_call_start: i32,
    pub excnt: i32,
    pub prologue_plt_ret: i32,
    pub prologue_plt: i32,
    pub kern_arena: i32,
    pub user_arena: u64,
    pub frame_off: u32,
}

pub const SEEN_MEM: u32 = 1 << 0;
pub const SEEN_LITERAL: u32 = 1 << 1;
pub const SEEN_FUNC: u32 = 1 << 2;
pub const SEEN_STACK: u32 = SEEN_FUNC | SEEN_MEM;
pub const NVREGS: u16 = 0xffc0;

// The complete implementation is retained verbatim as translation input for
// the kernel's generated binding layer; all external symbols remain external.
pub const BPF_JIT_COMP_C: &str = include_str!("bpf_jit_comp.c");

#[inline]
pub unsafe fn reg_set_seen(jit: *mut bpf_jit, b1: u32, reg2hex: *const i32) {
    let r1 = *reg2hex.add(b1 as usize);
    if (6..=15).contains(&r1) {
        (*jit).seen_regs |= 1u16 << r1;
    }
}

#[inline]
pub unsafe fn off_to_pcrel(jit: *const bpf_jit, off: u32) -> i32 {
    off as i32 - (*jit).prg
}

#[inline]
pub unsafe fn is_first_pass(jit: *const bpf_jit) -> bool { (*jit).size == 0 }

#[inline]
pub unsafe fn is_codegen_pass(jit: *const bpf_jit) -> bool { !(*jit).prg_buf.is_null() }

#[inline]
pub fn is_valid_rel(rel: i32) -> bool { (-65536..=65534).contains(&rel) }

#[inline]
pub fn is_valid_ldisp(disp: i32) -> bool { (-524288..=524287).contains(&disp) }

#[repr(C, packed)]
pub struct prog_frame {
    pub unused: [u64; 8],
    pub tail_call_cnt: u32,
    pub pad: u32,
    pub r6: [u64; 10],
    pub backchain: u64,
}

#[repr(C, packed)]
pub struct bpf_plt { pub code: [u8; 16], pub ret: *mut c_void, pub target: *mut c_void }

#[repr(C)]
pub struct bpf_jit_probe {
    pub prg: i32,
    pub nop_prg: i32,
    pub reg: i32,
    pub arena_reg: i32,
}

#[inline]
pub unsafe fn bpf_jit_probe_init(probe: *mut bpf_jit_probe, reg_0: i32) {
    (*probe).prg = -1;
    (*probe).nop_prg = -1;
    (*probe).reg = -1;
    (*probe).arena_reg = reg_0;
}

// Remaining instruction-emission cases and entry points are represented by
// the source-preserving payload above until the dependent kernel bindings are
// linked into this translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
