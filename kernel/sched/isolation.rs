// SPDX-License-Identifier: GPL-2.0-only
/*
 * Housekeeping management. Manage the targets for routine code that can run on
 * any CPU: unbound workqueues, timers, kthreads and any offloadable work.
 *
 * Continuation of the existing Rust at native baseline
 * 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. Native headers own every layout
 * and enum value; native leaves retain static-key, RCU and boot storage.
 */
#![no_std]

compile_error!("SOURCE ONLY HOLD: scheduler isolation is not admitted");

use kernel::bindings::sched_isolation_native::*;
use kernel::ffi::{
    c_char,
    c_int,
    c_uint,
    c_ulong, //
};

const HK_FLAG_DOMAIN_BOOT: c_ulong = LUPOS_ISOLATION_HK_FLAG_DOMAIN_BOOT as c_ulong;
const HK_FLAG_DOMAIN: c_ulong = LUPOS_ISOLATION_HK_FLAG_DOMAIN as c_ulong;
const HK_FLAG_MANAGED_IRQ: c_ulong = LUPOS_ISOLATION_HK_FLAG_MANAGED_IRQ as c_ulong;
const HK_FLAG_KERNEL_NOISE: c_ulong = LUPOS_ISOLATION_HK_FLAG_KERNEL_NOISE as c_ulong;

fn housekeeping_bit(ty: hk_type) -> c_ulong {
    (1 as c_ulong) << ty.0
}

/// Check the original non-RCU protections for the mutable domain mask.
///
/// # Safety
/// Called by the native RCU dereference macro with a valid housekeeping type.
#[export_name = "lupos_isolation_dereference_check"]
pub unsafe extern "C" fn housekeeping_dereference_check(ty: hk_type) -> bool {
    // SAFETY: Native leaves inspect boot state and lockdep in the caller's
    // context. The RCU macro retains the predicate's conditional evaluation.
    unsafe {
        if cfg!(CONFIG_LOCKDEP) && ty.0 == HK_TYPE_DOMAIN.0 {
            if lupos_isolation_at_most_scheduling() {
                return true;
            }
            #[cfg(all(CONFIG_LOCKDEP, CONFIG_HOTPLUG_CPU))]
            if lupos_isolation_cpus_write_held() != 0 {
                return true;
            }
            #[cfg(all(CONFIG_LOCKDEP, CONFIG_CPUSETS))]
            if lupos_isolation_cpuset_held() {
                return true;
            }
            return false;
        }
        true
    }
}

unsafe fn housekeeping_cpumask_dereference(ty: hk_type) -> *mut cpumask {
    // SAFETY: Caller supplies a native type and the original RCU/lock/boot
    // protection. No Rust reference is formed to the shared mask or slot.
    unsafe { lupos_isolation_mask_dereference(ty) }
}

/// Return whether the requested housekeeping type has a configured mask.
///
/// # Safety
/// `ty` is a native housekeeping type below HK_TYPE_MAX.
#[no_mangle]
pub unsafe extern "C" fn housekeeping_enabled(ty: hk_type) -> bool {
    // SAFETY: The native READ_ONCE leaf owns the shared flags load.
    unsafe { (lupos_isolation_flags_read_once() & housekeeping_bit(ty)) != 0 }
}

/// Return the selected housekeeping mask, or the possible-CPU mask.
///
/// # Safety
/// `ty` is valid. The caller retains the native RCU, boot, hotplug or cpuset
/// protection for every use of the returned mask, as required for that type.
#[no_mangle]
pub unsafe extern "C" fn housekeeping_cpumask(ty: hk_type) -> *const cpumask {
    // SAFETY: The static key and RCU pointer retain their native representations.
    unsafe {
        if lupos_isolation_overridden()
            && (lupos_isolation_flags_read_once() & housekeeping_bit(ty)) != 0
        {
            let mask = housekeeping_cpumask_dereference(ty);
            if !mask.is_null() {
                return mask;
            }
        }
        lupos_isolation_possible_mask()
    }
}

