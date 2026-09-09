// SPDX-License-Identifier: GPL-2.0-or-later
/* SCTP kernel implementation
 * These functions manipulate sctp tsn mapping array.
 */

use core::ffi::c_void;

#[repr(C)]
pub struct sctp_association { pub peer: sctp_peer }; 
#[repr(C)]
pub struct sctp_peer { pub sack_generation: u32 }
#[repr(C)]
pub struct sctp_transport { pub sack_generation: u32, pub asoc: *mut sctp_association }
#[repr(C)]
pub struct sctp_tsnmap {
    pub tsn_map: *mut c_ulong,
    pub len: u16,
    pub base_tsn: u32,
    pub cumulative_tsn_ack_point: u32,
    pub max_tsn_seen: u32,
    pub num_dup_tsns: u16,
}
#[repr(C)]
pub struct sctp_tsnmap_iter { pub start: u32 }
#[repr(C)]
pub struct sctp_gap_ack_block { pub start: u16, pub end: u16 }

type c_ulong = usize;
type gfp_t = u32;

const SCTP_TSN_MAP_SIZE: u32 = 65536;
const SCTP_TSN_MAP_INCREMENT: usize = 64;
const SCTP_MAX_GABS: i32 = 256;
const GFP_ATOMIC: gfp_t = 0;
const ENOMEM: i32 = 12;

extern "C" {
    fn kzalloc(size: usize, flags: gfp_t) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn bitmap_zero(map: *mut c_ulong, nbits: u16);
    fn bitmap_copy(dst: *mut c_ulong, src: *const c_ulong, nbits: u32);
    fn bitmap_shift_right(dst: *mut c_ulong, src: *const c_ulong, shift: u32, nbits: u16);
    fn test_bit(nr: u32, addr: *const c_ulong) -> bool;
    fn set_bit(nr: u32, addr: *mut c_ulong);
    fn clear_bit(nr: u32, addr: *mut c_ulong);
    fn find_first_zero_bit(addr: *const c_ulong, size: u16) -> usize;
    fn find_next_bit(addr: *const c_ulong, size: u16, offset: u16) -> usize;
    fn find_next_zero_bit(addr: *const c_ulong, size: u16, offset: usize) -> usize;
    fn bitmap_weight(addr: *const c_ulong, size: u32) -> u32;
    fn htons(x: u16) -> u16;
}

#[inline] unsafe fn TSN_lte(a: u32, b: u32) -> bool { (a.wrapping_sub(b) as i32) <= 0 }
#[inline] unsafe fn TSN_lt(a: u32, b: u32) -> bool { (a.wrapping_sub(b) as i32) < 0 }
#[inline] unsafe fn sctp_tsnmap_has_gap(map: *const sctp_tsnmap) -> bool {
    (*map).base_tsn != (*map).cumulative_tsn_ack_point.wrapping_add(1)
}

pub unsafe fn sctp_tsnmap_init(map: *mut sctp_tsnmap, len: u16, initial_tsn: u32, gfp: gfp_t) -> *mut sctp_tsnmap {
    if (*map).tsn_map.is_null() {
        (*map).tsn_map = kzalloc((len >> 3) as usize, gfp) as *mut c_ulong;
        if (*map).tsn_map.is_null() { return core::ptr::null_mut(); }
        (*map).len = len;
    } else { bitmap_zero((*map).tsn_map, (*map).len); }
    (*map).base_tsn = initial_tsn;
    (*map).cumulative_tsn_ack_point = initial_tsn.wrapping_sub(1);
    (*map).max_tsn_seen = (*map).cumulative_tsn_ack_point;
    (*map).num_dup_tsns = 0;
    map
}

pub unsafe fn sctp_tsnmap_free(map: *mut sctp_tsnmap) { (*map).len = 0; kfree((*map).tsn_map as *mut c_void); }

pub unsafe fn sctp_tsnmap_check(map: *const sctp_tsnmap, tsn: u32) -> i32 {
    if TSN_lte(tsn, (*map).cumulative_tsn_ack_point) { return 1; }
    if !TSN_lt(tsn, (*map).base_tsn.wrapping_add(SCTP_TSN_MAP_SIZE)) { return -1; }
    let gap = tsn.wrapping_sub((*map).base_tsn);
    if gap < (*map).len as u32 && test_bit(gap, (*map).tsn_map) { 1 } else { 0 }
}

pub unsafe fn sctp_tsnmap_mark(map: *mut sctp_tsnmap, tsn: u32, trans: *mut sctp_transport) -> i32 {
    if TSN_lt(tsn, (*map).base_tsn) { return 0; }
    let gap = tsn.wrapping_sub((*map).base_tsn) as u16;
    if gap >= (*map).len && sctp_tsnmap_grow(map, gap.wrapping_add(1)) == 0 { return -ENOMEM; }
    if !sctp_tsnmap_has_gap(map) && gap == 0 {
        (*map).max_tsn_seen = (*map).max_tsn_seen.wrapping_add(1);
        (*map).cumulative_tsn_ack_point = (*map).cumulative_tsn_ack_point.wrapping_add(1);
        if !trans.is_null() { (*trans).sack_generation = (*(*trans).asoc).peer.sack_generation; }
        (*map).base_tsn = (*map).base_tsn.wrapping_add(1);
    } else {
        if TSN_lt((*map).max_tsn_seen, tsn) { (*map).max_tsn_seen = tsn; }
        set_bit(gap as u32, (*map).tsn_map);
        sctp_tsnmap_update(map);
    }
    0
}

unsafe fn sctp_tsnmap_iter_init(map: *const sctp_tsnmap, iter: *mut sctp_tsnmap_iter) { (*iter).start = (*map).cumulative_tsn_ack_point.wrapping_add(1); }

