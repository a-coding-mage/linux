// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/arm/mach-footbridge/ebsa285-pci.c
 *
 * PCI bios-type initialisation for PCI machines
 *
 * Bits taken from various places.
 */

// The declarations below are supplied by the surrounding kernel translation.

#[repr(C)]
pub struct pci_dev {
    pub vendor: u16,
    pub device: u16,
    pub devfn: u8,
}

#[repr(C)]
pub struct hw_pci {
    pub map_irq: Option<unsafe extern "C" fn(*const pci_dev, u8, u8) -> i32>,
    pub nr_controllers: i32,
    pub ops: *const core::ffi::c_void,
    pub setup: Option<unsafe extern "C" fn()>,
    pub preinit: Option<unsafe extern "C" fn()>,
    pub postinit: Option<unsafe extern "C" fn()>,
}

extern "C" {
    static dc21285_ops: core::ffi::c_void;
    fn dc21285_setup();
    fn dc21285_preinit();
    fn dc21285_postinit();
    fn machine_is_ebsa285() -> i32;
    fn pci_common_init(pci: *mut hw_pci);
}

static mut irqmap_ebsa285: [i32; 4] = [IRQ_IN3, IRQ_IN1, IRQ_IN0, IRQ_PCI];

unsafe extern "C" fn ebsa285_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> i32 {
    if (*dev).vendor == PCI_VENDOR_ID_CONTAQ
        && (*dev).device == PCI_DEVICE_ID_CONTAQ_82C693
    {
        match (*dev).devfn & 7 {
            1 => return 14,
            2 => return 15,
            3 => return 12,
            _ => {}
        }
    }

    irqmap_ebsa285[((slot.wrapping_add(pin)) & 3) as usize]
}

static mut ebsa285_pci: hw_pci = hw_pci {
    map_irq: Some(ebsa285_map_irq),
    nr_controllers: 1,
    ops: unsafe { &dc21285_ops as *const core::ffi::c_void },
    setup: Some(dc21285_setup),
    preinit: Some(dc21285_preinit),
    postinit: Some(dc21285_postinit),
};

unsafe extern "C" fn ebsa285_init_pci() -> i32 {
    if machine_is_ebsa285() != 0 {
        pci_common_init(&raw mut ebsa285_pci);
    }
    0
}

// Equivalent to: subsys_initcall(ebsa285_init_pci);
#[used]
#[unsafe(link_section = ".initcall6.init")]
static EBSA285_INIT_PCI_INITCALL: unsafe extern "C" fn() -> i32 = ebsa285_init_pci;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
