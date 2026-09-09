/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the Linux PCI interface.
pub struct pci_dev;

/* Offsets to be accessed in the southbridge PCI
 * device configuration register */
pub const RDC321X_WDT_CTRL: u32 = 0x44;
pub const RDC321X_GPIO_CTRL_REG1: u32 = 0x48;
pub const RDC321X_GPIO_DATA_REG1: u32 = 0x4c;
pub const RDC321X_GPIO_CTRL_REG2: u32 = 0x84;
pub const RDC321X_GPIO_DATA_REG2: u32 = 0x88;

pub const RDC321X_NUM_GPIO: u32 = 59;

#[repr(C)]
pub struct rdc321x_gpio_pdata {
    pub sb_pdev: *mut pci_dev,
    pub max_gpios: u32,
}

#[repr(C)]
pub struct rdc321x_wdt_pdata {
    pub sb_pdev: *mut pci_dev,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
