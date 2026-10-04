// SPDX-License-Identifier: GPL-2.0
// Descriptor policy, lifetime and entry paths from irqdesc.c at be59db382996.
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

// include/linux/rcuref.h: keep the release atomic and every preemption/lockdep
// operation native. rcuref_put_slowpath remains an explicit subsystem provider.
#[inline(always)]
unsafe fn __lupos_irq_rcuref_put(reference: *mut rcuref_t) -> bool {
    lupos_irq_rcuref_put_assert();
    let count = lupos_irq_atomic_sub_return_release(1, addr_of_mut!((*reference).refcnt));
    if count >= 0 {
        return false;
    }
    rcuref_put_slowpath(reference, count as c_uint)
}

#[no_mangle]
pub unsafe extern "C" fn lupos_irq_rcuref_put(reference: *mut rcuref_t) -> bool {
    lupos_irq_preempt_disable();
    let released = __lupos_irq_rcuref_put(reference);
    lupos_irq_preempt_enable();
    released
}

include!("irq_core_storage.rs");

static mut irq_desc_lock_class: lock_class_key = unsafe { zeroed() };
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut total_nr_irqs: c_uint = LUPOS_IRQ_NR_IRQS;
#[cfg(all(CONFIG_SPARSE_IRQ, CONFIG_SYSFS))]
static mut irq_kobj_base: *mut kobject = null_mut();

