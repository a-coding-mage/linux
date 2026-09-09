/* Cache control for MicroBlaze cache memories. */

/* Dependencies supplied by the surrounding kernel translation. */
use core::ffi::c_ulong;

#[allow(non_camel_case_types)]
type ulong = c_ulong;

#[repr(C)]
pub struct scache {
    pub ie: unsafe extern "C" fn(), pub id: unsafe extern "C" fn(),
    pub ifl: unsafe extern "C" fn(), pub iflr: unsafe extern "C" fn(ulong, ulong),
    pub iin: unsafe extern "C" fn(), pub iinr: unsafe extern "C" fn(ulong, ulong),
    pub de: unsafe extern "C" fn(), pub dd: unsafe extern "C" fn(),
    pub dfl: unsafe extern "C" fn(), pub dflr: unsafe extern "C" fn(ulong, ulong),
    pub din: unsafe extern "C" fn(), pub dinr: unsafe extern "C" fn(ulong, ulong),
}

#[repr(C)]
pub struct CpuInfo { pub use_instr: u32, pub dcache_wb: u32, pub ver_code: u32,
    pub icache_line_length: ulong, pub icache_size: ulong,
    pub dcache_line_length: ulong, pub dcache_size: ulong }

extern "C" {
    pub static mut cpuinfo: CpuInfo;
    fn local_irq_save(flags: *mut ulong);
    fn local_irq_restore(flags: ulong);
    fn pr_debug(fmt: *const u8, ...);
    fn pr_info(fmt: *const u8, ...);
    fn enable_dcache();
    fn invalidate_icache();
    fn enable_icache();
}

const MSR_ICE: u32 = 1 <<  icache_msr_shift();
const MSR_DCE: u32 = 1 <<  dcache_msr_shift();
const PVR2_USE_MSR_INSTR: u32 = 1;
const CPUVER_7_20_A: u32 = 0x0c;
const CPUVER_7_20_D: u32 = 0x0f;

const fn icache_msr_shift() -> u32 { 0 }
const fn dcache_msr_shift() -> u32 { 1 }

#[inline]
unsafe fn __enable_icache_msr() { core::arch::asm!("msrset r0, {0}; nop", const MSR_ICE); }
#[inline]
unsafe fn __disable_icache_msr() { core::arch::asm!("msrclr r0, {0}; nop", const MSR_ICE); }
#[inline]
unsafe fn __enable_dcache_msr() { core::arch::asm!("msrset r0, {0}; nop", const MSR_DCE); }
#[inline]
unsafe fn __disable_dcache_msr() { core::arch::asm!("msrclr r0, {0}; nop", const MSR_DCE); }
#[inline]
unsafe fn __enable_icache_nomsr() { core::arch::asm!("mfs r12, rmsr; nop; ori r12, r12, {0}; mts rmsr, r12; nop", const MSR_ICE); }
#[inline]
unsafe fn __disable_icache_nomsr() { core::arch::asm!("mfs r12, rmsr; nop; andi r12, r12, {0}; mts rmsr, r12; nop", const !MSR_ICE); }
#[inline]
unsafe fn __enable_dcache_nomsr() { core::arch::asm!("mfs r12, rmsr; nop; ori r12, r12, {0}; mts rmsr, r12; nop", const MSR_DCE); }
#[inline]
unsafe fn __disable_dcache_nomsr() { core::arch::asm!("mfs r12, rmsr; nop; andi r12, r12, {0}; mts rmsr, r12; nop", const !MSR_DCE); }

unsafe fn cache_limits(start: &mut ulong, end: &mut ulong, line: ulong, size: ulong) {
    let align = !(line - 1);
    if *start < ulong::MAX - size { *end = core::cmp::min(start.wrapping_add(size), *end); }
    *start &= align;
}

unsafe fn cache_all_loop(size: ulong, line: ulong, op: &str) {
    let mut len = size - line;
    let step = -(line as isize);
    while len != 0 { core::arch::asm!("/* {op} len,r0 */", op = const op, inout(reg) len); len = len.wrapping_add(step as ulong); }
}

unsafe fn range_loop_1(start: &mut ulong, end: &mut ulong, line: ulong, op: &str) {
    let align = !(line - 1);
    *end = if (*end & align) == *end { end.wrapping_sub(line) } else { *end & align };
    let mut p = *start;
    while p <= *end { core::arch::asm!("/* {op} p,r0 */", op = const op, inout(reg) p); p = p.wrapping_add(line); }
}

unsafe fn range_loop_2(start: ulong, end: &mut ulong, line: ulong, op: &str) {
    let align = !(line - 1);
    *end = if (*end & align) == *end { end.wrapping_sub(line) } else { *end & align };
    let mut count = end.wrapping_sub(start);
    while count != 0 { core::arch::asm!("/* {op} start,count */", op = const op, in(reg) start, inout(reg) count); count = count.wrapping_sub(line); }
}

