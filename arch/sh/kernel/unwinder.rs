// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2009  Matt Fleming
 *
 * Based, in part, on kernel/time/clocksource.c.
 *
 * This file provides arbitration code for stack unwinders.
 *
 * Multiple stack unwinders can be available on a system, usually with
 * the most accurate unwinder being the currently active one.
 */

// Dependencies supplied by the surrounding kernel translation.
use crate::{list_add, list_del, list_empty, list_entry, list_for_each,
            list_is_singular, spin_lock_irqsave, spin_unlock_irqrestore,
            stack_reader_dump, list_head, pt_regs, stacktrace_ops,
            task_struct, unwinder, spinlock_t};

/*
 * This is the most basic stack unwinder an architecture can
 * provide. For architectures without reliable frame pointers, e.g.
 * RISC CPUs, it can be implemented by looking through the stack for
 * addresses that lie within the kernel text section.
 *
 * Other CPUs, e.g. x86, can use their frame pointer register to
 * construct more accurate stack traces.
 */
static mut UNWINDER_LIST: list_head = list_head {
    next: core::ptr::addr_of_mut!(UNWINDER_LIST),
    prev: core::ptr::addr_of_mut!(UNWINDER_LIST),
};

static mut STACK_READER: unwinder = unwinder {
    name: b"stack-reader\0".as_ptr() as *const i8,
    dump: Some(stack_reader_dump),
    rating: 50,
    list: list_head {
        next: core::ptr::addr_of_mut!(UNWINDER_LIST),
        prev: core::ptr::addr_of_mut!(UNWINDER_LIST),
    },
};

/*
 * "curr_unwinder" points to the stack unwinder currently in use. This
 * is the unwinder with the highest rating.
 *
 * "unwinder_list" is a linked-list of all available unwinders, sorted
 * by rating.
 *
 * All modifications of "curr_unwinder" and "unwinder_list" must be
 * performed whilst holding "unwinder_lock".
 */
static mut CURR_UNWINDER: *mut unwinder = core::ptr::addr_of_mut!(STACK_READER);

static mut UNWINDER_LOCK: spinlock_t = spinlock_t::new();

/**
 * select_unwinder - Select the best registered stack unwinder.
 *
 * Private function. Must hold unwinder_lock when called.
 *
 * Select the stack unwinder with the best rating. This is useful for
 * setting up curr_unwinder.
 */
unsafe fn select_unwinder() -> *mut unwinder {
    let best: *mut unwinder;

    if list_empty(core::ptr::addr_of!(UNWINDER_LIST)) {
        return core::ptr::null_mut();
    }

    best = list_entry((*core::ptr::addr_of!(UNWINDER_LIST)).next,
                      unwinder, list);
    if best == CURR_UNWINDER {
        return core::ptr::null_mut();
    }

    best
}

/* Enqueue the stack unwinder sorted by rating. */
unsafe fn unwinder_enqueue(ops: *mut unwinder) -> i32 {
    let mut entry: *mut list_head = core::ptr::addr_of_mut!(UNWINDER_LIST);
    let mut tmp: *mut list_head;

    list_for_each!(tmp, core::ptr::addr_of!(UNWINDER_LIST), {
        let o: *mut unwinder = list_entry(tmp, unwinder, list);
        if o == ops {
            return -16; // -EBUSY
        }
        /* Keep track of the place, where to insert */
        if (*o).rating >= (*ops).rating {
            entry = tmp;
        }
    });
    list_add(core::ptr::addr_of_mut!((*ops).list), entry);

    0
}

/** Used to install new stack unwinder. */
#[no_mangle]
pub unsafe extern "C" fn unwinder_register(u: *mut unwinder) -> i32 {
    let mut flags: usize = 0;
    let ret: i32;

    spin_lock_irqsave(core::ptr::addr_of_mut!(UNWINDER_LOCK), &mut flags);
    ret = unwinder_enqueue(u);
    if ret == 0 {
        CURR_UNWINDER = select_unwinder();
    }
    spin_unlock_irqrestore(core::ptr::addr_of_mut!(UNWINDER_LOCK), flags);

    ret
}

#[no_mangle]
pub static mut unwinder_faulted: i32 = 0;

/*
 * Unwind the call stack and pass information to the stacktrace_ops
 * functions. Also handle the case where we need to switch to a new
 * stack dumper because the current one faulted unexpectedly.
 */
#[no_mangle]
pub unsafe extern "C" fn unwind_stack(
    task: *mut task_struct,
    regs: *mut pt_regs,
    sp: *mut usize,
    ops: *const stacktrace_ops,
    data: *mut core::ffi::c_void,
) {
    let mut flags: usize = 0;

    /*
     * The problem with unwinders with high ratings is that they are
     * inherently more complicated than the simple ones with lower
     * ratings. We are therefore more likely to fault in the
     * complicated ones, e.g. hitting BUG()s. If we fault in the
     * code for the current stack unwinder we try to downgrade to
     * one with a lower rating.
     *
     * Hopefully this will give us a semi-reliable stacktrace so we
     * can diagnose why curr_unwinder->dump() faulted.
     */
    if unwinder_faulted != 0 {
        spin_lock_irqsave(core::ptr::addr_of_mut!(UNWINDER_LOCK), &mut flags);

        /* Make sure no one beat us to changing the unwinder */
        if unwinder_faulted != 0
            && !list_is_singular(core::ptr::addr_of!(UNWINDER_LIST))
        {
            list_del(core::ptr::addr_of_mut!((*CURR_UNWINDER).list));
            CURR_UNWINDER = select_unwinder();

            unwinder_faulted = 0;
        }

        spin_unlock_irqrestore(core::ptr::addr_of_mut!(UNWINDER_LOCK), flags);
    }

    ((*CURR_UNWINDER).dump.unwrap())(task, regs, sp, ops, data);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
