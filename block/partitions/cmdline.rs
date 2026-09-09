// SPDX-License-Identifier: GPL-2.0
/* Read block device partition table from the command line. */

// Kernel-provided types, constants, allocation, parsing, logging, and
// partition helpers are external dependencies supplied by the surrounding
// translation unit.
use core::ffi::{c_char, c_int, c_uint, c_void};

const BDEVNAME_SIZE: usize = 32;
const PAGE_SIZE: u64 = 4096;
const PF_RDONLY: c_int = 0x01;
const PF_POWERUP_LOCK: c_int = 0x02;

#[repr(C)]
struct CmdlineSubpart {
    name: [c_char; BDEVNAME_SIZE],
    from: u64,
    size: u64,
    flags: c_int,
    next_subpart: *mut CmdlineSubpart,
}

#[repr(C)]
struct CmdlineParts {
    name: [c_char; BDEVNAME_SIZE],
    nr_subparts: c_uint,
    subpart: *mut CmdlineSubpart,
    next_parts: *mut CmdlineParts,
}

// These layouts represent the fields consumed by this source; their complete
// definitions are supplied by the kernel partitioning code.
#[repr(C)]
struct PartitionMetaInfo { volname: [c_char; BDEVNAME_SIZE] }
#[repr(C)]
struct ParsedPart { flags: c_uint, info: PartitionMetaInfo, has_info: bool, from: u64, size: u64 }
#[repr(C)]
struct Disk { disk_name: *const c_char }
#[repr(C)]
struct SeqBuf { _private: [u8; 0] }
#[repr(C)]
struct ParsedPartitions {
    limit: c_int,
    parts: *mut ParsedPart,
    pp_buf: SeqBuf,
    disk: *mut Disk,
}

const ADDPART_FLAG_READONLY: c_uint = 1;

extern "C" {
    fn kzalloc_obj(size: usize) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memparse(ptr: *const c_char, retptr: *mut *mut c_char) -> u64;
    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn strscpy(dst: *mut c_char, src: *const c_char, size: usize) -> isize;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn kstrdup(s: *const c_char, flags: c_uint) -> *mut c_char;
    fn put_partition(state: *mut ParsedPartitions, slot: c_int, from: u64, size: u64);
    fn seq_buf_printf(buf: *mut SeqBuf, fmt: *const c_char, ...);
    fn seq_buf_puts(buf: *mut SeqBuf, s: *const c_char);
    fn get_capacity(disk: *mut Disk) -> u64;
    fn pr_warn(fmt: *const c_char, ...);
}

static mut CMDLINE: *mut c_char = core::ptr::null_mut();
static mut BDEV_PARTS: *mut CmdlineParts = core::ptr::null_mut();

unsafe fn parse_subpart(subpart: *mut *mut CmdlineSubpart, mut partdef: *mut c_char) -> c_int {
    let mut ret = 0;
    *subpart = core::ptr::null_mut();
    let new_subpart = kzalloc_obj(core::mem::size_of::<CmdlineSubpart>()) as *mut CmdlineSubpart;
    if new_subpart.is_null() { return -12; }
    if *partdef as u8 == b'-' {
        (*new_subpart).size = u64::MAX;
        partdef = partdef.add(1);
    } else {
        (*new_subpart).size = memparse(partdef, &mut partdef);
        if (*new_subpart).size < PAGE_SIZE { pr_warn(b"cmdline partition size is invalid.\0".as_ptr() as _); ret = -22; goto_fail(new_subpart, ret); return ret; }
    }
    if *partdef as u8 == b'@' { partdef = partdef.add(1); (*new_subpart).from = memparse(partdef, &mut partdef); } else { (*new_subpart).from = u64::MAX; }
    if *partdef as u8 == b'(' {
        partdef = partdef.add(1);
        let next = strsep(&mut partdef, b")\0".as_ptr() as _);
        if next.is_null() { pr_warn(b"cmdline partition format is invalid.\0".as_ptr() as _); ret = -22; goto_fail(new_subpart, ret); return ret; }
        strscpy((*new_subpart).name.as_mut_ptr(), next, BDEVNAME_SIZE);
    } else { (*new_subpart).name[0] = 0; }
    (*new_subpart).flags = 0;
    if strncmp(partdef, b"ro\0".as_ptr() as _, 2) == 0 { (*new_subpart).flags |= PF_RDONLY; partdef = partdef.add(2); }
    if strncmp(partdef, b"lk\0".as_ptr() as _, 2) == 0 { (*new_subpart).flags |= PF_POWERUP_LOCK; partdef = partdef.add(2); }
    *subpart = new_subpart; 0
}

unsafe fn goto_fail(p: *mut CmdlineSubpart, _ret: c_int) { kfree(p as _); }

unsafe fn free_subpart(parts: *mut CmdlineParts) { while !(*parts).subpart.is_null() { let p = (*parts).subpart; (*parts).subpart = (*p).next_subpart; kfree(p as _); } }

unsafe fn parse_parts(parts: *mut *mut CmdlineParts, mut bdevdef: *mut c_char) -> c_int {
    *parts = core::ptr::null_mut();
    let newparts = kzalloc_obj(core::mem::size_of::<CmdlineParts>()) as *mut CmdlineParts;
    if newparts.is_null() { return -12; }
    let next = strsep(&mut bdevdef, b":\0".as_ptr() as _);
    if next.is_null() { pr_warn(b"cmdline partition has no block device.\0".as_ptr() as _); free_subpart(newparts); kfree(newparts as _); return -22; }
    strscpy((*newparts).name.as_mut_ptr(), next, BDEVNAME_SIZE); (*newparts).nr_subparts = 0;
    let mut next_subpart = &mut (*newparts).subpart as *mut *mut CmdlineSubpart;
    loop { let next = strsep(&mut bdevdef, b",\0".as_ptr() as _); if next.is_null() { break; } let ret = parse_subpart(next_subpart, next); if ret != 0 { free_subpart(newparts); kfree(newparts as _); return ret; } (*newparts).nr_subparts += 1; next_subpart = &mut (*(*next_subpart)).next_subpart; }
    if (*newparts).subpart.is_null() { pr_warn(b"cmdline partition has no valid partition.\0".as_ptr() as _); free_subpart(newparts); kfree(newparts as _); return -22; }
    *parts = newparts; 0
}

