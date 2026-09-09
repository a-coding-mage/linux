/* SPDX-License-Identifier: GPL-2.0 */

use core::arch::asm;
use core::ffi::c_void;

/* Dependencies supplied by the surrounding kernel translation. */
use crate::{pt_regs, sparc_stackf, thread_info, PT_REGS_MAGIC, STACK_BIAS, THREAD_SIZE};

extern "C" {
    static mut hardirq_stack: *mut *mut c_void;
    static mut softirq_stack: *mut *mut c_void;
    fn smp_processor_id() -> usize;
}

/* SP must be STACK_BIAS adjusted already. */
#[inline]
pub unsafe fn kstack_valid(tp: *mut thread_info, sp: usize) -> bool {
    let mut base = tp as usize;

    /* Stack pointer must be 16-byte aligned. */
    if sp & (16usize - 1) != 0 {
        return false;
    }

    if sp >= base + core::mem::size_of::<thread_info>()
        && sp <= base + THREAD_SIZE - core::mem::size_of::<sparc_stackf>()
    {
        return true;
    }

    let cpu = (*tp).cpu as usize;
    let irq_stack = *hardirq_stack.add(cpu);
    if !irq_stack.is_null() {
        base = irq_stack as usize;
        if sp >= base && sp <= base + THREAD_SIZE - core::mem::size_of::<sparc_stackf>() {
            return true;
        }
        base = (*softirq_stack.add(cpu)) as usize;
        if sp >= base && sp <= base + THREAD_SIZE - core::mem::size_of::<sparc_stackf>() {
            return true;
        }
    }
    false
}

/* Does "regs" point to a valid pt_regs trap frame? */
#[inline]
pub unsafe fn kstack_is_trap_frame(tp: *mut thread_info, regs: *mut pt_regs) -> bool {
    let mut base = tp as usize;
    let addr = regs as usize;

    if addr >= base && addr <= base + THREAD_SIZE - core::mem::size_of::<pt_regs>() {
        if ((*regs).magic & !0x1ff) == PT_REGS_MAGIC {
            return true;
        }
        return false;
    }

    let cpu = (*tp).cpu as usize;
    let irq_stack = *hardirq_stack.add(cpu);
    if !irq_stack.is_null() {
        base = irq_stack as usize;
        if addr >= base && addr <= base + THREAD_SIZE - core::mem::size_of::<pt_regs>() {
            if ((*regs).magic & !0x1ff) == PT_REGS_MAGIC {
                return true;
            }
            return false;
        }
        base = (*softirq_stack.add(cpu)) as usize;
        if addr >= base && addr <= base + THREAD_SIZE - core::mem::size_of::<pt_regs>() {
            if ((*regs).magic & !0x1ff) == PT_REGS_MAGIC {
                return true;
            }
            return false;
        }
    }
    false
}

#[inline(always)]
pub unsafe fn set_hardirq_stack() -> *mut c_void {
    let mut orig_sp: *mut c_void;
    let mut sp = *hardirq_stack.add(smp_processor_id());

    asm!("mov {orig}, %sp", orig = out(reg) orig_sp);
    if (orig_sp as usize) < (sp as usize)
        || (orig_sp as usize) > (sp as usize + THREAD_SIZE)
    {
        sp = (sp as usize + THREAD_SIZE - 192 - STACK_BIAS) as *mut c_void;
        asm!("mov %sp, {stack}", stack = in(reg) sp);
    }

    orig_sp
}

#[inline(always)]
pub unsafe fn restore_hardirq_stack(orig_sp: *mut c_void) {
    asm!("mov %sp, {stack}", stack = in(reg) orig_sp);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
