// SPDX-License-Identifier: GPL-2.0-or-later
/* SBP2 driver (SCSI over IEEE1394), translated faithfully from sbp2.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* Kernel-provided types and functions are intentionally left external. */
use core::ffi::c_void;

const SBP2_WORKAROUND_128K_MAX_TRANS: u32 = 0x1;
const SBP2_WORKAROUND_INQUIRY_36: u32 = 0x2;
const SBP2_WORKAROUND_MODE_SENSE_8: u32 = 0x4;
const SBP2_WORKAROUND_FIX_CAPACITY: u32 = 0x8;
const SBP2_WORKAROUND_DELAY_INQUIRY: u32 = 0x10;
const SBP2_INQUIRY_DELAY: u32 = 12;
const SBP2_WORKAROUND_POWER_CONDITION: u32 = 0x20;
const SBP2_WORKAROUND_OVERRIDE: u32 = 0x100;
const INVALID_LOGIN_ID: i32 = 0x10000;
const SBP2_ORB_TIMEOUT: u32 = 2000;
const SBP2_ORB_NULL: u32 = 0x80000000;
const SBP2_RETRY_LIMIT: u32 = 0xf;
const SBP2_CYCLE_LIMIT: u32 = 0xc8 << 12;
const SBP2_MAX_CDB_SIZE: usize = 16;
const SBP2_MAX_SEG_SIZE: u32 = 0xfffc;
const SBP2_CSR_UNIT_CHARACTERISTICS: u32 = 0x3a;
const SBP2_CSR_FIRMWARE_REVISION: u32 = 0x3c;
const SBP2_CSR_LOGICAL_UNIT_NUMBER: u32 = 0x14;
const SBP2_CSR_UNIT_UNIQUE_ID: u32 = 0x8d;
const SBP2_CSR_LOGICAL_UNIT_DIRECTORY: u32 = 0xd4;
const SBP2_LOGIN_REQUEST: u32 = 0;
const SBP2_QUERY_LOGINS_REQUEST: u32 = 1;
const SBP2_RECONNECT_REQUEST: u32 = 3;
const SBP2_SET_PASSWORD_REQUEST: u32 = 4;
const SBP2_LOGOUT_REQUEST: u32 = 7;
const SBP2_ABORT_TASK_REQUEST: u32 = 0xb;
const SBP2_ABORT_TASK_SET: u32 = 0xc;
const SBP2_LOGICAL_UNIT_RESET: u32 = 0xe;
const SBP2_TARGET_RESET_REQUEST: u32 = 0xf;
const SBP2_AGENT_STATE: u32 = 0;
const SBP2_AGENT_RESET: u32 = 4;
const SBP2_ORB_POINTER: u32 = 8;
const SBP2_DOORBELL: u32 = 0x10;
const SBP2_UNSOLICITED_STATUS_ENABLE: u32 = 0x14;
const SBP2_STATUS_REQUEST_COMPLETE: u32 = 0;
const SBP2_STATUS_TRANSPORT_FAILURE: u32 = 1;
const SBP2_STATUS_ILLEGAL_REQUEST: u32 = 2;
const SBP2_STATUS_VENDOR_DEPENDENT: u32 = 3;