unsafe fn __flush_icache_range_msr_irq(mut start: ulong, mut end: ulong) { cache_limits(&mut start,&mut end,cpuinfo.icache_line_length,cpuinfo.icache_size); let mut f=0; local_irq_save(&mut f); __disable_icache_msr(); range_loop_1(&mut start,&mut end,cpuinfo.icache_line_length,"wic"); __enable_icache_msr(); local_irq_restore(f); }
unsafe fn __flush_icache_range_nomsr_irq(mut start: ulong, mut end: ulong) { cache_limits(&mut start,&mut end,cpuinfo.icache_line_length,cpuinfo.icache_size); let mut f=0; local_irq_save(&mut f); __disable_icache_nomsr(); range_loop_1(&mut start,&mut end,cpuinfo.icache_line_length,"wic"); __enable_icache_nomsr(); local_irq_restore(f); }
unsafe fn __flush_icache_range_noirq(mut start: ulong, mut end: ulong) { cache_limits(&mut start,&mut end,cpuinfo.icache_line_length,cpuinfo.icache_size); range_loop_1(&mut start,&mut end,cpuinfo.icache_line_length,"wic"); }
unsafe fn __flush_icache_all_msr_irq() { let mut f=0; local_irq_save(&mut f); __disable_icache_msr(); cache_all_loop(cpuinfo.icache_size,cpuinfo.icache_line_length,"wic"); __enable_icache_msr(); local_irq_restore(f); }
unsafe fn __flush_icache_all_nomsr_irq() { let mut f=0; local_irq_save(&mut f); __disable_icache_nomsr(); cache_all_loop(cpuinfo.icache_size,cpuinfo.icache_line_length,"wic"); __enable_icache_nomsr(); local_irq_restore(f); }
unsafe fn __flush_icache_all_noirq() { cache_all_loop(cpuinfo.icache_size,cpuinfo.icache_line_length,"wic"); }
unsafe fn __invalidate_dcache_all_msr_irq() { let mut f=0; local_irq_save(&mut f); __disable_dcache_msr(); cache_all_loop(cpuinfo.dcache_size,cpuinfo.dcache_line_length,"wdc"); __enable_dcache_msr(); local_irq_restore(f); }
unsafe fn __invalidate_dcache_all_nomsr_irq() { let mut f=0; local_irq_save(&mut f); __disable_dcache_nomsr(); cache_all_loop(cpuinfo.dcache_size,cpuinfo.dcache_line_length,"wdc"); __enable_dcache_nomsr(); local_irq_restore(f); }
unsafe fn __invalidate_dcache_all_noirq_wt() { cache_all_loop(cpuinfo.dcache_size,cpuinfo.dcache_line_length,"wdc"); }
unsafe fn __invalidate_dcache_all_wb() { cache_all_loop(cpuinfo.dcache_size,cpuinfo.dcache_line_length,"wdc"); }
unsafe fn __invalidate_dcache_range_wb(mut s:ulong,mut e:ulong){cache_limits(&mut s,&mut e,cpuinfo.dcache_line_length,cpuinfo.dcache_size);range_loop_2(s,&mut e,cpuinfo.dcache_line_length,"wdc.clear");}
unsafe fn __invalidate_dcache_range_nomsr_wt(mut s:ulong,mut e:ulong){cache_limits(&mut s,&mut e,cpuinfo.dcache_line_length,cpuinfo.dcache_size);range_loop_1(&mut s,&mut e,cpuinfo.dcache_line_length,"wdc");}
unsafe fn __invalidate_dcache_range_msr_irq_wt(mut s:ulong,mut e:ulong){cache_limits(&mut s,&mut e,cpuinfo.dcache_line_length,cpuinfo.dcache_size);let mut f=0;local_irq_save(&mut f);__disable_dcache_msr();range_loop_1(&mut s,&mut e,cpuinfo.dcache_line_length,"wdc");__enable_dcache_msr();local_irq_restore(f);}
unsafe fn __invalidate_dcache_range_nomsr_irq(mut s:ulong,mut e:ulong){cache_limits(&mut s,&mut e,cpuinfo.dcache_line_length,cpuinfo.dcache_size);let mut f=0;local_irq_save(&mut f);__disable_dcache_nomsr();range_loop_1(&mut s,&mut e,cpuinfo.dcache_line_length,"wdc");__enable_dcache_nomsr();local_irq_restore(f);}
unsafe fn __flush_dcache_all_wb(){cache_all_loop(cpuinfo.dcache_size,cpuinfo.dcache_line_length,"wdc.flush");}
unsafe fn __flush_dcache_range_wb(mut s:ulong,mut e:ulong){cache_limits(&mut s,&mut e,cpuinfo.dcache_line_length,cpuinfo.dcache_size);range_loop_2(s,&mut e,cpuinfo.dcache_line_length,"wdc.flush");}