unsafe fn sctp_tsnmap_next_gap_ack(map: *const sctp_tsnmap, iter: *mut sctp_tsnmap_iter, start: *mut u16, end: *mut u16) -> i32 {
    let mut ended = 0; let mut start_ = 0u16; let mut end_ = 0u16;
    if TSN_lte((*map).max_tsn_seen, (*iter).start) { return 0; }
    let offset = (*iter).start.wrapping_sub((*map).base_tsn) as u16;
    sctp_tsnmap_find_gap_ack((*map).tsn_map, offset, (*map).len, &mut start_, &mut end_);
    if start_ != 0 && end_ == 0 { end_ = (*map).len.wrapping_sub(1); }
    if end_ != 0 { *start = start_.wrapping_add(1); *end = end_.wrapping_add(1); (*iter).start = (*map).cumulative_tsn_ack_point.wrapping_add(*end as u32).wrapping_add(1); ended = 1; }
    ended
}

pub unsafe fn sctp_tsnmap_skip(map: *mut sctp_tsnmap, tsn: u32) {
    if TSN_lt(tsn, (*map).base_tsn) || !TSN_lt(tsn, (*map).base_tsn.wrapping_add(SCTP_TSN_MAP_SIZE)) { return; }
    if TSN_lt((*map).max_tsn_seen, tsn) { (*map).max_tsn_seen = tsn; }
    let gap = tsn.wrapping_sub((*map).base_tsn).wrapping_add(1);
    (*map).base_tsn = (*map).base_tsn.wrapping_add(gap); (*map).cumulative_tsn_ack_point = (*map).cumulative_tsn_ack_point.wrapping_add(gap);
    if gap >= (*map).len as u32 { bitmap_zero((*map).tsn_map, (*map).len); } else { bitmap_shift_right((*map).tsn_map, (*map).tsn_map, gap, (*map).len); sctp_tsnmap_update(map); }
}

unsafe fn sctp_tsnmap_update(map: *mut sctp_tsnmap) {
    let len = (*map).max_tsn_seen.wrapping_sub((*map).cumulative_tsn_ack_point) as u16;
    let zero_bit = find_first_zero_bit((*map).tsn_map, len);
    if zero_bit == 0 { return; }
    (*map).base_tsn = (*map).base_tsn.wrapping_add(zero_bit as u32); (*map).cumulative_tsn_ack_point = (*map).cumulative_tsn_ack_point.wrapping_add(zero_bit as u32);
    bitmap_shift_right((*map).tsn_map, (*map).tsn_map, zero_bit as u32, (*map).len);
}

pub unsafe fn sctp_tsnmap_pending(map: *mut sctp_tsnmap) -> u16 {
    let cum = (*map).cumulative_tsn_ack_point; let max = (*map).max_tsn_seen; let base = (*map).base_tsn;
    let mut pending = max.wrapping_sub(cum) as u16; let gap = max.wrapping_sub(base);
    if gap != 0 && gap < (*map).len as u32 { pending = pending.wrapping_sub(bitmap_weight((*map).tsn_map, gap + 1) as u16); } pending
}

unsafe fn sctp_tsnmap_find_gap_ack(map: *mut c_ulong, off: u16, len: u16, start: *mut u16, end: *mut u16) {
    let mut i = find_next_bit(map, len, off); if i < len as usize { *start = i as u16; }
    if *start != 0 { i = find_next_zero_bit(map, len, i); if i < len as usize { *end = (i - 1) as u16; } }
}

pub unsafe fn sctp_tsnmap_renege(map: *mut sctp_tsnmap, tsn: u32) {
    if TSN_lt(tsn, (*map).base_tsn) || !TSN_lt(tsn, (*map).base_tsn.wrapping_add((*map).len as u32)) { return; }
    clear_bit(tsn.wrapping_sub((*map).base_tsn), (*map).tsn_map);
}

pub unsafe fn sctp_tsnmap_num_gabs(map: *mut sctp_tsnmap, gabs: *mut sctp_gap_ack_block) -> u16 {
    let mut iter = sctp_tsnmap_iter { start: 0 }; let mut ngaps = 0u16;
    if sctp_tsnmap_has_gap(map) { sctp_tsnmap_iter_init(map, &mut iter); let (mut start, mut end) = (0u16, 0u16);
        while sctp_tsnmap_next_gap_ack(map, &mut iter, &mut start, &mut end) != 0 { (*gabs.add(ngaps as usize)).start = htons(start); (*gabs.add(ngaps as usize)).end = htons(end); ngaps += 1; if ngaps as i32 >= SCTP_MAX_GABS { break; } }
    } ngaps
}

unsafe fn sctp_tsnmap_grow(map: *mut sctp_tsnmap, size: u16) -> i32 {
    if size as u32 > SCTP_TSN_MAP_SIZE { return 0; }
    let inc = (((size - (*map).len) as usize + (usize::BITS as usize - 1)) / usize::BITS as usize) * usize::BITS as usize + SCTP_TSN_MAP_INCREMENT;
    let len = core::cmp::min((*map).len as usize + inc, SCTP_TSN_MAP_SIZE as usize) as u16;
    let new = kzalloc((len >> 3) as usize, GFP_ATOMIC) as *mut c_ulong; if new.is_null() { return 0; }
    bitmap_copy(new, (*map).tsn_map, (*map).max_tsn_seen.wrapping_sub((*map).cumulative_tsn_ack_point)); kfree((*map).tsn_map as *mut c_void); (*map).tsn_map = new; (*map).len = len; 1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
