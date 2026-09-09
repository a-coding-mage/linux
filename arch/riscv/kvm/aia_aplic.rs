// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2021 Western Digital Corporation or its affiliates.
 * Copyright (C) 2022 Ventana Micro Systems Inc.
 *
 * Authors:
 *	Anup Patel <apatel@ventanamicro.com>
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

#[repr(C)]
struct aplic_irq {
    lock: raw_spinlock_t,
    sourcecfg: u32,
    state: u32,
    target: u32,
}

const APLIC_IRQ_STATE_PENDING: u32 = 1 << 0;
const APLIC_IRQ_STATE_ENABLED: u32 = 1 << 1;
const APLIC_IRQ_STATE_ENPEND: u32 = APLIC_IRQ_STATE_PENDING | APLIC_IRQ_STATE_ENABLED;
const APLIC_IRQ_STATE_INPUT: u32 = 1 << 8;

#[repr(C)]
struct aplic {
    iodev: kvm_io_device,
    domaincfg: u32,
    genmsi: u32,
    nr_irqs: u32,
    nr_words: u32,
    irqs: [aplic_irq; 0],
}

unsafe fn aplic_read_sourcecfg(aplic: *mut aplic, irq: u32) -> u32 {
    if irq == 0 || (*aplic).nr_irqs <= irq { return 0; }
    let irqd = &mut (*aplic).irqs[array_index_nospec(irq, (*aplic).nr_irqs)];
    let mut flags = 0;
    raw_spin_lock_irqsave(&mut irqd.lock, &mut flags);
    let ret = irqd.sourcecfg;
    raw_spin_unlock_irqrestore(&mut irqd.lock, flags);
    ret
}

unsafe fn aplic_write_sourcecfg(aplic: *mut aplic, irq: u32, mut val: u32) {
    if irq == 0 || (*aplic).nr_irqs <= irq { return; }
    let irqd = &mut (*aplic).irqs[array_index_nospec(irq, (*aplic).nr_irqs)];
    if val & APLIC_SOURCECFG_D != 0 { val = 0; } else { val &= APLIC_SOURCECFG_SM_MASK; }
    let mut flags = 0;
    raw_spin_lock_irqsave(&mut irqd.lock, &mut flags);
    irqd.sourcecfg = val;
    raw_spin_unlock_irqrestore(&mut irqd.lock, flags);
}

unsafe fn aplic_read_target(aplic: *mut aplic, irq: u32) -> u32 {
    if irq == 0 || (*aplic).nr_irqs <= irq { return 0; }
    let irqd = &mut (*aplic).irqs[array_index_nospec(irq, (*aplic).nr_irqs)];
    let mut flags = 0;
    raw_spin_lock_irqsave(&mut irqd.lock, &mut flags);
    let ret = irqd.target;
    raw_spin_unlock_irqrestore(&mut irqd.lock, flags);
    ret
}

unsafe fn aplic_write_target(aplic: *mut aplic, irq: u32, mut val: u32) {
    if irq == 0 || (*aplic).nr_irqs <= irq { return; }
    let irqd = &mut (*aplic).irqs[array_index_nospec(irq, (*aplic).nr_irqs)];
    val &= APLIC_TARGET_EIID_MASK |
        (APLIC_TARGET_HART_IDX_MASK << APLIC_TARGET_HART_IDX_SHIFT) |
        (APLIC_TARGET_GUEST_IDX_MASK << APLIC_TARGET_GUEST_IDX_SHIFT);
    let mut flags = 0;
    raw_spin_lock_irqsave(&mut irqd.lock, &mut flags);
    irqd.target = val;
    raw_spin_unlock_irqrestore(&mut irqd.lock, flags);
}

unsafe fn aplic_read_pending(aplic: *mut aplic, irq: u32) -> bool {
    if irq == 0 || (*aplic).nr_irqs <= irq { return false; }
    let irqd = &mut (*aplic).irqs[array_index_nospec(irq, (*aplic).nr_irqs)];
    let mut flags = 0;
    raw_spin_lock_irqsave(&mut irqd.lock, &mut flags);
    let ret = irqd.state & APLIC_IRQ_STATE_PENDING != 0;
    raw_spin_unlock_irqrestore(&mut irqd.lock, flags);
    ret
}

