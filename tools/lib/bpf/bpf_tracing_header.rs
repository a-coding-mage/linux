/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */

/*
 * Rust translation of lib/bpf/bpf_tracing.h.
 *
 * The original header depends on bpf_helpers.h and on target-specific kernel
 * register layouts. Those dependencies are intentionally referenced here, not
 * implemented. C preprocessor target selection is preserved with Rust cfg
 * conditions and comments where the exact C build-time symbol has no direct
 * Rust equivalent.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_macros)]

pub type c_ulong = core::ffi::c_ulong;
pub type c_ulonglong = core::ffi::c_ulonglong;
pub type c_void = core::ffi::c_void;

pub const __BPF_TARGET_MISSING: &str =
    "GCC error \"Must specify a BPF target arch via __TARGET_ARCH_xxx\"";

/*
 * Scan the ARCH passed in from ARCH env variable (see Makefile).
 * Original C recognizes __TARGET_ARCH_x86, __TARGET_ARCH_s390,
 * __TARGET_ARCH_arm, __TARGET_ARCH_arm64, __TARGET_ARCH_mips,
 * __TARGET_ARCH_powerpc, __TARGET_ARCH_sparc, __TARGET_ARCH_riscv,
 * __TARGET_ARCH_arc, and __TARGET_ARCH_loongarch, then falls back to compiler
 * predefined architecture macros.
 */

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct user_pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct user_regs_struct {
    _private: [u8; 0],
}

/*
 * s390:
 * https://github.com/IBM/s390x-abi/releases/download/v1.6/lzsabi_s390x.pdf
 */
#[cfg(any(target_arch = "s390x", bpf_target_s390))]
#[repr(C)]
pub struct pt_regs___s390 {
    pub orig_gpr2: c_ulong,
}

/*
 * arm64:
 * https://github.com/ARM-software/abi-aa/blob/main/aapcs64/aapcs64.rst#machine-registers
 */
#[cfg(any(target_arch = "aarch64", bpf_target_arm64))]
#[repr(C)]
pub struct pt_regs___arm64 {
    pub orig_x0: c_ulong,
}

/*
 * riscv:
 * https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/master/riscv-cc.adoc#risc-v-calling-conventions
 */
#[cfg(any(target_arch = "riscv64", bpf_target_riscv))]
#[repr(C)]
pub struct pt_regs___riscv {
    pub orig_a0: c_ulong,
}

extern "C" {
    pub fn bpf_probe_read_kernel(dst: *mut c_void, size: c_ulong, unsafe_ptr: *const c_void) -> i64;
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86", bpf_target_x86))]
macro_rules! __PT_REGS_CAST {
    ($x:expr) => {
        $x
    };
}

#[cfg(any(target_arch = "s390x", bpf_target_s390))]
macro_rules! __PT_REGS_CAST {
    ($x:expr) => {
        ($x as *const user_pt_regs)
    };
}

#[cfg(any(target_arch = "aarch64", bpf_target_arm64))]
macro_rules! __PT_REGS_CAST {
    ($x:expr) => {
        ($x as *const user_pt_regs)
    };
}

#[cfg(any(target_arch = "riscv64", bpf_target_riscv))]
macro_rules! __PT_REGS_CAST {
    ($x:expr) => {
        ($x as *const user_regs_struct)
    };
}

#[cfg(any(target_arch = "arc", bpf_target_arc))]
macro_rules! __PT_REGS_CAST {
    ($x:expr) => {
        ($x as *const user_regs_struct)
    };
}

#[cfg(any(target_arch = "loongarch64", bpf_target_loongarch))]
macro_rules! __PT_REGS_CAST {
    ($x:expr) => {
        ($x as *const user_pt_regs)
    };
}

#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "s390x",
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "mips",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "sparc",
    target_arch = "sparc64",
    target_arch = "riscv64",
    target_arch = "arc",
    target_arch = "loongarch64",
    bpf_target_x86,
    bpf_target_s390,
    bpf_target_arm,
    bpf_target_arm64,
    bpf_target_mips,
    bpf_target_powerpc,
    bpf_target_sparc,
    bpf_target_riscv,
    bpf_target_arc,
    bpf_target_loongarch
)))]
macro_rules! __BPF_TARGET_MISSING_EXPR {
    () => {
        0 as c_ulong
    };
}

