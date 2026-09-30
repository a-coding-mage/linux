// SPDX-License-Identifier: GPL-2.0-only
//! Original early kernel entry and ordered boot subsystem initialization.

use super::{
    bindings, main_bootconfig, main_bootoptions, main_command_line, main_early,
    main_globals, main_initcall_context as context, main_initcall_trace,
    main_print, main_printk::main_printk, main_rest, main_start_arch,
    main_unknown, main_warn::main_warn,
};
use core::ptr;
use kernel::ffi::c_char;

/// Start the original kernel boot sequence and hand the boot task to idle.
///
/// # Safety
/// The architecture's kernel entry has established the original boot CPU,
/// stack, initial task and early address-space prerequisites. This routine must
/// be compiled without stack protection and address sanitizer instrumentation,
/// as it initializes both facilities before the final nonreturning handoff.
#[no_mangle]
#[no_sanitize(address)]
#[link_section = ".init.text"]
pub unsafe extern "C" fn start_kernel() {
    // SAFETY: the architecture entry establishes the original start_kernel
    // contract; every subsystem call retains the ordering of the C owner.
    unsafe {
        bindings::set_task_stack_end_magic(ptr::addr_of_mut!(bindings::init_task));
        bindings::smp_setup_processor_id();
        #[cfg(CONFIG_DEBUG_OBJECTS)]
        bindings::debug_objects_early_init();
        #[cfg(any(CONFIG_STACKTRACE_BUILD_ID, CONFIG_VMCORE_INFO))]
        bindings::init_vmlinux_build_id();
        #[cfg(CONFIG_CGROUPS)]
        bindings::cgroup_init_early();

        context::local_irq_disable();
        main_globals::early_boot_irqs_disabled = true;
        bindings::boot_cpu_init();
        bindings::rust_helper_page_address_init();
        main_printk!("start_kernel", b"\x015%s\0", ptr::addr_of!(bindings::linux_banner).cast::<c_char>());
        let mut command_line = core::mem::MaybeUninit::<*mut c_char>::uninit();
        bindings::setup_arch(command_line.as_mut_ptr());
        let command_line = command_line.assume_init();
        bindings::mm_core_init_early();
        #[cfg(CONFIG_JUMP_LABEL)]
        bindings::jump_label_init();
        #[cfg(not(CONFIG_JUMP_LABEL))]
        { main_globals::static_key_initialized = true; }
        #[cfg(CONFIG_HAVE_STATIC_CALL_INLINE)]
        bindings::static_call_init();
        #[cfg(CONFIG_SECURITY)]
        bindings::early_security_init();
        main_bootconfig::setup_boot_config();
        main_command_line::setup_command_line(command_line);
        #[cfg(CONFIG_SMP)]
        bindings::setup_nr_cpu_ids();
        bindings::setup_per_cpu_areas();
        bindings::smp_prepare_boot_cpu();
        main_start_arch::early_numa_node_init();
        bindings::boot_cpu_hotplug_init();

        main_print::print_kernel_cmdline(main_globals::saved_command_line);
        main_early::parse_early_param();
        let parameters = ptr::addr_of!(bindings::__start___param).cast::<bindings::kernel_param>();
        let end = ptr::addr_of!(bindings::__stop___param).cast::<bindings::kernel_param>();
        let count = (end as usize).wrapping_sub(parameters as usize)
            / core::mem::size_of::<bindings::kernel_param>();
        let after_dashes = bindings::parse_args(
            c"Booting kernel".as_ptr().cast(), main_globals::static_command_line,
            parameters, count as _, -1, -1, ptr::null_mut(),
            Some(main_bootoptions::unknown_bootoption),
        );
        main_unknown::print_unknown_bootoptions();
        if !after_dashes.is_null() && !kernel::bindings::IS_ERR(after_dashes.cast()) {
            bindings::parse_args(c"Setting init args".as_ptr().cast(), after_dashes,
                ptr::null(), 0, -1, -1, ptr::null_mut(), Some(main_bootoptions::set_init_arg));
        }
        if !main_globals::extra_init_args.is_null() {
            bindings::parse_args(c"Setting extra init args".as_ptr().cast(), main_globals::extra_init_args,
                ptr::null(), 0, -1, -1, ptr::null_mut(), Some(main_bootoptions::set_init_arg));
        }
        bindings::random_init_early(command_line);
        #[cfg(CONFIG_PRINTK)]
        bindings::setup_log_buf(0);
        bindings::vfs_caches_init_early();
        bindings::sort_main_extable();
        bindings::trap_init();
        bindings::mm_core_init();
        bindings::maple_tree_init();
        bindings::poking_init();
        #[cfg(CONFIG_DYNAMIC_FTRACE)]
        bindings::ftrace_init();
        #[cfg(CONFIG_TRACING)]
        bindings::early_trace_init();
        bindings::sched_init();
        if main_warn!(!context::irqs_disabled(),
            b"Interrupts were enabled *very* early, fixing it\n\0") {
            context::local_irq_disable();
        }
        bindings::radix_tree_init();
        #[cfg(CONFIG_CPU_ISOLATION)]
        bindings::housekeeping_init();
        bindings::workqueue_init_early();
        bindings::rcu_init();
        bindings::kvfree_rcu_init();
        #[cfg(CONFIG_TRACING)]
        bindings::trace_init();
        if main_globals::initcall_debug {
            main_initcall_trace::initcall_debug_enable();
        }
        #[cfg(CONFIG_CONTEXT_TRACKING_USER_FORCE)]
        bindings::context_tracking_init();
        bindings::early_irq_init();
        bindings::init_IRQ();
        #[cfg(CONFIG_GENERIC_CLOCKEVENTS)]
        bindings::tick_init();
        #[cfg(CONFIG_RCU_NOCB_CPU)]
        bindings::rcu_init_nohz();
        bindings::timers_init();
        bindings::srcu_init();
        bindings::hrtimers_init();
        bindings::softirq_init();
        #[cfg(CONFIG_VDSO_DATASTORE)]
        bindings::vdso_setup_data_pages();
        bindings::timekeeping_init();
        bindings::time_init();
        bindings::random_init();
        #[cfg(CONFIG_KFENCE)]
        bindings::kfence_init();
        main_start_arch::boot_init_stack_canary();
        #[cfg(CONFIG_PERF_EVENTS)]
        bindings::perf_event_init();
        #[cfg(CONFIG_PROFILING)]
        bindings::profile_init();
        #[cfg(CONFIG_SMP)]
        bindings::call_function_init();
        main_warn!(!context::irqs_disabled(), b"Interrupts were enabled early\n\0");

        main_globals::early_boot_irqs_disabled = false;
        context::local_irq_enable();
        bindings::kmem_cache_init_late();
        bindings::console_init();
        if !main_globals::panic_later.is_null() {
            bindings::panic(c"Too many boot %s vars at `%s'".as_ptr().cast(),
                main_globals::panic_later, main_globals::panic_param);
        }
        #[cfg(CONFIG_LOCKDEP)]
        bindings::lockdep_init();
        #[cfg(CONFIG_DEBUG_LOCKING_API_SELFTESTS)]
        bindings::locking_selftest();
        #[cfg(CONFIG_BLK_DEV_INITRD)]
        if bindings::initrd_start != 0 && bindings::initrd_below_start_ok == 0 {
            let pfn = bindings::rust_helper_page_to_pfn(
                bindings::rust_helper_virt_to_page(bindings::initrd_start as *const _));
            if pfn < bindings::min_low_pfn {
                // C evaluates the conversion again for this diagnostic.
                main_printk!("start_kernel",
                    b"\x012initrd overwritten (0x%08lx < 0x%08lx) - disabling it.\n\0",
                    bindings::rust_helper_page_to_pfn(bindings::rust_helper_virt_to_page(
                        bindings::initrd_start as *const _)), bindings::min_low_pfn);
                bindings::initrd_start = 0;
            }
        }
        bindings::setup_per_cpu_pageset();
        #[cfg(CONFIG_NUMA)]
        bindings::numa_policy_init();
        #[cfg(CONFIG_ACPI)]
        bindings::acpi_early_init();
        if let Some(late_time_init) = main_globals::late_time_init {
            late_time_init();
        }
        bindings::sched_clock_init();
        bindings::calibrate_delay();
        #[cfg(CONFIG_ARCH_HAS_CPU_FINALIZE_INIT)]
        bindings::arch_cpu_finalize_init();
        bindings::pid_idr_init();
        #[cfg(CONFIG_MMU)]
        bindings::anon_vma_init();
        bindings::thread_stack_cache_init();
        bindings::cred_init();
        bindings::fork_init();
        bindings::proc_caches_init();
        #[cfg(CONFIG_UTS_NS)]
        bindings::uts_ns_init();
        #[cfg(CONFIG_TIME_NS)]
        bindings::time_ns_init();
        #[cfg(CONFIG_KEYS)]
        bindings::key_init();
        #[cfg(CONFIG_SECURITY)]
        bindings::security_init();
        #[cfg(CONFIG_KGDB)]
        bindings::dbg_late_init();
        #[cfg(CONFIG_NET)]
        bindings::net_ns_init();
        bindings::vfs_caches_init();
        bindings::pagecache_init();
        bindings::signals_init();
        bindings::seq_file_init();
        #[cfg(CONFIG_PROC_FS)]
        bindings::proc_root_init();
        bindings::nsfs_init();
        bindings::pidfs_init();
        #[cfg(CONFIG_CPUSETS)]
        bindings::cpuset_init();
        #[cfg(CONFIG_MEMCG)]
        bindings::mem_cgroup_init();
        #[cfg(CONFIG_CGROUPS)]
        bindings::cgroup_init();
        #[cfg(CONFIG_TASKSTATS)]
        bindings::taskstats_init_early();
        #[cfg(CONFIG_TASK_DELAY_ACCT)]
        bindings::delayacct_init();
        #[cfg(CONFIG_ACPI)]
        bindings::acpi_subsystem_init();
        bindings::arch_post_acpi_subsys_init();
        #[cfg(CONFIG_KCSAN)]
        bindings::kcsan_init();
        main_rest::rest_init()
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
