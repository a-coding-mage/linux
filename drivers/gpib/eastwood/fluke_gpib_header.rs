/* SPDX-License-Identifier: GPL-2.0 */

/***************************************************************************
 *   Author: Frank Mori Hess <fmh6jj@gmail.com>
 *   copyright: (C) 2006, 2010, 2015 Fluke Corporation
 ***************************************************************************/

// Dependencies supplied by the surrounding driver and kernel bindings.

#[repr(C)]
pub struct fluke_priv {
    pub nec7210_priv: nec7210_priv,
    pub gpib_iomem_res: *mut resource,
    pub write_transfer_counter_res: *mut resource,
    pub dma_port_res: *mut resource,
    pub irq: ::core::ffi::c_int,
    pub dma_channel: *mut dma_chan,
    pub dma_buffer: *mut u8,
    pub dma_buffer_size: ::core::ffi::c_int,
    pub write_transfer_counter: *mut ::core::ffi::c_void,
}

// cb7210 specific registers and bits
#[repr(i32)]
pub enum cb7210_regs {
    STATE1_REG = 0x4,
    ISR0_IMR0 = 0x6,
    BUS_STATUS = 0x7,
}

#[repr(i32)]
pub enum cb7210_page_in {
    ISR0_IMR0_PAGE = 1,
    BUS_STATUS_PAGE = 1,
    STATE1_PAGE = 1,
}

/* IMR0 -- Interrupt Mode Register 0 */
#[repr(i32)]
pub enum imr0_bits {
    FLUKE_IFCIE_BIT = 0x8, /* interface clear interrupt */
}

/* ISR0 -- Interrupt Status Register 0 */
#[repr(i32)]
pub enum isr0_bits {
    FLUKE_IFCI_BIT = 0x8, /* interface clear interrupt */
}

#[repr(i32)]
pub enum state1_bits {
    SOURCE_HANDSHAKE_SIDS_BITS = 0x0, /* source idle state */
    SOURCE_HANDSHAKE_SGNS_BITS = 0x1, /* source generate state */
    SOURCE_HANDSHAKE_SDYS_BITS = 0x2, /* source delay state */
    SOURCE_HANDSHAKE_STRS_BITS = 0x5, /* source transfer state */
    SOURCE_HANDSHAKE_MASK = 0x7,
}

/*
 * we customized the cb7210 vhdl to give the "data in" status
 * on the unused bit 7 of the address0 register.
 */
#[repr(i32)]
pub enum cb7210_address0 {
    DATA_IN_STATUS = 0x80,
}

pub const fn cb7210_page_in_bits(page: u32) -> i32 {
    0x50 | (page & 0xf) as i32
}

// don't use without locking nec_priv->register_page_lock
pub unsafe fn fluke_read_byte_nolock(nec_priv: *mut nec7210_priv,
                                     register_num: i32) -> u8 {
    readl((*nec_priv).mmiobase.add(register_num as usize * (*nec_priv).offset as usize)) as u8
}

// don't use without locking nec_priv->register_page_lock
pub unsafe fn fluke_write_byte_nolock(nec_priv: *mut nec7210_priv, data: u8,
                                      register_num: i32) {
    writel(data as _, (*nec_priv).mmiobase.add(register_num as usize * (*nec_priv).offset as usize));
}

pub unsafe fn fluke_paged_read_byte(e_priv: *mut fluke_priv,
                                    register_num: u32, page: u32) -> u8 {
    let nec_priv = &mut (*e_priv).nec7210_priv as *mut nec7210_priv;
    let mut flags: usize = 0;

    spin_lock_irqsave(&mut (*nec_priv).register_page_lock, &mut flags);
    fluke_write_byte_nolock(nec_priv, cb7210_page_in_bits(page) as u8, AUXMR);
    udelay(1);
    /* chip auto clears the page after a read */
    let retval = fluke_read_byte_nolock(nec_priv, register_num as i32);
    spin_unlock_irqrestore(&mut (*nec_priv).register_page_lock, flags);
    retval
}

pub unsafe fn fluke_paged_write_byte(e_priv: *mut fluke_priv, data: u8,
                                     register_num: u32, page: u32) {
    let nec_priv = &mut (*e_priv).nec7210_priv as *mut nec7210_priv;
    let mut flags: usize = 0;

    spin_lock_irqsave(&mut (*nec_priv).register_page_lock, &mut flags);
    fluke_write_byte_nolock(nec_priv, cb7210_page_in_bits(page) as u8, AUXMR);
    udelay(1);
    fluke_write_byte_nolock(nec_priv, data, register_num as i32);
    spin_unlock_irqrestore(&mut (*nec_priv).register_page_lock, flags);
}

#[repr(i32)]
pub enum bus_status_bits {
    BSR_ATN_BIT = 0x1,
    BSR_EOI_BIT = 0x2,
    BSR_SRQ_BIT = 0x4,
    BSR_IFC_BIT = 0x8,
    BSR_REN_BIT = 0x10,
    BSR_DAV_BIT = 0x20,
    BSR_NRFD_BIT = 0x40,
    BSR_NDAC_BIT = 0x80,
}

#[repr(i32)]
pub enum cb7210_aux_cmds {
    /*
     * AUX_RTL2 is an undocumented aux command which causes cb7210 to assert
     * (and keep asserted) local rtl message.  This is used in conjunction
     * with the (stupid) cb7210 implementation
     * of the normal nec7210 AUX_RTL aux command, which
     * causes the rtl message to toggle between on and off.
     */
    AUX_RTL2 = 0xd,
    AUX_NBAF = 0xe, // new byte available false (also clears seoi)
    AUX_LO_SPEED = 0x40,
    AUX_HI_SPEED = 0x41,
}

pub const fluke_reg_offset: i32 = 4;
pub const fluke_num_regs: i32 = 8;
pub const write_transfer_counter_mask: i32 = 0x7ff;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
