// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (c) 2001,2002 Christer Weinigel <wingel@nano-system.com>
 *
 *  National Semiconductor SCx200 support.
 */

// Linux kernel headers and build-time configuration are supplied by the
// surrounding translation unit.

#[repr(C)]
pub struct pci_device_id {
    pub data: [u64; 8],
}

#[repr(C)]
pub struct pci_dev {
    pub device: u16,
}

#[repr(C)]
pub struct pci_driver {
    pub name: *const core::ffi::c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> i32>,
}

extern "C" {
    fn inw(port: u32) -> u16;
    fn inl(port: u32) -> u32;
    fn outl(value: u32, port: u32);
    fn request_region(start: u32, length: u32, name: *const core::ffi::c_char) -> *mut core::ffi::c_void;
    fn release_region(start: u32, length: u32);
    fn pci_resource_start(dev: *mut pci_dev, bar: u32) -> u32;
    fn pci_read_config_dword(dev: *mut pci_dev, where_: u32, value: *mut u32) -> i32;
    fn pci_register_driver(driver: *mut pci_driver) -> i32;
    fn pci_unregister_driver(driver: *mut pci_driver);
    fn mutex_lock(lock: *mut core::ffi::c_void);
    fn mutex_unlock(lock: *mut core::ffi::c_void);
}

// External constants supplied by the SCx200 and PCI headers.
extern "C" {
    static PCI_DEVICE_ID_NS_SCx200_BRIDGE: u16;
    static PCI_DEVICE_ID_NS_SC1100_BRIDGE: u16;
    static SCx200_CB_BASE_FIXED: u32;
    static SCx200_CBA: u32;
    static SCx200_CBA_SCRATCH: u32;
    static SCx200_GPIO_SIZE: u32;
}

// MODULE_AUTHOR("Christer Weinigel <wingel@nano-system.com>");
// MODULE_DESCRIPTION("NatSemi SCx200 Driver");
// MODULE_LICENSE("GPL");

macro_rules! scx200_cb_probe {
    ($base:expr) => {
        unsafe { inw(($base) + SCx200_CBA) == ($base) as u16 }
    };
}

pub static mut scx200_gpio_base: u32 = 0;
pub static mut scx200_gpio_shadow: [u64; 2] = [0; 2];
pub static mut scx200_cb_base: u32 = 0;
static mut scx200_gpio_config_lock: core::ffi::c_void = core::ffi::c_void {};

static mut scx200_tbl: [pci_device_id; 5] = [
    pci_device_id { data: [0; 8] },
    pci_device_id { data: [0; 8] },
    pci_device_id { data: [0; 8] },
    pci_device_id { data: [0; 8] },
    pci_device_id { data: [0; 8] },
];

unsafe extern "C" fn scx200_probe(pdev: *mut pci_dev, _ent: *const pci_device_id) -> i32 {
    let mut base: u32;

    if (*pdev).device == PCI_DEVICE_ID_NS_SCx200_BRIDGE ||
       (*pdev).device == PCI_DEVICE_ID_NS_SC1100_BRIDGE {
        base = pci_resource_start(pdev, 0);

        if request_region(base, SCx200_GPIO_SIZE, b"NatSemi SCx200 GPIO\0".as_ptr() as *const core::ffi::c_char).is_null() {
            return -16; // -EBUSY
        }

        scx200_gpio_base = base;
        scx200_init_shadow();
    } else {
        // find the base of the Configuration Block
        if scx200_cb_probe!(SCx200_CB_BASE_FIXED) {
            scx200_cb_base = SCx200_CB_BASE_FIXED;
        } else {
            pci_read_config_dword(pdev, SCx200_CBA_SCRATCH, &mut base);
            if scx200_cb_probe!(base) {
                scx200_cb_base = base;
            } else {
                return -19; // -ENODEV
            }
        }
    }

    0
}

unsafe fn scx200_init_shadow() {
    let mut bank: i32 = 0;
    // read the current values driven on the GPIO signals
    while bank < 2 {
        scx200_gpio_shadow[bank as usize] = inl(scx200_gpio_base + 0x10 * bank as u32) as u64;
        bank += 1;
    }
}

pub unsafe fn scx200_gpio_configure(index: u32, mask: u32, bits: u32) -> u32 {
    let config: u32;
    let new_config: u32;

    mutex_lock(&mut scx200_gpio_config_lock);
    outl(index, scx200_gpio_base + 0x20);
    config = inl(scx200_gpio_base + 0x24);
    new_config = (config & mask) | bits;
    outl(new_config, scx200_gpio_base + 0x24);
    mutex_unlock(&mut scx200_gpio_config_lock);

    config
}

static mut scx200_pci_driver: pci_driver = pci_driver {
    name: b"scx200\0".as_ptr() as *const core::ffi::c_char,
    id_table: unsafe { scx200_tbl.as_ptr() },
    probe: Some(scx200_probe),
};

pub unsafe extern "C" fn scx200_init() -> i32 {
    pci_register_driver(&mut scx200_pci_driver)
}

pub unsafe extern "C" fn scx200_cleanup() {
    pci_unregister_driver(&mut scx200_pci_driver);
    release_region(scx200_gpio_base, SCx200_GPIO_SIZE);
}

// module_init(scx200_init);
// module_exit(scx200_cleanup);
// EXPORT_SYMBOL(scx200_gpio_base);
// EXPORT_SYMBOL(scx200_gpio_shadow);
// EXPORT_SYMBOL(scx200_gpio_configure);
// EXPORT_SYMBOL(scx200_cb_base);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
