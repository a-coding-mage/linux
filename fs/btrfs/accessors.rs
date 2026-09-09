// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2007 Oracle.  All rights reserved.
 */

// C dependencies supplied by the surrounding translation unit:
// messages.h, extent_io.h, fs.h, and accessors.h.

unsafe fn report_setget_bounds(
    eb: *const extent_buffer,
    ptr: *const core::ffi::c_void,
    off: u32,
    size: i32,
) {
    let member_offset = ptr as usize + off as usize;
    btrfs_warn(
        (*eb).fs_info,
        if member_offset > (*eb).len as usize { "start" } else { "end" },
        ptr as usize,
        (*eb).start,
        member_offset,
        size,
    );
}

/* Copy bytes from @src1 and @src2 to @dest. */
#[inline(always)]
unsafe fn memcpy_split_src(
    dest: *mut u8,
    src1: *const u8,
    src2: *const u8,
    len1: usize,
    total: usize,
) {
    core::ptr::copy_nonoverlapping(src1, dest, len1);
    core::ptr::copy_nonoverlapping(src2, dest.add(len1), total - len1);
}

#[inline(always)]
unsafe fn btrfs_get_bits<const N: usize>(
    eb: *const extent_buffer,
    ptr: *const core::ffi::c_void,
    off: usize,
) -> u64 {
    let member_offset = ptr as usize + off;
    let idx = get_eb_folio_index(eb, member_offset);
    let oif = get_eb_offset_in_folio(eb, member_offset);
    let mut kaddr = folio_address((*eb).folios[idx]).add(oif);
    let part = (*eb).folio_size - oif;

    if member_offset + N > (*eb).len as usize {
        report_setget_bounds(eb, ptr, off as u32, N as i32);
        return 0;
    }
    if INLINE_EXTENT_BUFFER_PAGES == 1 || N == 1 || N <= part {
        let mut bytes = [0u8; 8];
        core::ptr::copy_nonoverlapping(kaddr, bytes.as_mut_ptr(), N);
        return u64::from_le_bytes(bytes);
    }

    let mut bytes = [0u8; 8];
    if N == 2 {
        bytes[0] = *kaddr;
        kaddr = folio_address((*eb).folios[idx + 1]);
        bytes[1] = *kaddr;
    } else {
        memcpy_split_src(bytes.as_mut_ptr(), kaddr, folio_address((*eb).folios[idx + 1]), part, N);
    }
    u64::from_le_bytes(bytes)
}

#[inline(always)]
unsafe fn btrfs_set_bits<const N: usize>(
    eb: *const extent_buffer,
    ptr: *mut core::ffi::c_void,
    off: usize,
    val: u64,
) {
    let member_offset = ptr as usize + off;
    let idx = get_eb_folio_index(eb, member_offset);
    let oif = get_eb_offset_in_folio(eb, member_offset);
    let mut kaddr = folio_address((*eb).folios[idx]).add(oif);
    let part = (*eb).folio_size - oif;
    let lebytes = val.to_le_bytes();

    if member_offset + N > (*eb).len as usize {
        report_setget_bounds(eb, ptr, off as u32, N as i32);
        return;
    }
    if INLINE_EXTENT_BUFFER_PAGES == 1 || N == 1 || N <= part {
        core::ptr::copy_nonoverlapping(lebytes.as_ptr(), kaddr, N);
        return;
    }
    if N == 2 {
        *kaddr = lebytes[0];
        kaddr = folio_address((*eb).folios[idx + 1]);
        *kaddr = lebytes[1];
    } else {
        core::ptr::copy_nonoverlapping(lebytes.as_ptr(), kaddr, part);
        kaddr = folio_address((*eb).folios[idx + 1]);
        core::ptr::copy_nonoverlapping(lebytes.as_ptr().add(part), kaddr, N - part);
    }
}

pub unsafe fn btrfs_get_8(eb: *const extent_buffer, ptr: *const core::ffi::c_void, off: usize) -> u8 { btrfs_get_bits::<1>(eb, ptr, off) as u8 }
pub unsafe fn btrfs_get_16(eb: *const extent_buffer, ptr: *const core::ffi::c_void, off: usize) -> u16 { btrfs_get_bits::<2>(eb, ptr, off) as u16 }
pub unsafe fn btrfs_get_32(eb: *const extent_buffer, ptr: *const core::ffi::c_void, off: usize) -> u32 { btrfs_get_bits::<4>(eb, ptr, off) as u32 }
pub unsafe fn btrfs_get_64(eb: *const extent_buffer, ptr: *const core::ffi::c_void, off: usize) -> u64 { btrfs_get_bits::<8>(eb, ptr, off) }
pub unsafe fn btrfs_set_8(eb: *const extent_buffer, ptr: *mut core::ffi::c_void, off: usize, val: u8) { btrfs_set_bits::<1>(eb, ptr, off, val as u64); }
pub unsafe fn btrfs_set_16(eb: *const extent_buffer, ptr: *mut core::ffi::c_void, off: usize, val: u16) { btrfs_set_bits::<2>(eb, ptr, off, val as u64); }
pub unsafe fn btrfs_set_32(eb: *const extent_buffer, ptr: *mut core::ffi::c_void, off: usize, val: u32) { btrfs_set_bits::<4>(eb, ptr, off, val as u64); }
pub unsafe fn btrfs_set_64(eb: *const extent_buffer, ptr: *mut core::ffi::c_void, off: usize, val: u64) { btrfs_set_bits::<8>(eb, ptr, off, val); }

pub unsafe fn btrfs_node_key(
    eb: *const extent_buffer,
    disk_key: *mut btrfs_disk_key,
    nr: i32,
) {
    let ptr = btrfs_node_key_ptr_offset(eb, nr) as *mut core::ffi::c_void;
    read_eb_member(eb, ptr as *mut btrfs_key_ptr, btrfs_key_ptr::key, disk_key);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
