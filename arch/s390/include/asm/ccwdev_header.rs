/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright IBM Corp. 2002, 2009
 *
 * Author(s): Arnd Bergmann <arndb@de.ibm.com>
 *
 * Interface for CCW device drivers
 */

/* Dependencies supplied by the surrounding kernel translation. */

use core::ffi::c_char;

pub const PE_NONE: u32 = 0x0;
pub const PE_PATH_GONE: u32 = 0x1;
pub const PE_PATH_AVAILABLE: u32 = 0x2;
pub const PE_PATHGROUP_ESTABLISHED: u32 = 0x4;
pub const PE_PATH_FCES_EVENT: u32 = 0x8;

pub const CCWDEV_EARLY_NOTIFICATION: u32 = 0x0001;
pub const CCWDEV_REPORT_ALL: u32 = 0x0002;
pub const CCWDEV_DO_PATHGROUP: u32 = 0x0004;
pub const CCWDEV_ALLOW_FORCE: u32 = 0x0008;
pub const CCWDEV_DO_MULTIPATH: u32 = 0x0010;

/* CCW_DEVICE(cu, cum) and CCW_DEVICE_DEVTYPE(cu, cum, dev, devm). */
pub const CCW_DEVICE_ID_MATCH_CU_TYPE: u16 = 0x01;
pub const CCW_DEVICE_ID_MATCH_CU_MODEL: u16 = 0x02;
pub const CCW_DEVICE_ID_MATCH_DEVICE_TYPE: u16 = 0x04;
pub const CCW_DEVICE_ID_MATCH_DEVICE_MODEL: u16 = 0x08;

