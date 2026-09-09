// SPDX-License-Identifier: GPL-2.0
/*
 * Library implementing the most common irq chip callback functions
 *
 * Copyright (C) 2011, Thomas Gleixner
 */

// C headers and build-time configuration supplied by the surrounding kernel.

static mut GC_LIST: ListHead = LIST_HEAD_INIT;
static mut GC_LOCK: RawSpinlock = DEFINE_RAW_SPINLOCK;

/// irq_gc_noop - NOOP function
/// @d: irq_data
pub unsafe extern "C" fn irq_gc_noop(_d: *mut IrqData) {}

/// irq_gc_mask_disable_reg - Mask chip via disable register
/// @d: irq_data
///
/// Chip has separate enable/disable registers instead of a single mask
/// register.
pub unsafe extern "C" fn irq_gc_mask_disable_reg(d: *mut IrqData) {
    let gc = irq_data_get_irq_chip_data(d);
    let ct = irq_data_get_chip_type(d);
    let mask: u32 = (*d).mask;
    let _guard = raw_spinlock_guard(&mut (*gc).lock);
    irq_reg_writel(gc, mask, (*ct).regs.disable);
    *(*ct).mask_cache &= !mask;
}

/// irq_gc_mask_set_bit - Mask chip via setting bit in mask register
/// @d: irq_data
///
/// Chip has a single mask register. Values of this register are cached
/// and protected by gc->lock
pub unsafe extern "C" fn irq_gc_mask_set_bit(d: *mut IrqData) {
    let gc = irq_data_get_irq_chip_data(d);
    let ct = irq_data_get_chip_type(d);
    let mask: u32 = (*d).mask;
    let _guard = raw_spinlock_guard(&mut (*gc).lock);
    *(*ct).mask_cache |= mask;
    irq_reg_writel(gc, *(*ct).mask_cache, (*ct).regs.mask);
}

/// irq_gc_mask_clr_bit - Mask chip via clearing bit in mask register
/// @d: irq_data
///
/// Chip has a single mask register. Values of this register are cached
/// and protected by gc->lock
pub unsafe extern "C" fn irq_gc_mask_clr_bit(d: *mut IrqData) {
    let gc = irq_data_get_irq_chip_data(d);
    let ct = irq_data_get_chip_type(d);
    let mask: u32 = (*d).mask;
    let _guard = raw_spinlock_guard(&mut (*gc).lock);
    *(*ct).mask_cache &= !mask;
    irq_reg_writel(gc, *(*ct).mask_cache, (*ct).regs.mask);
}

/// irq_gc_unmask_enable_reg - Unmask chip via enable register
/// @d: irq_data
///
/// Chip has separate enable/disable registers instead of a single mask
/// register.
pub unsafe extern "C" fn irq_gc_unmask_enable_reg(d: *mut IrqData) {
    let gc = irq_data_get_irq_chip_data(d);
    let ct = irq_data_get_chip_type(d);
    let mask: u32 = (*d).mask;
    let _guard = raw_spinlock_guard(&mut (*gc).lock);
    irq_reg_writel(gc, mask, (*ct).regs.enable);
    *(*ct).mask_cache |= mask;
}

/// irq_gc_ack_set_bit - Ack pending interrupt via setting bit
/// @d: irq_data
pub unsafe extern "C" fn irq_gc_ack_set_bit(d: *mut IrqData) {
    let gc = irq_data_get_irq_chip_data(d);
    let ct = irq_data_get_chip_type(d);
    let mask: u32 = (*d).mask;
    let _guard = raw_spinlock_guard(&mut (*gc).lock);
    irq_reg_writel(gc, mask, (*ct).regs.ack);
}

/// irq_gc_ack_clr_bit - Ack pending interrupt via clearing bit
/// @d: irq_data
pub unsafe extern "C" fn irq_gc_ack_clr_bit(d: *mut IrqData) {
    let gc = irq_data_get_irq_chip_data(d);
    let ct = irq_data_get_chip_type(d);
    let mask: u32 = !(*d).mask;
    let _guard = raw_spinlock_guard(&mut (*gc).lock);
    irq_reg_writel(gc, mask, (*ct).regs.ack);
}

