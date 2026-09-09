// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/fs/adfs/dir_fplus.c
 *
 *  Copyright (C) 1997-1999 Russell King
 */

use core::mem::{offset_of, size_of};
use crate::*;

unsafe fn adfs_fplus_offset(h: *const adfs_bigdirheader, pos: u32) -> u32 {
    offset_of!(adfs_bigdirheader, bigdirname) as u32
        + ((le32_to_cpu((*h).bigdirnamelen) + 3) & !3)
        + pos * size_of::<adfs_bigdirentry>() as u32
}

unsafe fn adfs_fplus_validate_header(h: *const adfs_bigdirheader) -> i32 {
    let mut size = le32_to_cpu((*h).bigdirsize);
    let mut len: u32;
    if (*h).bigdirversion[0] != 0 || (*h).bigdirversion[1] != 0 ||
       (*h).bigdirversion[2] != 0 || (*h).bigdirstartname != cpu_to_le32(BIGDIRSTARTNAME) ||
       size == 0 || size & 2047 != 0 || size > SZ_4M { return -EIO; }
    size -= size_of::<adfs_bigdirtail>() as u32 + offset_of!(adfs_bigdirheader, bigdirname) as u32;
    len = (le32_to_cpu((*h).bigdirnamelen) + 3) & !3;
    if len > size { return -EIO; }
    size -= len;
    len = le32_to_cpu((*h).bigdirnamesize);
    if len > size { return -EIO; }
    size -= len;
    len = le32_to_cpu((*h).bigdirentries);
    if len > SZ_4M / size_of::<adfs_bigdirentry>() as u32 ||
       len * size_of::<adfs_bigdirentry>() as u32 > size { return -EIO; }
    0
}

unsafe fn adfs_fplus_validate_tail(h: *const adfs_bigdirheader, t: *const adfs_bigdirtail) -> i32 {
    if (*t).bigdirendname != cpu_to_le32(BIGDIRENDNAME) ||
       (*t).bigdirendmasseq != (*h).startmasseq || (*t).reserved[0] != 0 || (*t).reserved[1] != 0 { return -EIO; }
    0
}

unsafe fn adfs_fplus_checkbyte(dir: *mut adfs_dir) -> u8 {
    let h = (*dir).bighead;
    let t = (*dir).bigtail;
    let mut end = adfs_fplus_offset(h, le32_to_cpu((*h).bigdirentries)) + le32_to_cpu((*h).bigdirnamesize);
    let mut dircheck: u32 = 0;
    let mut bi = 0usize;
    while end != 0 {
        let bh = (*dir).bhs[bi];
        let bp = (*bh).b_data as *const u32;
        let bs = core::cmp::min((*bh).b_size, end);
        let mut i = 0u32;
        while i < bs { dircheck = ror32(dircheck, 13) ^ le32_to_cpu(*bp.add((i / 4) as usize)); i += 4; }
        end -= bs;
        bi += 1;
    }
    dircheck = ror32(dircheck, 13) ^ le32_to_cpu((*t).bigdirendname);
    dircheck = ror32(dircheck, 13) ^ (*t).bigdirendmasseq;
    dircheck = ror32(dircheck, 13) ^ (*t).reserved[0];
    dircheck = ror32(dircheck, 13) ^ (*t).reserved[1];
    (dircheck ^ (dircheck >> 8) ^ (dircheck >> 16) ^ (dircheck >> 24)) as u8
}

unsafe fn adfs_fplus_read(sb: *mut super_block, indaddr: u32, size: u32, dir: *mut adfs_dir) -> i32 {
    let ret = adfs_dir_read_buffers(sb, indaddr, (*sb).s_blocksize, dir);
    if ret != 0 { return ret; }
    (*dir).bighead = (*dir).bhs[0].cast::<u8>().add(0) as *mut adfs_bigdirheader;
    let h = (*dir).bighead;
    let ret = adfs_fplus_validate_header(h);
    if ret != 0 { adfs_error(sb, c"dir %06x has malformed header", indaddr); adfs_dir_relse(dir); return ret; }
    let dirsize = le32_to_cpu((*h).bigdirsize);
    if size != 0 && dirsize != size { adfs_msg(sb, KERN_WARNING, c"dir %06x header size %X does not match directory size %X", indaddr, dirsize, size); }
    let ret = adfs_dir_read_buffers(sb, indaddr, dirsize, dir);
    if ret != 0 { return ret; }
    (*dir).bigtail = ((*dir).bhs[(*dir).nr_buffers - 1].cast::<u8>().add((*sb).s_blocksize as usize - 8)) as *mut adfs_bigdirtail;
    let ret = adfs_fplus_validate_tail(h, (*dir).bigtail);
    if ret != 0 || adfs_fplus_checkbyte(dir) != (*(*dir).bigtail).bigdircheckbyte { adfs_dir_relse(dir); return ret; }
    (*dir).parent_id = le32_to_cpu((*h).bigdirparent); 0
}

