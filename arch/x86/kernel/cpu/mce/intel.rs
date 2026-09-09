// SPDX-License-Identifier: GPL-2.0
/*
 * Intel specific MCE features.
 * Copyright 2004 Zwane Mwaikambo <zwane@linuxpower.ca>
 * Copyright (C) 2008, 2009 Intel Corporation
 * Author: Andi Kleen
 */

// Linux kernel headers and "internal.h" provide the external symbols used here.

/* Support for Intel Correct Machine Check Interrupts. */

static DEFINE_PER_CPU!(mce_banks_t, mce_banks_owned);
static DEFINE_RAW_SPINLOCK!(cmci_discover_lock);
static DEFINE_SPINLOCK!(cmci_poll_lock);

const CMCI_THRESHOLD: u16 = 1;
static mut cmci_threshold: [u16; MAX_NR_BANKS] = [0; MAX_NR_BANKS];
const CMCI_STORM_THRESHOLD: u64 = 32749;

unsafe fn cmci_supported(banks: *mut i32) -> bool {
    let mut cap: u64 = 0;
    if mca_cfg.cmci_disabled || mca_cfg.ignore_ce { return false; }
    if boot_cpu_data.x86_vendor != X86_VENDOR_INTEL && boot_cpu_data.x86_vendor != X86_VENDOR_ZHAOXIN { return false; }
    if !boot_cpu_has(X86_FEATURE_APIC) || lapic_get_maxlvt() < 6 { return false; }
    rdmsrq(MSR_IA32_MCG_CAP, &mut cap);
    *banks = core::cmp::min(MAX_NR_BANKS as u64, cap & MCG_BANKCNT_MASK) as i32;
    (cap & MCG_CMCI_P) != 0
}

unsafe fn lmce_supported() -> bool {
    let mut tmp: u64 = 0;
    if mca_cfg.lmce_disabled { return false; }
    rdmsrq(MSR_IA32_MCG_CAP, &mut tmp);
    if (tmp & (MCG_SER_P | MCG_LMCE_P)) != (MCG_SER_P | MCG_LMCE_P) { return false; }
    rdmsrq(MSR_IA32_FEAT_CTL, &mut tmp);
    if WARN_ON_ONCE((tmp & FEAT_CTL_LOCKED) == 0) { return false; }
    (tmp & FEAT_CTL_LMCE_ENABLED) != 0
}

unsafe fn cmci_set_threshold(bank: i32, thresh: u64) {
    let mut flags = 0UL;
    let mut val = 0u64;
    raw_spin_lock_irqsave(&mut cmci_discover_lock, &mut flags);
    rdmsrq(MSR_IA32_MCx_CTL2(bank), &mut val);
    val &= !MCI_CTL2_CMCI_THRESHOLD_MASK;
    wrmsrq(MSR_IA32_MCx_CTL2(bank), val | thresh);
    raw_spin_unlock_irqrestore(&mut cmci_discover_lock, flags);
}

pub unsafe fn mce_intel_handle_storm(bank: i32, on: bool) {
    if on { cmci_set_threshold(bank, CMCI_STORM_THRESHOLD); }
    else { cmci_set_threshold(bank, cmci_threshold[bank as usize] as u64); }
}

unsafe fn intel_threshold_interrupt() { machine_check_poll(MCP_TIMESTAMP, this_cpu_ptr(&mut mce_banks_owned)); }

unsafe fn cmci_skip_bank(bank: i32, val: *mut u64) -> bool {
    let owned = this_cpu_ptr(&mut mce_banks_owned) as *mut unsigned_long;
    if test_bit(bank, owned) || test_bit(bank, &mce_banks_ce_disabled) { return true; }
    rdmsrq(MSR_IA32_MCx_CTL2(bank), val);
    if (*val & MCI_CTL2_CMCI_EN) != 0 {
        clear_bit(bank, owned);
        __clear_bit(bank, this_cpu_ptr(&mut mce_poll_banks));
        return true;
    }
    false
}

unsafe fn cmci_pick_threshold(mut val: u64, bios_zero_thresh: *mut i32) -> u64 {
    if (val & MCI_CTL2_CMCI_THRESHOLD_MASK) == CMCI_STORM_THRESHOLD { return val; }
    if !mca_cfg.bios_cmci_threshold {
        val = (val & !MCI_CTL2_CMCI_THRESHOLD_MASK) | CMCI_THRESHOLD as u64;
    } else if (val & MCI_CTL2_CMCI_THRESHOLD_MASK) == 0 {
        *bios_zero_thresh = 1;
        val |= CMCI_THRESHOLD as u64;
    }
    val
}

