// SPDX-License-Identifier: GPL-2.0
/*
 *
 * Copyright (C) 2019-2021 Paragon Software GmbH, All rights reserved.
 *
 */

// Dependencies supplied by the surrounding NTFS/Rust port are intentionally
// left external, corresponding to the C includes.

#[inline]
unsafe fn compare_attr(left: *const ATTRIB, ty: ATTR_TYPE, name: *const __le16,
                       name_len: u8, upcase: *const u16) -> i32 {
    /* First, compare the type codes. */
    let diff = le32_to_cpu((*left).type_) as i32 - le32_to_cpu(ty) as i32;
    if diff != 0 { return diff; }

    /* They have the same type code, so we have to compare the names. */
    ntfs_cmp_names(attr_name(left), (*left).name_len, name, name_len, upcase, true)
}

/*
 * mi_new_attt_id
 *
 * Return: Unused attribute id that is less than mrec->next_attr_id.
 */
unsafe fn mi_new_attt_id(ni: *mut ntfs_inode, mi: *mut mft_inode) -> __le16 {
    let mut free_id: u16;
    let mut max_id: u16;
    let mut t16: u16;
    let rec = (*mi).mrec;
    let mut attr: *mut ATTRIB;
    let id = (*rec).next_attr_id;

    free_id = le16_to_cpu(id);
    if free_id < 0x7fff {
        (*rec).next_attr_id = cpu_to_le16(free_id + 1);
        return id;
    }

    /* One record can store up to 1024/24 ~= 42 attributes. */
    free_id = 0;
    max_id = 0;
    attr = core::ptr::null_mut();

    loop {
        attr = mi_enum_attr(ni, mi, attr);
        if attr.is_null() {
            (*rec).next_attr_id = cpu_to_le16(max_id + 1);
            (*mi).dirty = true;
            return cpu_to_le16(free_id);
        }
        t16 = le16_to_cpu((*attr).id);
        if t16 == free_id {
            free_id += 1;
            attr = core::ptr::null_mut();
        } else if max_id < t16 {
            max_id = t16;
        }
    }
}

pub unsafe fn mi_get(sbi: *mut ntfs_sb_info, rno: CLST, mi: *mut *mut mft_inode) -> i32 {
    let mut err: i32;
    let m = kzalloc_obj::<mft_inode>(GFP_NOFS);
    if m.is_null() { return -ENOMEM; }
    err = mi_init(m, sbi, rno);
    if err != 0 { kfree(m); return err; }
    err = mi_read(m, false);
    if err != 0 { mi_put(m); return err; }
    *mi = m;
    0
}

pub unsafe fn mi_put(mi: *mut mft_inode) { mi_clear(mi); kfree(mi); }

pub unsafe fn mi_init(mi: *mut mft_inode, sbi: *mut ntfs_sb_info, rno: CLST) -> i32 {
    (*mi).sbi = sbi;
    (*mi).rno = rno;
    (*mi).mrec = kmalloc((*sbi).record_size, GFP_NOFS);
    if (*mi).mrec.is_null() { return -ENOMEM; }
    0
}

