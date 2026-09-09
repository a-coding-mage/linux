/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2016,2017 IBM Corporation.
 */

// Dependency supplied by the OPAL API header: OPAL_XIVE_ANY_CHIP.
pub const XIVE_INVALID_VP: u32 = 0xffff_ffff;

// CONFIG_PPC_XIVE controls whether the XIVE declarations and implementations
// below are available.
#[cfg(CONFIG_PPC_XIVE)]
mod xive {
    use core::ffi::c_void;

    extern "C" {
        pub static mut xive_tima: *mut c_void;
        pub static mut xive_tima_os: ::core::ffi::c_ulong;
        pub static mut xive_tima_offset: u32;
        pub static mut __xive_enabled: bool;
    }

    #[repr(C)]
    pub struct xive_irq_data {
        pub flags: u64,
        pub eoi_page: u64,
        pub eoi_mmio: *mut c_void,
        pub trig_page: u64,
        pub trig_mmio: *mut c_void,
        pub esb_shift: u32,
        pub src_chip: i32,
        pub hw_irq: u32,
        pub target: i32,
        pub saved_p: bool,
        pub stale_p: bool,
    }

    pub const XIVE_IRQ_FLAG_STORE_EOI: u64 = 0x01;
    pub const XIVE_IRQ_FLAG_LSI: u64 = 0x02;
    // XIVE_IRQ_FLAG_SHIFT_BUG and XIVE_IRQ_FLAG_MASK_FW are P9 DD1.0 workarounds.
    // XIVE_IRQ_FLAG_EOI_FW is a P9 DD1.0 workaround.
    pub const XIVE_IRQ_FLAG_H_INT_ESB: u64 = 0x20;
    /* Special flag set by KVM for excalation interrupts */
    pub const XIVE_IRQ_FLAG_NO_EOI: u64 = 0x80;
    pub const XIVE_INVALID_CHIP_ID: i32 = -1;

    #[repr(C)]
    pub struct xive_q {
        pub qpage: *mut __be32,
        pub msk: u32,
        pub idx: u32,
        pub toggle: u32,
        pub eoi_phys: u64,
        pub esc_irq: u32,
        pub count: atomic_t,
        pub pending_count: atomic_t,
        pub guest_qaddr: u64,
        pub guest_qshift: u32,
    }

    #[inline]
    pub unsafe fn xive_enabled() -> bool { __xive_enabled }

    extern "C" {
        pub fn xive_spapr_init() -> bool;
        pub fn xive_native_init() -> bool;
        pub fn xive_smp_probe() -> i32;
        pub fn xive_smp_prepare_cpu(cpu: ::core::ffi::c_uint) -> i32;
        pub fn xive_smp_setup_cpu();
        pub fn xive_smp_disable_cpu();
        pub fn xive_teardown_cpu();
        pub fn xive_shutdown();
        pub fn xive_flush_interrupt();
        pub fn xmon_xive_do_dump(cpu: i32);
        pub fn xmon_xive_get_irq_config(hw_irq: u32, d: *mut irq_data) -> i32;
        pub fn xmon_xive_get_irq_all();
        pub fn xive_native_default_eq_shift() -> u32;
        pub fn xive_native_alloc_vp_block(max_vcpus: u32) -> u32;
        pub fn xive_native_free_vp_block(vp_base: u32);
        pub fn xive_native_populate_irq_data(hw_irq: u32, data: *mut xive_irq_data) -> i32;
        pub fn xive_cleanup_irq_data(xd: *mut xive_irq_data);
        pub fn xive_native_free_irq(irq: u32);
        pub fn xive_native_configure_irq(hw_irq: u32, target: u32, prio: u8, sw_irq: u32) -> i32;
        pub fn xive_native_configure_queue(vp_id: u32, q: *mut xive_q, prio: u8,
                                           qpage: *mut __be32, order: u32,
                                           can_escalate: bool) -> i32;
        pub fn xive_native_disable_queue(vp_id: u32, q: *mut xive_q, prio: u8);
        pub fn xive_native_sync_source(hw_irq: u32);
        pub fn xive_native_sync_queue(hw_irq: u32);
        pub fn is_xive_irq(chip: *mut irq_chip) -> bool;
        pub fn xive_native_enable_vp(vp_id: u32, single_escalation: bool) -> i32;
        pub fn xive_native_disable_vp(vp_id: u32) -> i32;
        pub fn xive_native_get_vp_info(vp_id: u32, out_cam_id: *mut u32, out_chip_id: *mut u32) -> i32;
        pub fn xive_native_has_single_escalation() -> bool;
        pub fn xive_native_has_save_restore() -> bool;
        pub fn xive_native_get_queue_info(vp_id: u32, prio: u32, out_qpage: *mut u64,
                                          out_qsize: *mut u64, out_qeoi_page: *mut u64,
                                          out_escalate_irq: *mut u32, out_qflags: *mut u64) -> i32;
        pub fn xive_native_get_queue_state(vp_id: u32, prio: u32, qtoggle: *mut u32,
                                           qindex: *mut u32) -> i32;
        pub fn xive_native_set_queue_state(vp_id: u32, prio: u32, qtoggle: u32, qindex: u32) -> i32;
        pub fn xive_native_get_vp_state(vp_id: u32, out_state: *mut u64) -> i32;
        pub fn xive_native_has_queue_state_support() -> bool;
        pub fn xive_native_alloc_irq_on_chip(chip_id: u32) -> u32;
    }

    #[inline]
    pub unsafe fn xive_native_alloc_irq() -> u32 {
        xive_native_alloc_irq_on_chip(OPAL_XIVE_ANY_CHIP)
    }
}

#[cfg(not(CONFIG_PPC_XIVE))]
#[inline]
pub const fn xive_enabled() -> bool { false }

#[cfg(not(CONFIG_PPC_XIVE))]
#[inline]
pub const fn xive_spapr_init() -> bool { false }

#[cfg(not(CONFIG_PPC_XIVE))]
#[inline]
pub const fn xive_native_init() -> bool { false }

#[cfg(not(CONFIG_PPC_XIVE))]
#[inline]
pub const fn xive_smp_probe() -> i32 { -22 /* -EINVAL */ }

#[cfg(not(CONFIG_PPC_XIVE))]
#[inline]
pub const fn xive_smp_prepare_cpu(_cpu: ::core::ffi::c_uint) -> i32 { -22 /* -EINVAL */ }

#[cfg(not(CONFIG_PPC_XIVE))]
#[inline]
pub fn xive_smp_setup_cpu() {}

#[cfg(not(CONFIG_PPC_XIVE))]
#[inline]
pub fn xive_smp_disable_cpu() {}

#[cfg(not(CONFIG_PPC_XIVE))]
#[inline]
pub fn xive_shutdown() {}

#[cfg(not(CONFIG_PPC_XIVE))]
#[inline]
pub fn xive_flush_interrupt() {}

#[cfg(not(CONFIG_PPC_XIVE))]
#[inline]
pub const fn xive_native_alloc_vp_block(_max_vcpus: u32) -> u32 { XIVE_INVALID_VP }

#[cfg(not(CONFIG_PPC_XIVE))]
#[inline]
pub fn xive_native_free_vp_block(_vp_base: u32) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
