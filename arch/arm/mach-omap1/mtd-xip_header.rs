/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * MTD primitives for XIP support. Architecture specific functions.
 *
 * Do not include this file directly. It's included from linux/mtd/xip.h
 *
 * Author: Vladimir Barinov <vbarinov@embeddedalley.com>
 *
 * (c) 2005 MontaVista Software, Inc.
 */

// The declarations supplied by "hardware.h" and <linux/soc/ti/omap1-io.h>
// are external dependencies of this translation.

pub const OMAP_MPU_TIMER_BASE: usize = 0xfffec500;
pub const OMAP_MPU_TIMER_OFFSET: usize = 0x100;

#[repr(C)]
pub struct xip_omap_mpu_timer_regs_t {
    pub cntl: u32,    /* CNTL_TIMER, R/W */
    pub load_tim: u32, /* LOAD_TIM,   W */
    pub read_tim: u32, /* READ_TIM,   R */
}

#[inline]
pub unsafe fn xip_omap_mpu_timer_base(n: i32) -> *mut xip_omap_mpu_timer_regs_t {
    // OMAP1_IO_ADDRESS(OMAP_MPU_TIMER_BASE + (n) * OMAP_MPU_TIMER_OFFSET)
    OMAP1_IO_ADDRESS(
        OMAP_MPU_TIMER_BASE.wrapping_add((n as usize).wrapping_mul(OMAP_MPU_TIMER_OFFSET)),
    ) as *mut xip_omap_mpu_timer_regs_t
}

#[inline]
pub unsafe fn xip_omap_mpu_timer_read(nr: i32) -> usize {
    let timer: *const xip_omap_mpu_timer_regs_t = xip_omap_mpu_timer_base(nr);
    core::ptr::read_volatile(core::ptr::addr_of!((*timer).read_tim)) as usize
}

#[inline]
pub unsafe fn xip_irqpending() -> u32 {
    omap_readl(OMAP_IH1_ITR) & !omap_readl(OMAP_IH1_MIR)
}

#[inline]
pub unsafe fn xip_currtime() -> usize {
    !xip_omap_mpu_timer_read(0)
}

/*
 * It's permitted to do approximation for xip_elapsed_since macro
 * (see linux/mtd/xip.h)
 */
#[inline]
pub unsafe fn xip_elapsed_since(x: usize) -> isize {
    ((!xip_omap_mpu_timer_read(0)).wrapping_sub(x) / 6) as isize
}

/*
 * xip_cpu_idle() is used when waiting for a delay equal or larger than
 * the system timer tick period.  This should put the CPU into idle mode
 * to save power and to be woken up only when some interrupts are pending.
 * As above, this should not rely upon standard kernel code.
 */
#[inline]
pub unsafe fn xip_cpu_idle() {
    core::arch::asm!("mcr p15, 0, {0}, c7, c0, 4", in(reg) 1u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
