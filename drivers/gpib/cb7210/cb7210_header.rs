/* SPDX-License-Identifier: GPL-2.0 */

/***************************************************************************
 *    copyright            : (C) 2002 by Frank Mori Hess
 ***************************************************************************/

// C dependencies: nec7210.h, gpibP.h, amccs5933.h, linux/delay.h,
// linux/interrupt.h. Their declarations are supplied by the surrounding
// translation unit.

#[repr(u32)]
pub enum _Cb7210PciDeviceId {
    PCI_DEVICE_ID_CBOARDS_PCI_GPIB = 0x6,
    PCI_DEVICE_ID_CBOARDS_CPCI_GPIB = 0xe,
}

#[repr(u32)]
pub enum pci_chip {
    PCI_CHIP_NONE = 0,
    PCI_CHIP_AMCC_S5933,
    PCI_CHIP_QUANCOM,
}

// struct which defines private_data for cb7210 boards
#[repr(C)]
pub struct cb7210_priv {
    pub nec7210_priv: nec7210_priv,
    pub pci_device: *mut pci_dev,
    // base address of amccs5933 pci chip
    pub amcc_iobase: libc::c_ulong,
    pub fifo_iobase: libc::c_ulong,
    pub irq: libc::c_uint,
    pub pci_chip: pci_chip,
    pub hs_mode_bits: u8,
    pub out_fifo_half_empty: u32,
    pub in_fifo_half_full: u32,
}

// pci-gpib register offset
pub const cb7210_reg_offset: libc::c_int = 1;
// uses 10 ioports
pub const cb7210_iosize: libc::c_int = 10;
// fifo size in bytes
pub const cb7210_fifo_size: libc::c_int = 2048;
pub const cb7210_fifo_width: libc::c_int = 2;

#[repr(u32)]
pub enum cb7210_regs { BUS_STATUS = 0x7 }
#[repr(u32)]
pub enum cb7210_page_in { BUS_STATUS_PAGE = 1 }
#[repr(u32)]
pub enum hs_regs {
    HS_MODE = 0x8,
    HS_INT_LEVEL = 0x9,
    HS_STATUS = 0x8,
}

#[inline]
pub unsafe fn nec7210_iobase(cb_priv: *const cb7210_priv) -> u32 {
    (*cb_priv).nec7210_priv.iobase
}

#[inline]
pub fn cb7210_page_in_bits(page: libc::c_uint) -> libc::c_int {
    0x50 | (page & 0xf) as libc::c_int
}

#[inline]
pub unsafe fn cb7210_paged_read_byte(
    cb_priv: *mut cb7210_priv, register_num: libc::c_uint, page: libc::c_uint,
) -> u8 {
    let nec_priv: *mut nec7210_priv = &mut (*cb_priv).nec7210_priv;
    let mut flags: libc::c_ulong = 0;
    spin_lock_irqsave(&mut (*nec_priv).register_page_lock, &mut flags);
    outb(cb7210_page_in_bits(page) as u8,
         nec7210_iobase(cb_priv) + AUXMR * (*nec_priv).offset);
    udelay(1);
    let retval = inb(nec7210_iobase(cb_priv) + register_num * (*nec_priv).offset);
    spin_unlock_irqrestore(&mut (*nec_priv).register_page_lock, flags);
    retval
}

// don't use for register_num < 8, since it doesn't lock
#[inline]
pub unsafe fn cb7210_read_byte(cb_priv: *const cb7210_priv, register_num: hs_regs) -> u8 {
    let nec_priv = &(*cb_priv).nec7210_priv;
    inb(nec7210_iobase(cb_priv) + (register_num as u32) * nec_priv.offset)
}

#[inline]
pub unsafe fn cb7210_paged_write_byte(
    cb_priv: *mut cb7210_priv, data: u8, register_num: libc::c_uint, page: libc::c_uint,
) {
    let nec_priv: *mut nec7210_priv = &mut (*cb_priv).nec7210_priv;
    let mut flags: libc::c_ulong = 0;
    spin_lock_irqsave(&mut (*nec_priv).register_page_lock, &mut flags);
    outb(cb7210_page_in_bits(page) as u8,
         nec7210_iobase(cb_priv) + AUXMR * (*nec_priv).offset);
    udelay(1);
    outb(data, nec7210_iobase(cb_priv) + register_num * (*nec_priv).offset);
    spin_unlock_irqrestore(&mut (*nec_priv).register_page_lock, flags);
}

// don't use for register_num < 8, since it doesn't lock
#[inline]
pub unsafe fn cb7210_write_byte(cb_priv: *const cb7210_priv, data: u8, register_num: hs_regs) {
    let nec_priv = &(*cb_priv).nec7210_priv;
    outb(data, nec7210_iobase(cb_priv) + (register_num as u32) * nec_priv.offset);
}

#[repr(u32)]
pub enum bus_status_bits {
    BSR_ATN_BIT = 0x1, BSR_EOI_BIT = 0x2, BSR_SRQ_BIT = 0x4, BSR_IFC_BIT = 0x8,
    BSR_REN_BIT = 0x10, BSR_DAV_BIT = 0x20, BSR_NRFD_BIT = 0x40, BSR_NDAC_BIT = 0x80,
}

/* CBI 488.2 HS control */
#[repr(u32)]
pub enum hs_mode_bits {
    HS_ENABLE_MASK = 0x3, HS_TX_ENABLE = 1 << 0, HS_RX_ENABLE = 1 << 1,
    HS_HF_INT_EN = 1 << 3, HS_CLR_SRQ_INT = 1 << 4, HS_CLR_EOI_EMPTY_INT = 1 << 5,
    HS_CLR_HF_INT = 1 << 6, HS_SYS_CONTROL = 1 << 7,
}

/* CBI 488.2 status */
#[repr(u32)]
pub enum hs_status_bits {
    HS_FIFO_FULL = 1 << 0, HS_HALF_FULL = 1 << 1, HS_SRQ_INT = 1 << 2, HS_EOI_INT = 1 << 3,
    HS_TX_MSB_NOT_EMPTY = 1 << 4, HS_RX_MSB_NOT_EMPTY = 1 << 5,
    HS_TX_LSB_NOT_EMPTY = 1 << 6, HS_RX_LSB_NOT_EMPTY = 1 << 7,
}

/* CBI488.2 hs_int_level register */
#[repr(u32)]
pub enum hs_int_level_bits { HS_RESET7210 = 1 << 7 }

#[inline]
pub fn irq_bits(irq: libc::c_uint) -> libc::c_uint {
    match irq { 2 | 3 | 4 | 5 => irq - 1, 7 => 0x5, 10 => 0x6, 11 => 0x7, _ => 0 }
}

#[repr(u32)]
pub enum cb7210_aux_cmds {
    /* AUX_RTL2 is an undocumented aux command which causes cb7210 to assert
     * (and keep asserted) local rtl message. This is used in conjunction
     * with the (stupid) cb7210 implementation of the normal nec7210 AUX_RTL
     * aux command, which causes the rtl message to toggle between on and off.
     */
    AUX_RTL2 = 0xd,
    AUX_LO_SPEED = 0x40,
    AUX_HI_SPEED = 0x41,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
