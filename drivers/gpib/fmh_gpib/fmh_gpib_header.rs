/* SPDX-License-Identifier: GPL-2.0 */

/***************************************************************************
 *    Author: Frank Mori Hess <fmh6jj@gmail.com>
 *   Copyright: (C) 2006, 2010, 2015 Fluke Corporation
 *	(C) 2017 Frank Mori Hess
 ***************************************************************************/

// External kernel and nec7210 declarations are supplied by other translation units.

static FIFO_REG_OFFSET: ::core::ffi::c_int = 2;

static GPIB_CONTROL_STATUS_PCI_RESOURCE_INDEX: ::core::ffi::c_int = 0;
static GPIB_FIFO_PCI_RESOURCE_INDEX: ::core::ffi::c_int = 1;

/* We don't have a real pci vendor/device id, the following will need to be
 * patched to match prototype hardware.
 */
const BOGUS_PCI_VENDOR_ID_FLUKE: u16 = 0xffff;
const BOGUS_PCI_DEVICE_ID_FLUKE_BLADERUNNER: u16 = 0x0;

#[repr(C)]
pub struct fmh_priv {
    pub nec7210_priv: nec7210_priv,
    pub gpib_iomem_res: *mut resource,
    pub write_transfer_counter_res: *mut resource,
    pub dma_port_res: *mut resource,
    pub irq: ::core::ffi::c_int,
    pub dma_channel: *mut dma_chan,
    pub dma_buffer: *mut u8,
    pub dma_buffer_size: ::core::ffi::c_int,
    pub dma_burst_length: ::core::ffi::c_int,
    pub fifo_base: *mut ::core::ffi::c_void,
    pub supports_fifo_interrupts: u8,
}

#[inline]
unsafe fn fmh_gpib_half_fifo_size(priv_: *mut fmh_priv) -> ::core::ffi::c_int {
    (*priv_).dma_burst_length
}

// Registers beyond the nec7210 register set.
#[repr(u32)]
pub enum fmh_gpib_regs {
    EXT_STATUS_1_REG = 0x9,
    STATE1_REG = 0xc,
    ISR0_IMR0_REG = 0xe,
    BUS_STATUS_REG = 0xf,
}

/* IMR0 -- Interrupt Mode Register 0 */
#[repr(u32)]
pub enum imr0_bits {
    ATN_INTERRUPT_ENABLE_BIT = 0x4,
    IFC_INTERRUPT_ENABLE_BIT = 0x8,
}

/* ISR0 -- Interrupt Status Register 0 */
#[repr(u32)]
pub enum isr0_bits {
    ATN_INTERRUPT_BIT = 0x4,
    IFC_INTERRUPT_BIT = 0x8,
}

#[repr(u32)]
pub enum state1_bits {
    SOURCE_HANDSHAKE_SIDS_BITS = 0x0,
    SOURCE_HANDSHAKE_SGNS_BITS = 0x1,
    SOURCE_HANDSHAKE_SDYS_BITS = 0x2,
    SOURCE_HANDSHAKE_STRS_BITS = 0x5,
    SOURCE_HANDSHAKE_MASK = 0x7,
}

#[repr(u32)]
pub enum fmh_gpib_auxmr_bits { AUX_I_REG = 0xe0 }

#[repr(u32)]
pub enum aux_reg_i_bits { LOCAL_PPOLL_MODE_BIT = 0x4 }

#[repr(u32)]
pub enum ext_status_1_bits {
    DATA_IN_STATUS_BIT = 0x01,
    DATA_OUT_STATUS_BIT = 0x02,
    COMMAND_OUT_STATUS_BIT = 0x04,
    RFD_HOLDOFF_STATUS_BIT = 0x08,
    END_STATUS_BIT = 0x10,
}

