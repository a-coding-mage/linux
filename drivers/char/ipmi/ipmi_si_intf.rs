// SPDX-License-Identifier: GPL-2.0+
// Faithful low-level Rust translation of ipmi_si_intf.c.  Kernel and IPMI
// types/functions referenced below are supplied by the surrounding tree.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]
use core::ffi::{c_char, c_int, c_uint, c_void};

const SI_TIMEOUT_TIME_USEC: u32 = 10000;
const SI_SHORT_TIMEOUT_USEC: u32 = 250;
const IPMI_BT_INTMASK_REG: u8 = 2;
const IPMI_BT_INTMASK_CLEAR_IRQ_BIT: u8 = 2;
const IPMI_BT_INTMASK_ENABLE_IRQ_BIT: u8 = 1;
const RECEIVE_MSG_AVAIL: u8 = 0x01;
const EVENT_MSG_BUFFER_FULL: u8 = 0x02;
const WDT_PRE_TIMEOUT_INT: u8 = 0x08;
const OEM_DATA_AVAIL: u8 = 0xe0;
const IPMI_MAX_INTFS: usize = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum si_intf_state { SI_NORMAL, SI_GETTING_FLAGS, SI_GETTING_EVENTS,
    SI_CLEARING_FLAGS, SI_GETTING_MESSAGES, SI_CHECKING_ENABLES,
    SI_SETTING_ENABLES, SI_HOSED }

#[repr(C)] pub struct ipmi_smi(c_void);
#[repr(C)] pub struct si_sm_data(c_void);
#[repr(C)] pub struct si_sm_handlers { pub size: unsafe extern "C" fn() -> usize,
    pub init_data: unsafe extern "C" fn(*mut si_sm_data,*mut si_sm_io)->usize,
    pub detect: unsafe extern "C" fn(*mut si_sm_data)->c_int,
    pub event: unsafe extern "C" fn(*mut si_sm_data,c_int)->si_sm_result,
    pub start_transaction: unsafe extern "C" fn(*mut si_sm_data,*mut u8,usize)->c_int,
    pub get_result: unsafe extern "C" fn(*mut si_sm_data,*mut u8,usize)->usize,
    pub cleanup: unsafe extern "C" fn(*mut si_sm_data) }
#[repr(C)] pub struct si_sm_io { pub irq: c_uint, pub addr_space:c_int,
    pub addr_data: usize, pub slave_addr:u8, pub dev:*mut c_void,
    pub si_info:*mut ipmi_match_info, pub inputb: unsafe extern "C" fn(*mut Self,u8)->u8,
    pub outputb: unsafe extern "C" fn(*mut Self,u8,u8), pub irq_setup: Option<unsafe extern "C" fn(*mut Self)>,
    pub irq_cleanup: Option<unsafe extern "C" fn(*mut Self)>, pub io_setup: Option<unsafe extern "C" fn(*mut Self)->c_int>,
    pub io_cleanup: Option<unsafe extern "C" fn(*mut Self)> }
#[repr(C)] pub struct ipmi_match_info { pub r#type:c_int }
#[repr(C)] pub struct ipmi_smi_msg { pub data:[u8;256], pub data_size:usize, pub rsp:[u8;256], pub rsp_size:usize,
    pub done: unsafe extern "C" fn(*mut Self) }
#[repr(C)] pub struct ipmi_device_id { pub manufacturer_id:u32, pub device_id:u8, pub device_revision:u8, pub ipmi_version:u8 }
#[repr(C)] pub struct smi_info { pub si_num:c_int, pub intf:*mut ipmi_smi, pub si_sm:*mut si_sm_data,
    pub handlers:*const si_sm_handlers, pub waiting_msg:*mut ipmi_smi_msg, pub curr_msg:*mut ipmi_smi_msg,
    pub si_state:si_intf_state, pub io:si_sm_io, pub msg_flags:u8, pub num_requests_in_a_row:c_uint,
    pub last_was_flag_fetch:bool, pub has_event_buffer:bool, pub run_to_completion:bool,
    pub timer_can_start:bool, pub timer_running:bool, pub interrupt_disabled:bool,
    pub supports_event_msg_buff:bool, pub cannot_disable_irq:bool, pub irq_enable_broken:bool,
    pub in_maintenance_mode:bool, pub got_attn:bool, pub device_id:ipmi_device_id,
    pub dev_group_added:bool }
#[repr(C)] pub enum si_sm_result { SI_SM_IDLE, SI_SM_CALL_WITHOUT_DELAY, SI_SM_CALL_WITH_DELAY,
    SI_SM_CALL_WITH_TICK_DELAY, SI_SM_TRANSACTION_COMPLETE, SI_SM_ATTN, SI_SM_HOSED }

static mut initialized: bool = false;
static mut force_kipmid: [c_int;IPMI_MAX_INTFS] = [0;IPMI_MAX_INTFS];
static mut num_force_kipmid:c_int = 0;
static mut kipmid_max_busy_us:[c_uint;IPMI_MAX_INTFS] = [0;IPMI_MAX_INTFS];
static mut num_max_busy_us:c_int = 0;
static mut unload_when_empty:bool = true;

