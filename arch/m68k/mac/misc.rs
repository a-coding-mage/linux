// SPDX-License-Identifier: GPL-2.0
/* Miscellaneous Mac68K-specific stuff */

// C includes and configuration symbols are supplied by the surrounding kernel translation.

const RTC_OFFSET: i64 = 2082844800;

static mut ROM_RESET: Option<unsafe extern "C" fn()> = None;

#[cfg(all(feature = "CONFIG_NVRAM", feature = "CONFIG_ADB_CUDA"))]
unsafe fn cuda_pram_read_byte(offset: i32) -> u8 {
    let mut req: adb_request = core::mem::zeroed();
    if cuda_request(&mut req, core::ptr::null_mut(), 4, CUDA_PACKET, CUDA_GET_PRAM,
                    (offset >> 8) & 0xff, offset & 0xff) < 0 { return 0; }
    while !req.complete { cuda_poll(); }
    req.reply[3]
}

#[cfg(all(feature = "CONFIG_NVRAM", feature = "CONFIG_ADB_CUDA"))]
unsafe fn cuda_pram_write_byte(data: u8, offset: i32) {
    let mut req: adb_request = core::mem::zeroed();
    if cuda_request(&mut req, core::ptr::null_mut(), 5, CUDA_PACKET, CUDA_SET_PRAM,
                    (offset >> 8) & 0xff, offset & 0xff, data as i32) < 0 { return; }
    while !req.complete { cuda_poll(); }
}

#[cfg(all(feature = "CONFIG_NVRAM", feature = "CONFIG_ADB_PMU"))]
unsafe fn pmu_pram_read_byte(offset: i32) -> u8 {
    let mut req: adb_request = core::mem::zeroed();
    if pmu_request(&mut req, core::ptr::null_mut(), 3, PMU_READ_XPRAM, offset & 0xff, 1) < 0 { return 0; }
    pmu_wait_complete(&mut req);
    req.reply[0]
}

#[cfg(all(feature = "CONFIG_NVRAM", feature = "CONFIG_ADB_PMU"))]
unsafe fn pmu_pram_write_byte(data: u8, offset: i32) {
    let mut req: adb_request = core::mem::zeroed();
    if pmu_request(&mut req, core::ptr::null_mut(), 4, PMU_WRITE_XPRAM, offset & 0xff, 1, data as i32) < 0 { return; }
    pmu_wait_complete(&mut req);
}

const RTC_FLG_READ: i32 = 1 << 7;
const RTC_FLG_WRITE_PROTECT: i32 = 1 << 7;
const RTC_REG_SECONDS_0: i32 = 0;
const RTC_REG_SECONDS_1: i32 = 1;
const RTC_REG_SECONDS_2: i32 = 2;
const RTC_REG_SECONDS_3: i32 = 3;
const RTC_REG_WRITE_PROTECT: i32 = 13;
const RTC_REG_XPRAM: i32 = 14;

#[inline] const fn rtc_cmd_read(r: i32) -> i32 { RTC_FLG_READ | (r << 2) }
#[inline] const fn rtc_cmd_write(r: i32) -> i32 { r << 2 }
#[inline] const fn rtc_cmd_xpram_arg(a: i32) -> i32 { ((a & 0xe0) << 3) | ((a & 0x1f) << 2) }
const RTC_CMD_XPRAM_READ: i32 = rtc_cmd_read(RTC_REG_XPRAM) << 8;
const RTC_CMD_XPRAM_WRITE: i32 = rtc_cmd_write(RTC_REG_XPRAM) << 8;

unsafe fn via_rtc_recv() -> u8 {
    let mut reg = via1[vBufB] & !VIA1B_vRTCClk;
    via1[vDirB] &= !VIA1B_vRTCData;
    let mut data = 0u8;
    for _ in 0..8 {
        via1[vBufB] = reg;
        via1[vBufB] = reg | VIA1B_vRTCClk;
        data = (data << 1) | (via1[vBufB] & VIA1B_vRTCData);
    }
    via1[vDirB] |= VIA1B_vRTCData;
    data
}

unsafe fn via_rtc_send(mut data: u8) {
    let reg = via1[vBufB] & !(VIA1B_vRTCClk | VIA1B_vRTCData);
    for _ in 0..8 {
        let bit = if data & 0x80 != 0 { 1 } else { 0 };
        data <<= 1;
        via1[vBufB] = reg | bit;
        via1[vBufB] = reg | bit | VIA1B_vRTCClk;
    }
}

unsafe fn via_rtc_command(mut command: i32, data: *mut u8) {
    let mut flags: unsigned_long = 0;
    local_irq_save(&mut flags);
    command = (command & !3) | 1;
    via1[vBufB] = (via1[vBufB] | VIA1B_vRTCClk) & !VIA1B_vRTCEnb;
    let is_read;
    if command & 0xff00 != 0 {
        via_rtc_send(((command & 0xff00) >> 8) as u8);
        via_rtc_send(command as u8);
        is_read = command & (RTC_FLG_READ << 8) != 0;
    } else { via_rtc_send(command as u8); is_read = command & RTC_FLG_READ != 0; }
    if is_read { *data = via_rtc_recv(); } else { via_rtc_send(*data); }
    via1[vBufB] |= VIA1B_vRTCEnb;
    local_irq_restore(flags);
}

