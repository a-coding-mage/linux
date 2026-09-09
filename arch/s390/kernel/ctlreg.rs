// SPDX-License-Identifier: GPL-2.0
/*
 *	Copyright IBM Corp. 1999, 2023
 */

// Dependencies supplied by the surrounding kernel translation unit.

extern "C" {
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn get_abs_lowcore() -> *mut lowcore;
    fn put_abs_lowcore(lc: *mut lowcore);
    fn __local_ctl_store(first: i32, last: i32, regs: *mut ctlreg);
    fn __local_ctl_load(first: i32, last: i32, regs: *mut ctlreg);
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn on_each_cpu(callback: unsafe extern "C" fn(*mut core::ffi::c_void), info: *mut core::ffi::c_void, wait: i32);

    static mut system_state: i32;
}

extern "C" {
    type spinlock_t;
    type lowcore;
    type ctlreg;
}

// ctl_lock guards access to global control register contents which
// are kept in the control register save area within absolute lowcore
// at physical address zero.
static mut system_ctl_lock: spinlock_t = unsafe { core::mem::zeroed() };

#[no_mangle]
pub unsafe extern "C" fn system_ctlreg_lock() {
    spin_lock(&raw mut system_ctl_lock);
}

#[no_mangle]
pub unsafe extern "C" fn system_ctlreg_unlock() {
    spin_unlock(&raw mut system_ctl_lock);
}

static mut system_ctlreg_area_init: bool = false;

#[no_mangle]
pub unsafe extern "C" fn system_ctlreg_init_save_area(lc: *mut lowcore) {
    let abs_lc: *mut lowcore;

    abs_lc = get_abs_lowcore();
    __local_ctl_store(0, 15, (*lc).cregs_save_area.as_mut_ptr());
    __local_ctl_store(0, 15, (*abs_lc).cregs_save_area.as_mut_ptr());
    put_abs_lowcore(abs_lc);
    system_ctlreg_area_init = true;
}

#[repr(C)]
struct ctlreg_parms {
    andval: usize,
    orval: usize,
    val: usize,
    request: i32,
    cr: i32,
}

unsafe extern "C" fn ctlreg_callback(info: *mut core::ffi::c_void) {
    let pp = info as *mut ctlreg_parms;
    let mut regs: [ctlreg; 16] = core::mem::zeroed();

    __local_ctl_store(0, 15, regs.as_mut_ptr());
    if (*pp).request == CTLREG_LOAD {
        regs[(*pp).cr as usize].val = (*pp).val;
    } else {
        regs[(*pp).cr as usize].val &= (*pp).andval;
        regs[(*pp).cr as usize].val |= (*pp).orval;
    }
    __local_ctl_load(0, 15, regs.as_mut_ptr());
}

unsafe fn system_ctlreg_update(info: *mut core::ffi::c_void) {
    let mut flags: usize = 0;

    if system_state == SYSTEM_BOOTING {
        /*
         * For very early calls do not call on_each_cpu()
         * since not everything might be setup.
         */
        local_irq_save(&mut flags);
        ctlreg_callback(info);
        local_irq_restore(flags);
    } else {
        on_each_cpu(ctlreg_callback, info, 1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn system_ctlreg_modify(cr: u32, data: usize, request: i32) {
    let mut pp = ctlreg_parms { andval: 0, orval: 0, val: 0, request, cr: cr as i32 };
    let abs_lc: *mut lowcore;

    match request {
        CTLREG_SET_BIT => {
            pp.orval = 1usize << data;
            pp.andval = usize::MAX;
        }
        CTLREG_CLEAR_BIT => {
            pp.orval = 0;
            pp.andval = !(1usize << data);
        }
        CTLREG_LOAD => {
            pp.val = data;
        }
        _ => {}
    }
    if system_ctlreg_area_init {
        system_ctlreg_lock();
        abs_lc = get_abs_lowcore();
        if request == CTLREG_LOAD {
            (*abs_lc).cregs_save_area[cr as usize].val = pp.val;
        } else {
            (*abs_lc).cregs_save_area[cr as usize].val &= pp.andval;
            (*abs_lc).cregs_save_area[cr as usize].val |= pp.orval;
        }
        put_abs_lowcore(abs_lc);
        system_ctlreg_update((&mut pp as *mut ctlreg_parms).cast());
        system_ctlreg_unlock();
    } else {
        system_ctlreg_update((&mut pp as *mut ctlreg_parms).cast());
    }
}

extern "C" {
    static CTLREG_SET_BIT: i32;
    static CTLREG_CLEAR_BIT: i32;
    static CTLREG_LOAD: i32;
    static SYSTEM_BOOTING: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
