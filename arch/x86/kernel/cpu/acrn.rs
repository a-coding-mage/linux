// SPDX-License-Identifier: GPL-2.0
/*
 * ACRN detection support
 *
 * Copyright (C) 2019 Intel Corporation. All rights reserved.
 *
 * Jason Chen CJ <jason.cj.chen@intel.com>
 * Zhao Yakui <yakui.zhao@intel.com>
 *
 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/interrupt.h, asm/acrn.h, asm/apic.h, asm/cpufeatures.h,
// asm/desc.h, asm/hypervisor.h, asm/idtentry.h, and asm/irq_regs.h.

unsafe extern "C" {
    fn acrn_cpuid_base() -> u32;
    fn sysvec_install(vector: u32, handler: unsafe extern "C" fn(*mut pt_regs));
    fn acrn_get_tsc_khz() -> u32;
    fn boot_cpu_has(feature: u32) -> bool;
    fn apic_eoi();
    fn inc_irq_stat(stat: u32);
    fn set_irq_regs(regs: *mut pt_regs) -> *mut pt_regs;
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    static mut x86_platform: x86_platform;
}

#[repr(C)]
pub struct x86_platform {
    pub calibrate_tsc: Option<unsafe extern "C" fn() -> u32>,
    pub calibrate_cpu: Option<unsafe extern "C" fn() -> u32>,
}

static mut acrn_intr_handler: Option<unsafe extern "C" fn()> = None;

unsafe fn acrn_detect() -> u32 {
    acrn_cpuid_base()
}

unsafe fn acrn_init_platform() {
    /* Install system interrupt handler for ACRN hypervisor callback */
    sysvec_install(HYPERVISOR_CALLBACK_VECTOR, sysvec_acrn_hv_callback);

    x86_platform.calibrate_tsc = Some(acrn_get_tsc_khz);
    x86_platform.calibrate_cpu = Some(acrn_get_tsc_khz);
}

unsafe fn acrn_x2apic_available() -> bool {
    boot_cpu_has(X86_FEATURE_X2APIC)
}

pub unsafe extern "C" fn sysvec_acrn_hv_callback(regs: *mut pt_regs) {
    let old_regs = set_irq_regs(regs);

    /*
     * The hypervisor requires that the APIC EOI should be acked.
     * If the APIC EOI is not acked, the APIC ISR bit for the
     * HYPERVISOR_CALLBACK_VECTOR will not be cleared and then it
     * will block the interrupt whose vector is lower than
     * HYPERVISOR_CALLBACK_VECTOR.
     */
    apic_eoi();
    inc_irq_stat(HYPERVISOR_CALLBACK);

    if let Some(handler) = acrn_intr_handler {
        handler();
    }

    set_irq_regs(old_regs);
}

pub unsafe extern "C" fn acrn_setup_intr_handler(
    handler: Option<unsafe extern "C" fn()>,
) {
    acrn_intr_handler = handler;
}

pub unsafe extern "C" fn acrn_remove_intr_handler() {
    acrn_intr_handler = None;
}

#[repr(C)]
pub struct hypervisor_x86 {
    pub name: *const u8,
    pub detect: Option<unsafe fn() -> u32>,
    pub type_: u32,
    pub init: hypervisor_x86_init,
}

#[repr(C)]
pub struct hypervisor_x86_init {
    pub init_platform: Option<unsafe fn()>,
    pub x2apic_available: Option<unsafe fn() -> bool>,
}

#[no_mangle]
pub static x86_hyper_acrn: hypervisor_x86 = hypervisor_x86 {
    name: b"ACRN\0".as_ptr(),
    detect: Some(acrn_detect),
    type_: X86_HYPER_ACRN,
    init: hypervisor_x86_init {
        init_platform: Some(acrn_init_platform),
        x2apic_available: Some(acrn_x2apic_available),
    },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
