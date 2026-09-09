/* SPDX-License-Identifier: GPL-2.0-only */
/* Sony MemoryStick support; translated from the C header. */

#[repr(C, packed)]
pub struct ms_status_register { pub reserved: u8, pub interrupt: u8, pub status0: u8, pub status1: u8 }
pub const MEMSTICK_INT_CMDNAK: u8 = 0x01;
pub const MEMSTICK_INT_IOREQ: u8 = 0x08;
pub const MEMSTICK_INT_IOBREQ: u8 = 0x10;
pub const MEMSTICK_INT_BREQ: u8 = 0x20;
pub const MEMSTICK_INT_ERR: u8 = 0x40;
pub const MEMSTICK_INT_CED: u8 = 0x80;
pub const MEMSTICK_STATUS0_WP: u8 = 0x01;
pub const MEMSTICK_STATUS0_SL: u8 = 0x02;
pub const MEMSTICK_STATUS0_BF: u8 = 0x10;
pub const MEMSTICK_STATUS0_BE: u8 = 0x20;
pub const MEMSTICK_STATUS0_FB0: u8 = 0x40;
pub const MEMSTICK_STATUS0_MB: u8 = 0x80;
pub const MEMSTICK_STATUS1_UCFG: u8 = 0x01;
pub const MEMSTICK_STATUS1_FGER: u8 = 0x02;
pub const MEMSTICK_STATUS1_UCEX: u8 = 0x04;
pub const MEMSTICK_STATUS1_EXER: u8 = 0x08;
pub const MEMSTICK_STATUS1_UCDT: u8 = 0x10;
pub const MEMSTICK_STATUS1_DTER: u8 = 0x20;
pub const MEMSTICK_STATUS1_FB1: u8 = 0x40;
pub const MEMSTICK_STATUS1_MB: u8 = 0x80;

#[repr(C, packed)] pub struct ms_id_register { pub type_: u8, pub if_mode: u8, pub category: u8, pub class: u8 }
#[repr(C, packed)] pub struct ms_param_register { pub system: u8, pub block_address_msb: u8, pub block_address: u16, pub cp: u8, pub page_address: u8 }
pub const MEMSTICK_SYS_PAM: u8 = 0x08; pub const MEMSTICK_SYS_BAMD: u8 = 0x80;
pub const MEMSTICK_CP_BLOCK: u8 = 0x00; pub const MEMSTICK_CP_PAGE: u8 = 0x20; pub const MEMSTICK_CP_EXTRA: u8 = 0x40; pub const MEMSTICK_CP_OVERWRITE: u8 = 0x80;
#[repr(C, packed)] pub struct ms_extra_data_register { pub overwrite_flag: u8, pub management_flag: u8, pub logical_address: u16 }
pub const MEMSTICK_OVERWRITE_UDST: u8 = 0x10; pub const MEMSTICK_OVERWRITE_PGST1: u8 = 0x20; pub const MEMSTICK_OVERWRITE_PGST0: u8 = 0x40; pub const MEMSTICK_OVERWRITE_BKST: u8 = 0x80;
pub const MEMSTICK_MANAGEMENT_SYSFLG: u8 = 0x04; pub const MEMSTICK_MANAGEMENT_ATFLG: u8 = 0x08; pub const MEMSTICK_MANAGEMENT_SCMS1: u8 = 0x10; pub const MEMSTICK_MANAGEMENT_SCMS0: u8 = 0x20;
#[repr(C, packed)] pub struct ms_register { pub status: ms_status_register, pub id: ms_id_register, pub reserved: [u8; 8], pub param: ms_param_register, pub extra_data: ms_extra_data_register }

#[repr(C, packed)] pub struct mspro_param_register { pub system: u8, pub data_count: u16, pub data_address: u32, pub tpc_param: u8 }
pub const MEMSTICK_SYS_PAR4: u8 = 0x00; pub const MEMSTICK_SYS_PAR8: u8 = 0x40; pub const MEMSTICK_SYS_SERIAL: u8 = 0x80;
#[repr(C, packed)] pub struct mspro_io_info_register { pub version: u8, pub io_category: u8, pub current_req: u8, pub card_opt_info: u8, pub rdy_wait_time: u8 }
#[repr(C, packed)] pub struct mspro_io_func_register { pub func_enable: u8, pub func_select: u8, pub func_intmask: u8, pub transfer_mode: u8 }
#[repr(C, packed)] pub struct mspro_io_cmd_register { pub tpc_param: u16, pub data_count: u16, pub data_address: u32 }
#[repr(C, packed)] pub struct mspro_register { pub status: ms_status_register, pub id: ms_id_register, pub reserved0: [u8;8], pub param: mspro_param_register, pub reserved1: [u8;8], pub io_info: mspro_io_info_register, pub io_func: mspro_io_func_register, pub reserved2: [u8;7], pub io_cmd: mspro_io_cmd_register, pub io_int: u8, pub io_int_func: u8 }
#[repr(C, packed)] pub struct ms_register_addr { pub r_offset: u8, pub r_length: u8, pub w_offset: u8, pub w_length: u8 }