/* mi_read - Read MFT data. */
pub unsafe fn mi_read(mi: *mut mft_inode, is_mft: bool) -> i32 {
    let mut err: i32;
    let rec = (*mi).mrec;
    let sbi = (*mi).sbi;
    let bpr = (*sbi).record_size;
    let vbo = ((*mi).rno as u64) << (*sbi).record_bits;
    let mft_ni = (*sbi).mft.ni;
    let run = if !mft_ni.is_null() { &mut (*mft_ni).file.run as *mut runs_tree } else { core::ptr::null_mut() };
    let mut rw_lock: *mut rw_semaphore = core::ptr::null_mut();

    if is_mounted(sbi) && !is_mft && !mft_ni.is_null() {
        rw_lock = &mut (*mft_ni).file.run_lock;
        down_read(rw_lock);
    }
    err = ntfs_read_bh(sbi, run, vbo, &mut (*rec).rhdr, bpr, &mut (*mi).nb);
    if !rw_lock.is_null() { up_read(rw_lock); }
    if err == 0 { return mi_read_check(mi, rec, bpr); }
    if err == -E_NTFS_FIXUP { (*mi).dirty = true; return mi_read_check(mi, rec, bpr); }
    if err != -ENOENT { return mi_read_error(sbi, err); }

    if !rw_lock.is_null() { ni_lock(mft_ni); down_write(rw_lock); }
    err = attr_load_runs_vcn(mft_ni, ATTR_DATA, core::ptr::null(), 0, run, vbo >> (*sbi).cluster_bits);
    if !rw_lock.is_null() { up_write(rw_lock); ni_unlock(mft_ni); }
    if err != 0 { return mi_read_error(sbi, err); }
    if !rw_lock.is_null() { down_read(rw_lock); }
    err = ntfs_read_bh(sbi, run, vbo, &mut (*rec).rhdr, bpr, &mut (*mi).nb);
    if !rw_lock.is_null() { up_read(rw_lock); }
    if err == -E_NTFS_FIXUP { (*mi).dirty = true; return mi_read_check(mi, rec, bpr); }
    if err != 0 { return mi_read_error(sbi, err); }
    mi_read_check(mi, rec, bpr)
}

unsafe fn mi_read_check(_mi: *mut mft_inode, rec: *mut MFT_REC, bpr: u32) -> i32 {
    if le32_to_cpu((*rec).total) != bpr { -EINVAL } else { 0 }
}
unsafe fn mi_read_error(sbi: *mut ntfs_sb_info, mut err: i32) -> i32 {
    if err == -E_NTFS_CORRUPT { ntfs_err((*sbi).sb, "mft corrupted"); ntfs_set_state(sbi, NTFS_DIRTY_ERROR); err = -EINVAL; }
    err
}

