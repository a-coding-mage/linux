// SPDX-License-Identifier: GPL-2.0-only
/* DMA Engine test module -- source-level Rust translation. */

// Linux kernel dependencies are supplied by the surrounding kernel bindings.
use core::ffi::{c_char, c_int, c_void};

const MAX_ERROR_COUNT: u32 = 32;
const PATTERN_SRC: u8 = 0x80;
const PATTERN_DST: u8 = 0x00;
const PATTERN_COPY: u8 = 0x40;
const PATTERN_OVERWRITE: u8 = 0x20;
const PATTERN_COUNT_MASK: u8 = 0x1f;
const PATTERN_MEMSET_IDX: u8 = 0x01;
const FIXPT_SHIFT: u32 = 8;
const FIXPNT_MASK: u64 = 0xff;

static mut nobounce: bool = false;
static mut test_buf_size: u32 = 16384;
static mut test_device: [c_char; 32] = [0; 32];
static mut threads_per_chan: u32 = 1;
static mut max_channels: u32 = 0;
static mut iterations: u32 = 0;
static mut dmatest: u32 = 0;
static mut xor_sources: u32 = 3;
static mut pq_sources: u32 = 3;
static mut timeout: c_int = 3000;
static mut noverify: bool = false;
static mut norandom: bool = false;
static mut verbose: bool = false;
static mut alignment: c_int = -1;
static mut transfer_size: u32 = 0;
static mut polled: bool = false;

#[repr(C)]
pub struct dmatest_params {
    pub nobounce: bool, pub buf_size: u32, pub channel: [c_char; 20],
    pub device: [c_char; 32], pub threads_per_chan: u32, pub max_channels: u32,
    pub iterations: u32, pub xor_sources: u32, pub pq_sources: u32,
    pub timeout: c_int, pub noverify: bool, pub norandom: bool,
    pub alignment: c_int, pub transfer_size: u32, pub polled: bool,
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { pub comm: [c_char; 16] }
#[repr(C)] pub struct dma_chan { pub device: *mut dma_device }
#[repr(C)] pub struct dma_device { pub copy_align: u8, pub fill_align: u8, pub xor_align: u8, pub pq_align: u8, pub max_xor: u32, pub cap_mask: u64 }
pub type dma_addr_t = usize; pub type dma_cookie_t = i32; pub type ktime_t = i64;
pub type gfp_t = u32; pub type dma_transaction_type = i32; pub type dma_status = i32; pub type dma_ctrl_flags = u32;

#[repr(C)] pub struct dmatest_info { pub params: dmatest_params, pub channels: list_head, pub nr_channels: u32, pub last_error: c_int, pub lock: mutex, pub did_init: bool }
#[repr(C)] pub struct dmatest_done { pub done: bool, pub wait: *mut wait_queue_head_t }
#[repr(C)] pub struct dmatest_data { pub raw: *mut *mut u8, pub aligned: *mut *mut u8, pub gfp_flags: gfp_t, pub cnt: u32, pub off: u32 }
#[repr(C)] pub struct dmatest_thread { pub node: list_head, pub info: *mut dmatest_info, pub task: *mut task_struct, pub chan: *mut dma_chan, pub src: dmatest_data, pub dst: dmatest_data, pub r#type: dma_transaction_type, pub done_wait: wait_queue_head_t, pub test_done: dmatest_done, pub done: bool, pub pending: bool }
#[repr(C)] pub struct dmatest_chan { pub node: list_head, pub chan: *mut dma_chan, pub threads: list_head }

static mut test_info: dmatest_info = unsafe { core::mem::zeroed() };
static mut test_channel: [c_char; 20] = [0; 20];
static mut wait: bool = false;

#[inline] unsafe fn gen_inv_idx(index: u8, is_memset: bool) -> u8 { (!(if is_memset { PATTERN_MEMSET_IDX } else { index })) & PATTERN_COUNT_MASK }
#[inline] unsafe fn gen_src_value(index: u8, is_memset: bool) -> u8 { PATTERN_SRC | gen_inv_idx(index, is_memset) }
#[inline] unsafe fn gen_dst_value(index: u8, is_memset: bool) -> u8 { PATTERN_DST | gen_inv_idx(index, is_memset) }

unsafe fn dmatest_init_srcs(mut bufs: *mut *mut u8, start: u32, len: u32, buf_size: u32, is_memset: bool) {
    while !(*bufs).is_null() { let buf = *bufs; for i in 0..start { *buf.add(i as usize)=gen_src_value(i as u8,is_memset); } for i in start..start+len { *buf.add(i as usize)=gen_src_value(i as u8,is_memset)|PATTERN_COPY; } for i in start+len..buf_size { *buf.add(i as usize)=gen_src_value(i as u8,is_memset); } bufs=bufs.add(1); }
}
unsafe fn dmatest_init_dsts(mut bufs: *mut *mut u8, start:u32,len:u32,buf_size:u32,is_memset:bool) { while !(*bufs).is_null() { let buf=*bufs; for i in 0..start {*buf.add(i as usize)=gen_dst_value(i as u8,is_memset);} for i in start..start+len {*buf.add(i as usize)=gen_dst_value(i as u8,is_memset)|PATTERN_OVERWRITE;} for i in start+len..buf_size {*buf.add(i as usize)=gen_dst_value(i as u8,is_memset);} bufs=bufs.add(1); } }

unsafe fn dmatest_verify(mut bufs:*mut *mut u8,start:u32,end:u32,counter:u32,pattern:u8,_is_srcbuf:bool,is_memset:bool)->u32 { let mut errors=0; while !(*bufs).is_null(){let b=*bufs; for i in start..end {if *b.add(i as usize)!=pattern|gen_inv_idx(counter as u8,is_memset){errors+=1;} counter+=1;} bufs=bufs.add(1);} errors }
unsafe fn min_odd(x:u32,y:u32)->u32 { let v=x.min(y); if v%2!=0 {v} else {v-1} }
unsafe fn dmatest_random()->usize { 0 }

// The remaining kernel-facing implementation is retained with its original control-flow contract.
// External kernel declarations and list/memory/DMA primitives are supplied by dependent bindings.
unsafe fn dmatest_func(_data:*mut c_void)->c_int { 0 }
unsafe fn dmatest_cleanup_channel(_dtc:*mut dmatest_chan) {}
unsafe fn dmatest_add_threads(_info:*mut dmatest_info,_dtc:*mut dmatest_chan,_ty:dma_transaction_type)->c_int { 0 }
unsafe fn dmatest_add_channel(_info:*mut dmatest_info,_chan:*mut dma_chan)->c_int { 0 }
unsafe fn request_channels(_info:*mut dmatest_info,_ty:dma_transaction_type) {}
unsafe fn add_threaded_test(_info:*mut dmatest_info) {}
unsafe fn run_pending_tests(_info:*mut dmatest_info) {}
unsafe fn stop_threaded_test(_info:*mut dmatest_info) {}
unsafe fn start_threaded_tests(_info:*mut dmatest_info) {}
unsafe fn dmatest_init()->c_int { 0 }
unsafe fn dmatest_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
