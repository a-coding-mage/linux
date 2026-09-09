// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2009 Extreme Engineering Solutions, Inc.
 *
 * X-ES board-specific functionality
 *
 * Based on mpc85xx_ds code from Freescale Semiconductor, Inc.
 *
 * Author: Nate Case <ncase@xes-inc.com>
 */

// Linux and architecture dependencies supplied by other translation units.

const MPC85XX_L2CTL_L2E: u32 = 0x8000_0000; // L2 enable
const MPC85XX_L2CTL_L2I: u32 = 0x4000_0000; // L2 flash invalidate
const MPC85XX_L2CTL_L2SIZ_MASK: u32 = 0x3000_0000; // L2 SRAM size (R/O)

extern "C" {
    fn mpic_alloc(
        node: *mut core::ffi::c_void,
        flags: i32,
        reg_type: i32,
        irq_offset: i32,
        irq_count: i32,
        name: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_void;
    fn mpic_init(mpic: *mut core::ffi::c_void);
    fn of_machine_is_compatible(compat: *const core::ffi::c_char) -> bool;
    fn in_be32(addr: *const u32) -> u32;
    fn out_be32(addr: *mut u32, value: u32);
    fn printk(format: *const core::ffi::c_char, ...);
    fn of_device_is_compatible(
        node: *mut core::ffi::c_void,
        compat: *const core::ffi::c_char,
    ) -> bool;
    fn of_address_to_resource(
        node: *mut core::ffi::c_void,
        index: i32,
        resource: *mut Resource,
    ) -> i32;
    fn ioremap(addr: u64, size: u64) -> *mut u32;
    fn of_find_node_by_path(path: *const core::ffi::c_char) -> *mut core::ffi::c_void;
    fn of_get_property(
        node: *mut core::ffi::c_void,
        name: *const core::ffi::c_char,
        length: *mut i32,
    ) -> *const core::ffi::c_char;
    fn strlen(string: *const core::ffi::c_char) -> usize;
    fn mpc85xx_smp_init();
    fn fsl_pci_assign_primary();
    fn mpic_get_irq() -> i32;
    fn udbg_progress(message: *const core::ffi::c_char, value: u16);
    fn fsl_pcibios_fixup_bus(bus: *mut core::ffi::c_void);
    fn fsl_pcibios_fixup_phb(phb: *mut core::ffi::c_void);
    fn mpc85xx_common_publish_devices() -> i32;
}

#[repr(C)]
struct Resource {
    start: u64,
    end: u64,
    flags: u64,
}

#[inline]
unsafe fn resource_size(resource: *const Resource) -> u64 {
    (*resource).end - (*resource).start + 1
}

unsafe fn xes_mpc85xx_pic_init() {
    let mpic = mpic_alloc(
        core::ptr::null_mut(),
        0,
        MPIC_BIG_ENDIAN,
        0,
        256,
        b" OpenPIC  \0".as_ptr() as *const core::ffi::c_char,
    );
    if mpic.is_null() {
        // BUG_ON(mpic == NULL)
        core::hint::unreachable_unchecked();
    }
    mpic_init(mpic);
}

unsafe fn xes_mpc85xx_configure_l2(l2_base: *mut u32) {
    let mut ctl: u32;
    let tmp: u32;

    core::arch::asm!("msync; isync");
    tmp = in_be32(l2_base);

    /*
     * xMon may have enabled part of L2 as SRAM, so we need to set it
     * up for all cache mode just to be safe.
     */
    printk(b"xes_mpc85xx: Enabling L2 as cache\n\0".as_ptr() as *const _);

    ctl = MPC85XX_L2CTL_L2E | MPC85XX_L2CTL_L2I;
    if of_machine_is_compatible(b"MPC8540\0".as_ptr() as *const _)
        || of_machine_is_compatible(b"MPC8560\0".as_ptr() as *const _)
    {
        /*
         * Assume L2 SRAM is used fully for cache, so set
         * L2BLKSZ (bits 4:5) to match L2SIZ (bits 2:3).
         */
        ctl |= (tmp & MPC85XX_L2CTL_L2SIZ_MASK) >> 2;
    }

    core::arch::asm!("msync; isync");
    out_be32(l2_base, ctl);
    core::arch::asm!("msync; isync");
}

unsafe fn xes_mpc85xx_fixups() {
    let mut np: *mut core::ffi::c_void;
    let mut err: i32;

    // for_each_node_by_name(np, "l2-cache-controller")
    np = for_each_node_by_name(b"l2-cache-controller\0".as_ptr() as *const _);
    while !np.is_null() {
        let mut r = [Resource { start: 0, end: 0, flags: 0 }; 2];
        let l2_base: *mut u32;

        /* Only MPC8548, MPC8540, and MPC8560 boards are affected */
        if !of_device_is_compatible(np, b"fsl,mpc8548-l2-cache-controller\0".as_ptr() as *const _)
            && !of_device_is_compatible(np, b"fsl,mpc8540-l2-cache-controller\0".as_ptr() as *const _)
            && !of_device_is_compatible(np, b"fsl,mpc8560-l2-cache-controller\0".as_ptr() as *const _)
        {
            np = of_node_next_by_name(np, b"l2-cache-controller\0".as_ptr() as *const _);
            continue;
        }

        err = of_address_to_resource(np, 0, &mut r[0]);
        if err != 0 {
            printk(b"xes_mpc85xx: Could not get resource for device tree node '%pOF'\0".as_ptr() as *const _, np);
            np = of_node_next_by_name(np, b"l2-cache-controller\0".as_ptr() as *const _);
            continue;
        }

        l2_base = ioremap(r[0].start, resource_size(&r[0]));
        xes_mpc85xx_configure_l2(l2_base);
        np = of_node_next_by_name(np, b"l2-cache-controller\0".as_ptr() as *const _);
    }
}

unsafe fn xes_mpc85xx_setup_arch() {
    let root = of_find_node_by_path(b"/\0".as_ptr() as *const _);
    let mut model = b"Unknown\0".as_ptr() as *const core::ffi::c_char;

    if root.is_null() {
        return;
    }

    model = of_get_property(root, b"model\0".as_ptr() as *const _, core::ptr::null_mut());
    printk(
        b"X-ES MPC85xx-based single-board computer: %s\n\0".as_ptr() as *const _,
        model.add(strlen(b"xes,\0".as_ptr() as *const _)),
    );

    xes_mpc85xx_fixups();
    mpc85xx_smp_init();
    fsl_pci_assign_primary();
}

// machine_arch_initcall(xes_mpc8572, mpc85xx_common_publish_devices);
// machine_arch_initcall(xes_mpc8548, mpc85xx_common_publish_devices);
// machine_arch_initcall(xes_mpc8540, mpc85xx_common_publish_devices);

// define_machine(xes_mpc8572)
pub static mut XES_MPC8572: MachineDescription = MachineDescription {
    name: b"X-ES MPC8572\0".as_ptr(),
    compatible: b"xes,MPC8572\0".as_ptr(),
    setup_arch: Some(xes_mpc85xx_setup_arch),
    init_irq: Some(xes_mpc85xx_pic_init),
    // CONFIG_PCI: pcibios_fixup_bus = fsl_pcibios_fixup_bus,
    // CONFIG_PCI: pcibios_fixup_phb = fsl_pcibios_fixup_phb,
    get_irq: Some(mpic_get_irq),
    progress: Some(udbg_progress),
};

// define_machine(xes_mpc8548)
pub static mut XES_MPC8548: MachineDescription = MachineDescription {
    name: b"X-ES MPC8548\0".as_ptr(),
    compatible: b"xes,MPC8548\0".as_ptr(),
    setup_arch: Some(xes_mpc85xx_setup_arch),
    init_irq: Some(xes_mpc85xx_pic_init),
    // CONFIG_PCI: pcibios_fixup_bus = fsl_pcibios_fixup_bus,
    // CONFIG_PCI: pcibios_fixup_phb = fsl_pcibios_fixup_phb,
    get_irq: Some(mpic_get_irq),
    progress: Some(udbg_progress),
};

// define_machine(xes_mpc8540)
pub static mut XES_MPC8540: MachineDescription = MachineDescription {
    name: b"X-ES MPC8540\0".as_ptr(),
    compatible: b"xes,MPC8540\0".as_ptr(),
    setup_arch: Some(xes_mpc85xx_setup_arch),
    init_irq: Some(xes_mpc85xx_pic_init),
    // CONFIG_PCI: pcibios_fixup_bus = fsl_pcibios_fixup_bus,
    // CONFIG_PCI: pcibios_fixup_phb = fsl_pcibios_fixup_phb,
    get_irq: Some(mpic_get_irq),
    progress: Some(udbg_progress),
};

// External kernel-provided definitions and iteration helpers.
const MPIC_BIG_ENDIAN: i32 = 0;
#[repr(C)]
pub struct MachineDescription {
    pub name: *const u8,
    pub compatible: *const u8,
    pub setup_arch: Option<unsafe fn()>,
    pub init_irq: Option<unsafe fn()>,
    pub get_irq: Option<unsafe extern "C" fn() -> i32>,
    pub progress: Option<unsafe extern "C" fn(*const core::ffi::c_char, u16)>,
}

extern "C" {
    fn for_each_node_by_name(name: *const core::ffi::c_char) -> *mut core::ffi::c_void;
    fn of_node_next_by_name(
        node: *mut core::ffi::c_void,
        name: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
