// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ppa8548 setup and early boot code.
 *
 * Copyright 2009 Prodrive B.V..
 *
 * By Stef van Os (see MAINTAINERS for contact information)
 *
 * Based on the SBC8548 support - Copyright 2007 Wind River Systems Inc.
 * Based on the MPC8548CDS support - Copyright 2005 Freescale Inc.
 */

// Dependencies supplied by the surrounding kernel translation.

extern "C" {
    fn mpic_alloc(
        node: *mut core::ffi::c_void,
        flags: i32,
        flags2: i32,
        offset: i32,
        irq_count: i32,
        name: *const core::ffi::c_char,
    ) -> *mut mpic;
    fn mpic_init(mpic: *mut mpic);
    fn mfspr(spr: u32) -> u32;
    fn seq_printf(m: *mut seq_file, format: *const core::ffi::c_char, ...);
    fn of_platform_bus_probe(
        node: *mut core::ffi::c_void,
        matches: *const of_device_id,
        parent: *mut core::ffi::c_void,
    );
    fn mpic_get_irq() -> i32;
    fn udbg_progress(message: *const core::ffi::c_char, value: u32);
}

#[repr(C)]
pub struct mpic {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub name: *const core::ffi::c_char,
    pub type_: *const core::ffi::c_char,
    pub compatible: *const core::ffi::c_char,
}

const MPIC_BIG_ENDIAN: i32 = 1 << 1;
const SPRN_SVR: u32 = 0x3e1;
const SPRN_HID1: u32 = 0x3f1;

unsafe fn ppa8548_pic_init() {
    let mpic = mpic_alloc(
        core::ptr::null_mut(),
        0,
        MPIC_BIG_ENDIAN,
        0,
        256,
        b" OpenPIC  \0".as_ptr() as *const core::ffi::c_char,
    );
    if mpic.is_null() {
        panic!("BUG_ON(mpic == NULL)");
    }
    mpic_init(mpic);
}

/*
 * Setup the architecture
 */
unsafe fn ppa8548_setup_arch() {
    if !ppc_md.progress.is_none() {
        (ppc_md.progress.unwrap())(
            b"ppa8548_setup_arch()\0".as_ptr() as *const core::ffi::c_char,
            0,
        );
    }
}

unsafe fn ppa8548_show_cpuinfo(m: *mut seq_file) {
    let svid: u32;
    let phid1: u32;

    svid = mfspr(SPRN_SVR);

    seq_printf(m, b"Vendor\t\t: Prodrive B.V.\n\0".as_ptr() as *const core::ffi::c_char);
    seq_printf(m, b"SVR\t\t: 0x%x\n\0".as_ptr() as *const core::ffi::c_char, svid);

    /* Display cpu Pll setting */
    phid1 = mfspr(SPRN_HID1);
    seq_printf(
        m,
        b"PLL setting\t: 0x%x\n\0".as_ptr() as *const core::ffi::c_char,
        (phid1 >> 24) & 0x3f,
    );
}

static OF_BUS_IDS: [of_device_id; 6] = [
    of_device_id { name: b"soc\0".as_ptr() as *const _, type_: core::ptr::null(), compatible: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: b"soc\0".as_ptr() as *const _, compatible: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: b"simple-bus\0".as_ptr() as *const _ },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: b"gianfar\0".as_ptr() as *const _ },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: b"fsl,srio\0".as_ptr() as *const _ },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: core::ptr::null() },
];

unsafe fn declare_of_platform_devices() -> i32 {
    of_platform_bus_probe(core::ptr::null_mut(), OF_BUS_IDS.as_ptr(), core::ptr::null_mut());
    0
}

// Equivalent of: machine_device_initcall(ppa8548, declare_of_platform_devices);
machine_device_initcall!(ppa8548, declare_of_platform_devices);

// Equivalent of the kernel's define_machine(ppa8548) initializer.
define_machine!(ppa8548 {
    name: "ppa8548",
    compatible: "ppa8548",
    setup_arch: ppa8548_setup_arch,
    init_IRQ: ppa8548_pic_init,
    show_cpuinfo: ppa8548_show_cpuinfo,
    get_irq: mpic_get_irq,
    progress: udbg_progress,
});

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
