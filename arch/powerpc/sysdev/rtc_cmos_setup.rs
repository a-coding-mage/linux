/*
 * Setup code for PC-style Real-Time Clock.
 *
 * Author: Wade Farnsworth <wfarnsworth@mvista.com>
 *
 * 2007 (c) MontaVista Software, Inc. This file is licensed under
 * the terms of the GNU General Public License version 2. This program
 * is licensed "as is" without any warranty of any kind, whether express
 * or implied.
 */

// Dependencies are supplied by the surrounding kernel translation.

use core::ffi::c_char;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct resource {
    pub start: usize,
    pub end: usize,
    pub name: *const c_char,
    pub flags: usize,
    pub desc: usize,
    pub parent: *mut resource,
    pub sibling: *mut resource,
    pub child: *mut resource,
}

extern "C" {
    fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    fn of_find_compatible_node(
        from: *mut device_node,
        type_: *const c_char,
        compatible: *const c_char,
    ) -> *mut device_node;
    fn of_address_to_resource(np: *mut device_node, index: i32, r: *mut resource) -> i32;
    fn of_node_put(np: *mut device_node);
    fn platform_device_register_simple(
        name: *const c_char,
        id: i32,
        res: *mut resource,
        num: u32,
    ) -> *mut platform_device;
    fn ptr_err_or_zero(ptr: *mut platform_device) -> i32;
    fn rtc_port(x: usize) -> usize;
}

const ENODEV: i32 = 19;
const EINVAL: i32 = 22;
const IORESOURCE_IRQ: usize = 0x00000400;

unsafe extern "C" fn add_rtc() -> i32 {
    let mut np: *mut device_node;
    let pd: *mut platform_device;
    let mut res: [resource; 2] = core::mem::zeroed();
    let mut num_res: u32 = 1;
    let ret: i32;

    memset(
        res.as_mut_ptr() as *mut core::ffi::c_void,
        0,
        core::mem::size_of::<[resource; 2]>(),
    );

    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"pnpPNP,b00\0".as_ptr() as *const c_char,
    );
    if np.is_null() {
        return -ENODEV;
    }

    ret = of_address_to_resource(np, 0, &mut res[0]);
    of_node_put(np);
    if ret != 0 {
        return ret;
    }

    /*
     * RTC_PORT(x) is hardcoded in asm/mc146818rtc.h. Verify that the
     * address provided by the device node matches.
     */
    if res[0].start != rtc_port(0) {
        return -EINVAL;
    }

    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"chrp,iic\0".as_ptr() as *const c_char,
    );
    if np.is_null() {
        np = of_find_compatible_node(
            core::ptr::null_mut(),
            core::ptr::null(),
            b"pnpPNP,000\0".as_ptr() as *const c_char,
        );
    }
    if !np.is_null() {
        of_node_put(np);
        /*
         * Use a fixed interrupt value of 8 since on PPC if we are
         * using this its off an i8259 which we ensure has interrupt
         * numbers 0..15.
         */
        res[1].start = 8;
        res[1].end = 8;
        res[1].flags = IORESOURCE_IRQ;
        num_res += 1;
    }

    pd = platform_device_register_simple(b"rtc_cmos\0".as_ptr() as *const c_char, -1, &mut res[0], num_res);

    ptr_err_or_zero(pd)
}

// Equivalent to fs_initcall(add_rtc).

// MODULE_DESCRIPTION("PPC RTC CMOS driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
