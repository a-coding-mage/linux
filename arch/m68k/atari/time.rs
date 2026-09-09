/*
 * linux/arch/m68k/atari/time.c
 *
 * Atari time and real time clock stuff
 */

/* Declarations supplied by the Linux/m68k environment are intentionally
 * referenced here rather than reimplemented. */

#[no_mangle]
pub static mut rtc_lock: spinlock_t = spinlock_t {};

unsafe extern "C" {
    static mut st_mfp: Mfp;
    static mut mste_rtc: MSTE_RTC;
    static mut tt_rtc: AtariRtc;
    static mut last_timer_count: u8;
    static mut clk_total: u32;
    static atari_clk: clocksource;
    fn local_irq_save(flags: *mut c_ulong);
    fn local_irq_restore(flags: c_ulong);
    fn legacy_timer_tick(n: c_int);
    fn timer_heartbeat();
    fn request_irq(irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
                   flags: c_ulong, name: *const c_char, dev: *mut c_void) -> c_int;
    fn pr_err(s: *const c_char);
    fn clocksource_register_hz(cs: *const clocksource, hz: c_uint) -> c_int;
    fn atari_writeb(reg: c_int, p: *mut u8);
    fn in_atomic() -> bool;
    fn irqs_disabled() -> bool;
    fn mdelay(ms: c_uint);
    fn schedule_timeout_interruptible(ticks: c_long) -> c_long;
    fn bin2bcd(v: c_int) -> c_int;
    fn bcd2bin(v: c_int) -> c_int;
}

static mut atari_clk: clocksource = clocksource {
    name: b"mfp\0".as_ptr() as *const c_char,
    rating: 100,
    read: Some(atari_read_clk),
    mask: 0xffff_ffff,
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
};

unsafe extern "C" fn mfp_timer_c_handler(_irq: c_int, _dev_id: *mut c_void) -> irqreturn_t {
    let mut flags: c_ulong = 0;
    local_irq_save(&mut flags);
    loop {
        last_timer_count = st_mfp.tim_dt_c;
        if last_timer_count != 1 { break; }
    }
    clk_total = clk_total.wrapping_add(INT_TICKS as u32);
    legacy_timer_tick(1);
    timer_heartbeat();
    local_irq_restore(flags);
    IRQ_HANDLED
}

pub unsafe fn atari_sched_init() {
    st_mfp.tim_dt_c = INT_TICKS as u8;
    st_mfp.tim_ct_cd = (st_mfp.tim_ct_cd & 15) | 0x60;
    if request_irq(IRQ_MFP_TIMC, mfp_timer_c_handler, IRQF_TIMER,
                   b"timer\0".as_ptr() as *const c_char, core::ptr::null_mut()) != 0 {
        pr_err(b"Couldn't register timer interrupt\n\0".as_ptr() as *const c_char);
    }
    clocksource_register_hz(&atari_clk, INT_CLK);
}

unsafe extern "C" fn atari_read_clk(_cs: *const clocksource) -> u64 {
    let mut flags: c_ulong = 0;
    let mut count: u8;
    local_irq_save(&mut flags);
    count = core::cmp::min(st_mfp.tim_dt_c, last_timer_count);
    last_timer_count = count;
    let mut ticks = (INT_TICKS as u32).wrapping_sub(count as u32);
    ticks = ticks.wrapping_add(clk_total);
    local_irq_restore(flags);
    ticks as u64
}

unsafe fn mste_read(val: *mut MSTE_RTC) {
    loop {
        (*val).sec_ones = mste_rtc.sec_ones & 0xf;
        (*val).sec_tens = mste_rtc.sec_tens & 0xf;
        (*val).min_ones = mste_rtc.min_ones & 0xf;
        (*val).min_tens = mste_rtc.min_tens & 0xf;
        (*val).hr_ones = mste_rtc.hr_ones & 0xf;
        (*val).hr_tens = mste_rtc.hr_tens & 0xf;
        (*val).weekday = mste_rtc.weekday & 0xf;
        (*val).day_ones = mste_rtc.day_ones & 0xf;
        (*val).day_tens = mste_rtc.day_tens & 0xf;
        (*val).mon_ones = mste_rtc.mon_ones & 0xf;
        (*val).mon_tens = mste_rtc.mon_tens & 0xf;
        (*val).year_ones = mste_rtc.year_ones & 0xf;
        (*val).year_tens = mste_rtc.year_tens & 0xf;
        if (*val).sec_ones == (mste_rtc.sec_ones & 0xf) { break; }
    }
}

unsafe fn mste_write(val: *const MSTE_RTC) {
    loop {
        mste_rtc.sec_ones = (*val).sec_ones; mste_rtc.sec_tens = (*val).sec_tens;
        mste_rtc.min_ones = (*val).min_ones; mste_rtc.min_tens = (*val).min_tens;
        mste_rtc.hr_ones = (*val).hr_ones; mste_rtc.hr_tens = (*val).hr_tens;
        mste_rtc.weekday = (*val).weekday; mste_rtc.day_ones = (*val).day_ones;
        mste_rtc.day_tens = (*val).day_tens; mste_rtc.mon_ones = (*val).mon_ones;
        mste_rtc.mon_tens = (*val).mon_tens; mste_rtc.year_ones = (*val).year_ones;
        mste_rtc.year_tens = (*val).year_tens;
        if (*val).sec_ones == (mste_rtc.sec_ones & 0xf) { break; }
    }
}

