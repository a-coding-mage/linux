// SPDX-License-Identifier: GPL-2.0-only
/*
 * asynchronous raid6 recovery self test
 * Copyright (c) 2009, Intel Corporation.
 *
 * based on drivers/md/raid6test/test.c:
 * 	Copyright 2002-2007 H. Peter Anvin
 */

// Dependencies supplied by the surrounding kernel/Rust bindings.
use core::ffi::{c_char, c_int, c_uint, c_void};

const NDISKS: usize = 64; /* Including P and Q */

extern "C" {
    static mut PAGE_SIZE: usize;
    static mut addr_conv: [addr_conv_t; NDISKS];
    fn page_address(page: *mut page) -> *mut c_void;
    fn get_random_bytes(buf: *mut c_void, len: usize);
    fn complete(cmp: *mut completion);
    fn init_async_submit(submit: *mut async_submit_ctl, flags: c_uint,
                         tx: *mut dma_async_tx_descriptor, cb: Option<unsafe extern "C" fn(*mut c_void)>,
                         cb_param: *mut c_void, scribble: *mut c_void,
                         addr_conv: *mut addr_conv_t);
    fn async_gen_syndrome(ptrs: *mut *mut page, offs: *mut c_uint, disks: c_int,
                          bytes: usize, submit: *mut async_submit_ctl) -> *mut dma_async_tx_descriptor;
    fn async_xor(dest: *mut page, blocks: *mut *mut page, offset: usize, count: c_int,
                 bytes: usize, submit: *mut async_submit_ctl) -> *mut dma_async_tx_descriptor;
    fn async_raid6_datap_recov(disks: c_int, bytes: usize, faila: c_int,
                               ptrs: *mut *mut page, offs: *mut c_uint,
                               submit: *mut async_submit_ctl) -> *mut dma_async_tx_descriptor;
    fn async_raid6_2data_recov(disks: c_int, bytes: usize, faila: c_int, failb: c_int,
                               ptrs: *mut *mut page, offs: *mut c_uint,
                               submit: *mut async_submit_ctl) -> *mut dma_async_tx_descriptor;
    fn async_syndrome_val(ptrs: *mut *mut page, offs: *mut c_uint, disks: c_int,
                          bytes: usize, result: *mut sum_check_flags, spare: *mut page,
                          offset: usize, submit: *mut async_submit_ctl) -> *mut dma_async_tx_descriptor;
    fn async_tx_issue_pending(tx: *mut dma_async_tx_descriptor);
    fn init_completion(cmp: *mut completion);
    fn wait_for_completion_timeout(cmp: *mut completion, timeout: c_ulong) -> c_ulong;
    fn msecs_to_jiffies(ms: c_uint) -> c_ulong;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int;
    fn alloc_page(gfp: c_uint) -> *mut page;
    fn put_page(page: *mut page);
    fn pr_info(fmt: *const c_char, ...);
}

type c_ulong = usize;
type addr_conv_t = usize;
type sum_check_flags = c_uint;
#[repr(C)] struct page { _private: [u8; 0] }
#[repr(C)] struct completion { _private: [u8; 0] }
#[repr(C)] struct dma_async_tx_descriptor { _private: [u8; 0] }
#[repr(C)] struct async_submit_ctl { _private: [u8; 0] }

const GFP_KERNEL: c_uint = 0;
const ASYNC_TX_XOR_ZERO_DST: c_uint = 1;
const ASYNC_TX_ACK: c_uint = 2;

static mut dataptrs: [*mut page; NDISKS] = [core::ptr::null_mut(); NDISKS];
static mut dataoffs: [c_uint; NDISKS] = [0; NDISKS];
static mut data: [*mut page; NDISKS + 3] = [core::ptr::null_mut(); NDISKS + 3];
static mut spare: *mut page = core::ptr::null_mut();
static mut recovi: *mut page = core::ptr::null_mut();
static mut recovj: *mut page = core::ptr::null_mut();

unsafe extern "C" fn callback(param: *mut c_void) { complete(param as *mut completion); }

unsafe fn makedata(disks: c_int) {
    for i in 0..disks as usize { get_random_bytes(page_address(data[i]), PAGE_SIZE); dataptrs[i] = data[i]; dataoffs[i] = 0; }
}

unsafe fn disk_type(d: c_int, disks: c_int) -> c_char {
    if d == disks - 2 { b'P' as c_char } else if d == disks - 1 { b'Q' as c_char } else { b'D' as c_char }
}

