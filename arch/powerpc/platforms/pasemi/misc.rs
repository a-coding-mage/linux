// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2007 PA Semi, Inc
 *
 * Parts based on arch/powerpc/sysdev/fsl_soc.c:
 *
 * 2006 (c) MontaVista Software, Inc.
 */

// The following declarations are supplied by the corresponding kernel
// dependencies.  The CONFIG_I2C_BOARDINFO condition is preserved below.

#[repr(C)]
pub struct i2c_driver_device {
    pub of_device: *mut core::ffi::c_char,
    pub i2c_type: *mut core::ffi::c_char,
}

#[cfg(CONFIG_I2C_BOARDINFO)]
static mut i2c_devices: [i2c_driver_device; 1] = [i2c_driver_device {
    of_device: b"dallas,ds1338\0" as *const u8 as *mut core::ffi::c_char,
    i2c_type: b"ds1338\0" as *const u8 as *mut core::ffi::c_char,
}];

#[cfg(CONFIG_I2C_BOARDINFO)]
unsafe fn find_i2c_driver(
    node: *mut device_node,
    info: *mut i2c_board_info,
) -> i32 {
    let mut i: usize = 0;

    while i < i2c_devices.len() {
        if of_device_is_compatible(node, i2c_devices[i].of_device) == 0 {
            i += 1;
            continue;
        }
        if strscpy(
            (*info).type_.as_mut_ptr(),
            i2c_devices[i].i2c_type,
            I2C_NAME_SIZE,
        ) < 0 {
            return -ENOMEM;
        }
        return 0;
    }
    -ENODEV
}

#[cfg(CONFIG_I2C_BOARDINFO)]
unsafe fn pasemi_register_i2c_devices() -> i32 {
    let mut pdev: *mut pci_dev = core::ptr::null_mut();

    while {
        pdev = pci_get_device(PCI_VENDOR_ID_PASEMI, 0xa003, pdev);
        !pdev.is_null()
    } {
        let adap_node: *mut device_node = pci_device_to_OF_node(pdev);

        if adap_node.is_null() {
            continue;
        }

        let mut node: *mut device_node = core::ptr::null_mut();
        while {
            node = for_each_child_of_node(adap_node, node);
            !node.is_null()
        } {
            let mut info: i2c_board_info = core::mem::zeroed();
            let mut len: i32 = 0;
            let addr: *const u32 = of_get_property(node, b"reg\0".as_ptr(), &mut len);

            if addr.is_null()
                || len < core::mem::size_of::<i32>() as i32
                || *addr > (1 << 10) - 1
            {
                pr_warn!("pasemi_register_i2c_devices: invalid i2c device entry\n");
                continue;
            }

            info.irq = irq_of_parse_and_map(node, 0);
            if info.irq == 0 {
                info.irq = -1;
            }

            if find_i2c_driver(node, &mut info) < 0 {
                continue;
            }

            info.addr = *addr as u16;
            i2c_register_board_info((*pdev).devfn as u32 & 7, &info, 1);
        }
    }
    0
}

#[cfg(CONFIG_I2C_BOARDINFO)]
device_initcall!(pasemi_register_i2c_devices);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
