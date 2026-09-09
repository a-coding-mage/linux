/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of bpf_jit.h (PPC BPF JIT compiler header). */

use core::ffi::c_void;

#[cfg(feature = "CONFIG_PPC64_ELF_ABI_V1")]
pub const FUNCTION_DESCR_SIZE: usize = 24;
#[cfg(not(feature = "CONFIG_PPC64_ELF_ABI_V1"))]
pub const FUNCTION_DESCR_SIZE: usize = 0;

pub const SZL: usize = core::mem::size_of::<usize>();
pub const BPF_INSN_SAFETY: usize = 64;
pub const BPF_PPC_TAILCALL: usize = 8;

/* Build-time C macros are represented as Rust macros; their referenced PPC
 * instruction helpers and context types are supplied by dependent code. */
#[macro_export]
macro_rules! CTX_NIA { ($ctx:expr) => { ($ctx.idx as u64).wrapping_mul(4) }; }

#[macro_export]
macro_rules! PLANT_INSTR {
    ($d:expr, $idx:expr, $instr:expr) => {{
        if !$d.is_null() { unsafe { *$d.add($idx) = $instr; } }
        $idx += 1;
    }};
}
#[macro_export]
macro_rules! EMIT { ($image:expr, $ctx:expr, $instr:expr) => { $crate::PLANT_INSTR!($image, $ctx.idx, $instr) }; }

#[macro_export]
macro_rules! PPC_JMP {
    ($image:expr, $ctx:expr, $dest:expr) => {{
        let offset: i64 = ($dest as i64).wrapping_sub($crate::CTX_NIA!($ctx) as i64);
        if $dest != 0 && !is_offset_in_branch_range(offset) { return -ERANGE; }
        $crate::EMIT!($image, $ctx, PPC_RAW_BRANCH(offset));
    }};
}
#[macro_export]
macro_rules! PPC_BCC_SHORT {
    ($image:expr, $ctx:expr, $cond:expr, $dest:expr) => {{
        let offset: i64 = ($dest as i64).wrapping_sub($crate::CTX_NIA!($ctx) as i64);
        if $dest != 0 && !is_offset_in_cond_branch_range(offset) { return -ERANGE; }
        $crate::EMIT!($image, $ctx, PPC_INST_BRANCH_COND | ((($cond) & 0x3ff) << 16) | (offset & 0xfffc));
    }};
}
#[macro_export]
macro_rules! PPC_BCC_CONST_SHORT {
    ($image:expr, $ctx:expr, $cond:expr, $offset:expr) => {{
        debug_assert!($offset >= -0x8000 && $offset <= 0x7fff && ($offset & 3) == 0);
        $crate::EMIT!($image, $ctx, PPC_INST_BRANCH_COND | ((($cond) & 0x3ff) << 16) | (($offset) & 0xfffc));
    }};
}

#[macro_export]
macro_rules! PPC_LI32 {
    ($image:expr, $ctx:expr, $d:expr, $i:expr) => {{
        if $image.is_null() { $ctx.idx += 2; }
        else if ($i as i32) >= -32768 && ($i as i32) < 32768 {
            $crate::EMIT!($image, $ctx, PPC_RAW_LI($d, $i));
        } else {
            $crate::EMIT!($image, $ctx, PPC_RAW_LIS($d, IMM_H($i)));
            if IMM_L($i) != 0 { $crate::EMIT!($image, $ctx, PPC_RAW_ORI($d, $d, IMM_L($i))); }
        }
    }};
}

#[cfg(feature = "CONFIG_PPC64")]
#[macro_export]
macro_rules! PPC_LI64 {
    ($image:expr, $ctx:expr, $d:expr, $i:expr) => {{
        if $image.is_null() { $ctx.idx += 5; }
        else if ($i as i64) >= -2_147_483_648 && ($i as i64) < 2_147_483_648 {
            $crate::PPC_LI32!($image, $ctx, $d, $i);
        } else {
            let v = $i as u64;
            if v & 0xffff_8000_0000_0000 == 0 { $crate::EMIT!($image, $ctx, PPC_RAW_LI($d, (v >> 32) & 0xffff)); }
            else { $crate::EMIT!($image, $ctx, PPC_RAW_LIS($d, v >> 48)); if v & 0x0000_ffff_0000_0000 != 0 { $crate::EMIT!($image, $ctx, PPC_RAW_ORI($d, $d, (v >> 32) & 0xffff)); } }
            $crate::EMIT!($image, $ctx, PPC_RAW_SLDI($d, $d, 32));
            if v & 0x0000_0000_ffff_0000 != 0 { $crate::EMIT!($image, $ctx, PPC_RAW_ORIS($d, $d, (v >> 16) & 0xffff)); }
            if v & 0xffff != 0 { $crate::EMIT!($image, $ctx, PPC_RAW_ORI($d, $d, v & 0xffff)); }
        }
    }};
}

#[cfg(not(feature = "CONFIG_PPC64"))]
#[macro_export] macro_rules! PPC_LI64 { ($($t:tt)*) => { compile_error!("PPC_LI64 requires CONFIG_PPC64") }; }
#[macro_export] macro_rules! PPC_LI_ADDR { ($($t:tt)*) => { $crate::PPC_LI64!($($t)*) }; }
#[macro_export] macro_rules! PPC64_LOAD_PACA { ($($t:tt)*) => {}; }

