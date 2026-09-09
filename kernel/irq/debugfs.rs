// SPDX-License-Identifier: GPL-2.0
// Copyright 2017 Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>

// C dependencies supplied by the surrounding kernel translation unit.

static mut irq_dir: *mut dentry = core::ptr::null_mut();

unsafe fn irq_debug_show_bits(
    m: *mut seq_file,
    ind: i32,
    state: u32,
    mut sd: *const irq_bit_descr,
    size: i32,
) {
    let mut i = 0;
    while i < size {
        if state & (*sd).mask != 0 {
            seq_printf(m, "%*s%s\n", ind + 12, "", (*sd).name);
        }
        i += 1;
        sd = sd.add(1);
    }
}

#[cfg(CONFIG_SMP)]
unsafe fn irq_debug_show_masks(m: *mut seq_file, desc: *mut irq_desc) {
    let data = irq_desc_get_irq_data(desc);
    let mut msk: *const cpumask;

    msk = irq_data_get_affinity_mask(data);
    seq_printf(m, "affinity: %*pbl\n", cpumask_pr_args(msk));
    #[cfg(CONFIG_GENERIC_IRQ_EFFECTIVE_AFF_MASK)]
    {
        msk = irq_data_get_effective_affinity_mask(data);
        seq_printf(m, "effectiv: %*pbl\n", cpumask_pr_args(msk));
    }
    #[cfg(CONFIG_GENERIC_PENDING_IRQ)]
    {
        msk = (*desc).pending_mask;
        seq_printf(m, "pending:  %*pbl\n", cpumask_pr_args(msk));
    }
}

#[cfg(not(CONFIG_SMP))]
unsafe fn irq_debug_show_masks(_m: *mut seq_file, _desc: *mut irq_desc) {}

static irqchip_flags: [irq_bit_descr; 12] = [
    BIT_MASK_DESCR!(IRQCHIP_SET_TYPE_MASKED),
    BIT_MASK_DESCR!(IRQCHIP_EOI_IF_HANDLED),
    BIT_MASK_DESCR!(IRQCHIP_MASK_ON_SUSPEND),
    BIT_MASK_DESCR!(IRQCHIP_ONOFFLINE_ENABLED),
    BIT_MASK_DESCR!(IRQCHIP_SKIP_SET_WAKE),
    BIT_MASK_DESCR!(IRQCHIP_ONESHOT_SAFE),
    BIT_MASK_DESCR!(IRQCHIP_EOI_THREADED),
    BIT_MASK_DESCR!(IRQCHIP_SUPPORTS_LEVEL_MSI),
    BIT_MASK_DESCR!(IRQCHIP_SUPPORTS_NMI),
    BIT_MASK_DESCR!(IRQCHIP_ENABLE_WAKEUP_ON_SUSPEND),
    BIT_MASK_DESCR!(IRQCHIP_IMMUTABLE),
    BIT_MASK_DESCR!(IRQCHIP_MOVE_DEFERRED),
];

unsafe fn irq_debug_show_chip(m: *mut seq_file, data: *mut irq_data, ind: i32) {
    let chip = (*data).chip;

    if chip.is_null() {
        seq_printf(m, "chip: None\n");
        return;
    }
    seq_printf(m, "%*schip:    ", ind, "");
    if let Some(irq_print_chip) = (*chip).irq_print_chip {
        irq_print_chip(data, m);
    } else {
        seq_printf(m, "%s", (*chip).name);
    }
    seq_printf(m, "\n%*sflags:   0x%lx\n", ind + 1, "", (*chip).flags);
    irq_debug_show_bits(m, ind, (*chip).flags, irqchip_flags.as_ptr(), irqchip_flags.len() as i32);
}