/* mi_enum_attr - start/continue attributes enumeration in record. */
pub unsafe fn mi_enum_attr(ni: *mut ntfs_inode, mi: *mut mft_inode,
                           mut attr: *mut ATTRIB) -> *mut ATTRIB {
    let rec = (*mi).mrec as *const MFT_REC;
    let used = le32_to_cpu((*rec).used);
    let total = le32_to_cpu((*rec).total);
    let mut off: u32;
    let mut asize: u32;
    let mut prev_type: u32;
    let mut t16: u16;
    let (mut svcn, mut evcn, mut data_size, mut alloc_size, mut tot_size): (u64,u64,u64,u64,u64);

    if attr.is_null() {
        off = le16_to_cpu((*rec).attr_off) as u32;
        if used > total { _ntfs_bad_inode(&mut (*ni).vfs_inode); return core::ptr::null_mut(); }
        if off >= used || off < MFTRECORD_FIXUP_OFFSET_1 || !IS_ALIGNED(off, 8) { _ntfs_bad_inode(&mut (*ni).vfs_inode); return core::ptr::null_mut(); }
        if !is_rec_inuse(rec) { return core::ptr::null_mut(); }
        prev_type = 0;
        attr = Add2Ptr(rec as *mut MFT_REC, off);
    } else {
        off = PtrOffset(rec, attr);
        asize = le32_to_cpu((*attr).size);
        prev_type = le32_to_cpu((*attr).type_);
        attr = Add2Ptr(attr, asize);
        off += asize;
    }
    if off + 8 > used { _ntfs_bad_inode(&mut (*ni).vfs_inode); return core::ptr::null_mut(); }
    if (*attr).type_ == ATTR_END { return core::ptr::null_mut(); }
    let mut t32 = le32_to_cpu((*attr).type_);
    if t32 == 0 || (t32 & 0xf) != 0 || t32 > 0x100 || t32 < prev_type { _ntfs_bad_inode(&mut (*ni).vfs_inode); return core::ptr::null_mut(); }
    asize = le32_to_cpu((*attr).size);
    if !IS_ALIGNED(asize, 8) || off + asize < off || off + asize > used || off + 9 > used { _ntfs_bad_inode(&mut (*ni).vfs_inode); return core::ptr::null_mut(); }
    if (*attr).non_res == 0 {
        if asize < SIZEOF_RESIDENT { _ntfs_bad_inode(&mut (*ni).vfs_inode); return core::ptr::null_mut(); }
        t16 = le16_to_cpu((*attr).res.data_off);
        if t16 as u32 > asize || le32_to_cpu((*attr).res.data_size) > asize - t16 as u32 || ((*attr).name_len as u32 != 0 && le16_to_cpu((*attr).name_off) as u32 + core::mem::size_of::<i16>() as u32 * (*attr).name_len as u32 > t16 as u32) { _ntfs_bad_inode(&mut (*ni).vfs_inode); return core::ptr::null_mut(); }
        return attr;
    }
    if (*attr).non_res != 1 || asize < SIZEOF_NONRESIDENT { _ntfs_bad_inode(&mut (*ni).vfs_inode); return core::ptr::null_mut(); }
    t16 = le16_to_cpu((*attr).nres.run_off);
    t32 = core::mem::size_of::<i16>() as u32 * (*attr).name_len as u32;
    if t16 as u32 > asize || (t32 != 0 && le16_to_cpu((*attr).name_off) as u32 + t32 > t16 as u32) { _ntfs_bad_inode(&mut (*ni).vfs_inode); return core::ptr::null_mut(); }
    svcn = le64_to_cpu((*attr).nres.svcn); evcn = le64_to_cpu((*attr).nres.evcn);
    if svcn > evcn.wrapping_add(1) { _ntfs_bad_inode(&mut (*ni).vfs_inode); return core::ptr::null_mut(); }
    if is_attr_ext(attr) {
        // CONFIG_NTFS3_64BIT_CLUSTER: no limits; otherwise the 32-bit check is retained.
        if evcn != u64::MAX && evcn >= (1u64 << 32) { _ntfs_bad_inode(&mut (*ni).vfs_inode); return core::ptr::null_mut(); }
    } else if evcn != u64::MAX && evcn >= (*mi).sbi.used.bitmap.nbits { _ntfs_bad_inode(&mut (*ni).vfs_inode); return core::ptr::null_mut(); }
    data_size = le64_to_cpu((*attr).nres.data_size); if le64_to_cpu((*attr).nres.valid_size) > data_size { _ntfs_bad_inode(&mut (*ni).vfs_inode); return core::ptr::null_mut(); }
    alloc_size = le64_to_cpu((*attr).nres.alloc_size); if data_size > alloc_size || (alloc_size & (*mi).sbi.cluster_mask as u64) != 0 { _ntfs_bad_inode(&mut (*ni).vfs_inode); return core::ptr::null_mut(); }
    if (*attr).nres.svcn == 0 && is_attr_ext(attr) {
        if asize < SIZEOF_NONRESIDENT_EX { _ntfs_bad_inode(&mut (*ni).vfs_inode); return core::ptr::null_mut(); }
        tot_size = le64_to_cpu((*attr).nres.total_size);
        if (tot_size & (*mi).sbi.cluster_mask as u64) != 0 || tot_size > alloc_size { _ntfs_bad_inode(&mut (*ni).vfs_inode); return core::ptr::null_mut(); }
    } else if (*attr).nres.c_unit != 0 || alloc_size > (*mi).sbi.volume.size { _ntfs_bad_inode(&mut (*ni).vfs_inode); return core::ptr::null_mut(); }
    attr
}