pub const MS_TPC_READ_MG_STATUS: u8=1; pub const MS_TPC_READ_LONG_DATA:u8=2; pub const MS_TPC_READ_SHORT_DATA:u8=3; pub const MS_TPC_READ_MG_DATA:u8=3; pub const MS_TPC_READ_REG:u8=4; pub const MS_TPC_READ_QUAD_DATA:u8=5; pub const MS_TPC_READ_IO_DATA:u8=5; pub const MS_TPC_GET_INT:u8=7; pub const MS_TPC_SET_RW_REG_ADRS:u8=8; pub const MS_TPC_EX_SET_CMD:u8=9; pub const MS_TPC_WRITE_QUAD_DATA:u8=0xa; pub const MS_TPC_WRITE_IO_DATA:u8=0xa; pub const MS_TPC_WRITE_REG:u8=0xb; pub const MS_TPC_WRITE_SHORT_DATA:u8=0xc; pub const MS_TPC_WRITE_MG_DATA:u8=0xc; pub const MS_TPC_WRITE_LONG_DATA:u8=0xd; pub const MS_TPC_SET_CMD:u8=0xe;
pub const MS_CMD_BLOCK_END:u8=0x33; pub const MS_CMD_RESET:u8=0x3c; pub const MS_CMD_BLOCK_WRITE:u8=0x55; pub const MS_CMD_SLEEP:u8=0x5a; pub const MS_CMD_BLOCK_ERASE:u8=0x99; pub const MS_CMD_BLOCK_READ:u8=0xaa; pub const MS_CMD_CLEAR_BUF:u8=0xc3; pub const MS_CMD_FLASH_STOP:u8=0xcc; pub const MS_CMD_LOAD_ID:u8=0x60; pub const MS_CMD_CMP_ICV:u8=0x7f;
pub const MSPRO_CMD_FORMAT:u8=0x10; pub const MSPRO_CMD_SLEEP:u8=0x11; pub const MSPRO_CMD_WAKEUP:u8=0x12; pub const MSPRO_CMD_READ_DATA:u8=0x20; pub const MSPRO_CMD_WRITE_DATA:u8=0x21; pub const MSPRO_CMD_READ_ATRB:u8=0x24; pub const MSPRO_CMD_STOP:u8=0x25; pub const MSPRO_CMD_ERASE:u8=0x26; pub const MSPRO_CMD_READ_QUAD:u8=0x27; pub const MSPRO_CMD_WRITE_QUAD:u8=0x28; pub const MSPRO_CMD_SET_IBD:u8=0x46; pub const MSPRO_CMD_GET_IBD:u8=0x47; pub const MSPRO_CMD_IN_IO_DATA:u8=0xb0; pub const MSPRO_CMD_OUT_IO_DATA:u8=0xb1; pub const MSPRO_CMD_READ_IO_ATRB:u8=0xb2; pub const MSPRO_CMD_IN_IO_FIFO:u8=0xb3; pub const MSPRO_CMD_OUT_IO_FIFO:u8=0xb4; pub const MSPRO_CMD_IN_IOM:u8=0xb5; pub const MSPRO_CMD_OUT_IOM:u8=0xb6;

pub const MEMSTICK_POWER: u32 = 1; pub const MEMSTICK_INTERFACE: u32 = 2;
pub const MEMSTICK_POWER_OFF:u32=0; pub const MEMSTICK_POWER_ON:u32=1; pub const MEMSTICK_SERIAL:u32=0; pub const MEMSTICK_PAR4:u32=1; pub const MEMSTICK_PAR8:u32=2;
pub const MEMSTICK_MATCH_ALL:u8=1; pub const MEMSTICK_TYPE_LEGACY:u8=0xff; pub const MEMSTICK_TYPE_DUO:u8=0; pub const MEMSTICK_TYPE_PRO:u8=1; pub const MEMSTICK_CATEGORY_STORAGE:u8=0xff; pub const MEMSTICK_CATEGORY_STORAGE_DUO:u8=0; pub const MEMSTICK_CATEGORY_IO:u8=1; pub const MEMSTICK_CATEGORY_IO_PRO:u8=0x10; pub const MEMSTICK_CLASS_FLASH:u8=0xff; pub const MEMSTICK_CLASS_DUO:u8=0; pub const MEMSTICK_CLASS_ROM:u8=1; pub const MEMSTICK_CLASS_RO:u8=2; pub const MEMSTICK_CLASS_WP:u8=3;