unsafe fn aplic_write_pending(aplic: *mut aplic, irq: u32, pending: bool) {
    if irq == 0 || (*aplic).nr_irqs <= irq { return; }
    let irqd = &mut (*aplic).irqs[array_index_nospec(irq, (*aplic).nr_irqs)];
    let mut flags = 0;
    raw_spin_lock_irqsave(&mut irqd.lock, &mut flags);
    let sm = irqd.sourcecfg & APLIC_SOURCECFG_SM_MASK;
    if sm != APLIC_SOURCECFG_SM_INACTIVE {
        let skip = (sm == APLIC_SOURCECFG_SM_LEVEL_HIGH || sm == APLIC_SOURCECFG_SM_LEVEL_LOW) &&
            (!pending || ((irqd.state & APLIC_IRQ_STATE_INPUT != 0) && sm == APLIC_SOURCECFG_SM_LEVEL_LOW) ||
             (irqd.state & APLIC_IRQ_STATE_INPUT == 0 && sm == APLIC_SOURCECFG_SM_LEVEL_HIGH));
        if !skip {
            if pending { irqd.state |= APLIC_IRQ_STATE_PENDING; }
            else { irqd.state &= !APLIC_IRQ_STATE_PENDING; }
        }
    }
    raw_spin_unlock_irqrestore(&mut irqd.lock, flags);
}

unsafe fn aplic_read_enabled(aplic: *mut aplic, irq: u32) -> bool {
    if irq == 0 || (*aplic).nr_irqs <= irq { return false; }
    let irqd = &mut (*aplic).irqs[array_index_nospec(irq, (*aplic).nr_irqs)];
    let mut flags = 0;
    raw_spin_lock_irqsave(&mut irqd.lock, &mut flags);
    let ret = irqd.state & APLIC_IRQ_STATE_ENABLED != 0;
    raw_spin_unlock_irqrestore(&mut irqd.lock, flags);
    ret
}

unsafe fn aplic_write_enabled(aplic: *mut aplic, irq: u32, enabled: bool) {
    if irq == 0 || (*aplic).nr_irqs <= irq { return; }
    let irqd = &mut (*aplic).irqs[array_index_nospec(irq, (*aplic).nr_irqs)];
    let mut flags = 0;
    raw_spin_lock_irqsave(&mut irqd.lock, &mut flags);
    if enabled { irqd.state |= APLIC_IRQ_STATE_ENABLED; } else { irqd.state &= !APLIC_IRQ_STATE_ENABLED; }
    raw_spin_unlock_irqrestore(&mut irqd.lock, flags);
}

unsafe fn aplic_read_input(aplic: *mut aplic, irq: u32) -> bool {
    if irq == 0 || (*aplic).nr_irqs <= irq { return false; }
    let irqd = &mut (*aplic).irqs[array_index_nospec(irq, (*aplic).nr_irqs)];
    let mut flags = 0;
    raw_spin_lock_irqsave(&mut irqd.lock, &mut flags);
    let sourcecfg = irqd.sourcecfg;
    let ret = if sourcecfg & APLIC_SOURCECFG_D != 0 || sourcecfg & APLIC_SOURCECFG_SM_MASK == APLIC_SOURCECFG_SM_INACTIVE { false } else {
        let raw_input = if irqd.state & APLIC_IRQ_STATE_INPUT != 0 { 1 } else { 0 };
        let inverted = if sourcecfg & APLIC_SOURCECFG_SM_MASK == APLIC_SOURCECFG_SM_LEVEL_LOW || sourcecfg & APLIC_SOURCECFG_SM_MASK == APLIC_SOURCECFG_SM_EDGE_FALL { 1 } else { 0 };
        (raw_input ^ inverted) != 0
    };
    raw_spin_unlock_irqrestore(&mut irqd.lock, flags);
    ret
}

unsafe fn aplic_inject_msi(kvm: *mut kvm, irq: u32, target: u32) {
    let hart_idx = (target >> APLIC_TARGET_HART_IDX_SHIFT) & APLIC_TARGET_HART_IDX_MASK;
    let guest_idx = (target >> APLIC_TARGET_GUEST_IDX_SHIFT) & APLIC_TARGET_GUEST_IDX_MASK;
    let eiid = target & APLIC_TARGET_EIID_MASK;
    kvm_riscv_aia_inject_msi_by_id(kvm, hart_idx, guest_idx, eiid);
}