/// irq_gc_mask_disable_and_ack_set - Mask and ack pending interrupt
/// @d: irq_data
pub unsafe extern "C" fn irq_gc_mask_disable_and_ack_set(d: *mut IrqData) {
    let gc = irq_data_get_irq_chip_data(d);
    let ct = irq_data_get_chip_type(d);
    let mask: u32 = (*d).mask;
    let _guard = raw_spinlock_guard(&mut (*gc).lock);
    irq_reg_writel(gc, mask, (*ct).regs.disable);
    *(*ct).mask_cache &= !mask;
    irq_reg_writel(gc, mask, (*ct).regs.ack);
}

/// irq_gc_eoi - EOI interrupt
/// @d: irq_data
pub unsafe extern "C" fn irq_gc_eoi(d: *mut IrqData) {
    let gc = irq_data_get_irq_chip_data(d);
    let ct = irq_data_get_chip_type(d);
    let mask: u32 = (*d).mask;
    let _guard = raw_spinlock_guard(&mut (*gc).lock);
    irq_reg_writel(gc, mask, (*ct).regs.eoi);
}

/// irq_gc_set_wake - Set/clr wake bit for an interrupt
/// @d: irq_data
/// @on: Indicates whether the wake bit should be set or cleared
pub unsafe extern "C" fn irq_gc_set_wake(d: *mut IrqData, on: u32) -> i32 {
    let gc = irq_data_get_irq_chip_data(d);
    let mask: u32 = (*d).mask;
    if mask & (*gc).wake_enabled == 0 { return -EINVAL; }
    let _guard = raw_spinlock_guard(&mut (*gc).lock);
    if on != 0 { (*gc).wake_active |= mask; } else { (*gc).wake_active &= !mask; }
    0
}

unsafe fn irq_readl_be(addr: *mut core::ffi::c_void) -> u32 { ioread32be(addr) }
unsafe fn irq_writel_be(val: u32, addr: *mut core::ffi::c_void) { iowrite32be(val, addr); }

pub unsafe extern "C" fn irq_init_generic_chip(gc: *mut IrqChipGeneric, name: *const i8,
    num_ct: i32, irq_base: u32, reg_base: *mut core::ffi::c_void, handler: IrqFlowHandlerT) {
    let ct = (*gc).chip_types;
    raw_spin_lock_init(&mut (*gc).lock);
    (*gc).num_ct = num_ct;
    (*gc).irq_base = irq_base;
    (*gc).reg_base = reg_base;
    for i in 0..num_ct { (*ct.add(i as usize)).chip.name = name; }
    (*gc).chip_types.as_mut().unwrap().handler = handler;
}

pub unsafe extern "C" fn irq_alloc_generic_chip(name: *const i8, num_ct: i32,
    irq_base: u32, reg_base: *mut core::ffi::c_void, handler: IrqFlowHandlerT)
    -> *mut IrqChipGeneric {
    let gc = kzalloc_flex(num_ct);
    if !gc.is_null() { irq_init_generic_chip(gc, name, num_ct, irq_base, reg_base, handler); }
    gc
}

unsafe fn irq_gc_init_mask_cache(gc: *mut IrqChipGeneric, flags: IrqGcFlags) {
    let ct = (*gc).chip_types;
    let mut mskptr = &mut (*gc).mask_cache as *mut u32;
    let mut mskreg = (*ct).regs.mask;
    for i in 0..(*gc).num_ct {
        if flags & IRQ_GC_MASK_CACHE_PER_TYPE != 0 { mskptr = &mut (*ct.add(i as usize)).mask_cache_priv; mskreg = (*ct.add(i as usize)).regs.mask; }
        (*ct.add(i as usize)).mask_cache = mskptr;
        if flags & IRQ_GC_INIT_MASK_CACHE != 0 { *mskptr = irq_reg_readl(gc, mskreg); }
    }
}

