// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Based on MPC8560 ADS and arch/ppc tqm85xx ports
 *
 * Maintained by Kumar Gala (see MAINTAINERS for contact information)
 *
 * Copyright 2008 Freescale Semiconductor Inc.
 *
 * Copyright (c) 2005-2006 DENX Software Engineering
 * Stefan Roese <sr@denx.de>
 *
 * Based on original work by
 * 	Kumar Gala <kumar.gala@freescale.com>
 *      Copyright 2004 Freescale Semiconductor Inc.
 */

// C dependencies supplied by other translation units:
// linux/stddef.h, linux/kernel.h, linux/pci.h, linux/kdev_t.h,
// linux/delay.h, linux/seq_file.h, linux/of.h, asm/time.h,
// asm/machdep.h, asm/pci-bridge.h, asm/mpic.h, mm/mmu_decl.h,
// asm/udbg.h, sysdev/fsl_soc.h, sysdev/fsl_pci.h, and mpc85xx.h.

#[allow(non_camel_case_types)]
type uint = u32;

#[repr(C)]
pub struct mpic {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

extern "C" {
    static mut ppc_md: ppc_machine_desc;
    fn mpic_alloc(
        node: *mut core::ffi::c_void,
        flags: u32,
        senses: u32,
        isu_size: u32,
        irq_count: u32,
        name: *const core::ffi::c_char,
    ) -> *mut mpic;
    fn mpic_init(mpic: *mut mpic);
    fn mpc85xx_cpm2_pic_init();
    fn cpm2_reset();
    fn fsl_pci_assign_primary();
    fn mfspr(spr: u32) -> u32;
    fn seq_printf(m: *mut seq_file, format: *const core::ffi::c_char, ...);
    fn machine_is(machine: *const core::ffi::c_char) -> bool;
    fn dev_info(dev: *mut device, format: *const core::ffi::c_char, ...);
    fn pci_read_config_dword(dev: *mut pci_dev, where_: u32, val: *mut u32);
    fn pci_write_config_dword(dev: *mut pci_dev, where_: u32, val: u32);
    fn mpic_get_irq() -> u32;
    fn udbg_progress(message: *const core::ffi::c_char, hex: u32);
}

#[repr(C)]
pub struct ppc_machine_desc {
    pub progress: Option<unsafe extern "C" fn(*const core::ffi::c_char, u32)>,
}

const MPIC_BIG_ENDIAN: u32 = 1;
const SPRN_PVR: u32 = 287;
const SPRN_SVR: u32 = 26;
const SPRN_HID1: u32 = 1009;
const PCI_VENDOR_ID_TI: u32 = 0x104c;
const PCI_DEVICE_ID_TI_1520: u32 = 0xac50;

static BOARD: [*const core::ffi::c_char; 6] = [
    b"tqc,tqm8540\0".as_ptr() as *const core::ffi::c_char,
    b"tqc,tqm8541\0".as_ptr() as *const core::ffi::c_char,
    b"tqc,tqm8548\0".as_ptr() as *const core::ffi::c_char,
    b"tqc,tqm8555\0".as_ptr() as *const core::ffi::c_char,
    b"tqc,tqm8560\0".as_ptr() as *const core::ffi::c_char,
    core::ptr::null(),
];

unsafe fn tqm85xx_pic_init() {
    let mpic = mpic_alloc(
        core::ptr::null_mut(),
        0,
        MPIC_BIG_ENDIAN,
        0,
        256,
        b" OpenPIC  \0".as_ptr() as *const core::ffi::c_char,
    );
    assert!(!mpic.is_null());
    mpic_init(mpic);

    mpc85xx_cpm2_pic_init();
}

/*
 * Setup the architecture
 */
unsafe fn tqm85xx_setup_arch() {
    if let Some(progress) = ppc_md.progress {
        progress(
            b"tqm85xx_setup_arch()\0".as_ptr() as *const core::ffi::c_char,
            0,
        );
    }

    // #ifdef CONFIG_CPM2
    cpm2_reset();
    // #endif

    fsl_pci_assign_primary();
}

unsafe fn tqm85xx_show_cpuinfo(m: *mut seq_file) {
    let pvid: uint;
    let svid: uint;
    let phid1: uint;

    pvid = mfspr(SPRN_PVR);
    svid = mfspr(SPRN_SVR);

    seq_printf(m, b"Vendor\t\t: TQ Components\n\0".as_ptr() as *const _,);
    seq_printf(m, b"PVR\t\t: 0x%x\n\0".as_ptr() as *const _, pvid);
    seq_printf(m, b"SVR\t\t: 0x%x\n\0".as_ptr() as *const _, svid);

    /* Display cpu Pll setting */
    phid1 = mfspr(SPRN_HID1);
    seq_printf(
        m,
        b"PLL setting\t: 0x%x\n\0".as_ptr() as *const _,
        (phid1 >> 24) & 0x3f,
    );
}

unsafe fn tqm85xx_ti1520_fixup(pdev: *mut pci_dev) {
    let mut val: u32 = 0;

    /* Do not do the fixup on other platforms! */
    if !machine_is(b"tqm85xx\0".as_ptr() as *const _) {
        return;
    }

    dev_info(
        &mut (*pdev).dev,
        b"Using TI 1520 fixup on TQM85xx\n\0".as_ptr() as *const _,
    );

    /*
     * Enable P2CCLK bit in system control register
     * to enable CLOCK output to power chip
     */
    pci_read_config_dword(pdev, 0x80, &mut val);
    pci_write_config_dword(pdev, 0x80, val | (1 << 27));
}

// DECLARE_PCI_FIXUP_HEADER(PCI_VENDOR_ID_TI, PCI_DEVICE_ID_TI_1520,
//     tqm85xx_ti1520_fixup);

// machine_arch_initcall(tqm85xx, mpc85xx_common_publish_devices);

// The original __initconst board array contains these compatible strings:
// "tqc,tqm8540", "tqc,tqm8541", "tqc,tqm8548", "tqc,tqm8555",
// "tqc,tqm8560", NULL.

// define_machine(tqm85xx) {
//     .name = "TQM85xx",
//     .compatibles = board,
//     .setup_arch = tqm85xx_setup_arch,
//     .init_IRQ = tqm85xx_pic_init,
//     .show_cpuinfo = tqm85xx_show_cpuinfo,
//     .get_irq = mpic_get_irq,
//     .progress = udbg_progress,
// };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
