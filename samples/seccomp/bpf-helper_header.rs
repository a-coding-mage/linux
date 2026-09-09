/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Example wrapper around BPF macros.
 *
 * Copyright (c) 2012 The Chromium OS Authors <chromium-os-dev@chromium.org>
 * Author: Will Drewry <wad@chromium.org>
 *
 * The code may be used by anyone for any purpose,
 * and can serve as a starting point for developing
 * applications using prctl(PR_SET_SECCOMP, 2, ...).
 *
 * No guarantees are provided with respect to the correctness
 * or functionality of this code.
 */

/* External kernel/BPF types and macros are supplied by dependent bindings. */

pub const BPF_LABELS_MAX: usize = 256;

#[repr(C)]
pub struct bpf_labels {
    pub count: ::core::ffi::c_int,
    pub labels: [__bpf_label; BPF_LABELS_MAX],
}

#[repr(C)]
pub struct __bpf_label {
    pub label: *const ::core::ffi::c_char,
    pub location: __u32,
}

unsafe extern "C" {
    pub fn bpf_resolve_jumps(labels: *mut bpf_labels, filter: *mut sock_filter, count: usize) -> ::core::ffi::c_int;
    pub fn seccomp_bpf_label(labels: *mut bpf_labels, label: *const ::core::ffi::c_char) -> __u32;
    pub fn seccomp_bpf_print(filter: *mut sock_filter, count: usize);
}

pub const JUMP_JT: u32 = 0xff;
pub const JUMP_JF: u32 = 0xff;
pub const LABEL_JT: u32 = 0xfe;
pub const LABEL_JF: u32 = 0xfe;

#[macro_export]
macro_rules! ALLOW { () => { BPF_STMT(BPF_RET + BPF_K, SECCOMP_RET_ALLOW) }; }
#[macro_export]
macro_rules! DENY { () => { BPF_STMT(BPF_RET + BPF_K, SECCOMP_RET_KILL) }; }
#[macro_export]
macro_rules! JUMP { ($labels:expr, $label:ident) => { BPF_JUMP(BPF_JMP + BPF_JA, FIND_LABEL!($labels, $label), JUMP_JT, JUMP_JF) }; }
#[macro_export]
macro_rules! LABEL { ($labels:expr, $label:ident) => { BPF_JUMP(BPF_JMP + BPF_JA, FIND_LABEL!($labels, $label), LABEL_JT, LABEL_JF) }; }
#[macro_export]
macro_rules! SYSCALL { ($nr:expr, $jt:expr) => { BPF_JUMP(BPF_JMP + BPF_JEQ + BPF_K, $nr, 0, 1), $jt }; }
#[macro_export]
macro_rules! FIND_LABEL { ($labels:expr, $label:ident) => { unsafe { seccomp_bpf_label($labels, concat!(stringify!($label), "\0").as_ptr() as *const ::core::ffi::c_char) } }; }
#[macro_export]
macro_rules! EXPAND { ($($tokens:tt)*) => { $($tokens)* }; }

/* Build-time endianness and word-size conditions from the original header. */
#[cfg(target_endian = "little")]
#[macro_export]
macro_rules! LO_ARG { ($idx:expr) => { core::mem::offset_of!(seccomp_data, args) + ($idx) * core::mem::size_of::<__u64>() }; }
#[cfg(target_endian = "big")]
#[macro_export]
macro_rules! LO_ARG { ($idx:expr) => { core::mem::offset_of!(seccomp_data, args) + ($idx) * core::mem::size_of::<__u64>() + core::mem::size_of::<__u32>() }; }

#[repr(C)]
pub union arg64 {
    pub parts: arg64_parts,
    pub u64: __u64,
}
#[repr(C)]
pub struct arg64_parts { pub lo32: __u32, pub hi32: __u32 }

#[macro_export]
macro_rules! ARG_32 { ($idx:expr) => { BPF_STMT(BPF_LD + BPF_W + BPF_ABS, LO_ARG!($idx)) }; }
#[macro_export]
macro_rules! ARG_64 { ($idx:expr) => { BPF_STMT(BPF_LD + BPF_W + BPF_ABS, LO_ARG!($idx)), BPF_STMT(BPF_ST, 0), BPF_STMT(BPF_LD + BPF_W + BPF_ABS, HI_ARG!($idx)), BPF_STMT(BPF_ST, 1) }; }

#[cfg(target_pointer_width = "32")]
#[macro_export]
macro_rules! ARG { ($i:expr) => { ARG_32!($i) }; }
#[cfg(target_pointer_width = "64")]
#[macro_export]
macro_rules! ARG { ($i:expr) => { ARG_64!($i) }; }

