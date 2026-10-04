// SPDX-License-Identifier: GPL-2.0-only
// Owned inline algorithms from the original configured Linux headers:
//   uaccess.h:393-415          copy_struct_from_user
//   ptrace.h:201-220           ptrace_init_task
//   rseq.h:132-162             rseq_reset and rseq_fork
//   posix-timers.h:101-107     posix_cputimers_init
//   sched/mm.h:47-56          mmdrop
//   sched/task.h:129-162      put_task_struct
//   asm/mmu_context.h:147-175 init_new_context and destroy_context
//   mm_types.h:1590-1648      CID and scheduler allocation/destruction
//   string.h:113-124          sized_strscpy_pad
// Included in fork.rs with generated b types and the parent's current_task().
// ABI scalar aliases deliberately use kernel::ffi, matching kernel bindings.
//
// The original user-copy translation adds a fifth parameter: destination object
// size known at the call site (size_of::<clone_args>() in copy_clone_args_from_user).
// This preserves __builtin_object_size(dst, 1)'s guard after moving ownership
// across languages. An untyped-pointer C helper cannot recover that bound.
// Selected size/tail/error decisions, tracing policy, rseq inheritance/reset,
// CPU-timer initialization and task/MM policies below are Rust-owned. Native
// leaves and the separately identified external providers remain explicit.
// CONFIG_RSEQ and CONFIG_POSIX_TIMERS disabled bodies match the header no-ops.
// H3/H5 additions are source-reviewed only; configured binding and compiler
// validation are pending. RCU, ASID, scheduler initialization and allocation
// services remain external providers. The original C/H authorities are intact.

unsafe fn fork_mmdrop(mm: *mut mm_struct) {
    // This native atomic retains the full membarrier barrier, including when
    // the decrement does not release the last reference.
    if rust_fork_mm_atomic_dec_and_test(rust_fork_mm_mm_mm_count(mm)) {
        __mmdrop(mm);
    }
}

unsafe fn fork_put_task_struct(task: *mut task_struct) {
    if !rust_fork_mm_refcount_dec_and_test(rust_fork_mm_task_usage(task)) {
        return;
    }
    // The native leaf passes the actual public __put_task_struct_rcu_cb symbol
    // to call_rcu. Its only definition is the existing Rust fork provider.
    // The original always defers, including on non-RT kernels.
    rust_fork_mm_defer_task_release(task);
}

unsafe fn fork_init_new_context(_task: *mut task_struct, mm: *mut mm_struct) -> c_int {
    // Keep one native initializer/key for each original lock site.
    rust_fork_mm_init_context_lock(mm);
    *rust_fork_mm_context_ctx_id(mm) = rust_fork_mm_next_context_id() as _;
    rust_fork_mm_atomic64_set(rust_fork_mm_context_tlb_gen(mm), 0);
    *rust_fork_mm_context_next_trim_cpumask(mm) =
        rust_fork_mm_context_jiffies().wrapping_add(rust_fork_mm_context_hz());

    #[cfg(CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS)]
    if rust_fork_mm_context_ospke_enabled() {
        *rust_fork_mm_context_pkey_allocation_map(mm) = 0x1;
        *rust_fork_mm_context_execute_only_pkey(mm) = -1;
    }

    rust_fork_mm_init_global_asid(mm);
    #[cfg(CONFIG_ADDRESS_MASKING)]
    {
        // The authority assigns -1UL to its u64 field, preserving ulong width.
        *rust_fork_mm_context_untag_mask(mm) = c_ulong::MAX as _;
    }
    #[cfg(CONFIG_MODIFY_LDT_SYSCALL)]
    {
        *rust_fork_mm_context_ldt(mm) = core::ptr::null_mut();
        rust_fork_mm_init_context_ldt_sem(mm);
    }
    0
}

unsafe fn fork_destroy_context(mm: *mut mm_struct) {
    #[cfg(CONFIG_MODIFY_LDT_SYSCALL)]
    rust_fork_mm_destroy_context_ldt(mm);
    rust_fork_mm_free_global_asid(mm);
}

#[cfg_attr(not(CONFIG_SCHED_MM_CID), allow(unused_variables))]
unsafe fn fork_mm_alloc_cid(mm: *mut mm_struct, task: *mut task_struct) -> c_int {
    #[cfg(CONFIG_SCHED_MM_CID)]
    {
        let pcpu = rust_fork_mm_alloc_cid_pcpu();
        // Unlike the sched allocation, the original stores even a null result.
        *rust_fork_mm_cid_pcpu(mm) = pcpu;
        if pcpu.is_null() {
            return -(ENOMEM as c_int);
        }
        rust_fork_mm_init_cid(mm, task);
    }
    0
}

#[cfg_attr(not(CONFIG_SCHED_MM_CID), allow(unused_variables))]
unsafe fn fork_mm_destroy_cid(mm: *mut mm_struct) {
    #[cfg(CONFIG_SCHED_MM_CID)]
    {
        rust_fork_mm_free_cid_pcpu(*rust_fork_mm_cid_pcpu(mm));
        *rust_fork_mm_cid_pcpu(mm) = core::ptr::null_mut();
    }
}

#[cfg_attr(not(CONFIG_SCHED_CACHE), allow(unused_variables))]
unsafe fn fork_mm_alloc_sched(mm: *mut mm_struct) -> c_int {
    #[cfg(CONFIG_SCHED_CACHE)]
    {
        let pcpu = rust_fork_mm_alloc_sched_pcpu();
        if pcpu.is_null() {
            return -(ENOMEM as c_int);
        }
        // The external initializer publishes pcpu_sched with store-release.
        // Do not write that field early, including on the allocation failure.
        rust_fork_mm_init_sched(mm, pcpu);
    }
    0
}

#[cfg_attr(not(CONFIG_SCHED_CACHE), allow(unused_variables))]
unsafe fn fork_mm_destroy_sched(mm: *mut mm_struct) {
    #[cfg(CONFIG_SCHED_CACHE)]
    {
        rust_fork_mm_free_sched_pcpu(*rust_fork_mm_sched_pcpu(mm));
        *rust_fork_mm_sched_pcpu(mm) = core::ptr::null_mut();
    }
}

unsafe fn fork_sized_strscpy_pad(dst: *mut c_char, src: *const c_char, count: usize) -> isize {
    let wrote = rust_fork_sized_strscpy(dst, src, count);
    if wrote >= 0 && (wrote as usize) < count {
        core::ptr::write_bytes(dst.add(wrote as usize + 1), 0, count - wrote as usize - 1);
    }
    wrote
}

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