unsafe fn cmci_claim_bank(bank: i32, mut val: u64, bios_zero_thresh: i32, bios_wrong_thresh: *mut i32) {
    let storm = this_cpu_ptr(&mut storm_desc);
    val |= MCI_CTL2_CMCI_EN;
    wrmsrq(MSR_IA32_MCx_CTL2(bank), val);
    rdmsrq(MSR_IA32_MCx_CTL2(bank), &mut val);
    if (val & MCI_CTL2_CMCI_EN) == 0 {
        WARN_ON(!test_bit(bank, this_cpu_ptr(&mut mce_poll_banks)));
        (*storm).banks[bank as usize].poll_only = true;
        return;
    }
    set_bit(bank, this_cpu_ptr(&mut mce_banks_owned) as *mut unsigned_long);
    if (val & MCI_CTL2_CMCI_THRESHOLD_MASK) == CMCI_STORM_THRESHOLD {
        pr_notice!("CPU{} BANK{} CMCI inherited storm\n", smp_processor_id(), bank);
        mce_inherit_storm(bank); cmci_storm_begin(bank);
    } else { __clear_bit(bank, this_cpu_ptr(&mut mce_poll_banks)); }
    if mca_cfg.bios_cmci_threshold && bios_zero_thresh != 0 && (val & MCI_CTL2_CMCI_THRESHOLD_MASK) != 0 { *bios_wrong_thresh = 1; }
    if cmci_threshold[bank as usize] == 0 { cmci_threshold[bank as usize] = (val & MCI_CTL2_CMCI_THRESHOLD_MASK) as u16; }
}

unsafe fn cmci_discover(banks: i32) {
    let mut bios_wrong_thresh = 0;
    let mut flags = 0UL;
    raw_spin_lock_irqsave(&mut cmci_discover_lock, &mut flags);
    for i in 0..banks {
        let mut val = 0u64; let mut bios_zero_thresh = 0;
        if cmci_skip_bank(i, &mut val) { continue; }
        val = cmci_pick_threshold(val, &mut bios_zero_thresh);
        cmci_claim_bank(i, val, bios_zero_thresh, &mut bios_wrong_thresh);
    }
    raw_spin_unlock_irqrestore(&mut cmci_discover_lock, flags);
    if mca_cfg.bios_cmci_threshold && bios_wrong_thresh != 0 {
        pr_info_once!("bios_cmci_threshold: Some banks do not have valid thresholds set\n");
        pr_info_once!("bios_cmci_threshold: Make sure your BIOS supports this boot option\n");
    }
}

pub unsafe fn cmci_recheck() {
    let mut banks = 0; let mut flags = 0UL;
    if !mce_available(raw_cpu_ptr(&mut cpu_info)) || !cmci_supported(&mut banks) { return; }
    local_irq_save(&mut flags); machine_check_poll(0, this_cpu_ptr(&mut mce_banks_owned)); local_irq_restore(flags);
}

unsafe fn __cmci_disable_bank(bank: i32) {
    let mut val = 0u64;
    if !test_bit(bank, this_cpu_ptr(&mut mce_banks_owned)) { return; }
    rdmsrq(MSR_IA32_MCx_CTL2(bank), &mut val); val &= !MCI_CTL2_CMCI_EN; wrmsrq(MSR_IA32_MCx_CTL2(bank), val);
    __clear_bit(bank, this_cpu_ptr(&mut mce_banks_owned));
    if (val & MCI_CTL2_CMCI_THRESHOLD_MASK) == CMCI_STORM_THRESHOLD { cmci_storm_end(bank); }
}

pub unsafe fn cmci_clear() {
    let mut flags = 0UL; let mut banks = 0;
    if !cmci_supported(&mut banks) { return; }
    raw_spin_lock_irqsave(&mut cmci_discover_lock, &mut flags); for i in 0..banks { __cmci_disable_bank(i); } raw_spin_unlock_irqrestore(&mut cmci_discover_lock, flags);
}

