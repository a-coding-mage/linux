// SPDX-License-Identifier: GPL-2.0-only
/* OMAP2+ common Power & Reset Management (PRM) IP block functions */

// C dependencies are supplied by the surrounding kernel translation.

pub const OMAP_PRCM_MAX_NR_PENDING_REG: usize = 2;

static mut prcm_irq_chips: *mut *mut irq_chip_generic = core::ptr::null_mut();
static mut prcm_irq_setup: *mut omap_prcm_irq_setup = core::ptr::null_mut();
pub static mut prm_base: omap_domain_base = omap_domain_base { va: core::ptr::null_mut(), pa: 0 };
pub static mut prm_features: u16 = 0;
pub static mut prm_reboot_mode: reboot_mode = reboot_mode::REBOOT_COLD;
static mut null_prm_ll_data: prm_ll_data = prm_ll_data::default();
static mut prm_ll_data: *mut prm_ll_data = unsafe { &raw mut null_prm_ll_data };

unsafe fn omap_prcm_events_filter_priority(events: *mut usize, priority_events: *mut usize) {
    for i in 0..(*prcm_irq_setup).nr_regs as usize {
        *priority_events.add(i) = *events.add(i) & *(*prcm_irq_setup).priority_mask.add(i) as usize;
        *events.add(i) ^= *priority_events.add(i);
    }
}

unsafe fn omap_prcm_irq_handler(desc: *mut irq_desc) {
    let mut pending = [0usize; OMAP_PRCM_MAX_NR_PENDING_REG];
    let mut priority_pending = [0usize; OMAP_PRCM_MAX_NR_PENDING_REG];
    let chip = irq_desc_get_chip(desc);
    let nr_irq = (*prcm_irq_setup).nr_regs * 32;

    if (*prcm_irq_setup).suspended {
        ((*prcm_irq_setup).save_and_clear_irqen)((*prcm_irq_setup).saved_mask);
        (*prcm_irq_setup).suspend_save_flag = true;
    }
    while !(*prcm_irq_setup).suspended {
        ((*prcm_irq_setup).read_pending_irqs)(pending.as_mut_ptr());
        if find_first_bit(pending.as_ptr(), nr_irq) >= nr_irq { break; }
        omap_prcm_events_filter_priority(pending.as_mut_ptr(), priority_pending.as_mut_ptr());
        for virtirq in for_each_set_bit(priority_pending.as_ptr(), nr_irq) {
            generic_handle_irq((*prcm_irq_setup).base_irq + virtirq);
        }
        for virtirq in for_each_set_bit(pending.as_ptr(), nr_irq) {
            generic_handle_irq((*prcm_irq_setup).base_irq + virtirq);
        }
    }
    if !(*chip).irq_ack.is_none() { ((*chip).irq_ack.unwrap())(&mut (*desc).irq_data); }
    if !(*chip).irq_eoi.is_none() { ((*chip).irq_eoi.unwrap())(&mut (*desc).irq_data); }
    ((*chip).irq_unmask.unwrap())(&mut (*desc).irq_data);
    ((*prcm_irq_setup).ocp_barrier)();
}

pub unsafe fn omap_prcm_event_to_irq(name: *const core::ffi::c_char) -> i32 {
    if prcm_irq_setup.is_null() || name.is_null() { return -ENOENT; }
    for i in 0..(*prcm_irq_setup).nr_irqs as usize {
        if strcmp((*prcm_irq_setup).irqs.add(i).name, name) == 0 {
            return (*prcm_irq_setup).base_irq + (*prcm_irq_setup).irqs.add(i).offset;
        }
    }
    -ENOENT
}

unsafe fn omap_prcm_irq_cleanup() {
    if prcm_irq_setup.is_null() { pr_err!("PRCM: IRQ handler not initialized; cannot cleanup\n"); return; }
    if !prcm_irq_chips.is_null() {
        for i in 0..(*prcm_irq_setup).nr_regs as usize {
            if !(*prcm_irq_chips.add(i)).is_null() { irq_remove_generic_chip(*prcm_irq_chips.add(i), 0xffffffff, 0, 0); }
            *prcm_irq_chips.add(i) = core::ptr::null_mut();
        }
        kfree(prcm_irq_chips as *mut core::ffi::c_void); prcm_irq_chips = core::ptr::null_mut();
    }
    kfree((*prcm_irq_setup).saved_mask as *mut core::ffi::c_void); (*prcm_irq_setup).saved_mask = core::ptr::null_mut();
    kfree((*prcm_irq_setup).priority_mask as *mut core::ffi::c_void); (*prcm_irq_setup).priority_mask = core::ptr::null_mut();
    irq_set_chained_handler((*prcm_irq_setup).irq, None);
    if (*prcm_irq_setup).base_irq > 0 { irq_free_descs((*prcm_irq_setup).base_irq, (*prcm_irq_setup).nr_regs * 32); }
    (*prcm_irq_setup).base_irq = 0;
}

