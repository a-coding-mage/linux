/*
 * SATA glue for Cavium Octeon III SOCs.
 *
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2010-2015 Cavium Networks
 *
 */

// Dependencies supplied by the surrounding kernel bindings:
// linux/module.h, linux/dma-mapping.h, linux/platform_device.h,
// linux/of_platform.h, and asm/octeon/octeon.h.

const CVMX_SATA_UCTL_SHIM_CFG: usize = 0xE8;

const SATA_UCTL_ENDIAN_MODE_BIG: u64 = 1;
const SATA_UCTL_ENDIAN_MODE_LITTLE: u64 = 0;
const SATA_UCTL_ENDIAN_MODE_MASK: u64 = 3;

const SATA_UCTL_DMA_ENDIAN_MODE_SHIFT: u32 = 8;
const SATA_UCTL_CSR_ENDIAN_MODE_SHIFT: u32 = 0;
const SATA_UCTL_DMA_READ_CMD_SHIFT: u32 = 12;

unsafe extern "C" {
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: u32) -> *mut core::ffi::c_void;
    fn cvmx_readq_csr(addr: *mut core::ffi::c_void) -> u64;
    fn cvmx_writeq_csr(addr: *mut core::ffi::c_void, value: u64);
    fn of_platform_populate(
        node: *mut device_node,
        matches: *const core::ffi::c_void,
        lookup: *const core::ffi::c_void,
        parent: *mut device,
    ) -> i32;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn PTR_ERR(ptr: *mut core::ffi::c_void) -> i32;
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node;

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub driver: device_driver,
}

unsafe fn ahci_octeon_probe(pdev: *mut platform_device) -> i32 {
    let dev: *mut device = &mut (*pdev).dev;
    let node: *mut device_node = (*dev).of_node;
    let base = devm_platform_ioremap_resource(pdev, 0);
    if (base as isize) < 0 {
        return PTR_ERR(base);
    }

    let cfg_addr = base.add(CVMX_SATA_UCTL_SHIM_CFG);
    let mut cfg = cvmx_readq_csr(cfg_addr);

    cfg &= !(SATA_UCTL_ENDIAN_MODE_MASK << SATA_UCTL_DMA_ENDIAN_MODE_SHIFT);
    cfg &= !(SATA_UCTL_ENDIAN_MODE_MASK << SATA_UCTL_CSR_ENDIAN_MODE_SHIFT);

    // The C build selects the endian mode with __BIG_ENDIAN.
    #[cfg(target_endian = "big")]
    {
        cfg |= SATA_UCTL_ENDIAN_MODE_BIG << SATA_UCTL_DMA_ENDIAN_MODE_SHIFT;
        cfg |= SATA_UCTL_ENDIAN_MODE_BIG << SATA_UCTL_CSR_ENDIAN_MODE_SHIFT;
    }
    #[cfg(not(target_endian = "big"))]
    {
        cfg |= SATA_UCTL_ENDIAN_MODE_LITTLE << SATA_UCTL_DMA_ENDIAN_MODE_SHIFT;
        cfg |= SATA_UCTL_ENDIAN_MODE_LITTLE << SATA_UCTL_CSR_ENDIAN_MODE_SHIFT;
    }

    cfg |= 1u64 << SATA_UCTL_DMA_READ_CMD_SHIFT;

    cvmx_writeq_csr(cfg_addr, cfg);

    if node.is_null() {
        dev_err(dev, b"no device node, failed to add octeon sata\0".as_ptr() as *const _);
        return -19;
    }

    let ret = of_platform_populate(node, core::ptr::null(), core::ptr::null(), dev);
    if ret != 0 {
        dev_err(dev, b"failed to add ahci-platform core\0".as_ptr() as *const _);
        return ret;
    }

    0
}

static OCTEON_AHCI_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: b"cavium,octeon-7130-sata-uctl\0".as_ptr() as *const _ },
    of_device_id { compatible: core::ptr::null() },
];

static mut AHCI_OCTEON_DRIVER: platform_driver = platform_driver {
    probe: Some(ahci_octeon_probe),
    driver: device_driver {
        name: b"octeon-ahci\0".as_ptr() as *const _,
        of_match_table: OCTEON_AHCI_MATCH.as_ptr(),
    },
};

// Equivalent to module_platform_driver(ahci_octeon_driver).

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Cavium, Inc. <support@cavium.com>");
// MODULE_DESCRIPTION("Cavium Inc. sata config.");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
