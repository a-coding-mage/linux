// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-orion5x/rd88f5182-setup.c
 *
 * Marvell Orion-NAS Reference Design Setup
 *
 * Maintainer: Ronen Shitrit <rshitrit@marvell.com>
 */

// Dependencies supplied by the surrounding kernel translation.

const RD88F5182_PCI_SLOT0_OFFS: u8 = 7;
const RD88F5182_PCI_SLOT0_IRQ_A_PIN: i32 = 7;
const RD88F5182_PCI_SLOT0_IRQ_B_PIN: i32 = 6;

extern "C" {
    fn gpio_request(pin: i32, label: *const u8) -> i32;
    fn gpio_direction_input(pin: i32) -> i32;
    fn gpio_to_irq(pin: i32) -> i32;
    fn gpio_free(pin: i32);
    fn irq_set_irq_type(irq: i32, irq_type: u32);
    fn printk(format: *const u8, ...);
    fn orion5x_pci_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> i32;
    fn orion5x_pci_sys_setup(controller: *mut core::ffi::c_void);
    fn orion5x_pci_sys_scan_bus(controller: *mut core::ffi::c_void);
    fn of_machine_is_compatible(compat: *const u8) -> bool;
    fn pci_common_init(pci: *mut hw_pci);
}

#[repr(C)]
pub struct pci_dev {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct hw_pci {
    pub nr_controllers: i32,
    pub preinit: Option<unsafe extern "C" fn()>,
    pub setup: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub scan: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub map_irq: Option<unsafe extern "C" fn(*const pci_dev, u8, u8) -> i32>,
}

const IRQ_TYPE_LEVEL_LOW: u32 = 8;

unsafe extern "C" fn rd88f5182_pci_preinit() {
    let mut pin: i32;

    /*
     * Configure PCI GPIO IRQ pins
     */
    pin = RD88F5182_PCI_SLOT0_IRQ_A_PIN;
    if gpio_request(pin, b"PCI IntA\0".as_ptr()) == 0 {
        if gpio_direction_input(pin) == 0 {
            irq_set_irq_type(gpio_to_irq(pin), IRQ_TYPE_LEVEL_LOW);
        } else {
            printk(b"rd88f5182_pci_preinit failed to set_irq_type pin %d\n\0".as_ptr(), pin);
            gpio_free(pin);
        }
    } else {
        printk(b"rd88f5182_pci_preinit failed to request gpio %d\n\0".as_ptr(), pin);
    }

    pin = RD88F5182_PCI_SLOT0_IRQ_B_PIN;
    if gpio_request(pin, b"PCI IntB\0".as_ptr()) == 0 {
        if gpio_direction_input(pin) == 0 {
            irq_set_irq_type(gpio_to_irq(pin), IRQ_TYPE_LEVEL_LOW);
        } else {
            printk(b"rd88f5182_pci_preinit failed to set_irq_type pin %d\n\0".as_ptr(), pin);
            gpio_free(pin);
        }
    } else {
        printk(b"rd88f5182_pci_preinit failed to gpio_request %d\n\0".as_ptr(), pin);
    }
}

unsafe extern "C" fn rd88f5182_pci_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> i32 {
    let irq: i32;

    /*
     * Check for devices with hard-wired IRQs.
     */
    irq = orion5x_pci_map_irq(dev, slot, pin);
    if irq != -1 {
        return irq;
    }

    /*
     * PCI IRQs are connected via GPIOs
     */
    match slot.wrapping_sub(RD88F5182_PCI_SLOT0_OFFS) {
        0 => {
            if pin == 1 {
                gpio_to_irq(RD88F5182_PCI_SLOT0_IRQ_A_PIN)
            } else {
                gpio_to_irq(RD88F5182_PCI_SLOT0_IRQ_B_PIN)
            }
        }
        _ => -1,
    }
}

static mut rd88f5182_pci: hw_pci = hw_pci {
    nr_controllers: 2,
    preinit: Some(rd88f5182_pci_preinit),
    setup: Some(orion5x_pci_sys_setup),
    scan: Some(orion5x_pci_sys_scan_bus),
    map_irq: Some(rd88f5182_pci_map_irq),
};

unsafe extern "C" fn rd88f5182_pci_init() -> i32 {
    if of_machine_is_compatible(b"marvell,rd-88f5182-nas\0".as_ptr()) {
        pci_common_init(&mut rd88f5182_pci);
    }

    0
}

// Equivalent registration for the kernel's subsys_initcall(rd88f5182_pci_init).
#[used]
static RD88F5182_PCI_INITCALL: unsafe extern "C" fn() -> i32 = rd88f5182_pci_init;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
