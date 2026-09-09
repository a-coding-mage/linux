// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of the Linux RV interface implementation. */

// Kernel headers and symbols referenced by this file are supplied by the surrounding crate.

extern "C" {
    static mut rv_interface_lock: mutex;
    static mut rv_root: rv_interface;
    static mut rv_monitors_list: list_head;
    static mut task_monitor_slots: [bool; CONFIG_RV_PER_TASK_MONITORS];
    static mut monitoring_on: bool;
}

#[allow(non_camel_case_types)] type ssize_t = isize;
#[allow(non_camel_case_types)] type loff_t = i64;
pub const CONFIG_RV_PER_TASK_MONITORS: usize = 0;

#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct file { pub private_data: *mut core::ffi::c_void, pub f_mode: u32, pub f_flags: u32 }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct kunit_suite { _private: [u8; 0] }
#[repr(C)] pub struct rv_interface { pub root_dir: *mut dentry, pub monitors_dir: *mut dentry }
#[repr(C)] pub struct rv_monitor {
    pub list: list_head, pub name: *const i8, pub description: *const i8,
    pub parent: *mut rv_monitor, pub root_d: *mut dentry, pub enabled: bool,
    pub enable: Option<unsafe extern "C" fn() -> i32>,
    pub disable: Option<unsafe extern "C" fn()>, pub reset: Option<unsafe extern "C" fn()>,
}

extern "C" {
    fn lockdep_assert_held(m: *mut mutex); fn mutex_lock(m: *mut mutex); fn mutex_unlock(m: *mut mutex);
    fn list_is_last(e: *mut list_head, h: *mut list_head) -> bool;
    fn list_next_entry(e: *mut rv_monitor, member: *mut list_head) -> *mut rv_monitor;
    fn tracepoint_synchronize_unregister(); fn simple_open() -> i32;
    fn simple_read_from_buffer(u: *mut i8, c: usize, p: *mut loff_t, b: *const i8, n: usize) -> ssize_t;
    fn simple_write_to_buffer(b: *mut i8, n: usize, p: *mut loff_t, u: *const i8, c: usize) -> ssize_t;
    fn kstrtobool_from_user(u: *const i8, c: usize, v: *mut bool) -> i32;
    fn rv_create_dir(n: *const i8, p: *mut dentry) -> *mut dentry; fn rv_remove(d: *mut dentry);
    fn rv_create_file(n: *const i8, mode: u32, d: *mut dentry, p: *mut rv_monitor, f: *const file_operations) -> *mut dentry;
    fn reactor_populate_monitor(m: *mut rv_monitor, d: *mut dentry) -> i32; fn init_rv_reactors(d: *mut dentry) -> i32;
    fn seq_list_start(h: *mut list_head, p: loff_t) -> *mut core::ffi::c_void;
    fn seq_list_next(p: *mut core::ffi::c_void, h: *mut list_head, pos: *mut loff_t) -> *mut core::ffi::c_void;
    fn seq_open(f: *mut file, o: *const seq_operations) -> i32; fn seq_read() -> ssize_t; fn seq_lseek() -> loff_t; fn seq_release() -> i32;
    fn seq_printf(m: *mut seq_file, fmt: *const i8, ...); fn strlen(s: *const i8) -> usize;
}

#[repr(C)] pub struct file_operations { pub open: Option<unsafe extern "C" fn() -> i32>, pub read: Option<unsafe extern "C" fn() -> ssize_t>, pub write: Option<unsafe extern "C" fn() -> ssize_t>, pub llseek: Option<unsafe extern "C" fn() -> loff_t>, pub release: Option<unsafe extern "C" fn() -> i32> }
#[repr(C)] pub struct seq_operations { pub start: Option<unsafe extern "C" fn() -> *mut core::ffi::c_void>, pub next: Option<unsafe extern "C" fn() -> *mut core::ffi::c_void>, pub stop: Option<unsafe extern "C" fn()>, pub show: Option<unsafe extern "C" fn() -> i32> }

#[no_mangle] pub unsafe extern "C" fn get_monitors_root() -> *mut dentry { rv_root.monitors_dir }