// External kernel types and functions are supplied by the surrounding translation.
extern "C" { pub fn memstick_register_driver(drv: *mut memstick_driver) -> i32; pub fn memstick_unregister_driver(drv: *mut memstick_driver); pub fn memstick_alloc_host(extra: u32, dev: *mut device) -> *mut memstick_host; pub fn memstick_add_host(host:*mut memstick_host)->i32; pub fn memstick_remove_host(host:*mut memstick_host); pub fn memstick_free_host(host:*mut memstick_host); pub fn memstick_detect_change(host:*mut memstick_host); pub fn memstick_suspend_host(host:*mut memstick_host); pub fn memstick_resume_host(host:*mut memstick_host); pub fn memstick_init_req_sg(mrq:*mut memstick_request,tpc:u8,sg:*const scatterlist); pub fn memstick_init_req(mrq:*mut memstick_request,tpc:u8,buf:*const core::ffi::c_void,length:usize); pub fn memstick_next_req(host:*mut memstick_host,mrq:*mut *mut memstick_request)->i32; pub fn memstick_new_req(host:*mut memstick_host); pub fn memstick_set_rw_addr(card:*mut memstick_dev)->i32; }

#[repr(C)] pub struct memstick_device_id { pub match_flags:u8, pub type_:u8, pub category:u8, pub class:u8 }
#[repr(C)] pub struct memstick_request { pub tpc:u8, pub flags:u8, pub int_reg:u8, pub error:i32, pub data: memstick_request_data }
#[repr(C)] pub union memstick_request_data { pub sg: scatterlist, pub inline_data: memstick_inline_data }
#[repr(C)] pub struct memstick_inline_data { pub data_len:u8, pub data:[u8;15] }
#[repr(C)] pub struct memstick_dev { pub id:memstick_device_id, pub host:*mut memstick_host, pub reg_addr:ms_register_addr, pub mrq_complete:completion, pub current_mrq:memstick_request, pub check:Option<unsafe extern "C" fn(*mut memstick_dev)->i32>, pub next_request:Option<unsafe extern "C" fn(*mut memstick_dev,*mut *mut memstick_request)->i32>, pub stop:Option<unsafe extern "C" fn(*mut memstick_dev)>, pub start:Option<unsafe extern "C" fn(*mut memstick_dev)>, pub dev:device }
#[repr(C)] pub struct memstick_host { pub lock:mutex, pub id:u32, pub caps:u32, pub media_checker:work_struct, pub dev:device, pub card:*mut memstick_dev, pub retries:u32, pub removing:bool, pub request:Option<unsafe extern "C" fn(*mut memstick_host)>, pub set_param:Option<unsafe extern "C" fn(*mut memstick_host,u32,i32)->i32>, pub private:[usize;0] }
#[repr(C)] pub struct memstick_driver { pub id_table:*const memstick_device_id, pub probe:Option<unsafe extern "C" fn(*mut memstick_dev)->i32>, pub remove:Option<unsafe extern "C" fn(*mut memstick_dev)>, pub suspend:Option<unsafe extern "C" fn(*mut memstick_dev,pm_message_t)->i32>, pub resume:Option<unsafe extern "C" fn(*mut memstick_dev)->i32>, pub driver:device_driver }

pub const MEMSTICK_CAP_AUTO_GET_INT:u32=1; pub const MEMSTICK_CAP_PAR4:u32=2; pub const MEMSTICK_CAP_PAR8:u32=4;

#[repr(C)] pub struct scatterlist { _private: [u8; 0] }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_driver { _private: [u8; 0] }
pub type pm_message_t = i32;

extern "C" {
    pub fn dev_get_drvdata(dev: *const device) -> *mut core::ffi::c_void;
    pub fn dev_set_drvdata(dev: *mut device, data: *mut core::ffi::c_void);
}

#[inline]
pub unsafe fn memstick_priv(host: *mut memstick_host) -> *mut core::ffi::c_void {
    (*host).private.as_mut_ptr() as *mut core::ffi::c_void
}
#[inline]
pub unsafe fn memstick_get_drvdata(card: *mut memstick_dev) -> *mut core::ffi::c_void {
    dev_get_drvdata(&(*card).dev)
}
#[inline]
pub unsafe fn memstick_set_drvdata(card: *mut memstick_dev, data: *mut core::ffi::c_void) {
    dev_set_drvdata(&mut (*card).dev, data)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