unsafe fn adfs_fplus_setpos(dir: *mut adfs_dir, fpos: u32) -> i32 { if fpos <= le32_to_cpu((*(*dir).bighead).bigdirentries) { (*dir).pos = fpos; 0 } else { -ENOENT } }

unsafe fn adfs_fplus_getnext(dir: *mut adfs_dir, obj: *mut object_info) -> i32 {
    let h = (*dir).bighead; if (*dir).pos >= le32_to_cpu((*h).bigdirentries) { return -ENOENT; }
    let mut bde: adfs_bigdirentry = core::mem::zeroed();
    let offset = adfs_fplus_offset(h, (*dir).pos); let ret = adfs_dir_copyfrom(&mut bde, dir, offset, size_of::<adfs_bigdirentry>() as u32); if ret != 0 { return ret; }
    (*obj).loadaddr=le32_to_cpu(bde.bigdirload); (*obj).execaddr=le32_to_cpu(bde.bigdirexec); (*obj).size=le32_to_cpu(bde.bigdirlen); (*obj).indaddr=le32_to_cpu(bde.bigdirindaddr); (*obj).attr=le32_to_cpu(bde.bigdirattr); (*obj).name_len=le32_to_cpu(bde.bigdirobnamelen);
    let offset = adfs_fplus_offset(h, le32_to_cpu((*h).bigdirentries)) + le32_to_cpu(bde.bigdirobnameptr);
    let ret = adfs_dir_copyfrom((*obj).name, dir, offset, (*obj).name_len); if ret != 0 { return ret; } adfs_object_fixup(dir, obj); (*dir).pos += 1; 0
}

unsafe fn adfs_fplus_iterate(dir: *mut adfs_dir, ctx: *mut dir_context) -> i32 { let mut obj: object_info = core::mem::zeroed(); if ((*ctx).pos.wrapping_sub(2) >> 32) != 0 || adfs_fplus_setpos(dir, (*ctx).pos.wrapping_sub(2)) != 0 { return 0; } while adfs_fplus_getnext(dir, &mut obj) == 0 { if !dir_emit(ctx, (*obj).name, (*obj).name_len, (*obj).indaddr, DT_UNKNOWN) { break; } (*ctx).pos += 1; } 0 }

unsafe fn adfs_fplus_update(dir: *mut adfs_dir, obj: *mut object_info) -> i32 {
    let h = (*dir).bighead;
    let mut offset = adfs_fplus_offset(h, 0) as i32 - size_of::<adfs_bigdirentry>() as i32;
    let end = adfs_fplus_offset(h, le32_to_cpu((*h).bigdirentries)) as i32;
    let mut bde: adfs_bigdirentry = core::mem::zeroed();
    loop {
        offset += size_of::<adfs_bigdirentry>() as i32;
        if offset >= end { adfs_error((*dir).sb, c"unable to locate entry to update"); return -ENOENT; }
        if adfs_dir_copyfrom(&mut bde, dir, offset as u32, size_of::<adfs_bigdirentry>() as u32) != 0 { adfs_error((*dir).sb, c"error reading directory entry"); return -ENOENT; }
        if le32_to_cpu(bde.bigdirindaddr) == (*obj).indaddr { break; }
    }
    bde.bigdirload=cpu_to_le32((*obj).loadaddr); bde.bigdirexec=cpu_to_le32((*obj).execaddr); bde.bigdirlen=cpu_to_le32((*obj).size); bde.bigdirindaddr=cpu_to_le32((*obj).indaddr); bde.bigdirattr=cpu_to_le32((*obj).attr);
    adfs_dir_copyto(dir, offset as u32, &bde, size_of::<adfs_bigdirentry>() as u32)
}

unsafe fn adfs_fplus_commit(dir: *mut adfs_dir) -> i32 {
    (*dir).bighead.startmasseq += 1; (*dir).bigtail.bigdirendmasseq += 1;
    (*dir).bigtail.bigdircheckbyte = adfs_fplus_checkbyte(dir);
    let mut ret = adfs_fplus_validate_header((*dir).bighead);
    if ret == 0 { ret = adfs_fplus_validate_tail((*dir).bighead, (*dir).bigtail); }
    ret
}

pub static adfs_fplus_dir_ops: adfs_dir_ops = adfs_dir_ops {
    read: Some(adfs_fplus_read), iterate: Some(adfs_fplus_iterate), setpos: Some(adfs_fplus_setpos),
    getnext: Some(adfs_fplus_getnext), update: Some(adfs_fplus_update), commit: Some(adfs_fplus_commit),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
