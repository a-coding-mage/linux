// SPDX-License-Identifier: GPL-2.0
/*
 * Intel Merrifield SoC GPIO driver
 *
 * Copyright (c) 2016, 2023 Intel Corporation.
 * Author: Andy Shevchenko <andriy.shevchenko@linux.intel.com>
 */

// External Linux kernel and gpio-tangier declarations are supplied by the
// surrounding kernel translation environment.

/* Intel Merrifield has 192 GPIO pins */
const MRFLD_NGPIO: u32 = 192;

#[repr(C)]
struct TngGpioPinrange {
    pin_base: u32,
    npins: u32,
    gpio_base: u32,
}

// GPIO_PINRANGE(start, end, gpio) expands to a range with an inclusive end.
const MRFIELD_GPIO_RANGES: [TngGpioPinrange; 26] = [
    TngGpioPinrange { pin_base: 0, npins: 12, gpio_base: 146 },
    TngGpioPinrange { pin_base: 12, npins: 2, gpio_base: 144 },
    TngGpioPinrange { pin_base: 14, npins: 2, gpio_base: 35 },
    TngGpioPinrange { pin_base: 16, npins: 1, gpio_base: 164 },
    TngGpioPinrange { pin_base: 17, npins: 2, gpio_base: 105 },
    TngGpioPinrange { pin_base: 19, npins: 4, gpio_base: 101 },
    TngGpioPinrange { pin_base: 23, npins: 8, gpio_base: 107 },
    TngGpioPinrange { pin_base: 32, npins: 12, gpio_base: 67 },
    TngGpioPinrange { pin_base: 44, npins: 20, gpio_base: 195 },
    TngGpioPinrange { pin_base: 64, npins: 4, gpio_base: 140 },
    TngGpioPinrange { pin_base: 68, npins: 2, gpio_base: 165 },
    TngGpioPinrange { pin_base: 70, npins: 2, gpio_base: 65 },
    TngGpioPinrange { pin_base: 72, npins: 5, gpio_base: 228 },
    TngGpioPinrange { pin_base: 77, npins: 10, gpio_base: 37 },
    TngGpioPinrange { pin_base: 87, npins: 1, gpio_base: 48 },
    TngGpioPinrange { pin_base: 88, npins: 1, gpio_base: 47 },
    TngGpioPinrange { pin_base: 89, npins: 8, gpio_base: 49 },
    TngGpioPinrange { pin_base: 97, npins: 1, gpio_base: 34 },
    TngGpioPinrange { pin_base: 102, npins: 18, gpio_base: 83 },
    TngGpioPinrange { pin_base: 120, npins: 4, gpio_base: 79 },
    TngGpioPinrange { pin_base: 124, npins: 12, gpio_base: 115 },
    TngGpioPinrange { pin_base: 137, npins: 6, gpio_base: 158 },
    TngGpioPinrange { pin_base: 154, npins: 10, gpio_base: 24 },
    TngGpioPinrange { pin_base: 164, npins: 13, gpio_base: 215 },
    TngGpioPinrange { pin_base: 177, npins: 13, gpio_base: 127 },
    TngGpioPinrange { pin_base: 190, npins: 2, gpio_base: 178 },
];

unsafe fn mrfld_gpio_get_pinctrl_dev_name(priv_: *mut TngGpio) -> *const core::ffi::c_char {
    let dev = (*priv_).dev;
    let adev = acpi_dev_get_first_match_dev(b"INTC1002\0".as_ptr() as *const _, core::ptr::null(), -1);
    let name;
    if !adev.is_null() {
        name = devm_kstrdup(dev, acpi_dev_name(adev), GFP_KERNEL);
        acpi_dev_put(adev);
    } else {
        name = b"pinctrl-merrifield\0".as_ptr() as *const _;
    }
    name
}

unsafe fn mrfld_gpio_probe(pdev: *mut PciDev, _id: *const PciDeviceId) -> i32 {
    let dev = &mut (*pdev).dev;
    let mut priv_: *mut TngGpio;
    let mut gpio_base: u32;
    let mut irq_base: u32;
    let base: *mut core::ffi::c_void;
    let mut retval: i32;

    retval = pcim_enable_device(pdev);
    if retval != 0 { return retval; }
    base = pcim_iomap_region(pdev, 1, pci_name(pdev));
    if IS_ERR(base) { return dev_err_probe(dev, PTR_ERR(base), b"I/O memory mapping error\n".as_ptr() as *const _); }
    irq_base = readl(base.add(0 * core::mem::size_of::<u32>()) as *const _);
    gpio_base = readl(base.add(1 * core::mem::size_of::<u32>()) as *const _);
    pcim_iounmap_region(pdev, 1);
    priv_ = devm_kzalloc(dev, core::mem::size_of::<TngGpio>(), GFP_KERNEL);
    if priv_.is_null() { return -12; }
    (*priv_).dev = dev;
    (*priv_).reg_base = pcim_iomap_region(pdev, 0, pci_name(pdev));
    if IS_ERR((*priv_).reg_base) { return dev_err_probe(dev, PTR_ERR((*priv_).reg_base), b"I/O memory mapping error\n".as_ptr() as *const _); }
    (*priv_).pin_info.pin_ranges = MRFIELD_GPIO_RANGES.as_ptr();
    (*priv_).pin_info.nranges = MRFIELD_GPIO_RANGES.len();
    (*priv_).pin_info.name = mrfld_gpio_get_pinctrl_dev_name(priv_);
    if (*priv_).pin_info.name.is_null() { return -12; }
    (*priv_).info.base = gpio_base;
    (*priv_).info.ngpio = MRFLD_NGPIO;
    (*priv_).info.first = irq_base;
    retval = pci_alloc_irq_vectors(pdev, 1, 1, PCI_IRQ_ALL_TYPES);
    if retval < 0 { return retval; }
    (*priv_).irq = pci_irq_vector(pdev, 0);
    (*priv_).wake_regs.gwmr = GWMR_MRFLD;
    (*priv_).wake_regs.gwsr = GWSR_MRFLD;
    (*priv_).wake_regs.gsir = GSIR_MRFLD;
    retval = devm_tng_gpio_probe(dev, priv_);
    if retval != 0 { return dev_err_probe(dev, retval, b"tng_gpio_probe error\n".as_ptr() as *const _); }
    pci_set_drvdata(pdev, priv_);
    0
}

#[repr(C)]
struct PciDeviceId {
    vendor: u16,
    device: u16,
}

static MRFIELD_GPIO_IDS: [PciDeviceId; 2] = [
    PciDeviceId { vendor: 0x8086, device: 0x1199 },
    PciDeviceId { vendor: 0, device: 0 },
];

// Equivalent of:
// static struct pci_driver mrfld_gpio_driver = {
//     .name = "gpio-merrifield", .id_table = mrfld_gpio_ids,
//     .probe = mrfld_gpio_probe,
// };
// module_pci_driver(mrfld_gpio_driver);
// MODULE_DEVICE_TABLE(pci, mrfld_gpio_ids);
// MODULE_AUTHOR("Andy Shevchenko <andriy.shevchenko@linux.intel.com>");
// MODULE_DESCRIPTION("Intel Merrifield SoC GPIO driver");
// MODULE_LICENSE("GPL v2");
// MODULE_IMPORT_NS("GPIO_TANGIER");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
