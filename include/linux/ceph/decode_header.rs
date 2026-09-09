/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies are supplied by the surrounding kernel translation.

/* In all cases, `p` points to the current position and `end` is one past the buffer. */

#[inline]
pub unsafe fn ceph_decode_64(p: *mut *mut core::ffi::c_void) -> u64 {
    let v = u64::from_le_bytes(core::slice::from_raw_parts(*p as *const u8, 8).try_into().unwrap());
    *p = (*p).add(8);
    v
}
#[inline]
pub unsafe fn ceph_decode_32(p: *mut *mut core::ffi::c_void) -> u32 {
    let v = u32::from_le_bytes(core::slice::from_raw_parts(*p as *const u8, 4).try_into().unwrap());
    *p = (*p).add(4);
    v
}
#[inline]
pub unsafe fn ceph_decode_16(p: *mut *mut core::ffi::c_void) -> u16 {
    let v = u16::from_le_bytes(core::slice::from_raw_parts(*p as *const u8, 2).try_into().unwrap());
    *p = (*p).add(2);
    v
}
#[inline]
pub unsafe fn ceph_decode_8(p: *mut *mut core::ffi::c_void) -> u8 {
    let v = *(*p as *const u8);
    *p = (*p).add(1);
    v
}
#[inline]
pub unsafe fn ceph_decode_copy(p: *mut *mut core::ffi::c_void, pv: *mut core::ffi::c_void, n: usize) {
    core::ptr::copy_nonoverlapping(*p as *const u8, pv as *mut u8, n);
    *p = (*p).add(n);
}

#[inline]
pub unsafe fn ceph_has_room(p: *const *mut core::ffi::c_void, end: *mut core::ffi::c_void, n: usize) -> bool {
    (end as usize) >= (*p as usize) && n <= (end as usize).wrapping_sub(*p as usize)
}

// The C `goto bad` argument is represented by the caller's control flow.
#[macro_export]
macro_rules! ceph_decode_need { ($p:expr, $end:expr, $n:expr, $bad:tt) => { if !unsafe { $crate::ceph_has_room($p, $end, $n) } { break $bad; } }; }
#[macro_export]
macro_rules! ceph_decode_64_safe { ($p:expr,$end:expr,$v:expr,$bad:tt) => {{ ceph_decode_need!($p,$end,8,$bad); $v=unsafe{ceph_decode_64($p)}; }}; }
#[macro_export]
macro_rules! ceph_decode_32_safe { ($p:expr,$end:expr,$v:expr,$bad:tt) => {{ ceph_decode_need!($p,$end,4,$bad); $v=unsafe{ceph_decode_32($p)}; }}; }
#[macro_export]
macro_rules! ceph_decode_16_safe { ($p:expr,$end:expr,$v:expr,$bad:tt) => {{ ceph_decode_need!($p,$end,2,$bad); $v=unsafe{ceph_decode_16($p)}; }}; }
#[macro_export]
macro_rules! ceph_decode_8_safe { ($p:expr,$end:expr,$v:expr,$bad:tt) => {{ ceph_decode_need!($p,$end,1,$bad); $v=unsafe{ceph_decode_8($p)}; }}; }
#[macro_export]
macro_rules! ceph_decode_copy_safe { ($p:expr,$end:expr,$pv:expr,$n:expr,$bad:tt) => {{ ceph_decode_need!($p,$end,$n,$bad); unsafe{ceph_decode_copy($p,$pv,$n)}; }}; }

/* Allocation and error-pointer behavior are supplied by the kernel environment. */
pub unsafe fn ceph_extract_encoded_string(_p: *mut *mut core::ffi::c_void, _end: *mut core::ffi::c_void, _lenp: *mut usize, _gfp: usize) -> *mut i8 {
    unimplemented!("requires kernel kmalloc/ERR_PTR")
}

// Skip helpers retain the C macro interface and caller-provided failure label.
#[macro_export] macro_rules! ceph_decode_skip_n { ($p:expr,$end:expr,$n:expr,$bad:tt) => {{ ceph_decode_need!($p,$end,$n,$bad); unsafe{$p.write((*$p).add($n));} }}; }
#[macro_export] macro_rules! ceph_decode_skip_64 { ($p:expr,$end:expr,$bad:tt) => { ceph_decode_skip_n!($p,$end,8,$bad) }; }
#[macro_export] macro_rules! ceph_decode_skip_32 { ($p:expr,$end:expr,$bad:tt) => { ceph_decode_skip_n!($p,$end,4,$bad) }; }
#[macro_export] macro_rules! ceph_decode_skip_16 { ($p:expr,$end:expr,$bad:tt) => { ceph_decode_skip_n!($p,$end,2,$bad) }; }
#[macro_export] macro_rules! ceph_decode_skip_8 { ($p:expr,$end:expr,$bad:tt) => { ceph_decode_skip_n!($p,$end,1,$bad) }; }

#[repr(C)]
pub struct ceph_timespec { pub tv_sec: u32, pub tv_nsec: u32 }
#[inline] pub unsafe fn ceph_decode_timespec64(ts: *mut timespec64, tv: *const ceph_timespec) { (*ts).tv_sec=(*tv).tv_sec as i64; (*ts).tv_nsec=(*tv).tv_nsec as i64; }
#[inline] pub unsafe fn ceph_encode_timespec64(tv: *mut ceph_timespec, ts: *const timespec64) { (*tv).tv_sec=(*ts).tv_sec as u32; (*tv).tv_nsec=(*ts).tv_nsec as u32; }