unsafe fn cmci_rediscover_work_func(_arg: *mut core::ffi::c_void) { let mut banks = 0; if cmci_supported(&mut banks) { cmci_discover(banks); } }
pub unsafe fn cmci_rediscover() { let mut banks = 0; if cmci_supported(&mut banks) { on_each_cpu(cmci_rediscover_work_func, core::ptr::null_mut(), 1); } }
pub unsafe fn cmci_reenable() { let mut banks = 0; if cmci_supported(&mut banks) { cmci_discover(banks); } }
pub unsafe fn cmci_disable_bank(bank: i32) { let mut banks = 0; let mut flags = 0UL; if !cmci_supported(&mut banks) { return; } raw_spin_lock_irqsave(&mut cmci_discover_lock, &mut flags); __cmci_disable_bank(bank); raw_spin_unlock_irqrestore(&mut cmci_discover_lock, flags); }

unsafe fn cmci_mc_poll_banks() { spin_lock(&mut cmci_poll_lock); machine_check_poll(0, this_cpu_ptr(&mut mce_poll_banks)); spin_unlock(&mut cmci_poll_lock); }

pub unsafe fn intel_init_cmci() {
    let mut banks = 0;
    if !cmci_supported(&mut banks) { mc_poll_banks = Some(cmci_mc_poll_banks); return; }
    mce_threshold_vector = Some(intel_threshold_interrupt); cmci_discover(banks);
    apic_write(APIC_LVTCMCI, THRESHOLD_APIC_VECTOR | APIC_DM_FIXED); cmci_recheck();
}

pub unsafe fn intel_init_lmce() { let mut val = 0u64; if !lmce_supported() { return; } rdmsrq(MSR_IA32_MCG_EXT_CTL, &mut val); if (val & MCG_EXT_CTL_LMCE_EN) == 0 { wrmsrq(MSR_IA32_MCG_EXT_CTL, val | MCG_EXT_CTL_LMCE_EN); } }
pub unsafe fn intel_clear_lmce() { let mut val = 0u64; if !lmce_supported() { return; } rdmsrq(MSR_IA32_MCG_EXT_CTL, &mut val); wrmsrq(MSR_IA32_MCG_EXT_CTL, val & !MCG_EXT_CTL_LMCE_EN); }

unsafe fn intel_imc_init(c: *mut cpuinfo_x86) {
    let mut error_control = 0u64;
    match (*c).x86_vfm { INTEL_SANDYBRIDGE_X | INTEL_IVYBRIDGE_X | INTEL_HASWELL_X => { if rdmsrq_safe(MSR_ERROR_CONTROL, &mut error_control) != 0 { return; } error_control |= 2; wrmsrq_safe(MSR_ERROR_CONTROL, error_control); }, _ => {} }
}

unsafe fn intel_apply_cpu_quirks(c: *mut cpuinfo_x86) { if (*c).x86_vfm < INTEL_NEHALEM_EP && this_cpu_read(mce_num_banks) != 0 { (*this_cpu_ptr(mce_banks_array)).init = false; } }
pub unsafe fn mce_intel_feature_init(c: *mut cpuinfo_x86) { intel_apply_cpu_quirks(c); intel_init_cmci(); intel_init_lmce(); intel_imc_init(c); }
pub unsafe fn mce_intel_feature_clear(_c: *mut cpuinfo_x86) { intel_clear_lmce(); cmci_clear(); }

pub unsafe fn intel_filter_mce(m: *mut mce) -> bool {
    let c = &boot_cpu_data;
    if (c.x86_vfm == INTEL_HASWELL || c.x86_vfm == INTEL_HASWELL_L || c.x86_vfm == INTEL_BROADWELL || c.x86_vfm == INTEL_HASWELL_G || c.x86_vfm == INTEL_SKYLAKE_X) && (*m).bank == 0 && ((*m).status & 0xa0000000ffffffff) == 0x80000000000f0005 { return true; }
    false
}

pub unsafe fn intel_mce_usable_address(m: *mut mce) -> bool {
    if (*m).status & MCI_STATUS_MISCV == 0 { return false; }
    if MCI_MISC_ADDR_LSB((*m).misc) > PAGE_SHIFT { return false; }
    if MCI_MISC_ADDR_MODE((*m).misc) != MCI_MISC_ADDR_PHYS { return false; }
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
