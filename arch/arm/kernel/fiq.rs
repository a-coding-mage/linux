// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/arm/kernel/fiq.c
 *
 *  Copyright (C) 1998 Russell King
 *  Copyright (C) 1998, 1999 Phil Blundell
 *
 *  FIQ support written by Philip Blundell <philb@gnu.org>, 1998.
 *
 *  FIQ support re-written by Russell King to be more generic
 *
 * We now properly support a method by which the FIQ handlers can
 * be stacked onto the vector.  We still do not support sharing
 * the FIQ vector itself.
 */

// Linux kernel dependencies supplied by the surrounding translation.
use crate::{fiq_handler, pt_regs, seq_file};

extern "C" {
    static mut vectors_page: *mut core::ffi::c_void;
    fn local_fiq_disable();
    fn local_fiq_enable();
    fn set_fiq_regs(regs: *const pt_regs);
    fn get_fiq_regs(regs: *mut pt_regs);
    fn cache_is_vipt_nonaliasing() -> bool;
    fn flush_icache_range(start: usize, end: usize);
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize);
    fn enable_irq(irq: i32);
    fn disable_irq(irq: i32);
    fn seq_printf(p: *mut seq_file, fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
    fn dump_stack();
    static mut vector_fiq_offset: core::ffi::c_void;
}

const EBUSY: i32 = 16;

static mut dfl_fiq_insn: usize = 0;
static mut dfl_fiq_regs: pt_regs = unsafe { core::mem::zeroed() };

/* Default reacquire function
 * - we always relinquish FIQ control
 * - we always reacquire FIQ control
 */
unsafe extern "C" fn fiq_def_op(_ref_: *mut core::ffi::c_void, relinquish: i32) -> i32 {
    if relinquish == 0 {
        /* Restore default handler and registers */
        local_fiq_disable();
        set_fiq_regs(&dfl_fiq_regs);
        set_fiq_handler((&raw mut dfl_fiq_insn).cast(), core::mem::size_of::<usize>() as u32);
        local_fiq_enable();

        /* FIXME: notify irq controller to standard enable FIQs */
    }

    0
}

static mut default_owner: fiq_handler = fiq_handler {
    name: b"default\0".as_ptr() as *const i8,
    fiq_op: Some(fiq_def_op),
    ..unsafe { core::mem::zeroed() }
};

static mut current_fiq: *mut fiq_handler = &raw mut default_owner;

pub unsafe extern "C" fn show_fiq_list(p: *mut seq_file, prec: i32) -> i32 {
    if current_fiq != &raw mut default_owner {
        seq_printf(p, b"%*s:              %s\n\0".as_ptr(), prec, b"FIQ\0".as_ptr(), (*current_fiq).name);
    }

    0
}

pub unsafe extern "C" fn set_fiq_handler(start: *mut core::ffi::c_void, length: u32) {
    let base = vectors_page;
    let offset = (&raw mut vector_fiq_offset as usize) as u32;

    memcpy(base.add(offset as usize), start, length as usize);
    if !cache_is_vipt_nonaliasing() {
        flush_icache_range(base.add(offset as usize) as usize,
                           base.add(offset as usize).add(length as usize) as usize);
    }
    flush_icache_range(0xffff0000usize + offset as usize,
                       0xffff0000usize + offset as usize + length as usize);
}

pub unsafe extern "C" fn claim_fiq(f: *mut fiq_handler) -> i32 {
    let mut ret = 0;

    if !current_fiq.is_null() {
        ret = -EBUSY;

        if let Some(op) = (*current_fiq).fiq_op {
            ret = op((*current_fiq).dev_id, 1);
        }
    }

    if ret == 0 {
        (*f).next = current_fiq;
        current_fiq = f;
    }

    ret
}

pub unsafe extern "C" fn release_fiq(f: *mut fiq_handler) {
    if current_fiq != f {
        pr_err(b"%s FIQ trying to release %s FIQ\n\0".as_ptr(), (*f).name, (*current_fiq).name);
        dump_stack();
        return;
    }

    loop {
        current_fiq = (*current_fiq).next;
        if !((*current_fiq).fiq_op.unwrap())((*current_fiq).dev_id, 0) != 0 {
            break;
        }
    }
}

static mut fiq_start: i32 = 0;

pub unsafe extern "C" fn enable_fiq(fiq: i32) {
    enable_irq(fiq + fiq_start);
}

pub unsafe extern "C" fn disable_fiq(fiq: i32) {
    disable_irq(fiq + fiq_start);
}

pub unsafe extern "C" fn init_FIQ(start: i32) {
    let offset = (&raw mut vector_fiq_offset as usize) as u32;
    dfl_fiq_insn = *((0xffff0000usize + offset as usize) as *const usize);
    get_fiq_regs(&raw mut dfl_fiq_regs);
    fiq_start = start;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
