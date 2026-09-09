// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 1992, 1998-2006 Linus Torvalds, Ingo Molnar
 * Copyright (C) 2005-2006, Thomas Gleixner, Russell King
 *
 * This file contains the interrupt descriptor management code. Detailed
 * information is available in Documentation/core-api/genericirq.rst
 */

// Dependencies are supplied by the surrounding kernel translation unit.

static mut IRQ_DESC_LOCK_CLASS: lock_class_key = lock_class_key {};

#[cfg(CONFIG_SMP)]
unsafe extern "C" fn irq_affinity_setup(str_: *mut c_char) -> c_int {
    alloc_bootmem_cpumask_var(&mut irq_default_affinity);
    cpulist_parse(str_, irq_default_affinity);
    cpumask_set_cpu(smp_processor_id(), irq_default_affinity);
    1
}

#[cfg(CONFIG_SMP)]
unsafe fn init_irq_default_affinity() {
    if !cpumask_available(irq_default_affinity) {
        zalloc_cpumask_var(&mut irq_default_affinity, GFP_NOWAIT);
    }
    if cpumask_empty(irq_default_affinity) {
        cpumask_setall(irq_default_affinity);
    }
}

#[cfg(not(CONFIG_SMP))]
unsafe fn init_irq_default_affinity() {}

#[cfg(CONFIG_SMP)]
unsafe fn alloc_masks(desc: *mut irq_desc, node: c_int) -> c_int {
    if !zalloc_cpumask_var_node(&mut (*desc).irq_common_data.affinity, GFP_KERNEL, node) {
        return -ENOMEM;
    }
    #[cfg(CONFIG_GENERIC_IRQ_EFFECTIVE_AFF_MASK)]
    if !zalloc_cpumask_var_node(&mut (*desc).irq_common_data.effective_affinity, GFP_KERNEL, node) {
        free_cpumask_var((*desc).irq_common_data.affinity);
        return -ENOMEM;
    }
    #[cfg(CONFIG_GENERIC_PENDING_IRQ)]
    if !zalloc_cpumask_var_node(&mut (*desc).pending_mask, GFP_KERNEL, node) {
        #[cfg(CONFIG_GENERIC_IRQ_EFFECTIVE_AFF_MASK)]
        free_cpumask_var((*desc).irq_common_data.effective_affinity);
        free_cpumask_var((*desc).irq_common_data.affinity);
        return -ENOMEM;
    }
    0
}

#[cfg(not(CONFIG_SMP))]
unsafe fn alloc_masks(_: *mut irq_desc, _: c_int) -> c_int { 0 }

#[cfg(CONFIG_SMP)]
unsafe fn irq_redirect_work(work: *mut irq_work) {
    handle_irq_desc(container_of!(work, irq_desc, redirect.work));
}

#[cfg(CONFIG_SMP)]
unsafe fn desc_smp_init(desc: *mut irq_desc, node: c_int, mut affinity: *const cpumask) {
    if affinity.is_null() { affinity = irq_default_affinity; }
    cpumask_copy((*desc).irq_common_data.affinity, affinity);
    #[cfg(CONFIG_GENERIC_PENDING_IRQ)]
    cpumask_clear((*desc).pending_mask);
    #[cfg(CONFIG_NUMA)]
    { (*desc).irq_common_data.node = node; }
    (*desc).redirect.work = IRQ_WORK_INIT_HARD!(irq_redirect_work);
}

#[cfg(not(CONFIG_SMP))]
unsafe fn desc_smp_init(_: *mut irq_desc, _: c_int, _: *const cpumask) {}

#[cfg(CONFIG_SMP)]
unsafe fn free_masks(desc: *mut irq_desc) {
    #[cfg(CONFIG_GENERIC_PENDING_IRQ)]
    free_cpumask_var((*desc).pending_mask);
    free_cpumask_var((*desc).irq_common_data.affinity);
    #[cfg(CONFIG_GENERIC_IRQ_EFFECTIVE_AFF_MASK)]
    free_cpumask_var((*desc).irq_common_data.effective_affinity);
}

