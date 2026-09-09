/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Header for ines GPIB boards
 *   copyright            : (C) 2002 by Frank Mori Hess
 *
 * C dependencies are supplied by the surrounding translation unit.
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ines_pci_chip {
    PCI_CHIP_NONE,
    PCI_CHIP_PLX9050,
    PCI_CHIP_AMCC5920,
    PCI_CHIP_QUANCOM,
    PCI_CHIP_QUICKLOGIC5030,
    PCI_CHIP_INES_72130,
}

#[repr(C)]
pub struct ines_priv {
    pub nec7210_priv: nec7210_priv,
    pub pci_device: *mut pci_dev,
    // base address for plx9052 pci chip
    pub plx_iobase: ::std::os::raw::c_ulong,
    // base address for amcc5920 pci chip
    pub amcc_iobase: ::std::os::raw::c_ulong,
    pub irq: ::std::os::raw::c_uint,
    pub pci_chip_type: ines_pci_chip,
    pub extend_mode_bits: u8,
}

extern "C" {
    pub fn inb(port: ::std::os::raw::c_ulong) -> ::std::os::raw::c_uint;
    pub fn outb(value: ::std::os::raw::c_uint, port: ::std::os::raw::c_ulong);
}

/* inb/outb wrappers */
#[inline]
pub unsafe fn ines_inb(priv_: *mut ines_priv, register_number: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint {
    inb((*priv_).nec7210_priv.iobase
        + register_number as ::std::os::raw::c_ulong * (*priv_).nec7210_priv.offset)
}

#[inline]
pub unsafe fn ines_outb(
    priv_: *mut ines_priv,
    value: ::std::os::raw::c_uint,
    register_number: ::std::os::raw::c_uint,
) {
    outb(
        value,
        (*priv_).nec7210_priv.iobase
            + register_number as ::std::os::raw::c_ulong * (*priv_).nec7210_priv.offset,
    );
}

#[repr(C)]
pub enum ines_regs {
    // read
    FIFO_STATUS = 0x8,
    ISR3 = 0x9,
    ISR4 = 0xa,
    IN_FIFO_COUNT = 0x10,
    OUT_FIFO_COUNT = 0x11,
    EXTEND_STATUS = 0xf,
    // write
    XDMA_CONTROL = 0x8,
    IMR3 = 0x9,
    IMR4 = 0xa,
    IN_FIFO_WATERMARK = 0x10,
    OUT_FIFO_WATERMARK = 0x11,
    EXTEND_MODE = 0xf,
    // read-write
    XFER_COUNT_LOWER = 0xb,
    XFER_COUNT_UPPER = 0xc,
    BUS_CONTROL_MONITOR = 0x13,
}

#[repr(C)]
pub enum isr3_imr3_bits {
    HW_TIMEOUT_BIT = 0x1, XFER_COUNT_BIT = 0x2, CMD_RECEIVED_BIT = 0x4,
    TCT_RECEIVED_BIT = 0x8, IFC_ACTIVE_BIT = 0x10, ATN_ACTIVE_BIT = 0x20,
    FIFO_ERROR_BIT = 0x40,
}

#[repr(C)]
pub enum isr4_imr4_bits {
    IN_FIFO_WATERMARK_BIT = 0x1, OUT_FIFO_WATERMARK_BIT = 0x2,
    IN_FIFO_FULL_BIT = 0x4, OUT_FIFO_EMPTY_BIT = 0x8,
    IN_FIFO_READY_BIT = 0x10, OUT_FIFO_READY_BIT = 0x20,
    IN_FIFO_EXIT_WATERMARK_BIT = 0x40, OUT_FIFO_EXIT_WATERMARK_BIT = 0x80,
}

#[repr(C)]
pub enum extend_mode_bits {
    TR3_TRIG_ENABLE_BIT = 0x1, // enable generation of trigger pulse T/R3 pin
    MAV_ENABLE_BIT = 0x2, // clear message available status bit when chip writes byte with EOI true
    EOS1_ENABLE_BIT = 0x4, // enable eos register 1
    EOS2_ENABLE_BIT = 0x8, // enable eos register 2
    EOIDIS_BIT = 0x10, // disable EOI interrupt when doing rfd holdoff on end?
    XFER_COUNTER_ENABLE_BIT = 0x20,
    XFER_COUNTER_OUTPUT_BIT = 0x40, // use counter for output, clear for input
    LAST_BYTE_HANDLING_BIT = 0x80, // when xfer counter hits 0, assert EOI on write or RFD holdoff on read
}

#[repr(C)]
pub enum extend_status_bits { OUTPUT_MESSAGE_IN_PROGRESS_BIT = 0x1, SCSEL_BIT = 0x2, LISTEN_DISABLED = 0x4, IN_FIFO_EMPTY_BIT = 0x8, OUT_FIFO_FULL_BIT = 0x10 }
#[repr(C)]
pub enum ines_admr_bits { IN_FIFO_ENABLE_BIT = 0x8, OUT_FIFO_ENABLE_BIT = 0x4 }
#[repr(C)]
pub enum xdma_control_bits { DMA_OUTPUT_BIT = 0x1, ENABLE_SYNC_DMA_BIT = 0x2, DMA_ACCESS_EVERY_CYCLE = 0x4, DMA_16BIT = 0x8 }
#[repr(C)]
pub enum bus_control_monitor_bits { BCM_DAV_BIT = 0x1, BCM_NRFD_BIT = 0x2, BCM_NDAC_BIT = 0x4, BCM_IFC_BIT = 0x8, BCM_ATN_BIT = 0x10, BCM_SRQ_BIT = 0x20, BCM_REN_BIT = 0x40, BCM_EOI_BIT = 0x80 }
#[repr(C)]
pub enum ines_aux_reg_bits { INES_AUXD = 0x40 }
#[repr(C)]
pub enum ines_aux_cmds { INES_RFD_HLD_IMMEDIATE = 0x4, INES_AUX_CLR_OUT_FIFO = 0x5, INES_AUX_CLR_IN_FIFO = 0x6, INES_AUX_XMODE = 0xa }
#[repr(C)]
pub enum ines_auxd_bits { INES_FOLLOWING_T1_MASK = 0x3, INES_FOLLOWING_T1_500ns = 0x0, INES_FOLLOWING_T1_350ns = 0x1, INES_FOLLOWING_T1_250ns = 0x2, INES_INITIAL_TI_MASK = 0xc, INES_INITIAL_T1_2000ns = 0x0, INES_INITIAL_T1_1100ns = 0x4, INES_INITIAL_T1_700ns = 0x8, INES_T6_2us = 0x0, INES_T6_50us = 0x10 }
#[repr(C)]
pub enum ines72130_regs { BUS_STATUS_REG = 0xc }
#[repr(C)]
pub enum ines_72130_bus_status_bits { BSR_NRFD_BIT = 0x1, BSR_NDAC_BIT = 0x2, BSR_DAV_BIT = 0x4, BSR_EOI_BIT = 0x8, BSR_SRQ_BIT = 0x10, BSR_ATN_BIT = 0x20, BSR_REN_BIT = 0x40, BSR_IFC_BIT = 0x80 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
