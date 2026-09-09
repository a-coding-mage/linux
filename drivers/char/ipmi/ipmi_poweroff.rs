// SPDX-License-Identifier: GPL-2.0+
/*
 * ipmi_poweroff.c
 *
 * MontaVista IPMI Poweroff extension to sys_reboot
 */

// C dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_void};

const IPMI_CHASSIS_POWER_DOWN: u8 = 0;
const IPMI_CHASSIS_POWER_CYCLE: u8 = 0x02;

static mut POWEROFF_POWERCYCLE: c_int = 0;
static mut IFNUM_TO_USE: c_int = -1;
static mut READY: c_int = 0;
static mut IPMI_USER: *mut ipmi_user = core::ptr::null_mut();
static mut IPMI_IFNUM: c_int = 0;
static mut SPECIFIC_POWEROFF_FUNC: Option<unsafe extern "C" fn(*mut ipmi_user)> = None;
static mut OLD_POWEROFF_FUNC: Option<unsafe extern "C" fn()> = None;
static mut MFG_ID: u32 = 0;
static mut PROD_ID: u32 = 0;
static mut CAPABILITIES: u8 = 0;
static mut IPMI_VERSION: u8 = 0;
static mut DUMMY_COUNT: atomic_t = atomic_t { counter: 0 };
static mut ATCA_OEM_POWEROFF_HOOK: Option<unsafe extern "C" fn(*mut ipmi_user)> = None;

#[repr(C)] pub struct ipmi_user { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct ipmi_addr { pub addr_type: u16, pub channel: u8, pub lun: u8 }
#[repr(C)] pub struct ipmi_system_interface_addr { pub addr_type: u16, pub channel: u8, pub lun: u8 }
#[repr(C)] pub struct ipmi_ipmb_addr { pub addr_type: u16, pub channel: u8, pub slave_addr: u8, pub lun: u8 }
#[repr(C)] pub struct kernel_ipmi_msg { pub netfn: u8, pub cmd: u8, pub data: *mut u8, pub data_len: usize }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { pub counter: c_int }
#[repr(C)] pub struct ipmi_recv_msg { pub user_msg_data: *mut c_void, pub msg: ipmi_msg }
#[repr(C)] pub struct ipmi_smi_msg { _private: [u8; 0] }
#[repr(C)] pub struct ipmi_msg { pub data: *mut u8, pub data_len: usize }
#[repr(C)] pub struct ipmi_user_hndl { pub ipmi_recv_hndl: Option<unsafe extern "C" fn(*mut ipmi_recv_msg, *mut c_void)> }
#[repr(C)] pub struct ipmi_smi_watcher { pub owner: *mut c_void, pub new_smi: Option<unsafe extern "C" fn(c_int, *mut device)>, pub smi_gone: Option<unsafe extern "C" fn(c_int)> }

extern "C" {
    fn param_set_int(*const c_char, *const c_void) -> c_int;
    fn ipmi_request_supply_msgs(*mut ipmi_user, *mut ipmi_addr, c_int, *mut kernel_ipmi_msg, *mut completion, *mut ipmi_smi_msg, *mut ipmi_recv_msg, c_int) -> c_int;
    fn ipmi_poll_interface(*mut ipmi_user);
    fn init_completion(*mut completion);
    fn wait_for_completion(*mut completion);
    fn complete(*mut completion);
    fn atomic_set(v: *mut atomic_t, n: c_int);
    fn atomic_read(v: *const atomic_t) -> c_int;
    fn atomic_dec(v: *mut atomic_t);
    fn cpu_relax();
    fn ipmi_create_user(c_int, *const ipmi_user_hndl, *mut c_void, *mut *mut ipmi_user) -> c_int;
    fn ipmi_destroy_user(*mut ipmi_user);
    fn ipmi_smi_watcher_register(*mut ipmi_smi_watcher) -> c_int;
    fn ipmi_smi_watcher_unregister(*mut ipmi_smi_watcher);
    fn memcmp(*const c_void, *const c_void, usize) -> c_int;
}

