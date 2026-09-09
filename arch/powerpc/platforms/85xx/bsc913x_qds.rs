// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * BSC913xQDS Board Setup
 *
 * Author:
 *   Harninder Rai <harninder.rai@freescale.com>
 *   Priyanka Jain <Priyanka.Jain@freescale.com>
 *
 * Copyright 2014 Freescale Semiconductor Inc.
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

extern "C" {
    static mut ppc_md: PpcMd;
    fn mpic_alloc(
        node: *mut c_void,
        flags: i32,
        flags2: i32,
        first_irq: i32,
        nr_irqs: i32,
        name: *const core::ffi::c_char,
    ) -> *mut Mpic;
    fn mpic_init(mpic: *mut Mpic);
    fn mpc85xx_smp_init();
    fn fsl_pci_assign_primary();
    fn mpc85xx_common_publish_devices();
    fn fsl_pcibios_fixup_bus(bus: *mut c_void);
    fn mpic_get_irq(regs: *mut c_void) -> i32;
    fn udbg_progress(s: *const core::ffi::c_char, hex: u32);
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn pr_info(fmt: *const core::ffi::c_char, ...);
}

#[repr(C)]
pub struct Mpic {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PpcMd {
    pub progress: Option<unsafe extern "C" fn(*const core::ffi::c_char, u32)>,
}

pub const MPIC_BIG_ENDIAN: i32 = 1 << 0;
pub const MPIC_SINGLE_DEST_CPU: i32 = 1 << 1;

unsafe fn bsc913x_qds_pic_init() {
    let mpic = mpic_alloc(
        core::ptr::null_mut(),
        0,
        MPIC_BIG_ENDIAN | MPIC_SINGLE_DEST_CPU,
        0,
        256,
        b" OpenPIC  \0".as_ptr() as *const core::ffi::c_char,
    );

    if mpic.is_null() {
        pr_err(b"bsc913x: Failed to allocate MPIC structure\n\0".as_ptr() as *const core::ffi::c_char);
    } else {
        mpic_init(mpic);
    }
}

/*
 * Setup the architecture
 */
unsafe fn bsc913x_qds_setup_arch() {
    if let Some(progress) = ppc_md.progress {
        progress(
            b"bsc913x_qds_setup_arch()\0".as_ptr() as *const core::ffi::c_char,
            0,
        );
    }

    // CONFIG_SMP conditional from the original source.
    #[cfg(feature = "CONFIG_SMP")]
    mpc85xx_smp_init();

    fsl_pci_assign_primary();

    pr_info(
        b"bsc913x board from Freescale Semiconductor\n\0".as_ptr()
            as *const core::ffi::c_char,
    );
}

// machine_arch_initcall(bsc9132_qds, mpc85xx_common_publish_devices);
// define_machine(bsc9132_qds) {
//     .name = "BSC9132 QDS",
//     .compatible = "fsl,bsc9132qds",
//     .setup_arch = bsc913x_qds_setup_arch,
//     .init_IRQ = bsc913x_qds_pic_init,
//     .pcibios_fixup_bus = fsl_pcibios_fixup_bus, // CONFIG_PCI
//     .get_irq = mpic_get_irq,
//     .progress = udbg_progress,
// };


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