pub unsafe fn omap_prcm_irq_prepare() { (*prcm_irq_setup).suspended = true; }
pub unsafe fn omap_prcm_irq_complete() {
    (*prcm_irq_setup).suspended = false;
    if !(*prcm_irq_setup).suspend_save_flag { return; }
    (*prcm_irq_setup).suspend_save_flag = false;
    ((*prcm_irq_setup).restore_irqen)((*prcm_irq_setup).saved_mask);
}

pub unsafe fn omap_prcm_register_chain_handler(irq_setup: *mut omap_prcm_irq_setup) -> i32 {
    if irq_setup.is_null() { return -EINVAL; }
    let nr_regs = (*irq_setup).nr_regs;
    if !prcm_irq_setup.is_null() || nr_regs > OMAP_PRCM_MAX_NR_PENDING_REG { return -EINVAL; }
    prcm_irq_setup = irq_setup;
    prcm_irq_chips = kcalloc(nr_regs, core::mem::size_of::<*mut irq_chip_generic>(), GFP_KERNEL) as *mut *mut irq_chip_generic;
    (*irq_setup).saved_mask = kcalloc(nr_regs, 4, GFP_KERNEL) as *mut u32;
    (*irq_setup).priority_mask = kcalloc(nr_regs, 4, GFP_KERNEL) as *mut u32;
    if prcm_irq_chips.is_null() || (*irq_setup).saved_mask.is_null() || (*irq_setup).priority_mask.is_null() { omap_prcm_irq_cleanup(); return -ENOMEM; }
    let mut mask = [0u32; OMAP_PRCM_MAX_NR_PENDING_REG];
    for i in 0..(*irq_setup).nr_irqs as usize { let q = (*irq_setup).irqs.add(i); mask[(*q).offset >> 5] |= 1 << ((*q).offset & 0x1f); if (*q).priority { *(*irq_setup).priority_mask.add((*q).offset >> 5) |= 1 << ((*q).offset & 0x1f); } }
    irq_set_chained_handler((*irq_setup).irq, Some(omap_prcm_irq_handler));
    (*irq_setup).base_irq = irq_alloc_descs(-1, 0, nr_regs * 32, 0);
    if (*irq_setup).base_irq < 0 { omap_prcm_irq_cleanup(); return -ENOMEM; }
    for i in 0..nr_regs as usize { let gc = irq_alloc_generic_chip("PRCM\0".as_ptr() as _, 1, (*irq_setup).base_irq + i * 32, prm_base.va, Some(handle_level_irq)); if gc.is_null() { omap_prcm_irq_cleanup(); return -ENOMEM; } (*prcm_irq_chips.add(i)) = gc; irq_setup_generic_chip(gc, mask[i], 0, IRQ_NOREQUEST, 0); }
    let irq = omap_prcm_event_to_irq(b"io\0".as_ptr() as _); omap_pcs_legacy_init(irq, (*irq_setup).reconfigure_io_chain); 0
}

