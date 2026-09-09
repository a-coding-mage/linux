/*
 * irq.c
 *
 * (C) Copyright 2007, Greg Ungerer <gerg@snapgear.com>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

use core::ffi::{c_char, c_int, c_uint};

// Types and functions supplied by the Linux and architecture headers.
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn set_irq_regs(regs: *mut pt_regs) -> *mut pt_regs;
    fn irq_enter();
    fn generic_handle_irq(irq: c_int);
    fn irq_exit();
    fn atomic_read(v: *const atomic_t) -> c_uint;
    fn seq_printf(p: *mut seq_file, fmt: *const c_char, ...);

}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_IRQ(irq: c_int, regs: *mut pt_regs) {
    let oldregs: *mut pt_regs = unsafe { set_irq_regs(regs) };

    unsafe { irq_enter() };
    unsafe { generic_handle_irq(irq) };
    unsafe { irq_exit() };

    unsafe { set_irq_regs(oldregs) };
}

/* The number of spurious interrupts */

#[unsafe(no_mangle)]
pub static mut irq_err_count: atomic_t = atomic_t { _private: [] };

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch_show_interrupts(p: *mut seq_file, prec: c_int) -> c_int {
    let format = b"%*s: %10u\n\0";
    let label = b"ERR\0";
    unsafe {
        seq_printf(
            p,
            format.as_ptr() as *const c_char,
            prec,
            label.as_ptr() as *const c_char,
            atomic_read(core::ptr::addr_of!(irq_err_count)),
        );
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
