// SPDX-License-Identifier: GPL-2.0
//
// Translated from C implementation source. Original includes referenced:
// linux/compiler.h, linux/types.h, linux/zalloc.h, inttypes.h, limits.h,
// unistd.h, tests.h, debug.h, env.h, machine.h, event.h, ../util/unwind.h,
// perf_regs.h, map.h, symbol.h, thread.h, callchain.h, stdlib.h.

use core::ffi::{c_char, c_int, c_ulong, c_void};

/*
 * The C source conditionally uses __attribute__((disable_tail_calls)) when
 * available, otherwise an asm volatile memory barrier. Rust has no stable,
 * file-local equivalent for the function attribute, so the translated callees
 * keep the no-inline intent and use compiler_fence where the C fallback barrier
 * was placed.
 */

/*
 * We need to keep these functions global, despite the
 * fact that they are used only locally in this object,
 * in order to keep them around even if the binary is
 * stripped. If they are gone, the unwind check for
 * symbol fails.
 */

const MAX_STACK: usize = 8;
const INT_MAX: c_int = c_int::MAX;

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_env {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    pub name: *mut c_char,
}

#[repr(C)]
pub struct map_symbol {
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct unwind_entry {
    pub ip: u64,
    pub ms: map_symbol,
}

#[repr(C)]
pub struct perf_sample_stack {
    pub data: *mut c_void,
}

#[repr(C)]
pub struct perf_sample_regs {
    pub regs: *mut c_void,
}

#[repr(C)]
pub struct perf_sample {
    pub user_stack: perf_sample_stack,
    pub user_regs: *mut perf_sample_regs,
}

#[repr(C)]
pub struct callchain_param_t {
    pub record_mode: c_int,
    pub order: c_int,
}

unsafe extern "C" {
    static ORDER_CALLER: c_int;
    static ORDER_CALLEE: c_int;
    static CALLCHAIN_DWARF: c_int;

    static mut callchain_param: callchain_param_t;
    static mut dwarf_callchain_users: bool;
    static mut verbose: c_int;
    static mut stderr: *mut c_void;

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn getpid() -> c_int;
    fn bsearch(
        key: *mut c_void,
        base: *mut c_void,
        nmemb: usize,
        size: usize,
        compar: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> c_int>,
    ) -> *mut c_void;

    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn zfree(ptr: *mut *mut c_void);

    fn perf_sample__init(sample: *mut perf_sample, all: bool);
    fn perf_sample__exit(sample: *mut perf_sample);
    fn test__arch_unwind_sample(sample: *mut perf_sample, thread: *mut thread) -> c_int;
    fn unwind__get_entries(
        cb: Option<unsafe extern "C" fn(*mut unwind_entry, *mut c_void) -> c_int>,
        arg: *mut c_void,
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: usize,
        best_effort: bool,
    ) -> c_int;

    fn perf_env__init(env: *mut perf_env);
    fn perf_env__exit(env: *mut perf_env);
    fn machine__new_live(env: *mut perf_env, kernel_maps: bool, pid: c_int) -> *mut machine;
    fn machine__create_kernel_maps(machine: *mut machine) -> c_int;
    fn machine__fprintf(machine: *mut machine, fp: *mut c_void);
    fn machine__find_thread(machine: *mut machine, pid: c_int, tid: c_int) -> *mut thread;
    fn thread__put(thread: *mut thread);
    fn machine__delete(machine: *mut machine);
}

unsafe extern "C" fn unwind_entry(entry: *mut unwind_entry, arg: *mut c_void) -> c_int {
    let cnt = arg as *mut c_ulong;
    let symbol = if !(*entry).ms.sym.is_null() {
        (*(*entry).ms.sym).name
    } else {
        core::ptr::null_mut()
    };
    let funcs: [*const c_char; MAX_STACK] = [
        c"test__arch_unwind_sample".as_ptr(),
        c"test_dwarf_unwind__thread".as_ptr(),
        c"test_dwarf_unwind__compare".as_ptr(),
        c"bsearch".as_ptr(),
        c"test_dwarf_unwind__krava_3".as_ptr(),
        c"test_dwarf_unwind__krava_2".as_ptr(),
        c"test_dwarf_unwind__krava_1".as_ptr(),
        c"test__dwarf_unwind".as_ptr(),
    ];
    /*
     * The funcs[MAX_STACK] array index, based on the
     * callchain order setup.
     */
    let idx: c_ulong = if callchain_param.order == ORDER_CALLER {
        (MAX_STACK as c_ulong).wrapping_sub(*cnt).wrapping_sub(1)
    } else {
        *cnt
    };

    if *cnt >= MAX_STACK as c_ulong {
        pr_debug(c"failed: crossed the max stack value %d\n".as_ptr(), MAX_STACK as c_int);
        return -1;
    }

    if symbol.is_null() {
        pr_debug(
            c"failed: got unresolved address 0x%lx\n".as_ptr(),
            (*entry).ip as c_ulong,
        );
        return -1;
    }

    *cnt = (*cnt).wrapping_add(1);
    pr_debug(
        c"got: %s 0x%lx, expecting %s\n".as_ptr(),
        symbol,
        (*entry).ip as c_ulong,
        funcs[idx as usize],
    );
    strcmp(symbol as *const c_char, funcs[idx as usize])
}

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn test_dwarf_unwind__thread(thread: *mut thread) -> c_int {
    let mut sample: perf_sample = core::mem::zeroed();
    let mut cnt: c_ulong = 0;
    let mut err: c_int = -1;

    perf_sample__init(&mut sample, true);
    if test__arch_unwind_sample(&mut sample, thread) != 0 {
        pr_debug(c"failed to get unwind sample\n".as_ptr());
        goto_out(&mut sample);
        return err;
    }

    err = unwind__get_entries(
        Some(unwind_entry),
        &mut cnt as *mut c_ulong as *mut c_void,
        thread,
        &mut sample,
        MAX_STACK,
        false,
    );
    if err != 0 {
        pr_debug(c"unwind failed\n".as_ptr());
    } else if cnt != MAX_STACK as c_ulong {
        pr_debug(
            c"got wrong number of stack entries %lu != %d\n".as_ptr(),
            cnt,
            MAX_STACK as c_int,
        );
        err = -1;
    }

    goto_out(&mut sample);
    err
}

unsafe fn goto_out(sample: *mut perf_sample) {
    zfree(&mut (*sample).user_stack.data);
    zfree(&mut (*(*sample).user_regs).regs);
    perf_sample__exit(sample);
}

static mut global_unwind_retval: c_int = -INT_MAX;

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn test_dwarf_unwind__compare(
    p1: *mut c_void,
    p2: *mut c_void,
) -> c_int {
    /* Any possible value should be 'thread' */
    let thread = *(p1 as *const *mut thread);

    if global_unwind_retval == -INT_MAX {
        /* Call unwinder twice for both callchain orders. */
        callchain_param.order = ORDER_CALLER;

        global_unwind_retval = test_dwarf_unwind__thread(thread);
        if global_unwind_retval == 0 {
            callchain_param.order = ORDER_CALLEE;
            global_unwind_retval = test_dwarf_unwind__thread(thread);
        }
    }

    (p1 as isize).wrapping_sub(p2 as isize) as c_int
}

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn test_dwarf_unwind__krava_3(thread: *mut thread) -> c_int {
    let mut array: [*mut thread; 2] = [thread, thread];
    let fp = bsearch as *mut c_void;
    /*
     * make _bsearch a volatile function pointer to
     * prevent potential optimization, which may expand
     * bsearch and call compare directly from this function,
     * instead of libc shared object.
     */
    let mut _bsearch: unsafe extern "C" fn(
        *mut c_void,
        *mut c_void,
        usize,
        usize,
        Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> c_int>,
    ) -> *mut c_void;

    core::ptr::write_volatile(&mut _bsearch, core::mem::transmute(fp));
    core::ptr::read_volatile(&_bsearch)(
        array.as_mut_ptr() as *mut c_void,
        &thread as *const *mut thread as *mut c_void,
        2,
        core::mem::size_of::<*mut thread>(),
        Some(test_dwarf_unwind__compare),
    );
    global_unwind_retval
}

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn test_dwarf_unwind__krava_2(thread: *mut thread) -> c_int {
    let ret: c_int;

    ret = test_dwarf_unwind__krava_3(thread);
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    ret
}

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn test_dwarf_unwind__krava_1(thread: *mut thread) -> c_int {
    let ret: c_int;

    ret = test_dwarf_unwind__krava_2(thread);
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    ret
}

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn test__dwarf_unwind(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut host_env: perf_env = core::mem::zeroed();
    let mut machine: *mut machine;
    let mut thread: *mut thread;
    let mut err: c_int = -1;
    let pid = getpid();

    callchain_param.record_mode = CALLCHAIN_DWARF;
    dwarf_callchain_users = true;

    perf_env__init(&mut host_env);
    machine = machine__new_live(&mut host_env, true, pid);
    if machine.is_null() {
        pr_err(c"Could not get machine\n".as_ptr());
        machine__delete(machine);
        perf_env__exit(&mut host_env);
        return err;
    }

    if machine__create_kernel_maps(machine) != 0 {
        pr_err(c"Failed to create kernel maps\n".as_ptr());
        machine__delete(machine);
        perf_env__exit(&mut host_env);
        return err;
    }

    if verbose > 1 {
        machine__fprintf(machine, stderr);
    }

    thread = machine__find_thread(machine, pid, pid);
    if thread.is_null() {
        pr_err(c"Could not get thread\n".as_ptr());
        machine__delete(machine);
        perf_env__exit(&mut host_env);
        return err;
    }

    err = test_dwarf_unwind__krava_1(thread);
    thread__put(thread);

    machine__delete(machine);
    perf_env__exit(&mut host_env);
    err
}

// DEFINE_SUITE("Test dwarf unwind", dwarf_unwind);
