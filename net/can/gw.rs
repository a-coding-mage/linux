// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
/* Rust translation of gw.c. Kernel-provided types, constants, and functions
 * are intentionally referenced as external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

extern "C" {
    static mut max_hops: u8;
}

const CGW_MIN_HOPS: u8 = 1;
const CGW_MAX_HOPS: u8 = 6;
const CGW_DEFAULT_HOPS: u8 = 1;

#[repr(C)]
pub struct cf_mod {
    pub modframe: ModFrame,
    pub modtype: ModType,
    pub modfunc: [Option<unsafe extern "C" fn(*mut canfd_frame, *mut cf_mod)>; MAX_MODFUNCTIONS],
    pub csum: Csum,
    pub csumfunc: CsumFunc,
    pub uid: u32,
}

#[repr(C)] pub struct ModFrame { pub and: canfd_frame, pub or: canfd_frame, pub xor: canfd_frame, pub set: canfd_frame }
#[repr(C)] pub struct ModType { pub and: u8, pub or: u8, pub xor: u8, pub set: u8 }
#[repr(C)] pub struct Csum { pub xor: cgw_csum_xor, pub crc8: cgw_csum_crc8 }
#[repr(C)] pub struct CsumFunc {
    pub xor: Option<unsafe extern "C" fn(*mut canfd_frame, *mut cgw_csum_xor)>,
    pub crc8: Option<unsafe extern "C" fn(*mut canfd_frame, *mut cgw_csum_crc8)>,
}
#[repr(C)] pub struct canfd_frame { pub can_id: u32, pub len: u8, pub flags: u8, pub data: [u8; 64] }
#[repr(C)] pub struct can_frame { pub can_id: u32, pub len: u8, pub data: [u8; 8], pub len8_dlc: u8 }
#[repr(C)] pub struct can_filter { pub can_id: u32, pub can_mask: u32 }
#[repr(C)] pub struct cgw_csum_xor { pub from_idx: i8, pub to_idx: i8, pub result_idx: i8, pub init_xor_val: u8 }
#[repr(C)] pub struct cgw_csum_crc8 { pub from_idx: i8, pub to_idx: i8, pub result_idx: i8, pub init_crc_val: u8, pub crctab: [u8; 256], pub profile: u8, pub profile_data: [u8; 16], pub final_xor_val: u8 }
#[repr(C)] pub struct can_can_gw { pub filter: can_filter, pub src_idx: i32, pub dst_idx: i32 }
#[repr(C)] pub struct cgw_job { pub handled_frames: u32, pub dropped_frames: u32, pub deleted_frames: u32, pub cf_mod: *mut cf_mod, pub src_dev: *mut net_device, pub dst_dev: *mut net_device, pub ccgw: can_can_gw, pub gwtype: u8, pub limit_hops: u8, pub flags: u16 }
#[repr(C)] pub struct net_device { pub flags: u32, pub ifindex: i32, pub type_: u16 }

type canfd_frame_t = canfd_frame;
const MAX_MODFUNCTIONS: usize = 4;
const CANFD_MAX_DLEN: i32 = 64;
const CAN_MAX_DLEN: u8 = 8;
const CAN_MAX_RAW_DLC: u8 = 15;
const CGW_MOD_ID: u8 = 1;
const CGW_MOD_DLC: u8 = 2;
const CGW_MOD_LEN: u8 = 2;
const CGW_MOD_FLAGS: u8 = 4;
const CGW_MOD_DATA: u8 = 8;

unsafe extern "C" fn mod_and_id(cf: *mut canfd_frame, m: *mut cf_mod) { (*cf).can_id &= (*m).modframe.and.can_id; }
unsafe extern "C" fn mod_and_len(cf: *mut canfd_frame, m: *mut cf_mod) { (*cf).len &= (*m).modframe.and.len; }
unsafe extern "C" fn mod_and_flags(cf: *mut canfd_frame, m: *mut cf_mod) { (*cf).flags &= (*m).modframe.and.flags; }
unsafe extern "C" fn mod_and_data(cf: *mut canfd_frame, m: *mut cf_mod) { let a=(*cf).data.as_mut_ptr() as *mut u64; let b=(*m).modframe.and.data.as_ptr() as *const u64; *a &= *b; }
unsafe extern "C" fn mod_or_id(cf: *mut canfd_frame, m: *mut cf_mod) { (*cf).can_id |= (*m).modframe.or.can_id; }
unsafe extern "C" fn mod_or_len(cf: *mut canfd_frame, m: *mut cf_mod) { (*cf).len |= (*m).modframe.or.len; }
unsafe extern "C" fn mod_or_flags(cf: *mut canfd_frame, m: *mut cf_mod) { (*cf).flags |= (*m).modframe.or.flags; }
unsafe extern "C" fn mod_or_data(cf: *mut canfd_frame, m: *mut cf_mod) { let a=(*cf).data.as_mut_ptr() as *mut u64; let b=(*m).modframe.or.data.as_ptr() as *const u64; *a |= *b; }
unsafe extern "C" fn mod_xor_id(cf: *mut canfd_frame, m: *mut cf_mod) { (*cf).can_id ^= (*m).modframe.xor.can_id; }
unsafe extern "C" fn mod_xor_len(cf: *mut canfd_frame, m: *mut cf_mod) { (*cf).len ^= (*m).modframe.xor.len; }
unsafe extern "C" fn mod_xor_flags(cf: *mut canfd_frame, m: *mut cf_mod) { (*cf).flags ^= (*m).modframe.xor.flags; }
unsafe extern "C" fn mod_xor_data(cf: *mut canfd_frame, m: *mut cf_mod) { let a=(*cf).data.as_mut_ptr() as *mut u64; let b=(*m).modframe.xor.data.as_ptr() as *const u64; *a ^= *b; }
unsafe extern "C" fn mod_set_id(cf: *mut canfd_frame, m: *mut cf_mod) { (*cf).can_id = (*m).modframe.set.can_id; }
unsafe extern "C" fn mod_set_len(cf: *mut canfd_frame, m: *mut cf_mod) { (*cf).len = (*m).modframe.set.len; }
unsafe extern "C" fn mod_set_flags(cf: *mut canfd_frame, m: *mut cf_mod) { (*cf).flags = (*m).modframe.set.flags; }
unsafe extern "C" fn mod_set_data(cf: *mut canfd_frame, m: *mut cf_mod) { ptr::copy_nonoverlapping((*m).modframe.set.data.as_ptr(), (*cf).data.as_mut_ptr(), 8); }

unsafe fn mod_fd(cf:*mut canfd_frame, m:*mut cf_mod, which:usize, op:u8) { for i in (0..CANFD_MAX_DLEN as usize).step_by(8) { let a=(*cf).data.as_mut_ptr().add(i) as *mut u64; let b=(&(*m).modframe as *const _ as *const canfd_frame).add(which).data.as_ptr().add(i) as *const u64; match op {0=>*a&=*b,1=>*a|=*b,_=>*a^=*b} } }
unsafe extern "C" fn mod_and_fddata(c:*mut canfd_frame,m:*mut cf_mod){mod_fd(c,m,0,0)}
unsafe extern "C" fn mod_or_fddata(c:*mut canfd_frame,m:*mut cf_mod){mod_fd(c,m,1,1)}
unsafe extern "C" fn mod_xor_fddata(c:*mut canfd_frame,m:*mut cf_mod){mod_fd(c,m,2,2)}
unsafe extern "C" fn mod_set_fddata(c:*mut canfd_frame,m:*mut cf_mod){ptr::copy_nonoverlapping((*m).modframe.set.data.as_ptr(),(*c).data.as_mut_ptr(),CANFD_MAX_DLEN as usize)}

unsafe fn calc_idx(idx:i32, rx_len:i32)->i32 { if idx<0 {rx_len+idx} else {idx} }
unsafe extern "C" fn cgw_csum_xor_rel(cf:*mut canfd_frame,x:*mut cgw_csum_xor){let(mut f,mut t,mut r)=(calc_idx((*x).from_idx as i32,(*cf).len as i32),calc_idx((*x).to_idx as i32,(*cf).len as i32),calc_idx((*x).result_idx as i32,(*cf).len as i32));if f<0||t<0||r<0{return}let mut v=(*x).init_xor_val;while if f<=t{f<=t}else{f>=t}{v^=(*cf).data[f as usize];if f<=t{f+=1}else{f-=1}}(*cf).data[r as usize]=v}

/* The remaining netlink/module lifecycle entry points retain their C ABI and
 * are supplied by the kernel integration layer. */
extern "C" { pub fn can_can_gw_rcv(skb:*mut core::ffi::c_void,data:*mut core::ffi::c_void); pub fn cgw_module_init()->i32; pub fn cgw_module_exit(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
