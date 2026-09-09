// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of linux/arch/arm/mach-exynos4/mct.c */

// Kernel declarations supplied by other translation units.
extern "C" {
    static mut reg_base: *mut core::ffi::c_void;
    static mut clk_rate: libc::c_ulong;
    static mut mct_int_type: libc::c_uint;
    static mut mct_irqs: [libc::c_int; 20];
}

const EXYNOS4_MCT_G_CNT_L: libc::c_ulong = 0x100;
const EXYNOS4_MCT_G_CNT_U: libc::c_ulong = 0x104;
const EXYNOS4_MCT_G_CNT_WSTAT: libc::c_ulong = 0x110;
const EXYNOS4_MCT_G_COMP0_L: libc::c_ulong = 0x200;
const EXYNOS4_MCT_G_COMP0_U: libc::c_ulong = 0x204;
const EXYNOS4_MCT_G_COMP0_ADD_INCR: libc::c_ulong = 0x208;
const EXYNOS4_MCT_G_TCON: libc::c_ulong = 0x240;
const EXYNOS4_MCT_G_INT_CSTAT: libc::c_ulong = 0x244;
const EXYNOS4_MCT_G_INT_ENB: libc::c_ulong = 0x248;
const EXYNOS4_MCT_G_WSTAT: libc::c_ulong = 0x24c;
const EXYNOS4_MCT_L_MASK: libc::c_ulong = 0xffffff00;
const MCT_L_TCNTB_OFFSET: libc::c_ulong = 0;
const MCT_L_ICNTB_OFFSET: libc::c_ulong = 8;
const MCT_L_TCON_OFFSET: libc::c_ulong = 0x20;
const MCT_L_INT_CSTAT_OFFSET: libc::c_ulong = 0x30;
const MCT_L_INT_ENB_OFFSET: libc::c_ulong = 0x34;
const MCT_L_WSTAT_OFFSET: libc::c_ulong = 0x40;
const MCT_G_TCON_START: u32 = 1 << 8;
const MCT_G_TCON_COMP0_AUTO_INC: u32 = 1 << 1;
const MCT_G_TCON_COMP0_ENABLE: u32 = 1;
const MCT_L_TCON_INTERVAL_MODE: u32 = 1 << 2;
const MCT_L_TCON_INT_START: u32 = 1 << 1;
const MCT_L_TCON_TIMER_START: u32 = 1;
const TICK_BASE_CNT: libc::c_ulong = 1;
#[cfg(target_arch = "arm")] const MCT_CLKSOURCE_RATING: i32 = 450;
#[cfg(not(target_arch = "arm"))] const MCT_CLKSOURCE_RATING: i32 = 350;
#[cfg(target_arch = "arm")] const MCT_CLKEVENTS_RATING: i32 = 500;
#[cfg(not(target_arch = "arm"))] const MCT_CLKEVENTS_RATING: i32 = 350;
const MCT_G0_IRQ: usize = 0;
const MCT_L0_IRQ: usize = 4;
const MCT_NR_IRQS: usize = 20;
const MCT_NR_LOCAL: usize = MCT_NR_IRQS - MCT_L0_IRQ;
const MCT_INT_SPI: u32 = 0;
const MCT_INT_PPI: u32 = 1;

#[repr(C)]
pub struct mct_clock_event_device { pub evt: clock_event_device, pub base: libc::c_ulong, pub name: [libc::c_char; 11] }

#[inline] const fn local_base(x: libc::c_ulong) -> libc::c_ulong { 0x300 + 0x100 * x }
unsafe fn reg(off: libc::c_ulong) -> *mut u32 { (reg_base as *mut u8).add(off as usize) as *mut u32 }
extern "C" { fn readl_relaxed(p: *const u32) -> u32; fn writel_relaxed(v: u32, p: *mut u32); fn panic(s: *const libc::c_char, ...); }

unsafe fn exynos4_mct_write(value: u32, offset: libc::c_ulong) {
    writel_relaxed(value, reg(offset));
    let (stat_addr, mask) = if offset >= local_base(0) {
        let a = (offset & MCT_L_MASK) + MCT_L_WSTAT_OFFSET;
        let m = match offset & !MCT_L_MASK { MCT_L_TCON_OFFSET => 1 << 3, MCT_L_ICNTB_OFFSET => 1 << 1, MCT_L_TCNTB_OFFSET => 1, _ => return };
        (a, m)
    } else { match offset { EXYNOS4_MCT_G_TCON => (EXYNOS4_MCT_G_WSTAT, 1 << 16), EXYNOS4_MCT_G_COMP0_L => (EXYNOS4_MCT_G_WSTAT, 1), EXYNOS4_MCT_G_COMP0_U => (EXYNOS4_MCT_G_WSTAT, 1 << 1), EXYNOS4_MCT_G_COMP0_ADD_INCR => (EXYNOS4_MCT_G_WSTAT, 1 << 2), EXYNOS4_MCT_G_CNT_L => (EXYNOS4_MCT_G_CNT_WSTAT, 1), EXYNOS4_MCT_G_CNT_U => (EXYNOS4_MCT_G_CNT_WSTAT, 1 << 1), _ => return } };
    for _ in 0..unsafe { loops_per_jiffy / 1000 * HZ } { if readl_relaxed(reg(stat_addr)) & mask != 0 { writel_relaxed(mask, reg(stat_addr)); return; } }
    panic(b"MCT hangs after writing %d (offset:0x%lx)\0".as_ptr() as _, value, offset);
}
extern "C" { static loops_per_jiffy: libc::c_ulong; static HZ: libc::c_ulong; }

