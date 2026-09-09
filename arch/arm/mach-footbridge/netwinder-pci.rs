// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/arm/mach-footbridge/netwinder-pci.c
 *
 * PCI bios-type initialisation for PCI machines
 *
 * Bits taken from various places.
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hw_pci {
    pub map_irq: Option<unsafe extern "C" fn(*const pci_dev, u8, u8) -> c_int>,
    pub nr_controllers: c_int,
    pub ops: *const pci_ops,
    pub setup: Option<unsafe extern "C" fn()>,
    pub preinit: Option<unsafe extern "C" fn()>,
    pub postinit: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" {
    static dc21285_ops: pci_ops;
    unsafe fn dc21285_setup();
    unsafe fn dc21285_preinit();
    unsafe fn dc21285_postinit();
    unsafe fn machine_is_netwinder() -> c_int;
    unsafe fn pci_common_init(pci: *mut hw_pci);
    unsafe fn pci_name(dev: *const pci_dev) -> *const c_char;
    unsafe fn printk(fmt: *const c_char, ...) -> c_int;
}

// Values supplied by <asm/irq.h>.
unsafe extern "C" {
    static IRQ_NETWINDER_VGA: c_int;
    static IRQ_NETWINDER_ETHER100: c_int;
    static IRQ_ISA_HARDDISK1: c_int;
    static IRQ_NETWINDER_ETHER10: c_int;
}

/*
 * We now use the slot ID instead of the device identifiers to select
 * which interrupt is routed where.
 */
unsafe extern "C" fn netwinder_map_irq(dev: *const pci_dev, slot: u8, _pin: u8) -> c_int {
    match slot {
        0 => 0, /* host bridge */

        9 => IRQ_NETWINDER_VGA, /* CyberPro */

        10 => IRQ_NETWINDER_ETHER100, /* DC21143 */

        12 => IRQ_ISA_HARDDISK1, /* Winbond 553 */

        13 => IRQ_NETWINDER_ETHER10, /* Winbond 89C940F */

        _ => {
            static FORMAT: &[u8] = b"PCI: unknown device in slot %s\n\0";
            printk(FORMAT.as_ptr() as *const c_char, pci_name(dev));
            0
        }
    }
}

static mut netwinder_pci: hw_pci = hw_pci {
    map_irq: Some(netwinder_map_irq),
    nr_controllers: 1,
    ops: unsafe { &dc21285_ops as *const pci_ops },
    setup: Some(dc21285_setup),
    preinit: Some(dc21285_preinit),
    postinit: Some(dc21285_postinit),
};

unsafe extern "C" fn netwinder_pci_init() -> c_int {
    if machine_is_netwinder() != 0 {
        pci_common_init(&raw mut netwinder_pci);
    }
    0
}

// Equivalent of subsys_initcall(netwinder_pci_init).
#[used]
#[cfg_attr(target_os = "linux", link_section = ".initcall4.init")]
static NETWINDER_PCI_INITCALL: unsafe extern "C" fn() -> c_int = netwinder_pci_init;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