#[repr(C)]
pub struct irb {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ccw1 {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ccw_dev_id {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ccw_device_private {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ccw_device_id {
    pub cu_type: u16,
    pub cu_model: u8,
    pub dev_type: u16,
    pub dev_model: u8,
    pub match_flags: u16,
}

#[inline]
pub unsafe fn ccw_device_id_match(array: *const ccw_device_id, r#match: *const ccw_device_id) -> *const ccw_device_id {
    let mut id = array;
    while (*id).match_flags != 0 {
        if ((*id).match_flags & CCW_DEVICE_ID_MATCH_CU_TYPE) != 0 && (*id).cu_type != (*r#match).cu_type { id = id.add(1); continue; }
        if ((*id).match_flags & CCW_DEVICE_ID_MATCH_CU_MODEL) != 0 && (*id).cu_model != (*r#match).cu_model { id = id.add(1); continue; }
        if ((*id).match_flags & CCW_DEVICE_ID_MATCH_DEVICE_TYPE) != 0 && (*id).dev_type != (*r#match).dev_type { id = id.add(1); continue; }
        if ((*id).match_flags & CCW_DEVICE_ID_MATCH_DEVICE_MODEL) != 0 && (*id).dev_model != (*r#match).dev_model { id = id.add(1); continue; }
        return id;
    }
    core::ptr::null()
}
#[repr(C)]
pub struct ccw_driver {
    pub ids: *mut ccw_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut ccw_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut ccw_device)>,
    pub set_online: Option<unsafe extern "C" fn(*mut ccw_device) -> i32>,
    pub set_offline: Option<unsafe extern "C" fn(*mut ccw_device) -> i32>,
    pub notify: Option<unsafe extern "C" fn(*mut ccw_device, i32) -> i32>,
    pub path_event: Option<unsafe extern "C" fn(*mut ccw_device, *mut i32)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut ccw_device)>,
    pub uc_handler: Option<unsafe extern "C" fn(*mut ccw_device, *mut irb) -> uc_todo>,
    pub driver: device_driver,
    pub int_class: interruption_class,
}

#[repr(C)]
pub struct ccw_device {
    pub ccwlock: *mut spinlock_t,
    pub private: *mut ccw_device_private,
    pub reg_mutex: mutex,
    pub id: ccw_device_id,
    pub drv: *mut ccw_driver,
    pub dev: device,
    pub online: i32,
    pub handler: Option<unsafe extern "C" fn(*mut ccw_device, libc::c_ulong, *mut irb)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum uc_todo {
    UC_TODO_RETRY,
    UC_TODO_RETRY_ON_NEW_PATH,
    UC_TODO_STOP,
}

extern "C" {
    pub fn get_ccwdev_by_busid(cdrv: *mut ccw_driver, bus_id: *const c_char) -> *mut ccw_device;
    pub fn ccw_driver_register(driver: *mut ccw_driver) -> i32;
    pub fn ccw_driver_unregister(driver: *mut ccw_driver);
    pub fn ccw_device_set_options_mask(cdev: *mut ccw_device, flags: libc::c_ulong) -> i32;
    pub fn ccw_device_set_options(cdev: *mut ccw_device, flags: libc::c_ulong) -> i32;
    pub fn ccw_device_clear_options(cdev: *mut ccw_device, flags: libc::c_ulong);
    pub fn ccw_device_is_pathgroup(cdev: *mut ccw_device) -> i32;
    pub fn ccw_device_is_multipath(cdev: *mut ccw_device) -> i32;
    pub fn ccw_device_start(cdev: *mut ccw_device, ccp: *mut ccw1, intparm: libc::c_ulong, lpm: u8, flags: libc::c_ulong) -> i32;
    pub fn ccw_device_start_timeout(cdev: *mut ccw_device, ccp: *mut ccw1, intparm: libc::c_ulong, lpm: u8, flags: libc::c_ulong, timeout: i32) -> i32;
    pub fn ccw_device_start_key(cdev: *mut ccw_device, ccp: *mut ccw1, intparm: libc::c_ulong, lpm: u8, key: u8, flags: libc::c_ulong) -> i32;
    pub fn ccw_device_start_timeout_key(cdev: *mut ccw_device, ccp: *mut ccw1, intparm: libc::c_ulong, lpm: u8, key: u8, flags: libc::c_ulong, timeout: i32) -> i32;
    pub fn ccw_device_resume(cdev: *mut ccw_device) -> i32;
    pub fn ccw_device_halt(cdev: *mut ccw_device, flags: libc::c_ulong) -> i32;
    pub fn ccw_device_clear(cdev: *mut ccw_device, flags: libc::c_ulong) -> i32;
    pub fn ccw_device_tm_start_key(cdev: *mut ccw_device, tcw: *mut tcw, intparm: libc::c_ulong, lpm: u8, key: u8) -> i32;
    pub fn ccw_device_tm_start_timeout_key(cdev: *mut ccw_device, tcw: *mut tcw, intparm: libc::c_ulong, lpm: u8, key: u8, timeout: i32) -> i32;
    pub fn ccw_device_tm_start(cdev: *mut ccw_device, tcw: *mut tcw, intparm: libc::c_ulong, lpm: u8) -> i32;
    pub fn ccw_device_tm_start_timeout(cdev: *mut ccw_device, tcw: *mut tcw, intparm: libc::c_ulong, lpm: u8, timeout: i32) -> i32;
    pub fn ccw_device_tm_intrg(cdev: *mut ccw_device) -> i32;
    pub fn ccw_device_get_mdc(cdev: *mut ccw_device, mask: u8) -> i32;
    pub fn ccw_device_set_online(cdev: *mut ccw_device) -> i32;
    pub fn ccw_device_set_offline(cdev: *mut ccw_device) -> i32;
    pub fn ccw_device_create_console(driver: *mut ccw_driver) -> *mut ccw_device;
    pub fn ccw_device_destroy_console(cdev: *mut ccw_device);
    pub fn ccw_device_enable_console(cdev: *mut ccw_device) -> i32;
    pub fn ccw_device_wait_idle(cdev: *mut ccw_device);
    pub fn ccw_device_dma_zalloc(cdev: *mut ccw_device, size: usize, dma_handle: *mut dma32_t) -> *mut core::ffi::c_void;
    pub fn ccw_device_dma_free(cdev: *mut ccw_device, cpu_addr: *mut core::ffi::c_void, size: usize);
    pub fn ccw_device_siosl(cdev: *mut ccw_device) -> i32;
}

/* The remaining declarations retain the C interfaces and dependency types. */
extern "C" {
    pub fn ccw_device_get_ciw(cdev: *mut ccw_device, cmd: u32) -> *mut ciw;
    pub fn ccw_device_get_path_mask(cdev: *mut ccw_device) -> u8;
    pub fn ccw_device_get_id(cdev: *mut ccw_device, id: *mut ccw_dev_id);
    pub fn ccw_device_get_schid(cdev: *mut ccw_device, schid: *mut subchannel_id);
    pub fn ccw_device_get_chp_desc(cdev: *mut ccw_device, chp_idx: i32) -> *mut channel_path_desc_fmt0;
    pub fn ccw_device_get_util_str(cdev: *mut ccw_device, chp_idx: i32) -> *mut u8;
    pub fn ccw_device_pnso(cdev: *mut ccw_device, pnso_area: *mut chsc_pnso_area, oc: u8, resume_token: chsc_pnso_resume_token, cnc: i32) -> i32;
    pub fn ccw_device_get_cssid(cdev: *mut ccw_device, cssid: *mut u8) -> i32;
    pub fn ccw_device_get_iid(cdev: *mut ccw_device, iid: *mut u8) -> i32;
    pub fn ccw_device_get_chpid(cdev: *mut ccw_device, chp_idx: i32, chpid: *mut u8) -> i32;
    pub fn ccw_device_get_chid(cdev: *mut ccw_device, chp_idx: i32, chid: *mut u16) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