pub unsafe extern "C" fn irq_domain_alloc_generic_chips(d: *mut IrqDomain,
    info: *const IrqDomainChipGenericInfo) -> i32 {
    if !(*d).gc.is_null() { return -EBUSY; }
    let numchips = ((*d).revmap_size + (*info).irqs_per_chip - 1) / (*info).irqs_per_chip;
    if numchips == 0 { return -EINVAL; }
    let dgc_sz = struct_size_dgc(numchips); let gc_sz = struct_size_gc((*info).num_ct);
    let dgc = kzalloc(dgc_sz + numchips * gc_sz);
    if dgc.is_null() { return -ENOMEM; }
    (*d).gc = dgc;
    (*dgc).irqs_per_chip = (*info).irqs_per_chip; (*dgc).num_chips = numchips;
    (*dgc).irq_flags_to_set = (*info).irq_flags_to_set; (*dgc).irq_flags_to_clear = (*info).irq_flags_to_clear;
    (*dgc).gc_flags = (*info).gc_flags; (*dgc).exit = (*info).exit;
    let mut i = 0usize;
    while i < numchips {
        let gc = dgc_gc_storage(dgc, i, gc_sz);
        (*dgc).gc[i] = gc;
        irq_init_generic_chip(gc, (*info).name, (*info).num_ct,
            i as u32 * (*dgc).irqs_per_chip, core::ptr::null_mut(), (*info).handler);
        (*gc).domain = d;
        if (*dgc).gc_flags & IRQ_GC_BE_IO != 0 { (*gc).reg_readl = Some(irq_readl_be); (*gc).reg_writel = Some(irq_writel_be); }
        if let Some(init) = (*info).init { let ret = init(gc); if ret != 0 { while i > 0 { i -= 1; if let Some(exit) = (*dgc).exit { exit((*dgc).gc[i]); } irq_remove_generic_chip((*dgc).gc[i], !0, 0, 0); } (*d).gc = core::ptr::null_mut(); kfree(dgc); return ret; } }
        let _guard = raw_spinlock_irqsave_guard(&mut GC_LOCK); list_add_tail(&mut (*gc).list, &mut GC_LIST);
        i += 1;
    }
    0
}

pub unsafe extern "C" fn irq_domain_remove_generic_chips(d: *mut IrqDomain) {
    let dgc = (*d).gc; if dgc.is_null() { return; }
    for i in 0..(*dgc).num_chips { if let Some(exit) = (*dgc).exit { exit((*dgc).gc[i]); } irq_remove_generic_chip((*dgc).gc[i], !0, 0, 0); }
    (*d).gc = core::ptr::null_mut(); kfree(dgc);
}

pub unsafe extern "C" fn __irq_alloc_domain_generic_chips(d: *mut IrqDomain, irqs_per_chip: usize,
    num_ct: i32, name: *const i8, handler: IrqFlowHandlerT, clr: u32, set: u32, gcflags: IrqGcFlags) -> i32 {
    let info = IrqDomainChipGenericInfo { irqs_per_chip, num_ct, name, handler,
        irq_flags_to_clear: clr, irq_flags_to_set: set, gc_flags: gcflags, init: None, exit: None };
    irq_domain_alloc_generic_chips(d, &info)
}

unsafe fn __irq_get_domain_generic_chip(d: *mut IrqDomain, hw_irq: u32) -> *mut IrqChipGeneric {
    let dgc = (*d).gc; if dgc.is_null() { return ERR_PTR(-ENODEV); }
    let idx = hw_irq as usize / (*dgc).irqs_per_chip; if idx >= (*dgc).num_chips { return ERR_PTR(-EINVAL); } (*dgc).gc[idx]
}

pub unsafe extern "C" fn irq_get_domain_generic_chip(d: *mut IrqDomain, hw_irq: u32) -> *mut IrqChipGeneric {
    let gc = __irq_get_domain_generic_chip(d, hw_irq); if IS_ERR(gc) { core::ptr::null_mut() } else { gc }
}

