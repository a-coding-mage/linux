// SPDX-License-Identifier: GPL-2.0
/*
 *
 * Copyright (C) 2019-2021 Paragon Software GmbH, All rights reserved.
 *
 */

/* Translated from the C implementation; declarations supplied by other files remain external dependencies. */

#[inline]
unsafe fn al_is_valid_le(ni: *const ntfs_inode, le: *mut ATTR_LIST_ENTRY) -> bool {
    let ni = (*ni).base;
    if le.is_null() || (*ni).attr_list.le.is_null() || (*ni).attr_list.size == 0 {
        return false;
    }

    ptr_offset((*ni).attr_list.le, le) + le16_to_cpu((*le).size) as usize
        <= (*ni).attr_list.size
}

pub unsafe fn al_destroy(ni: *mut ntfs_inode) {
    let ni = (*ni).base;
    run_close(&mut (*ni).attr_list.run);
    kvfree((*ni).attr_list.le);
    (*ni).attr_list.le = core::ptr::null_mut();
    (*ni).attr_list.size = 0;
    (*ni).attr_list.dirty = false;
}

pub unsafe fn ntfs_load_attr_list(ni: *mut ntfs_inode, attr: *mut ATTRIB) -> i32 {
    let ni = (*ni).base;
    if (*ni).attr_list.size != 0 {
        return 0;
    }

    let mut le: *mut core::ffi::c_void = core::ptr::null_mut();
    let lsize: usize;
    let mut err: i32;

    if !(*attr).non_res {
        lsize = le32_to_cpu((*attr).res.data_size) as usize;
        if lsize == 0 { err = -EINVAL; (*ni).attr_list.le = le; al_destroy(ni); return err; }
        le = kvmalloc(al_aligned(lsize), GFP_KERNEL);
        if le.is_null() { err = -ENOMEM; (*ni).attr_list.le = le; al_destroy(ni); return err; }
        memcpy(le, resident_data(attr), lsize);
    } else if (*attr).nres.svcn != 0 {
        err = -EINVAL;
        (*ni).attr_list.le = le; al_destroy(ni); return err;
    } else {
        let run_off = le16_to_cpu((*attr).nres.run_off) as usize;
        lsize = le64_to_cpu((*attr).nres.data_size) as usize;
        if lsize == 0 { err = -EINVAL; (*ni).attr_list.le = le; al_destroy(ni); return err; }
        run_init(&mut (*ni).attr_list.run);
        if run_off > le32_to_cpu((*attr).size) as usize { err = -EINVAL; (*ni).attr_list.le = le; al_destroy(ni); return err; }
        err = run_unpack_ex(&mut (*ni).attr_list.run, (*ni).mi.sbi, (*ni).mi.rno,
            0, le64_to_cpu((*attr).nres.evcn), 0,
            add2_ptr(attr, run_off), le32_to_cpu((*attr).size) as usize - run_off);
        if err < 0 { (*ni).attr_list.le = le; al_destroy(ni); return err; }
        le = kvmalloc(al_aligned(lsize), GFP_KERNEL);
        if le.is_null() { err = -ENOMEM; (*ni).attr_list.le = le; al_destroy(ni); return err; }
        err = ntfs_read_run_nb((*ni).mi.sbi, &mut (*ni).attr_list.run, 0, le, lsize, core::ptr::null_mut());
        if err != 0 { (*ni).attr_list.le = le; al_destroy(ni); return err; }
    }

    (*ni).attr_list.size = lsize;
    (*ni).attr_list.le = le;
    return 0;

}

pub unsafe fn al_enumerate(ni: *mut ntfs_inode, mut le: *mut ATTR_LIST_ENTRY) -> *mut ATTR_LIST_ENTRY {
    let le_min_size = le_size(0) as usize;
    if le.is_null() { le = (*ni).attr_list.le; }
    else {
        let sz = le16_to_cpu((*le).size) as usize;
        if sz < le_min_size { return core::ptr::null_mut(); }
        le = add2_ptr(le, sz);
    }
    let off = ptr_offset((*ni).attr_list.le, le);
    if off + le_min_size > (*ni).attr_list.size { return core::ptr::null_mut(); }
    let sz = le16_to_cpu((*le).size) as usize;
    if sz < le_min_size || off + sz > (*ni).attr_list.size ||
       sz < (*le).name_off as usize + (*le).name_len as usize * core::mem::size_of::<i16>() {
        return core::ptr::null_mut();
    }
    le
}

pub unsafe fn al_find_le(ni: *mut ntfs_inode, le: *mut ATTR_LIST_ENTRY, attr: *const ATTRIB) -> *mut ATTR_LIST_ENTRY {
    let svcn = attr_svcn(attr);
    al_find_ex(ni, le, (*attr).type_, attr_name(attr), (*attr).name_len, &svcn)
}

