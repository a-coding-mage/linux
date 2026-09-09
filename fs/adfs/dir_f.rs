// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/fs/adfs/dir_f.c
 *
 * Copyright (C) 1997-1999 Russell King
 *
 *  E and F format directory handling
 */

/* Dependencies supplied by the surrounding ADFS implementation. */

/* Read an (unaligned) value of length 1..4 bytes. */
#[inline]
unsafe fn adfs_readval(p: *const u8, len: i32) -> u32 {
    let mut val: u32 = 0;
    match len {
        4 => {
            val |= (*p.add(3) as u32) << 24;
            val |= (*p.add(2) as u32) << 16;
            val |= (*p.add(1) as u32) << 8;
            val |= *p as u32;
        }
        3 => {
            val |= (*p.add(2) as u32) << 16;
            val |= (*p.add(1) as u32) << 8;
            val |= *p as u32;
        }
        2 => {
            val |= (*p.add(1) as u32) << 8;
            val |= *p as u32;
        }
        _ => val |= *p as u32,
    }
    val
}

#[inline]
unsafe fn adfs_writeval(p: *mut u8, len: i32, val: u32) {
    match len {
        4 => {
            *p.add(3) = (val >> 24) as u8;
            *p.add(2) = (val >> 16) as u8;
            *p.add(1) = (val >> 8) as u8;
            *p = val as u8;
        }
        3 => {
            *p.add(2) = (val >> 16) as u8;
            *p.add(1) = (val >> 8) as u8;
            *p = val as u8;
        }
        2 => {
            *p.add(1) = (val >> 8) as u8;
            *p = val as u8;
        }
        _ => *p = val as u8,
    }
}

#[inline]
fn ror13(v: u32) -> u32 { (v >> 13) | (v << 19) }

/* The following helpers preserve the C macros' buffer-head addressing. */
unsafe fn dir_u8(bh: *const *mut buffer_head, blocksize_bits: i32, idx: i32) -> u8 {
    let buf = idx >> blocksize_bits;
    let off = idx - (buf << blocksize_bits);
    *((*bh.add(buf as usize)).b_data.add(off as usize) as *const u8)
}

unsafe fn dir_u32(bh: *const *mut buffer_head, blocksize_bits: i32, idx: i32) -> u32 {
    let buf = idx >> blocksize_bits;
    let off = idx - (buf << blocksize_bits);
    *((*bh.add(buf as usize)).b_data.add(off as usize) as *const u32)
}

unsafe fn bufoff(bh: *const *mut buffer_head, blocksize_bits: i32, idx: i32) -> *mut u8 {
    let buf = idx >> blocksize_bits;
    let off = idx - (buf << blocksize_bits);
    (*bh.add(buf as usize)).b_data.add(off as usize)
}

unsafe fn adfs_dir_checkbyte(dir: *const adfs_dir) -> u8 {
    let bh = (*dir).bh;
    let blocksize_bits = (*dir).sb.s_blocksize_bits;
    let mut dircheck: u32 = 0;
    let mut last: i32 = 5 - 26;
    let mut i: i32 = 0;

    loop {
        last += 26;
        loop {
            dircheck = u32::from_le(dir_u32(bh, blocksize_bits, i)) ^ ror13(dircheck);
            i += core::mem::size_of::<u32>() as i32;
            if i >= (last & !3) { break; }
        }
        if dir_u8(bh, blocksize_bits, last) == 0 { break; }
    }

    if i != last {
        let mut ptr = bufoff(bh, blocksize_bits, i);
        let end = ptr.add((last - i) as usize);
        while ptr < end {
            dircheck = *ptr as u32 ^ ror13(dircheck);
            ptr = ptr.add(1);
        }
    }

    let mut ptr = bufoff(bh, blocksize_bits, 2008) as *const u32;
    let end = ptr.add(9);
    while ptr < end {
        dircheck = u32::from_le(*ptr) ^ ror13(dircheck);
        ptr = ptr.add(1);
    }
    (dircheck ^ (dircheck >> 8) ^ (dircheck >> 16) ^ (dircheck >> 24)) as u8
}

unsafe fn adfs_f_validate(dir: *mut adfs_dir) -> i32 {
    let head = (*dir).dirhead;
    let tail = (*dir).newtail;
    if (*head).startmasseq != (*tail).endmasseq || (*tail).dirlastmask != 0 ||
       (*tail).reserved[0] != 0 || (*tail).reserved[1] != 0 ||
       (core::slice::from_raw_parts((&(*head).startname) as *const _ as *const u8, 4) != b"Nick" &&
        core::slice::from_raw_parts((&(*head).startname) as *const _ as *const u8, 4) != b"Hugo") ||
       core::slice::from_raw_parts((&(*head).startname) as *const _ as *const u8, 4) !=
       core::slice::from_raw_parts((&(*tail).endname) as *const _ as *const u8, 4) ||
       adfs_dir_checkbyte(dir) != (*tail).dircheckbyte { return -EIO; }
    0
}

