// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * GE PPC9A board support
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

// C header dependencies are supplied by the surrounding kernel translation.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct device_node { _private: [u8; 0] }
#[repr(C)]
pub struct seq_file { _private: [u8; 0] }
#[repr(C)]
pub struct pci_dev { _private: [u8; 0] }

type IoMem = *mut c_void;

extern "C" {
    fn mpc86xx_init_irq();
    fn of_find_compatible_node(from: *mut device_node, ty: *const c_char,
                               compatible: *const c_char) -> *mut device_node;
    fn printk(fmt: *const c_char, ...);
    fn gef_pic_init(node: *mut device_node);
    fn of_node_put(node: *mut device_node);
    fn mpc86xx_smp_init();
    fn fsl_pci_assign_primary();
    fn of_iomap(node: *mut device_node, index: c_int) -> IoMem;
    fn mmio_nvram_init();
    fn ioread32be(addr: IoMem) -> u32;
    fn mfspr(spr: c_uint) -> c_ulong;
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...);
    fn machine_is(machine: *const c_char) -> bool;
    fn pci_read_config_dword(dev: *mut pci_dev, where_: c_int, value: *mut c_uint) -> c_int;
    fn pci_write_config_dword(dev: *mut pci_dev, where_: c_int, value: c_uint) -> c_int;
    fn mpic_get_irq() -> c_int;
    fn mpc86xx_time_init();
    fn udbg_progress(message: *const c_char, value: c_uint);
    fn fsl_pcibios_fixup_bus(bus: *mut c_void);
}

pub static mut ppc9a_regs: IoMem = core::ptr::null_mut();

unsafe fn gef_ppc9a_init_irq() {
    let mut cascade_node: *mut device_node = core::ptr::null_mut();

    mpc86xx_init_irq();

    /* There is a simple interrupt handler in the main FPGA, this needs
     * to be cascaded into the MPIC. */
    cascade_node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(),
        b"gef,fpga-pic-1.00\0".as_ptr() as *const c_char);
    if cascade_node.is_null() {
        printk(b"PPC9A: No FPGA PIC\0".as_ptr() as *const c_char);
        return;
    }

    gef_pic_init(cascade_node);
    of_node_put(cascade_node);
}

unsafe fn gef_ppc9a_setup_arch() {
    let regs: *mut device_node;

    printk(b"GE Intelligent Platforms PPC9A 6U VME SBC\n\0".as_ptr() as *const c_char);

    // #ifdef CONFIG_SMP
    mpc86xx_smp_init();

    fsl_pci_assign_primary();

    /* Remap basic board registers */
    regs = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(),
        b"gef,ppc9a-fpga-regs\0".as_ptr() as *const c_char);
    if !regs.is_null() {
        ppc9a_regs = of_iomap(regs, 0);
        if ppc9a_regs.is_null() {
            printk(b"Unable to map board registers\n\0".as_ptr() as *const c_char);
        }
        of_node_put(regs);
    }

    // #if defined(CONFIG_MMIO_NVRAM)
    mmio_nvram_init();
}

/* Return the PCB revision */
unsafe fn gef_ppc9a_get_pcb_rev() -> c_uint {
    let reg = ioread32be(ppc9a_regs);
    (reg >> 16) & 0xff
}

/* Return the board (software) revision */
unsafe fn gef_ppc9a_get_board_rev() -> c_uint {
    let reg = ioread32be(ppc9a_regs);
    (reg >> 8) & 0xff
}

/* Return the FPGA revision */
unsafe fn gef_ppc9a_get_fpga_rev() -> c_uint {
    ioread32be(ppc9a_regs) & 0xf
}

/* Return VME Geographical Address */
unsafe fn gef_ppc9a_get_vme_geo_addr() -> c_uint {
    ioread32be(ppc9a_regs.wrapping_add(4)) & 0x1f
}

/* Return VME System Controller Status */
unsafe fn gef_ppc9a_get_vme_is_syscon() -> c_uint {
    (ioread32be(ppc9a_regs.wrapping_add(4)) >> 9) & 0x1
}

unsafe fn gef_ppc9a_show_cpuinfo(m: *mut seq_file) {
    let svid = mfspr(0);
    seq_printf(m, b"Vendor\t\t: GE Intelligent Platforms\n\0".as_ptr() as *const c_char);
    seq_printf(m, b"Revision\t: %u%c\n\0".as_ptr() as *const c_char,
        gef_ppc9a_get_pcb_rev(), (b'A' + gef_ppc9a_get_board_rev() as u8) as c_int);
    seq_printf(m, b"FPGA Revision\t: %u\n\0".as_ptr() as *const c_char, gef_ppc9a_get_fpga_rev());
    seq_printf(m, b"SVR\t\t: 0x%x\n\0".as_ptr() as *const c_char, svid);
    seq_printf(m, b"VME geo. addr\t: %u\n\0".as_ptr() as *const c_char, gef_ppc9a_get_vme_geo_addr());
    seq_printf(m, b"VME syscon\t: %s\n\0".as_ptr() as *const c_char,
        if gef_ppc9a_get_vme_is_syscon() != 0 { b"yes\0".as_ptr() } else { b"no\0".as_ptr() });
}

unsafe fn gef_ppc9a_nec_fixup(pdev: *mut pci_dev) {
    let mut val: c_uint = 0;
    // Do not do the fixup on other platforms!
    if !machine_is(b"gef_ppc9a\0".as_ptr() as *const c_char) { return; }
    printk(b"Running NEC uPD720101 Fixup\n\0".as_ptr() as *const c_char);
    pci_read_config_dword(pdev, 0xe0, &mut val);
    pci_write_config_dword(pdev, 0xe0, (val & !7) | 0x5);
    pci_write_config_dword(pdev, 0xe4, 1 << 5);
}

// DECLARE_PCI_FIXUP_HEADER(PCI_VENDOR_ID_NEC, PCI_DEVICE_ID_NEC_USB, gef_ppc9a_nec_fixup);
// machine_arch_initcall(gef_ppc9a, mpc86xx_common_publish_devices);
// define_machine(gef_ppc9a) metadata: GE PPC9A, gef,ppc9a, and the callbacks above.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
