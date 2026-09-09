// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * GE IMP3A Board Setup
 *
 * Author Martyn Welch <martyn.welch@ge.com>
 *
 * Copyright 2010 GE Intelligent Platforms Embedded Systems, Inc.
 *
 * Based on: mpc85xx_ds.c (MPC85xx DS Board Setup)
 * Copyright 2007 Freescale Semiconductor Inc.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct mpic;
#[repr(C)]
pub struct device_node;
#[repr(C)]
pub struct resource {
    pub start: usize,
}
#[repr(C)]
pub struct seq_file;

extern "C" {
    static mut fsl_pci_primary: *mut device_node;
    static mut ppc_md: ppc_machine_desc;

    fn of_machine_is_compatible(compat: *const c_char) -> c_int;
    fn mpic_alloc(
        node: *mut device_node,
        flags: c_uint,
        senses: c_uint,
        irq_offset: c_uint,
        irq_count: c_uint,
        name: *const c_char,
    ) -> *mut mpic;
    fn mpic_init(mpic: *mut mpic);
    fn of_device_is_compatible(node: *mut device_node, compat: *const c_char) -> c_int;
    fn gef_pic_init(node: *mut device_node);
    fn of_node_put(node: *mut device_node);
    fn of_node_get(node: *mut device_node) -> *mut device_node;
    fn of_address_to_resource(
        node: *mut device_node,
        index: c_int,
        resource: *mut resource,
    ) -> c_int;
    fn mpc85xx_smp_init();
    fn swiotlb_detect_4g();
    fn of_find_compatible_node(
        from: *mut device_node,
        type_: *const c_char,
        compatible: *const c_char,
    ) -> *mut device_node;
    fn of_iomap(node: *mut device_node, index: c_int) -> *mut c_void;
    fn mmio_nvram_init();
    fn ioread16(addr: *mut c_void) -> u16;
    fn seq_printf(m: *mut seq_file, format: *const c_char, ...);
    fn printk(format: *const c_char, ...);
    fn BUG_ON(condition: c_int);
    fn mpic_get_irq() -> c_int;
    fn udbg_progress(message: *const c_char, hex: c_uint);
    fn fsl_pcibios_fixup_bus(bus: *mut c_void);
    fn fsl_pcibios_fixup_phb(phb: *mut c_void);
}

#[repr(C)]
pub struct ppc_machine_desc {
    pub progress: Option<unsafe extern "C" fn(*const c_char, c_uint)>,
}

pub const MPIC_NO_RESET: c_uint = 1 << 0;
pub const MPIC_BIG_ENDIAN: c_uint = 1 << 1;
pub const MPIC_SINGLE_DEST_CPU: c_uint = 1 << 2;

pub static mut imp3a_regs: *mut c_void = core::ptr::null_mut();

unsafe fn ge_imp3a_pic_init() {
    let mpic: *mut mpic;
    let mut cascade_node: *mut device_node = core::ptr::null_mut();

    if of_machine_is_compatible(b"fsl,MPC8572DS-CAMP\0".as_ptr() as *const c_char) != 0 {
        mpic = mpic_alloc(
            core::ptr::null_mut(),
            0,
            MPIC_NO_RESET | MPIC_BIG_ENDIAN | MPIC_SINGLE_DEST_CPU,
            0,
            256,
            b" OpenPIC  \0".as_ptr() as *const c_char,
        );
    } else {
        mpic = mpic_alloc(
            core::ptr::null_mut(),
            0,
            MPIC_BIG_ENDIAN | MPIC_SINGLE_DEST_CPU,
            0,
            256,
            b" OpenPIC  \0".as_ptr() as *const c_char,
        );
    }

    BUG_ON(mpic.is_null() as c_int);
    mpic_init(mpic);
    /*
     * There is a simple interrupt handler in the main FPGA, this needs
     * to be cascaded into the MPIC
     */
    // for_each_node_by_type(np, "interrupt-controller")
    // is an external device-tree iterator in the kernel environment.
    let mut np: *mut device_node = core::ptr::null_mut();
    while !np.is_null() {
        if of_device_is_compatible(np, b"gef,fpga-pic-1.00\0".as_ptr() as *const c_char) != 0 {
            cascade_node = np;
            break;
        }
        break;
    }

    if cascade_node.is_null() {
        printk(b"IMP3A: No FPGA PIC\n\0".as_ptr() as *const c_char);
        return;
    }

    gef_pic_init(cascade_node);
    of_node_put(cascade_node);
}

