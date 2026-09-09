/* SPDX-License-Identifier: GPL-2.0 */
//! Rust translation of the Linux regmap tracepoint header.
//!
//! The C source uses the Linux tracepoint declaration macros.  Their generated
//! implementations are supplied by the tracepoint subsystem; this file keeps
//! the corresponding externally visible event-call interfaces as FFI items.

use core::ffi::{c_char, c_int, c_void};

/// Opaque `struct regmap` supplied by the regmap implementation.
#[repr(C)]
pub struct Regmap {
    _private: [u8; 0],
}

extern "C" {
    /// `DEFINE_EVENT(regmap_reg, regmap_reg_write, ...)`
    pub fn trace_regmap_reg_write(map: *mut Regmap, reg: u32, val: u32);

    /// `DEFINE_EVENT(regmap_reg, regmap_reg_read, ...)`
    pub fn trace_regmap_reg_read(map: *mut Regmap, reg: u32, val: u32);

    /// `DEFINE_EVENT(regmap_reg, regmap_reg_read_cache, ...)`
    pub fn trace_regmap_reg_read_cache(map: *mut Regmap, reg: u32, val: u32);

    /// `DEFINE_EVENT(regmap_bulk, regmap_bulk_write, ...)`
    pub fn trace_regmap_bulk_write(
        map: *mut Regmap,
        reg: u32,
        val: *const c_void,
        val_len: c_int,
    );

    /// `DEFINE_EVENT(regmap_bulk, regmap_bulk_read, ...)`
    pub fn trace_regmap_bulk_read(
        map: *mut Regmap,
        reg: u32,
        val: *const c_void,
        val_len: c_int,
    );

    /// `DEFINE_EVENT(regmap_block, regmap_hw_read_start, ...)`
    pub fn trace_regmap_hw_read_start(map: *mut Regmap, reg: u32, count: c_int);

    /// `DEFINE_EVENT(regmap_block, regmap_hw_read_done, ...)`
    pub fn trace_regmap_hw_read_done(map: *mut Regmap, reg: u32, count: c_int);

    /// `DEFINE_EVENT(regmap_block, regmap_hw_write_start, ...)`
    pub fn trace_regmap_hw_write_start(map: *mut Regmap, reg: u32, count: c_int);

    /// `DEFINE_EVENT(regmap_block, regmap_hw_write_done, ...)`
    pub fn trace_regmap_hw_write_done(map: *mut Regmap, reg: u32, count: c_int);

    /// `TRACE_EVENT(regcache_sync, ...)`
    pub fn trace_regcache_sync(
        map: *mut Regmap,
        type_: *const c_char,
        status: *const c_char,
    );

    /// `DEFINE_EVENT(regmap_bool, regmap_cache_only, ...)`
    pub fn trace_regmap_cache_only(map: *mut Regmap, flag: bool);

    /// `DEFINE_EVENT(regmap_bool, regmap_cache_bypass, ...)`
    pub fn trace_regmap_cache_bypass(map: *mut Regmap, flag: bool);

    /// `DEFINE_EVENT(regmap_block, regmap_async_write_start, ...)`
    pub fn trace_regmap_async_write_start(map: *mut Regmap, reg: u32, count: c_int);

    /// `DEFINE_EVENT(regmap_async, regmap_async_io_complete, ...)`
    pub fn trace_regmap_async_io_complete(map: *mut Regmap);

    /// `DEFINE_EVENT(regmap_async, regmap_async_complete_start, ...)`
    pub fn trace_regmap_async_complete_start(map: *mut Regmap);

    /// `DEFINE_EVENT(regmap_async, regmap_async_complete_done, ...)`
    pub fn trace_regmap_async_complete_done(map: *mut Regmap);

    /// `TRACE_EVENT(regcache_drop_region, ...)`
    pub fn trace_regcache_drop_region(map: *mut Regmap, from: u32, to: u32);
}

// Event payloads and print formats retained from the C declarations:
//
// regmap_reg:   name=regmap_name(map), reg: unsigned int, val: unsigned int;
//               "%s reg=%x val=%x"
// regmap_bulk:  name=regmap_name(map), reg: unsigned int, buf: char[val_len],
//               val_len: int; copies `val` with `memcpy`; "%s reg=%x val=%s"
// regmap_block: name=regmap_name(map), reg: unsigned int, count: int;
//               "%s reg=%x count=%d"
// regcache_sync: name=regmap_name(map), status: status, type: type;
//                "%s type=%s status=%s"
// regmap_bool:  name=regmap_name(map), flag: int; "%s flag=%d"
// regmap_async: name=regmap_name(map); "%s"
// regcache_drop_region: name=regmap_name(map), from: unsigned int,
//                       to: unsigned int; "%s %u-%u"

// The C include guard and TRACE_INCLUDE_PATH/TRACE_INCLUDE_FILE directives are
// preprocessor-only.  The tracepoint definition header is an external
// dependency and is intentionally not reimplemented here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