pub unsafe fn mi_find_attr(ni: *mut ntfs_inode, mi: *mut mft_inode, mut attr: *mut ATTRIB, ty: ATTR_TYPE, name: *const __le16, name_len: u8, id: *const __le16) -> *mut ATTRIB {
    let type_in = le32_to_cpu(ty);
    loop {
        attr = mi_enum_attr(ni, mi, attr); if attr.is_null() { return core::ptr::null_mut(); }
        let atype = le32_to_cpu((*attr).type_);
        if atype > type_in { return core::ptr::null_mut(); }
        if atype < type_in || (*attr).name_len != name_len || (name_len != 0 && memcmp(attr_name(attr), name, name_len as usize * core::mem::size_of::<i16>()) != 0) || (!id.is_null() && *id != (*attr).id) { continue; }
        return attr;
    }
}

pub unsafe fn mi_write(mi: *mut mft_inode, wait: i32) -> i32 {
    if !(*mi).dirty { return 0; }
    let sbi = (*mi).sbi; let rec = (*mi).mrec;
    let err = ntfs_write_bh(sbi, &mut (*rec).rhdr, &mut (*mi).nb, wait);
    if err != 0 { return err; }
    if (*mi).rno < (*sbi).mft.recs_mirr { (*sbi).flags |= NTFS_FLAGS_MFTMIRR; }
    (*mi).dirty = false; 0
}

pub unsafe fn mi_format_new(mi: *mut mft_inode, sbi: *mut ntfs_sb_info, rno: CLST, flags: __le16, is_mft: bool) -> i32 {
    let mut err = mi_init(mi, sbi, rno); if err != 0 { return err; }
    let mut seq: u16 = 1; let rec = (*mi).mrec; let vbo = (rno as u64) << (*sbi).record_bits;
    if rno != MFT_REC_MFT && rno < MFT_REC_FREE { seq = rno as u16; }
    else if rno < (*sbi).mft.used && mi_read(mi, is_mft) == 0 && (*rec).rhdr.sign == NTFS_FILE_SIGNATURE { seq = le16_to_cpu((*rec).seq) + 1; if seq == 0 { seq = 1; } }
    memcpy(rec, (*sbi).new_rec, (*sbi).record_size);
    (*rec).seq = cpu_to_le16(seq); (*rec).flags = RECORD_FLAG_IN_USE | flags;
    if MFTRECORD_FIXUP_OFFSET == MFTRECORD_FIXUP_OFFSET_3 { (*rec).mft_record = cpu_to_le32(rno as u64); }
    (*mi).dirty = true;
    if (*mi).nb.nbufs == 0 {
        let ni = (*sbi).mft.ni; let mut lock = false;
        if is_mounted(sbi) && !is_mft { down_read(&mut (*ni).file.run_lock); lock = true; }
        err = ntfs_get_bh(sbi, &mut (*ni).file.run, vbo, (*sbi).record_size, &mut (*mi).nb);
        if lock { up_read(&mut (*ni).file.run_lock); }
    }
    err
}

/* mi_insert_attr - Reserve space for new attribute. */
pub unsafe fn mi_insert_attr(ni: *mut ntfs_inode, mi: *mut mft_inode, ty: ATTR_TYPE, name: *const __le16, name_len: u8, asize: u32, name_off: u16) -> *mut ATTRIB {
    let rec = (*mi).mrec; let sbi = (*mi).sbi; let used = le32_to_cpu((*rec).used);
    if used + asize > (*sbi).record_size { return core::ptr::null_mut(); }
    let mut attr: *mut ATTRIB = core::ptr::null_mut();
    loop { attr = mi_enum_attr(ni, mi, attr); if attr.is_null() { break; } let diff = compare_attr(attr, ty, name, name_len, (*sbi).upcase); if diff < 0 { continue; } if diff == 0 && !is_attr_indexed(attr) { return core::ptr::null_mut(); } break; }
    let (tail, place) = if attr.is_null() { (8usize, Add2Ptr(rec, used - 8)) } else { ((used - PtrOffset(rec, attr)) as usize, attr) };
    let id = mi_new_attt_id(ni, mi); memmove(Add2Ptr(place, asize), place, tail); memset(place, 0, asize as usize);
    (*place).type_ = ty; (*place).size = cpu_to_le32(asize); (*place).name_len = name_len; (*place).name_off = cpu_to_le16(name_off); (*place).id = id;
    memmove(Add2Ptr(place, name_off as u32), name, name_len as usize * core::mem::size_of::<i16>()); (*rec).used = cpu_to_le32(used + asize); (*mi).dirty = true; place
}

