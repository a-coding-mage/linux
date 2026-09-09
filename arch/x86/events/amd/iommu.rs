// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2013 Advanced Micro Devices, Inc.
 *
 * Author: Steven Kinney <Steven.Kinney@amd.com>
 * Author: Suravee Suthikulpanit <Suraveee.Suthikulpanit@amd.com>
 *
 * Perf: amd_iommu - AMD IOMMU Performance Counter PMU implementation
 */

// C dependencies: linux/perf_event.h, linux/init.h, linux/cpumask.h,
// linux/slab.h, linux/amd-iommu.h, asm/msr.h, ../perf_event.h, iommu.h.

const IOMMU_NAME_SIZE: usize = 24;

#[inline]
unsafe fn get_csource(x: *const hw_perf_event) -> u64 { (*x).conf & 0xff }
#[inline]
unsafe fn get_devid(x: *const hw_perf_event) -> u64 { ((*x).conf >> 8) & 0xffff }
#[inline]
unsafe fn get_domid(x: *const hw_perf_event) -> u64 { ((*x).conf >> 24) & 0xffff }
#[inline]
unsafe fn get_pasid(x: *const hw_perf_event) -> u64 { ((*x).conf >> 40) & 0xfffff }
#[inline]
unsafe fn get_devid_mask(x: *const hw_perf_event) -> u64 { (*x).conf1 & 0xffff }
#[inline]
unsafe fn get_domid_mask(x: *const hw_perf_event) -> u64 { ((*x).conf1 >> 16) & 0xffff }
#[inline]
unsafe fn get_pasid_mask(x: *const hw_perf_event) -> u64 { ((*x).conf1 >> 32) & 0xfffff }

#[repr(C)]
struct perf_amd_iommu {
    list: list_head,
    pmu: pmu,
    iommu: *mut amd_iommu,
    name: [c_char; IOMMU_NAME_SIZE],
    max_banks: u8,
    max_counters: u8,
    cntr_assign_mask: u64,
    lock: raw_spinlock_t,
}

static mut perf_amd_iommu_list: list_head = LIST_HEAD_INIT;

// sysfs format attributes
PMU_FORMAT_ATTR!(csource, "config:0-7");
PMU_FORMAT_ATTR!(devid, "config:8-23");
PMU_FORMAT_ATTR!(domid, "config:24-39");
PMU_FORMAT_ATTR!(pasid, "config:40-59");
PMU_FORMAT_ATTR!(devid_mask, "config1:0-15");
PMU_FORMAT_ATTR!(domid_mask, "config1:16-31");
PMU_FORMAT_ATTR!(pasid_mask, "config1:32-51");

static mut iommu_format_attrs: [*mut attribute; 8] = [
    &mut format_attr_csource.attr, &mut format_attr_devid.attr,
    &mut format_attr_pasid.attr, &mut format_attr_domid.attr,
    &mut format_attr_devid_mask.attr, &mut format_attr_pasid_mask.attr,
    &mut format_attr_domid_mask.attr, core::ptr::null_mut(),
];
static mut amd_iommu_format_group: attribute_group = attribute_group { name: c"format", attrs: iommu_format_attrs.as_mut_ptr() };
static mut amd_iommu_events_group: attribute_group = attribute_group { name: c"events", attrs: core::ptr::null_mut() };

#[repr(C)]
struct amd_iommu_event_desc { attr: device_attribute, event: *const c_char }

unsafe extern "C" fn _iommu_event_show(_dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let event = container_of!(attr, amd_iommu_event_desc, attr);
    sprintf!(buf, c"%s\n", (*event).event)
}

static mut amd_iommu_v2_event_descs: [amd_iommu_event_desc; 25] = [
    event_desc!(mem_pass_untrans, c"csource=0x01"), event_desc!(mem_pass_pretrans, c"csource=0x02"),
    event_desc!(mem_pass_excl, c"csource=0x03"), event_desc!(mem_target_abort, c"csource=0x04"),
    event_desc!(mem_trans_total, c"csource=0x05"), event_desc!(mem_iommu_tlb_pte_hit, c"csource=0x06"),
    event_desc!(mem_iommu_tlb_pte_mis, c"csource=0x07"), event_desc!(mem_iommu_tlb_pde_hit, c"csource=0x08"),
    event_desc!(mem_iommu_tlb_pde_mis, c"csource=0x09"), event_desc!(mem_dte_hit, c"csource=0x0a"),
    event_desc!(mem_dte_mis, c"csource=0x0b"), event_desc!(page_tbl_read_tot, c"csource=0x0c"),
    event_desc!(page_tbl_read_nst, c"csource=0x0d"), event_desc!(page_tbl_read_gst, c"csource=0x0e"),
    event_desc!(int_dte_hit, c"csource=0x0f"), event_desc!(int_dte_mis, c"csource=0x10"),
    event_desc!(cmd_processed, c"csource=0x11"), event_desc!(cmd_processed_inv, c"csource=0x12"),
    event_desc!(tlb_inv, c"csource=0x13"), event_desc!(ign_rd_wr_mmio_1ff8h, c"csource=0x14"),
    event_desc!(vapic_int_non_guest, c"csource=0x15"), event_desc!(vapic_int_guest, c"csource=0x16"),
    event_desc!(smi_recv, c"csource=0x17"), event_desc!(smi_blk, c"csource=0x18"), event_desc!(zero_event_desc!()),
];