/* x86 System V AMD64 ABI and i386 regparm/syscall conventions. */
#[cfg(any(target_arch = "x86_64", bpf_target_x86))]
macro_rules! PT_REGS_PARM1 { ($x:expr) => { (*__PT_REGS_CAST!($x)).rdi }; }
#[cfg(any(target_arch = "x86_64", bpf_target_x86))]
macro_rules! PT_REGS_PARM2 { ($x:expr) => { (*__PT_REGS_CAST!($x)).rsi }; }
#[cfg(any(target_arch = "x86_64", bpf_target_x86))]
macro_rules! PT_REGS_PARM3 { ($x:expr) => { (*__PT_REGS_CAST!($x)).rdx }; }
#[cfg(any(target_arch = "x86_64", bpf_target_x86))]
macro_rules! PT_REGS_PARM4 { ($x:expr) => { (*__PT_REGS_CAST!($x)).rcx }; }
#[cfg(any(target_arch = "x86_64", bpf_target_x86))]
macro_rules! PT_REGS_PARM5 { ($x:expr) => { (*__PT_REGS_CAST!($x)).r8 }; }
#[cfg(any(target_arch = "x86_64", bpf_target_x86))]
macro_rules! PT_REGS_PARM6 { ($x:expr) => { (*__PT_REGS_CAST!($x)).r9 }; }
#[cfg(any(target_arch = "x86_64", bpf_target_x86))]
macro_rules! PT_REGS_PARM7 { ($x:expr) => { compile_error!("__unsupported__") }; }
#[cfg(any(target_arch = "x86_64", bpf_target_x86))]
macro_rules! PT_REGS_PARM8 { ($x:expr) => { compile_error!("__unsupported__") }; }
#[cfg(any(target_arch = "x86_64", bpf_target_x86))]
macro_rules! PT_REGS_RET { ($x:expr) => { (*__PT_REGS_CAST!($x)).rsp }; }
#[cfg(any(target_arch = "x86_64", bpf_target_x86))]
macro_rules! PT_REGS_FP { ($x:expr) => { (*__PT_REGS_CAST!($x)).rbp }; }
#[cfg(any(target_arch = "x86_64", bpf_target_x86))]
macro_rules! PT_REGS_RC { ($x:expr) => { (*__PT_REGS_CAST!($x)).rax }; }
#[cfg(any(target_arch = "x86_64", bpf_target_x86))]
macro_rules! PT_REGS_SP { ($x:expr) => { (*__PT_REGS_CAST!($x)).rsp }; }
#[cfg(any(target_arch = "x86_64", bpf_target_x86))]
macro_rules! PT_REGS_IP { ($x:expr) => { (*__PT_REGS_CAST!($x)).rip }; }
#[cfg(any(target_arch = "x86_64", bpf_target_x86))]
macro_rules! PT_REGS_PARM1_SYSCALL { ($x:expr) => { PT_REGS_PARM1!($x) }; }
#[cfg(any(target_arch = "x86_64", bpf_target_x86))]
macro_rules! PT_REGS_PARM2_SYSCALL { ($x:expr) => { PT_REGS_PARM2!($x) }; }
#[cfg(any(target_arch = "x86_64", bpf_target_x86))]
macro_rules! PT_REGS_PARM3_SYSCALL { ($x:expr) => { PT_REGS_PARM3!($x) }; }
#[cfg(any(target_arch = "x86_64", bpf_target_x86))]
macro_rules! PT_REGS_PARM4_SYSCALL { ($x:expr) => { (*__PT_REGS_CAST!($x)).r10 }; }
#[cfg(any(target_arch = "x86_64", bpf_target_x86))]
macro_rules! PT_REGS_PARM5_SYSCALL { ($x:expr) => { PT_REGS_PARM5!($x) }; }
#[cfg(any(target_arch = "x86_64", bpf_target_x86))]
macro_rules! PT_REGS_PARM6_SYSCALL { ($x:expr) => { PT_REGS_PARM6!($x) }; }
#[cfg(any(target_arch = "x86_64", bpf_target_x86))]
macro_rules! PT_REGS_PARM7_SYSCALL { ($x:expr) => { compile_error!("__unsupported__") }; }

