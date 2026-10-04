// SPDX-License-Identifier: GPL-2.0
// Faithful source translation of kernel/irq/handle.c at be59db382996.
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    missing_docs,
    unsafe_op_in_unsafe_fn,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_pub,
    clippy::all
)]
include!("irq_core.rs");

// kernel/irq/debug.h: native leaves retain the ratelimit state and typed printk
// callsites; Rust performs the exact gate, null check and ordered flag tests.
#[no_mangle]
pub unsafe extern "C" fn lupos_irq_print_desc(irq: c_uint, desc: *mut irq_desc) {
    if !lupos_irq_print_desc_ratelimit() {
        return;
    }
    lupos_irq_print_desc_summary(irq, desc);
    lupos_irq_print_desc_flow(desc);
    lupos_irq_print_desc_chip(desc);
    lupos_irq_print_desc_action(desc);
    if !(*desc).action.is_null() {
        lupos_irq_print_desc_handler(desc);
    }
    if setting(desc, _IRQ_LEVEL) {
        lupos_irq_print_desc_level();
    }
    if setting(desc, _IRQ_PER_CPU) {
        lupos_irq_print_desc_per_cpu();
    }
    if setting(desc, _IRQ_NOPROBE) {
        lupos_irq_print_desc_noprobe();
    }
    if setting(desc, _IRQ_NOREQUEST) {
        lupos_irq_print_desc_norequest();
    }
    if setting(desc, _IRQ_NOTHREAD) {
        lupos_irq_print_desc_nothread();
    }
    if setting(desc, _IRQ_NOAUTOEN) {
        lupos_irq_print_desc_noautoen();
    }
    if *state(desc) & IRQS_AUTODETECT != 0 {
        lupos_irq_print_desc_autodetect();
    }
    if *state(desc) & IRQS_REPLAY != 0 {
        lupos_irq_print_desc_replay();
    }
    if *state(desc) & IRQS_WAITING != 0 {
        lupos_irq_print_desc_waiting();
    }
    if *state(desc) & IRQS_PENDING != 0 {
        lupos_irq_print_desc_pending();
    }
    // ___PD(IRQS_INPROGRESS/DISABLED/MASKED) intentionally emits nothing.
}

#[cfg(CONFIG_GENERIC_IRQ_MULTI_HANDLER)]
#[no_mangle]
#[link_section = ".data..ro_after_init"]
pub static mut handle_arch_irq: Option<unsafe extern "C" fn(*mut pt_regs)> = None;

#[no_mangle]
pub unsafe extern "C" fn handle_bad_irq(desc: *mut irq_desc) {
    let irq = (*desc).irq_data.irq;
    lupos_irq_print_desc(irq, desc);
    lupos_irq_kstat_incr(desc);
    lupos_irq_ack_bad(irq);
}
#[no_mangle]
pub unsafe extern "C" fn no_action(_: c_int, _: *mut c_void) -> irqreturn_t {
    IRQ_NONE
}

unsafe fn warn_no_thread(irq: c_uint, action: *mut irqaction) {
    if lupos_irq_test_set_bit(IRQTF_WARNED, addr_of_mut!((*action).thread_flags)) {
        return;
    }
    lupos_irq_warn_no_thread(irq, (*action).name);
}
#[no_mangle]
pub unsafe extern "C" fn __irq_wake_thread(desc: *mut irq_desc, action: *mut irqaction) {
    if (*(*action).thread).flags & PF_EXITING != 0 {
        return;
    }
    if lupos_irq_test_set_bit(IRQTF_RUNTHREAD, addr_of_mut!((*action).thread_flags)) {
        return;
    }
    // INPROGRESS serializes the hardirq writer against finalize_oneshot(); the
    // action thread's RUNTHREAD bit is set before this lockless mask update.
    (*desc).threads_oneshot |= (*action).thread_mask;
    lupos_irq_atomic_inc(addr_of_mut!((*desc).threads_active));
    wake_up_state((*action).thread, TASK_INTERRUPTIBLE);
}

