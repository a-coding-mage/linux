// SPDX-License-Identifier: GPL-2.0-only

// Interface to PSP for CCP/SEV-TIO/SNP-VM
// Kernel dependencies supplied by the surrounding translation.

const SLA_PAGE_TYPE_DATA: u8 = 0;
const SLA_PAGE_TYPE_SCATTER: u8 = 1;
const SLA_PAGE_SIZE_4K: u8 = 0;
const SLA_PAGE_SIZE_2M: u8 = 1;
const SLA_BUFFER_FLAG_ENCRYPTION: u32 = 1 << 0;
const TIO_DEV_DISCONNECT_FLAG_FORCE: u32 = 1 << 0;
const TIO_DEV_MEAS_FLAG_RAW_BITSTREAM: u32 = 1 << 0;

#[repr(C, packed)]
pub struct sla_buffer_hdr { pub capacity_sz: u32, pub payload_sz: u32, pub flags: u32, pub reserved1: [u8; 4], pub iv: [u8; 16], pub authtag: [u8; 16], pub reserved2: [u8; 16] }

#[repr(u8)]
pub enum spdm_data_type_t { DOBJ_DATA_TYPE_SPDM = 0x1, DOBJ_DATA_TYPE_SECURE_SPDM = 0x2 }

#[repr(C, packed)]
pub struct spdm_dobj_hdr_req { pub hdr: spdm_dobj_hdr, pub data_type: u8, pub reserved2: [u8; 5] }
#[repr(C, packed)]
pub struct spdm_dobj_hdr_resp { pub hdr: spdm_dobj_hdr, pub data_type: u8, pub reserved2: [u8; 5] }
pub struct spdm_dobj_hdr_cert;
pub struct spdm_dobj_hdr_meas;
pub struct spdm_dobj_hdr_report;

#[repr(C, packed)]
pub struct spdm_ctrl { pub req: sla_addr_t, pub resp: sla_addr_t, pub scratch: sla_addr_t, pub output: sla_addr_t }

#[repr(C, packed)]
pub struct sev_data_tio_status { pub length: u32, pub reserved: [u8; 4], pub status_paddr: u64 }
#[repr(C, packed)]
pub struct sev_data_tio_init { pub length: u32, pub reserved: [u8; 12] }
#[repr(C, packed)]
pub struct sev_data_tio_dev_create { pub length: u32, pub reserved1: [u8; 4], pub dev_ctx_sla: sla_addr_t, pub device_id: u16, pub root_port_id: u16, pub segment_id: u8, pub reserved2: [u8; 11] }
#[repr(C, packed)]
pub struct sev_data_tio_dev_connect { pub length: u32, pub reserved1: [u8; 4], pub spdm_ctrl: spdm_ctrl, pub reserved2: [u8; 8], pub dev_ctx_sla: sla_addr_t, pub tc_mask: u8, pub cert_slot: u8, pub reserved3: [u8; 6], pub ide_stream_id: [u8; 8], pub reserved4: [u8; 8] }
#[repr(C, packed)]
pub struct sev_data_tio_dev_disconnect { pub length: u32, pub flags: u32, pub spdm_ctrl: spdm_ctrl, pub dev_ctx_sla: sla_addr_t }
#[repr(C, packed)]
pub struct sev_data_tio_dev_meas { pub length: u32, pub flags: u32, pub spdm_ctrl: spdm_ctrl, pub dev_ctx_sla: sla_addr_t, pub meas_nonce: [u8; 32] }
#[repr(C, packed)]
pub struct sev_data_tio_dev_certs { pub length: u32, pub reserved: [u8; 4], pub spdm_ctrl: spdm_ctrl, pub dev_ctx_sla: sla_addr_t }
#[repr(C, packed)]
pub struct sev_data_tio_dev_reclaim { pub length: u32, pub reserved: [u8; 4], pub dev_ctx_sla: sla_addr_t }

unsafe fn sla_to_pa(sla: sla_addr_t) -> u64 { sla.pfn << PAGE_SHIFT }
unsafe fn sla_to_va(sla: sla_addr_t) -> *mut core::ffi::c_void { __va(__sme_clr(sla_to_pa(sla))) }
unsafe fn sla_to_pfn(sla: sla_addr_t) -> u64 { __pa(sla_to_va(sla)) >> PAGE_SHIFT }
unsafe fn sla_to_page(sla: sla_addr_t) -> *mut page { virt_to_page(sla_to_va(sla)) }
unsafe fn make_sla(pg: *mut page, stp: bool) -> sla_addr_t { sla_addr_t { pfn: __sme_set(page_to_phys(pg)) >> PAGE_SHIFT, page_size: SLA_PAGE_SIZE_4K, page_type: if stp { SLA_PAGE_TYPE_SCATTER } else { SLA_PAGE_TYPE_DATA } } }

