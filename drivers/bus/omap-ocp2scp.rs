// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * omap-ocp2scp.c - transform ocp interface protocol to scp protocol
 *
 * Copyright (C) 2012 Texas Instruments Incorporated - http://www.ti.com
 * Author: Kishon Vijay Abraham I <kishon@ti.com>
 */

// Linux dependencies supplied by the surrounding kernel translation.

const OCP2SCP_TIMING: usize = 0x18;
const SYNC2_MASK: u32 = 0xf;

extern "C" {
    fn of_platform_populate(
        node: *mut device_node,
        matches: *const core::ffi::c_void,
        lookup: *const core::ffi::c_void,
        parent: *mut device,
    ) -> i32;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn pm_runtime_enable(dev: *mut device);
    fn of_device_is_compatible(node: *mut device_node, compatible: *const core::ffi::c_char) -> bool;
    fn platform_get_resource(pdev: *mut platform_device, resource_type: u32, index: u32) -> *mut resource;
    fn devm_ioremap_resource(dev: *mut device, res: *mut resource) -> *mut core::ffi::c_void;
    fn is_err(ptr: *mut core::ffi::c_void) -> bool;
    fn ptr_err(ptr: *mut core::ffi::c_void) -> i32;
    fn pm_runtime_get_sync(dev: *mut device) -> i32;
    fn readl_relaxed(addr: *mut u8) -> u32;
    fn writel_relaxed(value: u32, addr: *mut u8);
    fn pm_runtime_put_sync(dev: *mut device) -> i32;
    fn pm_runtime_disable(dev: *mut device);
    fn of_platform_depopulate(dev: *mut device);
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
    _private: [u8; 0],
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

const IORESOURCE_MEM: u32 = 0x0000_0200;

unsafe fn omap_ocp2scp_probe(pdev: *mut platform_device) -> i32 {
    let mut ret: i32;
    let mut reg: u32;
    let regs: *mut u8;
    let res: *mut resource;
    let np: *mut device_node;

    np = (*pdev).dev.of_node;

    if !np.is_null() {
        ret = of_platform_populate(
            np,
            core::ptr::null(),
            core::ptr::null(),
            &mut (*pdev).dev,
        );
        if ret != 0 {
            dev_err(&mut (*pdev).dev, b"failed to add resources for ocp2scp child\0".as_ptr() as *const _,);
            return ret;
        }
    }

    pm_runtime_enable(&mut (*pdev).dev);
    /*
     * As per AM572x TRM: http://www.ti.com/lit/ug/spruhz6/spruhz6.pdf
     * under section 26.3.2.2, table 26-26 OCP2SCP TIMING Caution;
     * As per OMAP4430 TRM: http://www.ti.com/lit/ug/swpu231ap/swpu231ap.pdf
     * under section 23.12.6.2.2 , Table 23-1213 OCP2SCP TIMING Caution;
     * As per OMAP4460 TRM: http://www.ti.com/lit/ug/swpu235ab/swpu235ab.pdf
     * under section 23.12.6.2.2, Table 23-1213 OCP2SCP TIMING Caution;
     * As per OMAP543x TRM http://www.ti.com/lit/pdf/swpu249
     * under section 27.3.2.2, Table 27-27 OCP2SCP TIMING Caution;
     *
     * Read path of OCP2SCP is not working properly due to low reset value
     * of SYNC2 parameter in OCP2SCP. Suggested reset value is 0x6 or more.
     */
    if !of_device_is_compatible(np, b"ti,am437x-ocp2scp\0".as_ptr() as *const _) {
        res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
        regs = devm_ioremap_resource(&mut (*pdev).dev, res) as *mut u8;
        if is_err(regs as *mut core::ffi::c_void) {
            ret = ptr_err(regs as *mut core::ffi::c_void);
            pm_runtime_disable(&mut (*pdev).dev);
            of_platform_depopulate(&mut (*pdev).dev);
            return ret;
        }

        pm_runtime_get_sync(&mut (*pdev).dev);
        reg = readl_relaxed(regs.add(OCP2SCP_TIMING));
        reg &= !SYNC2_MASK;
        reg |= 0x6;
        writel_relaxed(reg, regs.add(OCP2SCP_TIMING));
        pm_runtime_put_sync(&mut (*pdev).dev);
    }

    0
}

unsafe fn omap_ocp2scp_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
    of_platform_depopulate(&mut (*pdev).dev);
}

// CONFIG_OF declarations and module registration are supplied by the kernel build.
#[cfg(feature = "CONFIG_OF")]
#[repr(C)]
struct of_device_id {
    compatible: *const core::ffi::c_char,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    driver: driver,
}

#[repr(C)]
struct driver {
    name: *const core::ffi::c_char,
    of_match_table: *const of_device_id,
}

extern "C" {
    fn module_platform_driver(driver: *mut platform_driver);
}

// The C source uses the kernel's module_platform_driver() registration macro.
static mut omap_ocp2scp_driver: platform_driver = platform_driver {
    probe: Some(omap_ocp2scp_probe),
    remove: Some(omap_ocp2scp_remove),
    driver: driver {
        name: b"omap-ocp2scp\0".as_ptr() as *const _,
        of_match_table: core::ptr::null(),
    },
};

#[cfg(feature = "CONFIG_OF")]
static omap_ocp2scp_id_table: [of_device_id; 3] = [
    of_device_id { compatible: b"ti,omap-ocp2scp\0".as_ptr() as *const _ },
    of_device_id { compatible: b"ti,am437x-ocp2scp\0".as_ptr() as *const _ },
    of_device_id { compatible: core::ptr::null() },
];

// MODULE_DEVICE_TABLE(of, omap_ocp2scp_id_table);
// module_platform_driver(omap_ocp2scp_driver);
// MODULE_ALIAS("platform:omap-ocp2scp");
// MODULE_AUTHOR("Texas Instruments Inc.");
// MODULE_DESCRIPTION("OMAP OCP2SCP driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