#[link_section = ".data..ro_after_init"]
static mut irqhandler_duration_threshold_ns: u64 = 0;
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn lupos_irq_duration_setup(arg: *mut c_char) -> c_int {
    let mut val: c_ulong = 0;
    let ret = lupos_irq_parse_ulong(arg, 0, &mut val);
    if ret != 0 {
        lupos_irq_duration_parse_error(ret);
        return 0;
    }
    if val == 0 {
        lupos_irq_duration_zero_error();
        return 0;
    }
    irqhandler_duration_threshold_ns = val.wrapping_mul(1000) as u64;
    lupos_irq_duration_enable();
    1
}
unsafe fn irqhandler_duration_check(ts_start: u64, irq: c_uint, action: *const irqaction) {
    let delta_ns = lupos_irq_local_clock().wrapping_sub(ts_start);
    if delta_ns > irqhandler_duration_threshold_ns {
        lupos_irq_duration_warn(irq, action, delta_ns / NSEC_PER_USEC as u64);
    }
}
#[no_mangle]
pub unsafe extern "C" fn __handle_irq_event_percpu(desc: *mut irq_desc) -> irqreturn_t {
    let mut retval = IRQ_NONE;
    let irq = (*desc).irq_data.irq;
    let mut action = (*desc).action;
    while !action.is_null() {
        if lupos_irq_can_thread(desc)
            && (*action).flags & (IRQF_NO_THREAD | IRQF_PERCPU | IRQF_ONESHOT) == 0
        {
            lupos_irq_lockdep_threaded();
        }
        lupos_irq_trace_entry(irq, action);
        let res = if lupos_irq_duration_enabled() {
            let start = lupos_irq_local_clock();
            let ret = ((*action).handler.unwrap_unchecked())(
                irq as c_int,
                (*action).__bindgen_anon_1.dev_id,
            );
            irqhandler_duration_check(start, irq, action);
            ret
        } else {
            ((*action).handler.unwrap_unchecked())(irq as c_int, (*action).__bindgen_anon_1.dev_id)
        };
        lupos_irq_trace_exit(irq, action, res);
        if lupos_irq_warn_enabled(irq, action) {
            lupos_irq_local_disable();
        }
        if res.0 == IRQ_WAKE_THREAD.0 {
            if (*action).thread_fn.is_none() {
                warn_no_thread(irq, action);
            } else {
                __irq_wake_thread(desc, action);
            }
        }
        retval.0 |= res.0;
        action = (*action).next;
    }
    retval
}
#[no_mangle]
pub unsafe extern "C" fn handle_irq_event_percpu(desc: *mut irq_desc) -> irqreturn_t {
    let retval = __handle_irq_event_percpu(desc);
    add_interrupt_randomness((*desc).irq_data.irq as c_int);
    if !lupos_irq_no_debug(desc) {
        note_interrupt(desc, retval);
    }
    retval
}
#[no_mangle]
pub unsafe extern "C" fn handle_irq_event(desc: *mut irq_desc) -> irqreturn_t {
    *state(desc) &= !IRQS_PENDING;
    lupos_irq_data_set(data(desc), IRQD_IRQ_INPROGRESS);
    lupos_irq_raw_unlock(addr_of_mut!((*desc).lock));
    let ret = handle_irq_event_percpu(desc);
    lupos_irq_raw_lock(addr_of_mut!((*desc).lock));
    lupos_irq_data_clear(data(desc), IRQD_IRQ_INPROGRESS);
    ret
}
#[cfg(CONFIG_GENERIC_IRQ_MULTI_HANDLER)]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn set_handle_irq(
    handler: Option<unsafe extern "C" fn(*mut pt_regs)>,
) -> c_int {
    if handle_arch_irq.is_some() {
        return -(EBUSY as c_int);
    }
    handle_arch_irq = handler;
    0
}
#[cfg(CONFIG_GENERIC_IRQ_MULTI_HANDLER)]
#[no_mangle]
#[link_section = ".noinstr.text"]
#[inline(never)]
pub unsafe extern "C" fn generic_handle_arch_irq(regs: *mut pt_regs) {
    irq_enter();
    let old = lupos_irq_set_regs(regs);
    (handle_arch_irq.unwrap_unchecked())(regs);
    lupos_irq_set_regs(old);
    irq_exit();
}

// STATIC_KEY_FALSE_INIT: ATOMIC_INIT(0) and the header-derived jump type.
#[no_mangle]
pub static mut lupos_irq_duration_key: static_key_false = {
    let mut key: static_key_false = unsafe { zeroed() };
    #[cfg(CONFIG_JUMP_LABEL)]
    {
        key.key.__bindgen_anon_1.type_ = LUPOS_IRQ_JUMP_TYPE_FALSE;
    }
    key
};
const _: () = {
    assert!(size_of::<static_key_false>() == LUPOS_IRQ_KEY_SIZE as usize);
    assert!(core::mem::align_of::<static_key_false>() == LUPOS_IRQ_KEY_ALIGN as usize);
};
