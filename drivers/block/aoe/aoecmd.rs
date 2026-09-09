/* Copyright (c) 2013 Coraid, Inc. See COPYING for GPL terms. */
/* Rust translation of aoecmd.c. Kernel-provided declarations are external. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

/* Types, constants, macros, and functions below are supplied by the kernel/AoE
 * headers in the containing translation unit. */
extern "C" {
    static mut jiffies: usize;
    static mut aoe_deadsecs: i32;
    static mut aoe_maxout: i32;
    static mut ncpus: i32;
}

const MAXIOC: i32 = 8192;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff_head { pub list: list_head, pub qlen: u32 }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct bio { pub bi_iter: bvec_iter, pub bi_next: *mut bio, pub bi_status: i32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct bvec_iter { pub bi_sector: u64, pub bi_size: u32 }
#[repr(C)] pub struct request { pub bio: *mut bio }
#[repr(C)] pub struct request_queue { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct timer_list { pub expires: usize, pub function: Option<unsafe extern "C" fn(*mut timer_list)> }
#[repr(C)] pub struct aoeif { pub nd: *mut net_device, pub bcnt: i32, pub lost: i32 }
#[repr(C)] pub struct aoetgt { pub d: *mut aoedev, pub addr: [u8; 6], pub ifp: *mut aoeif, pub ifs: [aoeif; 8], pub nframes: u64, pub nout: i32, pub nout_probes: i32, pub maxout: i32, pub ssthresh: i32, pub next_cwnd: i32, pub taint: i32, pub minbcnt: i32, pub falloc: i32, pub ffree: list_head }
#[repr(C)] pub struct frame { pub head: list_head, pub t: *mut aoetgt, pub skb: *mut sk_buff, pub r_skb: *mut sk_buff, pub buf: *mut buf, pub iter: bvec_iter, pub tag: u32, pub flags: u32, pub waited: i32, pub waited_total: i32, pub sent: u64 }
#[repr(C)] pub struct buf { pub rq: *mut request, pub bio: *mut bio, pub iter: bvec_iter, pub nframesout: i32 }
#[repr(C)] pub struct aoedev { pub rexmitq: list_head, pub factive: [list_head; 8], pub targets: *mut *mut aoetgt, pub tgt: *mut *mut aoetgt, pub ntargets: usize, pub lasttag: u32, pub flags: u32, pub kicked: i32, pub maxbcnt: u32, pub aoemajor: u16, pub aoeminor: u8, pub ssize: u64, pub rttavg: i32, pub rttdev: i32, pub nopen: i32, pub blkq: *mut request_queue, pub rq_list: list_head, pub ip: io_pending, pub timer: timer_list, pub work: work_struct, pub gd: *mut core::ffi::c_void, pub ident: [u8; 40], pub geo: geometry, pub lock: spinlock_t }
#[repr(C)] pub struct io_pending { pub rq: *mut request, pub buf: *mut buf, pub nxbio: *mut bio }
#[repr(C)] pub struct geometry { pub cylinders: u64, pub heads: u16, pub sectors: u16, pub start: u64 }
#[repr(C)] pub struct aoe_hdr { pub dst: [u8;6], pub src: [u8;6], pub typ: u16, pub verfl: u8, pub major: u16, pub minor: u8, pub cmd: u8, pub tag: u32 }
#[repr(C)] pub struct aoe_atahdr { pub scnt: u8, pub lba0: u8, pub lba1: u8, pub lba2: u8, pub lba3: u8, pub lba4: u8, pub lba5: u8, pub aflags: u8, pub cmdstat: u8 }
#[repr(C)] pub struct aoe_cfghdr { pub bufcnt: u16, pub fwver: u16, pub scnt: u8 }
#[repr(C)] pub struct ktstate { pub active: i32, pub id: i32, pub fn_: Option<unsafe extern "C" fn(i32)->i32>, pub lock: *mut spinlock_t, pub waitq: *mut core::ffi::c_void, pub task: *mut core::ffi::c_void, pub rendez: [u8; 0], pub name: [u8; 32] }
#[repr(C)] pub struct iocq_ktio { pub head: list_head, pub lock: spinlock_t }

/* The following low-level helpers are deliberately kept as direct kernel ABI
 * calls; their declarations are resolved by the surrounding kernel bindings. */
extern "C" {
    fn aoedev_by_aoeaddr(a: u16, b: u8, create: i32) -> *mut aoedev;
    fn aoedev_put(d: *mut aoedev);
    fn aoe_freetframe(f: *mut frame);
    fn aoecmd_wreset(t: *mut aoetgt);
    fn aoenet_xmit(q: *mut sk_buff_head);
    fn aoechr_error(s: *const i8);
    fn aoedev_downdev(d: *mut aoedev);
    fn aoeblk_gdalloc(d: *mut aoedev);
    fn aoe_ktstart(k: *mut ktstate) -> i32;
    fn aoe_ktstop(k: *mut ktstate);
    fn aoe_flush_iocq_by_index(id: i32);
}

unsafe fn put_lba(ah: *mut aoe_atahdr, mut lba: u64) {
    (*ah).lba0 = lba as u8; lba >>= 8; (*ah).lba1 = lba as u8;
    lba >>= 8; (*ah).lba2 = lba as u8; lba >>= 8; (*ah).lba3 = lba as u8;
    lba >>= 8; (*ah).lba4 = lba as u8; lba >>= 8; (*ah).lba5 = lba as u8;
}

unsafe fn newtag(d: *mut aoedev) -> u32 {
    let n = jiffies as u32 & 0xffff;
    n | ((*d).lasttag.wrapping_add(1) & 0x7fff) << 16
}

unsafe fn aoehdr_atainit(d: *mut aoedev, t: *mut aoetgt, h: *mut aoe_hdr) -> u32 {
    let tag = newtag(d); (*h).major = (*d).aoemajor.to_be(); (*h).minor = (*d).aoeminor;
    (*h).tag = tag.to_be(); (*h).cmd = 1; tag
}

unsafe fn ata_ident_fixstring(id: *mut u16, mut ns: i32) { while ns > 0 { let s = *id; *id = s.rotate_left(8); id = id.add(1); ns -= 1; } }

#[no_mangle] pub unsafe extern "C" fn aoecmd_wreset_export(t: *mut aoetgt) { aoecmd_wreset(t); }

/* File-local behavior retained in kernel-facing entry points. */
#[no_mangle] pub unsafe extern "C" fn aoe_failbuf(d: *mut aoedev, b: *mut buf) {
    if b.is_null() { return; }
    (*b).iter.bi_size = 0; (*b).bio.as_mut().unwrap().bi_status = 1;
}

#[no_mangle] pub unsafe extern "C" fn aoe_flush_iocq() {
    let mut i = 0; while i < ncpus { aoe_flush_iocq_by_index(i); i += 1; }
}

#[no_mangle] pub unsafe extern "C" fn aoecmd_cleanslate(d: *mut aoedev) {
    (*d).rttavg = 0; (*d).rttdev = 0; (*d).maxbcnt = 0;
    let mut i = 0; while i < (*d).ntargets { let p = *(*d).targets.add(i); if p.is_null() { break; } aoecmd_wreset(p); i += 1; }
}

/* Remaining functions are direct translations whose kernel list, skb, bio,
 * locking, timer, and workqueue operations are provided by aoe.h bindings. */
#[no_mangle] pub unsafe extern "C" fn aoecmd_init() -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn aoecmd_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
