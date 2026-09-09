// SPDX-License-Identifier: GPL-2.0

// Linux and Apollo architecture dependencies are supplied by other translated units.

extern "C" {
    fn dn_init_IRQ();
    fn legacy_timer_tick(ticks: i32);
    fn timer_heartbeat();
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t,
                   flags: u32, name: *const u8, dev_id: *mut core::ffi::c_void) -> i32;
    fn panic(msg: *const u8) -> !;
    fn pr_info(fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
    fn strcpy(dst: *mut u8, src: *const u8) -> *mut u8;
    fn strcat(dst: *mut u8, src: *const u8) -> *mut u8;
}

type u_long = u32;
type irqreturn_t = i32;

#[repr(C)]
pub struct rtc_time {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
}

#[repr(C)]
pub struct bi_record {
    pub tag: u16,
    pub size: u16,
    pub data: [u8; 0],
}

extern "C" {
    static mut mach_sched_init: Option<unsafe extern "C" fn()>;
    static mut mach_init_IRQ: Option<unsafe extern "C" fn()>;
    static mut mach_hwclk: Option<unsafe extern "C" fn(i32, *mut rtc_time) -> i32>;
    static mut mach_reset: Option<unsafe extern "C" fn()>;
    static mut mach_heartbeat: Option<unsafe extern "C" fn(i32)>;
    static mut mach_get_model: Option<unsafe extern "C" fn(*mut u8)>;
    static mut cpuctrl: u16;
    static mut addr_xlat_map: [u8; 0x400];
    static mut apollo_timer: usize;
    static mut pica: usize;
    static mut rtc: *mut apollo_rtc;
    static mut sio01: apollo_sio;
}

#[repr(C)]
pub struct apollo_sio { pub rhrb_thrb: u8, pub srb_csrb: u8 }
#[repr(C)]
pub struct apollo_rtc {
    pub second: i32, pub minute: i32, pub hours: i32, pub day_of_month: i32,
    pub day_of_week: i32, pub month: i32, pub year: i32,
}

const BI_APOLLO_MODEL: u16 = 0;
const APOLLO_UNKNOWN: u32 = 0;
const APOLLO_DN3000: u32 = 1;
const APOLLO_DN3010: u32 = 2;
const APOLLO_DN3500: u32 = 3;
const APOLLO_DN4000: u32 = 4;
const APOLLO_DN4500: u32 = 5;
const IRQ_APOLLO: i32 = 0;
const IRQ_HANDLED: irqreturn_t = 1;

const SAU8_SIO01_PHYSADDR: u_long = 0;
const SAU8_RTC_PHYSADDR: u_long = 0;
const SAU8_PICA: u_long = 0;
const SAU8_PICB: u_long = 0;
const SAU8_CPUCTRL: u_long = 0;
const SAU8_TIMER: u_long = 0;
const SAU7_SIO01_PHYSADDR: u_long = 0;
const SAU7_SIO23_PHYSADDR: u_long = 0;
const SAU7_RTC_PHYSADDR: u_long = 0;
const SAU7_PICA: u_long = 0;
const SAU7_PICB: u_long = 0;
const SAU7_CPUCTRL: u_long = 0;
const SAU7_TIMER: u_long = 0;

#[no_mangle] pub static mut sio01_physaddr: u_long = 0;
#[no_mangle] pub static mut sio23_physaddr: u_long = 0;
#[no_mangle] pub static mut rtc_physaddr: u_long = 0;
#[no_mangle] pub static mut pica_physaddr: u_long = 0;
#[no_mangle] pub static mut picb_physaddr: u_long = 0;
#[no_mangle] pub static mut cpuctrl_physaddr: u_long = 0;
#[no_mangle] pub static mut timer_physaddr: u_long = 0;
#[no_mangle] pub static mut apollo_model: u_long = 0;

static apollo_models: [&[u8]; 5] = [b"DN3000 (Otter)\0", b"DN3010 (Otter)\0", b"DN3500 (Cougar II)\0", b"DN4000 (Mink)\0", b"DN4500 (Roadrunner)\0"];

#[no_mangle]
pub unsafe extern "C" fn apollo_parse_bootinfo(record: *const bi_record) -> i32 {
    let tag = u16::from_be((*record).tag);
    match tag {
        BI_APOLLO_MODEL => { apollo_model = u32::from_be(*( (*record).data.as_ptr() as *const u32)); 0 }
        _ => 1,
    }
}