#[cfg(not(CONFIG_SMP))]
unsafe fn free_masks(_: *mut irq_desc) {}

unsafe fn desc_set_defaults(irq: c_uint, desc: *mut irq_desc, node: c_int,
    affinity: *const cpumask, owner: *mut module) {
    (*desc).irq_common_data.handler_data = core::ptr::null_mut();
    (*desc).irq_common_data.msi_desc = core::ptr::null_mut();
    (*desc).irq_data.common = &mut (*desc).irq_common_data;
    (*desc).irq_data.irq = irq;
    (*desc).irq_data.chip = &mut no_irq_chip;
    (*desc).irq_data.chip_data = core::ptr::null_mut();
    irq_settings_clr_and_set(desc, !0, _IRQ_DEFAULT_INIT_FLAGS);
    irqd_set(&mut (*desc).irq_data, IRQD_IRQ_DISABLED);
    irqd_set(&mut (*desc).irq_data, IRQD_IRQ_MASKED);
    (*desc).handle_irq = Some(handle_bad_irq);
    (*desc).depth = 1;
    (*desc).irq_count = 0;
    (*desc).irqs_unhandled = 0;
    (*desc).tot_count = 0;
    (*desc).name = core::ptr::null();
    (*desc).owner = owner;
    rcuref_init(&mut (*desc).refcnt, 1);
    desc_smp_init(desc, node, affinity);
}

#[no_mangle]
pub static mut total_nr_irqs: c_uint = NR_IRQS;

#[no_mangle]
pub unsafe extern "C" fn irq_get_nr_irqs() -> c_uint { total_nr_irqs }

#[no_mangle]
pub unsafe extern "C" fn irq_set_nr_irqs(nr: c_uint) -> c_uint {
    total_nr_irqs = nr;
    irq_proc_calc_prec();
    nr
}

static mut SPARSE_IRQ_LOCK: mutex = mutex {};
static mut SPARSE_IRQS: maple_tree = maple_tree {};

unsafe fn irq_find_free_area(from: c_uint, cnt: c_uint) -> c_int {
    let mut mas = ma_state::new(&mut SPARSE_IRQS, 0, 0);
    if mas_empty_area(&mut mas, from, MAX_SPARSE_IRQS, cnt) != 0 { return -ENOSPC; }
    mas.index as c_int
}

pub unsafe extern "C" fn irq_find_desc_at_or_after(offset: c_uint) -> *mut irq_desc {
    let mut index = offset as c_ulong;
    lockdep_assert_in_rcu_read_lock();
    mt_find(&mut SPARSE_IRQS, &mut index, total_nr_irqs)
}

unsafe fn irq_insert_desc(irq: c_uint, desc: *mut irq_desc) {
    let mut mas = ma_state::new(&mut SPARSE_IRQS, irq as c_ulong, irq as c_ulong);
    WARN_ON(mas_store_gfp(&mut mas, desc, GFP_KERNEL) != 0);
}
unsafe fn delete_irq_desc(irq: c_uint) {
    let mut mas = ma_state::new(&mut SPARSE_IRQS, irq as c_ulong, irq as c_ulong);
    mas_erase(&mut mas);
}

unsafe fn init_desc(desc: *mut irq_desc, irq: c_int, node: c_int, flags: c_uint,
    affinity: *const cpumask, owner: *mut module) -> c_int {
    (*desc).kstat_irqs = alloc_percpu::<irqstat>();
    if (*desc).kstat_irqs.is_null() { return -ENOMEM; }
    if alloc_masks(desc, node) != 0 { free_percpu((*desc).kstat_irqs); return -ENOMEM; }
    raw_spin_lock_init(&mut (*desc).lock);
    lockdep_set_class(&mut (*desc).lock, &mut IRQ_DESC_LOCK_CLASS);
    mutex_init(&mut (*desc).request_mutex);
    init_waitqueue_head(&mut (*desc).wait_for_threads);
    desc_set_defaults(irq as c_uint, desc, node, affinity, owner);
    irqd_set(&mut (*desc).irq_data, flags);
    irq_resend_init(desc);
    #[cfg(CONFIG_SPARSE_IRQ)]
    { kobject_init(&mut (*desc).kobj, &IRQ_KOBJ_TYPE); init_rcu_head(&mut (*desc).rcu); }
    0
}

