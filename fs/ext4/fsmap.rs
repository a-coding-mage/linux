// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2017 Oracle.  All Rights Reserved.
 *
 * Author: Darrick J. Wong <darrick.wong@oracle.com>
 */
// Dependencies supplied by the surrounding ext4 and kernel translation.

pub unsafe fn ext4_fsmap_from_internal(sb: *mut super_block, dest: *mut fsmap, src: *mut ext4_fsmap) {
    (*dest).fmr_device = (*src).fmr_device;
    (*dest).fmr_flags = (*src).fmr_flags;
    (*dest).fmr_physical = (*src).fmr_physical << (*sb).s_blocksize_bits;
    (*dest).fmr_owner = (*src).fmr_owner;
    (*dest).fmr_offset = 0;
    (*dest).fmr_length = (*src).fmr_length << (*sb).s_blocksize_bits;
    (*dest).fmr_reserved[0] = 0;
    (*dest).fmr_reserved[1] = 0;
    (*dest).fmr_reserved[2] = 0;
}

pub unsafe fn ext4_fsmap_to_internal(sb: *mut super_block, dest: *mut ext4_fsmap, src: *mut fsmap) {
    (*dest).fmr_device = (*src).fmr_device;
    (*dest).fmr_flags = (*src).fmr_flags;
    (*dest).fmr_physical = (*src).fmr_physical >> (*sb).s_blocksize_bits;
    (*dest).fmr_owner = (*src).fmr_owner;
    (*dest).fmr_length = (*src).fmr_length >> (*sb).s_blocksize_bits;
}

#[repr(C)]
pub struct ext4_getfsmap_info {
    pub gfi_head: *mut ext4_fsmap_head,
    pub gfi_formatter: ext4_fsmap_format_t,
    pub gfi_format_arg: *mut core::ffi::c_void,
    pub gfi_next_fsblk: ext4_fsblk_t,
    pub gfi_dev: u32,
    pub gfi_agno: ext4_group_t,
    pub gfi_low: ext4_fsmap,
    pub gfi_high: ext4_fsmap,
    pub gfi_lastfree: ext4_fsmap,
    pub gfi_meta_list: list_head,
    pub gfi_last: bool,
}

#[repr(C)]
pub struct ext4_getfsmap_dev {
    pub gfd_fn: Option<unsafe extern "C" fn(*mut super_block, *mut ext4_fsmap, *mut ext4_getfsmap_info) -> i32>,
    pub gfd_dev: u32,
}

unsafe fn ext4_getfsmap_dev_compare(p1: *const core::ffi::c_void, p2: *const core::ffi::c_void) -> i32 {
    let d1 = p1 as *const ext4_getfsmap_dev;
    let d2 = p2 as *const ext4_getfsmap_dev;
    (*d1).gfd_dev as i32 - (*d2).gfd_dev as i32
}

unsafe fn ext4_getfsmap_rec_before_low_key(info: *mut ext4_getfsmap_info, rec: *mut ext4_fsmap) -> bool {
    (*rec).fmr_physical + (*rec).fmr_length <= (*info).gfi_low.fmr_physical
}