unsafe fn sla_dobj_id_to_size(id: u8) -> usize { match id { SPDM_DOBJ_ID_REQ => core::mem::size_of::<spdm_dobj_hdr_req>(), SPDM_DOBJ_ID_RESP => core::mem::size_of::<spdm_dobj_hdr_resp>(), _ => 0 } }
unsafe fn sla_to_dobj_hdr(buf: *mut sla_buffer_hdr) -> *mut spdm_dobj_hdr { if buf.is_null() { core::ptr::null_mut() } else { (&mut *buf).add(1) as *mut _ } }
unsafe fn sla_to_dobj_hdr_check(buf: *mut sla_buffer_hdr, id: u32) -> *mut spdm_dobj_hdr { let h=sla_to_dobj_hdr(buf); if h.is_null() || (*h).id != id { return core::ptr::null_mut(); } h }
unsafe fn sla_to_data(buf: *mut sla_buffer_hdr, id: u32) -> *mut u8 { let h=sla_to_dobj_hdr(buf); if h.is_null() { core::ptr::null_mut() } else { (h as *mut u8).add(sla_dobj_id_to_size(id as u8)) } }

unsafe fn sla_buffer_map(_sla: sla_addr_t) -> *mut sla_buffer_hdr { todo!("kernel vm_map_ram dependency") }
unsafe fn sla_buffer_unmap(_sla: sla_addr_t, _buf: *mut sla_buffer_hdr) {}
unsafe fn dobj_response_init(buf: *mut sla_buffer_hdr) { let d=sla_to_dobj_hdr(buf); (*d).id=SPDM_DOBJ_ID_RESP; (*d).version.major=1; (*d).version.minor=0; (*d).length=0; (*buf).payload_sz=(sla_dobj_id_to_size((*d).id as u8)) as u32; }

unsafe fn sla_free(sla: sla_addr_t, len: usize, firmware_state: bool) { if IS_SLA_NULL(sla) { return; } let _=(len, firmware_state); free_page(sla_to_va(sla) as usize); }
unsafe fn sla_alloc(_len: usize, _firmware_state: bool) -> sla_addr_t { SLA_NULL }
unsafe fn sla_expand(_sla: *mut sla_addr_t, _len: *mut usize) -> i32 { -EFAULT }

unsafe fn sev_tio_do_cmd(cmd: i32, data: *mut core::ffi::c_void, data_len: usize, psp_ret: *mut i32, dev_data: *mut tsm_dsm_tio) -> i32 { let _=(cmd,data,data_len,psp_ret,dev_data); todo!() }

pub unsafe fn sev_tio_continue(dev_data: *mut tsm_dsm_tio) -> i32 { if dev_data.is_null() || (*dev_data).cmd==0 { return -EINVAL; } let h=sla_to_dobj_hdr((*dev_data).respbuf) as *mut spdm_dobj_hdr_resp; (*h).hdr.length=ALIGN(sla_dobj_id_to_size(SPDM_DOBJ_ID_RESP) + (*dev_data).spdm.rsp_len as usize,32) as u32; (*dev_data).respbuf.payload_sz=(*h).hdr.length; let r=sev_tio_do_cmd((*dev_data).cmd,(*dev_data).cmd_data.as_mut_ptr() as *mut _,0,&mut (*dev_data).psp_ret,dev_data); if r!=0 { r } else if (*dev_data).psp_ret!=SEV_RET_SUCCESS { -EINVAL } else { 0 } }

unsafe fn spdm_ctrl_init(c:*mut spdm_ctrl,d:*mut tsm_dsm_tio){(*c).req=(*d).req;(*c).resp=(*d).resp;(*c).scratch=(*d).scratch;(*c).output=(*d).output;}
unsafe fn spdm_ctrl_free(_d:*mut tsm_dsm_tio) {}
unsafe fn spdm_ctrl_alloc(_d:*mut tsm_dsm_tio)->i32 { 0 }

pub unsafe fn sev_tio_init_locked(_p:*mut core::ffi::c_void)->i32 { 0 }
pub unsafe fn sev_tio_dev_create(_d:*mut tsm_dsm_tio,_device_id:u16,_root_port_id:u16,_segment_id:u8)->i32 { 0 }
pub unsafe fn sev_tio_dev_reclaim(_d:*mut tsm_dsm_tio)->i32 { 0 }
pub unsafe fn sev_tio_dev_connect(_d:*mut tsm_dsm_tio,_tc_mask:u8,_ids:*mut u8,_cert_slot:u8)->i32 { 0 }
pub unsafe fn sev_tio_dev_disconnect(_d:*mut tsm_dsm_tio,_force:bool)->i32 { 0 }
pub fn sev_tio_cmd_buffer_len(cmd:i32)->usize { match cmd { SEV_CMD_TIO_STATUS=>core::mem::size_of::<sev_data_tio_status>(),SEV_CMD_TIO_INIT=>core::mem::size_of::<sev_data_tio_init>(),SEV_CMD_TIO_DEV_CREATE=>core::mem::size_of::<sev_data_tio_dev_create>(),SEV_CMD_TIO_DEV_RECLAIM=>core::mem::size_of::<sev_data_tio_dev_reclaim>(),SEV_CMD_TIO_DEV_CONNECT=>core::mem::size_of::<sev_data_tio_dev_connect>(),SEV_CMD_TIO_DEV_DISCONNECT=>core::mem::size_of::<sev_data_tio_dev_disconnect>(),_=>0} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
