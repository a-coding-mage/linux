/*
 * linux/arch/m68k/atari/debug.c
 *
 * Atari debugging and serial console stuff
 */

// External kernel and Atari hardware symbols supplied by other translation units.
use core::ffi::c_char;

#[repr(C)]
pub struct Console {
    pub name: *const c_char,
    pub flags: i32,
    pub index: i32,
    pub write: Option<unsafe extern "C" fn(*mut Console, *const c_char, u32)>,
}

#[repr(C)]
pub struct Mfp { pub trn_stat: u8, pub usart_dta: u8, pub tim_ct_cd: u8, pub tim_dt_d: u8, pub rcv_stat: u8, pub par_dt_reg: u8 }
#[repr(C)]
pub struct Scc { pub cha_b_ctrl: u8, pub cha_b_data: u8 }
#[repr(C)]
pub struct Acia { pub mid_ctrl: u8, pub mid_data: u8 }
#[repr(C)]
pub struct Ym { pub rd_data_reg_sel: u8, pub wd_data: u8 }

extern "C" {
    pub static mut st_mfp: Mfp;
    pub static mut atari_scc: Scc;
    pub static mut acia: Acia;
    pub static mut sound_ym: Ym;
    pub static mut atari_switches: i32;
    pub static mut loops_per_jiffy: u64;
    pub fn barrier();
    pub fn mfpdelay();
    pub fn atari_turnoff_irq(irq: i32);
    pub fn register_console(co: *mut Console);
    pub static MACH_IS_ATARI: bool;
    pub static MACH_IS_FALCON: bool;
    pub static ATARIHW_PRESENT_TT_MFP: bool;
}

pub const CON_PRINTBUFFER: i32 = 1;
pub const CBAUD: i32 = 0x100f;
pub const CBAUDEX: i32 = 0x1000;
pub const B1200: i32 = 9;
pub const B4800: i32 = 12;
pub const B9600: i32 = 13;
pub const B38400: i32 = 15;
pub const CSIZE: i32 = 0x30;
pub const CS7: i32 = 0x20;
pub const CS8: i32 = 0x30;
pub const PARENB: i32 = 0x100;
pub const PARODD: i32 = 0x200;
pub const ACIA_TDRE: u8 = 0x02;
pub const ACIA_DIV64: u8 = 0x00;
pub const ACIA_DIV1: u8 = 0x01;
pub const ACIA_DIV16: u8 = 0x02;
pub const ACIA_RHTID: u8 = 0x20;
pub const ACIA_RLTID: u8 = 0x00;
pub const ATARI_SWITCH_MIDI: i32 = 1;
pub const IRQ_MFP_BUSY: i32 = 0;

#[no_mangle]
pub static mut atari_SCC_reset_done: i32 = 0;

static mut atari_console_driver: Console = Console {
    name: b"debug\0".as_ptr() as *const c_char,
    flags: CON_PRINTBUFFER,
    index: -1,
    write: None,
};

#[inline]
unsafe fn ata_mfp_out(c: u8) {
    while st_mfp.trn_stat & 0x80 == 0 { barrier(); }
    st_mfp.usart_dta = c;
}

unsafe extern "C" fn atari_mfp_console_write(_co: *mut Console, mut str_: *const c_char, mut count: u32) {
    while count != 0 { let c = *str_ as u8; if c == b'\n' { ata_mfp_out(b'\r'); } ata_mfp_out(c); str_ = str_.add(1); count -= 1; }
}

#[inline]
unsafe fn ata_scc_out(c: u8) {
    loop { mfpdelay(); if atari_scc.cha_b_ctrl & 0x04 != 0 { break; } }
    mfpdelay(); atari_scc.cha_b_data = c;
}

unsafe extern "C" fn atari_scc_console_write(_co: *mut Console, mut str_: *const c_char, mut count: u32) {
    while count != 0 { let c = *str_ as u8; if c == b'\n' { ata_scc_out(b'\r'); } ata_scc_out(c); str_ = str_.add(1); count -= 1; }
}

#[inline]
unsafe fn ata_midi_out(c: u8) { while acia.mid_ctrl & ACIA_TDRE == 0 { barrier(); } acia.mid_data = c; }

unsafe extern "C" fn atari_midi_console_write(_co: *mut Console, mut str_: *const c_char, mut count: u32) {
    while count != 0 { let c = *str_ as u8; if c == b'\n' { ata_midi_out(b'\r'); } ata_midi_out(c); str_ = str_.add(1); count -= 1; }
}

unsafe fn ata_par_out(c: u8) -> i32 {
    let mut tmp: u8;
    let mut i = if loops_per_jiffy > 1 { loops_per_jiffy } else { 10_000_000u64 / 100 }; // HZ is supplied by the kernel build.
    while st_mfp.par_dt_reg & 1 != 0 && { i -= 1; i != 0 } {}
    if i == 0 { return 0; }
    sound_ym.rd_data_reg_sel = 15; sound_ym.wd_data = c; sound_ym.rd_data_reg_sel = 14;
    tmp = sound_ym.rd_data_reg_sel; sound_ym.wd_data = tmp & !0x20; mfpdelay(); sound_ym.wd_data = tmp | 0x20; 1
}

unsafe extern "C" fn atari_par_console_write(_co: *mut Console, mut str_: *const c_char, mut count: u32) {
    static mut printer_present: i32 = 1;
    if printer_present == 0 { return; }
    while count != 0 { if *str_ as u8 == b'\n' && ata_par_out(b'\r') == 0 { printer_present = 0; return; } if ata_par_out(*str_ as u8) == 0 { printer_present = 0; return; } str_ = str_.add(1); count -= 1; }
}

