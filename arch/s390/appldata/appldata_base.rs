// SPDX-License-Identifier: GPL-2.0
/* Base infrastructure for Linux-z/VM Monitor Stream, Stage 1. */

use core::ffi::c_void;
use core::mem::{size_of, zeroed};
use core::ptr;

const APPLDATA_CPU_INTERVAL: i32 = 10000;
const TOD_MICRO: u64 = 0x01000;
const APPLDATA_ADD_TIMER: i32 = 0;
const APPLDATA_DEL_TIMER: i32 = 1;
const APPLDATA_MOD_TIMER: i32 = 2;

extern "C" {
    static mut appldata_timer_lock: spinlock_t;
    static mut appldata_ops_mutex: mutex;
    static mut appldata_ops_list: list_head;
    static mut appldata_timer: vtimer_list;
    static mut appldata_interval: i32;
    static mut appldata_timer_active: i32;
    static mut appldata_wq: *mut workqueue_struct;
    static mut appldata_work: work_struct;
    static appldata_proc_name: [u8; APPLDATA_PROC_NAME_LENGTH];
    static mut nr_threads: c_int;
    static mut nr_running: c_int;
    static mut nr_iowait: c_int;

    fn queue_work(wq: *mut workqueue_struct, work: *mut work_struct) -> c_int;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn list_entry<T>(ptr: *mut list_head, member: usize) -> *mut T;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn kmalloc(size: usize, flags: gfp_t) -> *mut c_void;
    fn kmemdup(src: *const c_void, size: usize, flags: gfp_t) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn appldata_asm(parm: *mut appldata_parameter_list, id: *mut appldata_product_id,
                    function: u16, buffer: *mut c_void, length: u16) -> c_int;
    fn add_virt_timer_periodic(timer: *mut vtimer_list);
    fn del_virt_timer(timer: *mut vtimer_list);
    fn mod_virt_timer_periodic(timer: *mut vtimer_list, interval: u64);
    fn proc_douintvec_minmax(ctl: *mut ctl_table, write: c_int, buffer: *mut c_void,
                             lenp: *mut usize, ppos: *mut loff_t) -> c_int;
    fn proc_dointvec_minmax(ctl: *mut ctl_table, write: c_int, buffer: *mut c_void,
                            lenp: *mut usize, ppos: *mut loff_t) -> c_int;
    fn try_module_get(owner: *mut module) -> bool;
    fn module_put(owner: *mut module);
    fn register_sysctl_sz(name: *const u8, table: *mut ctl_table, n: usize) -> *mut c_void;
    fn register_sysctl(name: *const u8, table: *const ctl_table) -> *mut c_void;
    fn unregister_sysctl_table(header: *mut c_void);
    fn alloc_ordered_workqueue(name: *const u8, flags: u32) -> *mut workqueue_struct;
    fn init_virt_timer(timer: *mut vtimer_list);
}

type c_int = i32;
type loff_t = i64;
type gfp_t = u32;
const GFP_KERNEL: gfp_t = 0;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const APPLDATA_START_INTERVAL_REC: u16 = 0;
const APPLDATA_STOP_REC: u16 = 0;
const APPLDATA_MAX_REC_SIZE: usize = 4096;
const APPLDATA_PROC_NAME_LENGTH: usize = 8;
const S_IRUGO: u32 = 0o444;
const S_IWUSR: u32 = 0o200;
const SYSCTL_ZERO: *mut c_void = ptr::null_mut();
const SYSCTL_ONE: *mut c_void = ptr::null_mut();

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { pub data: usize }
#[repr(C)] pub struct vtimer_list { pub expires: u64, pub function: Option<unsafe extern "C" fn(usize)>, pub data: usize }
#[repr(C)] pub struct appldata_parameter_list { _private: [u8; 0] }
#[repr(C)] pub struct appldata_product_id { pub prod_nr: [u8; 7], pub prod_fn: u16, pub version_nr: u16, pub release_nr: u16, pub record_nr: i8, pub mod_lvl: u16 }
#[repr(C)] pub struct ctl_table { pub procname: *const u8, pub mode: u32, pub proc_handler: Option<unsafe extern "C" fn(*const ctl_table, c_int, *mut c_void, *mut usize, *mut loff_t) -> c_int>, pub data: *mut c_void, pub maxlen: usize, pub extra1: *mut c_void, pub extra2: *mut c_void }
#[repr(C)] pub struct appldata_ops { pub list: list_head, pub active: c_int, pub callback: Option<unsafe extern "C" fn(*mut c_void)>, pub data: *mut c_void, pub owner: *mut module, pub record_nr: i8, pub size: u16, pub mod_lvl: *mut u8, pub name: *const u8, pub ctl_table: *mut ctl_table, pub sysctl_header: *mut c_void }