unsafe fn aplic_update_irq_range(kvm: *mut kvm, first: u32, last: u32) {
    let aplic = (*kvm).arch.aia.aplic_state;
    if (*aplic).domaincfg & APLIC_DOMAINCFG_IE == 0 { return; }
    let mut irq = first;
    while irq <= last {
        if irq != 0 && irq < (*aplic).nr_irqs {
            let irqd = &mut (*aplic).irqs[array_index_nospec(irq, (*aplic).nr_irqs)];
            let mut flags = 0;
            raw_spin_lock_irqsave(&mut irqd.lock, &mut flags);
            let target = irqd.target;
            let inject = (irqd.state & APLIC_IRQ_STATE_ENPEND) == APLIC_IRQ_STATE_ENPEND;
            if inject { irqd.state &= !APLIC_IRQ_STATE_PENDING; }
            raw_spin_unlock_irqrestore(&mut irqd.lock, flags);
            if inject { aplic_inject_msi(kvm, irq, target); }
        }
        irq += 1;
    }
}

pub unsafe fn kvm_riscv_aia_aplic_inject(kvm: *mut kvm, source: u32, level: bool) -> i32 {
    let aplic = (*kvm).arch.aia.aplic_state;
    if aplic.is_null() || source == 0 || (*aplic).nr_irqs <= source { return -ENODEV; }
    let irqd = &mut (*aplic).irqs[array_index_nospec(source, (*aplic).nr_irqs)];
    let ie = (*aplic).domaincfg & APLIC_DOMAINCFG_IE != 0;
    let mut flags = 0;
    raw_spin_lock_irqsave(&mut irqd.lock, &mut flags);
    if irqd.sourcecfg & APLIC_SOURCECFG_D != 0 { raw_spin_unlock_irqrestore(&mut irqd.lock, flags); return 0; }
    match irqd.sourcecfg & APLIC_SOURCECFG_SM_MASK {
        APLIC_SOURCECFG_SM_EDGE_RISE => if level && irqd.state & (APLIC_IRQ_STATE_INPUT | APLIC_IRQ_STATE_PENDING) == 0 { irqd.state |= APLIC_IRQ_STATE_PENDING; },
        APLIC_SOURCECFG_SM_EDGE_FALL => if !level && irqd.state & APLIC_IRQ_STATE_INPUT != 0 && irqd.state & APLIC_IRQ_STATE_PENDING == 0 { irqd.state |= APLIC_IRQ_STATE_PENDING; },
        APLIC_SOURCECFG_SM_LEVEL_HIGH => if level && irqd.state & APLIC_IRQ_STATE_PENDING == 0 { irqd.state |= APLIC_IRQ_STATE_PENDING; },
        APLIC_SOURCECFG_SM_LEVEL_LOW => if !level && irqd.state & APLIC_IRQ_STATE_PENDING == 0 { irqd.state |= APLIC_IRQ_STATE_PENDING; },
        _ => {}
    }
    if level { irqd.state |= APLIC_IRQ_STATE_INPUT; } else { irqd.state &= !APLIC_IRQ_STATE_INPUT; }
    let target = irqd.target;
    let inject = ie && irqd.state & APLIC_IRQ_STATE_ENPEND == APLIC_IRQ_STATE_ENPEND;
    if inject { irqd.state &= !APLIC_IRQ_STATE_PENDING; }
    raw_spin_unlock_irqrestore(&mut irqd.lock, flags);
    if inject { aplic_inject_msi(kvm, source, target); }
    0
}

unsafe fn aplic_read_input_word(aplic: *mut aplic, word: u32) -> u32 { let mut ret=0; for i in 0..32 { if aplic_read_input(aplic, word*32+i) { ret |= 1<<i; } } ret }
unsafe fn aplic_read_pending_word(aplic: *mut aplic, word: u32) -> u32 { let mut ret=0; for i in 0..32 { if aplic_read_pending(aplic, word*32+i) { ret |= 1<<i; } } ret }
unsafe fn aplic_write_pending_word(aplic: *mut aplic, word: u32, val: u32, pending: bool) { for i in 0..32 { if val & (1<<i) != 0 { aplic_write_pending(aplic, word*32+i, pending); } } }
unsafe fn aplic_read_enabled_word(aplic: *mut aplic, word: u32) -> u32 { let mut ret=0; for i in 0..32 { if aplic_read_enabled(aplic, word*32+i) { ret |= 1<<i; } } ret }
unsafe fn aplic_write_enabled_word(aplic: *mut aplic, word: u32, val: u32, enabled: bool) { for i in 0..32 { if val & (1<<i) != 0 { aplic_write_enabled(aplic, word*32+i, enabled); } } }

