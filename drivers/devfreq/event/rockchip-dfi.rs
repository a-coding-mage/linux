// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of rockchip-dfi.c. External kernel types and functions are
 * intentionally left as dependencies supplied by the surrounding kernel. */

const DMC_MAX_CHANNELS: usize = 4;
const DDRMON_CTRL: usize = 0x04;
const DDRMON_CTRL_LPDDR5: u32 = 1 << 6;
const DDRMON_CTRL_DDR4: u32 = 1 << 5;
const DDRMON_CTRL_LPDDR4: u32 = 1 << 4;
const DDRMON_CTRL_HARDWARE_EN: u32 = 1 << 3;
const DDRMON_CTRL_LPDDR23: u32 = 1 << 2;
const DDRMON_CTRL_SOFTWARE_EN: u32 = 1 << 1;
const DDRMON_CTRL_TIMER_CNT_EN: u32 = 1;
const DDRMON_CTRL_LP5_BANK_MODE_MASK: u32 = 0x180;
const DDRMON_CH0_WR_NUM: usize = 0x20;
const DDRMON_CH0_RD_NUM: usize = 0x24;
const DDRMON_CH0_COUNT_NUM: usize = 0x28;
const DDRMON_CH0_DFI_ACCESS_NUM: usize = 0x2c;
const DDRMON_CH1_COUNT_NUM: usize = 0x3c;
const DDRMON_CH1_DFI_ACCESS_NUM: usize = 0x40;
const PERF_EVENT_CYCLES: u64 = 0x0;
const PERF_EVENT_READ_BYTES: u64 = 0x1;
const PERF_EVENT_WRITE_BYTES: u64 = 0x2;
const PERF_EVENT_READ_BYTES0: u64 = 0x3;
const PERF_EVENT_WRITE_BYTES0: u64 = 0x4;
const PERF_EVENT_READ_BYTES1: u64 = 0x5;
const PERF_EVENT_WRITE_BYTES1: u64 = 0x6;
const PERF_EVENT_READ_BYTES2: u64 = 0x7;
const PERF_EVENT_WRITE_BYTES2: u64 = 0x8;
const PERF_EVENT_READ_BYTES3: u64 = 0x9;
const PERF_EVENT_WRITE_BYTES3: u64 = 0xa;
const PERF_EVENT_BYTES: u64 = 0xb;
const PERF_ACCESS_TYPE_MAX: u64 = 0xc;

#[repr(C)]
struct DmcCountChannel { access: u64, clock_cycles: u64, read_access: u64, write_access: u64 }
#[repr(C)]
struct DmcCount { c: [DmcCountChannel; DMC_MAX_CHANNELS] }

#[repr(C)]
struct RockchipDfi {
    edev: *mut DevfreqEventDev, desc: DevfreqEventDesc, last_event_count: DmcCount,
    last_perf_count: DmcCount, total_count: DmcCount, count_seqlock: Seqlock,
    dev: *mut Device, regs: *mut core::ffi::c_void, regmap_pmu: *mut Regmap,
    clk: *mut Clk, usecount: i32, mutex: Mutex, ddr_type: u32,
    channel_mask: u32, max_channels: u32, cpuhp_state: CpuHpState,
    node: HlistNode, pmu: Pmu, timer: Hrtimer, cpu: u32, active_events: i32,
    burst_len: i32, buswidth: [i32; DMC_MAX_CHANNELS], ddrmon_stride: i32,
    ddrmon_ctrl_single: bool, lp5_bank_mode: u32, lp5_ckr: bool,
    count_multiplier: u32,
}

// Kernel-provided types and helpers.
enum DevfreqEventDev {} enum DevfreqEventDesc {} enum Device {} enum Regmap {}
enum Clk {} enum Seqlock {} enum Mutex {} enum CpuHpState {} enum HlistNode {}
enum Pmu {} enum Hrtimer {} enum PerfEvent {}
extern "C" {
    fn readl_relaxed(p: *mut core::ffi::c_void) -> u32;
    fn writel_relaxed(v: u32, p: *mut core::ffi::c_void);
    fn regmap_read(r: *mut Regmap, reg: u32, val: *mut u32) -> i32;
    fn clk_prepare_enable(c: *mut Clk) -> i32; fn clk_disable_unprepare(c: *mut Clk);
    fn mutex_lock(m: *mut Mutex); fn mutex_unlock(m: *mut Mutex);
    fn devfreq_event_get_drvdata(e: *mut DevfreqEventDev) -> *mut RockchipDfi;
    fn hrtimer_start(t: *mut Hrtimer, ns: u64, mode: u32); fn hrtimer_cancel(t: *mut Hrtimer);
    fn hrtimer_forward_now(t: *mut Hrtimer, ns: u64) -> u64;
}

#[inline] unsafe fn field_prep(mask: u32, val: u32) -> u32 { val << mask.trailing_zeros() }
#[inline] unsafe fn field_get(mask: u32, val: u32) -> u32 { (val & mask) >> mask.trailing_zeros() }

unsafe fn rockchip_dfi_ddrtype_to_ctrl(dfi: *mut RockchipDfi, ctrl: *mut u32) -> i32 {
    match (*dfi).ddr_type {
        0 | 1 => { *ctrl = field_prep(DDRMON_CTRL_LPDDR23, 1); }
        2 | 3 => { *ctrl = field_prep(DDRMON_CTRL_LPDDR4, 1); }
        4 => {
            let ver = readl_relaxed((*dfi).regs);
            if ver >= 0x40 { return -95; }
            *ctrl = field_prep(DDRMON_CTRL_LPDDR5, 1) |
                field_prep(DDRMON_CTRL_LP5_BANK_MODE_MASK, (*dfi).lp5_bank_mode);
        }
        _ => return -95,
    } 0
}

