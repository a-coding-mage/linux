/* SPDX-License-Identifier: GPL-2.0
 *
 * include/asm-sh/snapgear.h
 *
 * Modified version of io_se.h for the snapgear-specific functions.
 *
 * IO functions for a SnapGear
 */

// C header guard: _ASM_SH_IO_SNAPGEAR_H

// C preprocessor configuration: __IO_PREFIX is snapgear.
// Dependency supplied by the surrounding build: <asm/io_generic.h>

/*
 * We need to remember what was written to the ioport as some bits
 * are shared with other functions and you cannot read back what was
 * written :-|
 *
 * Bit        Read                   Write
 * -----------------------------------------------
 * D0         DCD on ttySC1          power
 * D1         Reset Switch           heatbeat
 * D2         ttySC0 CTS (7100)      LAN
 * D3         -                      WAN
 * D4         ttySC0 DCD (7100)      CONSOLE
 * D5         -                      ONLINE
 * D6         -                      VPN
 * D7         DTR on ttySC1
 * D8         -                      ttySC0 RTS (7100)
 * D9         -                      ttySC0 DTR (7100)
 * D10        -                      RTC SCLK
 * D11        RTC DATA               RTC DATA
 * D12        -                      RTS RESET
 */

pub const SECUREEDGE_IOPORT_ADDR: *mut core::ffi::c_void = 0xb0000000usize as *mut core::ffi::c_void;

pub static mut secureedge5410_ioport: u16 = 0;

#[inline]
pub unsafe fn SECUREEDGE_WRITE_IOPORT(val: u16, mask: u16) {
    let value = (secureedge5410_ioport & !mask) | (val & mask);
    secureedge5410_ioport = value;
    core::ptr::write_volatile(SECUREEDGE_IOPORT_ADDR as *mut i16, value as i16);
}

#[inline]
pub unsafe fn SECUREEDGE_READ_IOPORT() -> u16 {
    (core::ptr::read_volatile(SECUREEDGE_IOPORT_ADDR as *const i16) as u16 & 0x0817)
        | (secureedge5410_ioport & !0x0817)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