unsafe fn adfs_f_read(sb: *mut super_block, indaddr: u32, size: u32, dir: *mut adfs_dir) -> i32 {
    if size != 0 && size != ADFS_NEWDIR_SIZE { return -EIO; }
    let ret = adfs_dir_read_buffers(sb, indaddr, ADFS_NEWDIR_SIZE, dir);
    if ret != 0 { return ret; }
    (*dir).dirhead = bufoff((*dir).bh, (*sb).s_blocksize_bits, 0) as *mut adfs_dirheader;
    (*dir).newtail = bufoff((*dir).bh, (*sb).s_blocksize_bits, 2007) as *mut adfs_newdirtail;
    if adfs_f_validate(dir) != 0 {
        adfs_error(sb, b"dir %06x is corrupted\0".as_ptr() as *const i8, indaddr);
        adfs_dir_relse(dir);
        return -EIO;
    }
    (*dir).parent_id = adfs_readval((*dir).newtail as *const u8, 3);
    0
}

// The remaining operations retain the original external ADFS interfaces.
unsafe fn __adfs_dir_get(dir: *mut adfs_dir, pos: i32, obj: *mut object_info) -> i32 {
    let mut de: adfs_direntry = core::mem::zeroed();
    let ret = adfs_dir_copyfrom(&mut de, dir, pos, 26);
    if ret != 0 { return ret; }
    if de.dirobname[0] == 0 { return -ENOENT; }
    let mut n = 0usize;
    while n < ADFS_F_NAME_LEN && de.dirobname[n] >= b' ' {
        (*obj).name[n] = de.dirobname[n]; n += 1;
    }
    (*obj).name_len = n;
    (*obj).indaddr = adfs_readval(de.dirinddiscadd.as_ptr(), 3);
    (*obj).loadaddr = adfs_readval(de.dirload.as_ptr(), 4);
    (*obj).execaddr = adfs_readval(de.direxec.as_ptr(), 4);
    (*obj).size = adfs_readval(de.dirlen.as_ptr(), 4);
    (*obj).attr = de.newdiratts;
    adfs_object_fixup(dir, obj);
    0
}

unsafe fn adfs_f_setpos(dir: *mut adfs_dir, fpos: u32) -> i32 {
    if fpos >= ADFS_NUM_DIR_ENTRIES { return -ENOENT; }
    (*dir).pos = 5 + (fpos * 26) as i32; 0
}

unsafe fn adfs_f_getnext(dir: *mut adfs_dir, obj: *mut object_info) -> i32 {
    let ret = __adfs_dir_get(dir, (*dir).pos, obj);
    if ret == 0 { (*dir).pos += 26; }
    ret
}

unsafe fn adfs_f_iterate(dir: *mut adfs_dir, ctx: *mut dir_context) -> i32 {
    let mut obj: object_info = core::mem::zeroed();
    let mut pos = 5 + ((*ctx).pos - 2) * 26;
    while (*ctx).pos < 2 + ADFS_NUM_DIR_ENTRIES {
        if __adfs_dir_get(dir, pos, &mut obj) != 0 { break; }
        if !dir_emit(ctx, obj.name.as_ptr(), obj.name_len, obj.indaddr, DT_UNKNOWN) { break; }
        pos += 26; (*ctx).pos += 1;
    }
    0
}

unsafe fn adfs_obj2dir(de: *mut adfs_direntry, obj: *const object_info) {
    adfs_writeval((*de).dirinddiscadd.as_mut_ptr(), 3, (*obj).indaddr);
    adfs_writeval((*de).dirload.as_mut_ptr(), 4, (*obj).loadaddr);
    adfs_writeval((*de).direxec.as_mut_ptr(), 4, (*obj).execaddr);
    adfs_writeval((*de).dirlen.as_mut_ptr(), 4, (*obj).size);
    (*de).newdiratts = (*obj).attr;
}

unsafe fn adfs_f_update(dir: *mut adfs_dir, obj: *const object_info) -> i32 {
    let mut de: adfs_direntry = core::mem::zeroed();
    let mut offset = 5 - core::mem::size_of::<adfs_direntry>() as i32;
    loop {
        offset += core::mem::size_of::<adfs_direntry>() as i32;
        if adfs_dir_copyfrom(&mut de, dir, offset, core::mem::size_of::<adfs_direntry>() as i32) != 0 {
            adfs_error((*dir).sb, b"error reading directory entry\0".as_ptr() as *const i8);
            return -ENOENT;
        }
        if de.dirobname[0] == 0 {
            adfs_error((*dir).sb, b"unable to locate entry to update\0".as_ptr() as *const i8);
            return -ENOENT;
        }
        if adfs_readval(de.dirinddiscadd.as_ptr(), 3) == (*obj).indaddr { break; }
    }
    adfs_obj2dir(&mut de, obj);
    adfs_dir_copyto(dir, offset, &de, 26)
}

unsafe fn adfs_f_commit(dir: *mut adfs_dir) -> i32 {
    (*dir).dirhead.startmasseq += 1;
    (*dir).newtail.endmasseq += 1;
    (*dir).newtail.dircheckbyte = adfs_dir_checkbyte(dir);
    let ret = adfs_f_validate(dir);
    if ret != 0 { adfs_msg((*dir).sb, KERN_ERR, b"error: update broke directory\0".as_ptr() as *const i8); }
    ret
}

const adfs_f_dir_ops: adfs_dir_ops = adfs_dir_ops {
    read: Some(adfs_f_read), iterate: Some(adfs_f_iterate), setpos: Some(adfs_f_setpos),
    getnext: Some(adfs_f_getnext), update: Some(adfs_f_update), commit: Some(adfs_f_commit),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