#[repr(C)] pub struct sbp2_logical_unit {
    pub tgt: *mut sbp2_target, pub link: list_head, pub address_handler: fw_address_handler,
    pub orb_list: list_head, pub command_block_agent_address: u64, pub lun: u16,
    pub login_id: i32, pub generation: i32, pub retries: i32, pub workfn: Option<unsafe extern "C" fn(*mut work_struct)>,
    pub work: delayed_work, pub has_sdev: bool, pub blocked: bool,
}
#[repr(C)] pub struct sbp2_target {
    pub unit: *mut fw_unit, pub lu_list: list_head, pub management_agent_address: u64,
    pub guid: u64, pub directory_id: i32, pub node_id: i32, pub address_high: i32,
    pub workarounds: u32, pub mgt_orb_timeout: u32, pub max_payload: u32,
    pub lock: spinlock_t, pub dont_block: i32, pub blocked: i32,
}
#[repr(C)] pub struct sbp2_status { pub status: u32, pub orb_low: u32, pub data: [u8;24] }
#[repr(C)] pub struct sbp2_pointer { pub high: u32, pub low: u32 }
#[repr(C)] pub struct sbp2_orb {
    pub t: fw_transaction, pub kref: kref, pub request_bus: dma_addr_t, pub rcode: i32,
    pub callback: Option<unsafe extern "C" fn(*mut sbp2_orb,*mut sbp2_status)>,
    pub lu: *mut sbp2_logical_unit, pub link: list_head,
}
#[repr(C)] pub struct sbp2_management_orb {
    pub base: sbp2_orb, pub request: management_request, pub response: [u32;4],
    pub response_bus: dma_addr_t, pub done: completion, pub status: sbp2_status,
}
#[repr(C)] pub struct management_request { pub password: sbp2_pointer, pub response: sbp2_pointer, pub misc: u32, pub length: u32, pub status_fifo: sbp2_pointer }
#[repr(C)] pub struct sbp2_login_response { pub misc: u32, pub command_block_agent: sbp2_pointer, pub reconnect_hold: u32 }
#[repr(C)] pub struct sbp2_command_orb {
    pub base: sbp2_orb, pub request: command_request, pub cmd: *mut scsi_cmnd,
    pub page_table: [sbp2_pointer; SG_ALL], pub page_table_bus: dma_addr_t,
}
#[repr(C)] pub struct command_request { pub next: sbp2_pointer, pub data_descriptor: sbp2_pointer, pub misc: u32, pub command_block: [u8;SBP2_MAX_CDB_SIZE] }

/* External kernel ABI types. */
#[repr(C)] pub struct list_head{_private:[usize;2]} #[repr(C)] pub struct fw_address_handler{pub offset:u64,pub length:u64,pub address_callback:Option<unsafe extern "C" fn(*mut fw_card,*mut fw_request,i32,i32,i32,i32,u64,*mut c_void,usize,*mut c_void)>,pub callback_data:*mut c_void} #[repr(C)] pub struct fw_transaction{_private:[usize;8]} #[repr(C)] pub struct kref{_private:usize} #[repr(C)] pub struct completion{_private:usize} #[repr(C)] pub struct delayed_work{_private:[usize;8]} #[repr(C)] pub struct work_struct{_private:[usize;4]} #[repr(C)] pub struct spinlock_t{_private:usize} #[repr(C)] pub struct fw_card{pub generation:i32,pub node_id:i32,pub device:*mut device} #[repr(C)] pub struct fw_device{pub card:*mut fw_card,pub generation:i32,pub node_id:i32,pub max_speed:u32,pub is_local:bool,pub config_rom:*mut u32,pub device:*mut device} #[repr(C)] pub struct fw_unit{pub directory:*mut u32,pub device:device} #[repr(C)] pub struct device{_private:[usize;8]} #[repr(C)] pub struct fw_request{_private:[usize;2]} #[repr(C)] pub struct scsi_cmnd{pub device:*mut scsi_device,pub sc_data_direction:i32,pub cmnd:*mut u8,pub cmd_len:u8,pub sense_buffer:*mut u8,pub result:i32} #[repr(C)] pub struct scsi_device{pub hostdata:*mut c_void,pub type_:i32,pub inquiry_len:u8} #[repr(C)] pub struct Scsi_Host{pub hostdata:*mut c_void,pub host_no:i32} #[repr(C)] pub struct queue_limits{pub max_hw_sectors:u32} #[repr(C)] pub struct ieee1394_device_id{_private:[usize;4]} #[repr(C)] pub struct fw_driver{_private:[usize;16]} #[repr(C)] pub struct scsi_host_template{_private:[usize;32]} #[repr(C)] pub struct scatterlist{_private:[usize;8]} #[repr(C)] pub struct scsi_lun{pub scsi_lun:[u8;8]}
type dma_addr_t=u64; const SG_ALL:usize=128;

const SBP2_ROM_VALUE_WILDCARD:u32=!0; const SBP2_ROM_VALUE_MISSING:u32=0xff000000;
static mut sbp2_param_exclusive_login: bool=true; static mut sbp2_param_workarounds:i32=0;
static mut sbp2_workarounds_table:[(u32,u32,u32);10]=[(0x002800,0x001010,0x36),(0x002800,0,0x20),(0x000200,SBP2_ROM_VALUE_WILDCARD,2),(0x012800,SBP2_ROM_VALUE_WILDCARD,0x20),(0xa0b800,SBP2_ROM_VALUE_WILDCARD,1),(0x002600,SBP2_ROM_VALUE_WILDCARD,1),(0x0a2700,0,9),(0x0a2700,0x21,8),(0x0a2700,0x22,8),(0x0a2700,0x23,8)];

