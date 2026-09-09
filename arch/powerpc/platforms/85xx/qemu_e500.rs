// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Paravirt target for a generic QEMU e500 machine
 *
 * This is intended to be a flexible device-tree-driven platform, not fixed
 * to a particular piece of hardware or a particular spec of virtual hardware,
 * beyond the assumption of an e500-family CPU.  Some things are still hardcoded
 * here, such as MPIC, but this is a limitation of the current code rather than
 * an interface contract with QEMU.
 *
 * Copyright 2012 Freescale Semiconductor Inc.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Declarations supplied by the surrounding kernel sources.
#[repr(C)]
pub struct mpic {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machdep_calls {
    pub progress: Option<unsafe extern "C" fn(*const c_char, c_int)>,
    pub setup_arch: Option<unsafe extern "C" fn()>,
    pub init_IRQ: Option<unsafe extern "C" fn()>,
    pub pcibios_fixup_bus: Option<unsafe extern "C" fn(*mut c_void)>,
    pub pcibios_fixup_phb: Option<unsafe extern "C" fn(*mut c_void)>,
    pub get_irq: Option<unsafe extern "C" fn() -> c_int>,
    pub power_save: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" {
    static mut ppc_md: machdep_calls;

    fn mpic_alloc(
        node: *mut c_void,
        flags: c_uint,
        flags2: c_uint,
        irq_offset: c_uint,
        irq_count: c_uint,
        name: *const c_char,
    ) -> *mut mpic;
    fn mpic_init(mpic: *mut mpic);
    fn fsl_pci_assign_primary();
    fn swiotlb_detect_4g();
    fn mpc85xx_smp_init();
    fn mpc85xx_common_publish_devices();
    fn fsl_pcibios_fixup_bus(bus: *mut c_void);
    fn fsl_pcibios_fixup_phb(phb: *mut c_void);
    fn mpic_get_coreint_irq() -> c_int;
    fn udbg_progress(message: *const c_char, value: c_int);
    fn e500_idle();
}

const MPIC_BIG_ENDIAN: c_uint = 1 << 0;
const MPIC_SINGLE_DEST_CPU: c_uint = 1 << 1;
const MPIC_ENABLE_COREINT: c_uint = 1 << 2;

unsafe fn qemu_e500_pic_init() {
    let mpic: *mut mpic;
    let flags: c_uint = MPIC_BIG_ENDIAN | MPIC_SINGLE_DEST_CPU | MPIC_ENABLE_COREINT;

    mpic = mpic_alloc(
        core::ptr::null_mut(),
        0,
        flags,
        0,
        256,
        b" OpenPIC  \0".as_ptr() as *const c_char,
    );

    if mpic.is_null() {
        // BUG_ON(mpic == NULL)
        core::intrinsics::abort();
    }
    mpic_init(mpic);
}

unsafe fn qemu_e500_setup_arch() {
    if let Some(progress) = ppc_md.progress {
        progress(b"qemu_e500_setup_arch()\0".as_ptr() as *const c_char, 0);
    }

    fsl_pci_assign_primary();
    swiotlb_detect_4g();
    mpc85xx_smp_init();
}

// machine_arch_initcall(qemu_e500, mpc85xx_common_publish_devices);
// The initcall registration is provided by the surrounding kernel build.

#[repr(C)]
pub struct machine_desc {
    pub name: *const c_char,
    pub compatible: *const c_char,
    pub setup_arch: Option<unsafe extern "C" fn()>,
    pub init_IRQ: Option<unsafe extern "C" fn()>,
    pub pcibios_fixup_bus: Option<unsafe extern "C" fn(*mut c_void)>,
    pub pcibios_fixup_phb: Option<unsafe extern "C" fn(*mut c_void)>,
    pub get_irq: Option<unsafe extern "C" fn() -> c_int>,
    pub progress: Option<unsafe extern "C" fn(*const c_char, c_int)>,
    pub power_save: Option<unsafe extern "C" fn()>,
}

#[no_mangle]
pub static qemu_e500: machine_desc = machine_desc {
    name: b"QEMU e500\0".as_ptr() as *const c_char,
    compatible: b"fsl,qemu-e500\0".as_ptr() as *const c_char,
    setup_arch: Some(qemu_e500_setup_arch),
    init_IRQ: Some(qemu_e500_pic_init),
    pcibios_fixup_bus: Some(fsl_pcibios_fixup_bus),
    pcibios_fixup_phb: Some(fsl_pcibios_fixup_phb),
    get_irq: Some(mpic_get_coreint_irq),
    progress: Some(udbg_progress),
    power_save: Some(e500_idle),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