#[macro_export]
macro_rules! JEQ32 { ($value:expr, $jt:expr) => { BPF_JUMP(BPF_JMP + BPF_JEQ + BPF_K, $value, 0, 1), $jt }; }
#[macro_export]
macro_rules! JNE32 { ($value:expr, $jt:expr) => { BPF_JUMP(BPF_JMP + BPF_JEQ + BPF_K, $value, 1, 0), $jt }; }
#[macro_export]
macro_rules! JA32 { ($value:expr, $jt:expr) => { BPF_JUMP(BPF_JMP + BPF_JSET + BPF_K, $value, 0, 1), $jt }; }
#[macro_export]
macro_rules! JGE32 { ($value:expr, $jt:expr) => { BPF_JUMP(BPF_JMP + BPF_JGE + BPF_K, $value, 0, 1), $jt }; }
#[macro_export]
macro_rules! JGT32 { ($value:expr, $jt:expr) => { BPF_JUMP(BPF_JMP + BPF_JGT + BPF_K, $value, 0, 1), $jt }; }
#[macro_export]
macro_rules! JLE32 { ($value:expr, $jt:expr) => { BPF_JUMP(BPF_JMP + BPF_JGT + BPF_K, $value, 1, 0), $jt }; }
#[macro_export]
macro_rules! JLT32 { ($value:expr, $jt:expr) => { BPF_JUMP(BPF_JMP + BPF_JGE + BPF_K, $value, 1, 0), $jt }; }

/* The 64-bit checks preserve the original M[0]/M[1] and A register invariant. */
#[macro_export]
macro_rules! JEQ64 { ($lo:expr, $hi:expr, $jt:expr) => { BPF_JUMP(BPF_JMP+BPF_JEQ+BPF_K, $hi, 0, 5), BPF_STMT(BPF_LD+BPF_MEM, 0), BPF_JUMP(BPF_JMP+BPF_JEQ+BPF_K, $lo, 0, 2), BPF_STMT(BPF_LD+BPF_MEM, 1), $jt, BPF_STMT(BPF_LD+BPF_MEM, 1) }; }
#[macro_export]
macro_rules! JNE64 { ($lo:expr, $hi:expr, $jt:expr) => { BPF_JUMP(BPF_JMP+BPF_JEQ+BPF_K, $hi, 0, 3), BPF_STMT(BPF_LD+BPF_MEM, 0), BPF_JUMP(BPF_JMP+BPF_JEQ+BPF_K, $lo, 2, 0), BPF_STMT(BPF_LD+BPF_MEM, 1), $jt, BPF_STMT(BPF_LD+BPF_MEM, 1) }; }
#[macro_export]
macro_rules! JA64 { ($lo:expr, $hi:expr, $jt:expr) => { BPF_JUMP(BPF_JMP+BPF_JSET+BPF_K, $hi, 3, 0), BPF_STMT(BPF_LD+BPF_MEM, 0), BPF_JUMP(BPF_JMP+BPF_JSET+BPF_K, $lo, 0, 2), BPF_STMT(BPF_LD+BPF_MEM, 1), $jt, BPF_STMT(BPF_LD+BPF_MEM, 1) }; }
#[macro_export]
macro_rules! JGE64 { ($lo:expr, $hi:expr, $jt:expr) => { BPF_JUMP(BPF_JMP+BPF_JGT+BPF_K, $hi, 4, 0), BPF_JUMP(BPF_JMP+BPF_JEQ+BPF_K, $hi, 0, 5), BPF_STMT(BPF_LD+BPF_MEM, 0), BPF_JUMP(BPF_JMP+BPF_JGE+BPF_K, $lo, 0, 2), BPF_STMT(BPF_LD+BPF_MEM, 1), $jt, BPF_STMT(BPF_LD+BPF_MEM, 1) }; }
#[macro_export]
macro_rules! JGT64 { ($lo:expr, $hi:expr, $jt:expr) => { BPF_JUMP(BPF_JMP+BPF_JGT+BPF_K, $hi, 4, 0), BPF_JUMP(BPF_JMP+BPF_JEQ+BPF_K, $hi, 0, 5), BPF_STMT(BPF_LD+BPF_MEM, 0), BPF_JUMP(BPF_JMP+BPF_JGT+BPF_K, $lo, 0, 2), BPF_STMT(BPF_LD+BPF_MEM, 1), $jt, BPF_STMT(BPF_LD+BPF_MEM, 1) }; }
#[macro_export]
macro_rules! JLE64 { ($lo:expr, $hi:expr, $jt:expr) => { BPF_JUMP(BPF_JMP+BPF_JGE+BPF_K, $hi, 0, 4), BPF_JUMP(BPF_JMP+BPF_JEQ+BPF_K, $hi, 0, 5), BPF_STMT(BPF_LD+BPF_MEM, 0), BPF_JUMP(BPF_JMP+BPF_JGT+BPF_K, $lo, 2, 0), BPF_STMT(BPF_LD+BPF_MEM, 1), $jt, BPF_STMT(BPF_LD+BPF_MEM, 1) }; }
#[macro_export]
macro_rules! JLT64 { ($lo:expr, $hi:expr, $jt:expr) => { BPF_JUMP(BPF_JMP+BPF_JGE+BPF_K, $hi, 0, 4), BPF_JUMP(BPF_JMP+BPF_JEQ+BPF_K, $hi, 0, 5), BPF_STMT(BPF_LD+BPF_MEM, 0), BPF_JUMP(BPF_JMP+BPF_JGE+BPF_K, $lo, 2, 0), BPF_STMT(BPF_LD+BPF_MEM, 1), $jt, BPF_STMT(BPF_LD+BPF_MEM, 1) }; }

#[macro_export]
macro_rules! LOAD_SYSCALL_NR { () => { BPF_STMT(BPF_LD + BPF_W + BPF_ABS, core::mem::offset_of!(seccomp_data, nr)) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
