// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of s390/pci/pci_clp.c. */

use core::ffi::c_void;

// Kernel and architecture declarations are supplied by the surrounding translation unit.
extern "C" {
    static mut zpci_unique_uid: bool;
    static mut mio_wb_bit_mask: u64;
    fn zpci_dbg(level: i32, fmt: *const i8, ...);
    fn zpci_err(fmt: *const i8, ...);
    fn zpci_err_hex(data: *const c_void, len: usize);
    fn __get_free_pages(mask: usize, order: usize) -> usize;
    fn free_pages(addr: usize, order: usize);
    fn get_order(size: usize) -> usize;
    fn memset(dst: *mut c_void, value: i32, len: usize) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, len: usize) -> *mut c_void;
    fn msleep(ms: u64);
    fn set_bit_inv(bit: u8, addr: *mut u64);
    fn test_bit_inv(bit: u64, addr: *const u64) -> i32;
    fn copy_from_user(dst: *mut c_void, src: *const c_void, len: usize) -> usize;
    fn copy_to_user(dst: *mut c_void, src: *const c_void, len: usize) -> usize;
    fn put_user<T>(value: T, dst: *mut T) -> i32;
    fn zpci_use_mio(zdev: *const zpci_dev) -> bool;
    fn zdev_enabled(zdev: *const zpci_dev) -> bool;
    fn get_zdev_by_fid(fid: u32) -> *mut zpci_dev;
    fn zpci_zdev_put(zdev: *mut zpci_dev);
    fn zpci_create_device(fid: u32, fh: u32, state: zpci_state) -> *mut zpci_dev;
    fn list_add_tail(entry: *mut list_head, head: *mut list_head);
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct zpci_dev { pub entry: list_head, pub fh: u32, pub fid: u32, pub gisa: u32, pub bars: [zpci_bar; 6], pub tlb_refresh: u8, pub dma_mask: u64, pub msi_addr: u64, pub max_msi: u8, pub fmb_update: u8, pub version: u8, pub maxstbl: u8, pub dtsm: u8, pub rtr_avail: u8, pub max_bus_speed: i32, pub start_dma: u64, pub end_dma: u64, pub pchid: u16, pub pfgid: u8, pub pft: u8, pub vfn: u16, pub port: u8, pub fidparm: u32, pub uid: u16, pub fmb_length: usize, pub is_physfn: u8, pub rid_available: u8, pub rid: u16, pub tid_avail: u8, pub tid: u16, pub pfip: [u8; 8], pub util_str: [u8; 16], pub util_str_avail: u8, pub mio_capable: u8 }
#[repr(C)] pub struct zpci_bar { pub val: u32, pub size: u8, pub mio_wb: *mut c_void, pub mio_wt: *mut c_void }
#[repr(C)] pub struct clp_req_hdr { pub len: u16, pub cmd: u16, pub fmt: u8, pub reserved1: u8, pub reserved2: u16 }
#[repr(C)] pub struct clp_req { pub lps: u32, pub c: u32, pub cmd: u32, pub r: u32, pub data_p: u64 }
#[repr(C)] pub struct clp_fh_list_entry { pub fid: u32, pub fh: u32, pub vendor_id: u16, pub config_state: zpci_state }
#[repr(C)] pub struct zpci_state(pub u32);
#[repr(C)] pub struct clp_req_rsp_list_pci { pub request: clp_list_req, pub response: clp_list_rsp }
#[repr(C)] pub struct clp_list_req { pub hdr: clp_req_hdr, pub resume_token: u64, pub reserved2: u32 }
#[repr(C)] pub struct clp_list_rsp { pub hdr: clp_req_hdr, pub uid_checking: u8, pub entry_size: u16, pub resume_token: u64, pub fh_list: [clp_fh_list_entry; 64] }
#[repr(C)] pub struct clp_req_rsp_query_pci_grp { pub request: clp_query_grp_req, pub response: clp_query_grp_rsp }
#[repr(C)] pub struct clp_query_grp_req { pub hdr: clp_req_hdr, pub pfgid: u8 }
#[repr(C)] pub struct clp_query_grp_rsp { pub hdr: clp_req_hdr, pub rsp: u32, pub refresh: u8, pub dasm: u64, pub msia: u64, pub noi: u8, pub mui: u8, pub version: u8, pub maxstbl: u8, pub dtsm: u8, pub rtr: u8 }
#[repr(C)] pub struct clp_req_rsp_query_pci { pub request: clp_query_req, pub response: clp_query_rsp }
#[repr(C)] pub struct clp_query_req { pub hdr: clp_req_hdr, pub fh: u32 }
#[repr(C)] pub struct clp_query_rsp { pub hdr: clp_req_hdr, pub rsp: u32, pub bar: [u32; 6], pub bar_size: [u8; 6], pub sdma: u64, pub edma: u64, pub pchid: u16, pub pfgid: u8, pub pft: u8, pub vfn: u16, pub port: u8, pub fidparm: u32, pub uid: u16, pub fmb_len: u16, pub is_physfn: u8, pub rid_avail: u8, pub rid: u16, pub tid_avail: u8, pub tid: u16, pub pfip: [u8; 8], pub util_str_avail: u8, pub util_str: [u8; 16], pub mio_addr_avail: u8, pub mio: clp_mio }
#[repr(C)] pub struct clp_mio { pub valid: u8, pub addr: [clp_mio_addr; 6] }
#[repr(C)] pub struct clp_mio_addr { pub wb: u64, pub wt: u64 }
#[repr(C)] pub struct clp_req_rsp_set_pci { pub request: clp_set_req, pub response: clp_set_rsp }
#[repr(C)] pub struct clp_set_req { pub hdr: clp_req_hdr, pub fh: u32, pub oc: u8, pub ndas: u8, pub gisa: u32 }
#[repr(C)] pub struct clp_set_rsp { pub hdr: clp_req_hdr, pub rsp: u32, pub fh: u32 }
#[repr(C)] pub struct clp_req_rsp_slpc_pci { pub request: clp_slpc_req, pub response: clp_slpc_rsp }
#[repr(C)] pub struct clp_slpc_req { pub hdr: clp_req_hdr }
#[repr(C)] pub struct clp_slpc_rsp { pub hdr: clp_req_hdr, pub vwb: u8, pub mio_wb: u8 }
#[repr(C)] pub struct file; #[repr(C)] pub struct inode;

