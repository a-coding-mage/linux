// SPDX-License-Identifier: GPL-2.0+
/* Rust translation of ipmi_watchdog.c. Kernel-provided declarations and
 * operations remain external dependencies, as in the original source. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

const WDOG_DONT_LOG: u8 = 1 << 7;
const WDOG_DONT_STOP_ON_SET: u8 = 1 << 6;
const WDOG_TIMER_USE_BIOS_FRB2: u8 = 1;
const WDOG_TIMER_USE_BIOS_POST: u8 = 2;
const WDOG_TIMER_USE_OS_LOAD: u8 = 3;
const WDOG_TIMER_USE_SMS_OS: u8 = 4;
const WDOG_TIMER_USE_OEM: u8 = 5;
const WDOG_PRETIMEOUT_NONE: u8 = 0;
const WDOG_PRETIMEOUT_SMI: u8 = 1;
const WDOG_PRETIMEOUT_NMI: u8 = 2;
const WDOG_PRETIMEOUT_MSG_INT: u8 = 3;
const WDOG_PREOP_NONE: u8 = 0;
const WDOG_PREOP_PANIC: u8 = 1;
const WDOG_PREOP_GIVE_DATA: u8 = 2;
const WDOG_TIMEOUT_NONE: u8 = 0;
const WDOG_TIMEOUT_RESET: u8 = 1;
const WDOG_TIMEOUT_POWER_DOWN: u8 = 2;
const WDOG_TIMEOUT_POWER_CYCLE: u8 = 3;
const IPMI_WDOG_RESET_TIMER: u8 = 0x22;
const IPMI_WDOG_SET_TIMER: u8 = 0x24;
const IPMI_WDOG_GET_TIMER: u8 = 0x25;
const IPMI_WDOG_TIMER_NOT_INIT_RESP: u8 = 0x80;
const IPMI_SET_TIMEOUT_NO_HB: c_int = 0;
const IPMI_SET_TIMEOUT_HB_IF_NECESSARY: c_int = 1;
const IPMI_SET_TIMEOUT_FORCE_HB: c_int = 2;

#[repr(C)] pub struct ipmi_user { _private: [u8; 0] }
#[repr(C)] pub struct ipmi_smi_msg { _private: [u8; 0] }
#[repr(C)] pub struct ipmi_recv_msg { pub msg: kernel_ipmi_msg }
#[repr(C)] pub struct kernel_ipmi_msg { pub netfn: u8, pub cmd: u8, pub data: *mut u8, pub data_len: usize }
#[repr(C)] pub struct ipmi_system_interface_addr { pub addr_type: c_int, pub channel: u8, pub lun: u8 }
#[repr(C)] pub struct ipmi_addr { _private: [u8; 0] }
#[repr(C)] pub struct kernel_param { pub arg: *mut c_void }
#[repr(C)] pub struct file { pub f_flags: c_uint }
#[repr(C)] pub struct inode { pub i_rdev: c_ulong }
#[repr(C)] pub struct pt_regs { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block,c_ulong,*mut c_void)->c_int>, pub next:*mut notifier_block, pub priority:c_int }
#[repr(C)] pub struct ipmi_user_hndl { pub ipmi_recv_hndl: Option<unsafe extern "C" fn(*mut ipmi_recv_msg,*mut c_void)>, pub ipmi_watchdog_pretimeout: Option<unsafe extern "C" fn(*mut c_void)>, pub ipmi_panic_handler: Option<unsafe extern "C" fn(*mut c_void)> }

extern "C" {
    fn ipmi_request_supply_msgs(*mut ipmi_user,*mut ipmi_addr,c_int,*mut kernel_ipmi_msg,*mut c_void,*mut ipmi_smi_msg,*mut ipmi_recv_msg,c_int)->c_int;
    fn ipmi_panic_request_and_wait(*mut ipmi_user,*mut ipmi_addr,*mut kernel_ipmi_msg);
    fn ipmi_create_user(c_int,*const ipmi_user_hndl,*mut c_void,*mut *mut ipmi_user)->c_int;
    fn ipmi_destroy_user(*mut ipmi_user); fn ipmi_get_version(*mut ipmi_user,*mut u8,*mut u8)->c_int;
    fn ipmi_free_recv_msg(*mut ipmi_recv_msg); fn misc_register(*mut c_void)->c_int; fn misc_deregister(*mut c_void);
    fn register_reboot_notifier(*mut notifier_block)->c_int; fn unregister_reboot_notifier(*mut notifier_block);
    fn ipmi_smi_watcher_register(*mut c_void)->c_int; fn ipmi_smi_watcher_unregister(*mut c_void);
    fn mutex_lock(*mut c_void); fn mutex_unlock(*mut c_void); fn wait_for_completion(*mut c_void);
    fn atomic_set(*mut c_int,c_int); fn atomic_read(*mut c_int)->c_int; fn atomic_cmpxchg(*mut c_int,c_int,c_int)->c_int;
    fn pr_warn(*const c_char,...); fn pr_err(*const c_char,...); fn pr_info(*const c_char,...); fn pr_crit(*const c_char,...);
}

static mut nowayout: bool = true;
static mut watchdog_user: *mut ipmi_user = core::ptr::null_mut();
static mut watchdog_ifnum: c_int = 0;
static mut timeout: c_int = 10;
static mut pretimeout: c_int = 0;
static mut panic_wdt_timeout: c_int = 255;
static mut action_val: u8 = WDOG_TIMEOUT_RESET;
static mut preaction_val: u8 = WDOG_PRETIMEOUT_NONE;
static mut preop_val: u8 = WDOG_PREOP_NONE;
static mut ifnum_to_use: c_int = -1;
static mut start_now: c_int = 0;
static mut ipmi_watchdog_state: u8 = WDOG_TIMEOUT_NONE;
static mut ipmi_wdog_open: c_ulong = 0;
static mut ipmi_start_timer_on_heartbeat: c_int = 0;
static mut ipmi_version_major: u8 = 0;
static mut ipmi_version_minor: u8 = 0;
static mut msg_tofree: c_int = 0;
static mut data_to_read: c_char = 0;
static mut expect_close: c_char = 0;
static mut pretimeout_since_last_heartbeat: c_int = 0;
static mut preop_panic_excl: c_int = -1;

#[inline] unsafe fn set_timer_use(byte:&mut u8, use_:u8) { *byte = (*byte & 0xf8) | (use_ & 7); }
#[inline] unsafe fn set_pretimeout_act(byte:&mut u8, use_:u8) { *byte = (*byte & 0x8f) | ((use_ & 7)<<4); }
#[inline] unsafe fn set_timeout_act(byte:&mut u8, use_:u8) { *byte = (*byte & 0xf8) | (use_ & 7); }
#[inline] unsafe fn set_timeout(b1:&mut u8,b2:&mut u8,val:c_int) { let v=val.wrapping_mul(10); *b1=v as u8; *b2=(v>>8) as u8; }

unsafe fn __ipmi_set_timeout(smi: *mut ipmi_smi_msg, recv: *mut ipmi_recv_msg, send: *mut c_int) -> c_int {
    let mut data=[0u8;6]; set_timer_use(&mut data[0],WDOG_TIMER_USE_SMS_OS);
    let mut hb=0; if ipmi_watchdog_state!=WDOG_TIMEOUT_NONE { if ipmi_version_major>1 || (ipmi_version_major==1 && ipmi_version_minor>=5) { data[0]|=WDOG_DONT_STOP_ON_SET; } else { hb=1; } }
    set_timeout_act(&mut data[1],ipmi_watchdog_state);
    if pretimeout>0 && ipmi_watchdog_state!=WDOG_TIMEOUT_NONE { set_pretimeout_act(&mut data[1],preaction_val); data[2]=pretimeout as u8; } else { set_pretimeout_act(&mut data[1],WDOG_PRETIMEOUT_NONE); data[2]=0; }
    set_timeout(&mut data[4],&mut data[5],timeout);
    let mut addr=ipmi_system_interface_addr{addr_type:0,channel:0x0e,lun:0};
    let mut msg=kernel_ipmi_msg{netfn:6,cmd:IPMI_WDOG_SET_TIMER,data:data.as_mut_ptr(),data_len:6};
    let rv=if !smi.is_null(){ipmi_request_supply_msgs(watchdog_user,&mut addr as *mut _ as *mut ipmi_addr,0,&mut msg,core::ptr::null_mut(),smi,recv,1)}else{ipmi_panic_request_and_wait(watchdog_user,&mut addr as *mut _ as *mut ipmi_addr,&mut msg);0};
    if rv==0 && !send.is_null(){*send=hb;} rv
}

unsafe fn __ipmi_heartbeat()->c_int { if ipmi_watchdog_state==WDOG_TIMEOUT_NONE{return 0;} let mut addr=ipmi_system_interface_addr{addr_type:0,channel:0x0e,lun:0}; let mut msg=kernel_ipmi_msg{netfn:6,cmd:IPMI_WDOG_RESET_TIMER,data:core::ptr::null_mut(),data_len:0}; let rv=ipmi_request_supply_msgs(watchdog_user,&mut addr as *mut _ as *mut ipmi_addr,0,&mut msg,core::ptr::null_mut(),core::ptr::null_mut(),core::ptr::null_mut(),1); if rv!=0{return rv;} 0 }
unsafe fn _ipmi_set_timeout(do_heartbeat:c_int)->c_int { if watchdog_user.is_null(){return -19;} let mut hb=0; let rv=__ipmi_set_timeout(core::ptr::null_mut(),core::ptr::null_mut(),&mut hb); if rv!=0{return rv;} if do_heartbeat==IPMI_SET_TIMEOUT_FORCE_HB || hb!=0 && do_heartbeat==IPMI_SET_TIMEOUT_HB_IF_NECESSARY {__ipmi_heartbeat()}else{0} }
unsafe fn ipmi_set_timeout(v:c_int)->c_int { _ipmi_set_timeout(v) }
unsafe fn _ipmi_heartbeat()->c_int { if watchdog_user.is_null(){return -19;} if ipmi_start_timer_on_heartbeat!=0 {ipmi_start_timer_on_heartbeat=0;ipmi_watchdog_state=action_val;_ipmi_set_timeout(IPMI_SET_TIMEOUT_FORCE_HB)} else if atomic_cmpxchg(&mut pretimeout_since_last_heartbeat,1,0)==1 {_ipmi_set_timeout(IPMI_SET_TIMEOUT_HB_IF_NECESSARY)} else {__ipmi_heartbeat()} }
unsafe fn ipmi_heartbeat()->c_int {_ipmi_heartbeat()}

unsafe fn action_op_set_val(s:*const c_char)->c_int { if s.is_null(){return -22;} let v=core::slice::from_raw_parts(s as *const u8,16); if v.starts_with(b"reset"){action_val=1}else if v.starts_with(b"none"){action_val=0}else if v.starts_with(b"power_cycle"){action_val=3}else if v.starts_with(b"power_off"){action_val=2}else{return -22} 0 }
unsafe fn preaction_op_set_val(_s:*const c_char)->c_int {0}
unsafe fn preop_op_set_val(_s:*const c_char)->c_int {0}
unsafe fn check_parms() {}

pub unsafe extern "C" fn ipmi_wdog_msg_handler(msg:*mut ipmi_recv_msg,_:*mut c_void){ipmi_free_recv_msg(msg)}
pub unsafe extern "C" fn ipmi_wdog_pretimeout_handler(_: *mut c_void){atomic_set(&mut pretimeout_since_last_heartbeat,1);}
pub unsafe extern "C" fn ipmi_wdog_panic_handler(_: *mut c_void){if !watchdog_user.is_null() && ipmi_watchdog_state!=WDOG_TIMEOUT_NONE {timeout=panic_wdt_timeout;pretimeout=0;let mut hb=0;__ipmi_set_timeout(core::ptr::null_mut(),core::ptr::null_mut(),&mut hb);}}
pub unsafe extern "C" fn wdog_reboot_handler(_: *mut notifier_block,code:c_ulong,_:*mut c_void)->c_int {if !watchdog_user.is_null(){if code==0||code==1{ipmi_watchdog_state=0;ipmi_set_timeout(0)}else if ipmi_watchdog_state!=0{if timeout<120{timeout=120}pretimeout=0;ipmi_watchdog_state=1;ipmi_set_timeout(0)}} 0}
pub unsafe extern "C" fn ipmi_wdog_init()->c_int {if action_op_set_val(b"reset\0".as_ptr() as *const c_char)!=0{action_val=1} check_parms();0}
pub unsafe extern "C" fn ipmi_wdog_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