pub unsafe fn atari_mste_hwclk(op: c_int, t: *mut rtc_time) -> c_int {
    let mut hour: c_int; let mut year: c_int; let mut hr24 = 0; let mut val: MSTE_RTC = core::mem::zeroed();
    mste_rtc.mode |= 1; hr24 = (mste_rtc.mon_tens & 1) as c_int; mste_rtc.mode &= !1;
    if op != 0 {
        val.sec_ones = (*t).tm_sec % 10; val.sec_tens = (*t).tm_sec / 10;
        val.min_ones = (*t).tm_min % 10; val.min_tens = (*t).tm_min / 10; hour = (*t).tm_hour;
        if hr24 == 0 { if hour > 11 { hour += 8; } if hour == 0 || hour == 20 { hour += 12; } }
        val.hr_ones = hour % 10; val.hr_tens = hour / 10; val.day_ones = (*t).tm_mday % 10; val.day_tens = (*t).tm_mday / 10;
        val.mon_ones = ((*t).tm_mon + 1) % 10; val.mon_tens = ((*t).tm_mon + 1) / 10; year = (*t).tm_year - 80;
        val.year_ones = year % 10; val.year_tens = year / 10; val.weekday = (*t).tm_wday; mste_write(&val);
        mste_rtc.mode |= 1; val.year_ones = year % 4; mste_rtc.mode &= !1;
    } else {
        mste_read(&mut val); (*t).tm_sec = val.sec_ones + val.sec_tens * 10; (*t).tm_min = val.min_ones + val.min_tens * 10;
        hour = val.hr_ones + val.hr_tens * 10; if hr24 == 0 { if hour == 12 || hour == 32 { hour -= 12; } if hour >= 20 { hour -= 8; } }
        (*t).tm_hour = hour; (*t).tm_mday = val.day_ones + val.day_tens * 10; (*t).tm_mon = val.mon_ones + val.mon_tens * 10 - 1;
        (*t).tm_year = val.year_ones + val.year_tens * 10 + 80; (*t).tm_wday = val.weekday;
    } 0
}

pub unsafe fn atari_tt_hwclk(op: c_int, t: *mut rtc_time) -> c_int {
    let mut ctrl = rtc_read(RTC_CONTROL); let mut sec=0; let mut min=0; let mut hour=0; let mut day=0; let mut mon=0; let mut year=0; let mut wday=0; let mut pm=0;
    if op != 0 { sec=(*t).tm_sec; min=(*t).tm_min; hour=(*t).tm_hour; day=(*t).tm_mday; mon=(*t).tm_mon+1; year=(*t).tm_year-atari_rtc_year_offset; wday=(*t).tm_wday + ((*t).tm_wday>=0) as c_int;
        if ctrl & RTC_24H == 0 { if hour > 11 { pm=0x80; if hour != 12 { hour-=12; } } else if hour==0 { hour=12; } }
        if ctrl & RTC_DM_BINARY == 0 { sec=bin2bcd(sec); min=bin2bcd(min); hour=bin2bcd(hour); day=bin2bcd(day); mon=bin2bcd(mon); year=bin2bcd(year); if wday>=0 {wday=bin2bcd(wday);} }
    }
    while rtc_read(RTC_FREQ_SELECT) & RTC_UIP != 0 { if in_atomic() || irqs_disabled() { mdelay(1); } else { schedule_timeout_interruptible(HWCLK_POLL_INTERVAL); } }
    let mut flags=0; local_irq_save(&mut flags); rtc_write(RTC_CONTROL, ctrl|RTC_SET);
    if op==0 { sec=rtc_read(RTC_SECONDS); min=rtc_read(RTC_MINUTES); hour=rtc_read(RTC_HOURS); day=rtc_read(RTC_DAY_OF_MONTH); mon=rtc_read(RTC_MONTH); year=rtc_read(RTC_YEAR); wday=rtc_read(RTC_DAY_OF_WEEK); }
    else { rtc_write(RTC_SECONDS,sec); rtc_write(RTC_MINUTES,min); rtc_write(RTC_HOURS,hour+pm); rtc_write(RTC_DAY_OF_MONTH,day); rtc_write(RTC_MONTH,mon); rtc_write(RTC_YEAR,year); if wday>=0 {rtc_write(RTC_DAY_OF_WEEK,wday);} }
    rtc_write(RTC_CONTROL,ctrl & !RTC_SET); local_irq_restore(flags);
    if op==0 { if hour&0x80!=0 {hour &= !0x80; pm=1;} if ctrl&RTC_DM_BINARY==0 {sec=bcd2bin(sec);min=bcd2bin(min);hour=bcd2bin(hour);day=bcd2bin(day);mon=bcd2bin(mon);year=bcd2bin(year);wday=bcd2bin(wday);} if ctrl&RTC_24H==0 {if pm==0&&hour==12 {hour=0;} else if pm!=0&&hour!=12 {hour+=12;}} (*t).tm_sec=sec;(*t).tm_min=min;(*t).tm_hour=hour;(*t).tm_mday=day;(*t).tm_mon=mon-1;(*t).tm_year=year+atari_rtc_year_offset;(*t).tm_wday=wday-1; } 0
}

unsafe fn rtc_read(reg: c_int) -> c_int { atari_writeb(reg, &mut tt_rtc.regsel); tt_rtc.data as c_int }
unsafe fn rtc_write(reg: c_int, val: c_int) { atari_writeb(reg, &mut tt_rtc.regsel); tt_rtc.data=val as u8; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
