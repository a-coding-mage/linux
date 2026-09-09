// SPDX-License-Identifier: GPL-2.0
//
// Register cache access API - maple tree based cache
//
// Copyright 2023 Arm, Ltd
//
// Author: Mark Brown <broonie@kernel.org>

// Dependencies supplied by the surrounding kernel translation.

unsafe fn regcache_maple_read(
    map: *mut regmap,
    reg: libc::c_uint,
    value: *mut libc::c_uint,
) -> libc::c_int {
    let mt = (*map).cache as *mut maple_tree;
    let mut mas = MA_STATE!(mt, reg as libc::c_ulong, reg as libc::c_ulong);
    let entry: *mut libc::c_ulong;

    rcu_read_lock();

    entry = mas_walk(&mut mas);
    if entry.is_null() {
        rcu_read_unlock();
        return -ENOENT;
    }

    *value = *entry.add((reg as libc::c_ulong - mas.index) as usize) as libc::c_uint;

    rcu_read_unlock();

    0
}

unsafe fn regcache_maple_write(
    map: *mut regmap,
    reg: libc::c_uint,
    val: libc::c_uint,
) -> libc::c_int {
    let mt = (*map).cache as *mut maple_tree;
    let mut mas = MA_STATE!(mt, reg as libc::c_ulong, reg as libc::c_ulong);
    let mut entry: *mut libc::c_ulong;
    let mut upper: *mut libc::c_ulong;
    let mut lower: *mut libc::c_ulong;
    let mut index = reg as libc::c_ulong;
    let mut last = reg as libc::c_ulong;
    let mut lower_sz: usize = 0;
    let mut upper_sz: usize = 0;
    let mut ret: libc::c_int;

    rcu_read_lock();

    entry = mas_walk(&mut mas);
    if !entry.is_null() {
        *entry.add((reg as libc::c_ulong - mas.index) as usize) = val as libc::c_ulong;
        rcu_read_unlock();
        return 0;
    }

    /* Any adjacent entries to extend/merge? */
    mas_set_range(&mut mas, (reg - 1) as libc::c_ulong, (reg + 1) as libc::c_ulong);
    lower = mas_find(&mut mas, (reg - 1) as libc::c_ulong);
    if !lower.is_null() {
        index = mas.index;
        lower_sz = ((mas.last - mas.index + 1) * core::mem::size_of::<libc::c_ulong>() as libc::c_ulong) as usize;
    }

    upper = mas_find(&mut mas, (reg + 1) as libc::c_ulong);
    if !upper.is_null() {
        last = mas.last;
        upper_sz = ((mas.last - mas.index + 1) * core::mem::size_of::<libc::c_ulong>() as libc::c_ulong) as usize;
    }

    rcu_read_unlock();

    entry = kmalloc_array(last - index + 1, core::mem::size_of::<libc::c_ulong>(), (*map).alloc_flags);
    if entry.is_null() {
        return -ENOMEM;
    }

    if !lower.is_null() { memcpy(entry as *mut _, lower as *const _, lower_sz); }
    *entry.add((reg as libc::c_ulong - index) as usize) = val as libc::c_ulong;
    if !upper.is_null() { memcpy(entry.add((reg as libc::c_ulong - index + 1) as usize) as *mut _, upper as *const _, upper_sz); }

    /* The regmap lock makes the Maple lock redundant, but lockdep requires it. */
    mas_lock(&mut mas);
    mas_set_range(&mut mas, index, last);
    ret = mas_store_gfp(&mut mas, entry, (*map).alloc_flags);
    mas_unlock(&mut mas);

    if ret != 0 {
        kfree(entry);
        return ret;
    }
    kfree(lower);
    kfree(upper);
    0
}