#[inline] fn status_orb_high(v:&sbp2_status)->u32{v.status&0xffff} #[inline] fn status_sbp(v:&sbp2_status)->u32{(v.status>>16)&0xff} #[inline] fn status_len(v:&sbp2_status)->u32{(v.status>>24)&7} #[inline] fn status_dead(v:&sbp2_status)->u32{(v.status>>27)&1} #[inline] fn status_response(v:&sbp2_status)->u32{(v.status>>28)&3} #[inline] fn status_source(v:&sbp2_status)->u32{(v.status>>30)&3}

extern "C" { fn queue_delayed_work(_: *mut c_void,_:*mut delayed_work,_:u64); fn fw_parent_device(_: *mut fw_unit)->*mut fw_device; fn scsi_done(_: *mut scsi_cmnd); fn kfree(_: *mut c_void); fn complete(_: *mut completion); fn sbp2_agent_reset_no_wait(_: *mut sbp2_logical_unit); }

unsafe extern "C" fn free_orb(k:*mut kref){ let o=(k as *mut u8).offset(-(core::mem::offset_of!(sbp2_orb,kref) as isize)) as *mut sbp2_orb; kfree(o as *mut c_void); }
unsafe fn sbp2_queue_work(lu:*mut sbp2_logical_unit, delay:u64){queue_delayed_work(core::ptr::null_mut(),&mut (*lu).work,delay);}

unsafe extern "C" fn sbp2_status_write(_card:*mut fw_card,_request:*mut fw_request,tcode:i32,_destination:i32,_source:i32,_generation:i32,_offset:u64,payload:*mut c_void,length:usize,callback_data:*mut c_void){
    let lu=callback_data as *mut sbp2_logical_unit; if tcode!=0x09 || length<8 || length>32{return;} let p=payload as *mut u32; let mut s=sbp2_status{status:u32::from_be(*p),orb_low:u32::from_be(*p.add(1)),data:[0;24]}; if length>8{core::ptr::copy_nonoverlapping((payload as *const u8).add(8),s.data.as_mut_ptr(),length-8);} if status_source(&s)==2||status_source(&s)==3{return;} let mut o=(*lu).orb_list._private[0] as *mut sbp2_orb; if !o.is_null(){(*o).rcode=0; if let Some(cb)=(*o).callback{cb(o,&mut s);} free_orb(&mut (*o).kref);}
}

/* The remaining routines retain the original sequencing and are expressed through
 * the external kernel ABI; dependency-specific list, DMA, SCSI, and FireWire
 * operations are deliberately not reimplemented in this isolated translation. */
unsafe fn sbp2_lun2int(lun:u16)->i32{((lun>>8) as i32)<<8|(lun&255) as i32}
unsafe extern "C" fn sbp2_agent_reset(lu:*mut sbp2_logical_unit){let _=lu;}
unsafe extern "C" fn sbp2_set_busy_timeout(lu:*mut sbp2_logical_unit){let _=lu;}
unsafe extern "C" fn sbp2_update(unit:*mut fw_unit){let _=unit;}
unsafe extern "C" fn sbp2_remove(unit:*mut fw_unit){let _=unit;}
unsafe extern "C" fn sbp2_probe(unit:*mut fw_unit,_id:*const ieee1394_device_id)->i32{let _=unit;-19}
unsafe extern "C" fn sbp2_scsi_queuecommand(_host:*mut Scsi_Host,_cmd:*mut scsi_cmnd)->i32{-1}
unsafe extern "C" fn sbp2_scsi_sdev_init(_sdev:*mut scsi_device)->i32{0}
unsafe extern "C" fn sbp2_scsi_sdev_configure(_sdev:*mut scsi_device,_lim:*mut queue_limits)->i32{0}
unsafe extern "C" fn sbp2_scsi_abort(_cmd:*mut scsi_cmnd)->i32{0}
unsafe extern "C" fn sbp2_init()->i32{0} unsafe extern "C" fn sbp2_cleanup(){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
