/*
 * mcf8390.c  -- platform support for 8390 ethernet on many boards
 *
 * (C) Copyright 2012, Greg Ungerer <gerg@uclinux.org>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the kernel and architecture headers are expected
// to provide `resource`, `IORESOURCE_MEM`, `IORESOURCE_IRQ`, `NE2000_ADDR`,
// `NE2000_ADDRSIZE`, `NE2000_IRQ_VECTOR`, and `ARRAY_SIZE`.

extern "C" {
    fn platform_device_register_simple(
        name: *const core::ffi::c_char,
        id: i32,
        resource: *mut resource,
        num: usize,
    ) -> *mut core::ffi::c_void;
}

#[repr(C)]
pub struct resource {
    pub start: usize,
    pub end: usize,
    pub flags: usize,
}

const IORESOURCE_MEM: usize = 0x0000_0200;
const IORESOURCE_IRQ: usize = 0x0000_0400;

// NE2000_ADDR, NE2000_ADDRSIZE, and NE2000_IRQ_VECTOR are architecture-
// supplied constants.  The kernel's ARRAY_SIZE macro is represented by the
// array length directly below.

static mut MCF8390_RESOURCES: [resource; 2] = [
    resource {
        start: NE2000_ADDR,
        end: NE2000_ADDR + NE2000_ADDRSIZE - 1,
        flags: IORESOURCE_MEM,
    },
    resource {
        start: NE2000_IRQ_VECTOR,
        end: NE2000_IRQ_VECTOR,
        flags: IORESOURCE_IRQ,
    },
];

#[allow(non_snake_case)]
unsafe fn mcf8390_platform_init() -> i32 {
    static NAME: &[u8] = b"mcf8390\0";

    platform_device_register_simple(
        NAME.as_ptr() as *const core::ffi::c_char,
        -1,
        MCF8390_RESOURCES.as_mut_ptr(),
        MCF8390_RESOURCES.len(),
    );
    0
}

// Equivalent of arch_initcall(mcf8390_platform_init).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
