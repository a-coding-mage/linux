/* SPDX-License-Identifier: GPL-2.0 */
/* apollohw.h : some structures to access apollo HW */

// C dependency: <linux/types.h>
// C dependency: <asm/bootinfo-apollo.h>

extern "C" {
    pub static mut apollo_model: u32;

    pub static mut sio01_physaddr: u32;
    pub static mut sio23_physaddr: u32;
    pub static mut rtc_physaddr: u32;
    pub static mut pica_physaddr: u32;
    pub static mut picb_physaddr: u32;
    pub static mut cpuctrl_physaddr: u32;
    pub static mut timer_physaddr: u32;
}

#[repr(C)]
pub struct SCN2681 {
    pub mra: u8, pub dummymra: u8,
    pub sra_csra: u8, pub dummysra_csra: u8,
    pub BRGtest_cra: u8, pub dummyBRGtest_cra: u8,
    pub rhra_thra: u8, pub dummyrhra_thra: u8,
    pub ipcr_acr: u8, pub dummyipcr_acr: u8,
    pub isr_imr: u8, pub dummyisr_imr: u8,
    pub ctu_ctur: u8, pub dummyctu_ctur: u8,
    pub ctl_ctlr: u8, pub dummyctl_ctlr: u8,
    pub mrb: u8, pub dummymrb: u8,
    pub srb_csrb: u8, pub dummysrb_csrb: u8,
    pub tst_crb: u8, pub dummytst_crb: u8,
    pub rhrb_thrb: u8, pub dummyrhrb_thrb: u8,
    pub reserved: u8, pub dummyreserved: u8,
    pub ip_opcr: u8, pub dummyip_opcr: u8,
    pub startCnt_setOutBit: u8, pub dummystartCnt_setOutBit: u8,
    pub stopCnt_resetOutBit: u8, pub dummystopCnt_resetOutBit: u8,
}

#[repr(C)]
pub struct mc146818 {
    pub second: u8, pub alarm_second: u8,
    pub minute: u8, pub alarm_minute: u8,
    pub hours: u8, pub alarm_hours: u8,
    pub day_of_week: u8, pub day_of_month: u8,
    pub month: u8, pub year: u8,
}

pub const IO_BASE: u32 = 0x80000000;

pub const SAU7_SIO01_PHYSADDR: u32 = 0x10400;
pub const SAU7_SIO23_PHYSADDR: u32 = 0x10500;
pub const SAU7_RTC_PHYSADDR: u32 = 0x10900;
pub const SAU7_PICA: u32 = 0x11000;
pub const SAU7_PICB: u32 = 0x11100;
pub const SAU7_CPUCTRL: u32 = 0x10100;
pub const SAU7_TIMER: u32 = 0x010800;

pub const SAU8_SIO01_PHYSADDR: u32 = 0x8400;
pub const SAU8_RTC_PHYSADDR: u32 = 0x8900;
pub const SAU8_PICA: u32 = 0x9400;
pub const SAU8_PICB: u32 = 0x9500;
pub const SAU8_CPUCTRL: u32 = 0x8100;
pub const SAU8_TIMER: u32 = 0x8800;

pub const unsafe fn sio01() -> *mut SCN2681 {
    (IO_BASE.wrapping_add(sio01_physaddr)) as *mut SCN2681
}
pub const unsafe fn sio23() -> *mut SCN2681 {
    (IO_BASE.wrapping_add(sio23_physaddr)) as *mut SCN2681
}
pub const unsafe fn rtc() -> *mut mc146818 {
    (IO_BASE.wrapping_add(rtc_physaddr)) as *mut mc146818
}
pub const unsafe fn cpuctrl() -> *mut u32 {
    (IO_BASE.wrapping_add(cpuctrl_physaddr)) as *mut u32
}
pub const unsafe fn pica() -> u32 { IO_BASE.wrapping_add(pica_physaddr) }
pub const unsafe fn picb() -> u32 { IO_BASE.wrapping_add(picb_physaddr) }
pub const unsafe fn apollo_timer() -> u32 { IO_BASE.wrapping_add(timer_physaddr) }
pub const fn addr_xlat_map() -> *mut u16 { (IO_BASE + 0x17000) as *mut u16 }

pub const fn isaIO2mem(x: u32) -> u32 {
    ((((x & 0x3f8) << 7) | ((x & 0xfc00) >> 6) | (x & 0x7)) + 0x40000 + IO_BASE)
}

// C dependency: IRQ_USER
pub const IRQ_APOLLO: u32 = IRQ_USER;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
