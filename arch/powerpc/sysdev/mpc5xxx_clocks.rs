// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation:
// linux/kernel.h, linux/export.h, linux/property.h, and asm/mpc5xxx.h

use core::ffi::{c_char, c_ulong};

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

extern "C" {
    fn fwnode_property_read_u32(
        fwnode: *const fwnode_handle,
        propname: *const c_char,
        val: *mut u32,
    ) -> i32;
    fn fwnode_get_parent(fwnode: *const fwnode_handle) -> *mut fwnode_handle;
    fn fwnode_handle_put(fwnode: *mut fwnode_handle);
}

/**
 * mpc5xxx_fwnode_get_bus_frequency - Find the bus frequency for a firmware node
 * @fwnode: firmware node
 *
 * Returns bus frequency (IPS on MPC512x, IPB on MPC52xx),
 * or 0 if the bus frequency cannot be found.
 */
#[no_mangle]
pub unsafe extern "C" fn mpc5xxx_fwnode_get_bus_frequency(
    fwnode: *mut fwnode_handle,
) -> c_ulong {
    let mut parent: *mut fwnode_handle;
    let mut bus_freq: u32 = 0;
    let mut ret: i32;

    ret = fwnode_property_read_u32(
        fwnode,
        b"bus-frequency\0".as_ptr() as *const c_char,
        &mut bus_freq,
    );
    if ret == 0 {
        return bus_freq as c_ulong;
    }

    parent = fwnode_get_parent(fwnode);
    while !parent.is_null() {
        ret = fwnode_property_read_u32(
            parent,
            b"bus-frequency\0".as_ptr() as *const c_char,
            &mut bus_freq,
        );
        if ret == 0 {
            fwnode_handle_put(parent);
            return bus_freq as c_ulong;
        }

        let next = fwnode_get_parent(parent);
        fwnode_handle_put(parent);
        parent = next;
    }

    0
}

// EXPORT_SYMBOL(mpc5xxx_fwnode_get_bus_frequency);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
