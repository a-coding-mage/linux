// SPDX-License-Identifier: GPL-2.0
/*
 * GPIO driver for AMD 8111 south bridges
 *
 * Copyright (c) 2012 Dmitry Eremin-Solenikov
 *
 * Based on the AMD RNG driver:
 * Copyright 2005 (c) MontaVista Software, Inc.
 * with the majority of the code coming from:
 *
 * Hardware driver for the Intel/AMD/VIA Random Number Generators (RNG)
 * (c) Copyright 2003 Red Hat Inc <jgarzik@redhat.com>
 *
 * derived from
 *
 * Hardware driver for the AMD 768 Random Number Generator (RNG)
 * (c) Copyright 2001 Red Hat Inc
 *
 * derived from
 *
 * Hardware driver for Intel i810 Random Number Generator (RNG)
 * Copyright 2000,2001 Jeff Garzik <jgarzik@pobox.com>
 * Copyright 2000,2001 Philipp Rumpf <prumpf@mandrakesoft.com>
 */

// Dependencies supplied by the Linux kernel bindings are intentionally external.

const PMBASE_OFFSET: u32 = 0xb0;
const PMBASE_SIZE: u32 = 0x30;

#[inline]
const fn amd_reg_gpio(i: u32) -> u32 { 0x10 + i }

const AMD_GPIO_LTCH_STS: u8 = 0x40; // Latch status, w1
const AMD_GPIO_RTIN: u8 = 0x20; // Real Time in, ro
const AMD_GPIO_DEBOUNCE: u8 = 0x10; // Debounce, rw
const AMD_GPIO_MODE_MASK: u8 = 0x0c; // Pin Mode Select, rw
const AMD_GPIO_MODE_IN: u8 = 0x00;
const AMD_GPIO_MODE_OUT: u8 = 0x04;
// Enable alternative (e.g. clkout, IRQ, etc) function of the pin
const AMD_GPIO_MODE_ALTFN: u8 = 0x08; // Or 0x09
const AMD_GPIO_X_MASK: u8 = 0x03; // In/Out specific, rw
const AMD_GPIO_X_IN_ACTIVEHI: u8 = 0x01; // Active High
const AMD_GPIO_X_IN_LATCH: u8 = 0x02; // Latched version is selected
const AMD_GPIO_X_OUT_LOW: u8 = 0x00;
const AMD_GPIO_X_OUT_HI: u8 = 0x01;
const AMD_GPIO_X_OUT_CLK0: u8 = 0x02;
const AMD_GPIO_X_OUT_CLK1: u8 = 0x03;

/*
 * Data for PCI driver interface
 *
 * This data only exists for exporting the supported
 * PCI ids via MODULE_DEVICE_TABLE.  We do not actually
 * register a pci_driver, because someone else might one day
 * want to register another driver on the same PCI id.
 */
static mut pci_tbl: [pci_device_id; 2] = [
    pci_device_id { vendor: PCI_VENDOR_ID_AMD, device: PCI_DEVICE_ID_AMD_8111_SMBUS },
    pci_device_id { vendor: 0, device: 0 }, // terminate list
];

#[repr(C)]
struct amd_gpio {
    chip: gpio_chip,
    pmbase: u32,
    pm: *mut core::ffi::c_void,
    pdev: *mut pci_dev,
    lock: spinlock_t, // guards hw registers and orig table
    orig: [u8; 32],
}

unsafe fn amd_gpio_request(chip: *mut gpio_chip, offset: u32) -> i32 {
    let agp = gpiochip_get_data(chip) as *mut amd_gpio;
    (*agp).orig[offset as usize] = ioread8((*agp).pm.add(amd_reg_gpio(offset) as usize))
        & (AMD_GPIO_DEBOUNCE | AMD_GPIO_MODE_MASK | AMD_GPIO_X_MASK);
    dev_dbg(&(*(*agp).pdev).dev, "Requested gpio %d, data %x\n", offset, (*agp).orig[offset as usize]);
    0
}

unsafe fn amd_gpio_free(chip: *mut gpio_chip, offset: u32) {
    let agp = gpiochip_get_data(chip) as *mut amd_gpio;
    dev_dbg(&(*(*agp).pdev).dev, "Freed gpio %d, data %x\n", offset, (*agp).orig[offset as usize]);
    iowrite8((*agp).orig[offset as usize], (*agp).pm.add(amd_reg_gpio(offset) as usize));
}

unsafe fn amd_gpio_set(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let agp = gpiochip_get_data(chip) as *mut amd_gpio;
    let mut flags: unsigned_long = 0;
    spin_lock_irqsave(&mut (*agp).lock, &mut flags);
    let mut temp = ioread8((*agp).pm.add(amd_reg_gpio(offset) as usize));
    temp = (temp & AMD_GPIO_DEBOUNCE) | AMD_GPIO_MODE_OUT |
        if value != 0 { AMD_GPIO_X_OUT_HI } else { AMD_GPIO_X_OUT_LOW };
    iowrite8(temp, (*agp).pm.add(amd_reg_gpio(offset) as usize));
    spin_unlock_irqrestore(&mut (*agp).lock, flags);
    dev_dbg(&(*(*agp).pdev).dev, "Setting gpio %d, value %d, reg=%02x\n", offset, (value != 0) as i32, temp);
    0
}

