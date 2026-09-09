/* SPDX-License-Identifier: GPL-2.0 */

pub const IC_PIC1: u32 = 0x20; // PCI I/O address of master 8259
pub const IC_PIC2: u32 = 0xA0; // PCI I/O address of slave

// Config Space Offsets to configuration and base address registers
pub const SIO_CR: u32 = 0x5A; // Configuration Register
pub const SIO_ACPIBAR: u32 = 0x88; // ACPI BAR
pub const SIO_FDCBAR: u32 = 0x90; // Floppy Disk Controller BAR
pub const SIO_SP1BAR: u32 = 0x94; // Serial 1 BAR
pub const SIO_SP2BAR: u32 = 0x98; // Serial 2 BAR
pub const SIO_PPBAR: u32 = 0x9C; // Parallel BAR

pub const TRIGGER_1: u32 = 0x67; // Edge/level trigger register 1
pub const TRIGGER_2: u32 = 0x68; // Edge/level trigger register 2

// Interrupt Routing Control registers
pub const CFG_IR_SER: u32 = 0x69; // Serial 1 [0:3] and Serial 2 [4:7]
pub const CFG_IR_PFD: u32 = 0x6a; // Parallel [0:3] and Floppy [4:7]
pub const CFG_IR_IDE: u32 = 0x6b; // IDE1 [0:3] and IDE2 [4:7]
pub const CFG_IR_INTAB: u32 = 0x6c; // PCI INTA [0:3] and INT B [4:7]
pub const CFG_IR_INTCD: u32 = 0x6d; // PCI INTC [0:3] and INT D [4:7]
pub const CFG_IR_PS2: u32 = 0x6e; // PS/2 KBINT [0:3] and Mouse [4:7]
pub const CFG_IR_FXBUS: u32 = 0x6f; // FXIRQ[0] [0:3] and FXIRQ[1] [4:7]
pub const CFG_IR_USB: u32 = 0x70; // FXIRQ[2] [0:3] and USB [4:7]
pub const CFG_IR_ACPI: u32 = 0x71; // ACPI SCI [0:3] and reserved [4:7]

pub const CFG_IR_LOW: u32 = CFG_IR_SER; // Lowest interrupt routing reg
pub const CFG_IR_HIGH: u32 = CFG_IR_ACPI; // Highest interrupt routing reg

// 8259 operational control words
pub const OCW2_EOI: u32 = 0x20; // Non-specific EOI
pub const OCW2_SEOI: u32 = 0x60; // Specific EOI
pub const OCW3_IIR: u32 = 0x0A; // Read request register
pub const OCW3_ISR: u32 = 0x0B; // Read service register
pub const OCW3_POLL: u32 = 0x0C; // Poll the PIC for an interrupt vector

// Interrupt lines. Only PIC1 is used
pub const USB_IRQ: i32 = 1; // USB
pub const SP1_IRQ: i32 = 3; // Serial port 1
pub const SP2_IRQ: i32 = 4; // Serial port 2
pub const PAR_IRQ: i32 = 5; // Parallel port
pub const FDC_IRQ: i32 = 6; // Floppy controller
pub const IDE_IRQ: i32 = 7; // IDE (pri+sec)

// ACPI registers
pub const USB_REG_CR: u32 = 0x1f; // USB Regulator Control Register

pub const SUPERIO_NIRQS: usize = 8;

#[repr(C)]
pub struct superio_device {
    pub fdc_base: u32,
    pub sp1_base: u32,
    pub sp2_base: u32,
    pub pp_base: u32,
    pub acpi_base: u32,
    pub suckyio_irq_enabled: i32,
    pub lio_pdev: *mut pci_dev, // pci device for legacy IO (fn 1)
    pub usb_pdev: *mut pci_dev, // pci device for USB (fn 2)
}

// Does NS make a 87415 based plug in PCI card? If so, because of this
// macro we currently don't support it being plugged into a machine
// that contains a SuperIO chip AND has CONFIG_SUPERIO enabled.
//
// This could be fixed by checking to see if function 1 exists, and
// if it is SuperIO Legacy IO; but really now, is this combination
// going to EVER happen?

pub const SUPERIO_IDE_FN: i32 = 0; // Function number of IDE controller
pub const SUPERIO_LIO_FN: i32 = 1; // Function number of Legacy IO controller
pub const SUPERIO_USB_FN: i32 = 2; // Function number of USB controller

#[macro_export]
macro_rules! is_superio_device {
    ($x:expr) => {
        (($x).vendor == PCI_VENDOR_ID_NS)
            && ((($x).device == PCI_DEVICE_ID_NS_87415)
                || (($x).device == PCI_DEVICE_ID_NS_87560_LIO)
                || (($x).device == PCI_DEVICE_ID_NS_87560_USB))
    };
}

unsafe extern "C" {
    pub fn superio_fixup_irq(pcidev: *mut pci_dev); // called by iosapic
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