unsafe fn aplic_mmio_read_offset(kvm: *mut kvm, off: gpa_t, val32: *mut u32) -> i32 {
    let aplic = (*kvm).arch.aia.aplic_state;
    if off & 3 != 0 { return -EOPNOTSUPP; }
    if off == APLIC_DOMAINCFG { *val32 = APLIC_DOMAINCFG_RDONLY | (*aplic).domaincfg | APLIC_DOMAINCFG_DM; }
    else if off >= APLIC_SOURCECFG_BASE && off < APLIC_SOURCECFG_BASE + ((*aplic).nr_irqs - 1) * 4 { *val32 = aplic_read_sourcecfg(aplic, ((off - APLIC_SOURCECFG_BASE) >> 2) + 1); }
    else if off >= APLIC_SETIP_BASE && off < APLIC_SETIP_BASE + (*aplic).nr_words * 4 { *val32 = aplic_read_pending_word(aplic, (off - APLIC_SETIP_BASE) >> 2); }
    else if off == APLIC_SETIPNUM { *val32 = 0; }
    else if off >= APLIC_CLRIP_BASE && off < APLIC_CLRIP_BASE + (*aplic).nr_words * 4 { *val32 = aplic_read_input_word(aplic, (off - APLIC_CLRIP_BASE) >> 2); }
    else if off == APLIC_CLRIPNUM { *val32 = 0; }
    else if off >= APLIC_SETIE_BASE && off < APLIC_SETIE_BASE + (*aplic).nr_words * 4 { *val32 = aplic_read_enabled_word(aplic, (off - APLIC_SETIE_BASE) >> 2); }
    else if off == APLIC_SETIENUM || off >= APLIC_CLRIE_BASE && off < APLIC_CLRIE_BASE + (*aplic).nr_words * 4 || off == APLIC_CLRIENUM || off == APLIC_SETIPNUM_LE || off == APLIC_SETIPNUM_BE { *val32 = 0; }
    else if off == APLIC_GENMSI { *val32 = (*aplic).genmsi; }
    else if off >= APLIC_TARGET_BASE && off < APLIC_TARGET_BASE + ((*aplic).nr_irqs - 1) * 4 { *val32 = aplic_read_target(aplic, ((off - APLIC_TARGET_BASE) >> 2) + 1); }
    else { return -ENODEV; }
    0
}

unsafe fn aplic_mmio_write_offset(kvm: *mut kvm, off: gpa_t, val32: u32) -> i32 {
    let aplic = (*kvm).arch.aia.aplic_state;
    if off & 3 != 0 { return -EOPNOTSUPP; }
    if off == APLIC_DOMAINCFG { (*aplic).domaincfg = val32 & APLIC_DOMAINCFG_IE; }
    else if off >= APLIC_SOURCECFG_BASE && off < APLIC_SOURCECFG_BASE + ((*aplic).nr_irqs - 1) * 4 { aplic_write_sourcecfg(aplic, ((off - APLIC_SOURCECFG_BASE) >> 2) + 1, val32); }
    else if off >= APLIC_SETIP_BASE && off < APLIC_SETIP_BASE + (*aplic).nr_words * 4 { aplic_write_pending_word(aplic, (off - APLIC_SETIP_BASE) >> 2, val32, true); }
    else if off == APLIC_SETIPNUM { aplic_write_pending(aplic, val32, true); }
    else if off >= APLIC_CLRIP_BASE && off < APLIC_CLRIP_BASE + (*aplic).nr_words * 4 { aplic_write_pending_word(aplic, (off - APLIC_CLRIP_BASE) >> 2, val32, false); }
    else if off == APLIC_CLRIPNUM { aplic_write_pending(aplic, val32, false); }
    else if off >= APLIC_SETIE_BASE && off < APLIC_SETIE_BASE + (*aplic).nr_words * 4 { aplic_write_enabled_word(aplic, (off - APLIC_SETIE_BASE) >> 2, val32, true); }
    else if off == APLIC_SETIENUM { aplic_write_enabled(aplic, val32, true); }
    else if off >= APLIC_CLRIE_BASE && off < APLIC_CLRIE_BASE + (*aplic).nr_words * 4 { aplic_write_enabled_word(aplic, (off - APLIC_CLRIE_BASE) >> 2, val32, false); }
    else if off == APLIC_CLRIENUM { aplic_write_enabled(aplic, val32, false); }
    else if off == APLIC_SETIPNUM_LE { aplic_write_pending(aplic, val32, true); }
    else if off == APLIC_SETIPNUM_BE { aplic_write_pending(aplic, val32.swap_bytes(), true); }
    else if off == APLIC_GENMSI { (*aplic).genmsi = val32 & !(APLIC_TARGET_GUEST_IDX_MASK << APLIC_TARGET_GUEST_IDX_SHIFT); kvm_riscv_aia_inject_msi_by_id(kvm, val32 >> APLIC_TARGET_HART_IDX_SHIFT, 0, val32 & APLIC_TARGET_EIID_MASK); }
    else if off >= APLIC_TARGET_BASE && off < APLIC_TARGET_BASE + ((*aplic).nr_irqs - 1) * 4 { aplic_write_target(aplic, ((off - APLIC_TARGET_BASE) >> 2) + 1, val32); }
    else { return -ENODEV; }
    aplic_update_irq_range(kvm, 1, (*aplic).nr_irqs - 1);
    0
}

