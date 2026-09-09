// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel and speakup sources.

use core::ptr;

pub type u16 = u16;

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vc_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tty_struct {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tiocl_selection {
    pub xs: u16,
    pub ys: u16,
    pub xe: u16,
    pub ye: u16,
    pub sel_mode: u16,
}

#[repr(C)]
pub struct speakup_selection_work {
    pub work: work_struct,
    pub sel: tiocl_selection,
    pub tty: *mut tty_struct,
}

pub static mut spk_xs: u16 = 0;
pub static mut spk_ys: u16 = 0;
pub static mut spk_xe: u16 = 0;
pub static mut spk_ye: u16 = 0;
pub static mut spk_sel_cons: *mut vc_data = ptr::null_mut();

extern "C" {
    static mut vc_cons: *mut vc_data;
    static mut fg_console: i32;

    fn clear_selection();
    fn console_lock();
    fn console_unlock();
    fn set_selection_kernel(sel: *const tiocl_selection, tty: *mut tty_struct);
    fn paste_selection(tty: *mut tty_struct);
    fn tty_kref_get(tty: *mut tty_struct);
    fn tty_kref_put(tty: *mut tty_struct);
    fn cancel_work_sync(work: *mut work_struct);
    fn schedule_work_on(cpu: i32, work: *mut work_struct);
}

const EBUSY: i32 = 16;
const WORK_CPU_UNBOUND: i32 = -1;
const TIOCL_SELCHAR: u16 = 0;

unsafe fn __speakup_set_selection(work: *mut work_struct) {
    let ssw = work as *mut speakup_selection_work;
    let sel = (*ssw).sel;

    // This ensures we copy sel before releasing the lock below.
    core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);

    // Release the lock by setting tty of the struct to NULL.
    let tty = core::ptr::read_volatile(&(*ssw).tty);
    core::ptr::write_volatile(&mut (*ssw).tty, ptr::null_mut());

    if spk_sel_cons != vc_cons.add(fg_console as usize) {
        spk_sel_cons = vc_cons.add(fg_console as usize);
        // pr_warn("Selection: mark console not the same as cut\n");
        tty_kref_put(tty);
        return;
    }

    console_lock();
    clear_selection();
    console_unlock();

    set_selection_kernel(&sel, tty);
    tty_kref_put(tty);
}

pub static mut speakup_sel_work: speakup_selection_work = speakup_selection_work {
    work: work_struct { _private: [] },
    sel: tiocl_selection { xs: 0, ys: 0, xe: 0, ye: 0, sel_mode: 0 },
    tty: ptr::null_mut(),
};

#[no_mangle]
pub unsafe extern "C" fn speakup_set_selection(tty: *mut tty_struct) -> i32 {
    tty_kref_get(tty);
    let old = core::ptr::read_volatile(&speakup_sel_work.tty);
    if !old.is_null()
        || core::ptr::compare_exchange_weak(
            &mut speakup_sel_work.tty,
            ptr::null_mut(),
            tty,
            core::sync::atomic::Ordering::AcqRel,
            core::sync::atomic::Ordering::Acquire,
        )
        .is_err()
    {
        tty_kref_put(tty);
        return -EBUSY;
    }

    // Ensure writes to speakup_sel_work do not happen before cmpxchg above.
    core::sync::atomic::fence(core::sync::atomic::Ordering::Release);

    speakup_sel_work.sel.xs = spk_xs.wrapping_add(1);
    speakup_sel_work.sel.ys = spk_ys.wrapping_add(1);
    speakup_sel_work.sel.xe = spk_xe.wrapping_add(1);
    speakup_sel_work.sel.ye = spk_ye.wrapping_add(1);
    speakup_sel_work.sel.sel_mode = TIOCL_SELCHAR;

    schedule_work_on(WORK_CPU_UNBOUND, &mut speakup_sel_work.work);
    0
}

#[no_mangle]
pub unsafe extern "C" fn speakup_cancel_selection() {
    cancel_work_sync(&mut speakup_sel_work.work);
    let tty = core::ptr::read_volatile(&speakup_sel_work.tty);
    core::ptr::write_volatile(&mut speakup_sel_work.tty, ptr::null_mut());
    if !tty.is_null() {
        tty_kref_put(tty);
    }
}

unsafe fn __speakup_paste_selection(work: *mut work_struct) {
    let ssw = work as *mut speakup_selection_work;
    let tty = core::ptr::read_volatile(&(*ssw).tty);
    core::ptr::write_volatile(&mut (*ssw).tty, ptr::null_mut());
    paste_selection(tty);
    tty_kref_put(tty);
}

pub static mut speakup_paste_work: speakup_selection_work = speakup_selection_work {
    work: work_struct { _private: [] },
    sel: tiocl_selection { xs: 0, ys: 0, xe: 0, ye: 0, sel_mode: 0 },
    tty: ptr::null_mut(),
};

#[no_mangle]
pub unsafe extern "C" fn speakup_paste_selection(tty: *mut tty_struct) -> i32 {
    tty_kref_get(tty);
    if !core::ptr::compare_exchange_weak(
        &mut speakup_paste_work.tty,
        ptr::null_mut(),
        tty,
        core::sync::atomic::Ordering::AcqRel,
        core::sync::atomic::Ordering::Acquire,
    )
    .is_ok()
    {
        tty_kref_put(tty);
        return -EBUSY;
    }
    schedule_work_on(WORK_CPU_UNBOUND, &mut speakup_paste_work.work);
    0
}

#[no_mangle]
pub unsafe extern "C" fn speakup_cancel_paste() {
    cancel_work_sync(&mut speakup_paste_work.work);
    let tty = core::ptr::read_volatile(&speakup_paste_work.tty);
    core::ptr::write_volatile(&mut speakup_paste_work.tty, ptr::null_mut());
    if !tty.is_null() {
        tty_kref_put(tty);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
