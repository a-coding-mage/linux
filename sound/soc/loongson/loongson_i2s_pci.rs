// SPDX-License-Identifier: GPL-2.0
//
// loongson_i2s_pci.c -- Loongson I2S controller driver
//
// Copyright (C) 2023 Loongson Technology Corporation Limited
// Author: Yingkun Meng <mengyingkun@loongson.cn>
//

// Dependencies from Linux, sound/soc, loongson_i2s.h, and loongson_dma.h are
// intentionally referenced as external items supplied by the surrounding tree.

const DRIVER_NAME: *const core::ffi::c_char = b"loongson-i2s-pci\0".as_ptr() as *const core::ffi::c_char;

extern "C" {
    static loongson_i2s_regmap_config: regmap_config;
    static loongson_i2s_idma_component: snd_soc_component_driver;
    static loongson_i2s_dai: snd_soc_dai_driver;
    static loongson_i2s_pm: dev_pm_ops;

    fn pcim_enable_device(pdev: *mut pci_dev) -> core::ffi::c_int;
    fn devm_kzalloc(
        dev: *mut device,
        size: usize,
        flags: gfp_t,
    ) -> *mut core::ffi::c_void;
    fn pci_set_drvdata(pdev: *mut pci_dev, data: *mut core::ffi::c_void);
    fn pcim_iomap_region(
        pdev: *mut pci_dev,
        bar: core::ffi::c_int,
        name: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_void;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut core::ffi::c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn pci_resource_start(pdev: *mut pci_dev, bar: core::ffi::c_int) -> resource_size_t;
    fn fwnode_irq_get_byname(
        fwnode: *const fwnode_handle,
        name: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn device_property_read_u32(
        dev: *mut device,
        propname: *const core::ffi::c_char,
        val: *mut u32,
    ) -> core::ffi::c_int;
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> core::ffi::c_int;
    fn regmap_write(map: *mut regmap, reg: core::ffi::c_uint, val: core::ffi::c_uint) -> core::ffi::c_int;
    fn udelay(usecs: core::ffi::c_ulong);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *const snd_soc_dai_driver,
        num_dai: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn pm_sleep_ptr(ptr: *const dev_pm_ops) -> *const dev_pm_ops;
    fn __module_pci_driver(driver: *mut pci_driver);
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_err_probe(
        dev: *mut device,
        err: core::ffi::c_int,
        fmt: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> core::ffi::c_int;
}

type gfp_t = core::ffi::c_uint;
type resource_size_t = u64;

const GFP_KERNEL: gfp_t = 0;
const ENODEV: core::ffi::c_int = 19;
const ENOMEM: core::ffi::c_int = 12;
const PCI_VENDOR_ID_LOONGSON: u32 = 0x0014;
const LS_I2S_TX_DATA: resource_size_t = 0;
const LS_I2S_TX_ORDER: usize = 0;
const LS_I2S_RX_DATA: resource_size_t = 0;
const LS_I2S_RX_ORDER: usize = 0;
const LS_I2S_CTRL: core::ffi::c_uint = 0;
const I2S_CTRL_RESET: core::ffi::c_uint = 0;

const fn DMA_BIT_MASK(nr: u32) -> u64 {
    if nr == 64 {
        !0u64
    } else {
        (1u64 << nr) - 1
    }
}

#[repr(C)]
struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
struct regmap_config {
    _private: [u8; 0],
}

#[repr(C)]
struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_component_driver {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dai_driver {
    _private: [u8; 0],
}

#[repr(C)]
struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
struct device {
    fwnode: *const fwnode_handle,
}

#[repr(C)]
struct pci_dev {
    dev: device,
    revision: u8,
}

#[repr(C)]
struct pci_device_id {
    vendor: u32,
    device: u32,
}

#[repr(C)]
struct pci_driver_inner {
    pm: *const dev_pm_ops,
}

#[repr(C)]
struct pci_driver {
    name: *const core::ffi::c_char,
    id_table: *const pci_device_id,
    probe: Option<
        unsafe extern "C" fn(
            pdev: *mut pci_dev,
            pid: *const pci_device_id,
        ) -> core::ffi::c_int,
    >,
    driver: pci_driver_inner,
}

#[repr(C)]
struct loongson_idma_data {
    dev_addr: resource_size_t,
    order_addr: *mut core::ffi::c_void,
    irq: core::ffi::c_int,
}

#[repr(C)]
struct loongson_i2s {
    rev_id: u8,
    dev: *mut device,
    reg_base: *mut core::ffi::c_void,
    regmap: *mut regmap,
    tx_dma_data: loongson_idma_data,
    rx_dma_data: loongson_idma_data,
    clk_rate: u32,
}

unsafe extern "C" fn loongson_i2s_pci_probe(
    pdev: *mut pci_dev,
    _pid: *const pci_device_id,
) -> core::ffi::c_int {
    let fwnode: *const fwnode_handle = (*pdev).dev.fwnode;
    let mut tx_data: *mut loongson_idma_data;
    let mut rx_data: *mut loongson_idma_data;
    let dev: *mut device = &mut (*pdev).dev;
    let i2s: *mut loongson_i2s;
    let mut ret: core::ffi::c_int;

    if pcim_enable_device(pdev) != 0 {
        dev_err(dev, b"pci_enable_device failed\n\0".as_ptr() as *const core::ffi::c_char);
        return -ENODEV;
    }

    i2s = devm_kzalloc(dev, core::mem::size_of::<loongson_i2s>(), GFP_KERNEL) as *mut loongson_i2s;
    if i2s.is_null() {
        return -ENOMEM;
    }

    (*i2s).rev_id = (*pdev).revision;
    (*i2s).dev = dev;
    pci_set_drvdata(pdev, i2s as *mut core::ffi::c_void);

    (*i2s).reg_base = pcim_iomap_region(pdev, 0, DRIVER_NAME);
    if IS_ERR((*i2s).reg_base as *const core::ffi::c_void) {
        dev_err(dev, b"iomap_region failed\n\0".as_ptr() as *const core::ffi::c_char);
        return PTR_ERR((*i2s).reg_base as *const core::ffi::c_void);
    }

    (*i2s).regmap = devm_regmap_init_mmio(dev, (*i2s).reg_base, &loongson_i2s_regmap_config);
    if IS_ERR((*i2s).regmap as *const core::ffi::c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*i2s).regmap as *const core::ffi::c_void),
            b"regmap_init_mmio failed\n\0".as_ptr() as *const core::ffi::c_char,
        );
    }

    tx_data = &mut (*i2s).tx_dma_data;
    rx_data = &mut (*i2s).rx_dma_data;

    (*tx_data).dev_addr = pci_resource_start(pdev, 0).wrapping_add(LS_I2S_TX_DATA);
    (*tx_data).order_addr = ((*i2s).reg_base as *mut u8).wrapping_add(LS_I2S_TX_ORDER) as *mut core::ffi::c_void;

    (*rx_data).dev_addr = pci_resource_start(pdev, 0).wrapping_add(LS_I2S_RX_DATA);
    (*rx_data).order_addr = ((*i2s).reg_base as *mut u8).wrapping_add(LS_I2S_RX_ORDER) as *mut core::ffi::c_void;

    (*tx_data).irq = fwnode_irq_get_byname(fwnode, b"tx\0".as_ptr() as *const core::ffi::c_char);
    if (*tx_data).irq < 0 {
        return dev_err_probe(
            dev,
            (*tx_data).irq,
            b"dma tx irq invalid\n\0".as_ptr() as *const core::ffi::c_char,
        );
    }

    (*rx_data).irq = fwnode_irq_get_byname(fwnode, b"rx\0".as_ptr() as *const core::ffi::c_char);
    if (*rx_data).irq < 0 {
        return dev_err_probe(
            dev,
            (*rx_data).irq,
            b"dma rx irq invalid\n\0".as_ptr() as *const core::ffi::c_char,
        );
    }

    ret = device_property_read_u32(
        dev,
        b"clock-frequency\0".as_ptr() as *const core::ffi::c_char,
        &mut (*i2s).clk_rate,
    );
    if ret != 0 {
        return dev_err_probe(
            dev,
            ret,
            b"clock-frequency property invalid\n\0".as_ptr() as *const core::ffi::c_char,
        );
    }

    dma_set_mask_and_coherent(dev, DMA_BIT_MASK(64));

    if (*i2s).rev_id == 1 {
        regmap_write((*i2s).regmap, LS_I2S_CTRL, I2S_CTRL_RESET);
        udelay(200);
    }

    ret = devm_snd_soc_register_component(
        dev,
        &loongson_i2s_idma_component,
        &loongson_i2s_dai,
        1,
    );
    if ret != 0 {
        return dev_err_probe(
            dev,
            ret,
            b"register DAI failed\n\0".as_ptr() as *const core::ffi::c_char,
        );
    }

    0
}

const fn PCI_DEVICE(vendor: u32, device: u32) -> pci_device_id {
    pci_device_id { vendor, device }
}

static loongson_i2s_ids: [pci_device_id; 2] = [
    PCI_DEVICE(PCI_VENDOR_ID_LOONGSON, 0x7a27),
    pci_device_id {
        vendor: 0,
        device: 0,
    },
];
// MODULE_DEVICE_TABLE(pci, loongson_i2s_ids);

static mut loongson_i2s_driver: pci_driver = pci_driver {
    name: DRIVER_NAME,
    id_table: loongson_i2s_ids.as_ptr(),
    probe: Some(loongson_i2s_pci_probe),
    driver: pci_driver_inner {
        pm: unsafe { pm_sleep_ptr(&loongson_i2s_pm) },
    },
};
// module_pci_driver(loongson_i2s_driver);

#[no_mangle]
unsafe extern "C" fn init_module() -> core::ffi::c_int {
    __module_pci_driver(&mut loongson_i2s_driver);
    0
}

// MODULE_DESCRIPTION("Loongson I2S Master Mode ASoC Driver");
// MODULE_AUTHOR("Loongson Technology Corporation Limited");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