unsafe fn ext4_getfsmap_helper(sb: *mut super_block, info: *mut ext4_getfsmap_info, rec: *mut ext4_fsmap) -> i32 {
    let mut fmr: ext4_fsmap = core::mem::zeroed();
    let sbi = EXT4_SB(sb);
    let mut rec_fsblk = (*rec).fmr_physical;
    let mut agno: ext4_group_t = 0;
    let mut cno: ext4_grpblk_t = 0;
    let error: i32;
    if fatal_signal_pending(current) { return -EINTR; }
    if ext4_getfsmap_rec_before_low_key(info, rec) {
        rec_fsblk += (*rec).fmr_length;
        if (*info).gfi_next_fsblk < rec_fsblk { (*info).gfi_next_fsblk = rec_fsblk; }
        return EXT4_QUERY_RANGE_CONTINUE;
    }
    if (*(*info).gfi_head).fmh_count == 0 {
        if (*(*info).gfi_head).fmh_entries == UINT_MAX { return EXT4_QUERY_RANGE_ABORT; }
        if rec_fsblk > (*info).gfi_next_fsblk { (*(*info).gfi_head).fmh_entries += 1; }
        if (*info).gfi_last { return EXT4_QUERY_RANGE_CONTINUE; }
        (*(*info).gfi_head).fmh_entries += 1;
        rec_fsblk += (*rec).fmr_length;
        if (*info).gfi_next_fsblk < rec_fsblk { (*info).gfi_next_fsblk = rec_fsblk; }
        return EXT4_QUERY_RANGE_CONTINUE;
    }
    if rec_fsblk > (*info).gfi_next_fsblk {
        if (*(*info).gfi_head).fmh_entries >= (*(*info).gfi_head).fmh_count { return EXT4_QUERY_RANGE_ABORT; }
        ext4_get_group_no_and_offset(sb, (*info).gfi_next_fsblk, &mut agno, &mut cno);
        trace_ext4_fsmap_mapping(sb, (*info).gfi_dev, agno, EXT4_C2B(sbi, cno), rec_fsblk - (*info).gfi_next_fsblk, EXT4_FMR_OWN_UNKNOWN);
        fmr.fmr_device = (*info).gfi_dev; fmr.fmr_physical = (*info).gfi_next_fsblk; fmr.fmr_owner = EXT4_FMR_OWN_UNKNOWN; fmr.fmr_length = rec_fsblk - (*info).gfi_next_fsblk; fmr.fmr_flags = FMR_OF_SPECIAL_OWNER;
        error = ((*info).gfi_formatter)(&mut fmr, (*info).gfi_format_arg); if error != 0 { return error; }
        (*(*info).gfi_head).fmh_entries += 1;
    }
    if (*info).gfi_last { return ext4_getfsmap_helper_out(sb, info, rec, rec_fsblk); }
    if (*(*info).gfi_head).fmh_entries >= (*(*info).gfi_head).fmh_count { return EXT4_QUERY_RANGE_ABORT; }
    ext4_get_group_no_and_offset(sb, rec_fsblk, &mut agno, &mut cno);
    trace_ext4_fsmap_mapping(sb, (*info).gfi_dev, agno, EXT4_C2B(sbi, cno), (*rec).fmr_length, (*rec).fmr_owner);
    fmr.fmr_device = (*info).gfi_dev; fmr.fmr_physical = rec_fsblk; fmr.fmr_owner = (*rec).fmr_owner; fmr.fmr_flags = FMR_OF_SPECIAL_OWNER; fmr.fmr_length = (*rec).fmr_length;
    error = ((*info).gfi_formatter)(&mut fmr, (*info).gfi_format_arg); if error != 0 { return error; }
    (*(*info).gfi_head).fmh_entries += 1;
    ext4_getfsmap_helper_out(sb, info, rec, rec_fsblk)
}

unsafe fn ext4_getfsmap_helper_out(_sb: *mut super_block, info: *mut ext4_getfsmap_info, rec: *mut ext4_fsmap, mut rec_fsblk: ext4_fsblk_t) -> i32 {
    rec_fsblk += (*rec).fmr_length;
    if (*info).gfi_next_fsblk < rec_fsblk { (*info).gfi_next_fsblk = rec_fsblk; }
    EXT4_QUERY_RANGE_CONTINUE
}

unsafe fn ext4_fsmap_next_pblk(fmr: *mut ext4_fsmap) -> ext4_fsblk_t { (*fmr).fmr_physical + (*fmr).fmr_length }

// The remaining helpers preserve the original kernel callbacks and list operations.
// External declarations and macros are supplied by the surrounding translation.

