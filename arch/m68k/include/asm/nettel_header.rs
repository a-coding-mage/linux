/* SPDX-License-Identifier: GPL-2.0 */
/****************************************************************************/
/*
 * nettel.h -- Lineo (formerly Moreton Bay) NETtel support.
 *
 * (C) Copyright 1999-2000, Moreton Bay (www.moretonbay.com)
 * (C) Copyright 2000-2001, Lineo Inc. (www.lineo.com)
 * (C) Copyright 2001-2002, SnapGear Inc., (www.snapgear.com)
 */
/****************************************************************************/

/* Original file was enabled by CONFIG_NETtel or CONFIG_CLEOPATRA. */

/*---------------------------------------------------------------------------*/
#[cfg(any(feature = "CONFIG_M5307", feature = "CONFIG_M5407"))]
/*
 * NETtel/5307 based hardware first. DTR/DCD lines are wired to GPIO lines.
 * Most of the LEDs are driven through a latch connected to CS2.
 */
pub const MCFPP_DCD1: u32 = 0x0001;
pub const MCFPP_DCD0: u32 = 0x0002;
pub const MCFPP_DTR1: u32 = 0x0004;
pub const MCFPP_DTR0: u32 = 0x0008;

#[cfg(not(feature = "ASSEMBLER"))]
pub const NETtel_LEDADDR: u32 = 0x3040_0000;

#[cfg(not(feature = "ASSEMBLER"))]
unsafe extern "C" {
    pub static mut ppdata: u16;
}

#[cfg(not(feature = "ASSEMBLER"))]
pub unsafe fn mcf_getppdata() -> u32 {
    let pp = MCFSIM_PADAT as *const u16;
    core::ptr::read_volatile(pp) as u32
}

#[cfg(not(feature = "ASSEMBLER"))]
pub unsafe fn mcf_setppdata(mask: u32, bits: u32) {
    let pp = MCFSIM_PADAT as *mut u16;
    ppdata = (ppdata & !mask as u16) | bits as u16;
    core::ptr::write_volatile(pp, ppdata);
}

/*---------------------------------------------------------------------------*/
#[cfg(feature = "CONFIG_M5206e")]
/*
 * NETtel/5206e based hardware has LEDs on latch on CS3.
 * No support modem for lines??
 */
pub const NETtel_LEDADDR: u32 = 0x5000_0000;

/*---------------------------------------------------------------------------*/
#[cfg(feature = "CONFIG_M5272")]
/* NETtel/5272 based hardware. DTR/DCD lines are wired to GPB lines. */
pub const MCFPP_DCD0: u32 = 0x0080;
pub const MCFPP_DCD1: u32 = 0x0000; /* Port 1 no DCD support */
pub const MCFPP_DTR0: u32 = 0x0040;
pub const MCFPP_DTR1: u32 = 0x0000; /* Port 1 no DTR support */

#[cfg(feature = "CONFIG_M5272")]
pub unsafe fn mcf_getppdata() -> u32 {
    mcf_read16(MCFSIM_PBDAT)
}

#[cfg(feature = "CONFIG_M5272")]
pub unsafe fn mcf_setppdata(mask: u32, bits: u32) {
    mcf_write16((mcf_read16(MCFSIM_PBDAT) & !mask) | bits, MCFSIM_PBDAT);
}

/* MCFSIM_PADAT, MCFSIM_PBDAT, mcf_read16, and mcf_write16 are supplied by
 * the platform dependencies included by the original header. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
