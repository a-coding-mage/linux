/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * ioc.h: Definitions for SGI I/O Controller
 *
 * Copyright (C) 1996 David S. Miller
 * Copyright (C) 1997, 1998, 1999, 2000 Ralf Baechle
 * Copyright (C) 2001, 2003 Ladislav Michl
 */

/* Dependency supplied by the surrounding translation unit: pi1_regs, HZ. */

/* All registers are 8-bit wide aligned on 32-bit boundary. */
#[repr(C)]
pub struct sgioc_uart_regs {
    pub _ctrl1: [u8; 3], pub ctrl1: u8,
    pub _data1: [u8; 3], pub data1: u8,
    pub _ctrl2: [u8; 3], pub ctrl2: u8,
    pub _data2: [u8; 3], pub data2: u8,
}

#[repr(C)]
pub struct sgioc_keyb_regs {
    pub _data: [u8; 3], pub data: u8,
    pub _command: [u8; 3], pub command: u8,
}

#[repr(C)]
pub struct sgint_regs {
    pub _istat0: [u8; 3], pub istat0: u8,
    pub _imask0: [u8; 3], pub imask0: u8,
    pub _istat1: [u8; 3], pub istat1: u8,
    pub _imask1: [u8; 3], pub imask1: u8,
    pub _vmeistat: [u8; 3], pub vmeistat: u8,
    pub _cmeimask0: [u8; 3], pub cmeimask0: u8,
    pub _cmeimask1: [u8; 3], pub cmeimask1: u8,
    pub _cmepol: [u8; 3], pub cmepol: u8,
    pub _tclear: [u8; 3], pub tclear: u8,
    pub _errstat: [u8; 3], pub errstat: u8,
    pub _unused0: [u32; 2],
    pub _tcnt0: [u8; 3], pub tcnt0: u8,
    pub _tcnt1: [u8; 3], pub tcnt1: u8,
    pub _tcnt2: [u8; 3], pub tcnt2: u8,
    pub _tcword: [u8; 3], pub tcword: u8,
}

pub const SGINT_ISTAT0_FFULL: u8 = 0x01;
pub const SGINT_ISTAT0_SCSI0: u8 = 0x02;
pub const SGINT_ISTAT0_SCSI1: u8 = 0x04;
pub const SGINT_ISTAT0_ENET: u8 = 0x08;
pub const SGINT_ISTAT0_GFXDMA: u8 = 0x10;
pub const SGINT_ISTAT0_PPORT: u8 = 0x20;
pub const SGINT_ISTAT0_HPC2: u8 = 0x40;
pub const SGINT_ISTAT0_LIO2: u8 = 0x80;
pub const SGINT_ISTAT1_ISDNI: u8 = 0x01;
pub const SGINT_ISTAT1_PWR: u8 = 0x02;
pub const SGINT_ISTAT1_ISDNH: u8 = 0x04;
pub const SGINT_ISTAT1_LIO3: u8 = 0x08;
pub const SGINT_ISTAT1_HPC3: u8 = 0x10;
pub const SGINT_ISTAT1_AFAIL: u8 = 0x20;
pub const SGINT_ISTAT1_VIDEO: u8 = 0x40;
pub const SGINT_ISTAT1_GIO2: u8 = 0x80;
pub const SGINT_TCWORD_BCD: u8 = 0x01;
pub const SGINT_TCWORD_MMASK: u8 = 0x0e;
pub const SGINT_TCWORD_MITC: u8 = 0x00;
pub const SGINT_TCWORD_MOS: u8 = 0x02;
pub const SGINT_TCWORD_MRGEN: u8 = 0x04;
pub const SGINT_TCWORD_MSWGEN: u8 = 0x06;
pub const SGINT_TCWORD_MSWST: u8 = 0x08;
pub const SGINT_TCWORD_MHWST: u8 = 0x0a;
pub const SGINT_TCWORD_CMASK: u8 = 0x30;
pub const SGINT_TCWORD_CLAT: u8 = 0x00;
pub const SGINT_TCWORD_CLSB: u8 = 0x10;
pub const SGINT_TCWORD_CMSB: u8 = 0x20;
pub const SGINT_TCWORD_CALL: u8 = 0x30;
pub const SGINT_TCWORD_CNT0: u8 = 0x00;
pub const SGINT_TCWORD_CNT1: u8 = 0x40;
pub const SGINT_TCWORD_CNT2: u8 = 0x80;
pub const SGINT_TCWORD_CRBCK: u8 = 0xc0;

pub const SGINT_TIMER_CLOCK: u32 = 1_000_000;
pub const SGINT_TCSAMP_COUNTER: u32 = (SGINT_TIMER_CLOCK / HZ) + 255;