#[cfg(target_arch = "x86")]
macro_rules! PT_REGS_PARM1 { ($x:expr) => { (*__PT_REGS_CAST!($x)).eax }; }
#[cfg(target_arch = "x86")]
macro_rules! PT_REGS_PARM2 { ($x:expr) => { (*__PT_REGS_CAST!($x)).edx }; }
#[cfg(target_arch = "x86")]
macro_rules! PT_REGS_PARM3 { ($x:expr) => { (*__PT_REGS_CAST!($x)).ecx }; }
#[cfg(target_arch = "x86")]
macro_rules! PT_REGS_PARM4 { ($x:expr) => { compile_error!("__unsupported__") }; }
#[cfg(target_arch = "x86")]
macro_rules! PT_REGS_PARM5 { ($x:expr) => { compile_error!("__unsupported__") }; }
#[cfg(target_arch = "x86")]
macro_rules! PT_REGS_PARM6 { ($x:expr) => { compile_error!("__unsupported__") }; }
#[cfg(target_arch = "x86")]
macro_rules! PT_REGS_PARM7 { ($x:expr) => { compile_error!("__unsupported__") }; }
#[cfg(target_arch = "x86")]
macro_rules! PT_REGS_PARM8 { ($x:expr) => { compile_error!("__unsupported__") }; }
#[cfg(target_arch = "x86")]
macro_rules! PT_REGS_RET { ($x:expr) => { (*__PT_REGS_CAST!($x)).esp }; }
#[cfg(target_arch = "x86")]
macro_rules! PT_REGS_FP { ($x:expr) => { (*__PT_REGS_CAST!($x)).ebp }; }
#[cfg(target_arch = "x86")]
macro_rules! PT_REGS_RC { ($x:expr) => { (*__PT_REGS_CAST!($x)).eax }; }
#[cfg(target_arch = "x86")]
macro_rules! PT_REGS_SP { ($x:expr) => { (*__PT_REGS_CAST!($x)).esp }; }
#[cfg(target_arch = "x86")]
macro_rules! PT_REGS_IP { ($x:expr) => { (*__PT_REGS_CAST!($x)).eip }; }
#[cfg(target_arch = "x86")]
macro_rules! PT_REGS_PARM1_SYSCALL { ($x:expr) => { (*__PT_REGS_CAST!($x)).ebx }; }
#[cfg(target_arch = "x86")]
macro_rules! PT_REGS_PARM2_SYSCALL { ($x:expr) => { (*__PT_REGS_CAST!($x)).ecx }; }
#[cfg(target_arch = "x86")]
macro_rules! PT_REGS_PARM3_SYSCALL { ($x:expr) => { (*__PT_REGS_CAST!($x)).edx }; }
#[cfg(target_arch = "x86")]
macro_rules! PT_REGS_PARM4_SYSCALL { ($x:expr) => { (*__PT_REGS_CAST!($x)).esi }; }
#[cfg(target_arch = "x86")]
macro_rules! PT_REGS_PARM5_SYSCALL { ($x:expr) => { (*__PT_REGS_CAST!($x)).edi }; }
#[cfg(target_arch = "x86")]
macro_rules! PT_REGS_PARM6_SYSCALL { ($x:expr) => { (*__PT_REGS_CAST!($x)).ebp }; }
#[cfg(target_arch = "x86")]
macro_rules! PT_REGS_PARM7_SYSCALL { ($x:expr) => { compile_error!("__unsupported__") }; }

