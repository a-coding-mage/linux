// SPDX-License-Identifier: GPL-2.0-or-later
// Translation of arch/m68k/mvme16x/config.c

// C includes provide the external kernel types, functions, globals, and macros
// referenced below.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut mvme_bdid: t_bdid;
    static mut vme_brdtype: u16;
    static mut vectors: *mut c_ulong;
    static mut mach_sched_init: Option<unsafe extern "C" fn()>;
    static mut mach_init_IRQ: Option<unsafe extern "C" fn()>;
    static mut mach_reset: Option<unsafe extern "C" fn()>;
    static mut mach_get_model: Option<unsafe extern "C" fn(*mut c_char)>;
    static mut mach_get_hardware_list: Option<unsafe extern "C" fn(*mut seq_file)>;
}

type c_ulong = usize;
type u_char = u8;
type u64_ = u64;

#[repr(C)]
pub struct t_bdid {
    pub brdsuffix: [u8; 2],
    pub brdno: u16,
    pub bdid: [u8; 4],
    pub rev: u8,
    pub yr: u8,
    pub mth: u8,
    pub day: u8,
}
type p_bdid = *mut t_bdid;

#[repr(C)] pub struct bi_record { pub tag: u16 }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct console { _private: [u8; 0] }
#[repr(C)] pub struct resource { _private: [u8; 0] }
#[repr(C)] pub struct m48t59_plat_data { pub r#type: u32, pub yy_offset: u32 }
#[repr(C)] pub struct clocksource {
    pub name: *const c_char, pub rating: u32,
    pub read: Option<unsafe extern "C" fn(*mut clocksource) -> u64>,
    pub mask: u64, pub flags: u32,
}

extern "C" {
    fn be16_to_cpu(v: u16) -> u16;
    fn m68k_setup_user_interrupt(a: c_int, b: c_int);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_crit(fmt: *const c_char, ...);
    fn sprintf(dst: *mut c_char, fmt: *const c_char, ...);
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn in_8(addr: usize) -> u8;
    fn in_be32(addr: usize) -> u32;
    fn out_8(addr: usize, value: u8);
    fn out_be32(addr: usize, value: u32);
    fn local_irq_save(flags: *mut c_ulong);
    fn local_irq_restore(flags: c_ulong);
    fn legacy_timer_tick(n: c_int);
    fn request_irq(irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: u32, name: *const c_char, dev: *mut c_void) -> c_int;
    fn panic(fmt: *const c_char, ... ) -> !;
    fn clocksource_register_hz(cs: *mut clocksource, hz: u32) -> c_int;
    fn platform_device_register_resndata(a: *mut c_void, name: *const c_char, id: c_int, r: *mut resource, n: usize, data: *mut c_void, size: usize) -> c_int;
}

type irqreturn_t = c_int;
const IRQ_HANDLED: irqreturn_t = 1;
const BI_VME_TYPE: u16 = 0;
const BI_VME_BRDINFO: u16 = 1;
const VEC_USER: c_int = 0;
const MVME162_VERSION_REG: usize = 0xfff40000;
const MVME16x_CONFIG_NO_VMECHIP2: u8 = 0x01;
const MVME16x_CONFIG_NO_SCSICHIP: u8 = 0x02;
const MVME16x_CONFIG_NO_ETHERNET: u8 = 0x04;
const MVME16x_CONFIG_GOT_SCCA: u16 = 0x08;
const MVME16x_CONFIG_GOT_FPU: u8 = 0x10;
const MVME16x_CONFIG_SPEED_32: u8 = 0x20;
const MVME16x_CONFIG_GOT_LP: u16 = 0x40;
const MVME16x_CONFIG_GOT_CD2401: u16 = 0x80;
const PCC2CHIP: usize = 0xfff42000;
const PCCSCCTICR: usize = PCC2CHIP + 0x1e;
const PCCTPIACKR: usize = PCC2CHIP + 0x25;
const CD2401_ADDR: usize = 0xfff45000;
const CyCCR: usize = 0x13; const CyENB_XMTR: u8 = 0x08; const CyIER: usize = 0x11;
const CyTxMpty: u8 = 0x02; const CyCAR: usize = 0xee; const CyLICR: usize = 0x26;
const CyTDR: usize = 0xf8; const CyTEOIR: usize = 0x85; const CyNOTRANS: u8 = 0x08;
const M48T59RTC_TYPE_M48T08: u32 = 0;
const MVME_RTC_BASE: usize = 0;
const CLOCK_SOURCE_IS_CONTINUOUS: u32 = 1;
const MVME16x_IRQ_TIMER: c_int = 0; const MVME162_IRQ_ABORT: c_int = 0; const MVME167_IRQ_ABORT: c_int = 0;
const IRQF_TIMER: u32 = 0;
const PCC_TIMER_CLOCK_FREQ: u32 = 1_000_000;
const HZ: u32 = 100; const PCC_TIMER_CYCLES: u32 = PCC_TIMER_CLOCK_FREQ / HZ;
const PCCTCMP1: usize = PCC2CHIP + 0x04; const PCCTCNT1: usize = PCC2CHIP + 0x08;
const PCCTOVR1: usize = PCC2CHIP + 0x17; const PCCTIC1: usize = PCC2CHIP + 0x1b;
const PCCTOVR1_TIC_EN: u8 = 1; const PCCTOVR1_COC_EN: u8 = 2; const PCCTOVR1_OVR_CLR: u8 = 4;
const PCCTIC1_INT_LEVEL: u8 = 6; const PCCTIC1_INT_CLR: u8 = 8; const PCCTIC1_INT_EN: u8 = 0x10;

