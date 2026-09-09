// SPDX-License-Identifier: GPL-2.0
//
// Register cache access API - rbtree caching support
//
// Copyright 2011 Wolfson Microelectronics plc
//
// Author: Dimitris Papastamos <dp@opensource.wolfsonmicro.com>

// Kernel headers and "internal.h" are supplied by the surrounding translation unit.

static mut REGCACHE_RBTREE_OPS: regcache_ops = regcache_ops {
    type_: REGCACHE_RBTREE,
    name: b"rbtree\0".as_ptr() as *const i8,
    init: Some(regcache_rbtree_init),
    exit: Some(regcache_rbtree_exit),
    populate: Some(regcache_rbtree_populate),
    #[cfg(CONFIG_DEBUG_FS)]
    debugfs_init: Some(rbtree_debugfs_init),
    read: Some(regcache_rbtree_read),
    write: Some(regcache_rbtree_write),
    sync: Some(regcache_rbtree_sync),
    drop: Some(regcache_rbtree_drop),
};

#[repr(C)]
struct regcache_rbtree_node {
    /* block of adjacent registers */
    block: *mut core::ffi::c_void,
    /* Which registers are present */
    cache_present: *mut c_ulong,
    /* base register handled by this block */
    base_reg: c_uint,
    /* number of registers available in the block */
    blklen: c_uint,
    /* the actual rbtree node holding this block */
    node: rb_node,
}

#[repr(C)]
struct regcache_rbtree_ctx {
    root: rb_root,
    cached_rbnode: *mut regcache_rbtree_node,
}

unsafe fn regcache_rbtree_get_base_top_reg(map: *mut regmap, rbnode: *mut regcache_rbtree_node,
                                            base: *mut c_uint, top: *mut c_uint) {
    *base = (*rbnode).base_reg;
    *top = (*rbnode).base_reg + ((*rbnode).blklen - 1) * (*map).reg_stride;
}

unsafe fn regcache_rbtree_get_register(map: *mut regmap, rbnode: *mut regcache_rbtree_node,
                                        idx: c_uint) -> c_uint {
    regcache_get_val(map, (*rbnode).block, idx)
}

unsafe fn regcache_rbtree_set_register(map: *mut regmap, rbnode: *mut regcache_rbtree_node,
                                       idx: c_uint, val: c_uint) {
    set_bit(idx, (*rbnode).cache_present);
    regcache_set_val(map, (*rbnode).block, idx, val);
}

unsafe fn regcache_rbtree_lookup(map: *mut regmap, reg: c_uint) -> *mut regcache_rbtree_node {
    let ctx = (*map).cache as *mut regcache_rbtree_ctx;
    let mut node: *mut rb_node;
    let mut rbnode: *mut regcache_rbtree_node;
    let (mut base_reg, mut top_reg) = (0, 0);
    rbnode = (*ctx).cached_rbnode;
    if !rbnode.is_null() {
        regcache_rbtree_get_base_top_reg(map, rbnode, &mut base_reg, &mut top_reg);
        if reg >= base_reg && reg <= top_reg { return rbnode; }
    }
    node = (*ctx).root.rb_node;
    while !node.is_null() {
        rbnode = rb_entry(node);
        regcache_rbtree_get_base_top_reg(map, rbnode, &mut base_reg, &mut top_reg);
        if reg >= base_reg && reg <= top_reg {
            (*ctx).cached_rbnode = rbnode; return rbnode;
        } else if reg > top_reg { node = (*node).rb_right; }
        else if reg < base_reg { node = (*node).rb_left; }
    }
    core::ptr::null_mut()
}

unsafe fn regcache_rbtree_insert(map: *mut regmap, root: *mut rb_root,
                                 rbnode: *mut regcache_rbtree_node) -> c_int {
    let mut parent = core::ptr::null_mut();
    let mut new = &mut (*root).rb_node as *mut *mut rb_node;
    let (mut base_tmp, mut top_tmp) = (0, 0);
    while !(*new).is_null() {
        let tmp = rb_entry(*new);
        regcache_rbtree_get_base_top_reg(map, tmp, &mut base_tmp, &mut top_tmp);
        let base = (*rbnode).base_reg;
        parent = *new;
        if base >= base_tmp && base <= top_tmp { return 0; }
        else if base > top_tmp { new = &mut (**new).rb_right; }
        else if base < base_tmp { new = &mut (**new).rb_left; }
    }
    rb_link_node(&mut (*rbnode).node, parent, new);
    rb_insert_color(&mut (*rbnode).node, root);
    1
}

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn rbtree_show(s: *mut seq_file, _ignored: *mut core::ffi::c_void) -> c_int {
    let map = (*s).private as *mut regmap;
    let ctx = (*map).cache as *mut regcache_rbtree_ctx;
    let mut mem_size = core::mem::size_of::<regcache_rbtree_ctx>();
    let mut nodes = 0; let mut registers = 0;
    let mut node = rb_first(&mut (*ctx).root);
    while !node.is_null() {
        let n = rb_entry(node);
        mem_size += core::mem::size_of::<regcache_rbtree_node>() +
            (*n).blklen as usize * (*map).cache_word_size as usize +
            bits_to_longs((*n).blklen) as usize * core::mem::size_of::<c_long>();
        let (mut base, mut top) = (0, 0);
        regcache_rbtree_get_base_top_reg(map, n, &mut base, &mut top);
        let this_registers = (top - base) / (*map).reg_stride + 1;
        seq_printf(s, b"%x-%x (%d)\n\0".as_ptr() as *const i8, base, top, this_registers);
        nodes += 1; registers += this_registers as c_int;
        node = rb_next(node);
    }
    let average = if nodes != 0 { registers / nodes } else { 0 };
    seq_printf(s, b"%d nodes, %d registers, average %d registers, used %zu bytes\n\0".as_ptr() as *const i8,
               nodes, registers, average, mem_size);
    0
}

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn rbtree_debugfs_init(map: *mut regmap) {
    debugfs_create_file(b"rbtree\0".as_ptr() as *const i8, 0o400, (*map).debugfs,
                        map as *mut _, &rbtree_fops);
}