#[repr(C)]
pub struct sgioc_regs {
    pub pport: pi1_regs,
    pub _unused0: [u32; 2],
    pub uart: sgioc_uart_regs,
    pub kbdmouse: sgioc_keyb_regs,
    pub _gcsel: [u8; 3], pub gcsel: u8,
    pub _genctrl: [u8; 3], pub genctrl: u8,
    pub _panel: [u8; 3], pub panel: u8,
    pub _unused1: u32,
    pub _sysid: [u8; 3], pub sysid: u8,
    pub _unused2: u32,
    pub _read: [u8; 3], pub read: u8,
    pub _unused3: u32,
    pub _dmasel: [u8; 3], pub dmasel: u8,
    pub _unused4: u32,
    pub _reset: [u8; 3], pub reset: u8,
    pub _unused5: u32,
    pub _write: [u8; 3], pub write: u8,
    pub _unused6: u32,
    pub int3: sgint_regs,
    pub _unused7: [u32; 16],
    pub extio: u32,
}

pub const fn SGIOC_SYSID_BOARDREV(x: u8) -> u8 { (x & 0x1e) >> 1 }
pub const fn SGIOC_SYSID_CHIPREV(x: u8) -> u8 { (x & 0xe0) >> 5 }

pub const SGIOC_PANEL_POWERON: u8 = 0x01;
pub const SGIOC_PANEL_POWERINTR: u8 = 0x02;
pub const SGIOC_PANEL_VOLDNINTR: u8 = 0x10;
pub const SGIOC_PANEL_VOLDNHOLD: u8 = 0x20;
pub const SGIOC_PANEL_VOLUPINTR: u8 = 0x40;
pub const SGIOC_PANEL_VOLUPHOLD: u8 = 0x80;
pub const SGIOC_SYSID_FULLHOUSE: u8 = 0x01;
pub const SGIOC_DMASEL_SCLK10MHZ: u8 = 0x00;
pub const SGIOC_DMASEL_ISDNB: u8 = 0x01;
pub const SGIOC_DMASEL_ISDNA: u8 = 0x02;
pub const SGIOC_DMASEL_PPORT: u8 = 0x04;
pub const SGIOC_DMASEL_SCLK667MHZ: u8 = 0x10;
pub const SGIOC_DMASEL_SCLKEXT: u8 = 0x20;
pub const SGIOC_RESET_PPORT: u8 = 0x01;
pub const SGIOC_RESET_KBDMOUSE: u8 = 0x02;
pub const SGIOC_RESET_EISA: u8 = 0x04;
pub const SGIOC_RESET_ISDN: u8 = 0x08;
pub const SGIOC_RESET_LC0OFF: u8 = 0x10;
pub const SGIOC_RESET_LC1OFF: u8 = 0x20;
pub const SGIOC_WRITE_NTHRESH: u8 = 0x01;
pub const SGIOC_WRITE_TPSPEED: u8 = 0x02;
pub const SGIOC_WRITE_EPSEL: u8 = 0x04;
pub const SGIOC_WRITE_EASEL: u8 = 0x08;
pub const SGIOC_WRITE_U1AMODE: u8 = 0x10;
pub const SGIOC_WRITE_U0AMODE: u8 = 0x20;
pub const SGIOC_WRITE_MLO: u8 = 0x40;
pub const SGIOC_WRITE_MHI: u8 = 0x80;

pub const EXTIO_S0_IRQ_3: u32 = 0x8000;
pub const EXTIO_S0_IRQ_2: u32 = 0x4000;
pub const EXTIO_S0_IRQ_1: u32 = 0x2000;
pub const EXTIO_S0_RETRACE: u32 = 0x1000;
pub const EXTIO_SG_IRQ_3: u32 = 0x0800;
pub const EXTIO_SG_IRQ_2: u32 = 0x0400;
pub const EXTIO_SG_IRQ_1: u32 = 0x0200;
pub const EXTIO_SG_RETRACE: u32 = 0x0100;
pub const EXTIO_GIO_33MHZ: u32 = 0x0080;
pub const EXTIO_EISA_BUSERR: u32 = 0x0040;
pub const EXTIO_MC_BUSERR: u32 = 0x0020;
pub const EXTIO_HPC3_BUSERR: u32 = 0x0010;
pub const EXTIO_S0_STAT_1: u32 = 0x0008;
pub const EXTIO_S0_STAT_0: u32 = 0x0004;
pub const EXTIO_SG_STAT_1: u32 = 0x0002;
pub const EXTIO_SG_STAT_0: u32 = 0x0001;

extern "C" {
    pub static mut sgi_ioc_reset: u8;
    pub static mut sgi_ioc_write: u8;
    pub static mut sgioc: *mut sgioc_regs;
    pub static mut sgint: *mut sgint_regs;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