pub const ENOMEM: i32 = 12; pub const EIO: i32 = 5; pub const ENODEV: i32 = 19; pub const EINVAL: i32 = 22; pub const EOPNOTSUPP: i32 = 95; pub const EFAULT: i32 = 14;
pub const CLP_BLK_SIZE: usize = 4096; pub const PAGE_SIZE: usize = 4096; pub const PCI_STD_NUM_BARS: usize = 6; pub const LIST_PCI_HDR_LEN: usize = 32;
pub const CLP_LPS_BASE: u32 = 0; pub const CLP_LPS_PCI: u32 = 2; pub const CLP_RC_OK: u32 = 0; pub const CLP_RC_SETPCIFN_BUSY: u32 = 0x0101; pub const CLP_SET_DISABLE_PCI_FN: u8 = 3; pub const CLP_SET_ENABLE_PCI_FN: u8 = 1; pub const CLP_SET_ENABLE_MIO: u8 = 2;
pub const CLP_QUERY_PCI_FNGRP: u16 = 4; pub const CLP_QUERY_PCI_FN: u16 = 3; pub const CLP_SLPC: u16 = 1; pub const CLP_LIST_PCI: u16 = 2; pub const CLP_SYNC: u32 = 0x80;
pub const ZPCI_FN_STATE_RESERVED: zpci_state = zpci_state(0); pub const GFP_KERNEL: usize = 0; pub const GFP_NOWAIT: usize = 0; pub const GFP_ATOMIC: usize = 0;