unsafe extern "C" fn aplic_mmio_read(vcpu: *mut kvm_vcpu, _dev: *mut kvm_io_device, addr: gpa_t, len: i32, val: *mut core::ffi::c_void) -> i32 {
    if len != 4 { return -EOPNOTSUPP; }
    aplic_mmio_read_offset((*vcpu).kvm, addr - (*vcpu).kvm.arch.aia.aplic_addr, val as *mut u32)
}
unsafe extern "C" fn aplic_mmio_write(vcpu: *mut kvm_vcpu, _dev: *mut kvm_io_device, addr: gpa_t, len: i32, val: *const core::ffi::c_void) -> i32 {
    if len != 4 { return -EOPNOTSUPP; }
    aplic_mmio_write_offset((*vcpu).kvm, addr - (*vcpu).kvm.arch.aia.aplic_addr, *(val as *const u32))
}

static aplic_iodoev_ops: kvm_io_device_ops = kvm_io_device_ops { read: Some(aplic_mmio_read), write: Some(aplic_mmio_write) };

pub unsafe fn kvm_riscv_aia_aplic_set_attr(kvm: *mut kvm, ty: c_ulong, v: u32) -> i32 {
    if (*kvm).arch.aia.aplic_state.is_null() { return -ENODEV; }
    aplic_mmio_write_offset(kvm, ty, v)
}
pub unsafe fn kvm_riscv_aia_aplic_get_attr(kvm: *mut kvm, ty: c_ulong, v: *mut u32) -> i32 {
    if (*kvm).arch.aia.aplic_state.is_null() { return -ENODEV; }
    aplic_mmio_read_offset(kvm, ty, v)
}
pub unsafe fn kvm_riscv_aia_aplic_has_attr(kvm: *mut kvm, ty: c_ulong) -> i32 {
    if (*kvm).arch.aia.aplic_state.is_null() { return -ENODEV; }
    let mut val = 0; aplic_mmio_read_offset(kvm, ty, &mut val)
}

pub unsafe fn kvm_riscv_aia_aplic_init(kvm: *mut kvm) -> i32 {
    if (*kvm).arch.aia.nr_sources == 0 { return 0; }
    let aplic = kzalloc_flex_aplic((*kvm).arch.aia.nr_sources + 1, GFP_KERNEL_ACCOUNT);
    if aplic.is_null() { return -ENOMEM; }
    (*kvm).arch.aia.aplic_state = aplic;
    (*aplic).nr_irqs = (*kvm).arch.aia.nr_sources + 1;
    (*aplic).nr_words = DIV_ROUND_UP((*aplic).nr_irqs, 32);
    for i in 0..(*aplic).nr_irqs { raw_spin_lock_init(&mut (*aplic).irqs[i as usize].lock); }
    kvm_iodevice_init(&mut (*aplic).iodev, &aplic_iodoev_ops);
    0
}

pub unsafe fn kvm_riscv_aia_aplic_cleanup(kvm: *mut kvm) {
    let aplic = (*kvm).arch.aia.aplic_state;
    if aplic.is_null() { return; }
    (*kvm).arch.aia.aplic_state = core::ptr::null_mut();
    kfree(aplic);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