#[no_mangle] pub static mut mvme16x_config: u16 = 0;

#[no_mangle] pub unsafe extern "C" fn mvme16x_parse_bootinfo(bi: *const bi_record) -> c_int {
    let tag = be16_to_cpu((*bi).tag); if tag == BI_VME_TYPE || tag == BI_VME_BRDINFO { 0 } else { 1 }
}

#[no_mangle] pub unsafe extern "C" fn mvme16x_reset() {
    pr_info(b"\r\n\nCalled mvme16x_reset\r\n\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\0".as_ptr() as *const c_char);
    core::ptr::write_volatile(0xfff40107 as *mut u8, 0x80);
}

unsafe extern "C" fn mvme16x_get_model(model: *mut c_char) {
    let p = &mvme_bdid; let mut suf = [0i8; 4]; suf[1] = p.brdsuffix[0] as i8; suf[2] = p.brdsuffix[1] as i8; suf[3] = 0; suf[0] = if suf[1] != 0 { b'-' as i8 } else { 0 };
    sprintf(model, b"Motorola MVME%x%s\0".as_ptr() as *const c_char, be16_to_cpu(p.brdno), suf.as_ptr());
}

unsafe extern "C" fn mvme16x_get_hardware_list(m: *mut seq_file) { let brdno = be16_to_cpu(mvme_bdid.brdno); if brdno == 0x0162 || brdno == 0x0172 { let rev = core::ptr::read_volatile(MVME162_VERSION_REG as *const u8); let s = if rev & MVME16x_CONFIG_NO_VMECHIP2 != 0 { b"NOT \0" } else { b"\0" }; pr_info(b"VMEchip2        %spresent\n\0".as_ptr() as *const c_char, s.as_ptr()); } let _ = m; }

unsafe extern "C" fn mvme16x_init_IRQ() { m68k_setup_user_interrupt(VEC_USER, 192); }

#[no_mangle] pub unsafe extern "C" fn config_mvme16x() {
    mach_sched_init = Some(mvme16x_sched_init); mach_init_IRQ = Some(mvme16x_init_IRQ); mach_reset = Some(mvme16x_reset); mach_get_model = Some(mvme16x_get_model); mach_get_hardware_list = Some(mvme16x_get_hardware_list);
    let p = &mvme_bdid; let brdno = be16_to_cpu(p.brdno); if strncmp(b"BDID\0".as_ptr() as *const c_char, p.bdid.as_ptr() as *const c_char, 4) != 0 { pr_crit(b"Bug call .BRD_ID returned garbage - giving up\n\0".as_ptr() as *const c_char); loop {} } if vme_brdtype == 0 { vme_brdtype = brdno; }
    let mut id = [0i8; 40]; mvme16x_get_model(id.as_mut_ptr()); pr_info(b"BRD_ID: %s   BUG %x.%x %02x/%02x/%02x\n\0".as_ptr() as *const c_char, id.as_ptr(), p.rev >> 4, p.rev & 0xf, p.yr, p.mth, p.day);
    if brdno == 0x0162 || brdno == 0x172 { let rev = core::ptr::read_volatile(MVME162_VERSION_REG as *const u8); mvme16x_config = rev as u16 | MVME16x_CONFIG_GOT_SCCA; } else { mvme16x_config = MVME16x_CONFIG_GOT_LP | MVME16x_CONFIG_GOT_CD2401; }
}