static mut APPLDATA_ID: appldata_product_id = appldata_product_id { prod_nr: [0xD3,0xC9,0xD5,0xE4,0xE7,0xD2,0xD9], prod_fn: 0xD5D3, version_nr: 0xF2F6, release_nr: 0xF0F1, record_nr: 0, mod_lvl: 0 };

#[no_mangle] pub unsafe extern "C" fn appldata_diag(record_nr: i8, function: u16, buffer: usize, length: u16, mod_lvl: *mut u8) -> c_int {
    let parm_list = kmalloc(size_of::<appldata_parameter_list>(), GFP_KERNEL) as *mut appldata_parameter_list;
    let id = kmemdup(&APPLDATA_ID as *const _ as *const c_void, size_of::<appldata_product_id>(), GFP_KERNEL) as *mut appldata_product_id;
    let mut rc = -ENOMEM;
    if !parm_list.is_null() && !id.is_null() {
        (*id).record_nr = record_nr;
        (*id).mod_lvl = ((*mod_lvl as u16) << 8) | *mod_lvl.add(1) as u16;
        rc = appldata_asm(parm_list, id, function, buffer as *mut c_void, length);
    }
    kfree(id as *mut c_void); kfree(parm_list as *mut c_void); rc
}

unsafe extern "C" fn appldata_timer_function(data: usize) { queue_work(appldata_wq, data as *mut work_struct); }
unsafe extern "C" fn appldata_work_fn(_work: *mut work_struct) {
    mutex_lock(&mut appldata_ops_mutex);
    let mut lh = appldata_ops_list.next;
    while lh != &mut appldata_ops_list {
        let ops = lh as *mut appldata_ops;
        if (*ops).active == 1 { if let Some(cb) = (*ops).callback { cb((*ops).data); } }
        lh = (*lh).next;
    }
    mutex_unlock(&mut appldata_ops_mutex);
}

unsafe fn __appldata_vtimer_setup(cmd: c_int) {
    let timer_interval = (appldata_interval as u64).wrapping_mul(1000).wrapping_mul(TOD_MICRO);
    match cmd {
        APPLDATA_ADD_TIMER if appldata_timer_active == 0 => { appldata_timer.expires = timer_interval; add_virt_timer_periodic(&mut appldata_timer); appldata_timer_active = 1; }
        APPLDATA_DEL_TIMER => { del_virt_timer(&mut appldata_timer); if appldata_timer_active != 0 { appldata_timer_active = 0; } }
        APPLDATA_MOD_TIMER if appldata_timer_active != 0 => mod_virt_timer_periodic(&mut appldata_timer, timer_interval),
        _ => {}
    }
}

#[no_mangle] pub unsafe extern "C" fn appldata_register_ops(ops: *mut appldata_ops) -> c_int {
    if (*ops).size as usize > APPLDATA_MAX_REC_SIZE { return -EINVAL; }
    (*ops).ctl_table = kmalloc(size_of::<ctl_table>(), GFP_KERNEL) as *mut ctl_table;
    if (*ops).ctl_table.is_null() { return -ENOMEM; }
    mutex_lock(&mut appldata_ops_mutex); list_add(&mut (*ops).list, &mut appldata_ops_list); mutex_unlock(&mut appldata_ops_mutex);
    (*ops).ctl_table = (*ops).ctl_table;
    if register_sysctl_sz(appldata_proc_name.as_ptr(), (*ops).ctl_table, 1).is_null() {
        mutex_lock(&mut appldata_ops_mutex); list_del(&mut (*ops).list); mutex_unlock(&mut appldata_ops_mutex); kfree((*ops).ctl_table as *mut c_void); return -ENOMEM;
    }
    0
}

#[no_mangle] pub unsafe extern "C" fn appldata_unregister_ops(ops: *mut appldata_ops) {
    mutex_lock(&mut appldata_ops_mutex); list_del(&mut (*ops).list); mutex_unlock(&mut appldata_ops_mutex);
    unregister_sysctl_table((*ops).sysctl_header); kfree((*ops).ctl_table as *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