#[macro_export]
macro_rules! PPC_BCC {
    ($image:expr, $ctx:expr, $cond:expr, $dest:expr) => {{
        if is_offset_in_cond_branch_range(($dest as i64) - $crate::CTX_NIA!($ctx) as i64) {
            $crate::PPC_BCC_SHORT!($image, $ctx, $cond, $dest);
            $crate::EMIT!($image, $ctx, PPC_RAW_NOP());
        } else {
            $crate::PPC_BCC_SHORT!($image, $ctx, ($cond) ^ COND_CMP_TRUE, $crate::CTX_NIA!($ctx) + 8);
            $crate::PPC_JMP!($image, $ctx, $dest);
        }
    }};
}

pub const PRIV_STACK_GUARD_SZ: usize = 16;
pub const PRIV_STACK_GUARD_VAL: u64 = 0xeb9f_1234_5678_eb9f;

pub const CR0_LT: u32 = 0;
pub const CR0_GT: u32 = 1;
pub const CR0_EQ: u32 = 2;
pub const COND_CMP_TRUE: u32 = 0x100;
pub const COND_CMP_FALSE: u32 = 0x000;
pub const COND_GT: u32 = CR0_GT | COND_CMP_TRUE;
pub const COND_GE: u32 = CR0_LT | COND_CMP_FALSE;
pub const COND_EQ: u32 = CR0_EQ | COND_CMP_TRUE;
pub const COND_NE: u32 = CR0_EQ | COND_CMP_FALSE;
pub const COND_LT: u32 = CR0_LT | COND_CMP_TRUE;
pub const COND_LE: u32 = CR0_GT | COND_CMP_FALSE;
pub const SEEN_FUNC: u32 = 0x2000_0000;
pub const SEEN_TAILCALL: u32 = 0x4000_0000;

#[repr(C)]
pub struct codegen_context {
    pub seen: u32,
    pub idx: u32,
    pub stack_size: u32,
    pub b2p: [i32; MAX_BPF_JIT_REG as usize + 3],
    pub exentry_idx: u32,
    pub alt_exit_addr: u32,
    pub arena_vm_start: u64,
    pub user_vm_start: u64,
    pub is_subprog: bool,
    pub exception_boundary: bool,
    pub exception_cb: bool,
    pub priv_sp: *mut c_void,
    pub priv_stack_size: u32,
}

#[macro_export]
macro_rules! bpf_to_ppc { ($ctx:expr, $r:expr) => { $ctx.b2p[$r as usize] }; }

#[cfg(feature = "CONFIG_PPC32")]
pub const BPF_FIXUP_LEN: usize = 3;
#[cfg(not(feature = "CONFIG_PPC32"))]
pub const BPF_FIXUP_LEN: usize = 2;

#[inline]
pub unsafe fn bpf_is_seen_register(ctx: *const codegen_context, i: i32) -> bool { (*ctx).seen & (1u32 << (31 - i)) != 0 }
#[inline]
pub unsafe fn bpf_set_seen_register(ctx: *mut codegen_context, i: i32) { (*ctx).seen |= 1u32 << (31 - i); }
#[inline]
pub unsafe fn bpf_clear_seen_register(ctx: *mut codegen_context, i: i32) { (*ctx).seen &= !(1u32 << (31 - i)); }

extern "C" {
    pub fn bpf_jit_init_reg_mapping(ctx: *mut codegen_context);
    pub fn bpf_jit_emit_func_call_rel(image: *mut u32, fimage: *mut u32, ctx: *mut codegen_context, func: u64) -> i32;
    pub fn bpf_jit_build_body(fp: *mut bpf_prog, image: *mut u32, fimage: *mut u32, ctx: *mut codegen_context, addrs: *mut u32, pass: i32, extra_pass: bool) -> i32;
    pub fn bpf_jit_build_prologue(image: *mut u32, ctx: *mut codegen_context);
    pub fn bpf_jit_build_epilogue(image: *mut u32, ctx: *mut codegen_context);
    pub fn bpf_jit_build_fentry_stubs(image: *mut u32, ctx: *mut codegen_context);
    pub fn bpf_jit_realloc_regs(ctx: *mut codegen_context);
    pub fn bpf_jit_emit_exit_insn(image: *mut u32, ctx: *mut codegen_context, tmp_reg: i32, exit_addr: isize) -> i32;
    pub fn prepare_for_fsession_fentry(image: *mut u32, ctx: *mut codegen_context, cookie_cnt: i32, cookie_off: i32, retval_off: i32);
    pub fn store_func_meta(image: *mut u32, ctx: *mut codegen_context, func_meta: u64, func_meta_off: i32);
    pub fn bpf_add_extable_entry(fp: *mut bpf_prog, image: *mut u32, fimage: *mut u32, pass: i32, ctx: *mut codegen_context, insn_idx: i32, jmp_off: i32, dst_reg: i32, code: u32) -> i32;
}

#[repr(C)] pub struct bpf_prog { _private: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
