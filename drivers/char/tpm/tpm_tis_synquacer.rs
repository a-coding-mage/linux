// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020 Linaro Ltd.
 *
 * This device driver implements MMIO TPM on SynQuacer Platform.
 */

// Linux kernel dependencies and build-time configuration are supplied by the
// surrounding translation unit.

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct tpm_tis_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tpm_chip {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tpm_tis_phy_ops {
    pub read_bytes: unsafe extern "C" fn(
        data: *mut tpm_tis_data,
        addr: u32,
        len: u16,
        result: *mut u8,
        io_mode: tpm_tis_io_mode,
    ) -> i32,
    pub write_bytes: unsafe extern "C" fn(
        data: *mut tpm_tis_data,
        addr: u32,
        len: u16,
        value: *const u8,
        io_mode: tpm_tis_io_mode,
    ) -> i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum tpm_tis_io_mode {
    TPM_TIS_PHYS_8,
    TPM_TIS_PHYS_16,
    TPM_TIS_PHYS_32,
}

#[repr(C)]
pub struct tpm_tis_synquacer_info {
    pub res: resource,
    pub irq: i32,
}

#[repr(C)]
pub struct tpm_tis_synquacer_phy {
    pub priv_: tpm_tis_data,
    pub iobase: *mut core::ffi::c_void,
}

extern "C" {
    fn ioread8(addr: *mut core::ffi::c_void) -> u8;
    fn iowrite8(value: u8, addr: *mut core::ffi::c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_ioremap_resource(dev: *mut device, res: *mut resource) -> *mut core::ffi::c_void;
    fn tpm_tis_core_init(
        dev: *mut device,
        priv_: *mut tpm_tis_data,
        irq: i32,
        phy_ops: *const tpm_tis_phy_ops,
        acpi_handle: *mut core::ffi::c_void,
    ) -> i32;
    fn tpm_pm_suspend(dev: *mut device) -> i32;
    fn tpm_tis_resume(dev: *mut device) -> i32;
    fn platform_get_resource(pdev: *mut platform_device, resource_type: u32, index: u32) -> *mut resource;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char);
    fn dev_get_drvdata(dev: *mut device) -> *mut tpm_chip;
    fn tpm_chip_unregister(chip: *mut tpm_chip);
    fn tpm_tis_remove(chip: *mut tpm_chip);
    fn acpi_handle(dev: *mut device) -> *mut core::ffi::c_void;
}

const GFP_KERNEL: u32 = 0;
const IORESOURCE_MEM: u32 = 0;
const ENOMEM: i32 = 12;
const ENODEV: i32 = 19;
const EINVAL: i32 = 22;

#[inline]
unsafe fn to_tpm_tis_tcg_phy(data: *mut tpm_tis_data) -> *mut tpm_tis_synquacer_phy {
    (data as *mut u8).sub(core::mem::offset_of!(tpm_tis_synquacer_phy, priv_))
        as *mut tpm_tis_synquacer_phy
}

unsafe extern "C" fn tpm_tis_synquacer_read_bytes(
    data: *mut tpm_tis_data,
    addr: u32,
    mut len: u16,
    mut result: *mut u8,
    io_mode: tpm_tis_io_mode,
) -> i32 {
    let phy = to_tpm_tis_tcg_phy(data);
    match io_mode {
        tpm_tis_io_mode::TPM_TIS_PHYS_8 => {
            while len != 0 {
                *result = ioread8((*phy).iobase.add(addr as usize));
                result = result.add(1);
                len -= 1;
            }
        }
        tpm_tis_io_mode::TPM_TIS_PHYS_16 => {
            *result.add(1) = ioread8((*phy).iobase.add(addr as usize + 1));
            *result = ioread8((*phy).iobase.add(addr as usize));
        }
        tpm_tis_io_mode::TPM_TIS_PHYS_32 => {
            *result.add(3) = ioread8((*phy).iobase.add(addr as usize + 3));
            *result.add(2) = ioread8((*phy).iobase.add(addr as usize + 2));
            *result.add(1) = ioread8((*phy).iobase.add(addr as usize + 1));
            *result = ioread8((*phy).iobase.add(addr as usize));
        }
    }
    0
}

unsafe extern "C" fn tpm_tis_synquacer_write_bytes(
    data: *mut tpm_tis_data,
    addr: u32,
    mut len: u16,
    mut value: *const u8,
    io_mode: tpm_tis_io_mode,
) -> i32 {
    let phy = to_tpm_tis_tcg_phy(data);
    match io_mode {
        tpm_tis_io_mode::TPM_TIS_PHYS_8 => {
            while len != 0 {
                iowrite8(*value, (*phy).iobase.add(addr as usize));
                value = value.add(1);
                len -= 1;
            }
        }
        tpm_tis_io_mode::TPM_TIS_PHYS_16 => return -EINVAL,
        tpm_tis_io_mode::TPM_TIS_PHYS_32 => {
            /*
             * Due to the limitation of SPI controller on SynQuacer,
             * 16/32 bits access must be done in byte-wise and descending order.
             */
            iowrite8(*value.add(3), (*phy).iobase.add(addr as usize + 3));
            iowrite8(*value.add(2), (*phy).iobase.add(addr as usize + 2));
            iowrite8(*value.add(1), (*phy).iobase.add(addr as usize + 1));
            iowrite8(*value, (*phy).iobase.add(addr as usize));
        }
    }
    0
}

static tpm_tcg_bw: tpm_tis_phy_ops = tpm_tis_phy_ops {
    read_bytes: tpm_tis_synquacer_read_bytes,
    write_bytes: tpm_tis_synquacer_write_bytes,
};

unsafe fn tpm_tis_synquacer_init(
    dev: *mut device,
    tpm_info: *mut tpm_tis_synquacer_info,
) -> i32 {
    let phy = devm_kzalloc(dev, core::mem::size_of::<tpm_tis_synquacer_phy>(), GFP_KERNEL)
        as *mut tpm_tis_synquacer_phy;
    if phy.is_null() {
        return -ENOMEM;
    }
    (*phy).iobase = devm_ioremap_resource(dev, &mut (*tpm_info).res);
    tpm_tis_core_init(dev, &mut (*phy).priv_, (*tpm_info).irq, &tpm_tcg_bw, acpi_handle(dev))
}

unsafe extern "C" fn tpm_tis_synquacer_probe(pdev: *mut platform_device) -> i32 {
    let mut tpm_info: tpm_tis_synquacer_info = core::mem::zeroed();
    let res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if res.is_null() {
        dev_err(&mut (*pdev).dev, b"no memory resource defined\0".as_ptr() as *const _);
        return -ENODEV;
    }
    tpm_info.res = core::ptr::read(res);
    tpm_info.irq = -1;
    tpm_tis_synquacer_init(&mut (*pdev).dev, &mut tpm_info)
}

unsafe extern "C" fn tpm_tis_synquacer_remove(pdev: *mut platform_device) {
    let chip = dev_get_drvdata(&mut (*pdev).dev);
    tpm_chip_unregister(chip);
    tpm_tis_remove(chip);
}

// CONFIG_OF and CONFIG_ACPI conditionally provide the device match tables.
// MODULE_DEVICE_TABLE, SIMPLE_DEV_PM_OPS, module_platform_driver,
// MODULE_DESCRIPTION, and MODULE_LICENSE are kernel registration metadata.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