/// Select an online housekeeping CPU, falling back to the current boot CPU.
///
/// # Safety
/// `ty` is valid and the caller retains the native CPU and mask protections.
#[no_mangle]
pub unsafe extern "C" fn housekeeping_any_cpu(ty: hk_type) -> c_int {
    // SAFETY: Native mask operations consume only protected opaque pointers.
    unsafe {
        if lupos_isolation_overridden()
            && (lupos_isolation_flags() & housekeeping_bit(ty)) != 0
        {
            let mut cpu = lupos_isolation_numa_find_closest(
                housekeeping_cpumask(ty),
                lupos_isolation_current_cpu(),
            );
            // Preserve C's int-to-unsigned conversion against nr_cpu_ids.
            if (cpu as c_uint) < lupos_isolation_nr_cpu_ids() {
                return cpu;
            }
            cpu = lupos_isolation_any_and_distribute(
                housekeeping_cpumask(ty),
                lupos_isolation_online_mask(),
            ) as c_int;
            if lupos_isolation_likely_cpu_valid(
                (cpu as c_uint) < lupos_isolation_nr_cpu_ids(),
            ) {
                return cpu;
            }
            lupos_isolation_warn_any_cpu(
                lupos_isolation_running() || ty.0 != HK_TYPE_TIMER.0,
            );
        }
        lupos_isolation_current_cpu()
    }
}

/// Apply the selected housekeeping affinity to a live task.
///
/// # Safety
/// Caller pins `t`, supplies a valid type, and meets native affinity and mask
/// lifetime/locking requirements, including a context that may sleep.
#[no_mangle]
pub unsafe extern "C" fn housekeeping_affine(t: *mut task_struct, ty: hk_type) {
    // SAFETY: Task and mask remain live through the native affinity update.
    unsafe {
        if lupos_isolation_overridden()
            && (lupos_isolation_flags() & housekeeping_bit(ty)) != 0
        {
            let _ = set_cpus_allowed_ptr(t, housekeeping_cpumask(ty));
        }
    }
}

/// Test CPU membership in a selected housekeeping mask.
///
/// # Safety
/// `cpu` and `ty` are valid and the caller retains native mask protection.
#[no_mangle]
pub unsafe extern "C" fn housekeeping_test_cpu(cpu: c_int, ty: hk_type) -> bool {
    // SAFETY: READ_ONCE and checked RCU dereference stay native.
    unsafe {
        if lupos_isolation_overridden()
            && (lupos_isolation_flags_read_once() & housekeeping_bit(ty)) != 0
        {
            return lupos_isolation_test_cpu(cpu, housekeeping_cpumask(ty));
        }
        true
    }
}

/// Publish the domain housekeeping mask and update its dependent subsystems.
///
/// # Safety
/// Caller holds native `cpuset_top_mutex`, keeps `isol_mask` live and stable,
/// and permits sleeping allocation, RCU and workqueue updates. Follow the
/// native caller's lock order: release `cpuset_mutex` and `cpus_read_lock`
/// before entering. CPU hotplug may proceed concurrently.
#[no_mangle]
pub unsafe extern "C" fn housekeeping_update(isol_mask: *mut cpumask) -> c_int {
    // SAFETY: The new allocation is private until RCU publication; the old mask
    // is retained through the grace period and dependent subsystem updates.
    unsafe {
        let trial = lupos_isolation_update_alloc();
        if trial.is_null() {
            return -(LUPOS_ISOLATION_ENOMEM as c_int);
        }
        lupos_isolation_andnot(trial, housekeeping_cpumask(HK_TYPE_DOMAIN_BOOT), isol_mask);
        if !lupos_isolation_intersects(trial, lupos_isolation_online_mask()) {
            lupos_isolation_mask_free(trial);
            return -(LUPOS_ISOLATION_EINVAL as c_int);
        }
        if lupos_isolation_flags() == 0 {
            lupos_isolation_enable();
        }
        let old = if lupos_isolation_flags() & HK_FLAG_DOMAIN != 0 {
            housekeeping_cpumask_dereference(HK_TYPE_DOMAIN)
        } else {
            lupos_isolation_flags_write_once(lupos_isolation_flags() | HK_FLAG_DOMAIN);
            core::ptr::null_mut()
        };
        lupos_isolation_mask_assign(HK_TYPE_DOMAIN, trial);
        synchronize_rcu();
        lupos_isolation_pci_flush();
        lupos_isolation_memcg_flush();
        lupos_isolation_vmstat_flush();
        let err = lupos_isolation_workqueue_update(housekeeping_cpumask(HK_TYPE_DOMAIN));
        lupos_isolation_warn_workqueue(err < 0);
        let err = lupos_isolation_timer_update(isol_mask);
        lupos_isolation_warn_timer(err < 0);
        let err = lupos_isolation_kthreads_update();
        lupos_isolation_warn_kthreads(err < 0);
        lupos_isolation_mask_free(old);
        0
    }
}

