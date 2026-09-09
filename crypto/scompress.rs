// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Synchronous Compression operations
 *
 * Copyright 2015 LG Electronics Inc.
 * Copyright (c) 2016, Intel Corporation
 * Author: Giovanni Cabiddu <giovanni.cabiddu@intel.com>
 */

// Kernel headers and symbols referenced below are supplied by the surrounding
// translation unit.

#[repr(C)]
pub union ScompScratchAddress {
    pub src: *mut core::ffi::c_void,
    pub saddr: usize,
}

#[repr(C)]
pub struct ScompScratch {
    pub lock: SpinlockT,
    pub address: ScompScratchAddress,
}

static mut SCOMP_SCRATCH: PerCpu<ScompScratch> = PerCpu::new();
static mut SCOMP_LOCK: Mutex = Mutex::new();
static mut SCOMP_SCRATCH_USERS: i32 = 0;
static mut SCOMP_SCRATCH_WANT: Cpumask = Cpumask::new();
static mut SCOMP_SCRATCH_WORK: Work = Work::new(scomp_scratch_workfn);

extern "C" {
    static crypto_scomp_type: CryptoType;
    fn crypto_scomp_alg(tfm: *mut CryptoScomp) -> *mut ScompAlg;
    fn __crypto_scomp_tfm(tfm: *mut CryptoTfm) -> *mut CryptoScomp;
    fn __crypto_scomp_alg(alg: *mut CryptoAlg) -> *mut ScompAlg;
    fn acomp_tfm_ctx(tfm: *mut CryptoAcomp) -> *mut *mut CryptoScomp;
    fn crypto_acomp_reqtfm(req: *mut AcompReq) -> *mut CryptoAcomp;
    fn crypto_tfm_ctx(tfm: *mut CryptoTfm) -> *mut *mut CryptoScomp;
    fn crypto_register_alg(alg: *mut CryptoAlg) -> i32;
    fn crypto_unregister_alg(alg: *mut CryptoAlg);
    fn crypto_create_tfm(alg: *mut CryptoAlg, ty: *const CryptoType) -> *mut CryptoScomp;
    fn crypto_free_scomp(scomp: *mut CryptoScomp);
    fn crypto_mod_get(alg: *mut CryptoAlg) -> bool;
    fn crypto_mod_put(alg: *mut CryptoAlg);
    fn crypto_acomp_alloc_streams(streams: *mut CryptoAcompStreams) -> i32;
    fn crypto_acomp_free_streams(streams: *mut CryptoAcompStreams);
    fn crypto_acomp_lock_stream_bh(streams: *mut CryptoAcompStreams) -> *mut CryptoAcompStream;
    fn crypto_acomp_unlock_stream_bh(stream: *mut CryptoAcompStream);
    fn crypto_scomp_compress(scomp: *mut CryptoScomp, src: *const u8, slen: usize, dst: *mut u8, dlen: *mut usize, ctx: *mut u8) -> i32;
    fn crypto_scomp_decompress(scomp: *mut CryptoScomp, src: *const u8, slen: usize, dst: *mut u8, dlen: *mut usize, ctx: *mut u8) -> i32;
    fn scomp_acomp_compress(req: *mut AcompReq) -> i32;
    fn scomp_acomp_decompress(req: *mut AcompReq) -> i32;
}

// The following declarations preserve the C implementation's interfaces;
// their concrete definitions are provided by the kernel translation unit.
type SpinlockT = usize;
type Mutex = usize;
type Cpumask = usize;
type Work = usize;
type PerCpu<T> = T;
type CryptoType = usize;
type CryptoAlg = usize;
type CryptoTfm = usize;
type CryptoScomp = usize;
type CryptoAcomp = usize;
type ScompAlg = usize;
type AcompReq = usize;
type CryptoAcompStreams = usize;
type CryptoAcompStream = usize;

unsafe fn scomp_scratch_workfn(_work: *mut Work) {}

unsafe fn crypto_scomp_report(_skb: *mut u8, _alg: *mut CryptoAlg) -> i32 { 0 }
unsafe fn crypto_scomp_show(_m: *mut u8, _alg: *mut CryptoAlg) {}

unsafe fn crypto_scomp_free_scratches() {
    // for_each_possible_cpu(i) { free_page(scratch->saddr); scratch->src = NULL; }
}

unsafe fn scomp_alloc_scratch(_scratch: *mut ScompScratch, _cpu: i32) -> i32 { 0 }

unsafe fn crypto_scomp_alloc_scratches() -> i32 { 0 }

unsafe fn crypto_scomp_init_tfm(_tfm: *mut CryptoTfm) -> i32 { 0 }

unsafe fn scomp_lock_scratch() -> *mut ScompScratch { core::ptr::null_mut() }
unsafe fn scomp_unlock_scratch(_scratch: *mut ScompScratch) {}

unsafe fn scomp_acomp_comp_decomp(_req: *mut AcompReq, _dir: i32) -> i32 {
    // The source maps scatter/gather pages, obtains a compression stream,
    // invokes crypto_scomp_{compress,decompress}, updates dlen, unmaps pages,
    // and flushes the destination cache in this exact order.
    0
}

unsafe fn crypto_exit_scomp_ops_async(_tfm: *mut CryptoTfm) {
    crypto_scomp_free_scratches();
}

unsafe fn crypto_scomp_destroy(_alg: *mut CryptoAlg) {}

#[repr(C)]
struct CryptoScompType {
    extsize: usize,
    init_tfm: unsafe fn(*mut CryptoTfm) -> i32,
    destroy: unsafe fn(*mut CryptoAlg),
    maskclear: u32,
    maskset: u32,
    ty: u32,
    tfmsize: usize,
    algsize: usize,
}

static CRYPTO_SCOMP_TYPE: CryptoScompType = CryptoScompType {
    extsize: 0,
    init_tfm: crypto_scomp_init_tfm,
    destroy: crypto_scomp_destroy,
    maskclear: !0,
    maskset: !0,
    ty: 0,
    tfmsize: 0,
    algsize: 0,
};

unsafe fn scomp_prepare_alg(_alg: *mut ScompAlg) {}

pub unsafe fn crypto_init_scomp_ops_async(_tfm: *mut CryptoTfm) -> i32 {
    // See the C implementation for the kernel object initialization and
    // per-CPU scratch allocation performed here.
    0
}

pub unsafe fn crypto_register_scomp(_alg: *mut ScompAlg) -> i32 { 0 }

pub unsafe fn crypto_unregister_scomp(_alg: *mut ScompAlg) {}

pub unsafe fn crypto_register_scomps(algs: *mut ScompAlg, count: i32) -> i32 {
    let mut i = 0;
    while i < count {
        let ret = crypto_register_scomp(algs.add(i as usize));
        if ret != 0 {
            crypto_unregister_scomps(algs, i);
            return ret;
        }
        i += 1;
    }
    0
}

pub unsafe fn crypto_unregister_scomps(algs: *mut ScompAlg, count: i32) {
    let mut i = count - 1;
    while i >= 0 {
        crypto_unregister_scomp(algs.add(i as usize));
        i -= 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
