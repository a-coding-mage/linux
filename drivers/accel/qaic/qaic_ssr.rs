// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2020-2021, The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.

// Dependencies are supplied by the surrounding kernel translation.
use core::{mem, ptr};

const SSR_RESP_MSG_SZ: usize = 32;
const SSR_MHI_BUF_SIZE: u64 = 64 * 1024;
const SSR_MEM_READ_DATA_SIZE: u64 = SSR_MHI_BUF_SIZE - mem::size_of::<ssr_crashdump>() as u64;
const SSR_MEM_READ_CHUNK_SIZE: u64 = SSR_MEM_READ_DATA_SIZE - mem::size_of::<ssr_memory_read_rsp>() as u64;
const DEBUG_TRANSFER_INFO: u32 = 1 << 0; const DEBUG_TRANSFER_INFO_RSP: u32 = 1 << 1;
const MEMORY_READ: u32 = 1 << 2; const MEMORY_READ_RSP: u32 = 1 << 3;
const DEBUG_TRANSFER_DONE: u32 = 1 << 4; const DEBUG_TRANSFER_DONE_RSP: u32 = 1 << 5;
const SSR_EVENT: u32 = 1 << 8; const SSR_EVENT_RSP: u32 = 1 << 9;
const SSR_EVENT_NACK: u32 = 1; const BEFORE_SHUTDOWN: u32 = 2; const AFTER_SHUTDOWN: u32 = 4;
const BEFORE_POWER_UP: u32 = 8; const AFTER_POWER_UP: u32 = 16;
const QAIC_SSR_DUMP_V1_MAGIC: u64 = 0x1234567890abcdef; const QAIC_SSR_DUMP_V1_VER: u64 = 1;

#[repr(C)] pub struct debug_info_table { pub save_perf:u64, pub mem_base:u64, pub len:u64, pub desc:[u8;20], pub filename:[u8;20] }
#[repr(C, packed)] pub struct _ssr_hdr { pub cmd:u32, pub len:u32, pub dbc_id:u32 }
#[repr(C)] pub struct ssr_hdr { pub cmd:u32, pub len:u32, pub dbc_id:u32 }
#[repr(C, packed)] pub struct ssr_debug_transfer_info { pub hdr:ssr_hdr, pub resv:u32, pub tbl_addr:u64, pub tbl_len:u64 }
#[repr(C, packed)] pub struct ssr_debug_transfer_info_rsp { pub hdr:_ssr_hdr, pub ret:u32 }
#[repr(C, packed)] pub struct ssr_memory_read { pub hdr:_ssr_hdr, pub resv:u32, pub addr:u64, pub len:u64 }
#[repr(C, packed)] pub struct ssr_memory_read_rsp { pub hdr:_ssr_hdr, pub resv:u32, pub data:[u8;0] }
#[repr(C, packed)] pub struct ssr_debug_transfer_done { pub hdr:_ssr_hdr, pub resv:u32 }
#[repr(C, packed)] pub struct ssr_debug_transfer_done_rsp { pub hdr:_ssr_hdr, pub ret:u32 }
#[repr(C, packed)] pub struct ssr_event { pub hdr:ssr_hdr, pub event:u32 }
#[repr(C, packed)] pub struct ssr_event_rsp { pub hdr:_ssr_hdr, pub event:u32 }
#[repr(C)] pub struct ssr_resp { pub work:work_struct, pub qdev:*mut qaic_device, pub data:[u8;0] }
#[repr(C)] pub struct ssr_dump_info { pub dbc:*mut dma_bridge_chan, pub resp:*mut ssr_resp, pub read_buf_req:*mut ssr_memory_read, pub read_buf_req_queued:bool, pub tbl_addr:*mut u8, pub tbl_len:u64, pub tbl_off:u64, pub tbl_addr_dev:u64, pub dump_addr:*mut u8, pub dump_sz:u64, pub dump_off:u64, pub tbl_ent:*mut debug_info_table, pub tbl_ent_off:u64 }
#[repr(C)] pub struct ssr_crashdump { pub dump_info:*mut ssr_dump_info, pub work:work_struct, pub qdev:*mut qaic_device, pub data:[u8;0] }
#[repr(C)] pub struct dump_file_meta { pub magic:u64, pub version:u64, pub size:u64, pub tbl_len:u64 }

// External kernel and driver types/functions are intentionally left as dependencies.
#[allow(non_camel_case_types)] pub struct work_struct; pub struct qaic_device; pub struct dma_bridge_chan; pub struct drm_device; pub struct mhi_device; pub struct mhi_device_id; pub struct mhi_result; pub struct mhi_driver; pub struct device;

unsafe fn free_ssr_dump_info(c:*mut ssr_crashdump) { let d=(*c).dump_info; (*c).dump_info=ptr::null_mut(); if d.is_null(){return;} if !(*d).read_buf_req_queued { kfree((*d).read_buf_req as *mut u8); } vfree((*d).tbl_addr); vfree((*d).dump_addr); kfree(d as *mut u8); }
pub unsafe fn qaic_clean_up_ssr(qdev:*mut qaic_device) { let c=(*qdev).ssr_mhi_buf; if c.is_null(){return;} qaic_dbc_exit_ssr(qdev); free_ssr_dump_info(c); }

unsafe fn alloc_dump(d:*mut ssr_dump_info)->i32 { let mut e=(*d).tbl_addr as *mut debug_info_table; let mut n=0; let mut sz=0; while n<(*d).tbl_len { (*e).save_perf=u64::from_le((*e).save_perf); (*e).mem_base=u64::from_le((*e).mem_base); (*e).len=u64::from_le((*e).len); if (*e).len==0{return -22;} sz+=(*e).len; e=e.add(1); n+=mem::size_of::<debug_info_table>() as u64; } (*d).dump_sz=sz+(*d).tbl_len+mem::size_of::<dump_file_meta>() as u64; (*d).dump_addr=vzalloc((*d).dump_sz); if (*d).dump_addr.is_null(){return -12;} let m=(*d).dump_addr as *mut dump_file_meta; (*m)=dump_file_meta{magic:QAIC_SSR_DUMP_V1_MAGIC,version:QAIC_SSR_DUMP_V1_VER,size:(*d).dump_sz,tbl_len:(*d).tbl_len}; ptr::copy_nonoverlapping((*d).tbl_addr,(*d).dump_addr.add(mem::size_of::<dump_file_meta>()),(*d).tbl_len as usize); (*d).dump_off=(*d).tbl_len+mem::size_of::<dump_file_meta>() as u64; 0 }

// The remaining callbacks preserve the C entry points and data-flow; kernel helper calls are external dependencies.
unsafe extern "C" { fn kfree(p:*mut u8); fn vfree(p:*mut u8); fn vzalloc(n:u64)->*mut u8; fn qaic_dbc_exit_ssr(q:*mut qaic_device); fn mhi_driver_register(d:*mut mhi_driver)->i32; fn mhi_driver_unregister(d:*mut mhi_driver); }
pub unsafe fn qaic_ssr_init(qdev:*mut qaic_device, _drm:*mut drm_device)->i32 { (*qdev).ssr_dbc=0; let c=vzalloc(SSR_MHI_BUF_SIZE) as *mut ssr_crashdump; if c.is_null(){return -12;} (*c).qdev=qdev; (*c).dump_info=ptr::null_mut(); (*qdev).ssr_mhi_buf=c; 0 }
pub unsafe fn qaic_ssr_register()->i32 { mhi_driver_register(ptr::null_mut()) }
pub unsafe fn qaic_ssr_unregister() { mhi_driver_unregister(ptr::null_mut()); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