pub unsafe fn mi_remove_attr(ni: *mut ntfs_inode, mi: *mut mft_inode, attr: *mut ATTRIB) -> bool {
    let rec = (*mi).mrec; let aoff = PtrOffset(rec, attr); let mut used = le32_to_cpu((*rec).used); let asize = le32_to_cpu((*attr).size);
    if aoff + asize > used { return false; }
    if !ni.is_null() && is_attr_indexed(attr) && (*attr).type_ == ATTR_NAME { let links = le16_to_cpu((*ni).mi.mrec.hard_links); if links != 0 { (*ni).mi.mrec.hard_links = cpu_to_le16(links - 1); (*ni).mi.dirty = true; } }
    used -= asize; memmove(attr, Add2Ptr(attr, asize), (used - aoff) as usize); (*rec).used = cpu_to_le32(used); (*mi).dirty = true; true
}

pub unsafe fn mi_resize_attr(mi: *mut mft_inode, attr: *mut ATTRIB, bytes: i32) -> bool {
    let rec = (*mi).mrec; let aoff = PtrOffset(rec, attr); let total = le32_to_cpu((*rec).total); let mut used = le32_to_cpu((*rec).used); let asize = le32_to_cpu((*attr).size); let mut rsize = le32_to_cpu((*attr).res.data_size); let tail = used as i32 - aoff as i32 - asize as i32; if tail < 0 || aoff >= used { return false; } if bytes == 0 { return true; }
    let next = Add2Ptr(attr, asize); let dsize = ALIGN(bytes.unsigned_abs(), 8); let nsize;
    if bytes > 0 { if used + dsize > total { return false; } nsize = asize + dsize; memmove(Add2Ptr(next, dsize), next, tail as usize); memset(next, 0, dsize as usize); used += dsize; rsize += dsize; }
    else { if dsize > asize { return false; } nsize = asize - dsize; memmove(Add2Ptr(attr, nsize), next, tail as usize); used -= dsize; rsize -= dsize; }
    (*rec).used = cpu_to_le32(used); (*attr).size = cpu_to_le32(nsize); if (*attr).non_res == 0 { (*attr).res.data_size = cpu_to_le32(rsize); } (*mi).dirty = true; true
}

/* Pack runs in MFT record. If failed record is not changed. */
pub unsafe fn mi_pack_runs(mi: *mut mft_inode, attr: *mut ATTRIB, run: *const runs_tree, len: CLST) -> i32 {
    let sbi = (*mi).sbi; let rec = (*mi).mrec; let svcn = le64_to_cpu((*attr).nres.svcn); let used = le32_to_cpu((*rec).used); let aoff = PtrOffset(rec, attr); let asize = le32_to_cpu((*attr).size); let next = Add2Ptr(attr, asize); let run_off = le16_to_cpu((*attr).nres.run_off) as u32; let run_size = asize - run_off; let tail = used - aoff - asize; let dsize = (*sbi).record_size - used;
    memmove(Add2Ptr(next, dsize), next, tail as usize);
    let mut plen: CLST = 0; let err = run_pack(run, svcn, len, Add2Ptr(attr, run_off), run_size + dsize, &mut plen);
    if err < 0 { memmove(next, Add2Ptr(next, dsize), tail as usize); return err; }
    let new_run_size = ALIGN(err as u32, 8); memmove(Add2Ptr(next, new_run_size - run_size), Add2Ptr(next, dsize), tail as usize);
    (*attr).size = cpu_to_le32(asize + new_run_size - run_size); (*attr).nres.evcn = cpu_to_le64(svcn + plen - 1); (*rec).used = cpu_to_le32(used + new_run_size - run_size); (*mi).dirty = true; 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
