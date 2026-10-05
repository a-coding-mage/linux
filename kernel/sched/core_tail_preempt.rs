// SPDX-License-Identifier: GPL-2.0-only
// core.c:5962..6039, 7781..8210; dynamic dispatch mutation is per-target native
// static-call/static-key machinery; policy and transition order remain Rust.
#[cfg(all(
    CONFIG_PREEMPTION,
    any(CONFIG_DEBUG_PREEMPT, CONFIG_TRACE_PREEMPT_TOGGLE)
))]
#[inline(always)]
unsafe fn preempt_latency_start(val: c_int) {
    // SAFETY: The native preemption caller supplies the matching count;
    // configured debug/trace state is accessed in source order. Caller-PC capture
    // and instrumentation remain explicit native admission gates.
    unsafe {
        if lupos_core_preempt_count() == val {
            let ip = lupos_core_get_lock_parent_ip();
            #[cfg(CONFIG_DEBUG_PREEMPT)]
            {
                (*lupos_core_current()).preempt_disable_ip = ip;
            }
            // Architecture/compiler caller-address boundary requires native admission.
            #[cfg(CONFIG_TRACE_PREEMPT_TOGGLE)]
            lupos_core_trace_preempt_off_caller(ip);
        }
    }
}
#[cfg(not(all(
    CONFIG_PREEMPTION,
    any(CONFIG_DEBUG_PREEMPT, CONFIG_TRACE_PREEMPT_TOGGLE)
)))]
#[inline(always)]
unsafe fn preempt_latency_start(_val: c_int) {}
#[cfg(all(
    CONFIG_PREEMPTION,
    any(CONFIG_DEBUG_PREEMPT, CONFIG_TRACE_PREEMPT_TOGGLE)
))]
#[no_mangle]
pub unsafe extern "C" fn preempt_count_add(val: c_int) {
    // SAFETY: The native preemption caller owns this CPU's count update;
    // configured debug checks and latency tracking retain their native order.
    unsafe {
        #[cfg(CONFIG_DEBUG_PREEMPT)]
        if !cfg!(CONFIG_HAS_SEPARATE_PREEMPT_RESCHED_BITS)
            && lupos_core_debug_locks_warn_on_site_5990(lupos_core_preempt_count() < 0)
        {
            return;
        }
        lupos_core_preempt_count_add_raw(val);
        #[cfg(CONFIG_DEBUG_PREEMPT)]
        lupos_core_debug_locks_warn_on_site_5999(
            (lupos_core_preempt_count() as c_uint & LUPOS_CORE_PREEMPT_MASK as c_uint)
                >= (LUPOS_CORE_PREEMPT_MASK as c_uint).wrapping_sub(10),
        );
        preempt_latency_start(val);
    }
}
#[cfg(all(
    CONFIG_PREEMPTION,
    any(CONFIG_DEBUG_PREEMPT, CONFIG_TRACE_PREEMPT_TOGGLE)
))]
#[inline(always)]
unsafe fn preempt_latency_stop(_val: c_int) {
    #[cfg(CONFIG_TRACE_PREEMPT_TOGGLE)]
    // SAFETY: The configured trace path follows the native preempt-count
    // contract; caller-address and tracing equivalence still require admission.
    unsafe {
        if lupos_core_preempt_count() == _val {
            lupos_core_trace_preempt_on_caller(lupos_core_get_lock_parent_ip());
        }
    }
}
#[cfg(not(all(
    CONFIG_PREEMPTION,
    any(CONFIG_DEBUG_PREEMPT, CONFIG_TRACE_PREEMPT_TOGGLE)
)))]
#[inline(always)]
unsafe fn preempt_latency_stop(_val: c_int) {}
#[cfg(all(
    CONFIG_PREEMPTION,
    any(CONFIG_DEBUG_PREEMPT, CONFIG_TRACE_PREEMPT_TOGGLE)
))]
#[no_mangle]
pub unsafe extern "C" fn preempt_count_sub(val: c_int) {
    // SAFETY: The native preemption caller owns this CPU's count update;
    // underflow checks precede tracing and the native decrement.
    unsafe {
        #[cfg(CONFIG_DEBUG_PREEMPT)]
        {
            let uval = val as c_uint;
            let pc = lupos_core_preempt_count() as c_uint;
            if lupos_core_debug_locks_warn_on_site_6026(pc.wrapping_sub(uval) > pc) {
                return;
            }
            if lupos_core_debug_locks_warn_on_site_6033(
                val < LUPOS_CORE_PREEMPT_MASK as c_int
                    && lupos_core_preempt_count() & LUPOS_CORE_PREEMPT_MASK as c_int == 0,
            ) {
                return;
            }
        }
        preempt_latency_stop(val);
        lupos_core_preempt_count_sub_raw(val);
    }
}
#[cfg(any(not(CONFIG_PREEMPTION), CONFIG_PREEMPT_DYNAMIC))]
#[no_mangle]
#[link_section = ".sched.text"]
pub unsafe extern "C" fn __cond_resched() -> c_int {
    // SAFETY: The caller satisfies the native conditional-reschedule
    // context; IRQ/preempt checks gate scheduling and native RCU reporting.
    unsafe {
        if lupos_core_should_resched(0) && !lupos_core_irqs_disabled() {
            preempt_schedule_common();
            return 1;
        }
        #[cfg(not(CONFIG_PREEMPT_RCU))]
        lupos_core_rcu_all_qs();
        0
    }
}
#[cfg(all(
    CONFIG_PREEMPTION,
    CONFIG_PREEMPT_DYNAMIC,
    not(CONFIG_HAVE_PREEMPT_DYNAMIC_CALL),
    CONFIG_HAVE_PREEMPT_DYNAMIC_KEY
))]
#[no_mangle]
#[link_section = ".sched.text"]
pub unsafe extern "C" fn dynamic_preempt_schedule() {
    // SAFETY: The native dynamic-preemption entry is valid for current;
    // the configured static key gates the ordinary preemption entry.
    unsafe {
        if !lupos_core_dynamic_preempt_schedule_enabled() {
            return;
        }
        preempt_schedule();
    }
}
#[cfg(all(
    CONFIG_PREEMPTION,
    CONFIG_PREEMPT_DYNAMIC,
    not(CONFIG_HAVE_PREEMPT_DYNAMIC_CALL),
    CONFIG_HAVE_PREEMPT_DYNAMIC_KEY
))]
#[no_mangle]
#[link_section = ".sched.text"]
pub unsafe extern "C" fn dynamic_preempt_schedule_notrace() {
    // SAFETY: The native dynamic tracing entry is valid for current;
    // the key guards the notrace source path without qualifying instrumentation.
    unsafe {
        if !lupos_core_dynamic_preempt_schedule_notrace_enabled() {
            return;
        }
        preempt_schedule_notrace();
    }
}
#[cfg(all(
    CONFIG_PREEMPT_DYNAMIC,
    not(CONFIG_HAVE_PREEMPT_DYNAMIC_CALL),
    CONFIG_HAVE_PREEMPT_DYNAMIC_KEY
))]
#[no_mangle]
#[link_section = ".sched.text"]
pub unsafe extern "C" fn dynamic_cond_resched() -> c_int {
    // SAFETY: The caller meets the native cond-resched contract; its
    // configured static key guards the corresponding scheduling/RCU operation.
    unsafe {
        if !lupos_core_dynamic_cond_resched_enabled() {
            return 0;
        }
        __cond_resched()
    }
}
#[cfg(all(
    CONFIG_PREEMPT_DYNAMIC,
    not(CONFIG_HAVE_PREEMPT_DYNAMIC_CALL),
    CONFIG_HAVE_PREEMPT_DYNAMIC_KEY
))]
#[no_mangle]
#[link_section = ".sched.text"]
pub unsafe extern "C" fn dynamic_might_resched() -> c_int {
    // SAFETY: The caller meets the native might-resched contract; its
    // configured key guards the corresponding scheduling/RCU operation.
    unsafe {
        if !lupos_core_dynamic_might_resched_enabled() {
            return 0;
        }
        __cond_resched()
    }
}
#[no_mangle]
pub unsafe extern "C" fn __cond_resched_lock(lock: *mut spinlock_t) -> c_int {
    // SAFETY: The caller holds a live spinlock and may drop/reacquire it
    // around native conditional scheduling; the function returns with it held.
    unsafe {
        let resched = lupos_core_should_resched(LUPOS_CORE_PREEMPT_LOCK_OFFSET as c_int);
        lupos_core_assert_spin_held(lock);
        if lupos_core_spin_needbreak(lock) || resched {
            lupos_core_spin_unlock(lock);
            if lupos_core_cond_resched() == 0 {
                lupos_core_cpu_relax();
            }
            lupos_core_spin_lock(lock);
            return 1;
        }
        0
    }
}
#[no_mangle]
pub unsafe extern "C" fn __cond_resched_rwlock_read(lock: *mut rwlock_t) -> c_int {
    // SAFETY: The caller holds a live rwlock for reading and may release
    // and reacquire that same mode around native conditional scheduling.
    unsafe {
        let resched = lupos_core_should_resched(LUPOS_CORE_PREEMPT_LOCK_OFFSET as c_int);
        lupos_core_assert_rwlock_held_read(lock);
        if lupos_core_rwlock_needbreak(lock) || resched {
            lupos_core_read_unlock(lock);
            if lupos_core_cond_resched() == 0 {
                lupos_core_cpu_relax();
            }
            lupos_core_read_lock(lock);
            return 1;
        }
        0
    }
}
#[no_mangle]
pub unsafe extern "C" fn __cond_resched_rwlock_write(lock: *mut rwlock_t) -> c_int {
    // SAFETY: The caller holds a live rwlock for writing and may release
    // and reacquire that same mode around native conditional scheduling.
    unsafe {
        let resched = lupos_core_should_resched(LUPOS_CORE_PREEMPT_LOCK_OFFSET as c_int);
        lupos_core_assert_rwlock_held_write(lock);
        if lupos_core_rwlock_needbreak(lock) || resched {
            lupos_core_write_unlock(lock);
            if lupos_core_cond_resched() == 0 {
                lupos_core_cpu_relax();
            }
            lupos_core_write_lock(lock);
            return 1;
        }
        0
    }
}
#[cfg(CONFIG_PREEMPT_DYNAMIC)]
#[no_mangle]
pub unsafe extern "C" fn sched_dynamic_mode(s: *const c_char) -> c_int {
    // SAFETY: s is a live NUL-terminated option string; native strcmp reads
    // it while configured mode names/constants select the result.
    unsafe {
        #[cfg(not(any(CONFIG_PREEMPT_RT, CONFIG_ARCH_HAS_PREEMPT_LAZY)))]
        {
            if strcmp(s, b"none\0".as_ptr().cast()) == 0 {
                return LUPOS_CORE_preempt_dynamic_none;
            }
            if strcmp(s, b"voluntary\0".as_ptr().cast()) == 0 {
                return LUPOS_CORE_preempt_dynamic_voluntary;
            }
        }
        if strcmp(s, b"full\0".as_ptr().cast()) == 0 {
            return LUPOS_CORE_preempt_dynamic_full;
        }
        #[cfg(CONFIG_ARCH_HAS_PREEMPT_LAZY)]
        if strcmp(s, b"lazy\0".as_ptr().cast()) == 0 {
            return LUPOS_CORE_preempt_dynamic_lazy;
        }
        -(LUPOS_CORE_EINVAL as c_int)
    }
}
#[cfg(CONFIG_PREEMPT_DYNAMIC)]
unsafe fn __sched_dynamic_update(mode: c_int) {
    // SAFETY: The caller holds the dynamic-mode mutex. Native per-target
    // key/static-call operations precede publication of the selected mode.
    unsafe {
        // Enable each target first: NONE/VOLUNTARY -> FULL must never pass ZERO.
        lupos_core_dynamic_set_cond_resched(true);
        lupos_core_dynamic_set_might_resched(true);
        lupos_core_dynamic_set_preempt_schedule(true);
        lupos_core_dynamic_set_preempt_schedule_notrace(true);
        lupos_core_dynamic_set_irqentry_exit_cond_resched(true);
        lupos_core_dynamic_set_preempt_lazy(false);
        let name: *const c_char;
        if mode == LUPOS_CORE_preempt_dynamic_none {
            lupos_core_dynamic_set_cond_resched(true);
            lupos_core_dynamic_set_might_resched(false);
            lupos_core_dynamic_set_preempt_schedule(false);
            lupos_core_dynamic_set_preempt_schedule_notrace(false);
            lupos_core_dynamic_set_irqentry_exit_cond_resched(false);
            lupos_core_dynamic_set_preempt_lazy(false);
            name = b"Dynamic Preempt: none\n\0".as_ptr().cast();
        } else if mode == LUPOS_CORE_preempt_dynamic_voluntary {
            lupos_core_dynamic_set_cond_resched(true);
            lupos_core_dynamic_set_might_resched(true);
            lupos_core_dynamic_set_preempt_schedule(false);
            lupos_core_dynamic_set_preempt_schedule_notrace(false);
            lupos_core_dynamic_set_irqentry_exit_cond_resched(false);
            lupos_core_dynamic_set_preempt_lazy(false);
            name = b"Dynamic Preempt: voluntary\n\0".as_ptr().cast();
        } else if mode == LUPOS_CORE_preempt_dynamic_full {
            lupos_core_dynamic_set_cond_resched(false);
            lupos_core_dynamic_set_might_resched(false);
            lupos_core_dynamic_set_preempt_schedule(true);
            lupos_core_dynamic_set_preempt_schedule_notrace(true);
            lupos_core_dynamic_set_irqentry_exit_cond_resched(true);
            lupos_core_dynamic_set_preempt_lazy(false);
            name = b"Dynamic Preempt: full\n\0".as_ptr().cast();
        } else if mode == LUPOS_CORE_preempt_dynamic_lazy {
            lupos_core_dynamic_set_cond_resched(false);
            lupos_core_dynamic_set_might_resched(false);
            lupos_core_dynamic_set_preempt_schedule(true);
            lupos_core_dynamic_set_preempt_schedule_notrace(true);
            lupos_core_dynamic_set_irqentry_exit_cond_resched(true);
            lupos_core_dynamic_set_preempt_lazy(true);
            name = b"Dynamic Preempt: lazy\n\0".as_ptr().cast();
        } else {
            name = core::ptr::null();
        }
        if !name.is_null() && mode != preempt_dynamic_mode {
            lupos_core_pr_info(name);
        }
        lupos_core_write_once_int(addr_of_mut!(preempt_dynamic_mode), mode);
    }
}
#[cfg(CONFIG_PREEMPT_DYNAMIC)]
#[no_mangle]
pub unsafe extern "C" fn sched_dynamic_update(mode: c_int) {
    // SAFETY: The native configuration caller may acquire the mode mutex;
    // it serializes per-target updates and mode publication through completion.
    unsafe {
        let mutex = lupos_core_sched_dynamic_mutex();
        lupos_core_mutex_lock(mutex);
        __sched_dynamic_update(mode);
        lupos_core_mutex_unlock(mutex);
    }
}
#[cfg(CONFIG_PREEMPT_DYNAMIC)]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn setup_preempt_mode(s: *mut c_char) -> c_int {
    // SAFETY: The boot caller supplies a live NUL-terminated option string;
    // parsing, diagnostics and serialized mode setup finish before it expires.
    unsafe {
        let mode = sched_dynamic_mode(s);
        if mode < 0 {
            _printk(
                b"\x014Dynamic Preempt: unsupported mode: %s\n\0"
                    .as_ptr()
                    .cast(),
                s,
            );
            return 0;
        }
        sched_dynamic_update(mode);
        1
    }
}
#[cfg(CONFIG_PREEMPT_DYNAMIC)]
#[link_section = ".init.text"]
unsafe fn preempt_dynamic_init() {
    // SAFETY: The boot caller initializes dynamic preemption before normal
    // configuration changes, using the native default-target/key contract.
    unsafe {
        if preempt_dynamic_mode == LUPOS_CORE_preempt_dynamic_undefined {
            if cfg!(CONFIG_PREEMPT_NONE) {
                sched_dynamic_update(LUPOS_CORE_preempt_dynamic_none);
            } else if cfg!(CONFIG_PREEMPT_VOLUNTARY) {
                sched_dynamic_update(LUPOS_CORE_preempt_dynamic_voluntary);
            } else if cfg!(CONFIG_PREEMPT_LAZY) {
                sched_dynamic_update(LUPOS_CORE_preempt_dynamic_lazy);
            } else {
                lupos_core_warn_once_site_8084(!cfg!(CONFIG_PREEMPT));
                preempt_dynamic_mode = LUPOS_CORE_preempt_dynamic_full;
                lupos_core_pr_info(b"Dynamic Preempt: full\n\0".as_ptr().cast());
            }
        }
    }
}
#[cfg(not(CONFIG_PREEMPT_DYNAMIC))]
unsafe fn preempt_dynamic_init() {}
#[cfg(CONFIG_PREEMPT_DYNAMIC)]
macro_rules! tail_preempt_model {
    ($name:ident, $value:ident, $warn:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name() -> bool {
            // SAFETY: The native mode lives in permanent storage; its
            // READ_ONCE snapshot and source-specific diagnostic retain the
            // configured dynamic-preemption accessor contract.
            unsafe {
                let mode = lupos_core_read_once_int(addr_of!(preempt_dynamic_mode));
                $warn(mode == LUPOS_CORE_preempt_dynamic_undefined);
                mode == $value
            }
        }
    };
}
#[cfg(CONFIG_PREEMPT_DYNAMIC)]
tail_preempt_model!(
    preempt_model_none,
    LUPOS_CORE_preempt_dynamic_none,
    lupos_core_warn_preempt_model_none
);
#[cfg(CONFIG_PREEMPT_DYNAMIC)]
tail_preempt_model!(
    preempt_model_voluntary,
    LUPOS_CORE_preempt_dynamic_voluntary,
    lupos_core_warn_preempt_model_voluntary
);
#[cfg(CONFIG_PREEMPT_DYNAMIC)]
tail_preempt_model!(
    preempt_model_full,
    LUPOS_CORE_preempt_dynamic_full,
    lupos_core_warn_preempt_model_full
);
#[cfg(CONFIG_PREEMPT_DYNAMIC)]
tail_preempt_model!(
    preempt_model_lazy,
    LUPOS_CORE_preempt_dynamic_lazy,
    lupos_core_warn_preempt_model_lazy
);
#[no_mangle]
pub unsafe extern "C" fn preempt_model_str() -> *const c_char {
    // SAFETY: The native mode/read contract keeps indices valid; the local
    // seq_buf describes permanent native output storage, not a returned stack buffer.
    unsafe {
        let brace =
            cfg!(CONFIG_PREEMPT_RT) && (cfg!(CONFIG_PREEMPT_DYNAMIC) || cfg!(CONFIG_PREEMPT_LAZY));
        if cfg!(CONFIG_PREEMPT_BUILD) {
            let mut seq = MaybeUninit::<seq_buf>::uninit();
            let seq = seq.as_mut_ptr();
            lupos_core_seq_buf_init(seq, lupos_core_preempt_model_buffer(), 128);
            seq_buf_puts(seq, b"PREEMPT\0".as_ptr().cast());
            if cfg!(CONFIG_PREEMPT_RT) {
                seq_buf_puts(
                    seq,
                    if brace {
                        b"_{RT,\0".as_ptr()
                    } else {
                        b"_RT\0".as_ptr()
                    }
                    .cast(),
                );
            }
            #[cfg(CONFIG_PREEMPT_DYNAMIC)]
            {
                seq_buf_puts(seq, b"(\0".as_ptr().cast());
                let name = if preempt_dynamic_mode >= 0 {
                    *lupos_core_preempt_modes().add(preempt_dynamic_mode as usize)
                } else {
                    b"undef\0".as_ptr().cast()
                };
                seq_buf_puts(seq, name);
                seq_buf_puts(
                    seq,
                    if brace {
                        b")}\0".as_ptr()
                    } else {
                        b")\0".as_ptr()
                    }
                    .cast(),
                );
                return lupos_core_seq_buf_str(seq);
            }
            #[cfg(not(CONFIG_PREEMPT_DYNAMIC))]
            {
                if cfg!(CONFIG_PREEMPT_LAZY) {
                    seq_buf_puts(
                        seq,
                        if brace {
                            b"LAZY}\0".as_ptr()
                        } else {
                            b"LAZY\0".as_ptr()
                        }
                        .cast(),
                    );
                }
                return lupos_core_seq_buf_str(seq);
            }
        }
        if cfg!(CONFIG_PREEMPT_VOLUNTARY_BUILD) {
            b"VOLUNTARY\0".as_ptr().cast()
        } else {
            b"NONE\0".as_ptr().cast()
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn io_schedule_prepare() -> c_int {
    // SAFETY: The caller owns current's I/O-wait token transition and may
    // flush its plug under the native I/O scheduling contract.
    unsafe {
        let current = lupos_core_current();
        let old = lupos_core_task_in_iowait(current);
        lupos_core_set_task_in_iowait(current, 1);
        lupos_core_header_blk_flush_plug((*current).plug, true);
        old
    }
}
#[no_mangle]
pub unsafe extern "C" fn io_schedule_finish(token: c_int) {
    // SAFETY: token comes from this task's matching io_schedule_prepare;
    // the native setter restores current's previous I/O-wait state.
    unsafe {
        lupos_core_set_task_in_iowait(lupos_core_current(), token);
    }
}
#[no_mangle]
#[link_section = ".sched.text"]
pub unsafe extern "C" fn io_schedule_timeout(timeout: c_long) -> c_long {
    // SAFETY: The caller may sleep for the native timeout; current remains
    // live and its I/O-wait token is restored after schedule_timeout returns.
    unsafe {
        let token = io_schedule_prepare();
        let ret = schedule_timeout(timeout);
        io_schedule_finish(token);
        ret
    }
}
#[no_mangle]
#[link_section = ".sched.text"]
pub unsafe extern "C" fn io_schedule() {
    // SAFETY: The caller may sleep in I/O context; current remains live and
    // its native I/O-wait token is restored after schedule returns.
    unsafe {
        let token = io_schedule_prepare();
        schedule();
        io_schedule_finish(token);
    }
}