unsafe fn regcache_maple_drop(map: *mut regmap, min: libc::c_uint, max: libc::c_uint) -> libc::c_int {
    let mt = (*map).cache as *mut maple_tree;
    let mut mas = MA_STATE!(mt, min as libc::c_ulong, max as libc::c_ulong);
    let mut entry: *mut libc::c_ulong;
    let mut lower: *mut libc::c_ulong = core::ptr::null_mut();
    let mut upper: *mut libc::c_ulong = core::ptr::null_mut();
    let mut lower_index = 0u64;
    let mut lower_last = 0u64;
    let mut upper_index = 0u64;
    let mut upper_last = 0u64;
    let mut ret = 0;

    mas_lock(&mut mas);
    mas_for_each!(&mut mas, entry, max as libc::c_ulong, {
        mas_unlock(&mut mas);
        if mas.index < min as libc::c_ulong {
            lower_index = mas.index;
            lower_last = (min - 1) as libc::c_ulong;
            lower = kmemdup_array(entry, min as libc::c_ulong - mas.index, core::mem::size_of::<libc::c_ulong>(), (*map).alloc_flags);
            if lower.is_null() { ret = -ENOMEM; goto_out_unlocked!(); }
        }
        if mas.last > max as libc::c_ulong {
            upper_index = (max + 1) as libc::c_ulong;
            upper_last = mas.last;
            upper = kmemdup_array(entry.add((max as libc::c_ulong - mas.index + 1) as usize), mas.last - max as libc::c_ulong, core::mem::size_of::<libc::c_ulong>(), (*map).alloc_flags);
            if upper.is_null() { ret = -ENOMEM; goto_out_unlocked!(); }
        }
        kfree(entry);
        mas_lock(&mut mas);
        mas_erase(&mut mas);
        if !lower.is_null() { mas_set_range(&mut mas, lower_index, lower_last); ret = mas_store_gfp(&mut mas, lower, (*map).alloc_flags); if ret != 0 { goto_out!(); } lower = core::ptr::null_mut(); }
        if !upper.is_null() { mas_set_range(&mut mas, upper_index, upper_last); ret = mas_store_gfp(&mut mas, upper, (*map).alloc_flags); if ret != 0 { goto_out!(); } upper = core::ptr::null_mut(); }
    });
goto_out!();
    mas_unlock(&mut mas);
goto_out_unlocked!();
    kfree(lower);
    kfree(upper);
    ret
}

unsafe fn regcache_maple_sync_block(map: *mut regmap, entry: *mut libc::c_ulong, mas: *mut ma_state, min: libc::c_uint, max: libc::c_uint) -> libc::c_int {
    let mut ret = 0;
    let val_bytes = (*map).format.val_bytes;
    mas_pause(mas);
    rcu_read_unlock();
    if max - min > 1 && regmap_can_raw_write(map) {
        let buf = kmalloc_array((max - min) as usize, val_bytes, (*map).alloc_flags);
        if buf.is_null() { ret = -ENOMEM; } else {
            for r in min..max { regcache_set_val(map, buf, (r - min) as usize, *entry.add((r as libc::c_ulong - (*mas).index) as usize)); }
            ret = _regmap_raw_write(map, min, buf, (max - min) as usize * val_bytes, false);
            kfree(buf);
        }
    } else {
        for r in min..max { ret = _regmap_write(map, r, *entry.add((r as libc::c_ulong - (*mas).index) as usize)); if ret != 0 { break; } }
    }
    rcu_read_lock();
    ret
}

unsafe fn regcache_maple_sync(map: *mut regmap, min: libc::c_uint, max: libc::c_uint) -> libc::c_int {
    let mt = (*map).cache as *mut maple_tree;
    let mut mas = MA_STATE!(mt, min as libc::c_ulong, max as libc::c_ulong);
    let mut entry: *mut libc::c_ulong;
    let mut sync_start = 0;
    let mut sync_needed = false;
    let mut ret = 0;
    (*map).cache_bypass = true;
    rcu_read_lock();
    mas_for_each!(&mut mas, entry, max as libc::c_ulong, {
        let start = core::cmp::max(mas.index, min as libc::c_ulong) as libc::c_uint;
        let end = core::cmp::min(mas.last, max as libc::c_ulong) as libc::c_uint;
        for r in start..=end {
            let v = *entry.add((r as libc::c_ulong - mas.index) as usize) as libc::c_uint;
            if regcache_reg_needs_sync(map, r, v) { if !sync_needed { sync_start = r; sync_needed = true; } continue; }
            if sync_needed { ret = regcache_maple_sync_block(map, entry, &mut mas, sync_start, r); if ret != 0 { break; } sync_needed = false; }
        }
        if ret == 0 && sync_needed { ret = regcache_maple_sync_block(map, entry, &mut mas, sync_start, end + 1); sync_needed = false; }
    });
    rcu_read_unlock();
    (*map).cache_bypass = false;
    ret
}