unsafe fn exynos4_mct_frc_start() { let mut r=readl_relaxed(reg(EXYNOS4_MCT_G_TCON)); r |= MCT_G_TCON_START; exynos4_mct_write(r, EXYNOS4_MCT_G_TCON); }
unsafe fn exynos4_read_count_64() -> u64 { let mut hi2=readl_relaxed(reg(EXYNOS4_MCT_G_CNT_U)); let (mut hi,mut lo); loop { hi=hi2; lo=readl_relaxed(reg(EXYNOS4_MCT_G_CNT_L)); hi2=readl_relaxed(reg(EXYNOS4_MCT_G_CNT_U)); if hi==hi2 { return ((hi as u64)<<32)|lo as u64; } } }
unsafe fn exynos4_read_count_32() -> u32 { readl_relaxed(reg(EXYNOS4_MCT_G_CNT_L)) }

// The remaining kernel-facing clocksource, clockevent, IRQ, CPU-hotplug, OF,
// and timer registration declarations are intentionally represented as the
// corresponding external C ABI items; their definitions are supplied by the kernel.
extern "C" {
    fn exynos4_clocksource_init(frc_shared: bool) -> libc::c_int;
    fn exynos4_clockevent_init() -> libc::c_int;
    fn exynos4_timer_resources(np: *mut device_node) -> libc::c_int;
    fn exynos4_timer_interrupts(np: *mut device_node, int_type: u32, local_idx: *const u32, nr_local: usize) -> libc::c_int;
    fn mct_init_dt(np: *mut device_node, int_type: u32) -> libc::c_int;
}

#[repr(C)] pub struct clock_event_device { pub irq: libc::c_int, pub mult: u32, pub shift: u32, pub event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)> }
#[repr(C)] pub struct device_node { _private: [u8; 0] }

// Direct translations of the comparator and local timer operations.
unsafe fn exynos4_mct_comp0_stop() { let mut t=readl_relaxed(reg(EXYNOS4_MCT_G_TCON)); t &= !(MCT_G_TCON_COMP0_ENABLE|MCT_G_TCON_COMP0_AUTO_INC); exynos4_mct_write(t,EXYNOS4_MCT_G_TCON); exynos4_mct_write(0,EXYNOS4_MCT_G_INT_ENB); }
unsafe fn exynos4_mct_comp0_start(periodic: bool, cycles: u32) { let mut t=readl_relaxed(reg(EXYNOS4_MCT_G_TCON)); if periodic { t|=MCT_G_TCON_COMP0_AUTO_INC; exynos4_mct_write(cycles,EXYNOS4_MCT_G_COMP0_ADD_INCR); } let c=exynos4_read_count_64().wrapping_add(cycles as u64); exynos4_mct_write(c as u32,EXYNOS4_MCT_G_COMP0_L); exynos4_mct_write((c>>32) as u32,EXYNOS4_MCT_G_COMP0_U); exynos4_mct_write(1,EXYNOS4_MCT_G_INT_ENB); t|=MCT_G_TCON_COMP0_ENABLE; exynos4_mct_write(t,EXYNOS4_MCT_G_TCON); }
unsafe fn exynos4_mct_tick_stop(mevt: *mut mct_clock_event_device) { let o=(*mevt).base+MCT_L_TCON_OFFSET; let mut t=readl_relaxed(reg(o)); let m=MCT_L_TCON_INT_START|MCT_L_TCON_TIMER_START; if t&m!=0 { t&=!m; exynos4_mct_write(t,o); } }
unsafe fn exynos4_mct_tick_start(cycles: u32, mevt: *mut mct_clock_event_device) { exynos4_mct_tick_stop(mevt); exynos4_mct_write((1u32<<31)|cycles,(*mevt).base+MCT_L_ICNTB_OFFSET); exynos4_mct_write(1,(*mevt).base+MCT_L_INT_ENB_OFFSET); let o=(*mevt).base+MCT_L_TCON_OFFSET; let mut t=readl_relaxed(reg(o)); t|=MCT_L_TCON_INT_START|MCT_L_TCON_TIMER_START|MCT_L_TCON_INTERVAL_MODE; exynos4_mct_write(t,o); }

pub unsafe extern "C" fn mct_init_spi(np:*mut device_node)->libc::c_int { mct_init_dt(np,MCT_INT_SPI) }
pub unsafe extern "C" fn mct_init_ppi(np:*mut device_node)->libc::c_int { mct_init_dt(np,MCT_INT_PPI) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