#[cfg(CONFIG_SMP)]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn lupos_irq_affinity_setup(arg: *mut c_char) -> c_int {
    lupos_irq_default_boot_alloc();
    lupos_irq_mask_parse(arg, lupos_irq_default_affinity());
    lupos_irq_mask_set_cpu(lupos_irq_cpu(), lupos_irq_default_affinity());
    1
}
#[link_section = ".init.text"]
unsafe fn init_irq_default_affinity() {
    #[cfg(CONFIG_SMP)]
    {
        if !lupos_irq_default_available() {
            lupos_irq_default_zalloc();
        }
        if lupos_irq_mask_empty(lupos_irq_default_affinity()) {
            lupos_irq_mask_setall(lupos_irq_default_affinity());
        }
    }
}
unsafe fn alloc_masks(desc: *mut irq_desc, node: c_int) -> c_int {
    #[cfg(CONFIG_SMP)]
    {
        if !lupos_irq_mask_alloc(addr_of_mut!((*desc).irq_common_data.affinity).cast(), node) {
            return -(ENOMEM as c_int);
        }
        #[cfg(CONFIG_GENERIC_IRQ_EFFECTIVE_AFF_MASK)]
        if !lupos_irq_mask_alloc(
            addr_of_mut!((*desc).irq_common_data.effective_affinity).cast(),
            node,
        ) {
            lupos_irq_mask_free(lupos_irq_desc_affinity(desc));
            return -(ENOMEM as c_int);
        }
        #[cfg(CONFIG_GENERIC_PENDING_IRQ)]
        if !lupos_irq_mask_alloc(addr_of_mut!((*desc).pending_mask).cast(), node) {
            #[cfg(CONFIG_GENERIC_IRQ_EFFECTIVE_AFF_MASK)]
            lupos_irq_mask_free(lupos_irq_desc_effective(desc));
            lupos_irq_mask_free(lupos_irq_desc_affinity(desc));
            return -(ENOMEM as c_int);
        }
    }
    0
}
#[cfg(CONFIG_SMP)]
unsafe extern "C" fn irq_redirect_work(work: *mut irq_work) {
    let desc = work
        .cast::<u8>()
        .sub(offset_of!(irq_desc, redirect) + offset_of!(irq_redirect, work))
        .cast();
    handle_irq_desc(desc);
}
unsafe fn desc_smp_init(desc: *mut irq_desc, node: c_int, affinity: *const cpumask) {
    #[cfg(CONFIG_SMP)]
    {
        let mask = if affinity.is_null() {
            lupos_irq_default_affinity()
        } else {
            affinity
        };
        lupos_irq_mask_copy(lupos_irq_desc_affinity(desc), mask);
        #[cfg(CONFIG_GENERIC_PENDING_IRQ)]
        lupos_irq_mask_clear(lupos_irq_pending_mask(desc));
        #[cfg(CONFIG_NUMA)]
        {
            (*desc).irq_common_data.node = node as _;
        }
        lupos_irq_redirect_init(addr_of_mut!((*desc).redirect.work), Some(irq_redirect_work));
    }
}
unsafe fn free_masks(desc: *mut irq_desc) {
    #[cfg(CONFIG_SMP)]
    {
        #[cfg(CONFIG_GENERIC_PENDING_IRQ)]
        lupos_irq_mask_free(lupos_irq_pending_mask(desc));
        lupos_irq_mask_free(lupos_irq_desc_affinity(desc));
        #[cfg(CONFIG_GENERIC_IRQ_EFFECTIVE_AFF_MASK)]
        lupos_irq_mask_free(lupos_irq_desc_effective(desc));
    }
}
unsafe fn desc_set_defaults(
    irq: c_uint,
    desc: *mut irq_desc,
    node: c_int,
    affinity: *const cpumask,
    owner: *mut module,
) {
    (*desc).irq_common_data.handler_data = null_mut();
    (*desc).irq_common_data.msi_desc = null_mut();
    (*desc).irq_data.common = addr_of_mut!((*desc).irq_common_data);
    (*desc).irq_data.irq = irq;
    (*desc).irq_data.chip = addr_of_mut!(no_irq_chip);
    (*desc).irq_data.chip_data = null_mut();
    settings(desc, !0, _IRQ_DEFAULT_INIT_FLAGS);
    lupos_irq_data_set(data(desc), IRQD_IRQ_DISABLED);
    lupos_irq_data_set(data(desc), IRQD_IRQ_MASKED);
    (*desc).handle_irq = Some(handle_bad_irq);
    (*desc).depth = 1;
    (*desc).irq_count = 0;
    (*desc).irqs_unhandled = 0;
    (*desc).tot_count = 0;
    (*desc).name = null();
    (*desc).owner = owner;
    lupos_irq_rcuref_init(addr_of_mut!((*desc).refcnt), 1);
    desc_smp_init(desc, node, affinity);
}
#[no_mangle]
pub unsafe extern "C" fn irq_get_nr_irqs() -> c_uint {
    total_nr_irqs
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn irq_set_nr_irqs(nr: c_uint) -> c_uint {
    total_nr_irqs = nr;
    lupos_irq_proc_prec();
    nr
}
unsafe fn irq_find_free_area(from: c_uint, cnt: c_uint) -> c_int {
    let mut mas: ma_state = zeroed();
    lupos_irq_ma_init(&mut mas, 0, 0);
    if mas_empty_area(
        &mut mas,
        from as c_ulong,
        LUPOS_IRQ_MAX_SPARSE_IRQS as c_ulong,
        cnt as c_ulong,
    ) != 0
    {
        return -(ENOSPC as c_int);
    }
    mas.index as c_int
}
#[no_mangle]
pub unsafe extern "C" fn irq_find_desc_at_or_after(offset: c_uint) -> *mut irq_desc {
    let mut index = offset as c_ulong;
    lupos_irq_rcu_assert();
    mt_find(
        addr_of_mut!(lupos_irq_sparse_irqs),
        &mut index,
        total_nr_irqs as c_ulong,
    )
    .cast()
}
unsafe fn irq_insert_desc(irq: c_uint, desc: *mut irq_desc) {
    let mut mas: ma_state = zeroed();
    lupos_irq_ma_init(&mut mas, irq as c_ulong, irq as c_ulong);
    lupos_irq_warn_insert(mas_store_gfp(&mut mas, desc.cast(), LUPOS_IRQ_GFP_KERNEL) != 0);
}
unsafe fn delete_irq_desc(irq: c_uint) {
    let mut mas: ma_state = zeroed();
    lupos_irq_ma_init(&mut mas, irq as c_ulong, irq as c_ulong);
    mas_erase(&mut mas);
}
unsafe fn init_desc(
    desc: *mut irq_desc,
    irq: c_int,
    node: c_int,
    flags: c_uint,
    affinity: *const cpumask,
    owner: *mut module,
) -> c_int {
    (*desc).kstat_irqs = lupos_irq_alloc_stats();
    if (*desc).kstat_irqs.is_null() {
        return -(ENOMEM as c_int);
    }
    if alloc_masks(desc, node) != 0 {
        lupos_irq_free_stats((*desc).kstat_irqs);
        return -(ENOMEM as c_int);
    }
    lupos_irq_raw_init(addr_of_mut!((*desc).lock));
    lupos_irq_raw_class(
        addr_of_mut!((*desc).lock),
        addr_of_mut!(irq_desc_lock_class),
    );
    lupos_irq_mutex_init(addr_of_mut!((*desc).request_mutex));
    lupos_irq_wait_init(addr_of_mut!((*desc).wait_for_threads));
    desc_set_defaults(irq as c_uint, desc, node, affinity, owner);
    lupos_irq_data_set(data(desc), flags);
    irq_resend_init(desc);
    #[cfg(CONFIG_SPARSE_IRQ)]
    {
        kobject_init(addr_of_mut!((*desc).kobj), addr_of!(lupos_irq_kobj_type));
        lupos_irq_rcu_init(addr_of_mut!((*desc).rcu));
    }
    0
}

#[cfg(all(CONFIG_SPARSE_IRQ, CONFIG_SYSFS))]
unsafe fn from_kobj(kobj: *mut kobject) -> *mut irq_desc {
    kobj.cast::<u8>().sub(offset_of!(irq_desc, kobj)).cast()
}
#[cfg(all(CONFIG_SPARSE_IRQ, CONFIG_SYSFS))]
#[no_mangle]
pub unsafe extern "C" fn lupos_irq_per_cpu_count_show(
    kobj: *mut kobject,
    _: *mut kobj_attribute,
    buf: *mut c_char,
) -> isize {
    let desc = from_kobj(kobj);
    let mut ret: isize = 0;
    let mut prefix = c"".as_ptr().cast::<c_char>();
    let mut cpu = lupos_irq_next_cpu(-1, lupos_irq_possible_mask());
    while cpu < lupos_irq_nr_cpu_ids() {
        ret += sysfs_emit_at(
            buf,
            ret as c_int,
            c"%s%u".as_ptr().cast::<c_char>(),
            prefix,
            lupos_irq_stat_cpu(desc, cpu),
        ) as isize;
        prefix = c",".as_ptr().cast::<c_char>();
        cpu = lupos_irq_next_cpu(cpu as c_int, lupos_irq_possible_mask());
    }
    ret + sysfs_emit_at(buf, ret as c_int, c"\n".as_ptr().cast::<c_char>()) as isize
}
#[cfg(all(CONFIG_SPARSE_IRQ, CONFIG_SYSFS))]
#[no_mangle]
pub unsafe extern "C" fn lupos_irq_chip_name_show(
    kobj: *mut kobject,
    _: *mut kobj_attribute,
    buf: *mut c_char,
) -> isize {
    let desc = from_kobj(kobj);
    let _lock = DescLock::irq(desc);
    let chip = (*desc).irq_data.chip;
    if !chip.is_null() && !(*chip).name.is_null() {
        sysfs_emit(buf, c"%s\n".as_ptr().cast::<c_char>(), (*chip).name) as isize
    } else {
        0
    }
}
#[cfg(all(CONFIG_SPARSE_IRQ, CONFIG_SYSFS))]
#[no_mangle]
pub unsafe extern "C" fn lupos_irq_hwirq_show(
    kobj: *mut kobject,
    _: *mut kobj_attribute,
    buf: *mut c_char,
) -> isize {
    let desc = from_kobj(kobj);
    let _lock = DescLock::irq(desc);
    if !(*desc).irq_data.domain.is_null() {
        sysfs_emit(
            buf,
            c"%lu\n".as_ptr().cast::<c_char>(),
            (*desc).irq_data.hwirq,
        ) as isize
    } else {
        0
    }
}
#[cfg(all(CONFIG_SPARSE_IRQ, CONFIG_SYSFS))]
#[no_mangle]
pub unsafe extern "C" fn lupos_irq_type_show(
    kobj: *mut kobject,
    _: *mut kobj_attribute,
    buf: *mut c_char,
) -> isize {
    let desc = from_kobj(kobj);
    let _lock = DescLock::irq(desc);
    let value = if lupos_irq_data_has(data(desc), IRQD_LEVEL) {
        c"level"
    } else {
        c"edge"
    };
    sysfs_emit(
        buf,
        c"%s\n".as_ptr().cast::<c_char>(),
        value.as_ptr().cast::<c_char>(),
    ) as isize
}
#[cfg(all(CONFIG_SPARSE_IRQ, CONFIG_SYSFS))]
#[no_mangle]
pub unsafe extern "C" fn lupos_irq_wakeup_show(
    kobj: *mut kobject,
    _: *mut kobj_attribute,
    buf: *mut c_char,
) -> isize {
    let desc = from_kobj(kobj);
    let _lock = DescLock::irq(desc);
    let value = if lupos_irq_data_has(data(desc), IRQD_WAKEUP_STATE) {
        c"enabled"
    } else {
        c"disabled"
    };
    sysfs_emit(
        buf,
        c"%s\n".as_ptr().cast::<c_char>(),
        value.as_ptr().cast::<c_char>(),
    ) as isize
}
#[cfg(all(CONFIG_SPARSE_IRQ, CONFIG_SYSFS))]
#[no_mangle]
pub unsafe extern "C" fn lupos_irq_name_show(
    kobj: *mut kobject,
    _: *mut kobj_attribute,
    buf: *mut c_char,
) -> isize {
    let desc = from_kobj(kobj);
    let _lock = DescLock::irq(desc);
    if !(*desc).name.is_null() {
        sysfs_emit(buf, c"%s\n".as_ptr().cast::<c_char>(), (*desc).name) as isize
    } else {
        0
    }
}
#[cfg(all(CONFIG_SPARSE_IRQ, CONFIG_SYSFS))]
#[no_mangle]
pub unsafe extern "C" fn lupos_irq_actions_show(
    kobj: *mut kobject,
    _: *mut kobj_attribute,
    buf: *mut c_char,
) -> isize {
    let desc = from_kobj(kobj);
    let mut ret: isize = 0;
    let mut prefix = c"".as_ptr().cast::<c_char>();
    {
        let _lock = DescLock::irq(desc);
        let mut action = (*desc).action;
        while !action.is_null() {
            ret += sysfs_emit_at(
                buf,
                ret as c_int,
                c"%s%s".as_ptr().cast::<c_char>(),
                prefix,
                (*action).name,
            ) as isize;
            prefix = c",".as_ptr().cast::<c_char>();
            action = (*action).next;
        }
    }
    if ret != 0 {
        ret += sysfs_emit_at(buf, ret as c_int, c"\n".as_ptr().cast::<c_char>()) as isize;
    }
    ret
}
#[cfg(CONFIG_SPARSE_IRQ)]
unsafe fn irq_sysfs_add(irq: c_int, desc: *mut irq_desc) {
    #[cfg(CONFIG_SYSFS)]
    if !irq_kobj_base.is_null() {
        if kobject_add(
            addr_of_mut!((*desc).kobj),
            irq_kobj_base,
            c"%d".as_ptr().cast::<c_char>(),
            irq,
        ) != 0
        {
            lupos_irq_warn_sysfs(irq);
        } else {
            *state(desc) |= IRQS_SYSFS;
        }
    }
}
#[cfg(CONFIG_SPARSE_IRQ)]
unsafe fn irq_sysfs_del(desc: *mut irq_desc) {
    #[cfg(CONFIG_SYSFS)]
    if *state(desc) & IRQS_SYSFS != 0 {
        kobject_del(addr_of_mut!((*desc).kobj));
    }
}
#[cfg(all(CONFIG_SPARSE_IRQ, CONFIG_SYSFS))]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn lupos_irq_sysfs_init() -> c_int {
    let _lock = SparseLock::new();
    irq_kobj_base = kobject_create_and_add(c"irq".as_ptr().cast::<c_char>(), kernel_kobj);
    if irq_kobj_base.is_null() {
        return -(ENOMEM as c_int);
    }
    let count = total_nr_irqs;
    for irq in 0..count {
        let desc = irq_to_desc(irq);
        if !desc.is_null() {
            irq_sysfs_add(irq as c_int, desc);
        }
    }
    0
}