#[cfg(CONFIG_SPARSE_IRQ)]
static IRQ_KOBJ_TYPE: kobj_type = kobj_type { release: Some(irq_kobj_release), ..kobj_type::zeroed() };

#[cfg(CONFIG_SPARSE_IRQ)]
unsafe fn alloc_desc(irq: c_int, node: c_int, flags: c_uint, affinity: *const cpumask, owner: *mut module) -> *mut irq_desc {
    let desc = kzalloc_node(core::mem::size_of::<irq_desc>(), GFP_KERNEL, node) as *mut irq_desc;
    if desc.is_null() { return core::ptr::null_mut(); }
    if init_desc(desc, irq, node, flags, affinity, owner) != 0 { kfree(desc as *mut c_void); return core::ptr::null_mut(); }
    desc
}

#[no_mangle]
pub unsafe extern "C" fn irq_to_desc(irq: c_uint) -> *mut irq_desc { mtree_load(&mut SPARSE_IRQS, irq as c_ulong) }

#[no_mangle]
pub unsafe extern "C" fn handle_irq_desc(desc: *mut irq_desc) -> c_int {
    if desc.is_null() { return -EINVAL; }
    let data = irq_desc_get_irq_data(desc);
    if WARN_ON_ONCE(!in_hardirq() && irqd_is_handle_enforce_irqctx(data)) { return -EPERM; }
    generic_handle_irq_desc(desc);
    0
}

#[no_mangle]
pub unsafe extern "C" fn generic_handle_irq(irq: c_uint) -> c_int { handle_irq_desc(irq_to_desc(irq)) }

#[no_mangle]
pub unsafe extern "C" fn generic_handle_irq_safe(irq: c_uint) -> c_int {
    let mut flags = 0 as c_ulong;
    local_irq_save(&mut flags);
    let ret = handle_irq_desc(irq_to_desc(irq));
    local_irq_restore(flags);
    ret
}

#[cfg(CONFIG_IRQ_DOMAIN)]
pub unsafe extern "C" fn generic_handle_domain_irq(domain: *mut irq_domain, hwirq: irq_hw_number_t) -> c_int {
    handle_irq_desc(irq_resolve_mapping(domain, hwirq))
}

#[cfg(CONFIG_IRQ_DOMAIN)]
pub unsafe extern "C" fn generic_handle_domain_irq_safe(domain: *mut irq_domain, hwirq: irq_hw_number_t) -> c_int {
    let mut flags = 0 as c_ulong; local_irq_save(&mut flags);
    let ret = handle_irq_desc(irq_resolve_mapping(domain, hwirq)); local_irq_restore(flags); ret
}

#[cfg(CONFIG_IRQ_DOMAIN)]
pub unsafe extern "C" fn generic_handle_domain_nmi(domain: *mut irq_domain, hwirq: irq_hw_number_t) -> c_int {
    WARN_ON_ONCE(!in_nmi()); handle_irq_desc(irq_resolve_mapping(domain, hwirq))
}

#[no_mangle]
pub unsafe extern "C" fn generic_handle_demux_domain_irq(domain: *mut irq_domain, hwirq: irq_hw_number_t) -> bool {
    let desc = irq_resolve_mapping(domain, hwirq);
    if desc.is_null() { return false; }
    !handle_irq_desc(desc) == true
}

#[no_mangle]
pub unsafe extern "C" fn irq_free_descs(from: c_uint, cnt: c_uint) {
    if from >= total_nr_irqs || from.wrapping_add(cnt) > total_nr_irqs { return; }
    mutex_lock(&mut SPARSE_IRQ_LOCK);
    for i in 0..cnt { free_desc(from + i); }
    mutex_unlock(&mut SPARSE_IRQ_LOCK);
}

