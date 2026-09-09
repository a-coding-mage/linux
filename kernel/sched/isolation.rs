// SPDX-License-Identifier: GPL-2.0-only
/*
 * Housekeeping management. Manage the targets for routine code that can run on
 * any CPU: unbound workqueues, timers, kthreads and any offloadable work.
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
struct Housekeeping {
    cpumasks: [*mut cpumask; HK_TYPE_MAX],
    flags: c_ulong,
}

const HK_FLAG_DOMAIN_BOOT: c_ulong = 1 << (HK_TYPE_DOMAIN_BOOT as usize);
const HK_FLAG_DOMAIN: c_ulong = 1 << (HK_TYPE_DOMAIN as usize);
const HK_FLAG_MANAGED_IRQ: c_ulong = 1 << (HK_TYPE_MANAGED_IRQ as usize);
const HK_FLAG_KERNEL_NOISE: c_ulong = 1 << (HK_TYPE_KERNEL_NOISE as usize);

static mut HOUSEKEEPING_OVERRIDDEN: bool = false;
static mut HOUSEKEEPING: Housekeeping = Housekeeping {
    cpumasks: [core::ptr::null_mut(); HK_TYPE_MAX],
    flags: 0,
};
static mut MEMBLOCK_FREELIST: *mut llist_head = core::ptr::null_mut();

unsafe extern "C" {
    type cpumask;
    type task_struct;
    type llist_head;
    type llist_node;
    type c_void;
    type hk_type;
    static cpu_possible_mask: *const cpumask;
    static cpu_online_mask: *const cpumask;
    static cpu_present_mask: *const cpumask;
    static nr_cpu_ids: c_int;
    static setup_max_cpus: c_uint;
    static system_state: c_int;

    fn sched_numa_find_closest(mask: *const cpumask, cpu: c_int) -> c_int;
    fn smp_processor_id() -> c_int;
    fn cpumask_any_and_distribute(mask: *const cpumask, online: *const cpumask) -> c_int;
    fn set_cpus_allowed_ptr(t: *mut task_struct, mask: *const cpumask);
    fn kmalloc(size: usize, flags: c_ulong) -> *mut cpumask;
    fn kfree(ptr: *mut cpumask);
    fn cpumask_size() -> usize;
    fn cpumask_andnot(dst: *mut cpumask, a: *const cpumask, b: *const cpumask);
    fn cpumask_intersects(a: *const cpumask, b: *const cpumask) -> bool;
    fn cpumask_test_cpu(cpu: c_int, mask: *const cpumask) -> bool;
    fn cpumask_copy(dst: *mut cpumask, src: *const cpumask);
    fn cpumask_empty(mask: *const cpumask) -> bool;
    fn cpumask_equal(a: *const cpumask, b: *const cpumask) -> bool;
    fn cpumask_first_and(a: *const cpumask, b: *const cpumask) -> c_uint;
    fn cpumask_first_and_and(a: *const cpumask, b: *const cpumask, c: *const cpumask) -> c_uint;
    fn __cpumask_set_cpu(cpu: c_int, mask: *mut cpumask);
    fn __cpumask_clear_cpu(cpu: c_int, mask: *mut cpumask);
    fn alloc_bootmem_cpumask_var(mask: *mut *mut cpumask);
    fn free_bootmem_cpumask_var(mask: *mut cpumask);
    fn cpulist_parse(s: *const c_char, mask: *mut cpumask) -> c_int;
    fn memblock_alloc_or_panic(size: usize, align: usize) -> *mut cpumask;
    fn memblock_free(ptr: *mut llist_node, size: usize);
    fn pci_probe_flush_workqueue();
    fn mem_cgroup_flush_workqueue();
    fn vmstat_flush_workqueue();
    fn workqueue_unbound_housekeeping_update(mask: *const cpumask) -> c_int;
    fn tmigr_isolated_exclude_cpumask(mask: *const cpumask) -> c_int;
    fn kthreads_update_housekeeping() -> c_int;
    fn sched_tick_offload_init();
    fn tick_nohz_full_setup(mask: *const cpumask);
    fn synchronize_rcu();
}

type c_int = i32;
type c_uint = u32;
type c_ulong = usize;
type c_char = i8;
const HK_TYPE_DOMAIN_BOOT: usize = 0;
const HK_TYPE_DOMAIN: usize = 1;
const HK_TYPE_MANAGED_IRQ: usize = 2;
const HK_TYPE_KERNEL_NOISE: usize = 3;
const HK_TYPE_MAX: usize = 4;

unsafe fn housekeeping_cpumask_dereference(ty: usize) -> *mut cpumask {
    HOUSEKEEPING.cpumasks[ty]
}

#[no_mangle]
pub unsafe extern "C" fn housekeeping_enabled(ty: usize) -> bool {
    (HOUSEKEEPING.flags & (1 << ty)) != 0
}

#[no_mangle]
pub unsafe extern "C" fn housekeeping_cpumask(ty: usize) -> *const cpumask {
    if HOUSEKEEPING_OVERRIDDEN && (HOUSEKEEPING.flags & (1 << ty)) != 0 {
        let mask = housekeeping_cpumask_dereference(ty);
        if !mask.is_null() { return mask; }
    }
    cpu_possible_mask
}

#[no_mangle]
pub unsafe extern "C" fn housekeeping_any_cpu(ty: usize) -> c_int {
    if HOUSEKEEPING_OVERRIDDEN && (HOUSEKEEPING.flags & (1 << ty)) != 0 {
        let mut cpu = sched_numa_find_closest(housekeeping_cpumask(ty), smp_processor_id());
        if cpu < nr_cpu_ids { return cpu; }
        cpu = cpumask_any_and_distribute(housekeeping_cpumask(ty), cpu_online_mask);
        if cpu < nr_cpu_ids { return cpu; }
    }
    smp_processor_id()
}

#[no_mangle]
pub unsafe extern "C" fn housekeeping_affine(t: *mut task_struct, ty: usize) {
    if HOUSEKEEPING_OVERRIDDEN && (HOUSEKEEPING.flags & (1 << ty)) != 0 {
        set_cpus_allowed_ptr(t, housekeeping_cpumask(ty));
    }
}

#[no_mangle]
pub unsafe extern "C" fn housekeeping_test_cpu(cpu: c_int, ty: usize) -> bool {
    if HOUSEKEEPING_OVERRIDDEN && (HOUSEKEEPING.flags & (1 << ty)) != 0 {
        return cpumask_test_cpu(cpu, housekeeping_cpumask(ty));
    }
    true
}

// Remaining setup/update entry points retain the kernel implementation's external orchestration.
pub unsafe fn housekeeping_update(isol_mask: *mut cpumask) -> c_int {
    let trial = kmalloc(cpumask_size(), 0);
    if trial.is_null() { return -12; }
    cpumask_andnot(trial, housekeeping_cpumask(HK_TYPE_DOMAIN_BOOT), isol_mask);
    if !cpumask_intersects(trial, cpu_online_mask) { kfree(trial); return -22; }
    if HOUSEKEEPING.flags == 0 { HOUSEKEEPING_OVERRIDDEN = true; }
    let old = if HOUSEKEEPING.flags & HK_FLAG_DOMAIN != 0 {
        housekeeping_cpumask_dereference(HK_TYPE_DOMAIN)
    } else {
        HOUSEKEEPING.flags |= HK_FLAG_DOMAIN;
        core::ptr::null_mut()
    };
    HOUSEKEEPING.cpumasks[HK_TYPE_DOMAIN] = trial;
    synchronize_rcu();
    pci_probe_flush_workqueue(); mem_cgroup_flush_workqueue(); vmstat_flush_workqueue();
    let _ = workqueue_unbound_housekeeping_update(housekeeping_cpumask(HK_TYPE_DOMAIN));
    let _ = tmigr_isolated_exclude_cpumask(isol_mask);
    let _ = kthreads_update_housekeeping();
    if !old.is_null() { kfree(old); }
    0
}

pub unsafe fn housekeeping_init() {
    if HOUSEKEEPING.flags == 0 { return; }
    HOUSEKEEPING_OVERRIDDEN = true;
    if HOUSEKEEPING.flags & HK_FLAG_KERNEL_NOISE != 0 { sched_tick_offload_init(); }
    for ty in 0..HK_TYPE_MAX {
        if HOUSEKEEPING.flags & (1 << ty) == 0 { continue; }
        let nmask = kmalloc(cpumask_size(), 0);
        if nmask.is_null() { return; }
        let omask = housekeeping_cpumask_dereference(ty);
        cpumask_copy(nmask, omask);
        HOUSEKEEPING.cpumasks[ty] = nmask;
    }
}

unsafe fn housekeeping_setup_type(ty: usize, staging: *mut cpumask) {
    let mask = memblock_alloc_or_panic(cpumask_size(), 64);
    cpumask_copy(mask, staging);
    HOUSEKEEPING.cpumasks[ty] = mask;
}

pub unsafe fn housekeeping_setup(str_: *mut c_char, flags: c_ulong) -> c_int {
    let non = kmalloc(cpumask_size(), 0);
    if non.is_null() && cpulist_parse(str_, non) < 0 { kfree(non); return 0; }
    let staging = kmalloc(cpumask_size(), 0);
    if staging.is_null() { if !non.is_null() { kfree(non); } return 0; }
    cpumask_andnot(staging, cpu_possible_mask, non);
    let first = cpumask_first_and(cpu_present_mask, staging);
    if first >= nr_cpu_ids as u32 || first >= setup_max_cpus {
        __cpumask_set_cpu(smp_processor_id(), staging);
        __cpumask_clear_cpu(smp_processor_id(), non);
    }
    if !cpumask_empty(non) {
        if HOUSEKEEPING.flags == 0 {
            for ty in 0..HK_TYPE_MAX { if flags & (1 << ty) != 0 { housekeeping_setup_type(ty, staging); } }
        } else {
            for ty in 0..HK_TYPE_MAX {
                if flags & HOUSEKEEPING.flags & (1 << ty) != 0 && !cpumask_equal(staging, housekeeping_cpumask(ty)) {
                    kfree(staging); kfree(non); return 0;
                }
            }
            for ty in 0..HK_TYPE_MAX { if flags & !HOUSEKEEPING.flags & (1 << ty) != 0 { housekeeping_setup_type(ty, staging); } }
        }
        if flags & HK_FLAG_KERNEL_NOISE != 0 && HOUSEKEEPING.flags & HK_FLAG_KERNEL_NOISE == 0 { tick_nohz_full_setup(non); }
        HOUSEKEEPING.flags |= flags;
        kfree(staging); kfree(non); return 1;
    }
    kfree(staging); kfree(non); 0
}

pub unsafe fn housekeeping_nohz_full_setup(str_: *mut c_char) -> c_int {
    housekeeping_setup(str_, HK_FLAG_KERNEL_NOISE)
}

pub unsafe fn housekeeping_isolcpus_setup(str_: *mut c_char) -> c_int {
    let mut flags = 0;
    if flags == 0 { flags |= HK_FLAG_DOMAIN | HK_FLAG_DOMAIN_BOOT; }
    housekeeping_setup(str_, flags)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