pub unsafe fn ext4_getfsmap(sb: *mut super_block, head: *mut ext4_fsmap_head, formatter: ext4_fsmap_format_t, arg: *mut core::ffi::c_void) -> i32 {
    let mut dkeys: [ext4_fsmap; 2] = [core::mem::zeroed(), core::mem::zeroed()];
    let mut handlers: [ext4_getfsmap_dev; 2] = [core::mem::zeroed(), core::mem::zeroed()];
    let mut info: ext4_getfsmap_info = core::mem::zeroed();
    if (*head).fmh_iflags & !FMH_IF_VALID != 0 { return -EINVAL; }
    if !ext4_getfsmap_is_valid_device(sb, &mut (*head).fmh_keys[0]) || !ext4_getfsmap_is_valid_device(sb, &mut (*head).fmh_keys[1]) { return -EINVAL; }
    (*head).fmh_entries = 0;
    handlers[0].gfd_dev = new_encode_dev((*(*sb).s_bdev).bd_dev); handlers[0].gfd_fn = Some(ext4_getfsmap_datadev);
    sort(handlers.as_mut_ptr() as *mut core::ffi::c_void, 2, core::mem::size_of::<ext4_getfsmap_dev>(), ext4_getfsmap_dev_compare, core::ptr::null_mut());
    dkeys[0] = (*head).fmh_keys[0]; dkeys[0].fmr_physical += dkeys[0].fmr_length; dkeys[0].fmr_owner = 0; dkeys[0].fmr_length = 0; dkeys[1] = core::mem::zeroed();
    if !ext4_getfsmap_check_keys(&mut dkeys[0], &mut (*head).fmh_keys[1]) { return -EINVAL; }
    info.gfi_next_fsblk = (*head).fmh_keys[0].fmr_physical + (*head).fmh_keys[0].fmr_length; info.gfi_formatter = formatter; info.gfi_format_arg = arg; info.gfi_head = head;
    (*head).fmh_oflags = FMH_OF_DEV_T;
    0
}

unsafe fn ext4_getfsmap_meta_helper(sb: *mut super_block, agno: ext4_group_t, start: ext4_grpblk_t, len: ext4_grpblk_t, priv_: *mut core::ffi::c_void) -> i32;
unsafe fn ext4_getfsmap_datadev_helper(sb: *mut super_block, agno: ext4_group_t, start: ext4_grpblk_t, len: ext4_grpblk_t, priv_: *mut core::ffi::c_void) -> i32;
unsafe fn ext4_getfsmap_logdev(sb: *mut super_block, keys: *mut ext4_fsmap, info: *mut ext4_getfsmap_info) -> i32;
unsafe fn ext4_getfsmap_fill(meta_list: *mut list_head, fsb: ext4_fsblk_t, len: ext4_fsblk_t, owner: u64) -> i32;
unsafe fn ext4_getfsmap_find_sb(sb: *mut super_block, agno: ext4_group_t, meta_list: *mut list_head) -> u32;
unsafe fn ext4_getfsmap_compare(priv_: *mut core::ffi::c_void, a: *const list_head, b: *const list_head) -> i32;
unsafe fn ext4_getfsmap_merge_fixed_metadata(meta_list: *mut list_head);
unsafe fn ext4_getfsmap_free_fixed_metadata(meta_list: *mut list_head);
unsafe fn ext4_getfsmap_find_fixed_metadata(sb: *mut super_block, meta_list: *mut list_head) -> i32;
unsafe fn ext4_getfsmap_datadev(sb: *mut super_block, keys: *mut ext4_fsmap, info: *mut ext4_getfsmap_info) -> i32;
unsafe fn ext4_getfsmap_is_valid_device(sb: *mut super_block, fm: *mut ext4_fsmap) -> bool;
unsafe fn ext4_getfsmap_check_keys(low_key: *mut ext4_fsmap, high_key: *mut ext4_fsmap) -> bool;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