const IPMI_SYSTEM_INTERFACE_ADDR_TYPE: u16 = 0x0c;
const IPMI_BMC_CHANNEL: u8 = 0x0e;
const IPMI_UNKNOWN_ERR_COMPLETION_CODE: c_int = 0xff;
const IPMI_NETFN_ATCA: u8 = 0x2c;
const IPMI_ATCA_SET_POWER_CMD: u8 = 0x11;
const IPMI_ATCA_GET_ADDR_INFO_CMD: u8 = 0x01;
const IPMI_PICMG_ID: u8 = 0;
const IPMI_NETFN_OEM: u8 = 0x2e;
const IPMI_ATCA_PPS_GRACEFUL_RESTART: u8 = 0x11;
const IPMI_ATCA_PPS_IANA: [u8; 3] = [0, 0x40, 0x0a];
const IPMI_MOTOROLA_MANUFACTURER_ID: u32 = 0x0000a1;
const IPMI_MOTOROLA_PPS_IPMC_PRODUCT_ID: u32 = 0x0051;
const IPMI_NETFN_OEM_1: u8 = 0xf8;
const OEM_GRP_CMD_SET_RESET_STATE: u8 = 0x84;
const OEM_GRP_CMD_SET_POWER_STATE: u8 = 0x82;
const IPMI_NETFN_OEM_8: u8 = 0xf8;
const OEM_GRP_CMD_REQUEST_HOTSWAP_CTRL: u8 = 0x80;
const OEM_GRP_CMD_GET_SLOT_GA: u8 = 0xa3;
const IPMI_NETFN_SENSOR_EVT: u8 = 0x10;
const IPMI_CMD_GET_EVENT_RECEIVER: u8 = 0x01;
const IPMI_CPI1_PRODUCT_ID: u32 = 0x000157;
const IPMI_CPI1_MANUFACTURER_ID: u32 = 0x0108;
const IPMI_IPMB_ADDR_TYPE: u16 = 0x01;
const IPMI_NETFN_CHASSIS_REQUEST: u8 = 0;
const IPMI_CHASSIS_CONTROL_CMD: u8 = 0x02;
const IPMI_NETFN_APP_REQUEST: u8 = 0x06;
const IPMI_GET_DEVICE_ID_CMD: u8 = 1;

static mut HALT_SMI_MSG: ipmi_smi_msg = ipmi_smi_msg { _private: [] };
static mut HALT_RECV_MSG: ipmi_recv_msg = ipmi_recv_msg { user_msg_data: core::ptr::null_mut(), msg: ipmi_msg { data: core::ptr::null_mut(), data_len: 0 } };

unsafe extern "C" fn receive_handler(recv_msg: *mut ipmi_recv_msg, _handler_data: *mut c_void) { let comp = (*recv_msg).user_msg_data as *mut completion; if !comp.is_null() { complete(comp); } }
static IPMI_POWEROFF_HANDLER: ipmi_user_hndl = ipmi_user_hndl { ipmi_recv_hndl: Some(receive_handler) };

unsafe extern "C" fn dummy_smi_free(_msg: *mut ipmi_smi_msg) { atomic_dec(&raw mut DUMMY_COUNT); }
unsafe extern "C" fn dummy_recv_free(_msg: *mut ipmi_recv_msg) { atomic_dec(&raw mut DUMMY_COUNT); }

unsafe fn ipmi_request_wait_for_response(user: *mut ipmi_user, addr: *mut ipmi_addr, send_msg: *mut kernel_ipmi_msg) -> c_int {
    let mut comp = core::mem::MaybeUninit::<completion>::uninit(); init_completion(comp.as_mut_ptr());
    let rv = ipmi_request_supply_msgs(user, addr, 0, send_msg, comp.as_mut_ptr(), &raw mut HALT_SMI_MSG, &raw mut HALT_RECV_MSG, 0); if rv != 0 { return rv; }
    wait_for_completion(comp.as_mut_ptr()); (*HALT_RECV_MSG).msg.data.read() as c_int
}
unsafe fn ipmi_request_in_rc_mode(user: *mut ipmi_user, addr: *mut ipmi_addr, send_msg: *mut kernel_ipmi_msg) -> c_int {
    atomic_set(&raw mut DUMMY_COUNT, 2); let rv = ipmi_request_supply_msgs(user, addr, 0, send_msg, core::ptr::null_mut(), &raw mut HALT_SMI_MSG, &raw mut HALT_RECV_MSG, 0); if rv != 0 { atomic_set(&raw mut DUMMY_COUNT, 0); return rv; }
    while atomic_read(&raw const DUMMY_COUNT) > 0 { ipmi_poll_interface(user); cpu_relax(); } (*HALT_RECV_MSG).msg.data.read() as c_int
}