static mut IRQ_NESTED_LOCK_CLASS: LockClassKey = LockClassKey::new();
static mut IRQ_NESTED_REQUEST_CLASS: LockClassKey = LockClassKey::new();

pub unsafe extern "C" fn irq_map_generic_chip(d: *mut IrqDomain, virq: u32, hw_irq: u32) -> i32 {
    let data = irq_domain_get_irq_data(d, virq); let dgc = (*d).gc;
    let gc = __irq_get_domain_generic_chip(d, hw_irq); if IS_ERR(gc) { return PTR_ERR(gc); }
    let idx = hw_irq as usize % (*dgc).irqs_per_chip; if test_bit(idx, &(*gc).unused) { return -ENOTSUPP; } if test_bit(idx, &(*gc).installed) { return -EBUSY; }
    let ct = (*gc).chip_types; let chip = &mut (*ct).chip;
    if (*gc).installed == 0 { let _guard = raw_spinlock_irqsave_guard(&mut (*gc).lock); irq_gc_init_mask_cache(gc, (*dgc).gc_flags); }
    set_bit(idx, &mut (*gc).installed);
    if (*dgc).gc_flags & IRQ_GC_INIT_NESTED_LOCK != 0 { irq_set_lockdep_class(virq, &mut IRQ_NESTED_LOCK_CLASS, &mut IRQ_NESTED_REQUEST_CLASS); }
    if let Some(calc) = (*chip).irq_calc_mask { calc(data); } else { (*data).mask = 1u32 << idx; }
    irq_domain_set_info(d, virq, hw_irq, chip, gc, (*ct).handler, core::ptr::null_mut(), core::ptr::null_mut());
    irq_modify_status(virq, (*dgc).irq_flags_to_clear, (*dgc).irq_flags_to_set); 0
}

pub unsafe extern "C" fn irq_unmap_generic_chip(d: *mut IrqDomain, virq: u32) {
    let data = irq_domain_get_irq_data(d, virq); let dgc = (*d).gc; let hw_irq = (*data).hwirq;
    let gc = irq_get_domain_generic_chip(d, hw_irq); if gc.is_null() { return; }
    clear_bit((hw_irq as usize) % (*dgc).irqs_per_chip, &mut (*gc).installed);
    irq_domain_set_info(d, virq, hw_irq, &mut NO_IRQ_CHIP, core::ptr::null_mut(), None, core::ptr::null_mut(), core::ptr::null_mut());
}

pub static mut IRQ_GENERIC_CHIP_OPS: IrqDomainOps = IrqDomainOps { map: Some(irq_map_generic_chip), unmap: Some(irq_unmap_generic_chip), xlate: Some(irq_domain_xlate_onetwocell) };

pub unsafe extern "C" fn irq_setup_generic_chip(gc: *mut IrqChipGeneric, mut msk: u32, flags: IrqGcFlags, clr: u32, set: u32) {
    let ct = (*gc).chip_types; let chip = &mut (*ct).chip; let _guard = raw_spinlock_guard(&mut GC_LOCK); list_add_tail(&mut (*gc).list, &mut GC_LIST);
    irq_gc_init_mask_cache(gc, flags); let mut i = (*gc).irq_base;
    while msk != 0 { if msk & 1 != 0 { if flags & IRQ_GC_INIT_NESTED_LOCK != 0 { irq_set_lockdep_class(i, &mut IRQ_NESTED_LOCK_CLASS, &mut IRQ_NESTED_REQUEST_CLASS); } if flags & IRQ_GC_NO_MASK == 0 { let d = irq_get_irq_data(i); if let Some(calc) = (*chip).irq_calc_mask { calc(d); } else { (*d).mask = 1u32 << (i - (*gc).irq_base); } } irq_set_chip_and_handler(i, chip, (*ct).handler); irq_set_chip_data(i, gc); irq_modify_status(i, clr, set); } msk >>= 1; i += 1; }
    (*gc).irq_cnt = i - (*gc).irq_base;
}

