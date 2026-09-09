/*
** asm-m68k/pcmcia.h -- Amiga Linux PCMCIA Definitions
**
** Copyright 1997 by Alain Malek
**
** This file is subject to the terms and conditions of the GNU General Public
** License.  See the file COPYING in the main directory of this archive
** for more details.
**
** Created: 12/10/97 by Alain Malek
*/

// Dependency supplied by <asm/amigayle.h>.

extern "C" {
    pub fn pcmcia_reset();
    pub fn pcmcia_copy_tuple(tuple_id: u8, tuple: *mut core::ffi::c_void, max_len: i32) -> i32;
    pub fn pcmcia_program_voltage(voltage: i32);
    pub fn pcmcia_access_speed(speed: i32);
    pub fn pcmcia_write_enable();
    pub fn pcmcia_write_disable();
}

// The definition of Gayle and the external `gayle` object are provided by
// <asm/amigayle.h> in the translated dependency set.
#[repr(C)]
pub struct Gayle {
    pub cardstatus: u8,
    pub intreq: u8,
    pub inten: u8,
}

extern "C" {
    pub static mut gayle: Gayle;
}

pub const PCMCIA_INSERTED: u8 = unsafe { gayle.cardstatus & GAYLE_CS_CCDET };

pub const PCMCIA_0V: i32 = 0;
pub const PCMCIA_5V: i32 = 5;
pub const PCMCIA_12V: i32 = 12;

pub const PCMCIA_SPEED_100NS: i32 = 100;
pub const PCMCIA_SPEED_150NS: i32 = 150;
pub const PCMCIA_SPEED_250NS: i32 = 250;
pub const PCMCIA_SPEED_720NS: i32 = 720;

pub const CISTPL_NULL: u8 = 0x00;
pub const CISTPL_DEVICE: u8 = 0x01;
pub const CISTPL_LONGLINK_CB: u8 = 0x02;
pub const CISTPL_CONFIG_CB: u8 = 0x04;
pub const CISTPL_CFTABLE_ENTRY_CB: u8 = 0x05;
pub const CISTPL_LONGLINK_MFC: u8 = 0x06;
pub const CISTPL_BAR: u8 = 0x07;
pub const CISTPL_CHECKSUM: u8 = 0x10;
pub const CISTPL_LONGLINK_A: u8 = 0x11;
pub const CISTPL_LONGLINK_C: u8 = 0x12;
pub const CISTPL_LINKTARGET: u8 = 0x13;
pub const CISTPL_NO_LINK: u8 = 0x14;
pub const CISTPL_VERS_1: u8 = 0x15;
pub const CISTPL_ALTSTR: u8 = 0x16;
pub const CISTPL_DEVICE_A: u8 = 0x17;
pub const CISTPL_JEDEC_C: u8 = 0x18;
pub const CISTPL_JEDEC_A: u8 = 0x19;
pub const CISTPL_CONFIG: u8 = 0x1a;
pub const CISTPL_CFTABLE_ENTRY: u8 = 0x1b;
pub const CISTPL_DEVICE_OC: u8 = 0x1c;
pub const CISTPL_DEVICE_OA: u8 = 0x1d;
pub const CISTPL_DEVICE_GEO: u8 = 0x1e;
pub const CISTPL_DEVICE_GEO_A: u8 = 0x1f;
pub const CISTPL_MANFID: u8 = 0x20;
pub const CISTPL_FUNCID: u8 = 0x21;
pub const CISTPL_FUNCE: u8 = 0x22;
pub const CISTPL_SWIL: u8 = 0x23;
pub const CISTPL_END: u8 = 0xff;

pub const CISTPL_FUNCID_MULTI: u8 = 0x00;
pub const CISTPL_FUNCID_MEMORY: u8 = 0x01;
pub const CISTPL_FUNCID_SERIAL: u8 = 0x02;
pub const CISTPL_FUNCID_PARALLEL: u8 = 0x03;
pub const CISTPL_FUNCID_FIXED: u8 = 0x04;
pub const CISTPL_FUNCID_VIDEO: u8 = 0x05;
pub const CISTPL_FUNCID_NETWORK: u8 = 0x06;
pub const CISTPL_FUNCID_AIMS: u8 = 0x07;
pub const CISTPL_FUNCID_SCSI: u8 = 0x08;

#[inline]
pub unsafe fn pcmcia_read_status() -> u8 {
    gayle.cardstatus & 0x7c
}

#[inline]
pub unsafe fn pcmcia_get_intreq() -> u8 {
    gayle.intreq
}

#[inline]
pub unsafe fn pcmcia_ack_int(_intreq: u8) {
    gayle.intreq = 0xf8;
}

#[inline]
pub unsafe fn pcmcia_enable_irq() {
    gayle.inten |= GAYLE_IRQ_IRQ;
}

#[inline]
pub unsafe fn pcmcia_disable_irq() {
    gayle.inten &= !GAYLE_IRQ_IRQ;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