/* The remaining architecture register macros mirror C field paths literally. */
#[cfg(any(target_arch = "s390x", bpf_target_s390))]
macro_rules! PT_REGS_PARM1 { ($x:expr) => { (*__PT_REGS_CAST!($x)).gprs[2] }; }
#[cfg(any(target_arch = "s390x", bpf_target_s390))]
macro_rules! PT_REGS_PARM2 { ($x:expr) => { (*__PT_REGS_CAST!($x)).gprs[3] }; }
#[cfg(any(target_arch = "s390x", bpf_target_s390))]
macro_rules! PT_REGS_PARM3 { ($x:expr) => { (*__PT_REGS_CAST!($x)).gprs[4] }; }
#[cfg(any(target_arch = "s390x", bpf_target_s390))]
macro_rules! PT_REGS_PARM4 { ($x:expr) => { (*__PT_REGS_CAST!($x)).gprs[5] }; }
#[cfg(any(target_arch = "s390x", bpf_target_s390))]
macro_rules! PT_REGS_PARM5 { ($x:expr) => { (*__PT_REGS_CAST!($x)).gprs[6] }; }
#[cfg(any(target_arch = "s390x", bpf_target_s390))]
macro_rules! PT_REGS_PARM6 { ($x:expr) => { compile_error!("__unsupported__") }; }
#[cfg(any(target_arch = "s390x", bpf_target_s390))]
macro_rules! PT_REGS_PARM7 { ($x:expr) => { compile_error!("__unsupported__") }; }
#[cfg(any(target_arch = "s390x", bpf_target_s390))]
macro_rules! PT_REGS_PARM8 { ($x:expr) => { compile_error!("__unsupported__") }; }
#[cfg(any(target_arch = "s390x", bpf_target_s390))]
macro_rules! PT_REGS_PARM1_SYSCALL { ($x:expr) => { (*($x as *const pt_regs___s390)).orig_gpr2 }; }
#[cfg(any(target_arch = "s390x", bpf_target_s390))]
macro_rules! PT_REGS_PARM2_SYSCALL { ($x:expr) => { PT_REGS_PARM2!($x) }; }
#[cfg(any(target_arch = "s390x", bpf_target_s390))]
macro_rules! PT_REGS_PARM3_SYSCALL { ($x:expr) => { PT_REGS_PARM3!($x) }; }
#[cfg(any(target_arch = "s390x", bpf_target_s390))]
macro_rules! PT_REGS_PARM4_SYSCALL { ($x:expr) => { PT_REGS_PARM4!($x) }; }
#[cfg(any(target_arch = "s390x", bpf_target_s390))]
macro_rules! PT_REGS_PARM5_SYSCALL { ($x:expr) => { PT_REGS_PARM5!($x) }; }
#[cfg(any(target_arch = "s390x", bpf_target_s390))]
macro_rules! PT_REGS_PARM6_SYSCALL { ($x:expr) => { (*__PT_REGS_CAST!($x)).gprs[7] }; }
#[cfg(any(target_arch = "s390x", bpf_target_s390))]
macro_rules! PT_REGS_PARM7_SYSCALL { ($x:expr) => { compile_error!("__unsupported__") }; }
#[cfg(any(target_arch = "s390x", bpf_target_s390))]
macro_rules! PT_REGS_RET { ($x:expr) => { (*__PT_REGS_CAST!($x)).gprs[14] }; }
#[cfg(any(target_arch = "s390x", bpf_target_s390))]
macro_rules! PT_REGS_FP { ($x:expr) => { (*__PT_REGS_CAST!($x)).gprs[11] } /* Works only with CONFIG_FRAME_POINTER */; }
#[cfg(any(target_arch = "s390x", bpf_target_s390))]
macro_rules! PT_REGS_RC { ($x:expr) => { (*__PT_REGS_CAST!($x)).gprs[2] }; }
#[cfg(any(target_arch = "s390x", bpf_target_s390))]
macro_rules! PT_REGS_SP { ($x:expr) => { (*__PT_REGS_CAST!($x)).gprs[15] }; }
#[cfg(any(target_arch = "s390x", bpf_target_s390))]
macro_rules! PT_REGS_IP { ($x:expr) => { (*__PT_REGS_CAST!($x)).psw.addr }; }

macro_rules! BPF_CORE_READ {
    ($ptr:expr, $($field:tt)+) => {
        unsafe { (*($ptr)).$($field)+ }
    };
}

macro_rules! PT_REGS_PARM1_CORE { ($x:expr) => { BPF_CORE_READ!(__PT_REGS_CAST!($x), __PT_PARM1_REG) }; }
macro_rules! PT_REGS_PARM2_CORE { ($x:expr) => { BPF_CORE_READ!(__PT_REGS_CAST!($x), __PT_PARM2_REG) }; }
macro_rules! PT_REGS_PARM3_CORE { ($x:expr) => { BPF_CORE_READ!(__PT_REGS_CAST!($x), __PT_PARM3_REG) }; }
macro_rules! PT_REGS_PARM4_CORE { ($x:expr) => { BPF_CORE_READ!(__PT_REGS_CAST!($x), __PT_PARM4_REG) }; }
macro_rules! PT_REGS_PARM5_CORE { ($x:expr) => { BPF_CORE_READ!(__PT_REGS_CAST!($x), __PT_PARM5_REG) }; }
macro_rules! PT_REGS_PARM6_CORE { ($x:expr) => { BPF_CORE_READ!(__PT_REGS_CAST!($x), __PT_PARM6_REG) }; }
macro_rules! PT_REGS_PARM7_CORE { ($x:expr) => { BPF_CORE_READ!(__PT_REGS_CAST!($x), __PT_PARM7_REG) }; }
macro_rules! PT_REGS_PARM8_CORE { ($x:expr) => { BPF_CORE_READ!(__PT_REGS_CAST!($x), __PT_PARM8_REG) }; }
macro_rules! PT_REGS_RET_CORE { ($x:expr) => { BPF_CORE_READ!(__PT_REGS_CAST!($x), __PT_RET_REG) }; }
macro_rules! PT_REGS_FP_CORE { ($x:expr) => { BPF_CORE_READ!(__PT_REGS_CAST!($x), __PT_FP_REG) }; }
macro_rules! PT_REGS_RC_CORE { ($x:expr) => { BPF_CORE_READ!(__PT_REGS_CAST!($x), __PT_RC_REG) }; }
macro_rules! PT_REGS_SP_CORE { ($x:expr) => { BPF_CORE_READ!(__PT_REGS_CAST!($x), __PT_SP_REG) }; }
macro_rules! PT_REGS_IP_CORE { ($x:expr) => { BPF_CORE_READ!(__PT_REGS_CAST!($x), __PT_IP_REG) }; }

