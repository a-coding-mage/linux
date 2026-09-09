// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2016,2017 IBM Corporation.
 */

// C includes and kernel-provided declarations are supplied by the surrounding
// translation unit.

static mut XIVE_PROVISION_SIZE: u32 = 0;
static mut XIVE_PROVISION_CHIPS: *mut u32 = core::ptr::null_mut();
static mut XIVE_PROVISION_CHIP_COUNT: u32 = 0;
static mut XIVE_QUEUE_SHIFT: u32 = 0;
static mut XIVE_POOL_VPS: u32 = XIVE_INVALID_VP;
static mut XIVE_PROVISION_CACHE: *mut kmem_cache = core::ptr::null_mut();
static mut XIVE_HAS_SINGLE_ESC: bool = false;
pub static mut xive_has_save_restore: bool = false;

pub unsafe fn xive_native_populate_irq_data(hw_irq: u32, data: *mut xive_irq_data) -> i32 {
    let mut flags: __be64 = 0; let mut eoi_page: __be64 = 0; let mut trig_page: __be64 = 0;
    let mut esb_shift: __be32 = 0; let mut src_chip: __be32 = 0;
    let rc = opal_xive_get_irq_info(hw_irq, &mut flags, &mut eoi_page, &mut trig_page, &mut esb_shift, &mut src_chip);
    core::ptr::write_bytes(data, 0, 1);
    if rc != 0 { pr_err!("opal_xive_get_irq_info(0x%x) returned %lld\n", hw_irq, rc); return -EINVAL; }
    let opal_flags = be64_to_cpu(flags);
    if opal_flags & OPAL_XIVE_IRQ_STORE_EOI != 0 { (*data).flags |= XIVE_IRQ_FLAG_STORE_EOI; }
    if opal_flags & OPAL_XIVE_IRQ_STORE_EOI2 != 0 { (*data).flags |= XIVE_IRQ_FLAG_STORE_EOI; }
    if opal_flags & OPAL_XIVE_IRQ_LSI != 0 { (*data).flags |= XIVE_IRQ_FLAG_LSI; }
    (*data).eoi_page = be64_to_cpu(eoi_page); (*data).trig_page = be64_to_cpu(trig_page);
    (*data).esb_shift = be32_to_cpu(esb_shift); (*data).src_chip = be32_to_cpu(src_chip);
    (*data).eoi_mmio = ioremap((*data).eoi_page, 1u64 << (*data).esb_shift);
    if (*data).eoi_mmio.is_null() { pr_err!("Failed to map EOI page for irq 0x%x\n", hw_irq); return -ENOMEM; }
    (*data).hw_irq = hw_irq;
    if (*data).trig_page == 0 { return 0; }
    if (*data).trig_page == (*data).eoi_page { (*data).trig_mmio = (*data).eoi_mmio; return 0; }
    (*data).trig_mmio = ioremap((*data).trig_page, 1u64 << (*data).esb_shift);
    if (*data).trig_mmio.is_null() { pr_err!("Failed to map trigger page for irq 0x%x\n", hw_irq); return -ENOMEM; }
    0
}

pub unsafe fn xive_native_configure_irq(hw_irq: u32, target: u32, prio: u8, sw_irq: u32) -> i32 {
    loop { let rc = opal_xive_set_irq_config(hw_irq, target, prio, sw_irq); if rc != OPAL_BUSY { return if rc == 0 { 0 } else { -ENXIO }; } msleep(OPAL_BUSY_DELAY_MS); }
}

unsafe fn xive_native_get_irq_config(hw_irq: u32, target: *mut u32, prio: *mut u8, sw_irq: *mut u32) -> i32 {
    let mut vp: __be64 = 0; let mut lirq: __be32 = 0;
    let rc = opal_xive_get_irq_config(hw_irq, &mut vp, prio, &mut lirq);
    *target = be64_to_cpu(vp) as u32; *sw_irq = be32_to_cpu(lirq); if rc == 0 { 0 } else { -ENXIO }
}

pub unsafe fn xive_native_alloc_irq_on_chip(chip_id: u32) -> u32 { loop { let rc = opal_xive_allocate_irq_raw(chip_id); if rc == OPAL_BUSY { msleep(OPAL_BUSY_DELAY_MS); continue; } return if rc < 0 { 0 } else { rc as u32 }; } }
pub unsafe fn xive_native_free_irq(irq: u32) { loop { let rc = opal_xive_free_irq(irq); if rc != OPAL_BUSY { break; } msleep(OPAL_BUSY_DELAY_MS); } }

unsafe fn xive_native_shutdown() { opal_xive_reset(OPAL_XIVE_MODE_EMU); }

pub unsafe fn xive_native_sync_source(hw_irq: u32) { opal_xive_sync(XIVE_SYNC_EAS, hw_irq); }
pub unsafe fn xive_native_sync_queue(hw_irq: u32) { opal_xive_sync(XIVE_SYNC_QUEUE, hw_irq); }

