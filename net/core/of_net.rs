// SPDX-License-Identifier: GPL-2.0-only
/*
 * OF helpers for network devices.
 *
 * Initially copied out of arch/powerpc/kernel/prom_parse.c
 */

use core::ffi::{c_char, c_int, c_void};

// Linux kernel types and declarations supplied by other translation units.
#[repr(C)]
pub struct device_node { _private: [u8; 0] }
#[repr(C)]
pub struct net_device { _private: [u8; 0] }
#[repr(C)]
pub struct nvmem_cell { _private: [u8; 0] }
#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct property {
    pub value: *const c_void,
    pub length: u32,
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

pub type phy_interface_t = c_int;
pub const ETH_ALEN: usize = 6;
pub const PHY_INTERFACE_MODE_NA: phy_interface_t = 0;
pub const PHY_INTERFACE_MODE_MAX: phy_interface_t = 0;
pub const ENODEV: c_int = 19;
pub const EINVAL: c_int = 22;

extern "C" {
    fn of_property_read_string(
        np: *mut device_node,
        propname: *const c_char,
        out_string: *mut *const c_char,
    ) -> c_int;
    fn phy_modes(mode: phy_interface_t) -> *const c_char;
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn of_find_property(
        np: *mut device_node,
        name: *const c_char,
        lenp: *mut u32,
    ) -> *mut property;
    fn is_valid_ether_addr(addr: *const c_void) -> bool;
    fn memcpy(dest: *mut c_void, src: *const c_void, count: usize) -> *mut c_void;
    fn of_find_device_by_node(np: *mut device_node) -> *mut platform_device;
    fn nvmem_get_mac_address(dev: *mut device, addr: *mut u8) -> c_int;
    fn put_device(dev: *mut device);
    fn of_nvmem_cell_get(np: *mut device_node, id: *const c_char) -> *mut nvmem_cell;
    fn nvmem_cell_read(cell: *mut nvmem_cell, len: *mut usize) -> *const c_void;
    fn nvmem_cell_put(cell: *mut nvmem_cell);
    fn kfree(ptr: *const c_void);
    fn ptr_err(ptr: *const c_void) -> c_int;
    fn is_err(ptr: *const c_void) -> bool;
    fn eth_hw_addr_set(dev: *mut net_device, addr: *const u8);
}

/// Get phy mode for given device_node.
#[no_mangle]
pub unsafe extern "C" fn of_get_phy_mode(
    np: *mut device_node,
    interface: *mut phy_interface_t,
) -> c_int {
    let mut pm: *const c_char = core::ptr::null();
    let mut err: c_int;

    *interface = PHY_INTERFACE_MODE_NA;

    err = of_property_read_string(np, b"phy-mode\0".as_ptr() as *const c_char, &mut pm);
    if err < 0 {
        err = of_property_read_string(
            np,
            b"phy-connection-type\0".as_ptr() as *const c_char,
            &mut pm,
        );
    }
    if err < 0 {
        return err;
    }

    let mut i: phy_interface_t = 0;
    while i < PHY_INTERFACE_MODE_MAX {
        if strcasecmp(pm, phy_modes(i)) == 0 {
            *interface = i;
            return 0;
        }
        i += 1;
    }

    -ENODEV
}

unsafe fn of_get_mac_addr(
    np: *mut device_node,
    name: *const c_char,
    addr: *mut u8,
) -> c_int {
    let pp = of_find_property(np, name, core::ptr::null_mut());

    if !pp.is_null()
        && (*pp).length as usize == ETH_ALEN
        && is_valid_ether_addr((*pp).value)
    {
        memcpy(addr as *mut c_void, (*pp).value, ETH_ALEN);
        return 0;
    }
    -ENODEV
}

#[no_mangle]
pub unsafe extern "C" fn of_get_mac_address_nvmem(
    np: *mut device_node,
    addr: *mut u8,
) -> c_int {
    let pdev = of_find_device_by_node(np);
    let cell: *mut nvmem_cell;
    let mac: *const c_void;
    let mut len: usize = 0;
    let ret: c_int;

    if !pdev.is_null() {
        ret = nvmem_get_mac_address(&mut (*pdev).dev, addr);
        put_device(&mut (*pdev).dev);
        return ret;
    }

    cell = of_nvmem_cell_get(np, b"mac-address\0".as_ptr() as *const c_char);
    if is_err(cell as *const c_void) {
        return ptr_err(cell as *const c_void);
    }

    mac = nvmem_cell_read(cell, &mut len);
    nvmem_cell_put(cell);

    if is_err(mac) {
        return ptr_err(mac);
    }

    if len != ETH_ALEN || !is_valid_ether_addr(mac) {
        kfree(mac);
        return -EINVAL;
    }

    memcpy(addr as *mut c_void, mac, ETH_ALEN);
    kfree(mac);

    0
}

#[no_mangle]
pub unsafe extern "C" fn of_get_mac_address(
    np: *mut device_node,
    addr: *mut u8,
) -> c_int {
    let mut ret: c_int;

    if np.is_null() {
        return -ENODEV;
    }

    ret = of_get_mac_addr(np, b"mac-address\0".as_ptr() as *const c_char, addr);
    if ret == 0 {
        return 0;
    }

    ret = of_get_mac_addr(np, b"local-mac-address\0".as_ptr() as *const c_char, addr);
    if ret == 0 {
        return 0;
    }

    ret = of_get_mac_addr(np, b"address\0".as_ptr() as *const c_char, addr);
    if ret == 0 {
        return 0;
    }

    of_get_mac_address_nvmem(np, addr)
}

#[no_mangle]
pub unsafe extern "C" fn of_get_ethdev_address(
    np: *mut device_node,
    dev: *mut net_device,
) -> c_int {
    let mut addr = [0u8; ETH_ALEN];
    let ret = of_get_mac_address(np, addr.as_mut_ptr());
    if ret == 0 {
        eth_hw_addr_set(dev, addr.as_ptr());
    }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
