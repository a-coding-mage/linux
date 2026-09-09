// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2008 Emcraft Systems
 * Sergei Poselenov <sposelenov@emcraft.com>
 *
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
 *  Kumar Gala <kumar.gala@freescale.com>
 *      Copyright 2004 Freescale Semiconductor Inc.
 */

use core::ffi::c_void;

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Mpic {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PpcMd {
    pub progress: Option<unsafe extern "C" fn(*const u8, u32)>,
}

#[repr(C)]
pub struct MachineDesc {
    pub name: *const u8,
    pub compatible: *const u8,
    pub setup_arch: Option<unsafe extern "C" fn()>,
    pub init_irq: Option<unsafe extern "C" fn()>,
    pub get_irq: Option<unsafe extern "C" fn() -> i32>,
    pub progress: Option<unsafe extern "C" fn(*const u8, u32)>,
}

extern "C" {
    static mut ppc_md: PpcMd;

    fn mpic_alloc(
        node: *mut c_void,
        flags: u32,
        senses: u32,
        irq_offset: u32,
        irq_count: u32,
        name: *const u8,
    ) -> *mut Mpic;
    fn mpic_init(mpic: *mut Mpic);
    fn of_find_compatible_node(
        from: *mut DeviceNode,
        type_: *const u8,
        compatible: *const u8,
    ) -> *mut DeviceNode;
    fn printk(format: *const u8, ...);
    fn socrates_fpga_pic_init(node: *mut DeviceNode);
    fn of_node_put(node: *mut DeviceNode);
    fn fsl_pci_assign_primary();
    fn mpc85xx_common_publish_devices() -> i32;
    fn mpic_get_irq() -> i32;
    fn udbg_progress(message: *const u8, value: u32);
}

const MPIC_BIG_ENDIAN: u32 = 0x0001;
const KERN_ERR: &[u8] = b"<3>";

unsafe extern "C" fn socrates_pic_init() {
    let mut np: *mut DeviceNode;

    let mpic = mpic_alloc(
        core::ptr::null_mut(),
        0,
        MPIC_BIG_ENDIAN,
        0,
        256,
        b" OpenPIC  \0".as_ptr(),
    );
    if mpic.is_null() {
        panic!("BUG_ON(mpic == NULL)");
    }
    mpic_init(mpic);

    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"abb,socrates-fpga-pic\0".as_ptr(),
    );
    if np.is_null() {
        printk(b"%sCould not find socrates-fpga-pic node\n\0".as_ptr(), KERN_ERR.as_ptr());
        return;
    }
    socrates_fpga_pic_init(np);
    of_node_put(np);
}

/*
 * Setup the architecture
 */
unsafe extern "C" fn socrates_setup_arch() {
    if let Some(progress) = ppc_md.progress {
        progress(b"socrates_setup_arch()\0".as_ptr(), 0);
    }

    fsl_pci_assign_primary();
}

// machine_arch_initcall(socrates, mpc85xx_common_publish_devices);
#[used]
#[link_section = ".initcall"]
static SOCRATES_ARCH_INITCALL: unsafe extern "C" fn() -> i32 = mpc85xx_common_publish_devices;

// define_machine(socrates)
#[no_mangle]
pub static mut MACHINE_SOCRATES: MachineDesc = MachineDesc {
    name: b"Socrates\0".as_ptr(),
    compatible: b"abb,socrates\0".as_ptr(),
    setup_arch: Some(socrates_setup_arch),
    init_irq: Some(socrates_pic_init),
    get_irq: Some(mpic_get_irq),
    progress: Some(udbg_progress),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