#[cfg(CONFIG_SPARSE_IRQ)]
#[no_mangle]
pub unsafe extern "C" fn irq_to_desc(irq: c_uint) -> *mut irq_desc {
    lupos_irq_tree_load(irq as c_ulong).cast()
}
#[cfg(CONFIG_SPARSE_IRQ)]
#[no_mangle]
pub unsafe extern "C" fn irq_lock_sparse() {
    lupos_irq_sparse_lock();
}
#[cfg(CONFIG_SPARSE_IRQ)]
#[no_mangle]
pub unsafe extern "C" fn irq_unlock_sparse() {
    lupos_irq_sparse_unlock();
}
#[cfg(CONFIG_SPARSE_IRQ)]
unsafe fn alloc_desc(
    irq: c_int,
    node: c_int,
    flags: c_uint,
    affinity: *const cpumask,
    owner: *mut module,
) -> *mut irq_desc {
    let desc = lupos_irq_zalloc(size_of::<irq_desc>(), node).cast::<irq_desc>();
    if desc.is_null() {
        return null_mut();
    }
    if init_desc(desc, irq, node, flags, affinity, owner) != 0 {
        kfree(desc.cast());
        return null_mut();
    }
    desc
}
#[cfg(CONFIG_SPARSE_IRQ)]
#[no_mangle]
pub unsafe extern "C" fn lupos_irq_kobj_release(kobj: *mut kobject) {
    let desc: *mut irq_desc = kobj.cast::<u8>().sub(offset_of!(irq_desc, kobj)).cast();
    free_masks(desc);
    lupos_irq_free_stats((*desc).kstat_irqs);
    kfree(desc.cast());
}
#[cfg(CONFIG_SPARSE_IRQ)]
unsafe extern "C" fn delayed_free_desc(head: *mut callback_head) {
    let desc: *mut irq_desc = head.cast::<u8>().sub(offset_of!(irq_desc, rcu)).cast();
    kobject_put(addr_of_mut!((*desc).kobj));
}
#[cfg(CONFIG_SPARSE_IRQ)]
#[no_mangle]
pub unsafe extern "C" fn irq_desc_free_rcu(desc: *mut irq_desc) {
    call_rcu(addr_of_mut!((*desc).rcu), Some(delayed_free_desc));
}
#[cfg(CONFIG_SPARSE_IRQ)]
unsafe fn free_desc(irq: c_uint) {
    let desc = irq_to_desc(irq);
    lupos_irq_debug_del(desc);
    lupos_irq_proc_unregister(irq, desc);
    irq_sysfs_del(desc);
    delete_irq_desc(irq);
    if lupos_irq_rcuref_put(addr_of_mut!((*desc).refcnt)) {
        irq_desc_free_rcu(desc);
    }
}
#[cfg(CONFIG_SPARSE_IRQ)]
unsafe fn alloc_descs(
    start: c_uint,
    cnt: c_uint,
    mut node: c_int,
    mut affinity: *const irq_affinity_desc,
    owner: *mut module,
) -> c_int {
    if !affinity.is_null() {
        for i in 0..cnt {
            if lupos_irq_mask_empty(addr_of!((*affinity.add(i as usize)).mask)) {
                return -(EINVAL as c_int);
            }
        }
    }
    for i in 0..cnt {
        let mut mask = null();
        let mut flags = 0;
        if !affinity.is_null() {
            if lupos_irq_affinity_managed(affinity) {
                flags = IRQD_AFFINITY_MANAGED | IRQD_MANAGED_SHUTDOWN;
            }
            flags |= IRQD_AFFINITY_SET;
            mask = addr_of!((*affinity).mask);
            node = lupos_irq_cpu_node(lupos_irq_mask_first(mask));
            affinity = affinity.add(1);
        }
        let irq = start.wrapping_add(i);
        let desc = alloc_desc(irq as c_int, node, flags, mask, owner);
        if desc.is_null() {
            for allocated in (0..i).rev() {
                free_desc(start.wrapping_add(allocated));
            }
            return -(ENOMEM as c_int);
        }
        irq_insert_desc(irq, desc);
        irq_sysfs_add(irq as c_int, desc);
        lupos_irq_debug_add(irq, desc);
    }
    start as c_int
}
#[cfg(CONFIG_SPARSE_IRQ)]
unsafe fn irq_expand_nr_irqs(nr: c_uint) -> bool {
    if nr > LUPOS_IRQ_MAX_SPARSE_IRQS {
        return false;
    }
    total_nr_irqs = nr;
    lupos_irq_proc_prec();
    true
}
#[cfg(CONFIG_SPARSE_IRQ)]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn early_irq_init() -> c_int {
    let node = lupos_irq_first_node();
    init_irq_default_affinity();
    let mut initcnt = arch_probe_nr_irqs();
    lupos_irq_log_sparse_nr(total_nr_irqs, initcnt);
    if lupos_irq_warn_total(total_nr_irqs > LUPOS_IRQ_MAX_SPARSE_IRQS) {
        total_nr_irqs = LUPOS_IRQ_MAX_SPARSE_IRQS;
    }
    if lupos_irq_warn_initcnt(initcnt > LUPOS_IRQ_MAX_SPARSE_IRQS as c_int) {
        initcnt = LUPOS_IRQ_MAX_SPARSE_IRQS as c_int;
    }
    if initcnt as c_uint > total_nr_irqs {
        total_nr_irqs = initcnt as c_uint;
    }
    for irq in 0..initcnt {
        let desc = alloc_desc(irq, node, 0, null(), null_mut());
        irq_insert_desc(irq as c_uint, desc);
    }
    lupos_irq_proc_prec();
    arch_early_irq_init()
}