#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64", bpf_target_powerpc))]
macro_rules! BPF_KPROBE_READ_RET_IP {
    ($ip:expr, $ctx:expr) => {{
        $ip = (*$ctx).link;
    }};
}

#[cfg(any(target_arch = "sparc", target_arch = "sparc64", target_arch = "aarch64", bpf_target_sparc, bpf_target_arm64))]
macro_rules! BPF_KPROBE_READ_RET_IP {
    ($ip:expr, $ctx:expr) => {{
        $ip = PT_REGS_RET!($ctx);
    }};
}

#[cfg(not(any(
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "sparc",
    target_arch = "sparc64",
    target_arch = "aarch64",
    bpf_target_powerpc,
    bpf_target_sparc,
    bpf_target_arm64
)))]
macro_rules! BPF_KPROBE_READ_RET_IP {
    ($ip:expr, $ctx:expr) => {{
        unsafe {
            bpf_probe_read_kernel(
                &mut $ip as *mut _ as *mut c_void,
                core::mem::size_of_val(&$ip) as c_ulong,
                PT_REGS_RET!($ctx) as *const c_void,
            )
        }
    }};
}

#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64", target_arch = "sparc", target_arch = "sparc64", target_arch = "aarch64", bpf_target_powerpc, bpf_target_sparc, bpf_target_arm64))]
macro_rules! BPF_KRETPROBE_READ_RET_IP {
    ($ip:expr, $ctx:expr) => {
        BPF_KPROBE_READ_RET_IP!($ip, $ctx)
    };
}

#[cfg(not(any(target_arch = "powerpc", target_arch = "powerpc64", target_arch = "sparc", target_arch = "sparc64", target_arch = "aarch64", bpf_target_powerpc, bpf_target_sparc, bpf_target_arm64)))]
macro_rules! BPF_KRETPROBE_READ_RET_IP {
    ($ip:expr, $ctx:expr) => {{
        unsafe {
            bpf_probe_read_kernel(
                &mut $ip as *mut _ as *mut c_void,
                core::mem::size_of_val(&$ip) as c_ulong,
                (PT_REGS_FP!($ctx) as usize + core::mem::size_of_val(&$ip)) as *const c_void,
            )
        }
    }};
}

macro_rules! PT_REGS_SYSCALL_REGS {
    ($ctx:expr) => {
        (PT_REGS_PARM1!($ctx) as *mut pt_regs)
    };
}

macro_rules! ___bpf_concat {
    ($a:ident, $b:ident) => {
        compile_error!("C token concatenation has no direct expression-level Rust equivalent")
    };
}

macro_rules! ___bpf_apply {
    ($fn:ident, $n:tt) => {
        compile_error!("C preprocessor macro application has no direct Rust equivalent")
    };
}

macro_rules! ___bpf_narg {
    () => { 0usize };
    ($a:tt) => { 1usize };
    ($a:tt, $b:tt) => { 2usize };
    ($a:tt, $b:tt, $c:tt) => { 3usize };
    ($a:tt, $b:tt, $c:tt, $d:tt) => { 4usize };
    ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt) => { 5usize };
    ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt) => { 6usize };
    ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt) => { 7usize };
    ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $h:tt) => { 8usize };
    ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $h:tt, $i:tt) => { 9usize };
    ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $h:tt, $i:tt, $j:tt) => { 10usize };
    ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $h:tt, $i:tt, $j:tt, $k:tt) => { 11usize };
    ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $h:tt, $i:tt, $j:tt, $k:tt, $l:tt) => { 12usize };
}