#[no_mangle]
pub unsafe extern "C" fn __irq_alloc_descs(irq: c_int, mut from: c_uint, cnt: c_uint, node: c_int,
    owner: *mut module, affinity: *const irq_affinity_desc) -> c_int {
    if cnt == 0 { return -EINVAL; }
    if irq >= 0 { if from > irq as c_uint { return -EINVAL; } from = irq as c_uint; }
    else { from = arch_dynirq_lower_bound(from); }
    mutex_lock(&mut SPARSE_IRQ_LOCK);
    let start = irq_find_free_area(from, cnt);
    if irq >= 0 && start != irq { mutex_unlock(&mut SPARSE_IRQ_LOCK); return -EEXIST; }
    if start as c_uint + cnt > total_nr_irqs && !irq_expand_nr_irqs(start as c_uint + cnt) {
        mutex_unlock(&mut SPARSE_IRQ_LOCK); return -ENOMEM;
    }
    let ret = alloc_descs(start as c_uint, cnt, node, affinity, owner);
    mutex_unlock(&mut SPARSE_IRQ_LOCK); ret
}

#[no_mangle]
pub unsafe extern "C" fn irq_get_next_irq(offset: c_uint) -> c_uint {
    rcu_read_lock(); let desc = irq_find_desc_at_or_after(offset);
    let ret = if !desc.is_null() { irq_desc_get_irq(desc) } else { total_nr_irqs };
    rcu_read_unlock(); ret
}

pub unsafe extern "C" fn __irq_get_desc_lock(irq: c_uint, flags: *mut c_ulong, bus: bool, check: c_uint) -> *mut irq_desc {
    let desc = irq_to_desc(irq); if desc.is_null() { return core::ptr::null_mut(); }
    if check & _IRQ_DESC_CHECK != 0 {
        if check & _IRQ_DESC_PERCPU != 0 && !irq_settings_is_per_cpu_devid(desc) { return core::ptr::null_mut(); }
        if check & _IRQ_DESC_PERCPU == 0 && irq_settings_is_per_cpu_devid(desc) { return core::ptr::null_mut(); }
    }
    if bus { chip_bus_lock(desc); } raw_spin_lock_irqsave(&mut (*desc).lock, flags); desc
}

pub unsafe extern "C" fn __irq_put_desc_unlock(desc: *mut irq_desc, flags: c_ulong, bus: bool) {
    raw_spin_unlock_irqrestore(&mut (*desc).lock, flags); if bus { chip_bus_sync_unlock(desc); }
}

pub unsafe extern "C" fn irq_set_percpu_devid(irq: c_uint) -> c_int {
    let desc = irq_to_desc(irq); if desc.is_null() || !(*desc).percpu_enabled.is_null() { return -EINVAL; }
    (*desc).percpu_enabled = kzalloc_obj::<c_void>();
    if (*desc).percpu_enabled.is_null() { return -ENOMEM; }
    irq_set_percpu_devid_flags(irq); 0
}

pub unsafe extern "C" fn kstat_incr_irq_this_cpu(irq: c_uint) { kstat_incr_irqs_this_cpu(irq_to_desc(irq)); }
pub unsafe extern "C" fn kstat_irqs_cpu(irq: c_uint, cpu: c_int) -> c_uint {
    let desc = irq_to_desc(irq); if desc.is_null() { 0 } else { irq_desc_kstat_cpu(desc, cpu) }
}

pub unsafe extern "C" fn kstat_irqs_usr(irq: c_uint) -> c_uint {
    rcu_read_lock(); let ret = kstat_irqs(irq); rcu_read_unlock(); ret
}

#[cfg(CONFIG_LOCKDEP)]
pub unsafe extern "C" fn __irq_set_lockdep_class(irq: c_uint, lock_class: *mut lock_class_key, request_class: *mut lock_class_key) {
    let desc = irq_to_desc(irq); if !desc.is_null() { lockdep_set_class(&mut (*desc).lock, lock_class); lockdep_set_class(&mut (*desc).request_mutex, request_class); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
