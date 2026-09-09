// SPDX-License-Identifier: GPL-2.0
/*
 *  fs/partitions/aix.c
 *
 *  Copyright (C) 2012-2013 Philippe De Muyter <phdm@macqel.be>
 */

// Dependency declarations supplied by the surrounding kernel translation.
use core::{ffi::c_void, mem::size_of, ptr};

type __be16 = u16;
type __be32 = u32;
type u8 = core::ffi::c_uchar;
type u16 = core::ffi::c_ushort;
type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;
type size_t = usize;
type Sector = u64;

#[repr(C)]
pub struct parsed_partitions {
    pub disk: *mut c_void,
    pub limit: i32,
    pub pp_buf: seq_buf,
}

#[repr(C)]
pub struct seq_buf {
    _private: [u8; 0],
}

extern "C" {
    fn get_capacity(disk: *mut c_void) -> u64;
    fn read_part_sector(state: *mut parsed_partitions, lba: u64, sect: *mut Sector) -> *mut u8;
    fn put_dev_sector(sect: Sector);
    fn kmalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn kzalloc_objs(size: usize, count: i32) -> *mut c_void;
    fn seq_buf_printf(buf: *mut seq_buf, fmt: *const u8, ...);
    fn put_partition(state: *mut parsed_partitions, number: i32, start: u32, length: u32);
    fn pr_warn(fmt: *const u8, ...);
}

const GFP_KERNEL: u32 = 0;
const SECTOR_SIZE: usize = 512;
const LVM_MAXLVS: usize = 256;

#[repr(C)]
struct lvm_rec {
    lvm_id: [u8; 4],
    reserved4: [u8; 16],
    lvmarea_len: __be32,
    vgda_len: __be32,
    vgda_psn: [__be32; 2],
    reserved36: [u8; 10],
    pp_size: __be16,
    reserved46: [u8; 12],
    version: __be16,
}

#[repr(C)]
struct vgda {
    secs: __be32,
    usec: __be32,
    reserved8: [u8; 16],
    numlvs: __be16,
    maxlvs: __be16,
    pp_size: __be16,
    numpvs: __be16,
    total_vgdas: __be16,
    vgda_size: __be16,
}

#[repr(C)]
struct lvd {
    lv_ix: __be16,
    res2: __be16,
    res4: __be16,
    maxsize: __be16,
    lv_state: __be16,
    mirror: __be16,
    mirror_policy: __be16,
    num_lps: __be16,
    res10: [__be16; 8],
}

#[repr(C)]
struct lvname { name: [u8; 64] }

#[repr(C)]
struct ppe {
    lv_ix: __be16,
    res2: u16,
    res4: u16,
    lp_ix: __be16,
    res8: [u16; 12],
}

#[repr(C)]
struct pvd {
    reserved0: [u8; 16],
    pp_count: __be16,
    reserved18: [u8; 2],
    psn_part1: __be32,
    reserved24: [u8; 8],
    ppe: [ppe; 1016],
}

#[inline]
unsafe fn be16_to_cpu(v: __be16) -> u16 { u16::from_be(v) }
#[inline]
unsafe fn be32_to_cpu(v: __be32) -> u32 { u32::from_be(v) }

unsafe fn read_lba(state: *mut parsed_partitions, mut lba: u64, mut buffer: *mut u8, mut count: usize) -> usize {
    let mut totalreadcount = 0;
    if buffer.is_null() || lba.wrapping_add((count / 512) as u64) > get_capacity((*state).disk).wrapping_sub(1) { return 0; }
    while count != 0 {
        let mut copied = 512usize;
        let mut sect = 0;
        let data = read_part_sector(state, lba, &mut sect);
        lba = lba.wrapping_add(1);
        if data.is_null() { break; }
        if copied > count { copied = count; }
        ptr::copy_nonoverlapping(data, buffer, copied);
        put_dev_sector(sect);
        buffer = buffer.add(copied);
        totalreadcount += copied;
        count -= copied;
    }
    totalreadcount
}

unsafe fn alloc_pvd(state: *mut parsed_partitions, lba: u32) -> *mut pvd {
    let count = size_of::<pvd>();
    let p = kmalloc(count, GFP_KERNEL) as *mut pvd;
    if p.is_null() { return ptr::null_mut(); }
    if read_lba(state, lba as u64, p as *mut u8, count) < count { kfree(p as *mut c_void); return ptr::null_mut(); }
    p
}

unsafe fn alloc_lvn(state: *mut parsed_partitions, lba: u32) -> *mut lvname {
    let count = size_of::<lvname>() * LVM_MAXLVS;
    let p = kmalloc(count, GFP_KERNEL) as *mut lvname;
    if p.is_null() { return ptr::null_mut(); }
    if read_lba(state, lba as u64, p as *mut u8, count) < count { kfree(p as *mut c_void); return ptr::null_mut(); }
    p
}

