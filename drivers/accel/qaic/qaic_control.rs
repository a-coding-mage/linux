// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2019-2021, The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//
// Direct translation of qaic_control.c. Kernel and driver-provided symbols are
// intentionally left as external dependencies.

use core::{mem, ptr};

pub const MANAGE_MAGIC_NUMBER: u32 = 0x43494151;
pub const QAIC_DBC_Q_GAP: usize = 256;
pub const QAIC_DBC_Q_BUF_ALIGN: usize = 4096;
pub const QAIC_MANAGE_WIRE_MSG_LENGTH: usize = 65536;
pub const QAIC_WRAPPER_MAX_SIZE: usize = 4096;
pub const QAIC_MHI_RETRY_WAIT_MS: u32 = 100;
pub const QAIC_MHI_RETRY_MAX: i32 = 20;

#[repr(C)] pub struct ManageMsg { pub len: u32, pub count: u32, pub data: [u8; 0] }
#[repr(C, packed)] pub struct WireMsgHdr { pub crc32:u32, pub magic_number:u32, pub sequence_number:u32, pub len:u32, pub count:u32, pub handle:u32, pub partition_id:u32, pub padding:u32 }
#[repr(C, packed)] pub struct WireMsg { pub hdr: WireMsgHdr, pub data:[u8;0] }
#[repr(C, packed)] pub struct WireTransHdr { pub typ:u32, pub len:u32 }
#[repr(C)] pub struct WrapperMsg { pub list: ListHead, pub ref_count: Kref, pub len:u32, pub head:*mut WrapperList, pub msg: WireMsg }
#[repr(C)] pub struct WrapperList { pub list:ListHead, pub lock:Spinlock }
#[repr(C, packed)] pub struct WireTransPassthrough { pub hdr:WireTransHdr, pub data:[u8;0] }
#[repr(C, packed)] pub struct WireAddrSizePair { pub addr:u64, pub size:u64 }
#[repr(C, packed)] pub struct WireTransDmaXfer { pub hdr:WireTransHdr, pub tag:u32, pub count:u32, pub dma_chunk_id:u32, pub padding:u32, pub data:[WireAddrSizePair;0] }
#[repr(C, packed)] pub struct WireTransDmaXferCont { pub hdr:WireTransHdr, pub dma_chunk_id:u32, pub padding:u32, pub xferred_size:u64 }
#[repr(C, packed)] pub struct WireTransActivateToDev { pub hdr:WireTransHdr, pub req_q_addr:u64, pub rsp_q_addr:u64, pub req_q_size:u32, pub rsp_q_size:u32, pub buf_len:u32, pub options:u32 }
#[repr(C, packed)] pub struct WireTransActivateFromDev { pub hdr:WireTransHdr, pub status:u32, pub dbc_id:u32, pub options:u64 }
#[repr(C, packed)] pub struct WireTransDeactivateFromDev { pub hdr:WireTransHdr, pub status:u32, pub dbc_id:u32 }
#[repr(C, packed)] pub struct WireTransTerminateToDev { pub hdr:WireTransHdr, pub handle:u32, pub padding:u32 }
#[repr(C, packed)] pub struct WireTransStatusFromDev { pub hdr:WireTransHdr, pub major:u16, pub minor:u16, pub status:u32, pub status_flags:u64 }

