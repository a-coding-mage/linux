/* SPDX-License-Identifier: GPL-2.0 */
/*
 *	OSS
 *
 *	This is used in place of VIA2 on the IIfx.
 */

pub const OSS_BASE: usize = 0x50f1a000;

/*
 * Interrupt level offsets for mac_oss->irq_level
 */

pub const OSS_NUBUS0: usize = 0;
pub const OSS_NUBUS1: usize = 1;
pub const OSS_NUBUS2: usize = 2;
pub const OSS_NUBUS3: usize = 3;
pub const OSS_NUBUS4: usize = 4;
pub const OSS_NUBUS5: usize = 5;
pub const OSS_IOPISM: usize = 6;
pub const OSS_IOPSCC: usize = 7;
pub const OSS_SOUND: usize = 8;
pub const OSS_SCSI: usize = 9;
pub const OSS_60HZ: usize = 10;
pub const OSS_VIA1: usize = 11;
pub const OSS_UNUSED1: usize = 12;
pub const OSS_UNUSED2: usize = 13;
pub const OSS_PARITY: usize = 14;
pub const OSS_UNUSED3: usize = 15;

pub const OSS_NUM_SOURCES: usize = 16;

/*
 * Pending interrupt bits in mac_oss->irq_pending
 */

pub const OSS_IP_NUBUS0: u16 = 0x0001;
pub const OSS_IP_NUBUS1: u16 = 0x0002;
pub const OSS_IP_NUBUS2: u16 = 0x0004;
pub const OSS_IP_NUBUS3: u16 = 0x0008;
pub const OSS_IP_NUBUS4: u16 = 0x0010;
pub const OSS_IP_NUBUS5: u16 = 0x0020;
pub const OSS_IP_IOPISM: u16 = 0x0040;
pub const OSS_IP_IOPSCC: u16 = 0x0080;
pub const OSS_IP_SOUND: u16 = 0x0100;
pub const OSS_IP_SCSI: u16 = 0x0200;
pub const OSS_IP_60HZ: u16 = 0x0400;
pub const OSS_IP_VIA1: u16 = 0x0800;
pub const OSS_IP_UNUSED1: u16 = 0x1000;
pub const OSS_IP_UNUSED2: u16 = 0x2000;
pub const OSS_IP_PARITY: u16 = 0x4000;
pub const OSS_IP_UNUSED3: u16 = 0x8000;

pub const OSS_IP_NUBUS: u16 = OSS_IP_NUBUS0
    | OSS_IP_NUBUS1
    | OSS_IP_NUBUS2
    | OSS_IP_NUBUS3
    | OSS_IP_NUBUS4
    | OSS_IP_NUBUS5;

/*
 * Rom Control Register
 */

pub const OSS_POWEROFF: u8 = 0x80;

#[repr(C)]
pub struct mac_oss {
    pub irq_level: [u8; 0x10],             /* [0x000-0x00f] Interrupt levels */
    pub padding0: [u8; 0x1F2],             /* [0x010-0x201] IO space filler */
    pub irq_pending: u16,                  /* [0x202-0x203] pending interrupts bits */
    pub rom_ctrl: u8,                      /* [0x204-0x204] ROM cntl reg (for poweroff) */
    pub padding1: [u8; 0x2],               /* [0x205-0x206] currently unused by A/UX */
    pub ack_60hz: u8,                     /* [0x207-0x207] 60 Hz ack. */
}

extern "C" {
    pub static mut oss: *mut mac_oss;
    pub static mut oss_present: core::ffi::c_int;

    pub fn oss_register_interrupts();
    pub fn oss_irq_enable(_: core::ffi::c_int);
    pub fn oss_irq_disable(_: core::ffi::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