macro_rules! ___bpf_ctx_cast0 { ($ctx:expr) => { $ctx }; }
macro_rules! ___bpf_ctx_cast1 { ($ctx:expr) => { $ctx, (*$ctx.add(0)) }; }
macro_rules! ___bpf_ctx_cast2 { ($ctx:expr) => { ___bpf_ctx_cast1!($ctx), (*$ctx.add(1)) }; }
macro_rules! ___bpf_ctx_cast3 { ($ctx:expr) => { ___bpf_ctx_cast2!($ctx), (*$ctx.add(2)) }; }
macro_rules! ___bpf_ctx_cast4 { ($ctx:expr) => { ___bpf_ctx_cast3!($ctx), (*$ctx.add(3)) }; }
macro_rules! ___bpf_ctx_cast5 { ($ctx:expr) => { ___bpf_ctx_cast4!($ctx), (*$ctx.add(4)) }; }
macro_rules! ___bpf_ctx_cast6 { ($ctx:expr) => { ___bpf_ctx_cast5!($ctx), (*$ctx.add(5)) }; }
macro_rules! ___bpf_ctx_cast7 { ($ctx:expr) => { ___bpf_ctx_cast6!($ctx), (*$ctx.add(6)) }; }
macro_rules! ___bpf_ctx_cast8 { ($ctx:expr) => { ___bpf_ctx_cast7!($ctx), (*$ctx.add(7)) }; }
macro_rules! ___bpf_ctx_cast9 { ($ctx:expr) => { ___bpf_ctx_cast8!($ctx), (*$ctx.add(8)) }; }
macro_rules! ___bpf_ctx_cast10 { ($ctx:expr) => { ___bpf_ctx_cast9!($ctx), (*$ctx.add(9)) }; }
macro_rules! ___bpf_ctx_cast11 { ($ctx:expr) => { ___bpf_ctx_cast10!($ctx), (*$ctx.add(10)) }; }
macro_rules! ___bpf_ctx_cast12 { ($ctx:expr) => { ___bpf_ctx_cast11!($ctx), (*$ctx.add(11)) }; }

/*
 * BPF_PROG is a convenience wrapper for generic tp_btf/fentry/fexit and
 * similar kinds of BPF programs, that accept input arguments as a single
 * pointer to untyped u64 array, where each u64 can actually be a typed pointer
 * or integer of different size. Rust cannot reproduce C's typeof(),
 * declaration-producing macro expansion, token pasting, or GCC diagnostic
 * pragmas exactly, so this macro preserves the external spelling and requires
 * the caller to provide the Rust function body shape.
 */
macro_rules! BPF_PROG {
    ($($tokens:tt)*) => {
        compile_error!("BPF_PROG declaration macro requires C preprocessor semantics and must be expanded by a C-compatible BPF build path")
    };
}

macro_rules! ___bpf_narg2 {
    () => { 0usize };
    ($t1:tt, $x1:tt) => { 1usize };
    ($t1:tt, $x1:tt, $t2:tt, $x2:tt) => { 2usize };
    ($t1:tt, $x1:tt, $t2:tt, $x2:tt, $t3:tt, $x3:tt) => { 3usize };
    ($t1:tt, $x1:tt, $t2:tt, $x2:tt, $t3:tt, $x3:tt, $t4:tt, $x4:tt) => { 4usize };
    ($t1:tt, $x1:tt, $t2:tt, $x2:tt, $t3:tt, $x3:tt, $t4:tt, $x4:tt, $t5:tt, $x5:tt) => { 5usize };
    ($t1:tt, $x1:tt, $t2:tt, $x2:tt, $t3:tt, $x3:tt, $t4:tt, $x4:tt, $t5:tt, $x5:tt, $t6:tt, $x6:tt) => { 6usize };
    ($t1:tt, $x1:tt, $t2:tt, $x2:tt, $t3:tt, $x3:tt, $t4:tt, $x4:tt, $t5:tt, $x5:tt, $t6:tt, $x6:tt, $t7:tt, $x7:tt) => { 7usize };
    ($t1:tt, $x1:tt, $t2:tt, $x2:tt, $t3:tt, $x3:tt, $t4:tt, $x4:tt, $t5:tt, $x5:tt, $t6:tt, $x6:tt, $t7:tt, $x7:tt, $t8:tt, $x8:tt) => { 8usize };
    ($t1:tt, $x1:tt, $t2:tt, $x2:tt, $t3:tt, $x3:tt, $t4:tt, $x4:tt, $t5:tt, $x5:tt, $t6:tt, $x6:tt, $t7:tt, $x7:tt, $t8:tt, $x8:tt, $t9:tt, $x9:tt) => { 9usize };
    ($t1:tt, $x1:tt, $t2:tt, $x2:tt, $t3:tt, $x3:tt, $t4:tt, $x4:tt, $t5:tt, $x5:tt, $t6:tt, $x6:tt, $t7:tt, $x7:tt, $t8:tt, $x8:tt, $t9:tt, $x9:tt, $t10:tt, $x10:tt) => { 10usize };
    ($t1:tt, $x1:tt, $t2:tt, $x2:tt, $t3:tt, $x3:tt, $t4:tt, $x4:tt, $t5:tt, $x5:tt, $t6:tt, $x6:tt, $t7:tt, $x7:tt, $t8:tt, $x8:tt, $t9:tt, $x9:tt, $t10:tt, $x10:tt, $t11:tt, $x11:tt) => { 11usize };
    ($t1:tt, $x1:tt, $t2:tt, $x2:tt, $t3:tt, $x3:tt, $t4:tt, $x4:tt, $t5:tt, $x5:tt, $t6:tt, $x6:tt, $t7:tt, $x7:tt, $t8:tt, $x8:tt, $t9:tt, $x9:tt, $t10:tt, $x10:tt, $t11:tt, $x11:tt, $t12:tt, $x12:tt) => { 12usize };
}