unsafe fn regcache_rbtree_init(map: *mut regmap) -> c_int {
    let ctx = kmalloc_obj::<regcache_rbtree_ctx>((*map).alloc_flags);
    if ctx.is_null() { return -ENOMEM; }
    (*map).cache = ctx as *mut _;
    (*ctx).root = RB_ROOT;
    (*ctx).cached_rbnode = core::ptr::null_mut();
    0
}

unsafe fn regcache_rbtree_exit(map: *mut regmap) {
    let ctx = (*map).cache as *mut regcache_rbtree_ctx;
    if ctx.is_null() { return; }
    let mut next = rb_first(&mut (*ctx).root);
    while !next.is_null() {
        let n = rb_entry(next);
        next = rb_next(&mut (*n).node);
        rb_erase(&mut (*n).node, &mut (*ctx).root);
        kfree((*n).cache_present as *mut _); kfree((*n).block);
        kfree(n as *mut _);
    }
    kfree((*map).cache); (*map).cache = core::ptr::null_mut();
}

unsafe fn regcache_rbtree_populate(map: *mut regmap) -> c_int {
    for i in 0..(*map).num_reg_defaults {
        let d = (*map).reg_defaults.add(i as usize);
        let ret = regcache_rbtree_write(map, (*d).reg, (*d).def);
        if ret != 0 { return ret; }
    }
    0
}

unsafe fn regcache_rbtree_read(map: *mut regmap, reg: c_uint, value: *mut c_uint) -> c_int {
    let n = regcache_rbtree_lookup(map, reg);
    if n.is_null() { return -ENOENT; }
    let idx = (reg - (*n).base_reg) / (*map).reg_stride;
    if test_bit(idx, (*n).cache_present) == 0 { return -ENOENT; }
    *value = regcache_rbtree_get_register(map, n, idx); 0
}

unsafe fn regcache_rbtree_insert_to_block(map: *mut regmap, n: *mut regcache_rbtree_node,
    base: c_uint, top: c_uint, reg: c_uint, value: c_uint) -> c_int {
    let len = (top - base) / (*map).reg_stride + 1;
    let pos = (reg - base) / (*map).reg_stride;
    let offset = ((*n).base_reg - base) / (*map).reg_stride;
    let blk = krealloc_array((*n).block, len, (*map).cache_word_size, (*map).alloc_flags);
    if blk.is_null() { return -ENOMEM; } (*n).block = blk;
    let present;
    if bits_to_longs(len) > bits_to_longs((*n).blklen) {
        present = krealloc_array((*n).cache_present, bits_to_longs(len), core::mem::size_of::<c_ulong>(), (*map).alloc_flags) as *mut c_ulong;
        if present.is_null() { return -ENOMEM; }
        core::ptr::write_bytes(present.add(bits_to_longs((*n).blklen) as usize), 0,
            (bits_to_longs(len) - bits_to_longs((*n).blklen)) as usize);
    } else { present = (*n).cache_present; }
    if pos == 0 {
        core::ptr::copy(blk, (blk as *mut u8).add(offset as usize * (*map).cache_word_size as usize),
                        (*n).blklen as usize * (*map).cache_word_size as usize);
        bitmap_shift_left(present, present, offset, len);
    }
    (*n).blklen = len; (*n).base_reg = base; (*n).cache_present = present;
    regcache_rbtree_set_register(map, n, pos, value); 0
}