#[repr(C)]
struct lv_info { pps_per_lv: u16, pps_found: u16, lv_is_contiguous: u8 }

pub unsafe fn aix_partition(state: *mut parsed_partitions) -> i32 {
    let mut ret = 0;
    let mut sect = 0;
    let mut vgda_len = 0u32;
    let mut vgda_sector = 0u32;
    let mut pp_blocks_size = 0u32;
    let mut numlvs = 0i32;
    let mut pvd: *mut pvd = ptr::null_mut();
    let mut n: *mut lvname = ptr::null_mut();
    let d = read_part_sector(state, 7, &mut sect);
    if !d.is_null() {
        let p = d as *mut lvm_rec;
        let lvm_version = be16_to_cpu((*p).version);
        if lvm_version == 1 {
            let pp_size_log2 = be16_to_cpu((*p).pp_size);
            pp_blocks_size = (1u32 << pp_size_log2) / 512;
            seq_buf_printf(&mut (*state).pp_buf, b" AIX LVM header version %u found\0".as_ptr(), lvm_version);
            vgda_len = be32_to_cpu((*p).vgda_len);
            vgda_sector = be32_to_cpu((*p).vgda_psn[0]);
        } else { seq_buf_printf(&mut (*state).pp_buf, b" unsupported AIX LVM version %d found\0".as_ptr(), lvm_version); }
        put_dev_sector(sect);
    }
    let d = if vgda_sector != 0 { read_part_sector(state, vgda_sector as u64, &mut sect) } else { ptr::null_mut() };
    if !d.is_null() { numlvs = be16_to_cpu((*(d as *mut vgda)).numlvs) as i32; put_dev_sector(sect); }
    let lvip = kzalloc_objs(size_of::<lv_info>(), (*state).limit) as *mut lv_info;
    if lvip.is_null() { return 0; }
    let d = if numlvs != 0 { read_part_sector(state, (vgda_sector + 1) as u64, &mut sect) } else { ptr::null_mut() };
    if !d.is_null() {
        let p = d as *mut lvd;
        n = alloc_lvn(state, vgda_sector.wrapping_add(vgda_len).wrapping_sub(33));
        if !n.is_null() {
            let mut foundlvs = 0;
            let max = core::cmp::min((*state).limit as usize, SECTOR_SIZE / size_of::<lvd>());
            for i in 0..max { if foundlvs >= numlvs { break; } (*lvip.add(i)).pps_per_lv = be16_to_cpu((*p.add(i)).num_lps); if (*lvip.add(i)).pps_per_lv != 0 { foundlvs += 1; } }
            pvd = alloc_pvd(state, vgda_sector + 17);
        }
        put_dev_sector(sect);
    }
    if !pvd.is_null() {
        let mut numpps = be16_to_cpu((*pvd).pp_count) as usize;
        if numpps > 1016 { numpps = 1016; }
        let psn_part1 = be32_to_cpu((*pvd).psn_part1);
        let mut cur_lv_ix: i32 = -1; let mut next_lp_ix = 1u16;
        for i in 0..numpps {
            let p = (*pvd).ppe.as_ptr().add(i); let lp_ix = be16_to_cpu((*p).lp_ix);
            if lp_ix == 0 { next_lp_ix = 1; continue; }
            let lv_ix = be16_to_cpu((*p).lv_ix).wrapping_sub(1) as usize;
            if lv_ix >= (*state).limit as usize { cur_lv_ix = -1; continue; }
            (*lvip.add(lv_ix)).pps_found += 1;
            if lp_ix == 1 { cur_lv_ix = lv_ix as i32; next_lp_ix = 1; } else if lv_ix as i32 != cur_lv_ix || lp_ix != next_lp_ix { next_lp_ix = 1; continue; }
            if lp_ix == (*lvip.add(lv_ix)).pps_per_lv { put_partition(state, (lv_ix + 1) as i32, ((i + 1 - lp_ix as usize) as u32).wrapping_mul(pp_blocks_size).wrapping_add(psn_part1), ((*lvip.add(lv_ix)).pps_per_lv as u32).wrapping_mul(pp_blocks_size)); seq_buf_printf(&mut (*state).pp_buf, b" <%s>\n\0".as_ptr(), (*n.add(lv_ix)).name.as_ptr()); (*lvip.add(lv_ix)).lv_is_contiguous = 1; ret = 1; next_lp_ix = 1; } else { next_lp_ix += 1; }
        }
        for i in 0..(*state).limit as usize { if (*lvip.add(i)).pps_found != 0 && (*lvip.add(i)).lv_is_contiguous == 0 { pr_warn(b"partition %s (%u pp's found) is not contiguous\n\0".as_ptr(), (*n.add(i)).name.as_ptr(), (*lvip.add(i)).pps_found); } }
        kfree(pvd as *mut c_void);
    }
    kfree(n as *mut c_void); kfree(lvip as *mut c_void); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