unsafe fn raid6_dual_recov(disks: c_int, bytes: usize, mut faila: c_int, mut failb: c_int,
                           ptrs: *mut *mut page, offs: *mut c_uint) {
    let mut submit: async_submit_ctl = core::mem::zeroed(); let mut cmp: completion = core::mem::zeroed();
    let mut tx: *mut dma_async_tx_descriptor = core::ptr::null_mut(); let mut result: sum_check_flags = !0;
    if faila > failb { core::mem::swap(&mut faila, &mut failb); }
    if failb == disks - 1 { if faila == disks - 2 {
        init_async_submit(&mut submit, 0, core::ptr::null_mut(), None, core::ptr::null_mut(), core::ptr::addr_of_mut!(addr_conv[0]));
        tx = async_gen_syndrome(ptrs, offs, disks, bytes, &mut submit);
    } else { let mut blocks = [core::ptr::null_mut(); NDISKS]; let mut count = 0; for i in (0..disks as usize).rev() { if i as c_int != faila && i as c_int != failb { blocks[count] = *ptrs.add(i); count += 1; } }
        init_async_submit(&mut submit, ASYNC_TX_XOR_ZERO_DST, core::ptr::null_mut(), None, core::ptr::null_mut(), core::ptr::addr_of_mut!(addr_conv[0])); tx = async_xor(*ptrs.add(faila as usize), blocks.as_mut_ptr(), 0, count as c_int, bytes, &mut submit);
        init_async_submit(&mut submit, 0, tx, None, core::ptr::null_mut(), core::ptr::addr_of_mut!(addr_conv[0])); tx = async_gen_syndrome(ptrs, offs, disks, bytes, &mut submit);
    }} else if failb == disks - 2 { init_async_submit(&mut submit, 0, core::ptr::null_mut(), None, core::ptr::null_mut(), core::ptr::addr_of_mut!(addr_conv[0])); tx = async_raid6_datap_recov(disks, bytes, faila, ptrs, offs, &mut submit); }
    else { init_async_submit(&mut submit, 0, core::ptr::null_mut(), None, core::ptr::null_mut(), core::ptr::addr_of_mut!(addr_conv[0])); tx = async_raid6_2data_recov(disks, bytes, faila, failb, ptrs, offs, &mut submit); }
    init_completion(&mut cmp); init_async_submit(&mut submit, ASYNC_TX_ACK, tx, Some(callback), &mut cmp as *mut _ as *mut c_void, core::ptr::addr_of_mut!(addr_conv[0]));
    tx = async_syndrome_val(ptrs, offs, disks, bytes, &mut result, spare, 0, &mut submit); async_tx_issue_pending(tx);
    let _ = wait_for_completion_timeout(&mut cmp, msecs_to_jiffies(3000));
}

unsafe fn test_disks(i: usize, j: usize, disks: c_int) -> c_int { memset(page_address(recovi), 0xf0, PAGE_SIZE); memset(page_address(recovj), 0xba, PAGE_SIZE); dataptrs[i] = recovi; dataptrs[j] = recovj; raid6_dual_recov(disks, PAGE_SIZE, i as c_int, j as c_int, dataptrs.as_mut_ptr(), dataoffs.as_mut_ptr()); let erra = memcmp(page_address(data[i]), page_address(recovi), PAGE_SIZE); let errb = memcmp(page_address(data[j]), page_address(recovj), PAGE_SIZE); dataptrs[i] = data[i]; dataptrs[j] = data[j]; (erra != 0 || errb != 0) as c_int }

unsafe fn test(disks: c_int, tests: *mut c_int) -> c_int { recovi=data[disks as usize]; recovj=data[disks as usize+1]; spare=data[disks as usize+2]; makedata(disks); memset(page_address(data[disks as usize-2]),0xee,PAGE_SIZE); memset(page_address(data[disks as usize-1]),0xee,PAGE_SIZE); let mut submit: async_submit_ctl=core::mem::zeroed(); let mut cmp: completion=core::mem::zeroed(); init_completion(&mut cmp); init_async_submit(&mut submit,ASYNC_TX_ACK,core::ptr::null_mut(),Some(callback),&mut cmp as *mut _ as *mut c_void,core::ptr::addr_of_mut!(addr_conv[0])); let tx=async_gen_syndrome(dataptrs.as_mut_ptr(),dataoffs.as_mut_ptr(),disks,PAGE_SIZE,&mut submit); async_tx_issue_pending(tx); if wait_for_completion_timeout(&mut cmp,msecs_to_jiffies(3000))==0{return 1} let mut err=0; for i in 0..disks-1 { for j in i+1..disks { *tests+=1; err+=test_disks(i as usize,j as usize,disks); }} err }

#[no_mangle] pub unsafe extern "C" fn raid6_test() -> c_int { let mut tests=0; let mut err=0; for i in 0..NDISKS+3 { data[i]=alloc_page(GFP_KERNEL); if data[i].is_null(){return -12;} } for &d in &[4,5,11,12,24,NDISKS as c_int] { if d as usize <= NDISKS { err+=test(d,&mut tests); }} for i in 0..NDISKS+3 {put_page(data[i]);} err }

pub unsafe extern "C" fn raid6_test_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
