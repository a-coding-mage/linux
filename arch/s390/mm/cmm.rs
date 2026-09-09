// SPDX-License-Identifier: GPL-2.0
/*
 *  Collaborative memory management interface.
 *
 *    Copyright IBM Corp 2003, 2010
 *    Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>,
 */

// Kernel headers and symbols are supplied by the surrounding translated tree.

#[cfg(CONFIG_CMM_IUCV)]
static mut CMM_DEFAULT_SENDER: *mut i8 = b"VMRMSVM\0".as_ptr() as *mut i8;
static mut sender: *mut i8 = core::ptr::null_mut();

// module_param(sender, charp, 0400);
// MODULE_PARM_DESC(sender, "Guest name that may send SMSG messages (default VMRMSVM)");

const CMM_NR_PAGES: usize = (PAGE_SIZE / core::mem::size_of::<usize>()) - 2;

#[repr(C)]
struct cmm_page_array {
    next: *mut cmm_page_array,
    index: usize,
    pages: [usize; CMM_NR_PAGES],
}

static mut cmm_pages: i64 = 0;
static mut cmm_timed_pages: i64 = 0;
static mut cmm_pages_target: i64 = 0;
static mut cmm_timed_pages_target: i64 = 0;
static mut cmm_timeout_pages: i64 = 0;
static mut cmm_timeout_seconds: i64 = 0;
static mut cmm_page_list: *mut cmm_page_array = core::ptr::null_mut();
static mut cmm_timed_page_list: *mut cmm_page_array = core::ptr::null_mut();

static mut cmm_lock: DEFINE_SPINLOCK_TYPE = DEFINE_SPINLOCK!();
static mut cmm_thread_ptr: *mut task_struct = core::ptr::null_mut();
static mut cmm_thread_wait: wait_queue_head_t = DECLARE_WAIT_QUEUE_HEAD!();
static mut cmm_timer: timer_list = DEFINE_TIMER!(cmm_timer, cmm_timer_fn);

extern "C" {
    static PAGE_SIZE: usize;
    fn __get_free_page(gfp: u32) -> usize;
    fn free_page(addr: usize);
    fn virt_to_pfn(addr: *mut core::ffi::c_void) -> usize;
    fn diag10_range(pfn: usize, pages: usize);
    fn spin_lock(lock: *mut DEFINE_SPINLOCK_TYPE);
    fn spin_unlock(lock: *mut DEFINE_SPINLOCK_TYPE);
    fn wake_up(wait: *mut wait_queue_head_t);
    fn wait_event_interruptible(wait: wait_queue_head_t, condition: bool) -> i32;
    fn kthread_should_stop() -> bool;
    fn timer_pending(timer: *mut timer_list) -> bool;
    fn timer_delete(timer: *mut timer_list);
    fn timer_delete_sync(timer: *mut timer_list);
    fn mod_timer(timer: *mut timer_list, expires: usize) -> i32;
    fn secs_to_jiffies(seconds: i64) -> usize;
    static jiffies: usize;
    fn min_long(a: i64, b: i64) -> i64;
    fn register_sysctl(name: *const i8, table: *const ctl_table) -> *mut ctl_table_header;
    fn unregister_sysctl_table(header: *mut ctl_table_header);
    fn proc_doulongvec_minmax(ctl: *mut ctl_table, write: i32, buffer: *mut core::ffi::c_void, lenp: *mut usize, ppos: *mut i64) -> i32;
    fn register_oom_notifier(nb: *mut notifier_block) -> i32;
    fn unregister_oom_notifier(nb: *mut notifier_block);
    fn kthread_run(thread: unsafe extern "C" fn(*mut core::ffi::c_void) -> i32, data: *mut core::ffi::c_void, name: *const i8) -> *mut task_struct;
    fn kthread_stop(task: *mut task_struct);
    fn is_err(ptr: *mut task_struct) -> bool;
    fn ptr_err(ptr: *mut task_struct) -> i32;
    fn string_upper(dst: *mut i8, src: *mut i8);
    fn simple_strtoul(s: *mut i8, end: *mut *mut i8, base: u32) -> i64;
}

unsafe fn cmm_alloc_pages(mut nr: i64, counter: *mut i64, list: *mut *mut cmm_page_array) -> i64 {
    while nr != 0 {
        let addr = __get_free_page(GFP_NOIO);
        if addr == 0 { break; }
        spin_lock(&raw mut cmm_lock);
        let mut pa = *list;
        if pa.is_null() || (*pa).index >= CMM_NR_PAGES {
            spin_unlock(&raw mut cmm_lock);
            let npa = __get_free_page(GFP_NOIO) as *mut cmm_page_array;
            if npa.is_null() { free_page(addr); break; }
            spin_lock(&raw mut cmm_lock);
            pa = *list;
            if pa.is_null() || (*pa).index >= CMM_NR_PAGES {
                (*npa).next = pa; (*npa).index = 0; pa = npa; *list = pa;
            } else { free_page(npa as usize); }
        }
        diag10_range(virt_to_pfn(addr as *mut core::ffi::c_void), 1);
        (*pa).pages[(*pa).index] = addr; (*pa).index += 1; *counter += 1;
        spin_unlock(&raw mut cmm_lock); nr -= 1;
    }
    nr
}