/// Replace boot masks with allocations that later updates can release.
///
/// # Safety
/// Called once in the native scheduler initialization order, after early setup
/// and before concurrent domain updates; init text and boot masks remain live.
#[export_name = "lupos_isolation_init"]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn housekeeping_init() {
    // SAFETY: Boot ordering owns all replacement slots and the memblock queue.
    unsafe {
        if lupos_isolation_flags() == 0 {
            return;
        }
        lupos_isolation_enable();
        if lupos_isolation_flags() & HK_FLAG_KERNEL_NOISE != 0 {
            lupos_isolation_tick_offload_init();
        }
        for index in 0..HK_TYPE_MAX.0 {
            let ty = hk_type(index);
            if lupos_isolation_flags() & housekeeping_bit(ty) == 0 {
                continue;
            }
            let nmask = lupos_isolation_init_alloc();
            if lupos_isolation_warn_alloc(nmask.is_null()) {
                return;
            }
            let omask = lupos_isolation_mask_init_dereference(ty);
            lupos_isolation_warn_empty(lupos_isolation_empty(omask));
            lupos_isolation_copy(nmask, omask);
            lupos_isolation_mask_init(ty, nmask);
            lupos_isolation_memblock_queue(omask);
        }
    }
}

/// Release boot memblock masks after their scheduler-init replacements.
///
/// # Safety
/// Called once by the native pure initcall while init storage remains live.
#[export_name = "lupos_isolation_late_init"]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn housekeeping_late_init() -> c_int {
    // SAFETY: Boot serialization owns the detached list. Read next before
    // freeing the current allocation, matching llist_for_each_safe.
    unsafe {
        let mut pos = lupos_isolation_memblock_take_all();
        while !pos.is_null() {
            let next = lupos_isolation_memblock_next(pos);
            lupos_isolation_memblock_free(pos);
            pos = next;
        }
        0
    }
}

#[cold]
#[link_section = ".init.text"]
unsafe fn housekeeping_setup_type(ty: hk_type, staging: *mut cpumask) {
    // SAFETY: Early boot owns the slot; memblock allocation is panic-on-failure
    // with native cpumask size and cache alignment, then initialized before use.
    unsafe {
        let mask = lupos_isolation_memblock_alloc();
        lupos_isolation_copy(mask, staging);
        lupos_isolation_mask_init(ty, mask);
    }
}

#[cold]
#[link_section = ".init.text"]
unsafe fn housekeeping_setup(
    str_: *mut c_char,
    flags: c_ulong,
    masks: *mut lupos_isolation_boot_masks,
) -> c_int {
    // SAFETY: Native boot adapters keep both cpumask_var_t slots on their stack.
    // Allocation/free/decay remain native; Rust does not dereference that
    // configured representation or retain either temporary past the callback.
    unsafe {
        if flags & HK_FLAG_KERNEL_NOISE != 0
            && lupos_isolation_flags() & HK_FLAG_KERNEL_NOISE == 0
            && !cfg!(CONFIG_NO_HZ_FULL)
        {
            lupos_isolation_warn_nohz_unsupported();
            return 0;
        }
        let non = lupos_isolation_boot_non_alloc(masks);
        if lupos_isolation_cpulist_parse(str_, non) < 0 {
            lupos_isolation_warn_range();
            lupos_isolation_boot_non_free(masks);
            return 0;
        }
        let staging = lupos_isolation_boot_staging_alloc(masks);
        lupos_isolation_andnot(staging, lupos_isolation_possible_mask(), non);
        let first = lupos_isolation_first_and(lupos_isolation_present_mask(), staging);
        if first >= lupos_isolation_nr_cpu_ids() || first >= lupos_isolation_setup_max_cpus() {
            lupos_isolation_set_cpu(lupos_isolation_current_cpu(), staging);
            lupos_isolation_clear_cpu(lupos_isolation_current_cpu(), non);
            if lupos_isolation_flags() == 0 {
                lupos_isolation_warn_present(lupos_isolation_current_cpu());
            }
        }
        if !lupos_isolation_empty(non) {
            if lupos_isolation_flags() == 0 {
                for index in 0..HK_TYPE_MAX.0 {
                    let ty = hk_type(index);
                    if flags & housekeeping_bit(ty) != 0 {
                        housekeeping_setup_type(ty, staging);
                    }
                }
            } else {
                let iter_flags = flags & lupos_isolation_flags();
                for index in 0..HK_TYPE_MAX.0 {
                    let ty = hk_type(index);
                    if iter_flags & housekeeping_bit(ty) != 0
                        && !lupos_isolation_equal(staging, housekeeping_cpumask(ty))
                    {
                        lupos_isolation_warn_mismatch();
                        lupos_isolation_boot_staging_free(masks);
                        lupos_isolation_boot_non_free(masks);
                        return 0;
                    }
                }

                // Keep a present CPU outside both nohz_full and isolcpus=domain
                // for timer migration. managed_irq does not participate.
                let previous = lupos_isolation_flags() & (HK_FLAG_KERNEL_NOISE | HK_FLAG_DOMAIN);
                let mut ty = HK_TYPE_MAX;
                for index in 0..HK_TYPE_MAX.0 {
                    let candidate = hk_type(index);
                    if previous & housekeeping_bit(candidate) != 0 {
                        ty = candidate;
                        break;
                    }
                }
                let iter_flags = flags & (HK_FLAG_KERNEL_NOISE | HK_FLAG_DOMAIN);
                let first = if ty.0 == HK_TYPE_MAX.0 || iter_flags == 0 {
                    0
                } else {
                    lupos_isolation_first_and_and(
                        lupos_isolation_present_mask(),
                        staging,
                        housekeeping_cpumask(ty),
                    )
                };
                if first >= core::cmp::min(
                    lupos_isolation_nr_cpu_ids(),
                    lupos_isolation_setup_max_cpus(),
                ) {
                    lupos_isolation_warn_joint_present(str_);
                    lupos_isolation_boot_staging_free(masks);
                    lupos_isolation_boot_non_free(masks);
                    return 0;
                }

                let iter_flags = flags & !lupos_isolation_flags();
                for index in 0..HK_TYPE_MAX.0 {
                    let ty = hk_type(index);
                    if iter_flags & housekeeping_bit(ty) != 0 {
                        housekeeping_setup_type(ty, staging);
                    }
                }
            }
            if flags & HK_FLAG_KERNEL_NOISE != 0
                && lupos_isolation_flags() & HK_FLAG_KERNEL_NOISE == 0
            {
                lupos_isolation_nohz_setup(non);
            }
            lupos_isolation_flags_or(flags);
            lupos_isolation_boot_staging_free(masks);
            lupos_isolation_boot_non_free(masks);
            return 1;
        }
        lupos_isolation_boot_staging_free(masks);
        lupos_isolation_boot_non_free(masks);
        0
    }
}

