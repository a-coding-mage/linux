// SPDX-License-Identifier: GPL-2.0-only
// Owned inline algorithms from the original configured Linux headers:
//   uaccess.h:393-415          copy_struct_from_user
//   ptrace.h:201-220           ptrace_init_task
//   rseq.h:132-162             rseq_reset and rseq_fork
//   posix-timers.h:101-107     posix_cputimers_init
// Included in fork.rs with generated b types and the parent's current_task().
// ABI scalar aliases deliberately use kernel::ffi, matching kernel bindings.
//
// Integration: replace the old C algorithm adapters at the four call sites.
// The user-copy call adds a fifth parameter: the original destination object
// size known at the call site (size_of::<clone_args>() in copy_clone_args_from_user).
// This preserves __builtin_object_size(dst, 1)'s guard after moving ownership
// across languages. An untyped-pointer C helper cannot recover that bound.
// All size/tail/error decisions, tracing policy, rseq inheritance/reset, and
// CPU-timer initialization are below in Rust. C provides only primitives.
// CONFIG_RSEQ and CONFIG_POSIX_TIMERS disabled bodies match the header no-ops.
// Source-reviewed only; configured binding and compiler validation are pending.

unsafe fn fork_copy_struct_from_user(
    dst: *mut kernel::ffi::c_void,
    ksize: usize,
    src: *const kernel::ffi::c_void,
    user_size: usize,
    dst_object_size: usize,
) -> kernel::ffi::c_int {
    let size = core::cmp::min(ksize, user_size);
    let rest = core::cmp::max(ksize, user_size) - size;

    // Equivalent to WARN_ON_ONCE(ksize > __builtin_object_size(dst, 1)).
    // The bound is captured by the typed Rust caller before pointer erasure.
    if rust_fork_header_warn_copy_bound(ksize > dst_object_size) {
        return -(E2BIG as kernel::ffi::c_int);
    }

    if user_size < ksize {
        // An older userspace structure receives zero defaults in new fields.
        core::ptr::write_bytes(dst.cast::<u8>().add(size), 0, rest);
    } else if user_size > ksize {
        // User pointers need not point into a valid Rust allocation; wrapping
        // address arithmetic retains kernel uaccess handling of bad addresses.
        let tail = src.cast::<u8>().wrapping_add(size).cast();
        let ret = b::check_zeroed_user(tail, rest as _);
        if ret <= 0 {
            return if ret != 0 {
                ret
            } else {
                -(E2BIG as kernel::ffi::c_int)
            };
        }
    }

    if rust_fork_header_copy_from_user(dst, src, size as _) != 0 {
        return -(EFAULT as kernel::ffi::c_int);
    }
    0
}

// INIT_LIST_HEAD's two WRITE_ONCE stores. The self-links are initialized in
// Rust before any ptrace relationship is published by __ptrace_link.
unsafe fn fork_header_init_list_head(head: *mut list_head) {
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*head).next), head);
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*head).prev), head);
}

// The sigaddset initializer needed by ptrace_init_task. Word count and word
// width are read from the actual architecture's signal-set definitions; no
// architecture layout or bit width is assumed. Only SIGSTOP is passed here.
unsafe fn fork_header_sigaddset(set: *mut sigset_t, signal: kernel::ffi::c_int) {
    let bit = (signal - 1) as usize;
    let words = rust_fork_header_signal_words(set);
    if rust_fork_header_signal_word_count() == 1 {
        *words |= (1 as kernel::ffi::c_ulong) << bit;
    } else {
        let bits_per_word = rust_fork_header_signal_word_bits() as usize;
        *words.add(bit / bits_per_word) |= (1 as kernel::ffi::c_ulong) << (bit % bits_per_word);
    }
}

// Caller holds current's sighand lock and write_lock_irq(&tasklist_lock).
unsafe fn fork_ptrace_init_task(child: *mut task_struct, trace_child: bool) {
    fork_header_init_list_head(rust_fork_header_task_ptrace_entry(child));
    fork_header_init_list_head(rust_fork_header_task_ptraced(child));
    *rust_fork_header_task_jobctl(child) = 0;
    *rust_fork_header_task_ptrace(child) = 0;
    *rust_fork_header_task_parent(child) = *rust_fork_header_task_real_parent(child);

    let current = current_task();
    if trace_child && *rust_fork_header_task_ptrace(current) != 0 {
        *rust_fork_header_task_ptrace(child) = *rust_fork_header_task_ptrace(current);
        b::__ptrace_link(
            child,
            *rust_fork_header_task_parent(current),
            *rust_fork_header_task_ptracer_cred(current),
        );
        if *rust_fork_header_task_ptrace(child) & PT_SEIZED as kernel::ffi::c_uint != 0 {
            b::task_set_jobctl_pending(child, JOBCTL_TRAP_STOP as _);
        } else {
            fork_header_sigaddset(
                rust_fork_header_pending_signal(child),
                SIGSTOP as kernel::ffi::c_int,
            );
        }
    } else {
        *rust_fork_header_task_ptracer_cred(child) = core::ptr::null();
    }
}

#[cfg(CONFIG_RSEQ)]
unsafe fn fork_rseq_reset(task: *mut task_struct) {
    // Matches guard(irqsave): exclude preemption and membarrier IPIs while
    // clearing the state, then restore exactly the caller's incoming flags.
    let flags = rust_fork_header_irq_save();
    core::ptr::write_bytes(rust_fork_header_task_rseq(task), 0, 1);
    *rust_fork_header_rseq_cpu_id(task) = rust_fork_header_rseq_cpu_uninitialized();
    rust_fork_header_irq_restore(flags);
}

#[cfg_attr(not(CONFIG_RSEQ), allow(unused_variables))]
unsafe fn fork_rseq_fork(task: *mut task_struct, clone_flags: u64) {
    #[cfg(CONFIG_RSEQ)]
    {
        if clone_flags & CLONE_VM as u64 != 0 {
            fork_rseq_reset(task);
        } else {
            // Keep the parent's cached CPU/MMCID to avoid an unnecessary COW
            // fault before a likely exec. The inherited registration is kept.
            // ptr::copy also retains C assignment semantics if ever aliased.
            core::ptr::copy(
                rust_fork_header_task_rseq(current_task()),
                rust_fork_header_task_rseq(task),
                1,
            );
        }
    }
}

#[cfg_attr(not(CONFIG_POSIX_TIMERS), allow(unused_variables))]
unsafe fn fork_posix_cputimers_init(pct: *mut posix_cputimers) {
    #[cfg(CONFIG_POSIX_TIMERS)]
    {
        core::ptr::write_bytes(pct, 0, 1);
        let bases = rust_fork_header_timer_bases(pct);
        let no_event = rust_fork_header_u64_max();
        // The source explicitly initializes these three CPU-clock deadlines;
        // all remaining fields retain the zero initialization above.
        (*bases.add(0)).nextevt = no_event;
        (*bases.add(1)).nextevt = no_event;
        (*bases.add(2)).nextevt = no_event;
    }
}
