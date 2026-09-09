// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * GE SBC610 board support
 *
 * Author: Martyn Welch <martyn.welch@ge.com>
 *
 * Copyright 2008 GE Intelligent Platforms Embedded Systems, Inc.
 *
 * Based on: mpc86xx_hpcn.c (MPC86xx HPCN board specific routines)
 * Copyright 2006 Freescale Semiconductor Inc.
 *
 * NEC fixup adapted from arch/mips/pci/fixup-lm2e.c
 */

use core::ffi::c_void;

// Kernel headers and build-time configuration supplied by the surrounding tree.

extern "C" {
    static mut sbc610_regs: *mut c_void;

    fn mpc86xx_init_irq();
    fn of_find_compatible_node(
        from: *mut device_node,
        type_: *const i8,
        compatible: *const i8,
    ) -> *mut device_node;
    fn printk(format: *const i8, ...);
    fn gef_pic_init(node: *mut device_node);
    fn of_node_put(node: *mut device_node);
    fn mpc86xx_smp_init();
    fn fsl_pci_assign_primary();
    fn of_iomap(node: *mut device_node, index: i32) -> *mut c_void;
    fn mmio_nvram_init();
    fn ioread32(addr: *mut c_void) -> u32;
    fn mfspr(spr: u32) -> u32;
    fn seq_printf(m: *mut seq_file, format: *const i8, ...);
    fn machine_is(machine: *const c_void) -> bool;
    fn pci_read_config_dword(pdev: *mut pci_dev, where_: u32, val: *mut u32) -> i32;
    fn pci_write_config_dword(pdev: *mut pci_dev, where_: u32, val: u32) -> i32;
    fn mpic_get_irq(regs: *mut c_void) -> i32;
    fn mpc86xx_time_init();
    fn udbg_progress(value: u32, regs: *mut c_void);
    fn fsl_pcibios_fixup_bus(bus: *mut c_void);
    static gef_sbc610: c_void;
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

const SPRN_SVR: u32 = 0x11e;

unsafe fn gef_sbc610_init_irq() {
    let mut cascade_node: *mut device_node = core::ptr::null_mut();

    mpc86xx_init_irq();

    /*
     * There is a simple interrupt handler in the main FPGA, this needs
     * to be cascaded into the MPIC
     */
    cascade_node = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"gef,fpga-pic\0".as_ptr() as *const i8,
    );
    if cascade_node.is_null() {
        printk(b"SBC610: No FPGA PIC\n\0".as_ptr() as *const i8);
        return;
    }

    gef_pic_init(cascade_node);
    of_node_put(cascade_node);
}

unsafe fn gef_sbc610_setup_arch() {
    let regs: *mut device_node;

    printk(b"GE Intelligent Platforms SBC610 6U VPX SBC\n\0".as_ptr() as *const i8);

    #[cfg(CONFIG_SMP)]
    mpc86xx_smp_init();

    fsl_pci_assign_primary();

    /* Remap basic board registers */
    regs = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"gef,fpga-regs\0".as_ptr() as *const i8,
    );
    if !regs.is_null() {
        sbc610_regs = of_iomap(regs, 0);
        if sbc610_regs.is_null() {
            printk(b"Unable to map board registers\n\0".as_ptr() as *const i8);
        }
        of_node_put(regs);
    }

    #[cfg(CONFIG_MMIO_NVRAM)]
    mmio_nvram_init();
}

/* Return the PCB revision */
unsafe fn gef_sbc610_get_pcb_rev() -> u32 {
    let reg: u32;

    reg = ioread32(sbc610_regs);
    (reg >> 8) & 0xff
}

/* Return the board (software) revision */
unsafe fn gef_sbc610_get_board_rev() -> u32 {
    let reg: u32;

    reg = ioread32(sbc610_regs);
    (reg >> 16) & 0xff
}

/* Return the FPGA revision */
unsafe fn gef_sbc610_get_fpga_rev() -> u32 {
    let reg: u32;

    reg = ioread32(sbc610_regs);
    (reg >> 24) & 0xf
}

unsafe fn gef_sbc610_show_cpuinfo(m: *mut seq_file) {
    let svid: u32 = mfspr(SPRN_SVR);

    seq_printf(m, b"Vendor\t\t: GE Intelligent Platforms\n\0".as_ptr() as *const i8);

    seq_printf(
        m,
        b"Revision\t: %u%c\n\0".as_ptr() as *const i8,
        gef_sbc610_get_pcb_rev(),
        b'A' + gef_sbc610_get_board_rev() as u8 - 1,
    );
    seq_printf(
        m,
        b"FPGA Revision\t: %u\n\0".as_ptr() as *const i8,
        gef_sbc610_get_fpga_rev(),
    );

    seq_printf(m, b"SVR\t\t: 0x%x\n\0".as_ptr() as *const i8, svid);
}

unsafe fn gef_sbc610_nec_fixup(pdev: *mut pci_dev) {
    let mut val: u32 = 0;

    /* Do not do the fixup on other platforms! */
    if !machine_is(&gef_sbc610 as *const c_void) {
        return;
    }

    printk(b"Running NEC uPD720101 Fixup\n\0".as_ptr() as *const i8);

    /* Ensure ports 1, 2, 3, 4 & 5 are enabled */
    pci_read_config_dword(pdev, 0xe0, &mut val);
    pci_write_config_dword(pdev, 0xe0, (val & !7) | 0x5);

    /* System clock is 48-MHz Oscillator and EHCI Enabled. */
    pci_write_config_dword(pdev, 0xe4, 1 << 5);
}

// DECLARE_PCI_FIXUP_HEADER(PCI_VENDOR_ID_NEC, PCI_DEVICE_ID_NEC_USB,
//     gef_sbc610_nec_fixup);
// machine_arch_initcall(gef_sbc610, mpc86xx_common_publish_devices);

// define_machine(gef_sbc610) {
//     .name = "GE SBC610",
//     .compatible = "gef,sbc610",
//     .setup_arch = gef_sbc610_setup_arch,
//     .init_IRQ = gef_sbc610_init_irq,
//     .show_cpuinfo = gef_sbc610_show_cpuinfo,
//     .get_irq = mpic_get_irq,
//     .time_init = mpc86xx_time_init,
//     .progress = udbg_progress,
//     .pcibios_fixup_bus = fsl_pcibios_fixup_bus,
// };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
