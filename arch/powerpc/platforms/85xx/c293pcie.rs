// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * C293PCIE Board Setup
 *
 * Copyright 2013 Freescale Semiconductor Inc.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/stddef.h, linux/kernel.h, linux/of.h
// asm/machdep.h, asm/udbg.h, asm/mpic.h
// sysdev/fsl_soc.h, sysdev/fsl_pci.h, and "mpc85xx.h"

extern "C" {
    static mut ppc_md: PpcMd;

    fn mpic_alloc(
        node: *mut core::ffi::c_void,
        flags: i32,
        flags2: u32,
        isu_size: i32,
        irq_count: i32,
        name: *const core::ffi::c_char,
    ) -> *mut Mpic;
    fn mpic_init(mpic: *mut Mpic);
    fn fsl_pci_assign_primary();
    fn printk(format: *const core::ffi::c_char, ...);
    fn mpc85xx_common_publish_devices() -> i32;
    fn mpic_get_irq() -> i32;
    fn udbg_progress(message: *const core::ffi::c_char, value: u16);
}

#[repr(C)]
struct Mpic;

#[repr(C)]
struct PpcMd {
    progress: Option<unsafe extern "C" fn(*const core::ffi::c_char, u16)>,
}

const MPIC_BIG_ENDIAN: u32 = 0;
const MPIC_SINGLE_DEST_CPU: u32 = 0;

// The original __init annotation is retained as a source-level marker.
unsafe fn c293_pcie_pic_init() {
    let mpic = mpic_alloc(
        core::ptr::null_mut(),
        0,
        MPIC_BIG_ENDIAN | MPIC_SINGLE_DEST_CPU,
        0,
        256,
        b" OpenPIC  \0".as_ptr() as *const core::ffi::c_char,
    );

    assert!(!mpic.is_null());

    mpic_init(mpic);
}

/*
 * Setup the architecture
 */
unsafe fn c293_pcie_setup_arch() {
    if let Some(progress) = ppc_md.progress {
        progress(
            b"c293_pcie_setup_arch()\0".as_ptr() as *const core::ffi::c_char,
            0,
        );
    }

    fsl_pci_assign_primary();

    printk(
        b"C293 PCIE board from Freescale Semiconductor\n\0".as_ptr()
            as *const core::ffi::c_char,
    );
}

// Equivalent of machine_arch_initcall(c293_pcie, mpc85xx_common_publish_devices).
#[allow(dead_code)]
static C293_PCIE_ARCH_INITCALL: unsafe extern "C" fn() -> i32 = mpc85xx_common_publish_devices;

#[repr(C)]
struct MachineDesc {
    name: *const core::ffi::c_char,
    compatible: *const core::ffi::c_char,
    setup_arch: unsafe fn(),
    init_irq: unsafe fn(),
    get_irq: unsafe extern "C" fn() -> i32,
    progress: unsafe extern "C" fn(*const core::ffi::c_char, u16),
}

// Equivalent of define_machine(c293_pcie).
#[allow(dead_code)]
static C293_PCIE: MachineDesc = MachineDesc {
    name: b"C293 PCIE\0".as_ptr() as *const core::ffi::c_char,
    compatible: b"fsl,C293PCIE\0".as_ptr() as *const core::ffi::c_char,
    setup_arch: c293_pcie_setup_arch,
    init_irq: c293_pcie_pic_init,
    get_irq: mpic_get_irq,
    progress: udbg_progress,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