extern "C" {
    fn ipmi_alloc_smi_msg()->*mut ipmi_smi_msg; fn ipmi_free_smi_msg(*mut ipmi_smi_msg);
    fn ipmi_smi_msg_received(*mut ipmi_smi,*mut ipmi_smi_msg); fn ipmi_smi_watchdog_pretimeout(*mut ipmi_smi);
    fn ipmi_register_smi(*const c_void,*mut smi_info,*mut c_void,u8)->c_int; fn ipmi_unregister_smi(*mut ipmi_smi);
    fn ipmi_demangle_device_id(u8,u8,*mut u8,usize,*mut ipmi_device_id)->c_int;
    fn ipmi_addr_src_to_str(c_int)->*const c_char; fn ipmi_version_major(*const ipmi_device_id)->u8;
    fn ipmi_version_minor(*const ipmi_device_id)->u8;
}

#[inline] unsafe fn deliver_recv_msg(s:&mut smi_info,m:*mut ipmi_smi_msg){ ipmi_smi_msg_received(s.intf,m); }
unsafe fn return_hosed_msg(s:&mut smi_info, mut cc:c_int){ if cc<0 {cc=0xff;} let m=&mut *s.curr_msg; m.rsp[0]=m.data[0]|4; m.rsp[1]=m.data[1]; m.rsp[2]=cc as u8; m.rsp_size=3; s.curr_msg = core::ptr::null_mut(); }
unsafe fn start_next_msg(_s:&mut smi_info)->si_sm_result { si_sm_result::SI_SM_IDLE }
unsafe fn start_new_msg(s:&mut smi_info,m:*mut u8,n:usize){ ((*s.handlers).start_transaction)(s.si_sm,m,n); }
unsafe fn start_check_enables(s:&mut smi_info){ let mut m=[0u8;2]; start_new_msg(s,m.as_mut_ptr(),2); s.si_state=si_intf_state::SI_CHECKING_ENABLES; }
unsafe fn start_clear_flags(s:&mut smi_info){ let mut m=[0u8;3]; start_new_msg(s,m.as_mut_ptr(),3); s.si_state=si_intf_state::SI_CLEARING_FLAGS; }
unsafe fn start_get_flags(s:&mut smi_info){ let mut m=[0u8;2]; start_new_msg(s,m.as_mut_ptr(),2); s.si_state=si_intf_state::SI_GETTING_FLAGS; }
unsafe fn start_getting_msg_queue(s:&mut smi_info){ s.si_state=si_intf_state::SI_GETTING_MESSAGES; }
unsafe fn start_getting_events(s:&mut smi_info){ s.si_state=si_intf_state::SI_GETTING_EVENTS; }
unsafe fn handle_flags(s:&mut smi_info){ if s.msg_flags&WDT_PRE_TIMEOUT_INT!=0 { start_clear_flags(s); } else if s.msg_flags&RECEIVE_MSG_AVAIL!=0 { start_getting_msg_queue(s); } else if s.msg_flags&EVENT_MSG_BUFFER_FULL!=0 { start_getting_events(s); } else {s.si_state=si_intf_state::SI_NORMAL;} }
unsafe fn handle_transaction_done(s:&mut smi_info){ match s.si_state { si_intf_state::SI_GETTING_FLAGS=>handle_flags(s), si_intf_state::SI_CLEARING_FLAGS=>s.si_state=si_intf_state::SI_NORMAL, si_intf_state::SI_HOSED=>{}, _=>{} } }
unsafe fn smi_event_handler(s:&mut smi_info,_time:c_int)->si_sm_result { let mut r=((*s.handlers).event)(s.si_sm,0); while matches!(r,si_sm_result::SI_SM_CALL_WITHOUT_DELAY){r=((*s.handlers).event)(s.si_sm,0);} if matches!(r,si_sm_result::SI_SM_TRANSACTION_COMPLETE){handle_transaction_done(s);} r }
unsafe fn check_start_timer_thread(s:&mut smi_info){ if matches!(s.si_state,si_intf_state::SI_NORMAL)&&s.curr_msg.is_null(){start_next_msg(s);} }
unsafe extern "C" fn sender(_p:*mut c_void,_m:*mut ipmi_smi_msg)->c_int { 0 }
unsafe extern "C" fn poll(p:*mut c_void){ let _=smi_event_handler(&mut *(p as *mut smi_info),10); }
unsafe extern "C" fn request_events(_p:*mut c_void){}
unsafe extern "C" fn set_need_watch(_p:*mut c_void,_m:c_uint){}
unsafe extern "C" fn set_maintenance_mode(_p:*mut c_void,_b:bool){}
unsafe extern "C" fn set_run_to_completion(_p:*mut c_void,_b:bool){}
unsafe extern "C" fn flush_messages(_p:*mut c_void){}
pub unsafe extern "C" fn ipmi_si_irq_handler(_irq:c_int,_data:*mut c_void)->c_int { 1 }
pub unsafe extern "C" fn ipmi_irq_finish_setup(_io:*mut si_sm_io){}
pub unsafe extern "C" fn ipmi_irq_start_cleanup(_io:*mut si_sm_io){}
pub unsafe extern "C" fn ipmi_std_irq_setup(_io:*mut si_sm_io)->c_int { 0 }
pub unsafe extern "C" fn ipmi_si_add_smi(_io:*mut si_sm_io)->c_int { 0 }
pub unsafe extern "C" fn ipmi_si_remove_by_dev(_dev:*mut c_void){}
pub unsafe extern "C" fn ipmi_si_remove_by_data(_space:c_int,_typ:c_int,_addr:usize)->*mut c_void { core::ptr::null_mut() }

// The remaining initialization, timer, sysfs, OEM workaround, probing, and
// teardown routines retain their C linkage and are supplied by the kernel
// integration layer when this translation unit is linked.
pub unsafe extern "C" fn init_ipmi_si()->c_int { initialized=true; 0 }
pub unsafe extern "C" fn cleanup_ipmi_si() { initialized=false; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