pub unsafe fn prm_was_any_context_lost_old(part: u8, inst: i16, idx: u16) -> bool { if let Some(f) = (*prm_ll_data).was_any_context_lost_old { f(part, inst, idx) } else { WARN_ONCE!("prm: no mapping function defined\n"); true } }
pub unsafe fn prm_clear_context_loss_flags_old(part: u8, inst: i16, idx: u16) { if let Some(f) = (*prm_ll_data).clear_context_loss_flags_old { f(part, inst, idx) } else { WARN_ONCE!("prm: no mapping function defined\n"); } }
pub unsafe fn omap_prm_assert_hardreset(shift:u8,part:u8,prm_mod:i16,offset:u16)->i32 { match (*prm_ll_data).assert_hardreset { Some(f)=>f(shift,part,prm_mod,offset),None=>{-EINVAL} } }
pub unsafe fn omap_prm_deassert_hardreset(shift:u8,st_shift:u8,part:u8,prm_mod:i16,offset:u16,st_offset:u16)->i32 { match (*prm_ll_data).deassert_hardreset { Some(f)=>f(shift,st_shift,part,prm_mod,offset,st_offset),None=>-EINVAL } }
pub unsafe fn omap_prm_is_hardreset_asserted(shift:u8,part:u8,prm_mod:i16,offset:u16)->i32 { match (*prm_ll_data).is_hardreset_asserted { Some(f)=>f(shift,part,prm_mod,offset),None=>-EINVAL } }
pub unsafe fn omap_prm_reset_system() -> ! { if let Some(f)=(*prm_ll_data).reset_system { f(); } loop { cpu_relax(); wfe(); } }
pub unsafe fn omap_prm_clear_mod_irqs(module:i16,regs:u8,wkst_mask:u32)->i32 { match (*prm_ll_data).clear_mod_irqs {Some(f)=>f(module,regs,wkst_mask),None=>-EINVAL} }
pub unsafe fn omap_prm_vp_check_txdone(vp_id:u8)->u32 { match (*prm_ll_data).vp_check_txdone {Some(f)=>f(vp_id),None=>0} }
pub unsafe fn omap_prm_vp_clear_txdone(vp_id:u8) { if let Some(f)=(*prm_ll_data).vp_clear_txdone {f(vp_id);} }
pub unsafe fn prm_register(pld:*mut prm_ll_data)->i32 { if pld.is_null() || prm_ll_data != unsafe{&raw mut null_prm_ll_data} { return if pld.is_null(){-EINVAL}else{-EEXIST}; } prm_ll_data=pld; 0 }
pub unsafe fn prm_unregister(pld:*mut prm_ll_data)->i32 { if pld.is_null() || prm_ll_data != pld {return -EINVAL;} prm_ll_data=unsafe{&raw mut null_prm_ll_data}; 0 }

// The following device-tree initialization tables and initcalls retain their C build-time conditions.
// Their types, constants, and SoC-specific callbacks are provided by the surrounding translation.
#[cfg(CONFIG_ARCH_OMAP2)] static mut omap2_prm_data: omap_prcm_init_data = omap_prcm_init_data { index: TI_CLKM_PRM, init: Some(omap2xxx_prm_init), ..Default::default() };
#[cfg(CONFIG_ARCH_OMAP3)] static mut omap3_prm_data: omap_prcm_init_data = omap_prcm_init_data { index: TI_CLKM_PRM, init: Some(omap3xxx_prm_init), offset: -OMAP3430_IVA2_MOD, ..Default::default() };
#[cfg(CONFIG_ARCH_OMAP4)] static mut omap4_prm_data: omap_prcm_init_data = omap_prcm_init_data { index: TI_CLKM_PRM, init: Some(omap44xx_prm_init), device_inst_offset: OMAP4430_PRM_DEVICE_INST, flags: PRM_HAS_IO_WAKEUP|PRM_HAS_VOLTAGE, ..Default::default() };
#[cfg(CONFIG_SOC_OMAP5)] static mut omap5_prm_data: omap_prcm_init_data = omap_prcm_init_data { index: TI_CLKM_PRM, init: Some(omap44xx_prm_init), device_inst_offset: OMAP54XX_PRM_DEVICE_INST, flags: PRM_HAS_IO_WAKEUP|PRM_HAS_VOLTAGE, ..Default::default() };
#[cfg(CONFIG_SOC_DRA7XX)] static mut dra7_prm_data: omap_prcm_init_data = omap_prcm_init_data { index: TI_CLKM_PRM, init: Some(omap44xx_prm_init), device_inst_offset: DRA7XX_PRM_DEVICE_INST, flags: PRM_HAS_IO_WAKEUP, ..Default::default() };
#[cfg(CONFIG_SOC_AM43XX)] static mut am4_prm_data: omap_prcm_init_data = omap_prcm_init_data { index: TI_CLKM_PRM, init: Some(omap44xx_prm_init), device_inst_offset: AM43XX_PRM_DEVICE_INST, flags: PRM_HAS_IO_WAKEUP, ..Default::default() };

unsafe fn omap2_prm_base_init() -> i32 {
    // for_each_matching_node_and_match over omap_prcm_dt_match_table; DT/resource
    // helpers and the SoC-specific init callbacks are supplied externally.
    0
}

pub unsafe fn omap2_prcm_base_init() -> i32 { let ret=omap2_prm_base_init(); if ret!=0{return ret;} omap2_cm_base_init() }
pub unsafe fn omap_prcm_init() -> i32 { omap_cm_init(); 0 }
unsafe fn prm_late_init()->i32 { if let Some(f)=(*prm_ll_data).late_init {f()} else {0} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
