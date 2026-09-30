// SPDX-License-Identifier: GPL-2.0-only
//! Original init task setup before its init-memory lifecycle is released.

use super::{bindings, main_console, main_globals, main_initcall_levels};
use super::main_printk::main_printk;

/// Assign the original init task's nodemask inside its required critical section.
#[cfg(CONFIG_CPUSETS)]
unsafe fn set_mems_allowed(mask: bindings::nodemask_t) {
    // SAFETY: current is the live init task. The original task lock, interrupt
    // state and sequence counter serialize exactly this nodemask assignment.
    unsafe {
        let current = kernel::bindings::get_current().cast::<bindings::task_struct>();
        let lock = core::ptr::addr_of_mut!((*current).alloc_lock);
        let sequence = core::ptr::addr_of_mut!((*current).mems_allowed_seq);
        kernel::bindings::spin_lock(lock.cast());
        let flags = bindings::rust_helper_local_irq_save();
        bindings::rust_helper_write_seqcount_spinlock_begin(sequence);
        core::ptr::addr_of_mut!((*current).mems_allowed).write(mask);
        bindings::rust_helper_write_seqcount_spinlock_end(sequence);
        bindings::rust_helper_local_irq_restore(flags);
        kernel::bindings::spin_unlock(lock.cast());
    }
}

/// Initialize init-task resources, CPUs, initcalls and the initial root namespace.
///
/// # Safety
///
/// The caller is the original init task after scheduler setup and the kthreadd
/// completion. Each subsystem must be at its original boot lifecycle phase.
#[inline(never)]
#[link_section = ".init.text"]
pub(super) unsafe fn kernel_init_freeable() {
    // SAFETY: the caller preserves the original serialized init-task lifecycle.
    unsafe {
        bindings::gfp_allowed_mask = bindings::RUST_INIT_MAIN_GFP_BITS_MASK;
        #[cfg(CONFIG_CPUSETS)]
        set_mems_allowed(bindings::node_states[bindings::node_states_N_MEMORY as usize]);

        // task_pid is the canonical thread_pid field. get_pid accepts null and
        // otherwise uses the existing saturation-aware refcount primitive.
        let current = kernel::bindings::get_current().cast::<bindings::task_struct>();
        let pid = (*current).thread_pid;
        if !pid.is_null() {
            kernel::bindings::refcount_inc(core::ptr::addr_of_mut!((*pid).count).cast());
        }
        bindings::cad_pid = pid;

        #[cfg(all(CONFIG_SMP, CONFIG_X86))]
        {
            // x86's smp_prepare_cpus is an inline dispatch through its actual
            // operations table; C requires this boot callback to be nonnull.
            bindings::smp_ops.smp_prepare_cpus.unwrap_unchecked()(bindings::setup_max_cpus);
        }
        #[cfg(all(CONFIG_SMP, not(CONFIG_X86)))]
        bindings::smp_prepare_cpus(bindings::setup_max_cpus);

        bindings::workqueue_init();
        bindings::init_mm_internals();
        main_initcall_levels::do_pre_smp_initcalls();
        #[cfg(CONFIG_LOCKUP_DETECTOR)]
        bindings::lockup_detector_init();
        #[cfg(CONFIG_SMP)]
        bindings::smp_init();
        #[cfg(all(not(CONFIG_SMP), CONFIG_UP_LATE_INIT))]
        bindings::up_late_init();
        bindings::sched_init_smp();
        bindings::workqueue_init_topology();
        bindings::async_init();
        #[cfg(CONFIG_PADATA)]
        bindings::padata_init();
        bindings::page_alloc_init_late();
        main_initcall_levels::do_basic_setup();

        #[cfg(CONFIG_KUNIT = "y")]
        bindings::kunit_run_all_tests();
        #[cfg(CONFIG_BLK_DEV_INITRD)]
        bindings::wait_for_initramfs();
        main_console::console_on_rootfs();

        let access = bindings::init_eaccess(main_globals::ramdisk_execute_command);
        if access != 0 {
            if main_globals::ramdisk_execute_command_set {
                main_printk!("kernel_init_freeable",
                    b"\x014check access for rdinit=%s failed: %i, ignoring\n\0",
                    main_globals::ramdisk_execute_command, access);
            }
            main_globals::ramdisk_execute_command = core::ptr::null_mut();
            bindings::prepare_namespace();
        }
        #[cfg(CONFIG_INTEGRITY)]
        bindings::integrity_load_keys();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
