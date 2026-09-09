// SPDX-License-Identifier: GPL-2.0
/*
 * This is a good place to put board specific reboot fixups.
 *
 * List of supported fixups:
 * geode-gx1/cs5530a - Jaya Kumar <jayalk@intworks.biz>
 * geode-gx/lx/cs5536 - Andres Salomon <dilinger@debian.org>
 *
 */

// C dependencies supplied by other files.
#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn pci_write_config_byte(dev: *mut pci_dev, where_: u32, val: u8);
    fn udelay(usecs: u32);
    fn wrmsrq(msr: u32, val: u64);
    fn outl(value: u32, port: u16);
    fn inl(port: u16) -> u32;
    fn outb(value: u8, port: u16);
    fn in_interrupt() -> bool;
    fn pci_get_device(vendor: u32, device: u32, from: *mut pci_dev) -> *mut pci_dev;
    fn pci_dev_put(dev: *mut pci_dev);
}

unsafe fn cs5530a_warm_reset(dev: *mut pci_dev) {
    /* writing 1 to the reset control register, 0x44 causes the
    cs5530a to perform a system warm reset */
    pci_write_config_byte(dev, 0x44, 0x1);
    udelay(50); /* shouldn't get here but be safe and spin-a-while */
    return;
}

unsafe fn cs5536_warm_reset(_dev: *mut pci_dev) {
    /* writing 1 to the LSB of this MSR causes a hard reset */
    wrmsrq(MSR_DIVIL_SOFT_RESET, 1u64);
    udelay(50); /* shouldn't get here but be safe and spin a while */
}

unsafe fn rdc321x_reset(_dev: *mut pci_dev) {
    let mut i: u32;
    /* Voluntary reset the watchdog timer */
    outl(0x80003840, 0xCF8);
    /* Generate a CPU reset on next tick */
    i = inl(0xCFC);
    /* Use the minimum timer resolution */
    i |= 0x1600;
    outl(i, 0xCFC);
    outb(1, 0x92);
}

unsafe fn ce4100_reset(_dev: *mut pci_dev) {
    let mut i: i32;

    for i in 0..10 {
        let _ = i;
        outb(0x2, 0xcf9);
        udelay(50);
    }
}

#[repr(C)]
struct device_fixup {
    vendor: u32,
    device: u32,
    reboot_fixup: unsafe fn(*mut pci_dev),
}

/*
 * PCI ids solely used for fixups_table go here
 */
const PCI_DEVICE_ID_INTEL_CE4100: u32 = 0x0708;

static fixups_table: &[device_fixup] = &[
    device_fixup { vendor: PCI_VENDOR_ID_CYRIX, device: PCI_DEVICE_ID_CYRIX_5530_LEGACY, reboot_fixup: cs5530a_warm_reset },
    device_fixup { vendor: PCI_VENDOR_ID_AMD, device: PCI_DEVICE_ID_AMD_CS5536_ISA, reboot_fixup: cs5536_warm_reset },
    device_fixup { vendor: PCI_VENDOR_ID_NS, device: PCI_DEVICE_ID_NS_SC1100_BRIDGE, reboot_fixup: cs5530a_warm_reset },
    device_fixup { vendor: PCI_VENDOR_ID_RDC, device: PCI_DEVICE_ID_RDC_R6030, reboot_fixup: rdc321x_reset },
    device_fixup { vendor: PCI_VENDOR_ID_INTEL, device: PCI_DEVICE_ID_INTEL_CE4100, reboot_fixup: ce4100_reset },
];

/*
 * we see if any fixup is available for our current hardware. if there
 * is a fixup, we call it and we expect to never return from it. if we
 * do return, we keep looking and then eventually fall back to the
 * standard mach_reboot on return.
 */
pub unsafe fn mach_reboot_fixups() {
    let cur: *const device_fixup;
    let dev: *mut pci_dev;

    /* we can be called from sysrq-B code. In such a case it is
     * prohibited to dig PCI */
    if in_interrupt() {
        return;
    }

    for i in 0..fixups_table.len() {
        cur = &fixups_table[i];
        dev = pci_get_device((*cur).vendor, (*cur).device, core::ptr::null_mut());
        if dev.is_null() {
            continue;
        }

        ((*cur).reboot_fixup)(dev);
        pci_dev_put(dev);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