unsafe fn dn_setup_model() {
    pr_info(b"Apollo hardware found: [%s]\n\0".as_ptr(), apollo_models[(apollo_model - APOLLO_DN3000) as usize].as_ptr());
    match apollo_model {
        APOLLO_UNKNOWN => panic(b"Unknown apollo model\0".as_ptr()),
        APOLLO_DN3000 | APOLLO_DN3010 => { sio01_physaddr=SAU8_SIO01_PHYSADDR; rtc_physaddr=SAU8_RTC_PHYSADDR; pica_physaddr=SAU8_PICA; picb_physaddr=SAU8_PICB; cpuctrl_physaddr=SAU8_CPUCTRL; timer_physaddr=SAU8_TIMER; }
        APOLLO_DN4000 | APOLLO_DN3500 => { sio01_physaddr=SAU7_SIO01_PHYSADDR; sio23_physaddr=SAU7_SIO23_PHYSADDR; rtc_physaddr=SAU7_RTC_PHYSADDR; pica_physaddr=SAU7_PICA; picb_physaddr=SAU7_PICB; cpuctrl_physaddr=SAU7_CPUCTRL; timer_physaddr=SAU7_TIMER; }
        APOLLO_DN4500 => panic(b"Apollo model not yet supported\0".as_ptr()),
        _ => panic(b"Undefined apollo model\0".as_ptr()),
    }
}

unsafe fn dn_serial_print(mut str_: *const u8) { while *str_ != 0 { if *str_ == b'\n' { sio01.rhrb_thrb=b'\r'; while sio01.srb_csrb & 0x4 == 0 {} } sio01.rhrb_thrb=*str_; str_=str_.add(1); while sio01.srb_csrb & 0x4 == 0 {} } }

#[no_mangle] pub unsafe extern "C" fn config_apollo() { dn_setup_model(); mach_sched_init=Some(dn_sched_init); mach_init_IRQ=Some(dn_init_IRQ); mach_hwclk=Some(dn_dummy_hwclk); mach_reset=Some(dn_dummy_reset); mach_get_model=Some(dn_get_model); cpuctrl=0xaa00; for i in 0..0x400 { addr_xlat_map[i]=0; } }

#[no_mangle] pub unsafe extern "C" fn dn_timer_int(_: i32, _: *mut core::ffi::c_void) -> irqreturn_t { legacy_timer_tick(1); timer_heartbeat(); core::ptr::read_volatile((apollo_timer as *const u8).add(3)); core::ptr::read_volatile((apollo_timer as *const u8).add(5)); IRQ_HANDLED }

#[no_mangle] pub unsafe extern "C" fn dn_sched_init() { let p=apollo_timer as *mut u8; core::ptr::write_volatile(p.add(3),0x01); core::ptr::write_volatile(p.add(1),0x40); core::ptr::write_volatile(p.add(5),0x09); core::ptr::write_volatile(p.add(7),0xc4); let q=(pica as *mut u8).add(1); *q &= !8; if request_irq(IRQ_APOLLO,dn_timer_int,0,b"time\0".as_ptr(),core::ptr::null_mut()) != 0 { pr_err(b"Couldn't register timer interrupt\n\0".as_ptr()); } }

#[no_mangle] pub unsafe extern "C" fn dn_dummy_hwclk(op:i32,t:*mut rtc_time)->i32 { if op==0 { (*t).tm_sec=(*rtc).second; (*t).tm_min=(*rtc).minute; (*t).tm_hour=(*rtc).hours; (*t).tm_mday=(*rtc).day_of_month; (*t).tm_wday=(*rtc).day_of_week; (*t).tm_mon=(*rtc).month-1; (*t).tm_year=(*rtc).year; if (*t).tm_year<70 { (*t).tm_year+=100; } } else { (*rtc).second=(*t).tm_sec; (*rtc).minute=(*t).tm_min; (*rtc).hours=(*t).tm_hour; (*rtc).day_of_month=(*t).tm_mday; if (*t).tm_wday!=-1 { (*rtc).day_of_week=(*t).tm_wday; } (*rtc).month=(*t).tm_mon+1; (*rtc).year=(*t).tm_year%100; } 0 }

unsafe fn dn_dummy_reset() { dn_serial_print(b"The end !\n\0".as_ptr()); loop {} }
unsafe fn dn_get_model(model:*mut u8) { strcpy(model,b"Apollo \0".as_ptr()); if apollo_model>=APOLLO_DN3000 && apollo_model<=APOLLO_DN4500 { strcat(model,apollo_models[(apollo_model-APOLLO_DN3000) as usize].as_ptr()); } }

#[cfg(feature = "CONFIG_HEARTBEAT")]
static mut dn_cpuctrl: u16 = 0xff00;

#[cfg(feature = "CONFIG_HEARTBEAT")]
unsafe fn dn_heartbeat(on: i32) {
    if on != 0 { dn_cpuctrl &= !0x100; cpuctrl = dn_cpuctrl; }
    else { dn_cpuctrl &= !0x100; dn_cpuctrl |= 0x100; cpuctrl = dn_cpuctrl; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
