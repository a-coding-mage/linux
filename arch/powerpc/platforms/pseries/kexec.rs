// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright 2006 Michael Ellerman, IBM Corporation
 */

// Dependencies supplied by the surrounding kernel translation unit.

use core::ffi::c_int;

#[repr(C)]
pub struct Lppaca {
    pub dtl_enable_mask: u8,
}

extern "C" {
    pub fn firmware_has_feature(feature: c_int) -> bool;
    pub fn smp_processor_id() -> c_int;
    pub fn hard_smp_processor_id() -> c_int;
    pub fn get_lppaca() -> *mut Lppaca;
    pub fn unregister_dtl(hwcpu: c_int) -> c_int;
    pub fn unregister_slb_shadow(hwcpu: c_int) -> c_int;
    pub fn unregister_vpa(hwcpu: c_int) -> c_int;
    pub fn xive_enabled() -> bool;
    pub fn xive_teardown_cpu();
    pub fn xive_shutdown();
    pub fn xics_kexec_teardown_cpu(secondary: c_int);
    pub fn pr_err(fmt: *const u8, ...);
}

pub const FW_FEATURE_SPLPAR: c_int = 0;

pub unsafe fn pseries_kexec_cpu_down(crash_shutdown: c_int, secondary: c_int) {
    /*
     * Ensure vpa/slb_shadow/dtl cleanup even while we are crashing.
     * Why? The hypervisor is not crashing so at least attempt unregister to
     * avoid the hypervisor stepping on our memory. If hypervisor or kexec
     * kernel steps on the old memory allocated to these areas before the
     * new kexec-kernel happens to allocate and register new areas,
     * the hypervisor will see invalid content which may cause
     * unexpected behavior.
     */
    if firmware_has_feature(FW_FEATURE_SPLPAR) {
        let mut ret: c_int;
        let cpu = smp_processor_id();
        let hwcpu = hard_smp_processor_id();

        if (*get_lppaca()).dtl_enable_mask != 0 {
            ret = unregister_dtl(hwcpu);
            if ret != 0 {
                pr_err(
                    b"WARNING: DTL deregistration for cpu %d (hw %d) failed with %d\n\0"
                        .as_ptr(),
                    cpu,
                    hwcpu,
                    ret,
                );
            }
        }

        ret = unregister_slb_shadow(hwcpu);
        if ret != 0 {
            pr_err(
                b"WARNING: SLB shadow buffer deregistration for cpu %d (hw %d) failed with %d\n\0"
                    .as_ptr(),
                cpu,
                hwcpu,
                ret,
            );
        }

        ret = unregister_vpa(hwcpu);
        if ret != 0 {
            pr_err(
                b"WARNING: VPA deregistration for cpu %d (hw %d) failed with %d\n\0"
                    .as_ptr(),
                cpu,
                hwcpu,
                ret,
            );
        }
    }

    if xive_enabled() {
        xive_teardown_cpu();

        if secondary == 0 {
            xive_shutdown();
        }
    } else {
        xics_kexec_teardown_cpu(secondary);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
