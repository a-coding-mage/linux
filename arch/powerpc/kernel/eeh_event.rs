// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Translated from powerpc/kernel/eeh_event.c.
 * Kernel and architecture definitions referenced below are supplied by the
 * surrounding kernel translation.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct eeh_pe {
    pub state: c_ulong,
    pub r#type: c_ulong,
    pub phb: *mut c_void,
    #[cfg(CONFIG_STACKTRACE)]
    pub trace_entries: usize,
    #[cfg(CONFIG_STACKTRACE)]
    pub stack_trace: [c_ulong; 1],
}

#[repr(C)]
pub struct eeh_event {
    pub list: list_head,
    pub pe: *mut eeh_pe,
}

extern "C" {
    static mut eeh_debugfs_no_recover: bool;
    static mut eeh_eventlist_lock: c_void;
    static mut eeh_eventlist_event: c_void;
    static mut eeh_eventlist: list_head;

    fn kthread_should_stop() -> bool;
    fn wait_for_completion_interruptible(event: *mut c_void) -> c_int;
    fn spin_lock_irqsave(lock: *mut c_void, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut c_void, flags: c_ulong);
    fn list_empty(head: *const list_head) -> bool;
    fn list_del(entry: *mut list_head);
    fn list_add(entry: *mut list_head, head: *mut list_head);
    fn complete(event: *mut c_void);
    fn kfree(ptr: *mut c_void);
    fn eeh_handle_normal_event(pe: *mut eeh_pe);
    fn eeh_handle_special_event();
    fn eeh_pe_state_mark(pe: *mut eeh_pe, state: c_ulong);
    fn stack_trace_save(entries: *mut c_ulong, size: usize, skipnr: usize) -> usize;
    fn kthread_run(threadfn: unsafe extern "C" fn(*mut c_void) -> c_int,
                   data: *mut c_void, name: *const c_char) -> *mut c_void;
    fn is_err(ptr: *mut c_void) -> bool;
    fn ptr_err(ptr: *mut c_void) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn kzalloc_event() -> *mut eeh_event;
}

const EEH_PE_RECOVERING: c_ulong = 0;
const EEH_PE_ISOLATED: c_ulong = 0;
const EEH_PE_PHB: c_ulong = 0;
const ENOMEM: c_int = 12;

unsafe extern "C" fn eeh_event_handler(_dummy: *mut c_void) -> c_int {
    let mut flags: c_ulong = 0;
    while !kthread_should_stop() {
        if wait_for_completion_interruptible(&mut eeh_eventlist_event) != 0 {
            break;
        }

        let mut event: *mut eeh_event = core::ptr::null_mut();
        spin_lock_irqsave(&mut eeh_eventlist_lock, &mut flags);
        if !list_empty(&eeh_eventlist) {
            // list_entry(eeh_eventlist.next, struct eeh_event, list)
            event = eeh_eventlist.next as *mut eeh_event;
            list_del(&mut (*event).list);
        }
        spin_unlock_irqrestore(&mut eeh_eventlist_lock, flags);
        if event.is_null() {
            continue;
        }

        if !(*event).pe.is_null() {
            eeh_handle_normal_event((*event).pe);
        } else {
            eeh_handle_special_event();
        }
        kfree(event as *mut c_void);
    }
    0
}

pub unsafe extern "C" fn eeh_event_init() -> c_int {
    let t = kthread_run(eeh_event_handler, core::ptr::null_mut(), b"eehd\0".as_ptr() as *const c_char);
    if is_err(t) {
        let ret = ptr_err(t);
        pr_err(b"%s: Failed to start EEH daemon (%d)\n\0".as_ptr() as *const c_char,
               b"eeh_event_init\0".as_ptr(), ret);
        return ret;
    }
    0
}

pub unsafe extern "C" fn __eeh_send_failure_event(pe: *mut eeh_pe) -> c_int {
    let mut flags: c_ulong = 0;
    let event = kzalloc_event();
    if event.is_null() {
        pr_err(b"EEH: out of memory, event not handled\n\0".as_ptr() as *const c_char);
        return -ENOMEM;
    }
    (*event).pe = pe;

    if !pe.is_null() {
        #[cfg(CONFIG_STACKTRACE)]
        { (*pe).trace_entries = stack_trace_save((*pe).stack_trace.as_mut_ptr(), (*pe).stack_trace.len(), 0); }
        eeh_pe_state_mark(pe, EEH_PE_RECOVERING);
    }

    spin_lock_irqsave(&mut eeh_eventlist_lock, &mut flags);
    list_add(&mut (*event).list, &mut eeh_eventlist);
    spin_unlock_irqrestore(&mut eeh_eventlist_lock, flags);
    complete(&mut eeh_eventlist_event);
    0
}

pub unsafe extern "C" fn eeh_send_failure_event(pe: *mut eeh_pe) -> c_int {
    if eeh_debugfs_no_recover {
        pr_err(b"EEH: Event dropped due to no_recover setting\n\0".as_ptr() as *const c_char);
        return 0;
    }
    __eeh_send_failure_event(pe)
}

pub unsafe extern "C" fn eeh_remove_event(pe: *mut eeh_pe, force: bool) {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut eeh_eventlist_lock, &mut flags);
    let mut pos = eeh_eventlist.next;
    while pos != &mut eeh_eventlist as *mut list_head {
        let next = (*pos).next;
        let event = pos as *mut eeh_event;
        if !force && !(*event).pe.is_null() && ((*(*event).pe).state & EEH_PE_ISOLATED) != 0 {
            pos = next;
            continue;
        }
        let remove = if pe.is_null() { true }
            else if ((*pe).r#type & EEH_PE_PHB) != 0 {
                !(*event).pe.is_null() && (*event).pe.as_ref().unwrap().phb == (*pe).phb
            } else { (*event).pe == pe };
        if remove { list_del(&mut (*event).list); kfree(event as *mut c_void); }
        pos = next;
    }
    spin_unlock_irqrestore(&mut eeh_eventlist_lock, flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
