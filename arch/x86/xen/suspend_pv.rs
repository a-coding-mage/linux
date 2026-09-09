// SPDX-License-Identifier: GPL-2.0
// External declarations correspond to the Linux/Xen headers included by the C source.

use core::ffi::c_void;

#[repr(C)]
pub struct XenConsoleDomU {
    pub mfn: u64,
}

#[repr(C)]
pub struct XenConsole {
    pub domU: XenConsoleDomU,
}

#[repr(C)]
pub struct XenStartInfo {
    pub store_mfn: u64,
    pub console: XenConsole,
    pub shared_info: u64,
}

extern "C" {
    static mut xen_start_info: *mut XenStartInfo;
    static mut HYPERVISOR_shared_info: *mut c_void;
    static mut xen_dummy_shared_info: c_void;
    static mut xen_cpu_initialized_map: *mut c_void;
    static mut cpu_online_mask: *const c_void;

    fn xen_mm_pin_all();
    fn xen_build_mfn_list_list();
    fn xen_setup_mfn_list_list();
    fn xen_vcpu_restore();
    fn xen_mm_unpin_all();
    fn mfn_to_pfn(mfn: u64) -> u64;
    fn pfn_to_mfn(pfn: u64) -> u64;
    fn fix_to_virt(fix: i32) -> usize;
    fn set_fixmap(fix: i32, phys: u64);
    fn HYPERVISOR_update_va_mapping(va: usize, pte: usize, flags: u64) -> i32;
    fn cpumask_copy(dst: *mut c_void, src: *const c_void);
    fn irqs_disabled() -> bool;
}

pub const FIX_PARAVIRT_BOOTMAP: i32 = 0; // Value supplied by asm/fixmap.h.

#[inline]
unsafe fn __pte_ma(_value: usize) -> usize {
    0
}

pub unsafe fn xen_pv_pre_suspend() {
    xen_mm_pin_all();

    (*xen_start_info).store_mfn = mfn_to_pfn((*xen_start_info).store_mfn);
    (*xen_start_info).console.domU.mfn =
        mfn_to_pfn((*xen_start_info).console.domU.mfn);

    assert!(irqs_disabled()); // BUG_ON(!irqs_disabled())

    HYPERVISOR_shared_info = &raw mut xen_dummy_shared_info as *mut c_void;
    if HYPERVISOR_update_va_mapping(
        fix_to_virt(FIX_PARAVIRT_BOOTMAP),
        __pte_ma(0),
        0,
    ) != 0 {
        panic!("BUG");
    }
}

pub unsafe fn xen_pv_post_suspend(suspend_cancelled: i32) {
    xen_build_mfn_list_list();
    set_fixmap(FIX_PARAVIRT_BOOTMAP, (*xen_start_info).shared_info);
    HYPERVISOR_shared_info = fix_to_virt(FIX_PARAVIRT_BOOTMAP) as *mut c_void;
    xen_setup_mfn_list_list();

    if suspend_cancelled != 0 {
        (*xen_start_info).store_mfn = pfn_to_mfn((*xen_start_info).store_mfn);
        (*xen_start_info).console.domU.mfn =
            pfn_to_mfn((*xen_start_info).console.domU.mfn);
    } else {
        // CONFIG_SMP conditional from the C build configuration.
        #[cfg(feature = "CONFIG_SMP")]
        {
            assert!(!xen_cpu_initialized_map.is_null());
            cpumask_copy(xen_cpu_initialized_map, cpu_online_mask);
        }
        xen_vcpu_restore();
    }

    xen_mm_unpin_all();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