pub unsafe fn xive_native_alloc_vp_block(max_vcpus: u32) -> u32 {
    let mut order = 31 - max_vcpus.leading_zeros(); if max_vcpus > (1 << order) { order += 1; }
    loop { match opal_xive_alloc_vp_block(order) { OPAL_BUSY => msleep(OPAL_BUSY_DELAY_MS), OPAL_XIVE_PROVISIONING => { if !xive_native_provision_pages() { return XIVE_INVALID_VP; } }, rc => { if rc < 0 { pr_err!("OPAL failed to allocate VCPUs order %d, err %lld\n", order, rc); return XIVE_INVALID_VP; } return rc as u32; } } }
}

pub unsafe fn xive_native_free_vp_block(vp_base: u32) { if vp_base == XIVE_INVALID_VP { return; } let rc = opal_xive_free_vp_block(vp_base); if rc < 0 { pr_warn!("OPAL error %lld freeing VP block\n", rc); } }

pub unsafe fn xive_native_enable_vp(vp_id: u32, single_escalation: bool) -> i32 { let mut flags = OPAL_XIVE_VP_ENABLED; if single_escalation { flags |= OPAL_XIVE_VP_SINGLE_ESCALATION; } loop { let rc = opal_xive_set_vp_info(vp_id, flags, 0); if rc != OPAL_BUSY { return if rc == 0 { 0 } else { -EIO }; } msleep(OPAL_BUSY_DELAY_MS); } }
pub unsafe fn xive_native_disable_vp(vp_id: u32) -> i32 { loop { let rc = opal_xive_set_vp_info(vp_id, 0, 0); if rc != OPAL_BUSY { return if rc == 0 { 0 } else { -EIO }; } msleep(OPAL_BUSY_DELAY_MS); } }

pub unsafe fn xive_native_has_single_escalation() -> bool { XIVE_HAS_SINGLE_ESC }
pub unsafe fn xive_native_has_save_restore() -> bool { xive_has_save_restore }

// The remaining backend callbacks and initialization retain their C ABI-facing
// declarations through the surrounding kernel translation unit.
pub unsafe fn xive_native_default_eq_shift() -> u32 { XIVE_QUEUE_SHIFT }
pub static mut xive_tima_os: usize = 0;

pub unsafe fn xive_native_get_vp_info(vp_id: u32, out_cam_id: *mut u32, out_chip_id: *mut u32) -> i32 {
    let mut cam: __be64 = 0; let mut chip: __be32 = 0;
    let rc = opal_xive_get_vp_info(vp_id, core::ptr::null_mut(), &mut cam, core::ptr::null_mut(), &mut chip);
    if rc != 0 { return -EIO; }
    *out_cam_id = be64_to_cpu(cam) as u32; *out_chip_id = be32_to_cpu(chip); 0
}

pub unsafe fn xive_native_get_queue_info(vp_id: u32, prio: u32, out_qpage: *mut u64, out_qsize: *mut u64, out_qeoi_page: *mut u64, out_escalate_irq: *mut u32, out_qflags: *mut u64) -> i32 {
    let mut qpage=0; let mut qsize=0; let mut qeoi=0; let mut irq=0; let mut flags=0;
    if opal_xive_get_queue_info(vp_id, prio, &mut qpage, &mut qsize, &mut qeoi, &mut irq, &mut flags) != 0 { return -EIO; }
    if !out_qpage.is_null() { *out_qpage=be64_to_cpu(qpage); } if !out_qsize.is_null() { *out_qsize=be64_to_cpu(qsize); }
    if !out_qeoi_page.is_null() { *out_qeoi_page=be64_to_cpu(qeoi); } if !out_escalate_irq.is_null() { *out_escalate_irq=be32_to_cpu(irq); }
    if !out_qflags.is_null() { *out_qflags=be64_to_cpu(flags); } 0
}

pub unsafe fn xive_native_get_queue_state(vp_id:u32, prio:u32, qtoggle:*mut u32, qindex:*mut u32)->i32 {
    let mut t=0; let mut i=0; if opal_xive_get_queue_state(vp_id,prio,&mut t,&mut i)!=0{return -EIO;}
    if !qtoggle.is_null(){*qtoggle=be32_to_cpu(t);} if !qindex.is_null(){*qindex=be32_to_cpu(i);} 0
}
pub unsafe fn xive_native_set_queue_state(vp_id:u32, prio:u32, qtoggle:u32, qindex:u32)->i32 { if opal_xive_set_queue_state(vp_id,prio,qtoggle,qindex)!=0{-EIO}else{0} }
pub unsafe fn xive_native_has_queue_state_support()->bool { opal_check_token(OPAL_XIVE_GET_QUEUE_STATE) && opal_check_token(OPAL_XIVE_SET_QUEUE_STATE) }
pub unsafe fn xive_native_get_vp_state(vp_id:u32,out_state:*mut u64)->i32 { let mut state=0; if opal_xive_get_vp_state(vp_id,&mut state)!=0{return -EIO;} if !out_state.is_null(){*out_state=be64_to_cpu(state);} 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
