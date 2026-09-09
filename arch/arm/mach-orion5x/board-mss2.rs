// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Maxtor Shared Storage II Board Setup
 *
 * Maintainer: Sylver Bruneau <sylver.bruneau@googlemail.com>
 */

// Kernel and architecture dependencies supplied by other translation units.

/*****************************************************************************
 * Maxtor Shared Storage II Info
 *****************************************************************************/

/****************************************************************************
 * PCI setup
 ****************************************************************************/
unsafe fn mss2_pci_map_irq(
    dev: *const pci_dev,
    slot: u8,
    pin: u8,
) -> i32 {
    let irq: i32;

    /*
     * Check for devices with hard-wired IRQs.
     */
    irq = orion5x_pci_map_irq(dev, slot, pin);
    if irq != -1 {
        return irq;
    }

    -1
}

static mut mss2_pci: hw_pci = hw_pci {
    nr_controllers: 2,
    setup: Some(orion5x_pci_sys_setup),
    scan: Some(orion5x_pci_sys_scan_bus),
    map_irq: Some(mss2_pci_map_irq),
};

unsafe fn mss2_pci_init() -> i32 {
    if of_machine_is_compatible("maxtor,shared-storage-2\0".as_ptr() as *const i8) {
        pci_common_init(&raw mut mss2_pci);
    }

    0
}

// Equivalent to: subsys_initcall(mss2_pci_init);

/*****************************************************************************
 * MSS2 power off method
 ****************************************************************************/
/*
 * On the Maxtor Shared Storage II, the shutdown process is the following :
 * - Userland modifies U-boot env to tell U-boot to go idle at next boot
 * - The board reboots
 * - U-boot starts and go into an idle mode until the user press "power"
 */
unsafe fn mss2_power_off() {
    let mut reg: u32;

    /*
     * Enable and issue soft reset
     */
    reg = readl(RSTOUTn_MASK);
    reg |= 1 << 2;
    writel(reg, RSTOUTn_MASK);

    reg = readl(CPU_SOFT_RESET);
    reg |= 1;
    writel(reg, CPU_SOFT_RESET);
}

unsafe fn mss2_init() {
    /* register mss2 specific power-off method */
    register_platform_power_off(mss2_power_off);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