unsafe fn irq_debug_show_data(m: *mut seq_file, data: *mut irq_data, ind: i32) {
    seq_printf(m, "%*sdomain:  %s\n", ind, "", if !(*data).domain.is_null() { (*(*data).domain).name } else { "" });
    seq_printf(m, "%*shwirq:   0x%lx\n", ind + 1, "", (*data).hwirq);
    irq_debug_show_chip(m, data, ind + 1);
    if !(*data).domain.is_null()
        && !(*(*data).domain).ops.is_null()
        && (*(*data).domain).ops.unwrap().debug_show.is_some()
    {
        ((*(*data).domain).ops.unwrap().debug_show.unwrap())(m, core::ptr::null_mut(), data, ind + 1);
    }
    #[cfg(CONFIG_IRQ_DOMAIN_HIERARCHY)]
    {
        if (*data).parent_data.is_null() { return; }
        seq_printf(m, "%*sparent:\n", ind + 1, "");
        irq_debug_show_data(m, (*data).parent_data, ind + 4);
    }
}

static irqdata_states: [irq_bit_descr; 26] = [
    BIT_MASK_DESCR!(IRQ_TYPE_EDGE_RISING), BIT_MASK_DESCR!(IRQ_TYPE_EDGE_FALLING),
    BIT_MASK_DESCR!(IRQ_TYPE_LEVEL_HIGH), BIT_MASK_DESCR!(IRQ_TYPE_LEVEL_LOW),
    BIT_MASK_DESCR!(IRQD_LEVEL), BIT_MASK_DESCR!(IRQD_ACTIVATED),
    BIT_MASK_DESCR!(IRQD_IRQ_STARTED), BIT_MASK_DESCR!(IRQD_IRQ_DISABLED),
    BIT_MASK_DESCR!(IRQD_IRQ_MASKED), BIT_MASK_DESCR!(IRQD_IRQ_INPROGRESS),
    BIT_MASK_DESCR!(IRQD_PER_CPU), BIT_MASK_DESCR!(IRQD_NO_BALANCING),
    BIT_MASK_DESCR!(IRQD_SINGLE_TARGET), BIT_MASK_DESCR!(IRQD_AFFINITY_SET),
    BIT_MASK_DESCR!(IRQD_SETAFFINITY_PENDING), BIT_MASK_DESCR!(IRQD_AFFINITY_MANAGED),
    BIT_MASK_DESCR!(IRQD_AFFINITY_ON_ACTIVATE), BIT_MASK_DESCR!(IRQD_MANAGED_SHUTDOWN),
    BIT_MASK_DESCR!(IRQD_CAN_RESERVE), BIT_MASK_DESCR!(IRQD_FORWARDED_TO_VCPU),
    BIT_MASK_DESCR!(IRQD_WAKEUP_STATE), BIT_MASK_DESCR!(IRQD_WAKEUP_ARMED),
    BIT_MASK_DESCR!(IRQD_DEFAULT_TRIGGER_SET), BIT_MASK_DESCR!(IRQD_HANDLE_ENFORCE_IRQCTX),
    BIT_MASK_DESCR!(IRQD_IRQ_ENABLED_ON_SUSPEND), BIT_MASK_DESCR!(IRQD_RESEND_WHEN_IN_PROGRESS),
];

static irqdesc_states: [irq_bit_descr; 9] = [
    BIT_MASK_DESCR!(_IRQ_NOPROBE), BIT_MASK_DESCR!(_IRQ_NOREQUEST), BIT_MASK_DESCR!(_IRQ_NOTHREAD),
    BIT_MASK_DESCR!(_IRQ_NOAUTOEN), BIT_MASK_DESCR!(_IRQ_NESTED_THREAD), BIT_MASK_DESCR!(_IRQ_PER_CPU_DEVID),
    BIT_MASK_DESCR!(_IRQ_IS_POLLED), BIT_MASK_DESCR!(_IRQ_DISABLE_UNLAZY), BIT_MASK_DESCR!(_IRQ_HIDDEN),
];

static irqdesc_istates: [irq_bit_descr; 9] = [
    BIT_MASK_DESCR!(IRQS_AUTODETECT), BIT_MASK_DESCR!(IRQS_SPURIOUS_DISABLED), BIT_MASK_DESCR!(IRQS_POLL_INPROGRESS),
    BIT_MASK_DESCR!(IRQS_ONESHOT), BIT_MASK_DESCR!(IRQS_REPLAY), BIT_MASK_DESCR!(IRQS_WAITING),
    BIT_MASK_DESCR!(IRQS_PENDING), BIT_MASK_DESCR!(IRQS_SUSPENDED), BIT_MASK_DESCR!(IRQS_NMI),
];