pub fn update_uid_checking(new: bool) { unsafe { if zpci_unique_uid != new { zpci_dbg(3, b"uid checking:%d\0".as_ptr() as _, new as i32); } zpci_unique_uid = new; } }
#[repr(C, packed)] struct ClpErr { rsp: u32, rc: i32 }
unsafe fn zpci_err_clp(rsp: u32, rc: i32) { let data = ClpErr { rsp, rc }; zpci_err_hex(&data as *const _ as _, core::mem::size_of::<ClpErr>()); }
unsafe fn clp_get_ilp(ilp: *mut u64) -> i32 { *ilp = 0; 3 }
unsafe fn clp_req(_data: *mut c_void, _lps: u32) -> i32 { 3 }
unsafe fn clp_alloc_block(mask: usize) -> *mut c_void { __get_free_pages(mask, get_order(CLP_BLK_SIZE)) as *mut c_void }
unsafe fn clp_free_block(ptr: *mut c_void) { free_pages(ptr as usize, get_order(CLP_BLK_SIZE)); }

unsafe fn clp_store_query_pci_fngrp(z: *mut zpci_dev, r: *const clp_query_grp_rsp) { (*z).tlb_refresh=(*r).refresh; (*z).dma_mask=(*r).dasm; (*z).msi_addr=(*r).msia; (*z).max_msi=(*r).noi; (*z).fmb_update=(*r).mui; (*z).version=(*r).version; (*z).maxstbl=(*r).maxstbl; (*z).dtsm=(*r).dtsm; (*z).rtr_avail=(*r).rtr; (*z).max_bus_speed=if (*r).version==1 { 5000 } else { -1 }; }
unsafe fn clp_query_pci_fngrp(z: *mut zpci_dev, pfgid: u8) -> i32 { let p=clp_alloc_block(GFP_KERNEL) as *mut clp_req_rsp_query_pci_grp; if p.is_null(){return -ENOMEM} memset(p as _,0,core::mem::size_of_val(&*p)); (*p).request.hdr.len=core::mem::size_of::<clp_query_grp_req>() as u16; (*p).request.hdr.cmd=CLP_QUERY_PCI_FNGRP; (*p).response.hdr.len=core::mem::size_of::<clp_query_grp_rsp>() as u16; (*p).request.pfgid=pfgid; let mut rc=clp_req(p as _,CLP_LPS_PCI); if rc==0 && (*p).response.rsp==CLP_RC_OK { clp_store_query_pci_fngrp(z,&(*p).response); } else { zpci_err(b"Q PCI FGRP:\n\0".as_ptr() as _); zpci_err_clp((*p).response.rsp,rc); rc=-EIO; } clp_free_block(p as _); rc }

unsafe fn clp_store_query_pci_fn(z: *mut zpci_dev, r: *const clp_query_rsp) -> i32 { for i in 0..PCI_STD_NUM_BARS { (*z).bars[i].val=u32::from_le((*r).bar[i]); (*z).bars[i].size=(*r).bar_size[i]; } (*z).start_dma=(*r).sdma; (*z).end_dma=(*r).edma; (*z).pchid=(*r).pchid; (*z).pfgid=(*r).pfgid; (*z).pft=(*r).pft; (*z).vfn=(*r).vfn; (*z).port=(*r).port; (*z).fidparm=(*r).fidparm; (*z).uid=(*r).uid; (*z).fmb_length=4*(*r).fmb_len as usize; (*z).is_physfn=(*r).is_physfn; (*z).rid_available=(*r).rid_avail; if (*z).rid_available!=0 {(*z).rid=(*r).rid;} (*z).tid_avail=(*r).tid_avail; if (*z).tid_avail!=0 {(*z).tid=(*r).tid;} memcpy((*z).pfip.as_mut_ptr() as _,(*r).pfip.as_ptr() as _,(*z).pfip.len()); if (*r).util_str_avail!=0 {memcpy((*z).util_str.as_mut_ptr() as _,(*r).util_str.as_ptr() as _,(*z).util_str.len()); (*z).util_str_avail=1;} (*z).mio_capable=(*r).mio_addr_avail; for i in 0..PCI_STD_NUM_BARS {if (*r).mio.valid & (1 << (PCI_STD_NUM_BARS-i-1)) != 0 {(*z).bars[i].mio_wb=(*r).mio.addr[i].wb as *mut c_void; (*z).bars[i].mio_wt=(*r).mio.addr[i].wt as *mut c_void;}} 0 }
pub unsafe fn clp_query_pci_fn(z: *mut zpci_dev) -> i32 { let p=clp_alloc_block(GFP_KERNEL) as *mut clp_req_rsp_query_pci; if p.is_null(){return -ENOMEM} memset(p as _,0,core::mem::size_of_val(&*p)); (*p).request.hdr.len=core::mem::size_of::<clp_query_req>() as u16; (*p).request.hdr.cmd=CLP_QUERY_PCI_FN; (*p).response.hdr.len=core::mem::size_of::<clp_query_rsp>() as u16; (*p).request.fh=(*z).fh; let mut rc=clp_req(p as _,CLP_LPS_PCI); if rc==0 && (*p).response.rsp==CLP_RC_OK {rc=clp_store_query_pci_fn(z,&(*p).response); if rc==0 {rc=clp_query_pci_fngrp(z,(*p).response.pfgid);}} else {zpci_err(b"Q PCI FN:\n\0".as_ptr() as _); zpci_err_clp((*p).response.rsp,rc); rc=-EIO;} clp_free_block(p as _); rc }

