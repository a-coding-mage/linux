// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation unit.

#[cfg(CONFIG_JUMP_LABEL)]
#[repr(C)]
pub struct static_key {
    _private: [u8; 0],
}

#[cfg(CONFIG_JUMP_LABEL)]
extern "C" {
    fn static_key_slow_inc(key: *mut static_key);
    fn static_key_slow_dec(key: *mut static_key);
}

#[cfg(CONFIG_JUMP_LABEL)]
#[no_mangle]
pub static mut opal_tracepoint_key: static_key = static_key { _private: [] };

#[cfg(CONFIG_JUMP_LABEL)]
#[no_mangle]
pub unsafe extern "C" fn opal_tracepoint_regfunc() -> i32 {
    static_key_slow_inc(&raw mut opal_tracepoint_key);
    0
}

#[cfg(CONFIG_JUMP_LABEL)]
#[no_mangle]
pub unsafe extern "C" fn opal_tracepoint_unregfunc() {
    static_key_slow_dec(&raw mut opal_tracepoint_key);
}

#[cfg(not(CONFIG_JUMP_LABEL))]
extern "C" {
    pub static mut opal_tracepoint_refcount: i64;
}

#[cfg(not(CONFIG_JUMP_LABEL))]
#[no_mangle]
pub unsafe extern "C" fn opal_tracepoint_regfunc() -> i32 {
    opal_tracepoint_refcount = opal_tracepoint_refcount.wrapping_add(1);
    0
}

#[cfg(not(CONFIG_JUMP_LABEL))]
#[no_mangle]
pub unsafe extern "C" fn opal_tracepoint_unregfunc() {
    opal_tracepoint_refcount = opal_tracepoint_refcount.wrapping_sub(1);
}

// Since the tracing code might execute OPAL calls we need to guard against
// recursion.
//
// This is a per-CPU variable in the original implementation.
#[no_mangle]
pub static mut opal_trace_depth: u32 = 0;

extern "C" {
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn this_cpu_ptr(ptr: *mut u32) -> *mut u32;
    fn preempt_disable();
    fn preempt_enable();
    fn trace_opal_entry(opcode: usize, args: *mut usize);
    fn trace_opal_exit(opcode: i64, retval: usize);
}

#[no_mangle]
pub unsafe extern "C" fn __trace_opal_entry(opcode: usize, args: *mut usize) {
    let mut flags: usize = 0;
    let depth: *mut u32;

    local_irq_save(&mut flags);

    depth = this_cpu_ptr(&raw mut opal_trace_depth);

    if *depth != 0 {
        local_irq_restore(flags);
        return;
    }

    *depth = (*depth).wrapping_add(1);
    preempt_disable();
    trace_opal_entry(opcode, args);
    *depth = (*depth).wrapping_sub(1);

    local_irq_restore(flags);
}

#[no_mangle]
pub unsafe extern "C" fn __trace_opal_exit(opcode: i64, retval: usize) {
    let mut flags: usize = 0;
    let depth: *mut u32;

    local_irq_save(&mut flags);

    depth = this_cpu_ptr(&raw mut opal_trace_depth);

    if *depth != 0 {
        local_irq_restore(flags);
        return;
    }

    *depth = (*depth).wrapping_add(1);
    trace_opal_exit(opcode, retval);
    preempt_enable();
    *depth = (*depth).wrapping_sub(1);

    local_irq_restore(flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
