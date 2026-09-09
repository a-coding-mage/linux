// SPDX-License-Identifier: GPL-2.0

// C dependencies: vmlinux.h, bpf tracing/core-read/helpers, net_shared.h,
// and xdp_sample_shared.h provide the referenced BPF types and definitions.

pub const EINVAL: i32 = 22;
pub const ENETDOWN: i32 = 100;
pub const EMSGSIZE: i32 = 90;
pub const EOPNOTSUPP: i32 = 95;
pub const ENOSPC: i32 = 28;

// C map declaration:
//   __uint(type, BPF_MAP_TYPE_ARRAY);
//   __uint(map_flags, BPF_F_MMAPABLE);
//   __type(key, unsigned int);
//   __type(value, struct datarec);
// The map metadata is supplied by the BPF toolchain.
#[allow(non_camel_case_types)]
pub struct array_map;

extern "C" {
    pub static mut rx_cnt: array_map;
    pub static nr_cpus: i32;
}

pub const XDP_REDIRECT_SUCCESS: i32 = 0;
pub const XDP_REDIRECT_ERROR: i32 = 1;

#[inline(always)]
pub unsafe fn swap_src_dst_mac(data: *mut core::ffi::c_void) {
    let p = data as *mut u16;
    let dst = [*p.add(0), *p.add(1), *p.add(2)];
    *p.add(0) = *p.add(3);
    *p.add(1) = *p.add(4);
    *p.add(2) = *p.add(5);
    *p.add(3) = dst[0];
    *p.add(4) = dst[1];
    *p.add(5) = dst[2];
}

/*
 * The original C code defines these aliases with __may_alias__ so that the
 * kernel's READ_ONCE/WRITE_ONCE operations do not violate aliasing rules.
 */
pub type __u8_alias_t = u8;
pub type __u16_alias_t = u16;
pub type __u32_alias_t = u32;
pub type __u64_alias_t = u64;

#[inline(always)]
pub unsafe fn __read_once_size(p: *const core::ffi::c_void, res: *mut core::ffi::c_void, size: i32) {
    match size {
        1 => *(res as *mut u8) = core::ptr::read_volatile(p as *const u8),
        2 => *(res as *mut u16) = core::ptr::read_volatile(p as *const u16),
        4 => *(res as *mut u32) = core::ptr::read_volatile(p as *const u32),
        8 => *(res as *mut u64) = core::ptr::read_volatile(p as *const u64),
        _ => {
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
            core::ptr::copy_nonoverlapping(p as *const u8, res as *mut u8, size as usize);
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        }
    }
}

#[inline(always)]
pub unsafe fn __write_once_size(p: *mut core::ffi::c_void, res: *const core::ffi::c_void, size: i32) {
    match size {
        1 => core::ptr::write_volatile(p as *mut u8, *(res as *const u8)),
        2 => core::ptr::write_volatile(p as *mut u16, *(res as *const u16)),
        4 => core::ptr::write_volatile(p as *mut u32, *(res as *const u32)),
        8 => core::ptr::write_volatile(p as *mut u64, *(res as *const u64)),
        _ => {
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
            core::ptr::copy_nonoverlapping(res as *const u8, p as *mut u8, size as usize);
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        }
    }
}

#[macro_export]
macro_rules! READ_ONCE {
    ($x:expr) => {{
        let __p = &($x) as *const _;
        unsafe { core::ptr::read_volatile(__p) }
    }};
}

#[macro_export]
macro_rules! WRITE_ONCE {
    ($x:expr, $val:expr) => {{
        unsafe { core::ptr::write_volatile(&mut ($x) as *mut _, $val); }
        $x
    }};
}

#[macro_export]
macro_rules! NO_TEAR_ADD {
    ($x:expr, $val:expr) => { $crate::WRITE_ONCE!($x, $crate::READ_ONCE!($x).wrapping_add($val)) };
}

#[macro_export]
macro_rules! NO_TEAR_INC {
    ($x:expr) => { $crate::NO_TEAR_ADD!($x, 1) };
}

#[macro_export]
macro_rules! ARRAY_SIZE {
    ($x:expr) => { ($x.len()) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