unsafe fn irq_debug_show(m: *mut seq_file, _p: *mut core::ffi::c_void) -> i32 {
    let desc = (*m).private as *mut irq_desc;
    let _guard = guard_raw_spinlock_irq(&mut (*desc).lock);
    let data = irq_desc_get_irq_data(desc);
    seq_printf(m, "handler:  %ps\n", (*desc).handle_irq);
    seq_printf(m, "device:   %s\n", (*desc).dev_name);
    seq_printf(m, "status:   0x%08x\n", (*desc).status_use_accessors);
    irq_debug_show_bits(m, 0, (*desc).status_use_accessors, irqdesc_states.as_ptr(), irqdesc_states.len() as i32);
    seq_printf(m, "istate:   0x%08x\n", (*desc).istate);
    irq_debug_show_bits(m, 0, (*desc).istate, irqdesc_istates.as_ptr(), irqdesc_istates.len() as i32);
    seq_printf(m, "ddepth:   %u\n", (*desc).depth);
    seq_printf(m, "wdepth:   %u\n", (*desc).wake_depth);
    seq_printf(m, "dstate:   0x%08x\n", irqd_get(data));
    irq_debug_show_bits(m, 0, irqd_get(data), irqdata_states.as_ptr(), irqdata_states.len() as i32);
    seq_printf(m, "node:     %d\n", irq_data_get_node(data));
    irq_debug_show_masks(m, desc);
    irq_debug_show_data(m, data, 0);
    0
}

unsafe fn irq_debug_open(inode: *mut inode, file: *mut file) -> i32 {
    single_open(file, irq_debug_show, (*inode).i_private)
}

unsafe fn irq_debug_write(file: *mut file, user_buf: *const u8, count: usize, _ppos: *mut loff_t) -> isize {
    let desc = (*file_inode(file)).i_private as *mut irq_desc;
    let mut buf = [0u8; 8];
    let size = core::cmp::min(buf.len() - 1, count);
    if copy_from_user(buf.as_mut_ptr(), user_buf, size) != 0 { return -EFAULT as isize; }
    if !strncmp(buf.as_ptr(), b"trigger\0".as_ptr(), size) {
        let err = irq_inject_interrupt(irq_desc_get_irq(desc));
        return if err != 0 { err as isize } else { count as isize };
    }
    count as isize
}

static dfs_irq_ops: file_operations = file_operations {
    open: Some(irq_debug_open), write: Some(irq_debug_write), read: Some(seq_read),
    llseek: Some(seq_lseek), release: Some(single_release),
};

unsafe fn irq_debugfs_copy_devname(irq: i32, dev: *mut device) {
    let desc = irq_to_desc(irq);
    let name = dev_name(dev);
    if !name.is_null() { (*desc).dev_name = kstrdup(name, GFP_KERNEL); }
}

unsafe fn irq_add_debugfs_entry(irq: u32, desc: *mut irq_desc) {
    let mut name = [0u8; 12];
    if irq_dir.is_null() || desc.is_null() || !(*desc).debugfs_file.is_null() { return; }
    sprintf(name.as_mut_ptr(), b"%u\0".as_ptr(), irq);
    (*desc).debugfs_file = debugfs_create_file(name.as_ptr(), 0o644, irq_dir, desc, &dfs_irq_ops);
}

unsafe fn irq_debugfs_init() -> i32 {
    let root_dir = debugfs_create_dir(b"irq\0".as_ptr(), core::ptr::null_mut());
    irq_domain_debugfs_init(root_dir);
    irq_dir = debugfs_create_dir(b"irqs\0".as_ptr(), root_dir);
    irq_lock_sparse();
    for_each_active_irq(|irq| irq_add_debugfs_entry(irq, irq_to_desc(irq)));
    irq_unlock_sparse();
    0
}

__initcall!(irq_debugfs_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