/* dma fifo reg and bits */
#[repr(u32)]
pub enum dma_fifo_regs {
    FIFO_DATA_REG = 0x0,
    FIFO_CONTROL_STATUS_REG = 0x1,
    FIFO_XFER_COUNTER_REG = 0x2,
    FIFO_MAX_BURST_LENGTH_REG = 0x3,
}

#[repr(u32)]
pub enum fifo_data_bits { FIFO_DATA_EOI_FLAG = 0x100 }

#[repr(u32)]
pub enum fifo_control_bits {
    TX_FIFO_DMA_REQUEST_ENABLE = 0x0001,
    TX_FIFO_CLEAR = 0x0002,
    TX_FIFO_HALF_EMPTY_INTERRUPT_ENABLE = 0x0008,
    RX_FIFO_DMA_REQUEST_ENABLE = 0x0100,
    RX_FIFO_CLEAR = 0x0200,
    RX_FIFO_HALF_FULL_INTERRUPT_ENABLE = 0x0800,
}

#[repr(u32)]
pub enum fifo_status_bits {
    TX_FIFO_EMPTY = 0x0001,
    TX_FIFO_FULL = 0x0002,
    TX_FIFO_HALF_EMPTY = 0x0004,
    TX_FIFO_HALF_EMPTY_INTERRUPT_IS_ENABLED = 0x0008,
    TX_FIFO_DMA_REQUEST_IS_ENABLED = 0x0010,
    RX_FIFO_EMPTY = 0x0100,
    RX_FIFO_FULL = 0x0200,
    RX_FIFO_HALF_FULL = 0x0400,
    RX_FIFO_HALF_FULL_INTERRUPT_IS_ENABLED = 0x0800,
    RX_FIFO_DMA_REQUEST_IS_ENABLED = 0x1000,
}

static FIFO_DATA_MASK: u32 = 0x00ff;
static FIFO_XFER_COUNTER_MASK: u32 = 0x0fff;
static FIFO_MAX_BURST_LENGTH_MASK: u32 = 0x00ff;

#[inline]
unsafe fn gpib_cs_read_byte(nec_priv: *mut nec7210_priv, register_num: u32) -> u8 {
    readb((*nec_priv).mmiobase.add(register_num as usize * (*nec_priv).offset as usize))
}

#[inline]
unsafe fn gpib_cs_write_byte(nec_priv: *mut nec7210_priv, data: u8, register_num: u32) {
    writeb(data, (*nec_priv).mmiobase.add(register_num as usize * (*nec_priv).offset as usize));
}

#[inline]
unsafe fn fifos_read(fmh_priv: *mut fmh_priv, register_num: ::core::ffi::c_int) -> u16 {
    if (*fmh_priv).fifo_base.is_null() { return 0; }
    readw((*fmh_priv).fifo_base.add(register_num as usize * FIFO_REG_OFFSET as usize))
}

#[inline]
unsafe fn fifos_write(fmh_priv: *mut fmh_priv, data: u16, register_num: ::core::ffi::c_int) {
    if (*fmh_priv).fifo_base.is_null() { return; }
    writew(data, (*fmh_priv).fifo_base.add(register_num as usize * FIFO_REG_OFFSET as usize));
}

#[repr(u32)]
pub enum bus_status_bits {
    BSR_ATN_BIT = 0x01, BSR_EOI_BIT = 0x02, BSR_SRQ_BIT = 0x04, BSR_IFC_BIT = 0x08,
    BSR_REN_BIT = 0x10, BSR_DAV_BIT = 0x20, BSR_NRFD_BIT = 0x40, BSR_NDAC_BIT = 0x80,
}

#[repr(u32)]
pub enum fmh_gpib_aux_cmds {
    /* AUX_RTL2 asserts and keeps asserted the local rtl message. */
    AUX_RTL2 = 0x0d,
    AUX_RFD_HOLDOFF_ASAP = 0x15,
    AUX_REQT = 0x18,
    AUX_REQF = 0x19,
    AUX_LO_SPEED = 0x40,
    AUX_HI_SPEED = 0x41,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