unsafe fn regcache_rbtree_node_alloc(map: *mut regmap, reg: c_uint) -> *mut regcache_rbtree_node {
    let n = kzalloc_obj::<regcache_rbtree_node>((*map).alloc_flags);
    if n.is_null() { return core::ptr::null_mut(); }
    /* If there is a read table then use it to guess at an allocation */
    if !(*map).rd_table.is_null() {
        let table = (*map).rd_table;
        for i in 0..(*table).n_yes_ranges {
            let range = (*table).yes_ranges.add(i as usize);
            if regmap_reg_in_range(reg, range) {
                (*n).blklen = ((*range).range_max - (*range).range_min) / (*map).reg_stride + 1;
                (*n).base_reg = (*range).range_min;
                break;
            }
        }
    }
    if (*n).blklen == 0 { (*n).blklen = 1; (*n).base_reg = reg; }
    (*n).block = kmalloc_array((*n).blklen, (*map).cache_word_size, (*map).alloc_flags);
    if (*n).block.is_null() { kfree(n as *mut _); return core::ptr::null_mut(); }
    (*n).cache_present = kcalloc(bits_to_longs((*n).blklen), core::mem::size_of::<c_ulong>(), (*map).alloc_flags) as *mut c_ulong;
    if (*n).cache_present.is_null() { kfree((*n).block); kfree(n as *mut _); return core::ptr::null_mut(); }
    n
}

unsafe fn regcache_rbtree_write(map: *mut regmap, reg: c_uint, value: c_uint) -> c_int {
    let ctx = (*map).cache as *mut regcache_rbtree_ctx;
    let n = regcache_rbtree_lookup(map, reg);
    if !n.is_null() {
        regcache_rbtree_set_register(map, n, (reg - (*n).base_reg) / (*map).reg_stride, value);
        return 0;
    }
    let max_dist = (*map).reg_stride * core::mem::size_of::<regcache_rbtree_node>() as c_uint /
        (*map).cache_word_size;
    let min = if reg < max_dist { 0 } else { reg - max_dist };
    let max = reg + max_dist;
    let mut node = rb_first(&mut (*ctx).root);
    let mut best: *mut regcache_rbtree_node = core::ptr::null_mut();
    let (mut new_base, mut new_top) = (0, 0);
    let mut best_dist = c_uint::MAX;
    while !node.is_null() {
        let candidate = rb_entry(node); let (mut base, mut top) = (0, 0);
        regcache_rbtree_get_base_top_reg(map, candidate, &mut base, &mut top);
        if base <= max && top >= min {
            let dist = if reg < base { base - reg } else if reg > top { reg - top } else { 0 };
            if dist < best_dist {
                best = candidate; best_dist = dist;
                new_base = if reg < base { reg } else { base };
                new_top = if reg > top { reg } else { top };
            }
        }
        if reg < base { node = (*node).rb_left; }
        else if reg > top { node = (*node).rb_right; }
        else { break; }
    }
    if !best.is_null() {
        let ret = regcache_rbtree_insert_to_block(map, best, new_base, new_top, reg, value);
        if ret != 0 { return ret; }
        (*ctx).cached_rbnode = best; return 0;
    }
    let n = regcache_rbtree_node_alloc(map, reg);
    if n.is_null() { return -ENOMEM; }
    regcache_rbtree_set_register(map, n, 0, value);
    regcache_rbtree_insert(map, &mut (*ctx).root, n);
    (*ctx).cached_rbnode = n; 0
}

unsafe fn regcache_rbtree_sync(map: *mut regmap, min: c_uint, max: c_uint) -> c_int {
    (*map).async_ = true;
    let ctx = (*map).cache as *mut regcache_rbtree_ctx;
    let mut node = rb_first(&mut (*ctx).root);
    while !node.is_null() {
        let n = rb_entry(node); let (mut base, mut top) = (0, 0);
        regcache_rbtree_get_base_top_reg(map, n, &mut base, &mut top);
        if base > max { break; } if top >= min {
            let start = if min > base { (min - base) / (*map).reg_stride } else { 0 };
            let end = if max < top { (max - base) / (*map).reg_stride + 1 } else { (*n).blklen };
            let ret = regcache_sync_block(map, (*n).block, (*n).cache_present, (*n).base_reg, start, end);
            if ret != 0 { return ret; }
        }
        node = rb_next(node);
    }
    (*map).async_ = false; regmap_async_complete(map)
}

unsafe fn regcache_rbtree_drop(map: *mut regmap, min: c_uint, max: c_uint) -> c_int {
    let ctx = (*map).cache as *mut regcache_rbtree_ctx;
    let mut node = rb_first(&mut (*ctx).root);
    while !node.is_null() {
        let n = rb_entry(node); let (mut base, mut top) = (0, 0);
        regcache_rbtree_get_base_top_reg(map, n, &mut base, &mut top);
        if base > max { break; }
        if top >= min {
            let start = if min > base { (min - base) / (*map).reg_stride } else { 0 };
            let end = if max < top { (max - base) / (*map).reg_stride + 1 } else { (*n).blklen };
            bitmap_clear((*n).cache_present, start, end - start);
        }
        node = rb_next(node);
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