unsafe fn regcache_maple_init(map: *mut regmap) -> libc::c_int {
    let mt = kmalloc_obj::<maple_tree>((*map).alloc_flags);
    if mt.is_null() { return -ENOMEM; }
    (*map).cache = mt as *mut _;
    mt_init(mt);
    if !mt_external_lock(mt) && !(*map).lock_key.is_null() { lockdep_set_class_and_subclass(&mut (*mt).ma_lock, (*map).lock_key, 1); }
    0
}

unsafe fn regcache_maple_exit(map: *mut regmap) {
    let mt = (*map).cache as *mut maple_tree;
    if mt.is_null() { return; }
    let mut mas = MA_STATE!(mt, 0, libc::c_uint::MAX as libc::c_ulong);
    let mut entry: *mut libc::c_uint;
    mas_lock(&mut mas);
    mas_for_each!(&mut mas, entry, libc::c_uint::MAX as libc::c_ulong, { kfree(entry); });
    __mt_destroy(mt);
    mas_unlock(&mut mas);
    kfree(mt);
    (*map).cache = core::ptr::null_mut();
}

unsafe fn regcache_maple_insert_block(map: *mut regmap, first: libc::c_int, last: libc::c_int) -> libc::c_int {
    let mt = (*map).cache as *mut maple_tree;
    let mut mas = MA_STATE!(mt, first as libc::c_ulong, last as libc::c_ulong);
    let entry = kmalloc_array((last - first + 1) as usize, core::mem::size_of::<libc::c_ulong>(), (*map).alloc_flags);
    if entry.is_null() { return -ENOMEM; }
    for i in 0..(last - first + 1) { *entry.add(i as usize) = (*map).reg_defaults.offset((first + i) as isize).read().def as libc::c_ulong; }
    mas_lock(&mut mas);
    mas_set_range(&mut mas, (*map).reg_defaults.offset(first as isize).read().reg as libc::c_ulong, (*map).reg_defaults.offset(last as isize).read().reg as libc::c_ulong);
    let ret = mas_store_gfp(&mut mas, entry, (*map).alloc_flags);
    mas_unlock(&mut mas);
    if ret != 0 { kfree(entry); }
    ret
}

unsafe fn regcache_maple_populate(map: *mut regmap) -> libc::c_int {
    let mut range_start = 0;
    for i in 1..(*map).num_reg_defaults {
        if (*map).reg_defaults.add(i).read().reg != (*map).reg_defaults.add(i - 1).read().reg + 1 {
            let ret = regcache_maple_insert_block(map, range_start as libc::c_int, (i - 1) as libc::c_int);
            if ret != 0 { return ret; }
            range_start = i;
        }
    }
    regcache_maple_insert_block(map, range_start as libc::c_int, ((*map).num_reg_defaults - 1) as libc::c_int)
}

#[no_mangle]
pub static mut regcache_maple_ops: regcache_ops = regcache_ops {
    type_: REGCACHE_MAPLE,
    name: c"maple".as_ptr(),
    init: Some(regcache_maple_init),
    exit: Some(regcache_maple_exit),
    populate: Some(regcache_maple_populate),
    read: Some(regcache_maple_read),
    write: Some(regcache_maple_write),
    drop: Some(regcache_maple_drop),
    sync: Some(regcache_maple_sync),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