static mut iommu_cpumask: cpumask_t = cpumask_t::default();
unsafe extern "C" fn _iommu_cpumask_show(_dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    sysfs_emit!(buf, c"%*pbl\n", cpumask_pr_args!(&iommu_cpumask))
}
DEVICE_ATTR!(cpumask, S_IRUGO, _iommu_cpumask_show, None);

unsafe fn get_next_avail_iommu_bnk_cntr(event: *mut perf_event) -> c_int {
    let piommu = container_of!((*event).pmu, perf_amd_iommu, pmu);
    let mut flags = 0UL;
    raw_spin_lock_irqsave!(&mut (*piommu).lock, &mut flags);
    for bank in 0..(*piommu).max_banks as u32 { for cntr in 0..(*piommu).max_counters as u32 {
        let shift = bank + bank * 3 + cntr;
        if (*piommu).cntr_assign_mask & (1u64 << shift) == 0 {
            (*piommu).cntr_assign_mask |= 1u64 << shift;
            (*event).hw.iommu_bank = bank as u8; (*event).hw.iommu_cntr = cntr as u8;
            raw_spin_unlock_irqrestore!(&mut (*piommu).lock, flags); return 0;
        }
    }}
    raw_spin_unlock_irqrestore!(&mut (*piommu).lock, flags); -ENOSPC
}

unsafe fn clear_avail_iommu_bnk_cntr(p: *mut perf_amd_iommu, bank: u8, cntr: u8) -> c_int {
    if bank > (*p).max_banks || cntr > (*p).max_counters { return -EINVAL; }
    let shift = bank + cntr + bank * 3; let mut flags = 0UL;
    raw_spin_lock_irqsave!(&mut (*p).lock, &mut flags); (*p).cntr_assign_mask &= !(1u64 << shift); raw_spin_unlock_irqrestore!(&mut (*p).lock, flags); 0
}

unsafe extern "C" fn perf_iommu_event_init(event: *mut perf_event) -> c_int {
    if (*event).attr.type_ != (*(*event).pmu).type_ { return -ENOENT; }
    if is_sampling_event!(event) || (*event).attach_state & PERF_ATTACH_TASK != 0 || (*event).cpu < 0 { return -EINVAL; }
    (*event).hw.conf = (*event).attr.config; (*event).hw.conf1 = (*event).attr.config1; 0
}

unsafe fn perf_event_2_iommu(ev: *mut perf_event) -> *mut amd_iommu { (container_of!((*ev).pmu, perf_amd_iommu, pmu)).iommu }

unsafe fn perf_iommu_enable_event(ev: *mut perf_event) {
    let iommu = perf_event_2_iommu(ev); let hwc = &mut (*ev).hw; let bank = hwc.iommu_bank; let cntr = hwc.iommu_cntr; let mut reg;
    reg = get_csource(hwc); amd_iommu_pc_set_reg(iommu, bank, cntr, IOMMU_PC_COUNTER_SRC_REG, &mut reg);
    reg = get_devid(hwc) | (get_devid_mask(hwc) << 32); if reg != 0 { reg |= 1 << 31; } amd_iommu_pc_set_reg(iommu, bank, cntr, IOMMU_PC_DEVID_MATCH_REG, &mut reg);
    reg = get_pasid(hwc) | (get_pasid_mask(hwc) << 32); if reg != 0 { reg |= 1 << 31; } amd_iommu_pc_set_reg(iommu, bank, cntr, IOMMU_PC_PASID_MATCH_REG, &mut reg);
    reg = get_domid(hwc) | (get_domid_mask(hwc) << 32); if reg != 0 { reg |= 1 << 31; } amd_iommu_pc_set_reg(iommu, bank, cntr, IOMMU_PC_DOMID_MATCH_REG, &mut reg);
}

unsafe fn perf_iommu_disable_event(event: *mut perf_event) { let iommu = perf_event_2_iommu(event); let hwc = &mut (*event).hw; let mut reg = 0u64; amd_iommu_pc_set_reg(iommu, hwc.iommu_bank, hwc.iommu_cntr, IOMMU_PC_COUNTER_SRC_REG, &mut reg); }

unsafe extern "C" fn perf_iommu_start(event: *mut perf_event, flags: c_int) { let hwc = &mut (*event).hw; if WARN_ON_ONCE!(hwc.state & PERF_HES_STOPPED == 0) { return; } WARN_ON_ONCE!(hwc.state & PERF_HES_UPTODATE == 0); hwc.state = 0; perf_iommu_enable_event(event); if flags & PERF_EF_RELOAD != 0 { let iommu = perf_event_2_iommu(event); let mut count = 0; amd_iommu_pc_set_reg(iommu, hwc.iommu_bank, hwc.iommu_cntr, IOMMU_PC_COUNTER_REG, &mut count); } perf_event_update_userpage(event); }