#[cfg(feature = "CONFIG_NVRAM")]
unsafe fn via_pram_read_byte(offset: i32) -> u8 { let mut temp = 0; via_rtc_command(RTC_CMD_XPRAM_READ | rtc_cmd_xpram_arg(offset), &mut temp); temp }

#[cfg(feature = "CONFIG_NVRAM")]
unsafe fn via_pram_write_byte(data: u8, offset: i32) {
    let mut temp = 0x55; via_rtc_command(rtc_cmd_write(RTC_REG_WRITE_PROTECT), &mut temp);
    temp = data; via_rtc_command(RTC_CMD_XPRAM_WRITE | rtc_cmd_xpram_arg(offset), &mut temp);
    temp = 0x55 | RTC_FLG_WRITE_PROTECT as u8; via_rtc_command(rtc_cmd_write(RTC_REG_WRITE_PROTECT), &mut temp);
}

unsafe fn via_read_time() -> time64_t {
    let mut result = [0u8; 4]; let mut last = [0u8; 4]; let mut count = 1;
    via_rtc_command(rtc_cmd_read(RTC_REG_SECONDS_0), &mut last[3]); via_rtc_command(rtc_cmd_read(RTC_REG_SECONDS_1), &mut last[2]);
    via_rtc_command(rtc_cmd_read(RTC_REG_SECONDS_2), &mut last[1]); via_rtc_command(rtc_cmd_read(RTC_REG_SECONDS_3), &mut last[0]);
    loop {
        via_rtc_command(rtc_cmd_read(RTC_REG_SECONDS_0), &mut result[3]); via_rtc_command(rtc_cmd_read(RTC_REG_SECONDS_1), &mut result[2]);
        via_rtc_command(rtc_cmd_read(RTC_REG_SECONDS_2), &mut result[1]); via_rtc_command(rtc_cmd_read(RTC_REG_SECONDS_3), &mut result[0]);
        let rv = u32::from_ne_bytes(result); let lv = u32::from_ne_bytes(last);
        if rv == lv { return rv as time64_t - RTC_OFFSET; }
        count += 1; if count > 10 { break; } last = result;
    }
    pr_err("{}: failed to read a stable value; got 0x{:08x} then 0x{:08x}\n", "via_read_time", u32::from_ne_bytes(last), u32::from_ne_bytes(result)); 0
}

unsafe fn via_set_rtc_time(tm: *mut rtc_time) {
    let time = mktime64((*tm).tm_year + 1900, (*tm).tm_mon + 1, (*tm).tm_mday, (*tm).tm_hour, (*tm).tm_min, (*tm).tm_sec);
    let mut temp = 0x55; via_rtc_command(rtc_cmd_write(RTC_REG_WRITE_PROTECT), &mut temp);
    let data = (time + RTC_OFFSET) as u32; let bytes = data.to_ne_bytes();
    via_rtc_command(rtc_cmd_write(RTC_REG_SECONDS_0), &mut (bytes[3].clone())); via_rtc_command(rtc_cmd_write(RTC_REG_SECONDS_1), &mut (bytes[2].clone()));
    via_rtc_command(rtc_cmd_write(RTC_REG_SECONDS_2), &mut (bytes[1].clone())); via_rtc_command(rtc_cmd_write(RTC_REG_SECONDS_3), &mut (bytes[0].clone()));
    temp = 0x55 | RTC_FLG_WRITE_PROTECT as u8; via_rtc_command(rtc_cmd_write(RTC_REG_WRITE_PROTECT), &mut temp);
}

unsafe fn via_shutdown() { if rbv_present { via2[rBufB] &= !0x04; } else { via2[vDirB] |= 0x04; via2[vBufB] &= !0x04; mdelay(1000); } }
unsafe fn oss_shutdown() { (*oss).rom_ctrl = OSS_POWEROFF; }

#[cfg(feature = "CONFIG_ADB_CUDA")]
unsafe fn cuda_restart() { let mut req: adb_request = core::mem::zeroed(); if cuda_request(&mut req, core::ptr::null_mut(), 2, CUDA_PACKET, CUDA_RESET_SYSTEM) < 0 { return; } while !req.complete { cuda_poll(); } }

#[cfg(feature = "CONFIG_ADB_CUDA")]
unsafe fn cuda_shutdown() {
    let mut req: adb_request = core::mem::zeroed(); if cuda_request(&mut req, core::ptr::null_mut(), 2, CUDA_PACKET, CUDA_POWERDOWN) < 0 { return; }
    match (*macintosh_config).ident { MAC_MODEL_C660 | MAC_MODEL_Q605 | MAC_MODEL_Q605_ACC | MAC_MODEL_P475 | MAC_MODEL_P475F => return, _ => {} }
    while !req.complete { cuda_poll(); }
}

