// SPDX-License-Identifier: GPL-2.0
/*
 * MPC5200 PSC serial console support.
 *
 * Author: Grant Likely <grant.likely@secretlab.ca>
 *
 * Copyright (c) 2007 Secret Lab Technologies Ltd.
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 *
 * It is assumed that the firmware (or the platform file) has already set
 * up the port.
 */

// Dependencies supplied by the surrounding platform implementation:
// types.h, io.h, and ops.h.

/* Programmable Serial Controller (PSC) status register bits */
const MPC52XX_PSC_SR: usize = 0x04;
const MPC52XX_PSC_SR_RXRDY: u16 = 0x0100;
const MPC52XX_PSC_SR_RXFULL: u16 = 0x0200;
const MPC52XX_PSC_SR_TXRDY: u16 = 0x0400;
const MPC52XX_PSC_SR_TXEMP: u16 = 0x0800;

const MPC52XX_PSC_BUFFER: usize = 0x0C;

extern "C" {
    fn in_be16(addr: *mut u8) -> u16;
    fn in_8(addr: *mut u8) -> u8;
    fn out_8(addr: *mut u8, value: u8);
    fn dt_get_virtual_reg(devp: *mut core::ffi::c_void,
                          reg: *mut *mut u8,
                          count: i32) -> i32;
}

#[repr(C)]
pub struct serial_console_data {
    pub open: Option<unsafe extern "C" fn() -> i32>,
    pub putc: Option<unsafe extern "C" fn(u8)>,
    pub getc: Option<unsafe extern "C" fn() -> u8>,
    pub tstc: Option<unsafe extern "C" fn() -> u8>,
}

static mut psc: *mut u8 = core::ptr::null_mut();

unsafe extern "C" fn psc_open() -> i32 {
    /* Assume the firmware has already configured the PSC into
     * uart mode */
    0
}

unsafe extern "C" fn psc_putc(c: u8) {
    while (in_be16(psc.add(MPC52XX_PSC_SR)) & MPC52XX_PSC_SR_TXRDY) == 0 {}
    out_8(psc.add(MPC52XX_PSC_BUFFER), c);
}

unsafe extern "C" fn psc_tstc() -> u8 {
    ((in_be16(psc.add(MPC52XX_PSC_SR)) & MPC52XX_PSC_SR_RXRDY) != 0) as u8
}

unsafe extern "C" fn psc_getc() -> u8 {
    while (in_be16(psc.add(MPC52XX_PSC_SR)) & MPC52XX_PSC_SR_RXRDY) == 0 {}
    in_8(psc.add(MPC52XX_PSC_BUFFER))
}

pub unsafe extern "C" fn mpc5200_psc_console_init(
    devp: *mut core::ffi::c_void,
    scdp: *mut serial_console_data,
) -> i32 {
    /* Get the base address of the psc registers */
    if dt_get_virtual_reg(devp, &raw mut psc, 1) < 1 {
        return -1;
    }

    (*scdp).open = Some(psc_open);
    (*scdp).putc = Some(psc_putc);
    (*scdp).getc = Some(psc_getc);
    (*scdp).tstc = Some(psc_tstc);

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
