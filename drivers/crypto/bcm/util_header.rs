/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2016 Broadcom
 */

/* Translated from util.h. Kernel and spu declarations are supplied externally. */

extern "C" {
    pub static mut flow_debug_logging: i32;
    pub static mut packet_debug_logging: i32;
    pub static mut debug_logging_sleep: i32;

    pub fn __dump_sg(sg: *mut scatterlist, skip: u32, len: u32);

    pub fn spu_sg_at_offset(
        sg: *mut scatterlist,
        skip: u32,
        sge: *mut *mut scatterlist,
        sge_offset: *mut u32,
    ) -> i32;
    pub fn sg_copy_part_to_buf(src: *mut scatterlist, dest: *mut u8, len: u32, skip: u32);
    pub fn sg_copy_part_from_buf(dest: *mut scatterlist, src: *mut u8, len: u32, skip: u32);
    pub fn spu_sg_count(sg_list: *mut scatterlist, skip: u32, nbytes: i32) -> i32;
    pub fn spu_msg_sg_add(
        to_sg: *mut *mut scatterlist,
        from_sg: *mut *mut scatterlist,
        skip: *mut u32,
        from_nents: u8,
        tot_len: u32,
    ) -> u32;

    pub fn add_to_ctr(ctr_pos: *mut u8, increment: u32);
    pub fn do_shash(
        name: *mut u8,
        result: *mut u8,
        data1: *const u8,
        data1_len: u32,
        data2: *const u8,
        data2_len: u32,
        key: *const u8,
        key_len: u32,
    ) -> i32;
    pub fn spu_alg_name(alg: spu_cipher_alg, mode: spu_cipher_mode) -> *mut i8;
    pub fn spu_setup_debugfs();
    pub fn spu_free_debugfs();
    pub fn format_value_ccm(val: u32, buf: *mut u8, len: u8);
}

/* Supplied by spu.h and the kernel scatterlist definitions. */
pub enum scatterlist {}
pub type spu_cipher_alg = u32;
pub type spu_cipher_mode = u32;

#[cfg(feature = "DEBUG")]
#[macro_export]
macro_rules! flow_log {
    ($($arg:tt)*) => {{
        if unsafe { $crate::flow_debug_logging != 0 } {
            unsafe { printk!($($arg)*); }
            if unsafe { $crate::debug_logging_sleep != 0 } {
                unsafe { msleep($crate::debug_logging_sleep as u64); }
            }
        }
    }};
}

#[cfg(feature = "DEBUG")]
#[macro_export]
macro_rules! packet_log {
    ($($arg:tt)*) => {{
        if unsafe { $crate::packet_debug_logging != 0 } {
            unsafe { printk!($($arg)*); }
            if unsafe { $crate::debug_logging_sleep != 0 } {
                unsafe { msleep($crate::debug_logging_sleep as u64); }
            }
        }
    }};
}

#[cfg(feature = "DEBUG")]
#[macro_export]
macro_rules! flow_dump {
    ($msg:expr, $var:expr, $var_len:expr) => {{
        if unsafe { $crate::flow_debug_logging != 0 } {
            unsafe { print_hex_dump!(KERN_ALERT, $msg, DUMP_PREFIX_NONE, 16, 1, $var, $var_len, false); }
            if unsafe { $crate::debug_logging_sleep != 0 } {
                unsafe { msleep($crate::debug_logging_sleep as u64); }
            }
        }
    }};
}

#[cfg(feature = "DEBUG")]
#[macro_export]
macro_rules! packet_dump {
    ($msg:expr, $var:expr, $var_len:expr) => {{
        if unsafe { $crate::packet_debug_logging != 0 } {
            unsafe { print_hex_dump!(KERN_ALERT, $msg, DUMP_PREFIX_NONE, 16, 1, $var, $var_len, false); }
            if unsafe { $crate::debug_logging_sleep != 0 } {
                unsafe { msleep($crate::debug_logging_sleep as u64); }
            }
        }
    }};
}

#[macro_export]
macro_rules! dump_sg {
    ($sg:expr, $skip:expr, $len:expr) => {{
        unsafe { $crate::__dump_sg($sg, $skip, $len) }
    }};
}

#[cfg(not(feature = "DEBUG"))]
#[inline]
pub fn flow_log() {}

#[cfg(not(feature = "DEBUG"))]
#[inline]
pub fn flow_dump(_msg: *const i8, _var: *const core::ffi::c_void, _var_len: usize) {}

#[cfg(not(feature = "DEBUG"))]
#[inline]
pub fn packet_log() {}

#[cfg(not(feature = "DEBUG"))]
#[inline]
pub fn packet_dump(_msg: *const i8, _var: *const core::ffi::c_void, _var_len: usize) {}

#[cfg(not(feature = "DEBUG"))]
#[inline]
pub fn dump_sg(_sg: *mut scatterlist, _skip: u32, _len: u32) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