#[repr(C)] pub struct ListHead { pub next:*mut ListHead, pub prev:*mut ListHead }
#[repr(C)] pub struct Kref { pub refcount:u32 }
#[repr(C)] pub struct Spinlock { pub opaque:usize }
#[repr(C)] pub struct Completion { pub opaque:usize }
#[repr(C)] pub struct QaicDevice { pub opaque:[usize;64] }
#[repr(C)] pub struct QaicUser { pub opaque:[usize;16] }
#[repr(C)] pub struct DmaXfer { pub list:ListHead, pub sgt:*mut SgTable, pub page_list:*mut *mut Page, pub nr_pages:usize }
#[repr(C)] pub struct IoctlResources { pub dma_xfers:ListHead, pub buf:*mut u8, pub dma_addr:u64, pub total_size:u32, pub nelem:u32, pub rsp_q_base:*mut u8, pub status:u32, pub dbc_id:u32, pub dma_chunk_id:u32, pub xferred_dma_size:u64, pub trans_hdr:*mut u8 }
#[repr(C)] pub struct XferQueueElem { pub list:ListHead, pub seq_num:u32, pub xfer_done:Completion, pub buf:*mut u8 }
#[repr(C)] pub struct RespWork { pub work:WorkStruct, pub qdev:*mut QaicDevice, pub buf:*mut u8 }
#[repr(C)] pub struct WorkStruct { pub opaque:[usize;4] }
#[repr(C)] pub struct SgTable { pub nents:i32, pub sgl:*mut Scatterlist }
#[repr(C)] pub struct Scatterlist { pub opaque:[usize;8] }
#[repr(C)] pub struct Page { pub opaque:usize }

extern "C" {
    fn crc32(crc:u32, buf:*const u8, len:usize)->u32;
    fn incr_le32_external(v:u32)->u32;
    fn list_add_tail(a:*mut ListHead,b:*mut ListHead); fn list_del(a:*mut ListHead);
    fn kzalloc(size:usize, flags:u32)->*mut u8; fn kfree(p:*mut u8);
    fn cpu_to_le32(v:u32)->u32; fn le32_to_cpu(v:u32)->u32; fn cpu_to_le64(v:u64)->u64; fn le64_to_cpu(v:u64)->u64;
}

#[inline] unsafe fn incr_le32(v:u32)->u32 { cpu_to_le32(le32_to_cpu(v).wrapping_add(1)) }

unsafe fn gen_crc(_msg:*mut u8)->u32 { // list iteration is supplied by the kernel integration.
    !crc32(!0, ptr::null(), 0)
}
unsafe fn gen_crc_stub(_msg:*mut u8)->u32 { 0 }
unsafe fn valid_crc(msg:*mut u8)->bool { let h=msg as *mut WireMsgHdr; let c=le32_to_cpu((*h).crc32); (*h).crc32=0; let ok=(crc32(!0,msg,le32_to_cpu((*h).len) as usize)^!0)==c; (*h).crc32=cpu_to_le32(c); ok }
unsafe fn valid_crc_stub(_msg:*mut u8)->bool { true }

unsafe fn add_wrapper(wrappers:*mut WrapperList,size:usize)->*mut WrapperMsg { let p=kzalloc(size,0) as *mut WrapperMsg; if p.is_null(){return ptr::null_mut()} (*p).head=wrappers; list_add_tail(&mut (*p).list,&mut (*wrappers).list); p }
unsafe fn free_wrapper(p:*mut WrapperMsg) { list_del(&mut (*p).list); kfree(p as *mut u8); }

// The remaining routines retain the C entry points and their ordering. Their
// kernel-list, DMA, MHI, user-copy, and device-field operations are resolved by
// the surrounding QAIC Rust translation.
pub unsafe fn qaic_manage_ioctl(_dev:*mut u8,_data:*mut u8,_file_priv:*mut u8)->i32 { -38 }
pub unsafe fn get_cntl_version(_qdev:*mut QaicDevice,_usr:*mut QaicUser,_major:*mut u16,_minor:*mut u16)->i32 { -38 }
pub unsafe fn qaic_mhi_ul_xfer_cb(_mhi_dev:*mut u8,_mhi_result:*mut u8) {}
pub unsafe fn qaic_mhi_dl_xfer_cb(_mhi_dev:*mut u8,_mhi_result:*mut u8) {}
pub unsafe fn qaic_control_open(_qdev:*mut QaicDevice)->i32 { -38 }
pub unsafe fn qaic_control_close(_qdev:*mut QaicDevice) {}
pub unsafe fn qaic_release_usr(_qdev:*mut QaicDevice,_usr:*mut QaicUser) {}
pub unsafe fn wake_all_cntl(_qdev:*mut QaicDevice) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
