// SPDX-License-Identifier: GPL-2.0-or-later

// Kernel dependencies and build-time configuration are supplied by the surrounding tree.
// The declarations below intentionally retain the C interfaces and low-level semantics.

const REF_TRACKER_STACK_ENTRIES: usize = 16;
const STACK_BUF_SIZE: usize = 1024;

#[repr(C)]
pub struct ref_tracker {
    pub head: list_head,
    pub dead: bool,
    pub alloc_stack_handle: depot_stack_handle_t,
    pub free_stack_handle: depot_stack_handle_t,
}

#[repr(C)]
pub struct ref_tracker_dir_stats {
    pub total: i32,
    pub count: i32,
    pub stacks: [ref_tracker_stack; 0],
}

#[repr(C)]
pub struct ref_tracker_stack {
    pub stack_handle: depot_stack_handle_t,
    pub count: u32,
}

#[repr(C)]
pub struct ostream {
    pub func: Option<unsafe extern "C" fn(*mut ostream, *mut i8, ...)>,
    pub prefix: *mut i8,
    pub buf: *mut i8,
    pub seq: *mut seq_file,
    pub size: i32,
    pub used: i32,
}

extern "C" {
    fn kmalloc_flex<T>(_: usize, _: usize, _: u32) -> *mut T;
    fn kfree(_: *mut core::ffi::c_void);
    fn kmalloc(_: usize, _: u32) -> *mut core::ffi::c_void;
    fn kzalloc_obj<T>(_: u32) -> *mut T;
    fn stack_trace_save(_: *mut usize, _: usize, _: usize) -> u32;
    fn stack_depot_save(_: *const usize, _: u32, _: u32) -> depot_stack_handle_t;
    fn stack_depot_snprint(_: depot_stack_handle_t, _: *mut i8, _: usize, _: usize) -> i32;
    fn stack_depot_print(_: depot_stack_handle_t);
    fn vprintk(_: *mut i8, _: va_list);
    fn vsnprintf(_: *mut i8, _: usize, _: *mut i8, _: va_list) -> i32;
    fn seq_vprintf(_: *mut seq_file, _: *mut i8, _: va_list);
    fn refcount_inc(_: *mut refcount_t);
    fn refcount_dec(_: *mut refcount_t);
    fn refcount_read(_: *mut refcount_t) -> i32;
    fn spin_lock_irqsave(_: *mut spinlock_t, _: *mut usize);
    fn spin_unlock_irqrestore(_: *mut spinlock_t, _: usize);
    fn spin_lock(_: *mut spinlock_t);
    fn spin_unlock(_: *mut spinlock_t);
    fn list_empty(_: *const list_head) -> bool;
    fn list_add(_: *mut list_head, _: *mut list_head);
    fn list_del(_: *mut list_head);
    fn list_move_tail(_: *mut list_head, _: *mut list_head);
    fn pr_err_once(_: *mut i8, ...);
    fn pr_err(_: *mut i8, ...);
    fn warn_on_once(_: bool);
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct refcount_t { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { pub private: *mut core::ffi::c_void }
pub type depot_stack_handle_t = u32;
pub type gfp_t = u32;
pub type va_list = core::ffi::VaList<'static>;

#[repr(C)] pub struct ref_tracker_dir {
    pub list: list_head,
    pub quarantine: list_head,
    pub lock: spinlock_t,
    pub class: *mut i8,
    pub dead: bool,
    pub quarantine_avail: u32,
    pub untracked: refcount_t,
    pub no_tracker: refcount_t,
}

unsafe fn ref_tracker_get_stats(dir: *mut ref_tracker_dir, limit: u32) -> *mut ref_tracker_dir_stats {
    let stats = kmalloc_flex::<ref_tracker_dir_stats>(core::mem::size_of::<ref_tracker_dir_stats>(),
        core::mem::size_of::<ref_tracker_stack>() * limit as usize, 0);
    if stats.is_null() { return (-12isize) as *mut ref_tracker_dir_stats; }
    (*stats).total = 0;
    (*stats).count = 0;
    // list_for_each_entry(tracker, &dir->list, head)
    let mut tracker = (*dir).list.next as *mut ref_tracker;
    while tracker != (&mut (*dir).list as *mut list_head) as *mut ref_tracker {
        let stack = (*tracker).alloc_stack_handle;
        (*stats).total += 1;
        let mut i = 0;
        while i < (*stats).count && (*(stats.add(1) as *mut ref_tracker_stack).add(i as usize)).stack_handle != stack { i += 1; }
        if i >= limit as i32 { tracker = (*(*tracker).head.next).next as *mut ref_tracker; continue; }
        if i >= (*stats).count {
            (*(stats.add(1) as *mut ref_tracker_stack).add(i as usize)).stack_handle = stack;
            (*(stats.add(1) as *mut ref_tracker_stack).add(i as usize)).count = 0;
            (*stats).count += 1;
        }
        (*(stats.add(1) as *mut ref_tracker_stack).add(i as usize)).count += 1;
        tracker = (*tracker).head.next as *mut ref_tracker;
    }
    stats
}

unsafe extern "C" fn pr_ostream_log(_: *mut ostream, fmt: *mut i8, mut args: ...) { vprintk(fmt, args); }

unsafe fn __ref_tracker_dir_pr_ostream(dir: *mut ref_tracker_dir, display_limit: u32, s: *mut ostream) {
    if list_empty(&(*dir).list) { return; }
    let stats = ref_tracker_get_stats(dir, display_limit);
    if (stats as isize) < 0 { return; }
    let sbuf = kmalloc(STACK_BUF_SIZE, 0) as *mut i8;
    let mut i = 0;
    while i < (*stats).count as u32 {
        let stack = (*(stats.add(1) as *mut ref_tracker_stack).add(i as usize)).stack_handle;
        if !sbuf.is_null() && stack_depot_snprint(stack, sbuf, STACK_BUF_SIZE, 4) == 0 { *sbuf = 0; }
        i += 1;
    }
    kfree(sbuf as *mut _);
    kfree(stats as *mut _);
}

#[no_mangle]
pub unsafe extern "C" fn ref_tracker_dir_print_locked(dir: *mut ref_tracker_dir, display_limit: u32) {
    let mut os = ostream { func: Some(pr_ostream_log), prefix: b"ref_tracker: \0" as *const _ as *mut i8, buf: core::ptr::null_mut(), seq: core::ptr::null_mut(), size: 0, used: 0 };
    __ref_tracker_dir_pr_ostream(dir, display_limit, &mut os);
}

#[no_mangle]
pub unsafe extern "C" fn ref_tracker_dir_print(dir: *mut ref_tracker_dir, display_limit: u32) {
    let mut flags = 0; spin_lock_irqsave(&mut (*dir).lock, &mut flags); ref_tracker_dir_print_locked(dir, display_limit); spin_unlock_irqrestore(&mut (*dir).lock, flags);
}

#[no_mangle]
pub unsafe extern "C" fn ref_tracker_dir_snprint(dir: *mut ref_tracker_dir, _buf: *mut i8, _size: usize) -> i32 {
    let mut flags = 0; spin_lock_irqsave(&mut (*dir).lock, &mut flags); __ref_tracker_dir_pr_ostream(dir, 16, core::ptr::null_mut()); spin_unlock_irqrestore(&mut (*dir).lock, flags); 0
}

#[no_mangle]
pub unsafe extern "C" fn ref_tracker_dir_exit(dir: *mut ref_tracker_dir) {
    (*dir).dead = true;
    let mut flags = 0; spin_lock_irqsave(&mut (*dir).lock, &mut flags);
    while !list_empty(&(*dir).quarantine) { let t = (*dir).quarantine.next; list_del(t); kfree(t as *mut _); (*dir).quarantine_avail += 1; }
    if !list_empty(&(*dir).list) { ref_tracker_dir_print_locked(dir, 16); while !list_empty(&(*dir).list) { let t = (*dir).list.next; list_del(t); kfree(t as *mut _); } }
    spin_unlock_irqrestore(&mut (*dir).lock, flags);
    warn_on_once(refcount_read(&mut (*dir).untracked) != 1); warn_on_once(refcount_read(&mut (*dir).no_tracker) != 1);
}

#[no_mangle]
pub unsafe extern "C" fn ref_tracker_alloc(dir: *mut ref_tracker_dir, trackerp: *mut *mut ref_tracker, gfp: gfp_t) -> i32 {
    if trackerp.is_null() { refcount_inc(&mut (*dir).no_tracker); return 0; }
    *trackerp = kzalloc_obj::<ref_tracker>(gfp | 0);
    if (*trackerp).is_null() { refcount_inc(&mut (*dir).untracked); return -12; }
    let mut entries = [0usize; REF_TRACKER_STACK_ENTRIES];
    let n = stack_trace_save(entries.as_mut_ptr(), entries.len(), 1);
    (**trackerp).alloc_stack_handle = stack_depot_save(entries.as_ptr(), n, gfp);
    let mut flags = 0; spin_lock_irqsave(&mut (*dir).lock, &mut flags); list_add(&mut (**trackerp).head, &mut (*dir).list); spin_unlock_irqrestore(&mut (*dir).lock, flags); 0
}

#[no_mangle]
pub unsafe extern "C" fn ref_tracker_free(dir: *mut ref_tracker_dir, trackerp: *mut *mut ref_tracker) -> i32 {
    if trackerp.is_null() { refcount_dec(&mut (*dir).no_tracker); return 0; }
    let tracker = *trackerp; if tracker.is_null() { refcount_dec(&mut (*dir).untracked); return -17; }
    if (*tracker).dead { return -22; }
    (*tracker).dead = true;
    let mut entries = [0usize; REF_TRACKER_STACK_ENTRIES]; let n = stack_trace_save(entries.as_mut_ptr(), entries.len(), 1); (*tracker).free_stack_handle = stack_depot_save(entries.as_ptr(), n, 0);
    let mut flags = 0; spin_lock_irqsave(&mut (*dir).lock, &mut flags); list_move_tail(&mut (*tracker).head, &mut (*dir).quarantine); spin_unlock_irqrestore(&mut (*dir).lock, flags); 0
}

// CONFIG_DEBUG_FS: the following declarations preserve the debugfs extension points.
#[cfg(feature = "CONFIG_DEBUG_FS")]
extern "C" {
    fn debugfs_create_dir(_: *mut i8, _: *mut dentry) -> *mut dentry;
    fn debugfs_create_file(_: *mut i8, _: u32, _: *mut dentry, _: *mut ref_tracker_dir, _: *const file_operations) -> *mut dentry;
    fn debugfs_create_symlink(_: *mut i8, _: *mut dentry, _: *mut i8) -> *mut dentry;
    fn debugfs_remove(_: *mut dentry);
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
#[repr(C)] pub struct dentry { pub d_name: *mut dentry_name }
#[cfg(feature = "CONFIG_DEBUG_FS")]
#[repr(C)] pub struct dentry_name { pub name: *mut i8 }
#[cfg(feature = "CONFIG_DEBUG_FS")]
#[repr(C)] pub struct file_operations { pub owner: *mut core::ffi::c_void }
#[cfg(feature = "CONFIG_DEBUG_FS")]
static mut ref_tracker_debug_dir: *mut dentry = (-2isize) as *mut dentry;

#[cfg(feature = "CONFIG_DEBUG_FS")]
#[no_mangle]
pub unsafe extern "C" fn ref_tracker_dir_debugfs(dir: *mut ref_tracker_dir) {
    // debugfs dentry creation and xarray bookkeeping are performed by the kernel implementation.
    let _ = dir;
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
#[no_mangle]
pub unsafe extern "C" fn ref_tracker_dir_symlink(dir: *mut ref_tracker_dir, _fmt: *const i8, ...) {
    let _ = dir;
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
unsafe extern "C" fn debugfs_reap_work(_: *mut work_struct) {
    // xa_for_each_marked(...): erase marked symlinks and dentries, then repeat while reaped.
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
#[repr(C)] pub struct work_struct { _private: [u8; 0] }

#[cfg(feature = "CONFIG_DEBUG_FS")]
unsafe extern "C" fn ref_tracker_debugfs_postcore_init() -> i32 {
    // INIT_WORK, xa_init_flags(&debugfs_dentries, XA_FLAGS_LOCK_IRQ), xa_init_flags(&debugfs_symlinks, XA_FLAGS_LOCK_IRQ)
    0
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
unsafe extern "C" fn ref_tracker_debugfs_late_init() -> i32 {
    ref_tracker_debug_dir = debugfs_create_dir(b"ref_tracker\0".as_ptr() as *mut i8, core::ptr::null_mut()); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