pub static mut mbc: *mut scache = core::ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn microblaze_cache_init() {
    let use_msr = (cpuinfo.use_instr & PVR2_USE_MSR_INSTR) != 0;
    if use_msr { if cpuinfo.dcache_wb != 0 { mbc = &wb_msr as *const _ as *mut _; } else if cpuinfo.ver_code >= CPUVER_7_20_A { mbc=&wt_msr_noirq as *const _ as *mut _; } else { mbc=&wt_msr as *const _ as *mut _; } }
    else if cpuinfo.dcache_wb != 0 { mbc=&wb_nomsr as *const _ as *mut _; } else if cpuinfo.ver_code >= CPUVER_7_20_A { mbc=&wt_nomsr_noirq as *const _ as *mut _; } else { mbc=&wt_nomsr as *const _ as *mut _; }
    enable_dcache(); invalidate_icache(); enable_icache();
}

static wb_msr: scache = scache{ie:__enable_icache_msr,id:__disable_icache_msr,ifl:__flush_icache_all_noirq,iflr:__flush_icache_range_noirq,iin:__flush_icache_all_noirq,iinr:__flush_icache_range_noirq,de:__enable_dcache_msr,dd:__disable_dcache_msr,dfl:__flush_dcache_all_wb,dflr:__flush_dcache_range_wb,din:__invalidate_dcache_all_wb,dinr:__invalidate_dcache_range_wb};
static wb_nomsr: scache = scache{ie:__enable_icache_nomsr,id:__disable_icache_nomsr,ifl:__flush_icache_all_noirq,iflr:__flush_icache_range_noirq,iin:__flush_icache_all_noirq,iinr:__flush_icache_range_noirq,de:__enable_dcache_nomsr,dd:__disable_dcache_nomsr,dfl:__flush_dcache_all_wb,dflr:__flush_dcache_range_wb,din:__invalidate_dcache_all_wb,dinr:__invalidate_dcache_range_wb};
static wt_msr: scache = scache{ie:__enable_icache_msr,id:__disable_icache_msr,ifl:__flush_icache_all_msr_irq,iflr:__flush_icache_range_msr_irq,iin:__flush_icache_all_msr_irq,iinr:__flush_icache_range_msr_irq,de:__enable_dcache_msr,dd:__disable_dcache_msr,dfl:__invalidate_dcache_all_msr_irq,dflr:__invalidate_dcache_range_msr_irq_wt,din:__invalidate_dcache_all_msr_irq,dinr:__invalidate_dcache_range_msr_irq_wt};
static wt_nomsr: scache = scache{ie:__enable_icache_nomsr,id:__disable_icache_nomsr,ifl:__flush_icache_all_nomsr_irq,iflr:__flush_icache_range_nomsr_irq,iin:__flush_icache_all_nomsr_irq,iinr:__flush_icache_range_nomsr_irq,de:__enable_dcache_nomsr,dd:__disable_dcache_nomsr,dfl:__invalidate_dcache_all_nomsr_irq,dflr:__invalidate_dcache_range_nomsr_irq,din:__invalidate_dcache_all_nomsr_irq,dinr:__invalidate_dcache_range_nomsr_irq};
static wt_msr_noirq: scache = scache{ie:__enable_icache_msr,id:__disable_icache_msr,ifl:__flush_icache_all_noirq,iflr:__flush_icache_range_noirq,iin:__flush_icache_all_noirq,iinr:__flush_icache_range_noirq,de:__enable_dcache_msr,dd:__disable_dcache_msr,dfl:__invalidate_dcache_all_noirq_wt,dflr:__invalidate_dcache_range_nomsr_wt,din:__invalidate_dcache_all_noirq_wt,dinr:__invalidate_dcache_range_nomsr_wt};
static wt_nomsr_noirq: scache = scache{ie:__enable_icache_nomsr,id:__disable_icache_nomsr,ifl:__flush_icache_all_noirq,iflr:__flush_icache_range_noirq,iin:__flush_icache_all_noirq,iinr:__flush_icache_range_noirq,de:__enable_dcache_nomsr,dd:__disable_dcache_nomsr,dfl:__invalidate_dcache_all_noirq_wt,dflr:__invalidate_dcache_range_nomsr_wt,din:__invalidate_dcache_all_noirq_wt,dinr:__invalidate_dcache_range_nomsr_wt};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
