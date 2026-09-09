// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * MX31 CPU type detection
 *
 * Copyright (c) 2009 Daniel Mack <daniel@caiaq.de>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/module.h, linux/of_address.h, linux/io.h, common.h, hardware.h, iim.h

use core::ffi::{c_char, c_void};

#[repr(C)]
struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
struct Mx31CpuType {
    srev: u8,
    name: *const c_char,
    rev: u32,
}

extern "C" {
    fn of_find_compatible_node(
        from: *mut DeviceNode,
        typ: *const c_char,
        compatible: *const c_char,
    ) -> *mut DeviceNode;
    fn of_iomap(np: *mut DeviceNode, index: i32) -> *mut c_void;
    fn of_node_put(np: *mut DeviceNode);
    fn imx_readl(addr: *const c_void) -> u32;
    fn imx_print_silicon_rev(name: *const c_char, rev: u32);
    fn iounmap(addr: *mut c_void);
}

// Constants supplied by hardware.h and iim.h.
extern "C" {
    static MXC_IIMSREV: usize;
    static IMX_CHIP_REVISION_UNKNOWN: u32;
    static IMX_CHIP_REVISION_1_0: u32;
    static IMX_CHIP_REVISION_1_1: u32;
    static IMX_CHIP_REVISION_1_2: u32;
    static IMX_CHIP_REVISION_2_0: u32;
}

static mut mx31_cpu_rev: i32 = -1;

static mx31_cpu_type: [Mx31CpuType; 9] = [
    Mx31CpuType { srev: 0x00, name: b"i.MX31(L)\0".as_ptr() as *const c_char, rev: unsafe { IMX_CHIP_REVISION_1_0 } },
    Mx31CpuType { srev: 0x10, name: b"i.MX31\0".as_ptr() as *const c_char, rev: unsafe { IMX_CHIP_REVISION_1_1 } },
    Mx31CpuType { srev: 0x11, name: b"i.MX31L\0".as_ptr() as *const c_char, rev: unsafe { IMX_CHIP_REVISION_1_1 } },
    Mx31CpuType { srev: 0x12, name: b"i.MX31\0".as_ptr() as *const c_char, rev: unsafe { IMX_CHIP_REVISION_1_1 } },
    Mx31CpuType { srev: 0x13, name: b"i.MX31L\0".as_ptr() as *const c_char, rev: unsafe { IMX_CHIP_REVISION_1_1 } },
    Mx31CpuType { srev: 0x14, name: b"i.MX31\0".as_ptr() as *const c_char, rev: unsafe { IMX_CHIP_REVISION_1_2 } },
    Mx31CpuType { srev: 0x15, name: b"i.MX31L\0".as_ptr() as *const c_char, rev: unsafe { IMX_CHIP_REVISION_1_2 } },
    Mx31CpuType { srev: 0x28, name: b"i.MX31\0".as_ptr() as *const c_char, rev: unsafe { IMX_CHIP_REVISION_2_0 } },
    Mx31CpuType { srev: 0x29, name: b"i.MX31L\0".as_ptr() as *const c_char, rev: unsafe { IMX_CHIP_REVISION_2_0 } },
];

unsafe fn mx31_read_cpu_rev() -> i32 {
    let iim_base: *mut c_void;
    let np: *mut DeviceNode;
    let mut srev: u32;
    let mut rev: i32 = IMX_CHIP_REVISION_UNKNOWN as i32;

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"fsl,imx31-iim\0".as_ptr() as *const c_char);
    iim_base = of_iomap(np, 0);
    of_node_put(np);
    assert!(!iim_base.is_null());

    // read SREV register from IIM module
    srev = imx_readl((iim_base as *mut u8).add(MXC_IIMSREV) as *const c_void);
    srev &= 0xff;

    for i in 0..mx31_cpu_type.len() {
        if srev == mx31_cpu_type[i].srev as u32 {
            rev = mx31_cpu_type[i].rev as i32;
            imx_print_silicon_rev(mx31_cpu_type[i].name, mx31_cpu_type[i].rev);
            break;
        }
    }

    if rev == IMX_CHIP_REVISION_UNKNOWN as i32 {
        imx_print_silicon_rev(b"i.MX31\0".as_ptr() as *const c_char, IMX_CHIP_REVISION_UNKNOWN);
    }

    iounmap(iim_base);
    rev
}

pub unsafe fn mx31_revision() -> i32 {
    if mx31_cpu_rev == -1 {
        mx31_cpu_rev = mx31_read_cpu_rev();
    }

    mx31_cpu_rev
}

// EXPORT_SYMBOL(mx31_revision);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