unsafe fn atari_init_mfp_port(cflag: i32) {
    static baud_table: [u8; 9] = [16,11,8,4,2,1,175,143,128];
    let mut baud = cflag & CBAUD; let parity = if cflag & PARENB != 0 { if cflag & PARODD != 0 { 4 } else { 6 } } else { 0 }; let csize = if cflag & CSIZE == CS7 { 0x20 } else { 0 };
    if cflag & CBAUDEX != 0 { baud += B38400; } if baud < B1200 || baud > B38400 + 2 { baud = B9600; } baud -= B1200;
    st_mfp.trn_stat &= !1; st_mfp.usart_ctr = (parity | csize | 0x88) as u8; st_mfp.tim_ct_cd &= !0x70; st_mfp.tim_dt_d = baud_table[baud as usize]; st_mfp.tim_ct_cd |= 1; st_mfp.trn_stat |= 1;
}

unsafe fn scc_write(reg: u8, val: u8) { atari_scc.cha_b_ctrl = reg; mfpdelay(); atari_scc.cha_b_ctrl = val; mfpdelay(); }
unsafe fn long_delay() { for _ in 0..100 { mfpdelay(); } }

unsafe fn atari_init_scc_port(cflag: i32) {
    let clksrc_table = [0x50,0x50,0x50,0x50,0x50,0x50,0x50,0,0]; let brgsrc_table = [2,2,2,2,2,2,0,2,2]; let clkmode_table = [0x40,0x40,0x40,0x40,0x40,0x40,0x40,0xc0,0x80]; let div_table = [208,138,103,50,24,11,1,0,0];
    let mut baud = cflag & CBAUD; if cflag & CBAUDEX != 0 { baud += B38400; } if baud < B1200 || baud > B38400+2 { baud = B9600; } baud -= B1200; let mut clksrc=clksrc_table[baud as usize]; let mut clkmode=clkmode_table[baud as usize]; let mut div=div_table[baud as usize];
    if ATARIHW_PRESENT_TT_MFP && baud >= 6 { clksrc=0x28; clkmode=if baud==6 {0xc0} else if baud==7 {0x80} else {0x40}; div=0; }
    let reg3=if cflag & CSIZE == CS8 {0xc0} else {0x40}; let reg5=if cflag & CSIZE == CS8 {0x60} else {0x20|0x82}; let _ = atari_scc.cha_b_ctrl;
    scc_write(9,0xc0); long_delay(); scc_write(4, if cflag&PARENB!=0 {if cflag&PARODD!=0 {1} else {3}} else {0}|4|clkmode); scc_write(3,reg3); scc_write(5,reg5); scc_write(9,0); long_delay(); scc_write(10,0); scc_write(11,clksrc); scc_write(12,div); scc_write(13,0); scc_write(14,brgsrc_table[baud as usize]); scc_write(14,brgsrc_table[baud as usize]|if div!=0 {1} else {0}); scc_write(3,reg3|1); scc_write(5,reg5|8); atari_SCC_reset_done=1;
}

unsafe fn atari_init_midi_port(cflag: i32) { let baud=cflag&CBAUD; let csize=if cflag&CSIZE==CS8 {0x10} else {0}; let parity=if cflag&PARENB!=0 {if cflag&PARODD!=0 {0x0c} else {0x08}} else {0x04}; let div=if baud==B4800 {ACIA_DIV64} else if baud==B38400+2 {ACIA_DIV1} else {ACIA_DIV16}; acia.mid_ctrl=div|csize|parity|if atari_switches&ATARI_SWITCH_MIDI!=0 {ACIA_RHTID} else {ACIA_RLTID}; }

#[no_mangle]
pub unsafe extern "C" fn atari_debug_setup(arg: *const c_char) -> i32 {
    if !MACH_IS_ATARI { return 0; }
    let mut name = arg; let ser = b"ser\0"; if libc_strcmp(name,ser.as_ptr() as *const c_char)==0 { name=if MACH_IS_FALCON {b"ser2\0".as_ptr()} else {b"ser1\0".as_ptr()} as *const c_char; }
    let registered=atari_console_driver.write.is_some();
    if libc_strcmp(name,b"ser1\0".as_ptr() as *const c_char)==0 { atari_init_mfp_port(B9600|CS8); atari_console_driver.write=Some(atari_mfp_console_write); }
    else if libc_strcmp(name,b"ser2\0".as_ptr() as *const c_char)==0 { atari_init_scc_port(B9600|CS8); atari_console_driver.write=Some(atari_scc_console_write); }
    else if libc_strcmp(name,b"midi\0".as_ptr() as *const c_char)==0 { atari_init_midi_port(B9600|CS8); atari_console_driver.write=Some(atari_midi_console_write); }
    else if libc_strcmp(name,b"par\0".as_ptr() as *const c_char)==0 { atari_turnoff_irq(IRQ_MFP_BUSY); sound_ym.rd_data_reg_sel=7; sound_ym.wd_data=0xff; sound_ym.rd_data_reg_sel=15; sound_ym.wd_data=0; sound_ym.rd_data_reg_sel=14; sound_ym.wd_data=sound_ym.rd_data_reg_sel|0x20; atari_console_driver.write=Some(atari_par_console_write); }
    if atari_console_driver.write.is_some() && !registered { register_console(&mut atari_console_driver); } 0
}

extern "C" { fn libc_strcmp(a: *const c_char, b: *const c_char) -> i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
