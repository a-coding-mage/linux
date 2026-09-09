// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of rtasd.c. */

use core::ffi::c_void;

// Kernel and architecture dependencies are supplied by the surrounding tree.
extern "C" {
    static mut rtasd_log_lock: c_void;
    static mut rtas_log_wait: c_void;
    static mut rtas: Rtas;
    static mut cpu_online_mask: c_void;
    static mut nr_cpu_ids: c_uint;
    fn printk(fmt: *const u8, ...);
    fn pr_debug(fmt: *const u8, ...);
    fn pr_info_ratelimited(fmt: *const u8, ...);
    fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void;
    fn memcpy(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;
    fn kmalloc(n: usize, flags: c_uint) -> *mut u8;
    fn kfree(p: *mut u8);
    fn access_ok(p: *const c_void, n: usize) -> bool;
    fn copy_to_user(d: *mut c_void, s: *const c_void, n: usize) -> usize;
    fn vmalloc(n: usize) -> *mut u8;
    fn spin_lock_irqsave(lock: *mut c_void, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut c_void, flags: c_ulong);
    fn wake_up_interruptible(wait: *mut c_void);
    fn wait_event_interruptible(wait: *mut c_void, condition: bool) -> i32;
    fn rtas_set_indicator(token: c_uint, index: c_int, value: c_int) -> c_int;
    fn rtas_call(token: c_uint, nargs: c_int, nret: c_int, ret: *mut c_int, ...) -> c_int;
    fn rtas_function_token(token: c_uint) -> c_uint;
    fn rtas_get_error_log_max() -> usize;
    fn nvram_write_error_log(buf: *const u8, len: i32, typ: c_uint, count: i32);
    fn nvram_clear_error_log();
    fn nvram_read_error_log(buf: *mut u8, len: usize, typ: *mut c_uint, count: *mut i32) -> c_int;
    fn machine_is(name: c_uint) -> bool;
    fn cpus_read_lock();
    fn cpus_read_unlock();
    fn raw_smp_processor_id() -> c_uint;
    fn cpumask_next(n: c_uint, mask: *mut c_void) -> c_uint;
    fn cpumask_first(mask: *mut c_void) -> c_uint;
    fn schedule_delayed_work_on(cpu: c_uint, work: *mut DelayedWork, delay: c_ulong);
    fn cancel_delayed_work_sync(work: *mut DelayedWork);
    fn __round_jiffies_relative(delay: c_ulong, cpu: c_uint) -> c_ulong;
    fn of_property_read_u32(dev: *mut c_void, name: *const u8, value: *mut c_uint) -> c_int;
    fn proc_create(name: *const u8, mode: c_uint, parent: *mut c_void, ops: *const ProcOps) -> *mut c_void;
    fn get_option(s: *mut *mut u8, value: *mut c_int) -> c_int;
    fn kstrtobool(s: *const u8, value: *mut bool) -> c_int;
}

use core::os::raw::{c_int, c_uint, c_ulong};

#[repr(C)] pub struct Rtas { pub dev: *mut c_void }
#[repr(C)] pub struct WorkStruct { _private: [u8; 0] }
#[repr(C)] pub struct DelayedWork { _private: [u8; 0] }
#[repr(C)] pub struct Inode { _private: [u8; 0] }
#[repr(C)] pub struct File { pub f_flags: c_uint }
#[repr(C)] pub struct PollTable { _private: [u8; 0] }
#[repr(C)] pub struct ProcOps { pub proc_read: Option<unsafe extern "C" fn(*mut File,*mut u8,usize,*mut i64)->isize>, pub proc_poll: Option<unsafe extern "C" fn(*mut File,*mut PollTable)->c_uint>, pub proc_open: Option<unsafe extern "C" fn(*mut Inode,*mut File)->c_int>, pub proc_release: Option<unsafe extern "C" fn(*mut Inode,*mut File)->c_int>, pub proc_lseek: *const c_void }
#[repr(C)] pub struct RtasErrorLog { _private: [u8; 0] }

static mut RTAS_LOG_BUF: *mut u8 = core::ptr::null_mut();
static mut RTAS_LOG_START: c_ulong = 0;
static mut RTAS_LOG_SIZE: c_ulong = 0;
static mut SURVEILLANCE_TIMEOUT: c_int = -1;
static mut RTAS_ERROR_LOG_MAX: c_uint = 0;
static mut RTAS_ERROR_LOG_BUFFER_MAX: c_uint = 0;
static mut EVENT_SCAN: c_uint = 0;
static mut RTAS_EVENT_SCAN_RATE: c_uint = 0;
static mut FULL_RTAS_MSGS: bool = false;
static mut LOGGING_ENABLED: c_int = 0;
static mut ERROR_LOG_CNT: c_int = 0;
const RTAS_ERROR_LOG_MAX_CONST: usize = 1024;
static mut LOGDATA: [u8; RTAS_ERROR_LOG_MAX_CONST] = [0; RTAS_ERROR_LOG_MAX_CONST];
static mut RTAS_TYPE: [&[u8]; 11] = [b"Unknown",b"Retry",b"TCE Error",b"Internal Device Failure",b"Timeout",b"Data Parity",b"Address Parity",b"Cache Parity",b"Address Invalid",b"ECC Uncorrected",b"ECC Corrupted"];

extern "C" { fn rtas_error_type(log: *const RtasErrorLog) -> c_int; fn rtas_error_severity(log: *const RtasErrorLog) -> c_int; fn rtas_error_extended(log: *const RtasErrorLog) -> bool; fn rtas_error_extended_log_length(log: *const RtasErrorLog) -> usize; }

unsafe fn rtas_event_type(typ: c_int) -> *const u8 { if typ > 0 && typ < 11 { return RTAS_TYPE[typ as usize].as_ptr(); } RTAS_TYPE[0].as_ptr() }

unsafe fn printk_log_rtas(buf: *mut u8, len: c_int) {
    if FULL_RTAS_MSGS { printk(b"%d -------- RTAS event begin --------\n\0".as_ptr(), ERROR_LOG_CNT); let mut i=0; while i<len { printk(b"%02x\0".as_ptr(), *buf.add(i as usize)); i+=1; } printk(b"\n%d -------- RTAS event end ----------\n\0".as_ptr(), ERROR_LOG_CNT); }
    else { let e=buf as *const RtasErrorLog; printk(b"event: %d, Type: %s (%d), Severity: %d\n\0".as_ptr(), ERROR_LOG_CNT, rtas_event_type(rtas_error_type(e)), rtas_error_type(e), rtas_error_severity(e)); }
}

unsafe fn log_rtas_len(buf: *mut u8) -> usize { let e=buf as *const RtasErrorLog; let ext=if rtas_error_extended(e){rtas_error_extended_log_length(e)}else{0}; let len=core::mem::size_of::<RtasErrorLog>()+ext; if RTAS_ERROR_LOG_MAX==0 { RTAS_ERROR_LOG_MAX=rtas_get_error_log_max() as c_uint; } core::cmp::min(len,RTAS_ERROR_LOG_MAX as usize) }

#[no_mangle] pub unsafe extern "C" fn pSeries_log_error(buf:*mut u8, err_type:c_uint, fatal:c_int) { if buf.is_null(){return} ; let mut s=0; spin_lock_irqsave(&mut rtasd_log_lock,&mut s); let len=if err_type & 0xff == 0 {let x=log_rtas_len(buf); if err_type&0x100==0{ERROR_LOG_CNT+=1}; x}else{spin_unlock_irqrestore(&mut rtasd_log_lock,s);return}; printk_log_rtas(buf,len as c_int); if fatal!=0 || LOGGING_ENABLED==0 {LOGGING_ENABLED=0;spin_unlock_irqrestore(&mut rtasd_log_lock,s);return} let off=RTAS_ERROR_LOG_BUFFER_MAX as usize*((RTAS_LOG_START+RTAS_LOG_SIZE)&63) as usize; memcpy(RTAS_LOG_BUF.add(off) as *mut c_void,&ERROR_LOG_CNT as *const _ as *const c_void,4); memcpy(RTAS_LOG_BUF.add(off+4) as *mut c_void,buf,len); if RTAS_LOG_SIZE<64{RTAS_LOG_SIZE+=1}else{RTAS_LOG_START+=1}; spin_unlock_irqrestore(&mut rtasd_log_lock,s); wake_up_interruptible(&mut rtas_log_wait); }

pub unsafe fn rtas_cancel_event_scan(){cancel_delayed_work_sync(core::ptr::null_mut());}

unsafe fn handle_rtas_event(log:*const RtasErrorLog){ if rtas_error_type(log)==7 { pr_info_ratelimited(b"Platform resource reassignment ignored.\n\0".as_ptr()); } }
unsafe fn rtas_log_open(_: *mut Inode,_: *mut File)->c_int{0}
unsafe fn rtas_log_release(_: *mut Inode,_: *mut File)->c_int{0}
unsafe fn enable_surveillance(timeout:c_int)->c_int{let e=rtas_set_indicator(0,0,timeout);if e==0||e==-22{0}else{-1}}
unsafe fn do_event_scan(){loop{memset(LOGDATA.as_mut_ptr() as *mut c_void,0,RTAS_ERROR_LOG_MAX as usize);let e=rtas_call(EVENT_SCAN,4,1,core::ptr::null_mut(),0,0,LOGDATA.as_ptr(),RTAS_ERROR_LOG_MAX);if e!=0{if e==-1{printk(b"event-scan failed\n\0".as_ptr())};break}let l=LOGDATA.as_ptr() as *const RtasErrorLog;if rtas_error_type(l)!=7{pSeries_log_error(LOGDATA.as_mut_ptr(),0,0)}handle_rtas_event(l);}}
unsafe fn rtas_event_scan(_: *mut WorkStruct){do_event_scan();}
unsafe fn retrieve_nvram_error_log(){}
unsafe fn start_event_scan(){printk(b"RTAS daemon started\n\0".as_ptr());retrieve_nvram_error_log();}
#[no_mangle] pub unsafe extern "C" fn rtas_event_scan_init()->c_int{if EVENT_SCAN==0{return -19} EVENT_SCAN=rtas_function_token(0);RTAS_ERROR_LOG_MAX=rtas_get_error_log_max() as c_uint;RTAS_ERROR_LOG_BUFFER_MAX=RTAS_ERROR_LOG_MAX+4;RTAS_LOG_BUF=vmalloc(64*RTAS_ERROR_LOG_BUFFER_MAX as usize);if RTAS_LOG_BUF.is_null(){return -12}start_event_scan();0}
pub unsafe fn rtas_init()->c_int{if RTAS_LOG_BUF.is_null(){-19}else{0}}
pub unsafe fn surveillance_setup(_: *mut u8)->c_int{1}
pub unsafe fn rtasmsgs_setup(_: *mut u8)->c_int{1}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