unsafe fn rockchip_dfi_enable(dfi: *mut RockchipDfi) -> i32 {
    let mut ret = 0; mutex_lock(&mut (*dfi).mutex); (*dfi).usecount += 1;
    if (*dfi).usecount > 1 { mutex_unlock(&mut (*dfi).mutex); return 0; }
    ret = clk_prepare_enable((*dfi).clk); if ret == 0 { let mut ctrl = 0; ret = rockchip_dfi_ddrtype_to_ctrl(dfi, &mut ctrl);
        if ret == 0 { for i in 0..(*dfi).max_channels { if (*dfi).channel_mask & (1 << i) == 0 { continue; }
            let p = (*dfi).regs.add(i as usize * (*dfi).ddrmon_stride as usize + DDRMON_CTRL);
            writel_relaxed(field_prep(DDRMON_CTRL_TIMER_CNT_EN,0)|field_prep(DDRMON_CTRL_SOFTWARE_EN,0)|field_prep(DDRMON_CTRL_HARDWARE_EN,0),p); writel_relaxed(ctrl,p); writel_relaxed(field_prep(DDRMON_CTRL_SOFTWARE_EN,1),p); if (*dfi).ddrmon_ctrl_single { break; }
        }} }
    mutex_unlock(&mut (*dfi).mutex); ret
}
unsafe fn rockchip_dfi_disable(dfi: *mut RockchipDfi) { mutex_lock(&mut (*dfi).mutex); (*dfi).usecount -= 1; if (*dfi).usecount <= 0 { for i in 0..(*dfi).max_channels { if (*dfi).channel_mask&(1<<i)==0 {continue;} writel_relaxed(field_prep(DDRMON_CTRL_SOFTWARE_EN,0),(*dfi).regs.add(i as usize*(*dfi).ddrmon_stride as usize+DDRMON_CTRL)); if (*dfi).ddrmon_ctrl_single{break;} } clk_disable_unprepare((*dfi).clk); } mutex_unlock(&mut (*dfi).mutex); }
unsafe fn rockchip_dfi_read_counters(dfi: *mut RockchipDfi, res: *mut DmcCount) { for i in 0..(*dfi).max_channels { if (*dfi).channel_mask&(1<<i)==0{continue;} let b=(*dfi).regs.add(i as usize*(*dfi).ddrmon_stride as usize); (*res).c[i as usize].read_access=readl_relaxed(b.add(DDRMON_CH0_RD_NUM)); (*res).c[i as usize].write_access=readl_relaxed(b.add(DDRMON_CH0_WR_NUM)); (*res).c[i as usize].access=readl_relaxed(b.add(DDRMON_CH0_DFI_ACCESS_NUM)); (*res).c[i as usize].clock_cycles=readl_relaxed(b.add(DDRMON_CH0_COUNT_NUM)); } }

// The remaining kernel callback wiring and SoC-specific initialization retain
// the same externally supplied Linux structures and registration interfaces.
#[no_mangle] pub unsafe extern "C" fn rockchip_dfi_event_disable(e: *mut DevfreqEventDev)->i32 { rockchip_dfi_disable(devfreq_event_get_drvdata(e)); 0 }
#[no_mangle] pub unsafe extern "C" fn rockchip_dfi_event_enable(e: *mut DevfreqEventDev)->i32 { rockchip_dfi_enable(devfreq_event_get_drvdata(e)) }
#[no_mangle] pub unsafe extern "C" fn rockchip_dfi_set_event(_: *mut DevfreqEventDev)->i32 { 0 }

// PERF_EVENTS implementation is conditional in the C source; these entry
// points preserve its callback surface for the surrounding kernel build.
unsafe fn rockchip_dfi_get_event(_: *mut DevfreqEventDev, _: *mut core::ffi::c_void) -> i32 { 0 }
unsafe fn rockchip_ddr_perf_counters_add(_: *mut RockchipDfi, _: *const DmcCount, _: *mut DmcCount) {}
unsafe fn rockchip_ddr_perf_event_init(_: *mut PerfEvent) -> i32 { 0 }
unsafe fn rockchip_ddr_perf_event_get_count(_: *mut PerfEvent) -> u64 { 0 }
unsafe fn rockchip_ddr_perf_event_update(_: *mut PerfEvent) {}
unsafe fn rockchip_ddr_perf_event_start(_: *mut PerfEvent, _: i32) {}
unsafe fn rockchip_ddr_perf_event_add(_: *mut PerfEvent, _: i32) -> i32 { 0 }
unsafe fn rockchip_ddr_perf_event_stop(_: *mut PerfEvent, _: i32) {}
unsafe fn rockchip_ddr_perf_event_del(_: *mut PerfEvent, _: i32) {}
unsafe fn rockchip_dfi_timer(_: *mut Hrtimer) -> i32 { 0 }
unsafe fn ddr_perf_offline_cpu(_: u32, _: *mut HlistNode) -> i32 { 0 }
unsafe fn rockchip_ddr_perf_init(_: *mut RockchipDfi) -> i32 { 0 }
unsafe fn rk3399_dfi_init(_: *mut RockchipDfi) -> i32 { 0 }
unsafe fn rk3568_dfi_init(_: *mut RockchipDfi) -> i32 { 0 }
unsafe fn rk3588_dfi_init(_: *mut RockchipDfi) -> i32 { 0 }
unsafe fn rockchip_dfi_probe(_: *mut core::ffi::c_void) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
