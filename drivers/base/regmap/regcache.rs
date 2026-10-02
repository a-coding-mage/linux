// SPDX-License-Identifier: GPL-2.0
//! Register cache access API, preserving the original regcache.c state machine.
//!
//! The caller supplies a live regmap and serializes cache access as specified by
//! the C API. Configuration, callbacks and buffers retain their original C
//! lifetimes. Public entry points take the original locks where required.
#![allow(missing_docs, unsafe_op_in_unsafe_fn)]

#[allow(
    clippy::all,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    improper_ctypes,
    unreachable_pub
)]
mod bindings {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/regcache_generated.rs"
    ));
}
#[path = "../../../rust/ffi_export.rs"]
mod ffi_export;

use bindings::*;
use core::{ffi::c_void, mem, num::NonZeroU32, ptr};
use kernel::ffi::{c_int, c_ulong};

const EINVAL_: c_int = -(EINVAL as c_int);
const ENOMEM_: c_int = -(ENOMEM as c_int);
const ENOENT_: c_int = -(ENOENT as c_int);

// regmap initialization replaces a zero configured stride with one. Every C
// caller of the remainder/division paths requires a nonzero map stride; a zero
// divisor in those original expressions is undefined behavior. Keep that
// invariant explicit so Rust does not introduce a new panic-only control path.
unsafe fn nonzero_stride(map: *mut regmap) -> NonZeroU32 {
    NonZeroU32::new_unchecked((*map).reg_stride as u32)
}

unsafe fn cache_ops(map: *mut regmap) -> *const regcache_ops {
    if (*map).cache_ops.is_null() {
        kernel::bindings::BUG();
    }
    (*map).cache_ops
}

unsafe fn lock(map: *mut regmap) {
    ((*map).lock.unwrap_unchecked())((*map).lock_arg);
}
unsafe fn unlock(map: *mut regmap) {
    ((*map).unlock.unwrap_unchecked())((*map).lock_arg);
}