#[cfg(not(CONFIG_SPARSE_IRQ))]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn early_irq_init() -> c_int {
    let node = lupos_irq_first_node();
    init_irq_default_affinity();
    lupos_irq_log_nr();
    for irq in 0..LUPOS_IRQ_NR_IRQS {
        let ret = init_desc(irq_to_desc(irq), irq as c_int, node, 0, null(), null_mut());
        if ret != 0 {
            for old in (0..irq).rev() {
                let desc = irq_to_desc(old);
                free_masks(desc);
                lupos_irq_free_stats((*desc).kstat_irqs);
            }
            return ret;
        }
    }
    lupos_irq_proc_prec();
    arch_early_irq_init()
}
#[cfg(not(CONFIG_SPARSE_IRQ))]
#[no_mangle]
pub unsafe extern "C" fn irq_to_desc(irq: c_uint) -> *mut irq_desc {
    if irq < LUPOS_IRQ_NR_IRQS {
        addr_of_mut!(lupos_irq_flat_desc.descs)
            .cast::<irq_desc>()
            .add(irq as usize)
    } else {
        null_mut()
    }
}
#[cfg(not(CONFIG_SPARSE_IRQ))]
unsafe fn free_desc(irq: c_uint) {
    let desc = irq_to_desc(irq);
    {
        let _lock = DescLock::save(desc);
        desc_set_defaults(irq, desc, lupos_irq_node(desc), null(), null_mut());
    }
    let mut cpu = lupos_irq_next_cpu(-1, lupos_irq_possible_mask());
    while cpu < lupos_irq_nr_cpu_ids() {
        lupos_irq_stat_cpu_zero(desc, cpu);
        cpu = lupos_irq_next_cpu(cpu as c_int, lupos_irq_possible_mask());
    }
    delete_irq_desc(irq);
}
#[cfg(not(CONFIG_SPARSE_IRQ))]
unsafe fn alloc_descs(
    start: c_uint,
    cnt: c_uint,
    _: c_int,
    _: *const irq_affinity_desc,
    owner: *mut module,
) -> c_int {
    for i in 0..cnt {
        let irq = start.wrapping_add(i);
        let desc = irq_to_desc(irq);
        (*desc).owner = owner;
        irq_insert_desc(irq, desc);
    }
    start as c_int
}
#[cfg(not(CONFIG_SPARSE_IRQ))]
unsafe fn irq_expand_nr_irqs(_: c_uint) -> bool {
    false
}
#[cfg(not(CONFIG_SPARSE_IRQ))]
#[no_mangle]
pub unsafe extern "C" fn irq_mark_irq(irq: c_uint) {
    let _lock = SparseLock::new();
    irq_insert_desc(irq, irq_to_desc(irq));
}