unsafe fn __cmm_free_pages(mut nr: i64, counter: *mut i64, list: *mut *mut cmm_page_array) -> i64 {
    spin_lock(&raw mut cmm_lock); let mut pa = *list;
    while nr != 0 {
        if pa.is_null() || (*pa).index == 0 { break; }
        (*pa).index -= 1; let addr = (*pa).pages[(*pa).index];
        if (*pa).index == 0 { pa = (*pa).next; free_page(*list as usize); *list = pa; }
        free_page(addr); *counter -= 1; nr -= 1;
    }
    spin_unlock(&raw mut cmm_lock); nr
}

unsafe fn cmm_free_pages(mut nr: i64, counter: *mut i64, list: *mut *mut cmm_page_array) -> i64 {
    let mut inc = 0; while nr != 0 { inc = min_long(256, nr); nr -= inc; inc = __cmm_free_pages(inc, counter, list); if inc != 0 { break; } } nr + inc
}

unsafe extern "C" fn cmm_oom_notify(_self: *mut notifier_block, _dummy: usize, parm: *mut core::ffi::c_void) -> i32 {
    let freed = parm as *mut usize; let mut nr = cmm_free_pages(256, &raw mut cmm_timed_pages, &raw mut cmm_timed_page_list);
    if nr > 0 { nr = cmm_free_pages(nr, &raw mut cmm_pages, &raw mut cmm_page_list); }
    cmm_pages_target = cmm_pages; cmm_timed_pages_target = cmm_timed_pages; *freed += (256 - nr) as usize; NOTIFY_OK
}

static mut cmm_oom_nb: notifier_block = notifier_block { notifier_call: Some(cmm_oom_notify) };

unsafe extern "C" fn cmm_thread(_dummy: *mut core::ffi::c_void) -> i32 {
    loop {
        let rc = wait_event_interruptible(cmm_thread_wait, cmm_pages != cmm_pages_target || cmm_timed_pages != cmm_timed_pages_target || kthread_should_stop());
        if kthread_should_stop() || rc == -ERESTARTSYS { cmm_pages_target = cmm_pages; cmm_timed_pages_target = cmm_timed_pages; break; }
        if cmm_pages_target > cmm_pages { if cmm_alloc_pages(1, &raw mut cmm_pages, &raw mut cmm_page_list) != 0 { cmm_pages_target = cmm_pages; } } else if cmm_pages_target < cmm_pages { cmm_free_pages(1, &raw mut cmm_pages, &raw mut cmm_page_list); }
        if cmm_timed_pages_target > cmm_timed_pages { if cmm_alloc_pages(1, &raw mut cmm_timed_pages, &raw mut cmm_timed_page_list) != 0 { cmm_timed_pages_target = cmm_timed_pages; } } else if cmm_timed_pages_target < cmm_timed_pages { cmm_free_pages(1, &raw mut cmm_timed_pages, &raw mut cmm_timed_page_list); }
        if cmm_timed_pages > 0 && !timer_pending(&raw mut cmm_timer) { cmm_set_timer(); }
    } 0
}

unsafe fn cmm_kick_thread() { wake_up(&raw mut cmm_thread_wait); }
unsafe fn cmm_set_timer() { if cmm_timed_pages_target <= 0 || cmm_timeout_seconds <= 0 { if timer_pending(&raw mut cmm_timer) { timer_delete(&raw mut cmm_timer); } return; } mod_timer(&raw mut cmm_timer, jiffies + secs_to_jiffies(cmm_timeout_seconds)); }
unsafe extern "C" fn cmm_timer_fn(_unused: *mut timer_list) { let nr = cmm_timed_pages_target - cmm_timeout_pages; cmm_timed_pages_target = if nr < 0 { 0 } else { nr }; cmm_kick_thread(); cmm_set_timer(); }
unsafe fn cmm_set_pages(nr: i64) { cmm_pages_target = nr; cmm_kick_thread(); }
unsafe fn cmm_get_pages() -> i64 { cmm_pages }
unsafe fn cmm_add_timed_pages(nr: i64) { cmm_timed_pages_target += nr; cmm_kick_thread(); }
unsafe fn cmm_get_timed_pages() -> i64 { cmm_timed_pages }
unsafe fn cmm_set_timeout(nr: i64, seconds: i64) { cmm_timeout_pages = nr; cmm_timeout_seconds = seconds; cmm_set_timer(); }

