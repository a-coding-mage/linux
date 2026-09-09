// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * GE SBC310 board support
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

// C headers provide the external kernel types, functions, constants, and macros used below.
// DEBUG is undefined in the source; the DBG macro is therefore a no-op.

extern "C" {
    static mut sbc310_regs: *mut core::ffi::c_void;

    fn mpc86xx_init_irq();
    fn of_find_compatible_node(
        from: *mut device_node,
        typ: *const core::ffi::c_char,
        compatible: *const core::ffi::c_char,
    ) -> *mut device_node;
    fn printk(format: *const core::ffi::c_char, ...);
    fn gef_pic_init(node: *mut device_node);
    fn of_node_put(node: *mut device_node);
    fn mpc86xx_smp_init();
    fn fsl_pci_assign_primary();
    fn of_iomap(node: *mut device_node, index: i32) -> *mut core::ffi::c_void;
    fn mmio_nvram_init();
    fn ioread32(addr: *mut core::ffi::c_void) -> u32;
    fn mfspr(spr: u32) -> u32;
    fn seq_printf(m: *mut seq_file, format: *const core::ffi::c_char, ...);
    fn machine_is(machine: machine_id) -> bool;
    fn pci_read_config_dword(dev: *mut pci_dev, offset: u32, value: *mut u32) -> i32;
    fn pci_write_config_dword(dev: *mut pci_dev, offset: u32, value: u32) -> i32;
    fn mpic_get_irq(regs: *mut pt_regs) -> i32;
    fn mpc86xx_time_init();
    fn udbg_progress(message: *const core::ffi::c_char, value: u32);
    fn fsl_pcibios_fixup_bus(bus: *mut pci_bus);
    fn mpc86xx_common_publish_devices() -> i32;

    static gef_sbc310: machine_id;
}

#[repr(C)]
struct device_node;
#[repr(C)]
struct seq_file;
#[repr(C)]
struct pci_dev;
#[repr(C)]
struct pt_regs;
#[repr(C)]
struct pci_bus;
#[repr(C)]
struct machine_id;

const SPRN_SVR: u32 = 0x11E;
const PCI_VENDOR_ID_NEC: u16 = 0x1033;
const PCI_DEVICE_ID_NEC_USB: u16 = 0x00E0;

unsafe fn gef_sbc310_init_irq() {
    let mut cascade_node: *mut device_node = core::ptr::null_mut();

    mpc86xx_init_irq();

    /*
     * There is a simple interrupt handler in the main FPGA, this needs
     * to be cascaded into the MPIC
     */
    cascade_node = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"gef,fpga-pic\0".as_ptr() as *const core::ffi::c_char,
    );
    if cascade_node.is_null() {
        printk(b"SBC310: No FPGA PIC\0".as_ptr() as *const core::ffi::c_char);
        return;
    }

    gef_pic_init(cascade_node);
    of_node_put(cascade_node);
}

unsafe fn gef_sbc310_setup_arch() {
    let regs: *mut device_node;
    printk(b"GE Intelligent Platforms SBC310 6U VPX SBC\n\0".as_ptr() as *const core::ffi::c_char);

    // CONFIG_SMP conditional from the C source.
    #[cfg(CONFIG_SMP)]
    mpc86xx_smp_init();

    fsl_pci_assign_primary();

    /* Remap basic board registers */
    regs = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"gef,fpga-regs\0".as_ptr() as *const core::ffi::c_char,
    );
    if !regs.is_null() {
        sbc310_regs = of_iomap(regs, 0);
        if sbc310_regs.is_null() {
            printk(b"Unable to map board registers\n\0".as_ptr() as *const core::ffi::c_char);
        }
        of_node_put(regs);
    }

    // CONFIG_MMIO_NVRAM conditional from the C source.
    #[cfg(CONFIG_MMIO_NVRAM)]
    mmio_nvram_init();
}

/* Return the PCB revision */
unsafe fn gef_sbc310_get_board_id() -> u32 {
    let reg = ioread32(sbc310_regs);
    reg & 0xff
}

/* Return the PCB revision */
unsafe fn gef_sbc310_get_pcb_rev() -> u32 {
    let reg = ioread32(sbc310_regs);
    (reg >> 8) & 0xff
}

/* Return the board (software) revision */
unsafe fn gef_sbc310_get_board_rev() -> u32 {
    let reg = ioread32(sbc310_regs);
    (reg >> 16) & 0xff
}

/* Return the FPGA revision */
unsafe fn gef_sbc310_get_fpga_rev() -> u32 {
    let reg = ioread32(sbc310_regs);
    (reg >> 24) & 0xf
}

unsafe fn gef_sbc310_show_cpuinfo(m: *mut seq_file) {
    let svid = mfspr(SPRN_SVR);

    seq_printf(m, b"Vendor\t\t: GE Intelligent Platforms\n\0".as_ptr() as *const core::ffi::c_char);
    seq_printf(m, b"Board ID\t: 0x%2.2x\n\0".as_ptr() as *const core::ffi::c_char, gef_sbc310_get_board_id());
    seq_printf(
        m,
        b"Revision\t: %u%c\n\0".as_ptr() as *const core::ffi::c_char,
        gef_sbc310_get_pcb_rev(),
        ('A' as u32).wrapping_add(gef_sbc310_get_board_rev()).wrapping_sub(1),
    );
    seq_printf(m, b"FPGA Revision\t: %u\n\0".as_ptr() as *const core::ffi::c_char, gef_sbc310_get_fpga_rev());
    seq_printf(m, b"SVR\t\t: 0x%x\n\0".as_ptr() as *const core::ffi::c_char, svid);
}

unsafe fn gef_sbc310_nec_fixup(pdev: *mut pci_dev) {
    let mut val: u32 = 0;

    /* Do not do the fixup on other platforms! */
    if !machine_is(gef_sbc310) {
        return;
    }

    printk(b"Running NEC uPD720101 Fixup\n\0".as_ptr() as *const core::ffi::c_char);

    /* Ensure only ports 1 & 2 are enabled */
    pci_read_config_dword(pdev, 0xe0, &mut val);
    pci_write_config_dword(pdev, 0xe0, (val & !7) | 0x2);

    /* System clock is 48-MHz Oscillator and EHCI Enabled. */
    pci_write_config_dword(pdev, 0xe4, 1 << 5);
}

// DECLARE_PCI_FIXUP_HEADER(PCI_VENDOR_ID_NEC, PCI_DEVICE_ID_NEC_USB,
//     gef_sbc310_nec_fixup);
// machine_arch_initcall(gef_sbc310, mpc86xx_common_publish_devices);
// define_machine(gef_sbc310) registers the following machine descriptor:
// .name = "GE SBC310", .compatible = "gef,sbc310",
// .setup_arch = gef_sbc310_setup_arch, .init_IRQ = gef_sbc310_init_irq,
// .show_cpuinfo = gef_sbc310_show_cpuinfo, .get_irq = mpic_get_irq,
// .time_init = mpc86xx_time_init, .progress = udbg_progress,
// .pcibios_fixup_bus = fsl_pcibios_fixup_bus when CONFIG_PCI is enabled.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
