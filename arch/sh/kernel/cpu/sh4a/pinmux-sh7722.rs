// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_ulong};

// Corresponds to struct resource from <linux/ioport.h>.
#[repr(C)]
pub struct Resource {
    pub start: u64,
    pub end: u64,
    pub name: *const c_char,
    pub flags: c_ulong,
    pub desc: c_ulong,
    pub parent: *mut Resource,
    pub sibling: *mut Resource,
    pub child: *mut Resource,
}

// Corresponds to IORESOURCE_MEM from <linux/ioport.h>.
const IORESOURCE_MEM: c_ulong = 0x0000_0200;

static mut sh7722_pfc_resources: [Resource; 1] = [Resource {
    start: 0xa4050100,
    end: 0xa405018f,
    name: core::ptr::null(),
    flags: IORESOURCE_MEM,
    desc: 0,
    parent: core::ptr::null_mut(),
    sibling: core::ptr::null_mut(),
    child: core::ptr::null_mut(),
}];

unsafe extern "C" {
    fn sh_pfc_register(
        name: *const c_char,
        resources: *mut Resource,
        num_resources: usize,
    ) -> c_int;
}

// __init / arch_initcall(plat_pinmux_setup) are build-time kernel annotations.
unsafe fn plat_pinmux_setup() -> c_int {
    sh_pfc_register(
        b"pfc-sh7722\0".as_ptr() as *const c_char,
        sh7722_pfc_resources.as_mut_ptr(),
        sh7722_pfc_resources.len(),
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
