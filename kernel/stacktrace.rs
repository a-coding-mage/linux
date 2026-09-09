// SPDX-License-Identifier: GPL-2.0-only
/*
 * kernel/stacktrace.c
 *
 * Stack trace management functions
 *
 *  Copyright (C) 2006 Red Hat, Inc., Ingo Molnar <mingo@redhat.com>
 */

// Declarations supplied by the kernel headers and other translation units.
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

type StackTraceConsumeFn = unsafe extern "C" fn(*mut core::ffi::c_void, usize) -> bool;

extern "C" {
    fn warn_on(condition: bool) -> bool;
    fn printk(format: *const core::ffi::c_char, ...);
    fn snprintf(buf: *mut core::ffi::c_char, size: usize, format: *const core::ffi::c_char, ...)
        -> usize;
    fn in_sched_functions(addr: usize) -> bool;
    fn arch_stack_walk(
        consume: StackTraceConsumeFn,
        cookie: *mut core::ffi::c_void,
        task: *mut task_struct,
        regs: *mut pt_regs,
    );
    fn arch_stack_walk_reliable(
        consume: StackTraceConsumeFn,
        cookie: *mut core::ffi::c_void,
        task: *mut task_struct,
    ) -> i32;
    fn arch_stack_walk_user(
        consume: StackTraceConsumeFn,
        cookie: *mut core::ffi::c_void,
        regs: *mut pt_regs,
    );
    fn try_get_task_stack(task: *mut task_struct) -> bool;
    fn put_task_stack(task: *mut task_struct);
    static mut current: *mut task_struct;
}

#[repr(C)]
struct StacktraceCookie {
    store: *mut usize,
    size: u32,
    skip: u32,
    len: u32,
}

/// stack_trace_print - Print the entries in the stack trace
pub unsafe extern "C" fn stack_trace_print(entries: *const usize, nr_entries: u32, spaces: i32) {
    if warn_on(entries.is_null()) {
        return;
    }
    for i in 0..nr_entries {
        printk(b"%*c%pS\n\0".as_ptr() as *const _, 1 + spaces, b' ' as i32,
            entries.add(i as usize) as *const core::ffi::c_void);
    }
}

/// stack_trace_snprint - Print the entries in the stack trace into a buffer
pub unsafe extern "C" fn stack_trace_snprint(
    mut buf: *mut core::ffi::c_char,
    mut size: usize,
    entries: *const usize,
    nr_entries: u32,
    spaces: i32,
) -> i32 {
    let mut total: usize = 0;
    if warn_on(entries.is_null()) {
        return 0;
    }
    for i in 0..nr_entries {
        if size == 0 { break; }
        let generated = snprintf(buf, size, b"%*c%pS\n\0".as_ptr() as *const _,
            1 + spaces, b' ' as i32, *entries.add(i as usize) as *const core::ffi::c_void);
        total = total.wrapping_add(generated);
        if generated >= size {
            buf = buf.add(size);
            size = 0;
        } else {
            buf = buf.add(generated);
            size -= generated;
        }
    }
    total as i32
}

#[cfg(CONFIG_ARCH_STACKWALK)]
unsafe extern "C" fn stack_trace_consume_entry(cookie: *mut core::ffi::c_void, addr: usize) -> bool {
    let c = &mut *(cookie as *mut StacktraceCookie);
    if c.len >= c.size { return false; }
    if c.skip > 0 { c.skip -= 1; return true; }
    *c.store.add(c.len as usize) = addr;
    c.len += 1;
    c.len < c.size
}

#[cfg(CONFIG_ARCH_STACKWALK)]
unsafe extern "C" fn stack_trace_consume_entry_nosched(cookie: *mut core::ffi::c_void, addr: usize) -> bool {
    if in_sched_functions(addr) { return true; }
    stack_trace_consume_entry(cookie, addr)
}

#[cfg(CONFIG_ARCH_STACKWALK)]
pub unsafe extern "C" fn stack_trace_save(store: *mut usize, size: u32, skipnr: u32) -> u32 {
    let mut c = StacktraceCookie { store, size, skip: skipnr.wrapping_add(1), len: 0 };
    arch_stack_walk(stack_trace_consume_entry, &mut c as *mut _ as *mut _, current, core::ptr::null_mut());
    c.len
}

#[cfg(CONFIG_ARCH_STACKWALK)]
pub unsafe extern "C" fn stack_trace_save_tsk(tsk: *mut task_struct, store: *mut usize, size: u32, skipnr: u32) -> u32 {
    let mut c = StacktraceCookie { store, size, skip: skipnr + (current == tsk) as u32, len: 0 };
    if !try_get_task_stack(tsk) { return 0; }
    arch_stack_walk(stack_trace_consume_entry_nosched, &mut c as *mut _ as *mut _, tsk, core::ptr::null_mut());
    put_task_stack(tsk);
    c.len
}

#[cfg(CONFIG_ARCH_STACKWALK)]
pub unsafe extern "C" fn stack_trace_save_regs(regs: *mut pt_regs, store: *mut usize, size: u32, skipnr: u32) -> u32 {
    let mut c = StacktraceCookie { store, size, skip: skipnr, len: 0 };
    arch_stack_walk(stack_trace_consume_entry, &mut c as *mut _ as *mut _, current, regs);
    c.len
}

#[cfg(all(CONFIG_ARCH_STACKWALK, CONFIG_HAVE_RELIABLE_STACKTRACE))]
pub unsafe extern "C" fn stack_trace_save_tsk_reliable(tsk: *mut task_struct, store: *mut usize, size: u32) -> i32 {
    let mut c = StacktraceCookie { store, size, skip: 0, len: 0 };
    if !try_get_task_stack(tsk) { return 0; }
    let ret = arch_stack_walk_reliable(stack_trace_consume_entry, &mut c as *mut _ as *mut _, tsk);
    put_task_stack(tsk);
    if ret != 0 { ret } else { c.len as i32 }
}

#[cfg(all(CONFIG_ARCH_STACKWALK, CONFIG_USER_STACKTRACE_SUPPORT))]
pub unsafe extern "C" fn stack_trace_save_user(store: *mut usize, size: u32) -> u32 {
    let mut c = StacktraceCookie { store, size, skip: 0, len: 0 };
    // Trace user stack if not a kernel thread.
    if ((*current).flags & PF_KTHREAD) != 0 { return 0; }
    arch_stack_walk_user(stack_trace_consume_entry, &mut c as *mut _ as *mut _, task_pt_regs(current));
    c.len
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
