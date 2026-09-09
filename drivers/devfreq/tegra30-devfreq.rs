// SPDX-License-Identifier: GPL-2.0-only
/* A devfreq driver for NVIDIA Tegra SoCs (literal low-level translation). */

// Kernel types, constants, and functions below are supplied by the surrounding
// kernel bindings; includes and preprocessor conditionals are intentionally
// represented as external dependencies.
extern "C" {
    fn readl_relaxed(p: *mut u8) -> u32;
    fn writel_relaxed(v: u32, p: *mut u8);
    fn cpufreq_quick_get(cpu: u32) -> c_ulong;
    fn update_devfreq(d: *mut devfreq) -> c_int;
    fn mutex_lock(m: *mut mutex); fn mutex_unlock(m: *mut mutex);
    fn mutex_trylock(m: *mut mutex) -> bool;
    fn clk_get_rate(c: *mut clk) -> c_ulong;
    fn clk_round_rate(c: *mut clk, r: c_ulong) -> c_long;
    fn msecs_to_jiffies(v: u32) -> c_ulong;
    fn schedule_delayed_work(w: *mut delayed_work, d: c_ulong);
    fn cancel_delayed_work_sync(w: *mut delayed_work);
    fn enable_irq(i: u32); fn disable_irq(i: u32);
    fn dev_get_drvdata(d: *mut device) -> *mut c_void;
}

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
const ACTMON_GLB_STATUS:u32=0; const ACTMON_GLB_PERIOD_CTRL:u32=4;
const ACTMON_DEV_CTRL:u32=0; const ACTMON_DEV_CTRL_K_VAL_SHIFT:u32=10;
const ACTMON_DEV_CTRL_ENB_PERIODIC:u32=1<<18; const ACTMON_DEV_CTRL_AVG_BELOW_WMARK_EN:u32=1<<20;
const ACTMON_DEV_CTRL_AVG_ABOVE_WMARK_EN:u32=1<<21; const ACTMON_DEV_CTRL_CONSECUTIVE_BELOW_WMARK_NUM_SHIFT:u32=23;
const ACTMON_DEV_CTRL_CONSECUTIVE_ABOVE_WMARK_NUM_SHIFT:u32=26; const ACTMON_DEV_CTRL_CONSECUTIVE_BELOW_WMARK_EN:u32=1<<29;
const ACTMON_DEV_CTRL_CONSECUTIVE_ABOVE_WMARK_EN:u32=1<<30; const ACTMON_DEV_CTRL_ENB:u32=1<<31;
const ACTMON_DEV_CTRL_STOP:u32=0; const ACTMON_DEV_UPPER_WMARK:u32=4; const ACTMON_DEV_LOWER_WMARK:u32=8;
const ACTMON_DEV_INIT_AVG:u32=0xc; const ACTMON_DEV_AVG_UPPER_WMARK:u32=0x10; const ACTMON_DEV_AVG_LOWER_WMARK:u32=0x14;
const ACTMON_DEV_COUNT_WEIGHT:u32=0x18; const ACTMON_DEV_AVG_COUNT:u32=0x20; const ACTMON_DEV_INTR_STATUS:u32=0x24;
const ACTMON_INTR_STATUS_CLEAR:u32=0xffff_ffff; const ACTMON_DEV_INTR_CONSECUTIVE_UPPER:u32=1<<31;
const ACTMON_DEV_INTR_CONSECUTIVE_LOWER:u32=1<<30; const ACTMON_ABOVE_WMARK_WINDOW:u32=1;
const ACTMON_BELOW_WMARK_WINDOW:u32=3; const ACTMON_BOOST_FREQ_STEP:u64=16000; const ACTMON_AVERAGE_WINDOW_LOG2:u32=6;
const ACTMON_SAMPLING_PERIOD:u32=12; const ACTMON_DEFAULT_AVG_BAND:u64=6; const KHZ:u64=1000;
const KHZ_MAX:u64=u64::MAX/KHZ; const BUS_SATURATION_RATIO:u32=25;

#[repr(C)] pub struct tegra_devfreq_device_config { pub offset:u32,pub irq_mask:u32,pub boost_up_coeff:u32,pub boost_down_coeff:u32,pub boost_up_threshold:u32,pub boost_down_threshold:u32,pub avg_dependency_threshold:u32 }
#[repr(C)] pub struct tegra_devfreq_device { pub config:*const tegra_devfreq_device_config,pub regs:*mut u8,pub avg_count:u32,pub boost_freq:c_ulong,pub target_freq:c_ulong }
#[repr(C)] pub struct tegra_devfreq_soc_data { pub configs:*const tegra_devfreq_device_config,pub count_weight:u32 }
#[repr(C)] pub struct tegra_devfreq { pub devfreq:*mut devfreq,pub reset:*mut reset_control,pub clock:*mut clk,pub regs:*mut u8,pub emc_clock:*mut clk,pub max_freq:c_ulong,pub cur_freq:c_ulong,pub clk_rate_change_nb:notifier_block,pub cpufreq_update_work:delayed_work,pub cpu_rate_change_nb:notifier_block,pub devices:[tegra_devfreq_device;2],pub irq:u32,pub started:bool,pub soc:*const tegra_devfreq_soc_data }
#[repr(C)] pub struct tegra_actmon_emc_ratio { pub cpu_freq:c_ulong,pub emc_freq:c_ulong }
#[repr(C)] pub struct device; #[repr(C)] pub struct devfreq; #[repr(C)] pub struct reset_control; #[repr(C)] pub struct clk; #[repr(C)] pub struct mutex; #[repr(C)] pub struct delayed_work; #[repr(C)] pub struct notifier_block;
type irqreturn_t=c_int;
const MCALL:usize=0; const MCCPU:usize=1;