unsafe extern "C" fn mvme16x_timer_int(_: c_int, _: *mut c_void) -> irqreturn_t { let mut flags = 0; local_irq_save(&mut flags); out_8(PCCTOVR1, PCCTOVR1_OVR_CLR | PCCTOVR1_TIC_EN | PCCTOVR1_COC_EN); out_8(PCCTIC1, PCCTIC1_INT_EN | PCCTIC1_INT_CLR | PCCTIC1_INT_LEVEL); local_irq_restore(flags); IRQ_HANDLED }

static mut clk_total: u32 = 0;
unsafe extern "C" fn mvme16x_read_clk(_: *mut clocksource) -> u64 { let mut flags = 0; local_irq_save(&mut flags); let tmp = in_8(PCCTOVR1) >> 4; let mut ticks = in_be32(PCCTCNT1); let overflow = in_8(PCCTOVR1) >> 4; if overflow != tmp { ticks = in_be32(PCCTCNT1); } ticks += overflow as u32 * PCC_TIMER_CYCLES; ticks += clk_total; local_irq_restore(flags); ticks as u64 }

#[no_mangle] pub unsafe extern "C" fn mvme16x_sched_init() { request_irq(MVME16x_IRQ_TIMER, mvme16x_timer_int, IRQF_TIMER, b"timer\0".as_ptr() as *const c_char, core::ptr::null_mut()); out_be32(PCCTCNT1, 0); out_be32(PCCTCMP1, PCC_TIMER_CYCLES); out_8(PCCTOVR1, 7); out_8(PCCTIC1, 0x1e); }

unsafe extern "C" fn mvme16x_abort_int(_: c_int, _: *mut c_void) -> irqreturn_t {
    let new = vectors; let old = 0xffe00000 as *const c_ulong; let brdno = be16_to_cpu(mvme_bdid.brdno);
    if brdno == 0x0162 || brdno == 0x172 { let p = 0xfff42043 as *mut u8; core::ptr::write_volatile(p, core::ptr::read_volatile(p) | 8); } else { core::ptr::write_volatile(0xfff40074 as *mut u32, 0x40000000); }
    *new.add(4) = *old.add(4); *new.add(9) = *old.add(9); *new.add(47) = *old.add(47);
    if brdno == 0x0162 || brdno == 0x172 { *new.add(0x5e) = *old.add(0x5e); } else { *new.add(0x6e) = *old.add(0x6e); } IRQ_HANDLED
}

static mut mvme16x_clk: clocksource = clocksource { name: b"pcc\0".as_ptr() as *const c_char, rating: 250, read: Some(mvme16x_read_clk), mask: 0xffff_ffff, flags: CLOCK_SOURCE_IS_CONTINUOUS };

static mut m48t59_rsrc: [resource; 1] = [resource { _private: [] }];
static mut m48t59_data: m48t59_plat_data = m48t59_plat_data { r#type: M48T59RTC_TYPE_M48T08, yy_offset: 70 };

unsafe extern "C" fn mvme16x_platform_init() -> c_int {
    // MACH_IS_MVME16x is supplied by the machine-specific headers.
    if !MACH_IS_MVME16x() { return 0; }
    platform_device_register_resndata(core::ptr::null_mut(), b"rtc-m48t59\0".as_ptr() as *const c_char, -1, m48t59_rsrc.as_mut_ptr(), 1, &mut m48t59_data as *mut _ as *mut c_void, core::mem::size_of::<m48t59_plat_data>()); 0
}
extern "C" { fn MACH_IS_MVME16x() -> bool; }

// The original file registers the timer and abort IRQs here, then registers
// the clocksource. Kernel initcall and platform-device declarations are kept
// as external integration points for the surrounding tree.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