unsafe extern "C" fn regcache_defaults_cmp(a: *const c_void, b: *const c_void) -> c_int {
    let a = (*a.cast::<reg_default>()).reg;
    let b = (*b.cast::<reg_default>()).reg;
    if a > b {
        1
    } else if a < b {
        -1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn regcache_sort_defaults(defaults: *mut reg_default, ndefaults: u32) {
    sort(
        defaults.cast(),
        ndefaults as usize,
        mem::size_of::<reg_default>(),
        Some(regcache_defaults_cmp),
        None,
    );
}

unsafe fn regcache_count_cacheable_registers(map: *mut regmap) -> c_int {
    let mut count = 0u32;
    for i in 0..(*map).num_reg_defaults_raw {
        let reg = i.wrapping_mul((*map).reg_stride as u32);
        if regmap_readable(map, reg) && !regmap_volatile(map, reg) {
            count = count.wrapping_add(1);
        }
    }
    count as c_int
}

unsafe fn regcache_hw_init(map: *mut regmap) -> c_int {
    if (*map).reg_defaults_raw.is_null() {
        let bypass = (*map).cache_bypass;
        rust_regcache_dbg_hw((*map).dev);
        (*map).cache_bypass = true;
        let buffer = rust_regcache_kmalloc((*map).cache_size_raw as usize);
        if buffer.is_null() {
            // The C allocation-failure path also leaves bypass set.
            return ENOMEM_;
        }
        let ret = regmap_raw_read(map, 0, buffer, (*map).cache_size_raw as usize);
        (*map).cache_bypass = bypass;
        if ret == 0 {
            (*map).reg_defaults_raw = buffer;
            (*map).cache_free = true;
        } else {
            kfree(buffer);
        }
    }

    let mut j = 0usize;
    for i in 0..(*map).num_reg_defaults_raw {
        let reg = i.wrapping_mul((*map).reg_stride as u32);
        if !regmap_readable(map, reg) || regmap_volatile(map, reg) {
            continue;
        }
        let mut value = 0;
        if !(*map).reg_defaults_raw.is_null() {
            value = regcache_get_val(map, (*map).reg_defaults_raw, i);
        } else {
            let bypass = (*map).cache_bypass;
            (*map).cache_bypass = true;
            let ret = regmap_read(map, reg, &mut value);
            (*map).cache_bypass = bypass;
            if ret != 0 {
                rust_regcache_err_read((*map).dev, reg, ret);
                return ret;
            }
        }
        (*map)
            .reg_defaults
            .add(j)
            .write(reg_default { reg, def: value });
        j += 1;
    }
    0
}

unsafe fn regcache_hw_exit(map: *mut regmap) {
    if (*map).cache_free {
        kfree((*map).reg_defaults_raw);
    }
}

// Shared original err_exit/normal-exit callback, always under the map lock.
unsafe fn exit_backend(map: *mut regmap) {
    if let Some(exit) = (*(*map).cache_ops).exit {
        rust_regcache_dbg_exit((*map).dev, (*(*map).cache_ops).name);
        lock(map);
        exit(map);
        unlock(map);
    }
}

#[no_mangle]
pub unsafe extern "C" fn regcache_init(map: *mut regmap, config: *const regmap_config) -> c_int {
    if (*map).cache_type == regcache_type_REGCACHE_NONE {
        if !(*config).reg_defaults.is_null() || (*config).num_reg_defaults_raw != 0 {
            rust_regcache_warn_defaults((*map).dev);
        }
        (*map).cache_bypass = true;
        return 0;
    }
    if !(*config).reg_defaults.is_null() && (*config).num_reg_defaults == 0 {
        rust_regcache_err_count((*map).dev);
        return EINVAL_;
    }
    if (*config).num_reg_defaults != 0 && (*config).reg_defaults.is_null() {
        rust_regcache_err_defaults((*map).dev);
        return EINVAL_;
    }

    let mut sort_defaults = false;
    let mut previous = 0;
    for i in 0..(*config).num_reg_defaults {
        let reg = (*(*config).reg_defaults.add(i as usize)).reg;
        if reg % nonzero_stride(map) != 0 {
            return EINVAL_;
        }
        if previous > reg {
            sort_defaults = true;
        }
        previous = reg;
    }
    let types = [
        ptr::addr_of!(regcache_flat_sparse_ops),
        ptr::addr_of!(regcache_rbtree_ops),
        ptr::addr_of!(regcache_maple_ops),
        ptr::addr_of!(regcache_flat_ops),
    ];
    let mut selected = ptr::null();
    for ops in types {
        if (*ops).type_ == (*map).cache_type {
            selected = ops;
            break;
        }
    }
    if selected.is_null() {
        rust_regcache_err_type((*map).dev, (*map).cache_type);
        return EINVAL_;
    }

    (*map).num_reg_defaults = (*config).num_reg_defaults;
    (*map).num_reg_defaults_raw = (*config).num_reg_defaults_raw;
    (*map).reg_defaults_raw = (*config).reg_defaults_raw;
    (*map).cache_word_size = (((*config).val_bits as usize).wrapping_add(7) / 8) as u32;
    (*map).cache_size_raw = (*map)
        .cache_word_size
        .wrapping_mul((*config).num_reg_defaults_raw);
    (*map).cache = ptr::null_mut();
    (*map).cache_ops = selected;
    if (*selected).read.is_none() || (*selected).write.is_none() || (*selected).name.is_null() {
        return EINVAL_;
    }

    let mut count = 0;
    if !(*config).reg_defaults.is_null() {
        let defaults = rust_regcache_dup_defaults((*config).reg_defaults, (*map).num_reg_defaults)
            .cast::<reg_default>();
        if defaults.is_null() {
            return ENOMEM_;
        }
        if sort_defaults {
            rust_regcache_warn_unsorted((*map).dev);
            regcache_sort_defaults(defaults, (*map).num_reg_defaults);
        }
        (*map).reg_defaults = defaults;
    } else if (*map).num_reg_defaults_raw != 0 {
        count = regcache_count_cacheable_registers(map);
        if count == 0 {
            (*map).cache_bypass = true;
        }
        if (*map).cache_bypass {
            return 0;
        }
        (*map).num_reg_defaults = count as u32;
        (*map).reg_defaults = rust_regcache_alloc_defaults(count);
        if (*map).reg_defaults.is_null() {
            return ENOMEM_;
        }
    }
    if !(*map).max_register_is_set && (*map).num_reg_defaults_raw != 0 {
        (*map).max_register = (*map)
            .num_reg_defaults_raw
            .wrapping_sub(1)
            .wrapping_mul((*map).reg_stride as u32);
        (*map).max_register_is_set = true;
    }
    if let Some(init) = (*selected).init {
        rust_regcache_dbg_init((*map).dev, (*selected).name);
        lock(map);
        let ret = init(map);
        unlock(map);
        if ret != 0 {
            kfree((*map).reg_defaults.cast());
            return ret;
        }
    }
    if count != 0 {
        let ret = regcache_hw_init(map);
        if ret != 0 {
            // C's err_exit does not free reg_defaults_raw on this path.
            exit_backend(map);
            kfree((*map).reg_defaults.cast());
            return ret;
        }
    }
    if let Some(populate) = (*selected).populate {
        if (*map).num_reg_defaults != 0 || (*map).reg_default_cb.is_some() {
            rust_regcache_dbg_populate((*map).dev, (*selected).name);
            lock(map);
            let ret = populate(map);
            unlock(map);
            if ret != 0 {
                regcache_hw_exit(map);
                exit_backend(map);
                kfree((*map).reg_defaults.cast());
                return ret;
            }
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn regcache_exit(map: *mut regmap) {
    if (*map).cache_type == regcache_type_REGCACHE_NONE {
        return;
    }
    cache_ops(map);
    regcache_hw_exit(map);
    exit_backend(map);
    kfree((*map).reg_defaults.cast());
}

#[no_mangle]
pub unsafe extern "C" fn regcache_read(map: *mut regmap, reg: u32, value: *mut u32) -> c_int {
    if (*map).cache_type == regcache_type_REGCACHE_NONE {
        return EINVAL_;
    }
    let ops = cache_ops(map);
    if !regmap_volatile(map, reg) {
        let ret = ((*ops).read.unwrap_unchecked())(map, reg, value);
        if ret == 0 {
            rust_regcache_trace_read(map, reg, *value);
        }
        return ret;
    }
    EINVAL_
}

#[no_mangle]
pub unsafe extern "C" fn regcache_write(map: *mut regmap, reg: u32, value: u32) -> c_int {
    if (*map).cache_type == regcache_type_REGCACHE_NONE {
        return 0;
    }
    let ops = cache_ops(map);
    if !regmap_volatile(map, reg) {
        return ((*ops).write.unwrap_unchecked())(map, reg, value);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn regcache_reg_needs_sync(map: *mut regmap, reg: u32, value: u32) -> bool {
    if !regmap_writeable(map, reg) {
        return false;
    }
    if !(*map).no_sync_defaults {
        return true;
    }
    let index = regcache_lookup_reg(map, reg);
    !(index >= 0 && value == (*(*map).reg_defaults.add(index as usize)).def)
}

unsafe fn regcache_default_sync(map: *mut regmap, min: u32, max: u32) -> c_int {
    let mut reg = min;
    while reg <= max {
        if !regmap_volatile(map, reg) && regmap_writeable(map, reg) {
            let mut value = 0;
            let ret = regcache_read(map, reg, &mut value);
            if ret != ENOENT_ {
                if ret != 0 {
                    return ret;
                }
                if regcache_reg_needs_sync(map, reg, value) {
                    (*map).cache_bypass = true;
                    let ret = _regmap_write(map, reg, value);
                    (*map).cache_bypass = false;
                    if ret != 0 {
                        rust_regcache_err_sync((*map).dev, reg, ret);
                        return ret;
                    }
                    rust_regcache_dbg_synced((*map).dev, reg, value);
                }
            }
        }
        reg = reg.wrapping_add((*map).reg_stride as u32);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn regcache_sync(map: *mut regmap) -> c_int {
    if rust_regcache_warn((*map).cache_type == regcache_type_REGCACHE_NONE) {
        return EINVAL_;
    }
    let ops = cache_ops(map);
    lock(map);
    if rust_regcache_warn((*map).cache_only) {
        unlock(map);
        return EINVAL_;
    }
    let bypass = (*map).cache_bypass;
    let name = (*ops).name;
    rust_regcache_dbg_sync((*map).dev, name);
    rust_regcache_trace_sync(map, name, c"start".as_ptr().cast());
    let mut sync_ret = 0;
    if (*map).cache_dirty {
        (*map).cache_bypass = true;
        for i in 0..(*map).patch_regs {
            let patch = (*map).patch.add(i as usize);
            sync_ret = _regmap_write(map, (*patch).reg, (*patch).def);
            if sync_ret != 0 {
                rust_regcache_err_write((*map).dev, (*patch).reg, (*patch).def, sync_ret);
                break;
            }
        }
        if sync_ret == 0 {
            (*map).cache_bypass = false;
            sync_ret = if let Some(sync) = (*ops).sync {
                sync(map, 0, (*map).max_register)
            } else {
                regcache_default_sync(map, 0, (*map).max_register)
            };
            if sync_ret == 0 {
                (*map).cache_dirty = false;
            }
        }
    }
    (*map).cache_bypass = bypass;
    (*map).no_sync_defaults = false;

    // rb_for_each(..., rbtree_all) visits every node in order: the comparator
    // always returns zero, so rb_first/rb_next are exactly the same traversal.
    let mut selector_ret = 0;
    let mut node = rust_regcache_rb_first(ptr::addr_of!((*map).range_tree));
    while !node.is_null() {
        let range = node
            .cast::<u8>()
            .sub(mem::offset_of!(regmap_range_node, node))
            .cast::<regmap_range_node>();
        let reg = (*range).selector_reg;
        let mut value = 0;
        if regcache_read(map, reg, &mut value) == 0 {
            selector_ret = _regmap_write(map, reg, value);
            if selector_ret != 0 {
                (*map).cache_dirty = true;
                rust_regcache_err_write((*map).dev, reg, value, selector_ret);
                break;
            }
        }
        node = rb_next(node);
    }
    unlock(map);
    regmap_async_complete(map);
    rust_regcache_trace_sync(map, name, c"stop".as_ptr().cast());
    if sync_ret != 0 {
        sync_ret
    } else {
        selector_ret
    }
}

#[no_mangle]
pub unsafe extern "C" fn regcache_sync_region(map: *mut regmap, min: u32, max: u32) -> c_int {
    if rust_regcache_warn((*map).cache_type == regcache_type_REGCACHE_NONE) {
        return EINVAL_;
    }
    let ops = cache_ops(map);
    lock(map);
    if rust_regcache_warn((*map).cache_only) {
        unlock(map);
        return EINVAL_;
    }
    let bypass = (*map).cache_bypass;
    let name = (*ops).name;
    rust_regcache_dbg_region((*map).dev, name, min, max);
    rust_regcache_trace_sync(map, name, c"start region".as_ptr().cast());
    let mut ret = 0;
    if (*map).cache_dirty {
        (*map).async_ = true;
        ret = if let Some(sync) = (*ops).sync {
            sync(map, min, max)
        } else {
            regcache_default_sync(map, min, max)
        };
    }
    (*map).cache_bypass = bypass;
    (*map).async_ = false;
    (*map).no_sync_defaults = false;
    unlock(map);
    regmap_async_complete(map);
    rust_regcache_trace_sync(map, name, c"stop region".as_ptr().cast());
    ret
}

#[no_mangle]
pub unsafe extern "C" fn regcache_drop_region(map: *mut regmap, min: u32, max: u32) -> c_int {
    if (*map).cache_ops.is_null() {
        return EINVAL_;
    }
    let Some(drop) = (*(*map).cache_ops).drop else {
        return EINVAL_;
    };
    lock(map);
    rust_regcache_trace_drop(map, min, max);
    let ret = drop(map, min, max);
    unlock(map);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn regcache_cache_only(map: *mut regmap, enable: bool) {
    lock(map);
    rust_regcache_warn(
        (*map).cache_type != regcache_type_REGCACHE_NONE && (*map).cache_bypass && enable,
    );
    (*map).cache_only = enable;
    rust_regcache_trace_only(map, enable);
    unlock(map);
}

#[no_mangle]
pub unsafe extern "C" fn regcache_mark_dirty(map: *mut regmap) {
    lock(map);
    (*map).cache_dirty = true;
    (*map).no_sync_defaults = true;
    unlock(map);
}

#[no_mangle]
pub unsafe extern "C" fn regcache_cache_bypass(map: *mut regmap, enable: bool) {
    lock(map);
    rust_regcache_warn((*map).cache_only && enable);
    (*map).cache_bypass = enable;
    rust_regcache_trace_bypass(map, enable);
    unlock(map);
}

#[no_mangle]
pub unsafe extern "C" fn regcache_reg_cached(map: *mut regmap, reg: u32) -> bool {
    let mut value = 0;
    lock(map);
    let ret = regcache_read(map, reg, &mut value);
    unlock(map);
    ret == 0
}

unsafe fn regcache_get_val_addr(map: *mut regmap, base: *const c_void, idx: u32) -> *const c_void {
    base.cast::<u8>()
        .add((*map).cache_word_size.wrapping_mul(idx) as usize)
        .cast()
}

#[no_mangle]
pub unsafe extern "C" fn regcache_set_val(map: *mut regmap, base: *mut c_void, idx: u32, val: u32) {
    if let Some(format) = (*map).format.format_val {
        format(regcache_get_val_addr(map, base, idx).cast_mut(), val, 0);
        return;
    }
    match (*map).cache_word_size {
        1 => base.cast::<u8>().add(idx as usize).write(val as u8),
        2 => base.cast::<u16>().add(idx as usize).write(val as u16),
        4 => base.cast::<u32>().add(idx as usize).write(val),
        _ => kernel::bindings::BUG(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn regcache_get_val(map: *mut regmap, base: *const c_void, idx: u32) -> u32 {
    if base.is_null() {
        return EINVAL_ as u32;
    }
    if let Some(parse) = (*map).format.parse_val {
        return parse(regcache_get_val_addr(map, base, idx));
    }
    match (*map).cache_word_size {
        1 => base.cast::<u8>().add(idx as usize).read() as u32,
        2 => base.cast::<u16>().add(idx as usize).read() as u32,
        4 => base.cast::<u32>().add(idx as usize).read(),
        _ => kernel::bindings::BUG(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn regcache_lookup_reg(map: *mut regmap, reg: u32) -> c_int {
    let key = reg_default { reg, def: 0 };
    let result = bsearch(
        ptr::addr_of!(key).cast(),
        (*map).reg_defaults.cast(),
        (*map).num_reg_defaults as usize,
        mem::size_of::<reg_default>(),
        Some(regcache_defaults_cmp),
    )
    .cast::<reg_default>();
    if result.is_null() {
        ENOENT_
    } else {
        result.offset_from((*map).reg_defaults) as c_int
    }
}

unsafe fn regcache_reg_present(present: *mut c_ulong, idx: u32) -> bool {
    present.is_null() || rust_regcache_test_bit(idx, present)
}

#[no_mangle]
pub unsafe extern "C" fn regcache_sync_val(map: *mut regmap, reg: u32, val: u32) -> c_int {
    if !regcache_reg_needs_sync(map, reg, val) {
        return 0;
    }
    (*map).cache_bypass = true;
    let ret = _regmap_write(map, reg, val);
    (*map).cache_bypass = false;
    if ret != 0 {
        rust_regcache_err_sync((*map).dev, reg, ret);
        return ret;
    }
    rust_regcache_dbg_synced((*map).dev, reg, val);
    0
}

unsafe fn regcache_sync_block_single(
    map: *mut regmap,
    block: *mut c_void,
    present: *mut c_ulong,
    block_base: u32,
    start: u32,
    end: u32,
) -> c_int {
    for i in start..end {
        let reg = block_base.wrapping_add(i.wrapping_mul((*map).reg_stride as u32));
        if !regcache_reg_present(present, i) || !regmap_writeable(map, reg) {
            continue;
        }
        let val = regcache_get_val(map, block, i);
        let ret = regcache_sync_val(map, reg, val);
        if ret != 0 {
            return ret;
        }
    }
    0
}

unsafe fn regcache_sync_block_raw_flush(
    map: *mut regmap,
    data: *mut *const c_void,
    base: u32,
    cur: u32,
) -> c_int {
    let val_bytes = (*map).format.val_bytes;
    if (*data).is_null() {
        return 0;
    }
    let count = (cur.wrapping_sub(base) / nonzero_stride(map)) as c_int;
    let bytes = (count as usize).wrapping_mul(val_bytes);
    let end = cur.wrapping_sub((*map).reg_stride as u32);
    rust_regcache_dbg_raw((*map).dev, bytes, count, base, end);
    (*map).cache_bypass = true;
    let ret = _regmap_raw_write(map, base, *data, bytes, false);
    if ret != 0 {
        rust_regcache_err_raw((*map).dev, base, end, ret);
    }
    (*map).cache_bypass = false;
    *data = ptr::null();
    ret
}

unsafe fn regcache_sync_block_raw(
    map: *mut regmap,
    block: *mut c_void,
    present: *mut c_ulong,
    block_base: u32,
    start: u32,
    end: u32,
) -> c_int {
    let mut reg = 0;
    let mut base = 0;
    let mut data = ptr::null();
    for i in start..end {
        reg = block_base.wrapping_add(i.wrapping_mul((*map).reg_stride as u32));
        if !regcache_reg_present(present, i) || !regmap_writeable(map, reg) {
            let ret = regcache_sync_block_raw_flush(map, &mut data, base, reg);
            if ret != 0 {
                return ret;
            }
            continue;
        }
        let val = regcache_get_val(map, block, i);
        if !regcache_reg_needs_sync(map, reg, val) {
            let ret = regcache_sync_block_raw_flush(map, &mut data, base, reg);
            if ret != 0 {
                return ret;
            }
            continue;
        }
        if data.is_null() {
            data = regcache_get_val_addr(map, block, i);
            base = reg;
        }
    }
    regcache_sync_block_raw_flush(
        map,
        &mut data,
        base,
        reg.wrapping_add((*map).reg_stride as u32),
    )
}

#[no_mangle]
pub unsafe extern "C" fn regcache_sync_block(
    map: *mut regmap,
    block: *mut c_void,
    present: *mut c_ulong,
    block_base: u32,
    start: u32,
    end: u32,
) -> c_int {
    if regmap_can_raw_write(map) && !(*map).use_single_write {
        regcache_sync_block_raw(map, block, present, block_base, start, end)
    } else {
        regcache_sync_block_single(map, block, present, block_base, start, end)
    }
}

ffi_export::export_symbol!(regcache_sort_defaults, regcache_sort_defaults, "GPL", "");
ffi_export::export_symbol!(regcache_sync, regcache_sync, "GPL", "");
ffi_export::export_symbol!(regcache_sync_region, regcache_sync_region, "GPL", "");
ffi_export::export_symbol!(regcache_drop_region, regcache_drop_region, "GPL", "");
ffi_export::export_symbol!(regcache_cache_only, regcache_cache_only, "GPL", "");
ffi_export::export_symbol!(regcache_mark_dirty, regcache_mark_dirty, "GPL", "");
ffi_export::export_symbol!(regcache_cache_bypass, regcache_cache_bypass, "GPL", "");
ffi_export::export_symbol!(regcache_reg_cached, regcache_reg_cached, "GPL", "");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