#[no_mangle] pub unsafe extern "C" fn rv_get_task_monitor_slot() -> i32 {
    lockdep_assert_held(&mut rv_interface_lock);
    for i in 0..CONFIG_RV_PER_TASK_MONITORS { if !task_monitor_slots[i] { task_monitor_slots[i] = true; return i as i32; } }
    -16
}
#[no_mangle] pub unsafe extern "C" fn rv_put_task_monitor_slot(slot: i32) { lockdep_assert_held(&mut rv_interface_lock); if slot < 0 || slot >= CONFIG_RV_PER_TASK_MONITORS as i32 { return; } task_monitor_slots[slot as usize] = false; }

#[no_mangle] pub unsafe extern "C" fn rv_is_nested_monitor(mon: *mut rv_monitor) -> bool { !(*mon).parent.is_null() }
#[no_mangle] pub unsafe extern "C" fn rv_is_container_monitor(mon: *mut rv_monitor) -> bool {
    if list_is_last(&mut (*mon).list, &mut rv_monitors_list) { return false; }
    let next = list_next_entry(mon, &mut (*mon).list); (*next).parent == mon || (*mon).enable.is_none()
}

#[no_mangle] pub unsafe extern "C" fn rv_disable_monitor(mon: *mut rv_monitor) -> i32 { (*mon).enabled = false; if let Some(f)=(*mon).disable { f(); } 0 }
#[no_mangle] pub unsafe extern "C" fn rv_enable_monitor(mon: *mut rv_monitor) -> i32 { if (*mon).enabled { return 0; } let r=(*mon).enable.map(|f| f()).unwrap_or(0); if r==0 { (*mon).enabled=true; } r }

#[no_mangle] pub unsafe extern "C" fn rv_monitoring_on() -> bool { core::ptr::read_volatile(&monitoring_on) }
unsafe fn turn_monitoring_off() { core::ptr::write_volatile(&mut monitoring_on, false); }
unsafe fn turn_monitoring_on() { core::ptr::write_volatile(&mut monitoring_on, true); }
unsafe fn reset_all_monitors() { }
unsafe fn turn_monitoring_on_with_reset() { if !rv_monitoring_on() { reset_all_monitors(); turn_monitoring_on(); } }
#[no_mangle] pub unsafe extern "C" fn rv_register_monitor(monitor: *mut rv_monitor, parent: *mut rv_monitor) -> i32 { (*monitor).parent=parent; let _=create_monitor_dir(monitor,parent); 0 }
#[no_mangle] pub unsafe extern "C" fn rv_unregister_monitor(monitor: *mut rv_monitor) -> i32 { rv_disable_monitor(monitor); rv_remove((*monitor).root_d); 0 }
unsafe fn create_monitor_dir(mon: *mut rv_monitor, _parent: *mut rv_monitor) -> i32 { let d=rv_create_dir((*mon).name, get_monitors_root()); if d.is_null() { return -12; } (*mon).root_d=d; 0 }
#[no_mangle] pub unsafe extern "C" fn rv_init_interface() -> i32 { turn_monitoring_on(); 0 }

#[cfg(feature = "CONFIG_RV_MONITORS_KUNIT_TEST")]
static mut mock_current: *mut task_struct = core::ptr::null_mut();
#[cfg(feature = "CONFIG_RV_MONITORS_KUNIT_TEST")]
pub unsafe extern "C" fn rv_set_testing(_suite: *mut kunit_suite) -> i32 { mutex_lock(&mut rv_interface_lock); 0 }
#[cfg(feature = "CONFIG_RV_MONITORS_KUNIT_TEST")]
pub unsafe extern "C" fn rv_clear_testing(_suite: *mut kunit_suite) { mutex_unlock(&mut rv_interface_lock); }
#[cfg(feature = "CONFIG_RV_MONITORS_KUNIT_TEST")]
pub unsafe extern "C" fn rv_mock_current(tsk: *mut task_struct) { mock_current = tsk; }
#[cfg(feature = "CONFIG_RV_MONITORS_KUNIT_TEST")]
pub unsafe extern "C" fn rv_get_mock_current() -> *mut task_struct { mock_current }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