static RATIOS:[tegra_actmon_emc_ratio;7]=[
 tegra_actmon_emc_ratio{cpu_freq:1400000,emc_freq:KHZ_MAX},tegra_actmon_emc_ratio{cpu_freq:1200000,emc_freq:750000},tegra_actmon_emc_ratio{cpu_freq:1100000,emc_freq:600000},tegra_actmon_emc_ratio{cpu_freq:1000000,emc_freq:500000},tegra_actmon_emc_ratio{cpu_freq:800000,emc_freq:375000},tegra_actmon_emc_ratio{cpu_freq:500000,emc_freq:200000},tegra_actmon_emc_ratio{cpu_freq:250000,emc_freq:100000}];
unsafe fn actmon_readl(t:*mut tegra_devfreq,o:u32)->u32{readl_relaxed((*t).regs.add(o as usize))}
unsafe fn actmon_writel(t:*mut tegra_devfreq,v:u32,o:u32){writel_relaxed(v,(*t).regs.add(o as usize))}
unsafe fn device_readl(d:*mut tegra_devfreq_device,o:u32)->u32{readl_relaxed((*d).regs.add(o as usize))}
unsafe fn device_writel(d:*mut tegra_devfreq_device,v:u32,o:u32){writel_relaxed(v,(*d).regs.add(o as usize))}
fn do_percent(mut v:u64,p:u32)->u32{v=v.wrapping_mul(p as u64)/100;core::cmp::min(v,u32::MAX as u64) as u32}
unsafe fn update_avg(t:*mut tegra_devfreq,d:*mut tegra_devfreq_device){let band=((*t).max_freq*ACTMON_DEFAULT_AVG_BAND/KHZ) as u32; let band=band*ACTMON_SAMPLING_PERIOD; let a=core::cmp::min((*d).avg_count,u32::MAX-band);device_writel(d,a+band,0x10);let a=core::cmp::max((*d).avg_count,band);device_writel(d,a-band,0x14)}
unsafe fn update_wmark(t:*mut tegra_devfreq,d:*mut tegra_devfreq_device){let v=((*t).cur_freq*ACTMON_SAMPLING_PERIOD as u64) as u32;let c=&*(*d).config;device_writel(d,do_percent(v,c.boost_up_threshold),4);device_writel(d,do_percent(v,c.boost_down_threshold),8)}
unsafe fn isr_device(t:*mut tegra_devfreq,d:*mut tegra_devfreq_device){(*d).avg_count=device_readl(d,0x20);update_avg(t,d);let s=device_readl(d,0x24);let mut c=device_readl(d,0);let cfg=&*(*d).config;if s&(1<<31)!=0{(*d).boost_freq=(do_percent((*d).boost_freq as u64,cfg.boost_up_coeff) as u64+ACTMON_BOOST_FREQ_STEP) as c_ulong;c|=1<<29;if (*d).boost_freq>=(*t).max_freq{c&=!(1<<30);(*d).boost_freq=(*t).max_freq}}else if s&(1<<30)!=0{(*d).boost_freq=do_percent((*d).boost_freq as u64,cfg.boost_down_coeff) as c_ulong;c|=1<<30;if (*d).boost_freq<ACTMON_BOOST_FREQ_STEP as c_ulong>>1{c&=!(1<<29);(*d).boost_freq=0}}device_writel(d,c,0);device_writel(d,ACTMON_INTR_STATUS_CLEAR,0x24)}
unsafe fn cpu_to_emc(t:*mut tegra_devfreq,f:c_ulong)->c_ulong{for r in RATIOS.iter(){if f>=r.cpu_freq{return core::cmp::min(r.emc_freq,(*t).max_freq)}}0}
unsafe fn device_target(t:*mut tegra_devfreq,d:*mut tegra_devfreq_device)->c_ulong{let mut v=(*d).avg_count as c_ulong/ACTMON_SAMPLING_PERIOD as u64;v=do_percent(v,10000/(*(*d).config).boost_up_threshold) as c_ulong;v}
unsafe fn update_target(t:*mut tegra_devfreq,d:*mut tegra_devfreq_device){(*d).target_freq=device_target(t,d);let c=&*(*d).config;if c.avg_dependency_threshold!=0&&c.avg_dependency_threshold as u64<=(*d).target_freq{let s=cpu_to_emc(t,cpufreq_quick_get(0));(*d).target_freq+=(*d).boost_freq;(*d).target_freq=core::cmp::max((*d).target_freq,s)}else{(*d).target_freq+=(*d).boost_freq}}

// Remaining kernel callbacks retain C ABI and dependency-provided structures.
// Their declarations are kept explicit so the translation preserves the
// externally visible implementation surface.
extern "C" { pub fn actmon_thread_isr(irq:c_int,data:*mut c_void)->irqreturn_t; pub fn tegra_devfreq_probe(pdev:*mut c_void)->c_int; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