pub unsafe fn al_find_ex(mut ni: *mut ntfs_inode, mut le: *mut ATTR_LIST_ENTRY, type_: ATTR_TYPE,
                         name: *const __le16, name_len: u8, vcn: *const CLST) -> *mut ATTR_LIST_ENTRY {
    let mut ret = core::ptr::null_mut();
    ni = (*ni).base;
    let type_in = le32_to_cpu(type_);
    while { le = al_enumerate(ni, le); !le.is_null() } {
        let mut diff = le32_to_cpu((*le).type_) as i64 - type_in as i64;
        if diff < 0 { continue; }
        if diff > 0 { return ret; }
        if (*le).name_len != name_len { continue; }
        let le_vcn = le64_to_cpu((*le).vcn);
        if le_vcn == 0 {
            diff = ntfs_cmp_names(le_name(le), name_len, name, name_len, (*(*ni).mi.sbi).upcase, true) as i64;
            if diff < 0 { continue; }
            if diff > 0 { return ret; }
        }
        if vcn.is_null() || *vcn == le_vcn { return le; }
        if *vcn < le_vcn { return ret; }
        ret = le;
    }
    ret
}

// The remaining declarations preserve the source-level interfaces and operations.
pub unsafe fn al_remove_le(ni: *mut ntfs_inode, le: *mut ATTR_LIST_ENTRY) -> bool {
    let ni = (*ni).base;
    if !al_is_valid_le(ni, le) { return false; }
    let size = le16_to_cpu((*le).size) as usize;
    let off = ptr_offset((*ni).attr_list.le, le);
    memmove(le, add2_ptr(le, size), (*ni).attr_list.size - off - size);
    (*ni).attr_list.size -= size;
    (*ni).attr_list.dirty = true;
    true
}

unsafe fn al_find_le_to_insert(ni: *mut ntfs_inode, type_: ATTR_TYPE, name: *const __le16,
                               name_len: u8, vcn: CLST) -> *mut ATTR_LIST_ENTRY {
    let ni = (*ni).base;
    let mut le = core::ptr::null_mut();
    let mut prev;
    let type_in = le32_to_cpu(type_);
    loop {
        prev = le;
        le = al_enumerate(ni, prev);
        if le.is_null() { break; }
        let mut diff = le32_to_cpu((*le).type_) as i64 - type_in as i64;
        if diff < 0 { continue; }
        if diff > 0 { return le; }
        if le64_to_cpu((*le).vcn) == 0 {
            diff = ntfs_cmp_names(le_name(le), (*le).name_len, name, name_len,
                                  (*(*ni).mi.sbi).upcase, true) as i64;
            if diff < 0 { continue; }
            if diff > 0 { return le; }
        }
        if le64_to_cpu((*le).vcn) >= vcn { return le; }
    }
    if !prev.is_null() { add2_ptr(prev, le16_to_cpu((*prev).size) as usize) } else { (*ni).attr_list.le }
}

pub unsafe fn al_add_le(ni: *mut ntfs_inode, type_: ATTR_TYPE, name: *const __le16, name_len: u8,
                        svcn: CLST, id: __le16, r#ref: *const MFT_REF,
                        new_le: *mut *mut ATTR_LIST_ENTRY) -> i32 {
    let ni = (*ni).base;
    let old_size = (*ni).attr_list.size;
    let sz = le_size(name_len) as usize;
    let new_size = old_size + sz;
    let off_le = al_find_le_to_insert(ni, type_, name, name_len, svcn);
    let off = ptr_offset((*ni).attr_list.le, off_le);
    if new_size > al_aligned(old_size) {
        let ptr = kmalloc(al_aligned(new_size), GFP_NOFS);
        if ptr.is_null() { return -ENOMEM; }
        memcpy(ptr, (*ni).attr_list.le, off);
        memcpy(add2_ptr(ptr, off + sz), off_le, old_size - off);
        *new_le = add2_ptr(ptr, off);
        kvfree((*ni).attr_list.le);
        (*ni).attr_list.le = ptr;
    } else {
        memmove(add2_ptr(off_le, sz), off_le, old_size - off);
        *new_le = off_le;
    }
    (*ni).attr_list.size = new_size;
    let le = *new_le;
    (*le).type_ = type_; (*le).size = cpu_to_le16(sz as u16); (*le).name_len = name_len;
    (*le).name_off = core::mem::offset_of!(ATTR_LIST_ENTRY, name) as u8;
    (*le).vcn = cpu_to_le64(svcn); (*le).r#ref = *r#ref; (*le).id = id;
    memcpy((*le).name.as_mut_ptr() as *mut _, name, core::mem::size_of::<i16>() * name_len as usize);
    (*ni).attr_list.dirty = true;
    0
}

pub unsafe fn al_update(ni: *mut ntfs_inode, sync: i32) -> i32 {
    let ni = (*ni).base;
    if !(*ni).attr_list.dirty || (*ni).attr_list.size == 0 { return 0; }
    let mut attr: *mut ATTRIB = core::ptr::null_mut();
    let err = attr_set_size_ex(ni, ATTR_LIST, core::ptr::null(), 0, &mut (*ni).attr_list.run,
        (*ni).attr_list.size, core::ptr::null_mut(), false, &mut attr, false);
    if err != 0 { return err; }
    if !(*attr).non_res {
        memcpy(resident_data(attr), (*ni).attr_list.le, (*ni).attr_list.size);
    } else {
        let err = ntfs_sb_write_run((*ni).mi.sbi, &mut (*ni).attr_list.run, 0,
            (*ni).attr_list.le, (*ni).attr_list.size, sync);
        if err != 0 { return err; }
        (*attr).nres.valid_size = (*attr).nres.data_size;
    }
    (*ni).mi.dirty = true;
    (*ni).attr_list.dirty = false;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