/// Apply the native nohz_full boot argument.
///
/// # Safety
/// Called only by the registered native boot adapter with its live masks
/// context and native command-line backing storage, before init text is freed.
#[export_name = "lupos_isolation_nohz_full_setup"]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn housekeeping_nohz_full_setup(
    str_: *mut c_char,
    masks: *mut lupos_isolation_boot_masks,
) -> c_int {
    // SAFETY: The adapter owns both temporaries throughout this setup call.
    unsafe { housekeeping_setup(str_, HK_FLAG_KERNEL_NOISE, masks) }
}

/// Parse isolcpus flag prefixes and apply its native CPU-range argument.
///
/// # Safety
/// Called only by the native boot adapter with its live masks context and
/// command-line backing storage. Every load after the preserved unknown-flag
/// advancement past NUL requires readable storage in the same live allocation.
/// The native caller does not establish this for a final unquoted argument;
/// that unresolved oracle edge blocks admission.
#[export_name = "lupos_isolation_isolcpus_setup"]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn housekeeping_isolcpus_setup(
    mut str_: *mut c_char,
    masks: *mut lupos_isolation_boot_masks,
) -> c_int {
    // SAFETY: Native ctype/strncmp behavior is retained, but the post-NUL
    // buffer-domain obligation above remains unresolved. Only native leaves
    // access the configured mask representation.
    unsafe {
        let mut flags: c_ulong = 0;
        let mut illegal = false;
        while lupos_isolation_isalpha(*str_) {
            if strncmp(str_, b"nohz,\0".as_ptr().cast(), 5) == 0 {
                str_ = str_.add(5);
                flags |= HK_FLAG_KERNEL_NOISE;
                continue;
            }
            if strncmp(str_, b"domain,\0".as_ptr().cast(), 7) == 0 {
                str_ = str_.add(7);
                flags |= HK_FLAG_DOMAIN | HK_FLAG_DOMAIN_BOOT;
                continue;
            }
            if strncmp(str_, b"managed_irq,\0".as_ptr().cast(), 12) == 0 {
                str_ = str_.add(12);
                flags |= HK_FLAG_MANAGED_IRQ;
                continue;
            }
            let par = str_;
            let mut len: c_int = 0;
            while *str_ != 0 && *str_ != b',' as c_char {
                if !lupos_isolation_isalpha(*str_) && *str_ != b'_' as c_char {
                    illegal = true;
                }
                str_ = str_.add(1);
                len += 1;
            }
            if illegal {
                lupos_isolation_warn_illegal(len, par);
                return 0;
            }
            lupos_isolation_info_unknown(len, par);
            str_ = str_.add(1);
        }
        if flags == 0 {
            flags |= HK_FLAG_DOMAIN | HK_FLAG_DOMAIN_BOOT;
        }
        housekeeping_setup(str_, flags, masks)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