unsafe fn ge_imp3a_pci_assign_primary() {
    // #ifdef CONFIG_PCI
    let mut np: *mut device_node = core::ptr::null_mut();
    let mut rsrc = resource { start: 0 };

    // for_each_node_by_type(np, "pci")
    while !np.is_null() {
        if of_device_is_compatible(np, b"fsl,mpc8540-pci\0".as_ptr() as *const c_char) != 0
            || of_device_is_compatible(np, b"fsl,mpc8548-pcie\0".as_ptr() as *const c_char) != 0
            || of_device_is_compatible(np, b"fsl,p2020-pcie\0".as_ptr() as *const c_char) != 0
        {
            of_address_to_resource(np, 0, &mut rsrc);
            if (rsrc.start & 0xfffff) == 0x9000 {
                of_node_put(fsl_pci_primary);
                fsl_pci_primary = of_node_get(np);
            }
        }
        break;
    }
    // #endif
}

unsafe fn ge_imp3a_setup_arch() {
    let mut regs: *mut device_node;

    if let Some(progress) = ppc_md.progress {
        progress(b"ge_imp3a_setup_arch()\0".as_ptr() as *const c_char, 0);
    }

    mpc85xx_smp_init();
    ge_imp3a_pci_assign_primary();
    swiotlb_detect_4g();

    /* Remap basic board registers */
    regs = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"ge,imp3a-fpga-regs\0".as_ptr() as *const c_char,
    );
    if !regs.is_null() {
        imp3a_regs = of_iomap(regs, 0);
        if imp3a_regs.is_null() {
            printk(b"Unable to map board registers\n\0".as_ptr() as *const c_char);
        }
        of_node_put(regs);
    }

    // #if defined(CONFIG_MMIO_NVRAM)
    mmio_nvram_init();
    // #endif

    printk(b"GE Intelligent Platforms IMP3A 3U cPCI SBC\n\0".as_ptr() as *const c_char);
}

/* Return the PCB revision */
unsafe fn ge_imp3a_get_pcb_rev() -> c_uint {
    let reg = ioread16(imp3a_regs);
    ((reg >> 8) & 0xff) as c_uint
}

/* Return the board (software) revision */
unsafe fn ge_imp3a_get_board_rev() -> c_uint {
    let reg = ioread16(imp3a_regs.add(2));
    (reg & 0xff) as c_uint
}

/* Return the FPGA revision */
unsafe fn ge_imp3a_get_fpga_rev() -> c_uint {
    let reg = ioread16(imp3a_regs.add(2));
    ((reg >> 8) & 0xff) as c_uint
}

/* Return compactPCI Geographical Address */
unsafe fn ge_imp3a_get_cpci_geo_addr() -> c_uint {
    let reg = ioread16(imp3a_regs.add(6));
    ((reg & 0x0f00) >> 8) as c_uint
}

/* Return compactPCI System Controller Status */
unsafe fn ge_imp3a_get_cpci_is_syscon() -> c_uint {
    let reg = ioread16(imp3a_regs.add(6));
    (reg & (1 << 12)) as c_uint
}

unsafe fn ge_imp3a_show_cpuinfo(m: *mut seq_file) {
    seq_printf(m, b"Vendor\t\t: GE Intelligent Platforms\n\0".as_ptr() as *const c_char);
    seq_printf(
        m,
        b"Revision\t: %u%c\n\0".as_ptr() as *const c_char,
        ge_imp3a_get_pcb_rev(),
        ('A' as c_uint + ge_imp3a_get_board_rev() - 1) as c_int,
    );
    seq_printf(
        m,
        b"FPGA Revision\t: %u\n\0".as_ptr() as *const c_char,
        ge_imp3a_get_fpga_rev(),
    );
    seq_printf(
        m,
        b"cPCI geo. addr\t: %u\n\0".as_ptr() as *const c_char,
        ge_imp3a_get_cpci_geo_addr(),
    );
    seq_printf(
        m,
        b"cPCI syscon\t: %s\n\0".as_ptr() as *const c_char,
        if ge_imp3a_get_cpci_is_syscon() != 0 {
            b"yes\0".as_ptr()
        } else {
            b"no\0".as_ptr()
        },
    );
}

// machine_arch_initcall(ge_imp3a, mpc85xx_common_publish_devices);
// define_machine(ge_imp3a) {
//     .name = "GE_IMP3A",
//     .compatible = "ge,IMP3A",
//     .setup_arch = ge_imp3a_setup_arch,
//     .init_IRQ = ge_imp3a_pic_init,
//     .show_cpuinfo = ge_imp3a_show_cpuinfo,
// #ifdef CONFIG_PCI
//     .pcibios_fixup_bus = fsl_pcibios_fixup_bus,
//     .pcibios_fixup_phb = fsl_pcibios_fixup_phb,
// #endif
//     .get_irq = mpic_get_irq,
//     .progress = udbg_progress,
// };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
