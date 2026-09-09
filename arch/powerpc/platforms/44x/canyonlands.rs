// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * This contain platform specific code for APM PPC460EX based Canyonlands
 * board.
 *
 * Copyright (c) 2010, Applied Micro Circuits Corporation
 * Author: Rupjyoti Sarmah <rsarmah@apm.com>
 */

// C header dependencies are supplied by the surrounding kernel translation.

const BCSR_USB_EN: u8 = 0x11;

#[repr(C)]
struct OfDeviceId {
    name: *const core::ffi::c_char,
    type_: *const core::ffi::c_char,
    compatible: *const core::ffi::c_char,
    data: *const core::ffi::c_void,
}

static PPC460EX_OF_BUS: [OfDeviceId; 5] = [
    OfDeviceId { name: core::ptr::null(), type_: core::ptr::null(), compatible: c"ibm,plb4".as_ptr(), data: core::ptr::null() },
    OfDeviceId { name: core::ptr::null(), type_: core::ptr::null(), compatible: c"ibm,opb".as_ptr(), data: core::ptr::null() },
    OfDeviceId { name: core::ptr::null(), type_: core::ptr::null(), compatible: c"ibm,ebc".as_ptr(), data: core::ptr::null() },
    OfDeviceId { name: core::ptr::null(), type_: core::ptr::null(), compatible: c"simple-bus".as_ptr(), data: core::ptr::null() },
    OfDeviceId { name: core::ptr::null(), type_: core::ptr::null(), compatible: core::ptr::null(), data: core::ptr::null() },
];

enum DeviceNode {}

extern "C" {
    fn of_platform_bus_probe(
        root: *mut DeviceNode,
        matches: *const OfDeviceId,
        parent: *mut core::ffi::c_void,
    ) -> i32;
    fn pci_set_flags(flags: u32);
    fn of_find_compatible_node(
        from: *mut DeviceNode,
        type_: *const core::ffi::c_char,
        compatible: *const core::ffi::c_char,
    ) -> *mut DeviceNode;
    fn of_iomap(node: *mut DeviceNode, index: i32) -> *mut u8;
    fn of_node_put(node: *mut DeviceNode);
    fn iounmap(addr: *mut core::ffi::c_void);
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn msleep(msecs: u32);
    fn udbg_progress(message: *const core::ffi::c_char, hex: u16);
    fn uic_init_tree();
    fn uic_get_irq() -> i32;
    fn ppc4xx_reset_system();
    fn setbits8(addr: *mut u8, mask: u8);
    fn clrbits8(addr: *mut u8, mask: u8);
    fn setbits32(addr: *mut u32, mask: u32);
}

const PCI_REASSIGN_ALL_RSRC: u32 = 0x00000001;
const ENODEV: i32 = 19;
const GPIO0_OSRH: usize = 0;
const GPIO0_TSRH: usize = 0;

unsafe fn ppc460ex_device_probe() -> i32 {
    of_platform_bus_probe(core::ptr::null_mut(), PPC460EX_OF_BUS.as_ptr(), core::ptr::null_mut());
    0
}

// Using this code only for the Canyonlands board.
unsafe fn ppc460ex_probe() -> i32 {
    pci_set_flags(PCI_REASSIGN_ALL_RSRC);
    1
}

// USB PHY fixup code on Canyonlands kit.
unsafe fn ppc460ex_canyonlands_fixup() -> i32 {
    let mut bcsr: *mut u8;
    let mut vaddr: *mut u8;
    let mut np: *mut DeviceNode;
    let mut ret: i32 = 0;

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"amcc,ppc460ex-bcsr".as_ptr());
    if np.is_null() {
        printk(c"failed did not find amcc, ppc460ex bcsr node\n".as_ptr());
        return -ENODEV;
    }

    bcsr = of_iomap(np, 0);
    of_node_put(np);

    if bcsr.is_null() {
        printk(c"Could not remap bcsr\n".as_ptr());
        ret = -ENODEV;
        iounmap(bcsr.cast());
        return ret;
    }

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"ibm,ppc4xx-gpio".as_ptr());
    if np.is_null() {
        printk(c"failed did not find ibm, ppc4xx-gpio node\n".as_ptr());
        return -ENODEV;
    }

    vaddr = of_iomap(np, 0);
    of_node_put(np);

    if vaddr.is_null() {
        printk(c"Could not get gpio node address\n".as_ptr());
        ret = -ENODEV;
        iounmap(vaddr.cast());
        iounmap(bcsr.cast());
        return ret;
    }

    // Disable USB, through the BCSR7 bits
    setbits8(bcsr.add(7), BCSR_USB_EN);

    // Wait for a while after reset
    msleep(100);

    // Enable USB here
    clrbits8(bcsr.add(7), BCSR_USB_EN);

    /*
     * Configure multiplexed gpio16 and gpio19 as alternate1 output
     * source after USB reset. In this configuration gpio16 will be
     * USB2HStop and gpio19 will be USB2DStop. For more details refer to
     * table 34-7 of PPC460EX user manual.
     */
    setbits32(vaddr.add(GPIO0_OSRH).cast(), 0x42000000);
    setbits32(vaddr.add(GPIO0_TSRH).cast(), 0x42000000);

    iounmap(vaddr.cast());
    iounmap(bcsr.cast());
    ret
}

// machine_device_initcall(canyonlands, ppc460ex_device_probe);
// machine_device_initcall(canyonlands, ppc460ex_canyonlands_fixup);

#[repr(C)]
struct MachineDesc {
    name: *const core::ffi::c_char,
    compatible: *const core::ffi::c_char,
    probe: unsafe fn() -> i32,
    progress: unsafe extern "C" fn(*const core::ffi::c_char, u16),
    init_irq: unsafe extern "C" fn(),
    get_irq: unsafe extern "C" fn() -> i32,
    restart: unsafe extern "C" fn(),
}

#[allow(non_upper_case_globals)]
static canyonlands: MachineDesc = MachineDesc {
    name: c"Canyonlands".as_ptr(),
    compatible: c"amcc,canyonlands".as_ptr(),
    probe: ppc460ex_probe,
    progress: udbg_progress,
    init_irq: uic_init_tree,
    get_irq: uic_get_irq,
    restart: ppc4xx_reset_system,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