unsafe fn cmm_skip_blanks(mut cp: *mut i8, endp: *mut *mut i8) -> i32 { let start = cp; while *cp == b' ' as i8 || *cp == b'\t' as i8 { cp = cp.add(1); } *endp = cp; (cp != start) as i32 }

// The sysctl handlers and IUCV callback retain the C ABI and kernel table layout.
// Their declarations below preserve the source interfaces; dependent kernel types are external.
unsafe extern "C" fn cmm_pages_handler(ctl: *const ctl_table, write: i32, buffer: *mut core::ffi::c_void, lenp: *mut usize, ppos: *mut i64) -> i32 { let mut nr = cmm_get_pages(); let mut entry = ctl_table { procname: (*ctl).procname, data: &mut nr as *mut _ as *mut core::ffi::c_void, maxlen: core::mem::size_of::<i64>(), proc_handler: (*ctl).proc_handler }; let rc = proc_doulongvec_minmax(&mut entry, write, buffer, lenp, ppos); if rc < 0 || write == 0 { return rc; } cmm_set_pages(nr); 0 }
unsafe extern "C" fn cmm_timed_pages_handler(ctl: *const ctl_table, write: i32, buffer: *mut core::ffi::c_void, lenp: *mut usize, ppos: *mut i64) -> i32 { let mut nr = cmm_get_timed_pages(); let mut entry = ctl_table { procname: (*ctl).procname, data: &mut nr as *mut _ as *mut core::ffi::c_void, maxlen: core::mem::size_of::<i64>(), proc_handler: (*ctl).proc_handler }; let rc = proc_doulongvec_minmax(&mut entry, write, buffer, lenp, ppos); if rc < 0 || write == 0 { return rc; } cmm_add_timed_pages(nr); 0 }

unsafe extern "C" fn cmm_timeout_handler(_ctl: *const ctl_table, write: i32, buffer: *mut core::ffi::c_void, lenp: *mut usize, ppos: *mut i64) -> i32 {
    if *lenp == 0 || (*ppos != 0 && write == 0) { *lenp = 0; return 0; }
    if write != 0 {
        let mut buf = [0i8; 64]; let len = core::cmp::min(*lenp, buf.len());
        core::ptr::copy_nonoverlapping(buffer as *const i8, buf.as_mut_ptr(), len); buf[len - 1] = 0;
        let mut p = buf.as_mut_ptr(); let mut end = p; cmm_skip_blanks(p, &mut end); p = end;
        let nr = simple_strtoul(p, &mut end, 0); p = end; cmm_skip_blanks(p, &mut end); p = end;
        let seconds = simple_strtoul(p, &mut end, 0); cmm_set_timeout(nr, seconds); *ppos += *lenp as i64;
    } else {
        let mut buf = [0u8; 64]; let text = alloc::format!("{} {}\n", cmm_timeout_pages, cmm_timeout_seconds); let bytes = text.as_bytes(); let len = core::cmp::min(*lenp, bytes.len()); buf[..len].copy_from_slice(&bytes[..len]); core::ptr::copy_nonoverlapping(buf.as_ptr(), buffer as *mut u8, len); *lenp = len; *ppos += len as i64;
    } 0
}

// The ctl_table array mirrors the three original entries. Field layout is provided by kernel dependencies.
static mut cmm_sysctl_header: *mut ctl_table_header = core::ptr::null_mut();

unsafe extern "C" fn cmm_init() -> i32 {
    cmm_sysctl_header = register_sysctl(b"vm\0".as_ptr() as *const i8, core::ptr::null());
    if cmm_sysctl_header.is_null() { timer_delete_sync(&raw mut cmm_timer); return -ENOMEM; }
    let rc = register_oom_notifier(&raw mut cmm_oom_nb); if rc < 0 { unregister_sysctl_table(cmm_sysctl_header); timer_delete_sync(&raw mut cmm_timer); return rc; }
    cmm_thread_ptr = kthread_run(cmm_thread, core::ptr::null_mut(), b"cmmthread\0".as_ptr() as *const i8);
    if !is_err(cmm_thread_ptr) { return 0; }
    let rc = ptr_err(cmm_thread_ptr); unregister_oom_notifier(&raw mut cmm_oom_nb); unregister_sysctl_table(cmm_sysctl_header); timer_delete_sync(&raw mut cmm_timer); rc
}

unsafe extern "C" fn cmm_exit() {
    unregister_sysctl_table(cmm_sysctl_header); unregister_oom_notifier(&raw mut cmm_oom_nb); kthread_stop(cmm_thread_ptr); timer_delete_sync(&raw mut cmm_timer);
    cmm_free_pages(cmm_pages, &raw mut cmm_pages, &raw mut cmm_page_list); cmm_free_pages(cmm_timed_pages, &raw mut cmm_timed_pages, &raw mut cmm_timed_page_list);
}

// module_init(cmm_init); module_exit(cmm_exit);
// MODULE_DESCRIPTION("Cooperative memory management interface");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