unsafe extern "C" fn perf_iommu_read(event: *mut perf_event) { let hwc = &mut (*event).hw; let iommu = perf_event_2_iommu(event); let mut count = 0; if amd_iommu_pc_get_reg(iommu, hwc.iommu_bank, hwc.iommu_cntr, IOMMU_PC_COUNTER_REG, &mut count) != 0 { return; } count &= (1u64 << 48) - 1; local64_add(count as i64, &mut (*event).count); }

unsafe extern "C" fn perf_iommu_stop(event: *mut perf_event, _flags: c_int) { let hwc = &mut (*event).hw; if hwc.state & PERF_HES_UPTODATE != 0 { return; } perf_iommu_read(event); hwc.state |= PERF_HES_UPTODATE; perf_iommu_disable_event(event); WARN_ON_ONCE!(hwc.state & PERF_HES_STOPPED != 0); hwc.state |= PERF_HES_STOPPED; }

unsafe extern "C" fn perf_iommu_add(event: *mut perf_event, flags: c_int) -> c_int { (*event).hw.state = PERF_HES_UPTODATE | PERF_HES_STOPPED; let ret = get_next_avail_iommu_bnk_cntr(event); if ret != 0 { return ret; } if flags & PERF_EF_START != 0 { perf_iommu_start(event, PERF_EF_RELOAD); } 0 }
unsafe extern "C" fn perf_iommu_del(event: *mut perf_event, _flags: c_int) { let hwc = &mut (*event).hw; let p = container_of!((*event).pmu, perf_amd_iommu, pmu); perf_iommu_stop(event, PERF_EF_UPDATE); clear_avail_iommu_bnk_cntr(p, hwc.iommu_bank, hwc.iommu_cntr); perf_event_update_userpage(event); }

// The remaining initialization is a direct translation of the kernel registration path.
unsafe fn _init_events_attrs() -> c_int { let mut i = 0; while !(*amd_iommu_v2_event_descs[i].attr.attr.name).is_null() { i += 1; } let attrs = kzalloc_objs!(*mut attribute, i + 1); if attrs.is_null() { return -ENOMEM; } for j in 0..i { *attrs.add(j) = &mut amd_iommu_v2_event_descs[j].attr.attr; } amd_iommu_events_group.attrs = attrs; 0 }

unsafe fn init_one_iommu(idx: c_uint) -> c_int {
    let perf_iommu = kzalloc_obj!(perf_amd_iommu);
    if perf_iommu.is_null() { return -ENOMEM; }
    raw_spin_lock_init!(&mut (*perf_iommu).lock);
    (*perf_iommu).pmu = iommu_pmu;
    (*perf_iommu).iommu = get_amd_iommu(idx);
    (*perf_iommu).max_banks = amd_iommu_pc_get_max_banks(idx);
    (*perf_iommu).max_counters = amd_iommu_pc_get_max_counters(idx);
    if (*perf_iommu).iommu.is_null() || (*perf_iommu).max_banks == 0 || (*perf_iommu).max_counters == 0 { kfree!(perf_iommu); return -EINVAL; }
    snprintf!((*perf_iommu).name.as_mut_ptr(), IOMMU_NAME_SIZE, c"amd_iommu_%u", idx);
    let ret = perf_pmu_register!(&mut (*perf_iommu).pmu, (*perf_iommu).name.as_ptr(), -1);
    if ret == 0 { pr_info!(c"Detected AMD IOMMU #%d (%d banks, %d counters/bank).\n", idx, (*perf_iommu).max_banks, (*perf_iommu).max_counters); list_add_tail!(&mut (*perf_iommu).list, &mut perf_amd_iommu_list); } else { pr_warn!(c"Error initializing IOMMU %d.\n", idx); kfree!(perf_iommu); }
    ret
}

static mut amd_iommu_attr_groups: [*const attribute_group; 4] = [&amd_iommu_format_group, &amd_iommu_cpumask_group, &amd_iommu_events_group, core::ptr::null()];
static iommu_pmu: pmu = pmu { event_init: Some(perf_iommu_event_init), add: Some(perf_iommu_add), del: Some(perf_iommu_del), start: Some(perf_iommu_start), stop: Some(perf_iommu_stop), read: Some(perf_iommu_read), task_ctx_nr: perf_invalid_context, attr_groups: amd_iommu_attr_groups.as_ptr(), capabilities: PERF_PMU_CAP_NO_EXCLUDE };

unsafe extern "C" fn amd_iommu_pc_init() -> c_int { if !amd_iommu_pc_supported() { return -ENODEV; } let ret = _init_events_attrs(); if ret != 0 { return ret; } let mut cnt = 0; for i in 0..amd_iommu_get_num_iommus() { if init_one_iommu(i) == 0 { cnt += 1; } } if cnt == 0 { kfree!(amd_iommu_events_group.attrs); return -ENODEV; } cpumask_set_cpu(0, &mut iommu_cpumask); 0 }

device_initcall!(amd_iommu_pc_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