pub const fn ___bpf_treg_cnt<T>() -> usize {
    let sz = core::mem::size_of::<T>();
    if sz == 16 {
        2
    } else if sz == 1 || sz == 2 || sz == 4 || sz == 8 {
        1
    } else {
        0
    }
}

pub unsafe fn ___bpf_union_arg<T: Copy>(ctx: *const u64, n: usize) -> T {
    let mut storage = [0u64; 2];
    storage[0] = *ctx.add(n);
    if core::mem::size_of::<T>() == 16 {
        storage[1] = *ctx.add(n + 1);
    }
    core::ptr::read_unaligned(storage.as_ptr() as *const T)
}

macro_rules! BPF_PROG2 {
    ($($tokens:tt)*) => {
        compile_error!("BPF_PROG2 declaration macro requires C preprocessor semantics and must be expanded by a C-compatible BPF build path")
    };
}

macro_rules! ___bpf_kprobe_args0 { ($ctx:expr) => { $ctx }; }
macro_rules! ___bpf_kprobe_args1 { ($ctx:expr) => { $ctx, PT_REGS_PARM1!($ctx) as c_ulonglong }; }
macro_rules! ___bpf_kprobe_args2 { ($ctx:expr) => { ___bpf_kprobe_args1!($ctx), PT_REGS_PARM2!($ctx) as c_ulonglong }; }
macro_rules! ___bpf_kprobe_args3 { ($ctx:expr) => { ___bpf_kprobe_args2!($ctx), PT_REGS_PARM3!($ctx) as c_ulonglong }; }
macro_rules! ___bpf_kprobe_args4 { ($ctx:expr) => { ___bpf_kprobe_args3!($ctx), PT_REGS_PARM4!($ctx) as c_ulonglong }; }
macro_rules! ___bpf_kprobe_args5 { ($ctx:expr) => { ___bpf_kprobe_args4!($ctx), PT_REGS_PARM5!($ctx) as c_ulonglong }; }
macro_rules! ___bpf_kprobe_args6 { ($ctx:expr) => { ___bpf_kprobe_args5!($ctx), PT_REGS_PARM6!($ctx) as c_ulonglong }; }
macro_rules! ___bpf_kprobe_args7 { ($ctx:expr) => { ___bpf_kprobe_args6!($ctx), PT_REGS_PARM7!($ctx) as c_ulonglong }; }
macro_rules! ___bpf_kprobe_args8 { ($ctx:expr) => { ___bpf_kprobe_args7!($ctx), PT_REGS_PARM8!($ctx) as c_ulonglong }; }

/*
 * BPF_KPROBE serves the same purpose for kprobes as BPF_PROG for
 * tp_btf/fentry/fexit BPF programs. It hides the platform-specific low-level
 * way of getting kprobe input arguments from struct pt_regs.
 */
macro_rules! BPF_KPROBE {
    ($($tokens:tt)*) => {
        compile_error!("BPF_KPROBE declaration macro requires C preprocessor semantics and must be expanded by a C-compatible BPF build path")
    };
}

macro_rules! ___bpf_kretprobe_args0 { ($ctx:expr) => { $ctx }; }
macro_rules! ___bpf_kretprobe_args1 { ($ctx:expr) => { $ctx, PT_REGS_RC!($ctx) as c_ulonglong }; }

/*
 * BPF_KRETPROBE is similar to BPF_KPROBE, except it only provides optional
 * return value in addition to struct pt_regs *ctx.
 */
macro_rules! BPF_KRETPROBE {
    ($($tokens:tt)*) => {
        compile_error!("BPF_KRETPROBE declaration macro requires C preprocessor semantics and must be expanded by a C-compatible BPF build path")
    };
}