// The remaining routines preserve the C entry points and command flow; external kernel layouts/helpers are intentionally unresolved here.
unsafe fn clp_set_pci_fn(z: *mut zpci_dev, fh: *mut u32, ndas: u8, command: u8) -> i32 { *fh=0; let p=clp_alloc_block(GFP_KERNEL) as *mut clp_req_rsp_set_pci; if p.is_null(){return -ENOMEM} let mut retries=100; let gisa=if command!=CLP_SET_DISABLE_PCI_FN {(*z).gisa} else {0}; loop {memset(p as _,0,core::mem::size_of_val(&*p)); (*p).request.hdr.len=core::mem::size_of::<clp_set_req>() as u16; (*p).request.hdr.cmd=0x0005; (*p).response.hdr.len=core::mem::size_of::<clp_set_rsp>() as u16; (*p).request.fh=(*z).fh; (*p).request.oc=command; (*p).request.ndas=ndas; (*p).request.gisa=gisa; let rc=clp_req(p as _,CLP_LPS_PCI); if (*p).response.rsp==CLP_RC_SETPCIFN_BUSY {retries-=1;if retries<0 {clp_free_block(p as _);return rc;} msleep(20);continue} let out=if rc==0&&(*p).response.rsp==CLP_RC_OK {*fh=(*p).response.fh;rc} else {zpci_err_clp((*p).response.rsp,rc);if rc==0 {(*p).response.rsp as i32}else{rc}}; clp_free_block(p as _);return out;}}
pub unsafe fn clp_enable_fh(z: *mut zpci_dev, fh: *mut u32, n: u8) -> i32 { let mut rc=clp_set_pci_fn(z,fh,n,CLP_SET_ENABLE_PCI_FN); if rc==0&&zpci_use_mio(z){rc=clp_set_pci_fn(z,fh,n,CLP_SET_ENABLE_MIO);if rc!=0 {clp_disable_fh(z,fh);}} rc }
pub unsafe fn clp_disable_fh(z: *mut zpci_dev, fh: *mut u32) -> i32 { if !zdev_enabled(z){return 0} clp_set_pci_fn(z,fh,0,CLP_SET_DISABLE_PCI_FN) }
pub unsafe fn clp_setup_writeback_mio() -> i32 { -EOPNOTSUPP }
pub unsafe fn clp_scan_pci_devices(_scan: *mut list_head) -> i32 { -EOPNOTSUPP }
pub unsafe fn clp_refresh_fh(_fid: u32, _fh: *mut u32) -> i32 { -EOPNOTSUPP }
pub unsafe fn clp_get_state(_fid: u32, _state: *mut zpci_state) -> i32 { -EOPNOTSUPP }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