unsafe extern "C" fn pps_poweroff_atca(user: *mut ipmi_user) { let mut a = ipmi_system_interface_addr { addr_type: IPMI_SYSTEM_INTERFACE_ADDR_TYPE, channel: IPMI_BMC_CHANNEL, lun: 0 }; let mut d = IPMI_ATCA_PPS_IANA; let mut m = kernel_ipmi_msg { netfn: IPMI_NETFN_OEM, cmd: IPMI_ATCA_PPS_GRACEFUL_RESTART, data: d.as_mut_ptr(), data_len: 3 }; let rv = ipmi_request_in_rc_mode(user, &mut a as *mut _ as *mut ipmi_addr, &mut m); let _ = rv; }
unsafe extern "C" fn ipmi_atca_detect(_user: *mut ipmi_user) -> c_int { if MFG_ID == IPMI_MOTOROLA_MANUFACTURER_ID && PROD_ID == IPMI_MOTOROLA_PPS_IPMC_PRODUCT_ID { ATCA_OEM_POWEROFF_HOOK = Some(pps_poweroff_atca); } 1 }
unsafe extern "C" fn ipmi_poweroff_atca(user: *mut ipmi_user) { let mut a = ipmi_system_interface_addr { addr_type: IPMI_SYSTEM_INTERFACE_ADDR_TYPE, channel: IPMI_BMC_CHANNEL, lun: 0 }; let mut d = [0,0,0,0]; let mut m = kernel_ipmi_msg { netfn: IPMI_NETFN_ATCA, cmd: IPMI_ATCA_SET_POWER_CMD, data: d.as_mut_ptr(), data_len: 4 }; let rv = ipmi_request_in_rc_mode(user, &mut a as *mut _ as *mut ipmi_addr, &mut m); if rv == 0 || rv == IPMI_UNKNOWN_ERR_COMPLETION_CODE { if let Some(f) = ATCA_OEM_POWEROFF_HOOK { f(user); } } }

unsafe extern "C" fn ipmi_cpi1_detect(_user: *mut ipmi_user) -> c_int { (MFG_ID == IPMI_CPI1_MANUFACTURER_ID && PROD_ID == IPMI_CPI1_PRODUCT_ID) as c_int }
unsafe extern "C" fn ipmi_poweroff_cpi1(user: *mut ipmi_user) { let mut a = ipmi_system_interface_addr { addr_type: IPMI_SYSTEM_INTERFACE_ADDR_TYPE, channel: IPMI_BMC_CHANNEL, lun: 0 }; let mut m = kernel_ipmi_msg { netfn: IPMI_NETFN_OEM_8 >> 2, cmd: OEM_GRP_CMD_GET_SLOT_GA, data: core::ptr::null_mut(), data_len: 0 }; if ipmi_request_in_rc_mode(user, &mut a as *mut _ as *mut ipmi_addr, &mut m) != 0 { return; } let slot = (*HALT_RECV_MSG).msg.data.add(1).read(); let hotswap = if slot > 9 { 0xb0u8.wrapping_add(2*slot) } else { 0xaeu8.wrapping_add(2*slot) }; m.netfn = IPMI_NETFN_SENSOR_EVT >> 2; m.cmd = IPMI_CMD_GET_EVENT_RECEIVER; if ipmi_request_in_rc_mode(user, &mut a as *mut _ as *mut ipmi_addr, &mut m) != 0 { return; } let mut ipmb = ipmi_ipmb_addr { addr_type: IPMI_IPMB_ADDR_TYPE, channel: 0, slave_addr: (*HALT_RECV_MSG).msg.data.add(1).read(), lun: (*HALT_RECV_MSG).msg.data.add(2).read() }; m.netfn = IPMI_NETFN_OEM_8 >> 2; m.cmd = OEM_GRP_CMD_REQUEST_HOTSWAP_CTRL; m.data = &hotswap as *const u8 as *mut u8; m.data_len = 1; ipmi_request_in_rc_mode(user, &mut ipmb as *mut _ as *mut ipmi_addr, &mut m); let mut d = [1]; m.netfn = IPMI_NETFN_OEM_1 >> 2; m.cmd = OEM_GRP_CMD_SET_RESET_STATE; m.data = d.as_mut_ptr(); if ipmi_request_in_rc_mode(user, &mut a as *mut _ as *mut ipmi_addr, &mut m) != 0 { return; } m.cmd = OEM_GRP_CMD_SET_POWER_STATE; ipmi_request_in_rc_mode(user, &mut a as *mut _ as *mut ipmi_addr, &mut m); }

