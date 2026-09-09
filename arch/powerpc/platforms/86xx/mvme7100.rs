// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Board setup routines for the Emerson/Artesyn MVME7100
 *
 * Copyright 2016 Elettra-Sincrotrone Trieste S.C.p.A.
 *
 * Author: Alessio Igor Bogani <alessio.bogani@elettra.eu>
 *
 * Based on earlier code by:
 *
 *\tAjit Prem <ajit.prem@emerson.com>
 *\tCopyright 2008 Emerson
 *
 * USB host fixup is borrowed by:
 *
 *\tMartyn Welch <martyn.welch@ge.com>
 *\tCopyright 2008 GE Intelligent Platforms Embedded Systems, Inc.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// External declarations supplied by the kernel and related platform files.
#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PciDev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PpcMd {
    pub progress: Option<unsafe extern "C" fn(*const c_char, c_uint)>,
}

extern "C" {
    static mut ppc_md: PpcMd;

    fn mpc86xx_smp_init();
    fn fsl_pci_assign_primary();
    fn of_find_compatible_node(
        from: *mut DeviceNode,
        typ: *const c_char,
        compatible: *const c_char,
    ) -> *mut DeviceNode;
    fn of_iomap(node: *mut DeviceNode, index: c_int) -> *mut c_void;
    fn of_node_put(node: *mut DeviceNode);
    fn readb(addr: *mut c_void) -> u8;
    fn writeb(value: u8, addr: *mut c_void);
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn of_get_flat_dt_root() -> c_ulong;
    fn of_flat_dt_is_compatible(root: c_ulong, compatible: *const c_char) -> c_int;
    fn machine_is_mvme7100() -> c_int;
    fn pci_read_config_dword(dev: *mut PciDev, offset: c_uint, value: *mut c_uint) -> c_int;
    fn pci_write_config_dword(dev: *mut PciDev, offset: c_uint, value: c_uint) -> c_int;
    fn mpc86xx_common_publish_devices() -> c_int;
    fn mpc86xx_init_irq();
    fn mpic_get_irq() -> c_int;
    fn mpc86xx_time_init();
    fn udbg_progress(message: *const c_char, hex: c_uint);
    fn fsl_pcibios_fixup_bus(bus: *mut c_void);
}

const MVME7100_INTERRUPT_REG_2_OFFSET: usize = 0x05;
const MVME7100_DS1375_MASK: u8 = 0x40;
const MVME7100_MAX6649_MASK: u8 = 0x20;
const MVME7100_ABORT_MASK: u8 = 0x10;

/*
 * Setup the architecture
 */
unsafe extern "C" fn mvme7100_setup_arch() {
    let mut bcsr_node: *mut DeviceNode;
    let mut mvme7100_regs: *mut c_void = core::ptr::null_mut();
    let mut reg: u8;

    if let Some(progress) = ppc_md.progress {
        progress(b"mvme7100_setup_arch()\0".as_ptr() as *const c_char, 0);
    }

    // CONFIG_SMP conditional preserved from the C implementation.
    #[cfg(CONFIG_SMP)]
    {
        mpc86xx_smp_init();
    }

    fsl_pci_assign_primary();

    /* Remap BCSR registers */
    bcsr_node = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"artesyn,mvme7100-bcsr\0".as_ptr() as *const c_char,
    );
    if !bcsr_node.is_null() {
        mvme7100_regs = of_iomap(bcsr_node, 0);
        of_node_put(bcsr_node);
    }

    if !mvme7100_regs.is_null() {
        /* Disable ds1375, max6649, and abort interrupts */
        reg = readb(mvme7100_regs.add(MVME7100_INTERRUPT_REG_2_OFFSET));
        reg |= MVME7100_DS1375_MASK | MVME7100_MAX6649_MASK | MVME7100_ABORT_MASK;
        writeb(reg, mvme7100_regs.add(MVME7100_INTERRUPT_REG_2_OFFSET));
    } else {
        pr_warn(b"Unable to map board registers\n\0".as_ptr() as *const c_char);
    }

    pr_info(b"MVME7100 board from Artesyn\n\0".as_ptr() as *const c_char);
}

/*
 * Called very early, device-tree isn't unflattened
 */
unsafe extern "C" fn mvme7100_probe() -> c_int {
    let root: c_ulong = of_get_flat_dt_root();
    of_flat_dt_is_compatible(root, b"artesyn,MVME7100\0".as_ptr() as *const c_char)
}

unsafe extern "C" fn mvme7100_usb_host_fixup(pdev: *mut PciDev) {
    let mut val: c_uint;

    if machine_is_mvme7100() == 0 {
        return;
    }

    /* Ensure only ports 1 & 2 are enabled */
    pci_read_config_dword(pdev, 0xe0, &mut val);
    pci_write_config_dword(pdev, 0xe0, (val & !7) | 0x2);

    /* System clock is 48-MHz Oscillator and EHCI Enabled. */
    pci_write_config_dword(pdev, 0xe4, 1 << 5);
}

// DECLARE_PCI_FIXUP_HEADER(PCI_VENDOR_ID_NEC, PCI_DEVICE_ID_NEC_USB,
//     mvme7100_usb_host_fixup);
// machine_arch_initcall(mvme7100, mpc86xx_common_publish_devices);
// define_machine(mvme7100) registration:
// .name = "MVME7100", .probe = mvme7100_probe,
// .setup_arch = mvme7100_setup_arch, .init_IRQ = mpc86xx_init_irq,
// .get_irq = mpic_get_irq, .time_init = mpc86xx_time_init,
// .progress = udbg_progress, and, when CONFIG_PCI is enabled,
// .pcibios_fixup_bus = fsl_pcibios_fixup_bus.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