#[cfg(feature = "CONFIG_NVRAM")]
pub unsafe fn mac_pram_get_size() -> isize { 256 }

#[cfg(feature = "CONFIG_NVRAM")]
pub unsafe fn mac_pram_read_byte(addr: i32) -> u8 { match (*macintosh_config).adb_type {
    MAC_ADB_IOP | MAC_ADB_II | MAC_ADB_PB1 => via_pram_read_byte(addr),
    #[cfg(feature = "CONFIG_ADB_CUDA")] MAC_ADB_EGRET | MAC_ADB_CUDA => cuda_pram_read_byte(addr),
    #[cfg(feature = "CONFIG_ADB_PMU")] MAC_ADB_PB2 => pmu_pram_read_byte(addr), _ => 0xff } }

#[cfg(feature = "CONFIG_NVRAM")]
pub unsafe fn mac_pram_write_byte(val: u8, addr: i32) { match (*macintosh_config).adb_type {
    MAC_ADB_IOP | MAC_ADB_II | MAC_ADB_PB1 => via_pram_write_byte(val, addr),
    #[cfg(feature = "CONFIG_ADB_CUDA")] MAC_ADB_EGRET | MAC_ADB_CUDA => cuda_pram_write_byte(val, addr),
    #[cfg(feature = "CONFIG_ADB_PMU")] MAC_ADB_PB2 => pmu_pram_write_byte(val, addr), _ => {} } }

pub unsafe fn mac_poweroff() -> ! { if oss_present { oss_shutdown(); } else if (*macintosh_config).adb_type == MAC_ADB_II { via_shutdown(); }
    #[cfg(feature = "CONFIG_ADB_CUDA")] if (*macintosh_config).adb_type == MAC_ADB_EGRET || (*macintosh_config).adb_type == MAC_ADB_CUDA { cuda_shutdown(); }
    pr_crit("It is now safe to turn off your Macintosh.\n"); local_irq_disable(); loop {} }

pub unsafe fn mac_reset() -> ! {
    #[cfg(feature = "CONFIG_ADB_CUDA")] if (*macintosh_config).adb_type == MAC_ADB_EGRET || (*macintosh_config).adb_type == MAC_ADB_CUDA { cuda_restart(); }
    // The original 68030/ROM reset paths use architecture-specific inline assembly; retain their control-flow intent.
    local_irq_disable(); if let Some(reset) = ROM_RESET { reset(); } pr_crit("Restart failed. Please restart manually.\n"); loop {}
}

const SECS_PER_MINUTE: i64 = 60; const SECS_PER_HOUR: i64 = 3600; const SECS_PER_DAY: i64 = 86400;

unsafe fn unmktime(time: time64_t, offset: i64, yearp: *mut i32, monp: *mut i32, dayp: *mut i32, hourp: *mut i32, minp: *mut i32, secp: *mut i32) {
    let mut days = time.div_euclid(SECS_PER_DAY); let mut rem = time.rem_euclid(SECS_PER_DAY) + offset;
    while rem < 0 { rem += SECS_PER_DAY; days -= 1; } while rem >= SECS_PER_DAY { rem -= SECS_PER_DAY; days += 1; }
    *hourp = (rem / SECS_PER_HOUR) as i32; rem %= SECS_PER_HOUR; *minp = (rem / SECS_PER_MINUTE) as i32; *secp = (rem % SECS_PER_MINUTE) as i32;
    let mut y = 1970i64; let leap = |v: i64| v % 4 == 0 && (v % 100 != 0 || v % 400 == 0);
    while days < 0 || days >= if leap(y) { 366 } else { 365 } { let yg = y + days / 365 - if days % 365 < 0 { 1 } else { 0 }; days -= (yg-y)*365 + (yg-1).div_euclid(4)-(yg-1).div_euclid(100)+(yg-1).div_euclid(400) - ((y-1).div_euclid(4)-(y-1).div_euclid(100)+(y-1).div_euclid(400)); y = yg; }
    *yearp = y as i32 - 1900; let md = if leap(y) { [0,31,60,91,121,152,182,213,244,274,305,335,366] } else { [0,31,59,90,120,151,181,212,243,273,304,334,365] }; let mut m=11; while days < md[m] { m-=1; } days -= md[m]; *monp=m as i32; *dayp=days as i32+1;
}

pub unsafe fn mac_hwclk(op: i32, t: *mut rtc_time) -> i32 { if op == 0 { let now = match (*macintosh_config).adb_type { MAC_ADB_IOP|MAC_ADB_II|MAC_ADB_PB1 => via_read_time(), _ => 0 }; (*t).tm_wday=0; unmktime(now,0,&mut (*t).tm_year,&mut (*t).tm_mon,&mut (*t).tm_mday,&mut (*t).tm_hour,&mut (*t).tm_min,&mut (*t).tm_sec); } else { match (*macintosh_config).adb_type { MAC_ADB_IOP|MAC_ADB_II|MAC_ADB_PB1 => via_set_rtc_time(t), _ => return -ENODEV } } 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