unsafe extern "C" fn ipmi_dell_chassis_detect(_user: *mut ipmi_user) -> c_int { let major = IPMI_VERSION & 0xf; let minor = (IPMI_VERSION >> 4) & 0xf; let mfr = [0xa2, 0x02, 0]; (!memcmp(mfr.as_ptr() as *const c_void, &raw const MFG_ID as *const _ as *const c_void, 3) != 0 && major <= 1 && minor < 5) as c_int }
unsafe extern "C" fn ipmi_hp_chassis_detect(_user: *mut ipmi_user) -> c_int { (MFG_ID == 0x0b && PROD_ID == 0x8201 && IPMI_VERSION == 1) as c_int }
unsafe extern "C" fn ipmi_chassis_detect(_user: *mut ipmi_user) -> c_int { (CAPABILITIES & 0x80) as c_int }
unsafe extern "C" fn ipmi_poweroff_chassis(user: *mut ipmi_user) { let mut a = ipmi_system_interface_addr { addr_type: IPMI_SYSTEM_INTERFACE_ADDR_TYPE, channel: IPMI_BMC_CHANNEL, lun: 0 }; let mut d = [if POWEROFF_POWERCYCLE != 0 { IPMI_CHASSIS_POWER_CYCLE } else { IPMI_CHASSIS_POWER_DOWN }]; loop { let mut m = kernel_ipmi_msg { netfn: 0, cmd: IPMI_CHASSIS_CONTROL_CMD, data: d.as_mut_ptr(), data_len: 1 }; let rv = ipmi_request_in_rc_mode(user, &mut a as *mut _ as *mut ipmi_addr, &mut m); if rv == 0 { break; } if POWEROFF_POWERCYCLE != 0 { POWEROFF_POWERCYCLE = 0; continue; } break; } }

#[repr(C)] struct poweroff_function { platform_type: *const c_char, detect: unsafe extern "C" fn(*mut ipmi_user)->c_int, poweroff_func: unsafe extern "C" fn(*mut ipmi_user) }
static mut POWEROFF_FUNCTIONS: [poweroff_function; 5] = [poweroff_function{platform_type:b"ATCA\0".as_ptr() as _,detect:ipmi_atca_detect,poweroff_func:ipmi_poweroff_atca},poweroff_function{platform_type:b"CPI1\0".as_ptr() as _,detect:ipmi_cpi1_detect,poweroff_func:ipmi_poweroff_cpi1},poweroff_function{platform_type:b"chassis\0".as_ptr() as _,detect:ipmi_dell_chassis_detect,poweroff_func:ipmi_poweroff_chassis},poweroff_function{platform_type:b"chassis\0".as_ptr() as _,detect:ipmi_hp_chassis_detect,poweroff_func:ipmi_poweroff_chassis},poweroff_function{platform_type:b"chassis\0".as_ptr() as _,detect:ipmi_chassis_detect,poweroff_func:ipmi_poweroff_chassis}];
unsafe extern "C" fn ipmi_poweroff_function() { if READY != 0 { if let Some(f)=SPECIFIC_POWEROFF_FUNC { f(IPMI_USER); } } }
unsafe extern "C" fn ipmi_po_new_smi(if_num:c_int,_device:*mut device) { if READY!=0 || (IFNUM_TO_USE>=0 && IFNUM_TO_USE!=if_num) { return; } if ipmi_create_user(if_num,&IPMI_POWEROFF_HANDLER,core::ptr::null_mut(),&raw mut IPMI_USER)!=0{return;} IPMI_IFNUM=if_num; for f in &POWEROFF_FUNCTIONS { if (f.detect)(IPMI_USER)!=0 { SPECIFIC_POWEROFF_FUNC=Some(f.poweroff_func); READY=1; return; } } ipmi_destroy_user(IPMI_USER); }
unsafe extern "C" fn ipmi_po_smi_gone(if_num:c_int) { if READY!=0 && IPMI_IFNUM==if_num { READY=0; ipmi_destroy_user(IPMI_USER); if let Some(f)=OLD_POWEROFF_FUNC { f(); } } }
static mut SMI_WATCHER: ipmi_smi_watcher = ipmi_smi_watcher { owner: core::ptr::null_mut(), new_smi: Some(ipmi_po_new_smi), smi_gone: Some(ipmi_po_smi_gone) };

unsafe extern "C" fn ipmi_poweroff_init() -> c_int { ipmi_smi_watcher_register(&raw mut SMI_WATCHER) }
#[cfg(feature="module")] unsafe extern "C" fn ipmi_poweroff_cleanup() { ipmi_smi_watcher_unregister(&raw mut SMI_WATCHER); if READY!=0 { ipmi_destroy_user(IPMI_USER); if let Some(f)=OLD_POWEROFF_FUNC { f(); } } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