unsafe fn amd_gpio_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let agp = gpiochip_get_data(chip) as *mut amd_gpio;
    let temp = ioread8((*agp).pm.add(amd_reg_gpio(offset) as usize));
    dev_dbg(&(*(*agp).pdev).dev, "Getting gpio %d, reg=%02x\n", offset, temp);
    if temp & AMD_GPIO_RTIN != 0 { 1 } else { 0 }
}

unsafe fn amd_gpio_dirout(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let agp = gpiochip_get_data(chip) as *mut amd_gpio;
    let mut flags: unsigned_long = 0;
    spin_lock_irqsave(&mut (*agp).lock, &mut flags);
    let mut temp = ioread8((*agp).pm.add(amd_reg_gpio(offset) as usize));
    temp = (temp & AMD_GPIO_DEBOUNCE) | AMD_GPIO_MODE_OUT |
        if value != 0 { AMD_GPIO_X_OUT_HI } else { AMD_GPIO_X_OUT_LOW };
    iowrite8(temp, (*agp).pm.add(amd_reg_gpio(offset) as usize));
    spin_unlock_irqrestore(&mut (*agp).lock, flags);
    dev_dbg(&(*(*agp).pdev).dev, "Dirout gpio %d, value %d, reg=%02x\n", offset, (value != 0) as i32, temp);
    0
}

unsafe fn amd_gpio_dirin(chip: *mut gpio_chip, offset: u32) -> i32 {
    let agp = gpiochip_get_data(chip) as *mut amd_gpio;
    let mut flags: unsigned_long = 0;
    spin_lock_irqsave(&mut (*agp).lock, &mut flags);
    let mut temp = ioread8((*agp).pm.add(amd_reg_gpio(offset) as usize));
    temp = (temp & AMD_GPIO_DEBOUNCE) | AMD_GPIO_MODE_IN;
    iowrite8(temp, (*agp).pm.add(amd_reg_gpio(offset) as usize));
    spin_unlock_irqrestore(&mut (*agp).lock, flags);
    dev_dbg(&(*(*agp).pdev).dev, "Dirin gpio %d, reg=%02x\n", offset, temp);
    0
}

static mut gp: amd_gpio = amd_gpio {
    chip: gpio_chip {
        label: "AMD GPIO" as *const str as *const i8,
        owner: THIS_MODULE,
        base: -1,
        ngpio: 32,
        request: Some(amd_gpio_request),
        free: Some(amd_gpio_free),
        set: Some(amd_gpio_set),
        get: Some(amd_gpio_get),
        direction_output: Some(amd_gpio_dirout),
        direction_input: Some(amd_gpio_dirin),
        ..core::mem::zeroed()
    },
    pmbase: 0,
    pm: core::ptr::null_mut(),
    pdev: core::ptr::null_mut(),
    lock: core::mem::zeroed(),
    orig: [0; 32],
};

unsafe fn amd_gpio_init() -> i32 {
    let mut err = -ENODEV;
    let mut pdev: *mut pci_dev = core::ptr::null_mut();
    let mut ent: *const pci_device_id;
    /* We look for our device - AMD South Bridge
     * I don't know about a system with two such bridges,
     * so we can assume that there is max. one device.
     *
     * We can't use plain pci_driver mechanism,
     * as the device is really a multiple function device,
     * main driver that binds to the pci_device is an smbus
     * driver and have to find & bind to the device this way.
     */
    for_each_pci_dev!(pdev) {
        ent = pci_match_id(pci_tbl.as_ptr(), pdev);
        if !ent.is_null() { break; }
    }
    if ent.is_null() { pci_dev_put(pdev); return err; }

    'found: {
        err = pci_read_config_dword(pdev, 0x58, &mut gp.pmbase);
        if err != 0 { err = pcibios_err_to_errno(err); pci_dev_put(pdev); return err; }
        err = -EIO;
        gp.pmbase &= 0x0000FF00;
        if gp.pmbase == 0 { pci_dev_put(pdev); return err; }
        if devm_request_region(&(*pdev).dev, gp.pmbase + PMBASE_OFFSET, PMBASE_SIZE, "AMD GPIO").is_null() {
            dev_err(&(*pdev).dev, "AMD GPIO region 0x%x already in use!\n", gp.pmbase + PMBASE_OFFSET);
            err = -EBUSY; pci_dev_put(pdev); return err;
        }
        gp.pm = ioport_map(gp.pmbase + PMBASE_OFFSET, PMBASE_SIZE);
        if gp.pm.is_null() { dev_err(&(*pdev).dev, "Couldn't map io port into io memory\n"); err = -ENOMEM; pci_dev_put(pdev); return err; }
        gp.pdev = pdev;
        gp.chip.parent = &mut (*pdev).dev;
        spin_lock_init(&mut gp.lock);
        dev_info(&(*pdev).dev, "AMD-8111 GPIO detected\n");
        err = gpiochip_add_data(&mut gp.chip, &mut gp as *mut amd_gpio as *mut core::ffi::c_void);
        if err != 0 { dev_err(&(*pdev).dev, "GPIO registering failed (%d)\n", err); ioport_unmap(gp.pm); pci_dev_put(pdev); return err; }
        0
    }
}

unsafe fn amd_gpio_exit() {
    gpiochip_remove(&mut gp.chip);
    ioport_unmap(gp.pm);
    pci_dev_put(gp.pdev);
}

// module_init(amd_gpio_init);
// module_exit(amd_gpio_exit);
// MODULE_AUTHOR("The Linux Kernel team");
// MODULE_DESCRIPTION("GPIO driver for AMD chipsets");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