unsafe fn cmdline_parts_free(parts: *mut *mut CmdlineParts) { while !(*parts).is_null() { let next = (**parts).next_parts; free_subpart(*parts); kfree(*parts as _); *parts = next; } }

unsafe fn cmdline_parts_parse(parts: *mut *mut CmdlineParts, cmdline: *const c_char) -> c_int {
    *parts = core::ptr::null_mut(); let mut buf = kstrdup(cmdline, 0); if buf.is_null() { return -12; } let mut pbuf = buf; let mut next_parts = parts;
    loop { let next = strsep(&mut pbuf, b";\0".as_ptr() as _); if next.is_null() { break; } let ret = parse_parts(next_parts, next); if ret != 0 { cmdline_parts_free(parts); kfree(buf as _); return ret; } next_parts = &mut (**next_parts).next_parts; }
    let ret = if (*parts).is_null() { pr_warn(b"cmdline partition has no valid partition.\0".as_ptr() as _); -22 } else { 0 }; kfree(buf as _); ret
}

unsafe fn cmdline_parts_find(mut parts: *mut CmdlineParts, bdev: *const c_char) -> *mut CmdlineParts { while !parts.is_null() && strncmp(bdev, (*parts).name.as_ptr(), BDEVNAME_SIZE) != 0 { parts = (*parts).next_parts; } parts }

unsafe fn add_part(slot: c_int, subpart: *mut CmdlineSubpart, state: *mut ParsedPartitions) -> c_int {
    if slot >= (*state).limit { return 1; } put_partition(state, slot, (*subpart).from >> 9, (*subpart).size >> 9);
    let part = (*state).parts.add(slot as usize); if (*subpart).flags & PF_RDONLY != 0 { (*part).flags |= ADDPART_FLAG_READONLY; }
    strscpy((*part).info.volname.as_mut_ptr(), (*subpart).name.as_ptr(), BDEVNAME_SIZE); seq_buf_printf(&mut (*state).pp_buf, b"(%s)\0".as_ptr() as _, (*part).info.volname.as_ptr()); (*part).has_info = true; 0
}

unsafe fn cmdline_parts_set(parts: *mut CmdlineParts, disk_size: u64, state: *mut ParsedPartitions) -> c_int {
    let mut from = 0; let mut subpart = (*parts).subpart; let mut slot = 1;
    while !subpart.is_null() { if (*subpart).from == u64::MAX { (*subpart).from = from; } else { from = (*subpart).from; } if from >= disk_size { break; } if (*subpart).size > disk_size - from { (*subpart).size = disk_size - from; } from += (*subpart).size; if add_part(slot, subpart, state) != 0 { break; } subpart = (*subpart).next_subpart; slot += 1; } slot
}

unsafe fn has_overlaps(from: u64, size: u64, from2: u64, size2: u64) -> bool { let end = from + size; let end2 = from2 + size2; (from >= from2 && from < end2) || (end > from2 && end <= end2) || (from2 >= from && from2 < end) || (end2 > from && end2 <= end) }

unsafe fn overlaps_warns_header() { pr_warn(b"Overlapping partitions are used in command line partitions.\0".as_ptr() as _); pr_warn(b"Don't use filesystems on overlapping partitions:\0".as_ptr() as _); }

unsafe fn cmdline_parts_verifier(mut slot: c_int, state: *mut ParsedPartitions) { let mut header = true; while slot < (*state).limit && (*state).parts.add(slot as usize).read().has_info { let mut i = slot + 1; while i < (*state).limit && (*state).parts.add(i as usize).read().has_info { let a = (*state).parts.add(slot as usize).read(); let b = (*state).parts.add(i as usize).read(); if has_overlaps(a.from, a.size, b.from, b.size) { if header { header = false; overlaps_warns_header(); } pr_warn(b"%s[%llu,%llu] overlaps with %s[%llu,%llu].\0".as_ptr() as _, a.info.volname.as_ptr(), a.from << 9, a.size << 9, b.info.volname.as_ptr(), b.from << 9, b.size << 9); } i += 1; } slot += 1; } }

pub unsafe extern "C" fn cmdline_partition(state: *mut ParsedPartitions) -> c_int {
    if !CMDLINE.is_null() { if !BDEV_PARTS.is_null() { cmdline_parts_free(&mut BDEV_PARTS); } if cmdline_parts_parse(&mut BDEV_PARTS, CMDLINE) != 0 { CMDLINE = core::ptr::null_mut(); return -1; } CMDLINE = core::ptr::null_mut(); }
    if BDEV_PARTS.is_null() { return 0; } let parts = cmdline_parts_find(BDEV_PARTS, (*(*state).disk).disk_name); if parts.is_null() { return 0; }
    let disk_size = get_capacity((*state).disk) << 9; cmdline_parts_set(parts, disk_size, state); cmdline_parts_verifier(1, state); seq_buf_puts(&mut (*state).pp_buf, b"\n\0".as_ptr() as _); 1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