pub const CEPH_ENTITY_ADDR_TYPE_NONE: u32 = 0;
pub const CEPH_ENTITY_ADDR_TYPE_LEGACY: u32 = 1;
pub const CEPH_ENTITY_ADDR_TYPE_MSGR2: u32 = 2;
pub const CEPH_ENTITY_ADDR_TYPE_ANY: u32 = 3;

extern "C" { pub fn ceph_decode_entity_addr(p: *mut *mut core::ffi::c_void, end: *mut core::ffi::c_void, addr: *mut ceph_entity_addr) -> i32; pub fn ceph_decode_entity_addrvec(p:*mut *mut core::ffi::c_void,end:*mut core::ffi::c_void,msgr2:bool,addr:*mut ceph_entity_addr)->i32; pub fn ceph_entity_addr_encoding_len(addr:*const ceph_entity_addr)->i32; pub fn ceph_encode_entity_addr(p:*mut *mut core::ffi::c_void,addr:*const ceph_entity_addr); }

#[inline] pub unsafe fn ceph_encode_64(p:*mut *mut core::ffi::c_void,v:u64){(*p as *mut u64).write_unaligned(v.to_le());*p=(*p).add(8)}
#[inline] pub unsafe fn ceph_encode_32(p:*mut *mut core::ffi::c_void,v:u32){(*p as *mut u32).write_unaligned(v.to_le());*p=(*p).add(4)}
#[inline] pub unsafe fn ceph_encode_16(p:*mut *mut core::ffi::c_void,v:u16){(*p as *mut u16).write_unaligned(v.to_le());*p=(*p).add(2)}
#[inline] pub unsafe fn ceph_encode_8(p:*mut *mut core::ffi::c_void,v:u8){(*p as *mut u8).write(v);*p=(*p).add(1)}
#[inline] pub unsafe fn ceph_encode_copy(p:*mut *mut core::ffi::c_void,s:*const core::ffi::c_void,len:usize){core::ptr::copy_nonoverlapping(s as *const u8,*p as *mut u8,len);*p=(*p).add(len)}

pub const CEPH_ENCODING_START_BLK_LEN: usize = 6;
#[inline] pub unsafe fn ceph_start_encoding(p:*mut *mut core::ffi::c_void,struct_v:u8,struct_compat:u8,struct_len:u32){ceph_encode_8(p,struct_v);ceph_encode_8(p,struct_compat);ceph_encode_32(p,struct_len)}
#[inline] pub unsafe fn ceph_encode_filepath(p:*mut *mut core::ffi::c_void,end:*mut core::ffi::c_void,ino:u64,path:*const i8){let len=if path.is_null(){0}else{libc_strlen(path)};assert!((*p as usize).wrapping_add(1+8+4+len)<=end as usize);ceph_encode_8(p,1);ceph_encode_64(p,ino);ceph_encode_32(p,len as u32);if len!=0{ceph_encode_copy(p,path as *const _,len)}}
#[inline] pub unsafe fn ceph_encode_string(p:*mut *mut core::ffi::c_void,end:*mut core::ffi::c_void,s:*const i8,len:u32){assert!((*p as usize).wrapping_add(4+len as usize)<=end as usize);ceph_encode_32(p,len);if len!=0{ceph_encode_copy(p,s as *const _,len as usize)}}
extern "C" { fn libc_strlen(s:*const i8)->usize; }
#[inline] pub unsafe fn ceph_start_decoding(p:*mut *mut core::ffi::c_void,end:*mut core::ffi::c_void,v:u8,_name:*const i8,struct_v:*mut u8,struct_len:*mut u32)->i32 { if !ceph_has_room(p,end,6){return -34};*struct_v=ceph_decode_8(p);let compat=ceph_decode_8(p);if v<compat{return -22};*struct_len=ceph_decode_32(p);if !ceph_has_room(p,end,*struct_len as usize){return -34};0 }
extern "C" { pub fn pr_warn(fmt:*const i8,...); }

#[macro_export] macro_rules! ceph_encode_need { ($p:expr,$end:expr,$n:expr,$bad:tt) => { if !unsafe{$crate::ceph_has_room($p,$end,$n)} { break $bad; } }; }
#[macro_export] macro_rules! ceph_encode_64_safe { ($p:expr,$end:expr,$v:expr,$bad:tt)=>{{ceph_encode_need!($p,$end,8,$bad);unsafe{ceph_encode_64($p,$v)}}}; }
#[macro_export] macro_rules! ceph_encode_32_safe { ($p:expr,$end:expr,$v:expr,$bad:tt)=>{{ceph_encode_need!($p,$end,4,$bad);unsafe{ceph_encode_32($p,$v)}}}; }
#[macro_export] macro_rules! ceph_encode_16_safe { ($p:expr,$end:expr,$v:expr,$bad:tt)=>{{ceph_encode_need!($p,$end,2,$bad);unsafe{ceph_encode_16($p,$v)}}}; }
#[macro_export] macro_rules! ceph_encode_8_safe { ($p:expr,$end:expr,$v:expr,$bad:tt)=>{{ceph_encode_need!($p,$end,1,$bad);unsafe{ceph_encode_8($p,$v)}}}; }
#[macro_export] macro_rules! ceph_encode_copy_safe { ($p:expr,$end:expr,$pv:expr,$n:expr,$bad:tt)=>{{ceph_encode_need!($p,$end,$n,$bad);unsafe{ceph_encode_copy($p,$pv,$n)}}}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