#[no_mangle]
pub unsafe extern "C" fn handle_irq_desc(desc: *mut irq_desc) -> c_int {
    if desc.is_null() {
        return -(EINVAL as c_int);
    }
    if lupos_irq_warn_context(
        !lupos_irq_in_hardirq() && lupos_irq_data_has(data(desc), IRQD_HANDLE_ENFORCE_IRQCTX),
    ) {
        return -(EPERM as c_int);
    }
    ((*desc).handle_irq.unwrap_unchecked())(desc);
    0
}
#[no_mangle]
pub unsafe extern "C" fn generic_handle_irq(irq: c_uint) -> c_int {
    handle_irq_desc(irq_to_desc(irq))
}
#[no_mangle]
pub unsafe extern "C" fn generic_handle_irq_safe(irq: c_uint) -> c_int {
    let flags = lupos_irq_local_save();
    let ret = handle_irq_desc(irq_to_desc(irq));
    lupos_irq_local_restore(flags);
    ret
}
#[cfg(CONFIG_IRQ_DOMAIN)]
#[no_mangle]
pub unsafe extern "C" fn generic_handle_domain_irq(
    domain: *mut irq_domain,
    hwirq: irq_hw_number_t,
) -> c_int {
    handle_irq_desc(irq_resolve_mapping(domain, hwirq))
}
#[cfg(CONFIG_IRQ_DOMAIN)]
#[no_mangle]
pub unsafe extern "C" fn generic_handle_domain_irq_safe(
    domain: *mut irq_domain,
    hwirq: irq_hw_number_t,
) -> c_int {
    let flags = lupos_irq_local_save();
    let ret = handle_irq_desc(irq_resolve_mapping(domain, hwirq));
    lupos_irq_local_restore(flags);
    ret
}
#[cfg(CONFIG_IRQ_DOMAIN)]
#[no_mangle]
pub unsafe extern "C" fn generic_handle_domain_nmi(
    domain: *mut irq_domain,
    hwirq: irq_hw_number_t,
) -> c_int {
    lupos_irq_warn_nmi(!lupos_irq_in_nmi());
    handle_irq_desc(irq_resolve_mapping(domain, hwirq))
}
#[cfg(all(CONFIG_IRQ_DOMAIN, CONFIG_SMP))]
unsafe fn demux_redirect_remote(desc: *mut irq_desc) -> bool {
    let _lock = DescLock::raw(desc);
    let mask = lupos_irq_effective_affinity(data(desc));
    let target = lupos_irq_target_read(addr_of!((*desc).redirect.target_cpu));
    if let Some(pre_redirect) = (*(*desc).irq_data.chip).irq_pre_redirect {
        pre_redirect(data(desc));
    }
    if lupos_irq_mask_test(lupos_irq_cpu(), mask) {
        return false;
    }
    if !(*desc).action.is_null() {
        irq_work_queue_on(addr_of_mut!((*desc).redirect.work), target as c_int);
    }
    true
}
#[cfg(all(CONFIG_IRQ_DOMAIN, not(CONFIG_SMP)))]
unsafe fn demux_redirect_remote(_: *mut irq_desc) -> bool {
    false
}
#[cfg(CONFIG_IRQ_DOMAIN)]
#[no_mangle]
pub unsafe extern "C" fn generic_handle_demux_domain_irq(
    domain: *mut irq_domain,
    hwirq: irq_hw_number_t,
) -> bool {
    let desc = irq_resolve_mapping(domain, hwirq);
    if desc.is_null() {
        return false;
    }
    if demux_redirect_remote(desc) {
        return true;
    }
    handle_irq_desc(desc) == 0
}
#[no_mangle]
pub unsafe extern "C" fn irq_free_descs(from: c_uint, cnt: c_uint) {
    if from >= total_nr_irqs || from.wrapping_add(cnt) > total_nr_irqs {
        return;
    }
    let _lock = SparseLock::new();
    for i in 0..cnt {
        free_desc(from.wrapping_add(i));
    }
}
#[no_mangle]
#[link_section = ".ref.text"]
pub unsafe extern "C" fn __irq_alloc_descs(
    irq: c_int,
    mut from: c_uint,
    cnt: c_uint,
    node: c_int,
    owner: *mut module,
    affinity: *const irq_affinity_desc,
) -> c_int {
    if cnt == 0 {
        return -(EINVAL as c_int);
    }
    if irq >= 0 {
        if from > irq as c_uint {
            return -(EINVAL as c_int);
        }
        from = irq as c_uint;
    } else {
        from = arch_dynirq_lower_bound(from);
    }
    let _lock = SparseLock::new();
    let start = irq_find_free_area(from, cnt);
    if irq >= 0 && start != irq {
        return -(EEXIST as c_int);
    }
    // Preserve C's unsigned promotion and wrap when adding the unsigned count.
    let end = (start as c_uint).wrapping_add(cnt);
    if end > total_nr_irqs && !irq_expand_nr_irqs(end) {
        return -(ENOMEM as c_int);
    }
    alloc_descs(start as c_uint, cnt, node, affinity, owner)
}
#[no_mangle]
pub unsafe extern "C" fn irq_get_next_irq(offset: c_uint) -> c_uint {
    let _rcu = RcuLock::new();
    let desc = irq_find_desc_at_or_after(offset);
    if desc.is_null() {
        total_nr_irqs
    } else {
        (*desc).irq_data.irq
    }
}
#[no_mangle]
pub unsafe extern "C" fn __irq_get_desc_lock(
    irq: c_uint,
    flags: *mut c_ulong,
    bus: bool,
    check: c_uint,
) -> *mut irq_desc {
    let desc = irq_to_desc(irq);
    if desc.is_null() {
        return null_mut();
    }
    if check & _IRQ_DESC_CHECK != 0 {
        let percpu = setting(desc, _IRQ_PER_CPU_DEVID);
        if check & _IRQ_DESC_PERCPU != 0 && !percpu {
            return null_mut();
        }
        if check & _IRQ_DESC_PERCPU == 0 && percpu {
            return null_mut();
        }
    }
    if bus {
        if let Some(lock) = (*(*desc).irq_data.chip).irq_bus_lock {
            lock(data(desc));
        }
    }
    *flags = lupos_irq_raw_lock_save(addr_of_mut!((*desc).lock));
    desc
}
#[no_mangle]
pub unsafe extern "C" fn __irq_put_desc_unlock(desc: *mut irq_desc, flags: c_ulong, bus: bool) {
    lupos_irq_raw_unlock_restore(addr_of_mut!((*desc).lock), flags);
    if bus {
        if let Some(unlock) = (*(*desc).irq_data.chip).irq_bus_sync_unlock {
            unlock(data(desc));
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn irq_set_percpu_devid(irq: c_uint) -> c_int {
    let desc = irq_to_desc(irq);
    if desc.is_null() || !(*desc).percpu_enabled.is_null() {
        return -(EINVAL as c_int);
    }
    (*desc).percpu_enabled = lupos_irq_alloc_percpu_mask();
    if (*desc).percpu_enabled.is_null() {
        return -(ENOMEM as c_int);
    }
    lupos_irq_set_percpu_flags(irq);
    0
}
#[no_mangle]
pub unsafe extern "C" fn kstat_incr_irq_this_cpu(irq: c_uint) {
    lupos_irq_kstat_incr(irq_to_desc(irq));
}
#[no_mangle]
pub unsafe extern "C" fn kstat_irqs_cpu(irq: c_uint, cpu: c_int) -> c_uint {
    let desc = irq_to_desc(irq);
    if desc.is_null() {
        0
    } else {
        lupos_irq_stat_cpu(desc, cpu as c_uint)
    }
}
unsafe fn kstat_irqs_desc(desc: *mut irq_desc, mask: *const cpumask) -> c_uint {
    if !setting(desc, _IRQ_PER_CPU_DEVID)
        && !setting(desc, _IRQ_PER_CPU)
        && *state(desc) & IRQS_NMI == 0
    {
        return lupos_irq_total_race(desc);
    }
    let mut sum: c_uint = 0;
    let mut cpu = lupos_irq_next_cpu(-1, mask);
    while cpu < lupos_irq_nr_cpu_ids() {
        sum = sum.wrapping_add(lupos_irq_stat_cpu_race(desc, cpu));
        cpu = lupos_irq_next_cpu(cpu as c_int, mask);
    }
    sum
}
unsafe fn kstat_irqs(irq: c_uint) -> c_uint {
    let desc = irq_to_desc(irq);
    if desc.is_null() {
        0
    } else {
        kstat_irqs_desc(desc, lupos_irq_possible_mask())
    }
}
#[cfg(CONFIG_GENERIC_IRQ_STAT_SNAPSHOT)]
#[no_mangle]
pub unsafe extern "C" fn kstat_snapshot_irqs() {
    let count = total_nr_irqs;
    for irq in 0..count {
        let desc = irq_to_desc(irq);
        if !desc.is_null() {
            lupos_irq_stat_ref_write(desc, lupos_irq_stat_cnt(desc));
        }
    }
}
#[cfg(CONFIG_GENERIC_IRQ_STAT_SNAPSHOT)]
#[no_mangle]
pub unsafe extern "C" fn kstat_get_irq_since_snapshot(irq: c_uint) -> c_uint {
    let desc = irq_to_desc(irq);
    if desc.is_null() {
        0
    } else {
        lupos_irq_stat_cnt(desc).wrapping_sub(lupos_irq_stat_ref(desc))
    }
}
#[no_mangle]
pub unsafe extern "C" fn kstat_irqs_usr(irq: c_uint) -> c_uint {
    let _rcu = RcuLock::new();
    kstat_irqs(irq)
}
#[cfg(CONFIG_LOCKDEP)]
#[no_mangle]
pub unsafe extern "C" fn __irq_set_lockdep_class(
    irq: c_uint,
    lock_class: *mut lock_class_key,
    request_class: *mut lock_class_key,
) {
    let desc = irq_to_desc(irq);
    if !desc.is_null() {
        lupos_irq_raw_class(addr_of_mut!((*desc).lock), lock_class);
        lupos_irq_mutex_class(addr_of_mut!((*desc).request_mutex), request_class);
    }
}