pub unsafe extern "C" fn irq_setup_alt_chip(d: *mut IrqData, typ: u32) -> i32 {
    let gc = irq_data_get_irq_chip_data(d); let mut ct = (*gc).chip_types;
    for _ in 0..(*gc).num_ct { if (*ct).typ & typ != 0 { (*d).chip = &mut (*ct).chip; (*irq_data_to_desc(d)).handle_irq = (*ct).handler; return 0; } ct = ct.add(1); } -EINVAL
}

pub unsafe extern "C" fn irq_remove_generic_chip(gc: *mut IrqChipGeneric, mut msk: u32, clr: u32, set: u32) {
    let _guard = raw_spinlock_guard(&mut GC_LOCK); list_del(&mut (*gc).list); let mut i = 0u32;
    while msk != 0 { if msk & 1 != 0 { let virq = if !(*gc).domain.is_null() { let v = irq_find_mapping((*gc).domain, (*gc).irq_base + i); if v == 0 { i += 1; msk >>= 1; continue; } v } else { (*gc).irq_base + i }; irq_set_handler(virq, None); irq_set_chip(virq, &mut NO_IRQ_CHIP); irq_set_chip_data(virq, core::ptr::null_mut()); irq_modify_status(virq, clr, set); } i += 1; msk >>= 1; }
}

unsafe fn irq_gc_get_irq_data(gc: *mut IrqChipGeneric) -> *mut IrqData {
    if (*gc).domain.is_null() { return irq_get_irq_data((*gc).irq_base); }
    if (*gc).installed == 0 { return core::ptr::null_mut(); }
    let virq = irq_find_mapping((*gc).domain, (*gc).irq_base + __ffs((*gc).installed)); if virq != 0 { irq_get_irq_data(virq) } else { core::ptr::null_mut() }
}

// CONFIG_PM: these callbacks are present when power management is enabled.
#[cfg(feature = "CONFIG_PM")]
unsafe fn irq_gc_suspend(_data: *mut core::ffi::c_void) -> i32 { let mut gc = gc_list_first(); while !gc.is_null() { let ct = (*gc).chip_types; if let Some(f) = (*ct).chip.irq_suspend { let d = irq_gc_get_irq_data(gc); if !d.is_null() { f(d); } } if let Some(f) = (*gc).suspend { f(gc); } gc = gc_list_next(gc); } 0 }
#[cfg(feature = "CONFIG_PM")]
unsafe fn irq_gc_resume(_data: *mut core::ffi::c_void) { let mut gc = gc_list_first(); while !gc.is_null() { let ct = (*gc).chip_types; if let Some(f) = (*gc).resume { f(gc); } if let Some(f) = (*ct).chip.irq_resume { let d = irq_gc_get_irq_data(gc); if !d.is_null() { f(d); } } gc = gc_list_next(gc); } }
#[cfg(not(feature = "CONFIG_PM"))]
const IRQ_GC_SUSPEND: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32> = None;
#[cfg(not(feature = "CONFIG_PM"))]
const IRQ_GC_RESUME: Option<unsafe extern "C" fn(*mut core::ffi::c_void)> = None;

unsafe fn irq_gc_shutdown(_data: *mut core::ffi::c_void) { let mut gc = gc_list_first(); while !gc.is_null() { let ct = (*gc).chip_types; if let Some(f) = (*ct).chip.irq_pm_shutdown { let d = irq_gc_get_irq_data(gc); if !d.is_null() { f(d); } } gc = gc_list_next(gc); } }

static IRQ_GC_SYSCORE_OPS: SyscoreOps = SyscoreOps { suspend: Some(irq_gc_suspend), resume: Some(irq_gc_resume), shutdown: Some(irq_gc_shutdown) };
static mut IRQ_GC_SYSCORE: Syscore = Syscore { ops: &IRQ_GC_SYSCORE_OPS };

unsafe extern "C" fn irq_gc_init_ops() -> i32 { register_syscore(&mut IRQ_GC_SYSCORE); 0 }
// device_initcall(irq_gc_init_ops)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
