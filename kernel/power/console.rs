// SPDX-License-Identifier: GPL-2.0
/*
 * Functions for saving/restoring console.
 *
 * Originally from swsusp.
 */

// C dependencies supplied by the surrounding kernel translation unit.

use core::ffi::c_int;

pub const SUSPEND_CONSOLE: c_int = MAX_NR_CONSOLES - 1;

pub type bool_t = bool;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

extern "C" {
    static console_suspend_enabled: bool;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn kmalloc(size: usize) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn vt_move_to_console(vt: c_int, alloc: c_int) -> c_int;
    fn vt_kmsg_redirect(console: c_int) -> c_int;
    static mut pm_vt_switch_list: list_head;
}

extern "C" {
    static mut MAX_NR_CONSOLES: c_int;
}

static mut orig_fgconsole: c_int = 0;
static mut orig_kmsg: c_int = 0;
static mut vt_switch_done: bool = false;

static mut vt_switch_mutex: mutex = mutex { _private: [] };

#[repr(C)]
pub struct pm_vt_switch {
    pub head: list_head,
    pub dev: *mut device,
    pub required: bool,
}

/*
 * pm_vt_switch_required - indicate VT switch at suspend requirements
 * @dev: device
 * @required: if true, caller needs VT switch at suspend/resume time
 *
 * The different console drivers may or may not require VT switches across
 * suspend/resume, depending on how they handle restoring video state and
 * what may be running.
 *
 * Drivers can indicate support for switchless suspend/resume, which can
 * save time and flicker, by using this routine and passing 'false' as the
 * argument.  If any loaded driver needs VT switching, or the
 * no_console_suspend argument has been passed on the command line, VT
 * switches will occur.
 */
#[no_mangle]
pub unsafe extern "C" fn pm_vt_switch_required(dev: *mut device, required: bool) -> c_int {
    let mut entry: *mut pm_vt_switch;
    let mut tmp: *mut pm_vt_switch;
    let mut ret: c_int = 0;

    mutex_lock(&mut vt_switch_mutex);
    // list_for_each_entry(tmp, &pm_vt_switch_list, head)
    let mut pos = pm_vt_switch_list.next;
    while pos != &mut pm_vt_switch_list as *mut list_head {
        tmp = pos.cast::<pm_vt_switch>();
        if (*tmp).dev == dev {
            /* already registered, update requirement */
            (*tmp).required = required;
            mutex_unlock(&mut vt_switch_mutex);
            return ret;
        }
        pos = (*pos).next;
    }

    entry = kmalloc(core::mem::size_of::<pm_vt_switch>()) as *mut pm_vt_switch;
    if entry.is_null() {
        ret = -12; // -ENOMEM
        mutex_unlock(&mut vt_switch_mutex);
        return ret;
    }

    (*entry).required = required;
    (*entry).dev = dev;

    list_add(&mut (*entry).head, &mut pm_vt_switch_list);
    mutex_unlock(&mut vt_switch_mutex);
    ret
}

/*
 * pm_vt_switch_unregister - stop tracking a device's VT switching needs
 * @dev: device
 *
 * Remove @dev from the vt switch list.
 */
#[no_mangle]
pub unsafe extern "C" fn pm_vt_switch_unregister(dev: *mut device) {
    mutex_lock(&mut vt_switch_mutex);
    // list_for_each_entry(tmp, &pm_vt_switch_list, head)
    let mut pos = pm_vt_switch_list.next;
    while pos != &mut pm_vt_switch_list as *mut list_head {
        let tmp = pos.cast::<pm_vt_switch>();
        if (*tmp).dev == dev {
            list_del(&mut (*tmp).head);
            kfree(tmp.cast());
            break;
        }
        pos = (*pos).next;
    }
    mutex_unlock(&mut vt_switch_mutex);
}

unsafe fn pm_vt_switch() -> bool {
    let mut ret: bool = true;

    mutex_lock(&mut vt_switch_mutex);
    if pm_vt_switch_list.next == &mut pm_vt_switch_list as *mut list_head {
        mutex_unlock(&mut vt_switch_mutex);
        return ret;
    }

    if !console_suspend_enabled {
        mutex_unlock(&mut vt_switch_mutex);
        return ret;
    }

    // list_for_each_entry(entry, &pm_vt_switch_list, head)
    let mut pos = pm_vt_switch_list.next;
    while pos != &mut pm_vt_switch_list as *mut list_head {
        let entry = pos.cast::<pm_vt_switch>();
        if (*entry).required {
            mutex_unlock(&mut vt_switch_mutex);
            return ret;
        }
        pos = (*pos).head.next;
    }

    ret = false;
    mutex_unlock(&mut vt_switch_mutex);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn pm_prepare_console() {
    if !pm_vt_switch() {
        return;
    }

    orig_fgconsole = vt_move_to_console(SUSPEND_CONSOLE, 1);
    if orig_fgconsole < 0 {
        return;
    }

    vt_switch_done = true;

    orig_kmsg = vt_kmsg_redirect(SUSPEND_CONSOLE);
    return;
}

#[no_mangle]
pub unsafe extern "C" fn pm_restore_console() {
    if !pm_vt_switch() && !vt_switch_done {
        return;
    }

    if orig_fgconsole >= 0 {
        vt_move_to_console(orig_fgconsole, 0);
        vt_kmsg_redirect(orig_kmsg);
    }

    vt_switch_done = false;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
