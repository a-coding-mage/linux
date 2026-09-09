/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Machine specific NMI handling for generic.
 *  Split out from traps.c by Osamu Tomita <tomita@cinet.co.jp>
 */

// Dependency supplied by asm/mc146818rtc.h in the C source.

pub const NMI_REASON_PORT: u16 = 0x61;

pub const NMI_REASON_SERR: u8 = 0x80;
pub const NMI_REASON_IOCHK: u8 = 0x40;
pub const NMI_REASON_MASK: u8 = NMI_REASON_SERR | NMI_REASON_IOCHK;

pub const NMI_REASON_CLEAR_SERR: u8 = 0x04;
pub const NMI_REASON_CLEAR_IOCHK: u8 = 0x08;
pub const NMI_REASON_CLEAR_MASK: u8 = 0x0f;

pub unsafe fn default_get_nmi_reason() -> u8 {
	unsafe { inb(NMI_REASON_PORT) }
}

pub unsafe fn reassert_nmi() {
	let mut old_reg: i32 = -1;

	if unsafe { do_i_have_lock_cmos() } {
		old_reg = unsafe { current_lock_cmos_reg() };
	} else {
		unsafe { lock_cmos(0) }; /* register doesn't matter here */
	}
	unsafe { outb(0x8f, 0x70) };
	unsafe { inb(0x71) }; /* dummy */
	unsafe { outb(0x0f, 0x70) };
	unsafe { inb(0x71) }; /* dummy */
	if old_reg >= 0 {
		unsafe { outb(old_reg as u8, 0x70) };
	} else {
		unsafe { unlock_cmos() };
	}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