/* If kernel has CONFIG_ARCH_HAS_SYSCALL_WRAPPER, read pt_regs directly. */
macro_rules! ___bpf_syscall_args0 { ($ctx:expr, $regs:expr) => { $ctx }; }
macro_rules! ___bpf_syscall_args1 { ($ctx:expr, $regs:expr) => { $ctx, PT_REGS_PARM1_SYSCALL!($regs) as c_ulonglong }; }
macro_rules! ___bpf_syscall_args2 { ($ctx:expr, $regs:expr) => { ___bpf_syscall_args1!($ctx, $regs), PT_REGS_PARM2_SYSCALL!($regs) as c_ulonglong }; }
macro_rules! ___bpf_syscall_args3 { ($ctx:expr, $regs:expr) => { ___bpf_syscall_args2!($ctx, $regs), PT_REGS_PARM3_SYSCALL!($regs) as c_ulonglong }; }
macro_rules! ___bpf_syscall_args4 { ($ctx:expr, $regs:expr) => { ___bpf_syscall_args3!($ctx, $regs), PT_REGS_PARM4_SYSCALL!($regs) as c_ulonglong }; }
macro_rules! ___bpf_syscall_args5 { ($ctx:expr, $regs:expr) => { ___bpf_syscall_args4!($ctx, $regs), PT_REGS_PARM5_SYSCALL!($regs) as c_ulonglong }; }
macro_rules! ___bpf_syscall_args6 { ($ctx:expr, $regs:expr) => { ___bpf_syscall_args5!($ctx, $regs), PT_REGS_PARM6_SYSCALL!($regs) as c_ulonglong }; }
macro_rules! ___bpf_syscall_args7 { ($ctx:expr, $regs:expr) => { ___bpf_syscall_args6!($ctx, $regs), PT_REGS_PARM7_SYSCALL!($regs) as c_ulonglong }; }

/* If kernel doesn't have CONFIG_ARCH_HAS_SYSCALL_WRAPPER, BPF_CORE_READ from pt_regs. */
macro_rules! ___bpf_syswrap_args0 { ($ctx:expr, $regs:expr) => { $ctx }; }
macro_rules! ___bpf_syswrap_args1 { ($ctx:expr, $regs:expr) => { $ctx, PT_REGS_PARM1_CORE_SYSCALL!($regs) as c_ulonglong }; }
macro_rules! ___bpf_syswrap_args2 { ($ctx:expr, $regs:expr) => { ___bpf_syswrap_args1!($ctx, $regs), PT_REGS_PARM2_CORE_SYSCALL!($regs) as c_ulonglong }; }
macro_rules! ___bpf_syswrap_args3 { ($ctx:expr, $regs:expr) => { ___bpf_syswrap_args2!($ctx, $regs), PT_REGS_PARM3_CORE_SYSCALL!($regs) as c_ulonglong }; }
macro_rules! ___bpf_syswrap_args4 { ($ctx:expr, $regs:expr) => { ___bpf_syswrap_args3!($ctx, $regs), PT_REGS_PARM4_CORE_SYSCALL!($regs) as c_ulonglong }; }
macro_rules! ___bpf_syswrap_args5 { ($ctx:expr, $regs:expr) => { ___bpf_syswrap_args4!($ctx, $regs), PT_REGS_PARM5_CORE_SYSCALL!($regs) as c_ulonglong }; }
macro_rules! ___bpf_syswrap_args6 { ($ctx:expr, $regs:expr) => { ___bpf_syswrap_args5!($ctx, $regs), PT_REGS_PARM6_CORE_SYSCALL!($regs) as c_ulonglong }; }
macro_rules! ___bpf_syswrap_args7 { ($ctx:expr, $regs:expr) => { ___bpf_syswrap_args6!($ctx, $regs), PT_REGS_PARM7_CORE_SYSCALL!($regs) as c_ulonglong }; }

/*
 * BPF_KSYSCALL is a variant of BPF_KPROBE intended for tracing syscall
 * functions, like __x64_sys_close. It relies on BPF CO-RE support and virtual
 * __kconfig externs in C.
 */
extern "C" {
    pub static LINUX_HAS_SYSCALL_WRAPPER: bool;
}

macro_rules! BPF_KSYSCALL {
    ($($tokens:tt)*) => {
        compile_error!("BPF_KSYSCALL declaration macro requires C preprocessor semantics and must be expanded by a C-compatible BPF build path")
    };
}

macro_rules! BPF_KPROBE_SYSCALL {
    ($($tokens:tt)*) => {
        BPF_KSYSCALL!($($tokens)*)
    };
}

/*
 * BPF_UPROBE and BPF_URETPROBE are identical to BPF_KPROBE and BPF_KRETPROBE,
 * but are named way less confusingly for SEC("uprobe") and SEC("uretprobe")
 * use cases.
 */
macro_rules! BPF_UPROBE {
    ($name:ident $(, $args:tt)*) => {
        BPF_KPROBE!($name $(, $args)*)
    };
}

macro_rules! BPF_URETPROBE {
    ($name:ident $(, $args:tt)*) => {
        BPF_KRETPROBE!($name $(, $args)*)
    };
}
