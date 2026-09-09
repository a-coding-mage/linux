// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the kernel headers:
// linux/init.h, linux/kernel.h, linux/pci.h, asm/ip32/ip32_ints.h

use core::ffi::c_int;

// O2 has up to 5 PCI devices connected into the MACE bridge.  The device
// map looks like this:
//
// 0  aic7xxx 0
// 1  aic7xxx 1
// 2  expansion slot
// 3  N/C
// 4  N/C

const SCSI0: i8 = MACEPCI_SCSI0_IRQ as i8;
const SCSI1: i8 = MACEPCI_SCSI1_IRQ as i8;
const INTA0: i8 = MACEPCI_SLOT0_IRQ as i8;
const INTA1: i8 = MACEPCI_SLOT1_IRQ as i8;
const INTA2: i8 = MACEPCI_SLOT2_IRQ as i8;
const INTB: i8 = MACEPCI_SHARED0_IRQ as i8;
const INTC: i8 = MACEPCI_SHARED1_IRQ as i8;
const INTD: i8 = MACEPCI_SHARED2_IRQ as i8;

static mut irq_tab_mace: [[i8; 5]; 6] = [
    // Dummy  INT#A  INT#B  INT#C  INT#D
    [0, 0, 0, 0, 0], // This is placeholder row - never used
    [0, SCSI0, SCSI0, SCSI0, SCSI0],
    [0, SCSI1, SCSI1, SCSI1, SCSI1],
    [0, INTA0, INTB, INTC, INTD],
    [0, INTA1, INTC, INTD, INTB],
    [0, INTA2, INTD, INTB, INTC],
];

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

// Given a PCI slot number (a la PCI_SLOT(...)) and the interrupt pin of
// the device (1-4 => A-D), tell what irq to use.  Note that we don't
// in theory have slots 4 and 5, and we never normally use the shared
// irqs.  I suppose a device without a pin A will thank us for doing it
// right if there exists such a broken piece of crap.
#[no_mangle]
pub unsafe extern "C" fn pcibios_map_irq(_dev: *const pci_dev, slot: u8, pin: u8) -> c_int {
    irq_tab_mace[slot as usize][pin as usize] as c_int
}

// Do platform specific device initialization at pci_enable_device() time
#[no_mangle]
pub unsafe extern "C" fn pcibios_plat_dev_init(_dev: *mut pci_dev) -> c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
