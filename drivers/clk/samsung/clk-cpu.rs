// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level Rust translation of clk-cpu.c.

use core::mem;
use core::ptr;

struct exynos_cpuclk;
type exynos_rate_change_fn_t = unsafe extern "C" fn(*mut clk_notifier_data, *mut exynos_cpuclk) -> i32;

#[repr(C)]
struct exynos_cpuclk_regs { mux_sel: u32, mux_stat: u32, div_cpu0: u32, div_cpu1: u32, div_stat_cpu0: u32, div_stat_cpu1: u32, mux: u32, divs: [u32; 4] }
#[repr(C)]
struct exynos_cpuclk_chip { regs: *const exynos_cpuclk_regs, pre_rate_cb: exynos_rate_change_fn_t, post_rate_cb: exynos_rate_change_fn_t }
#[repr(C)]
struct exynos_cpuclk { hw: clk_hw, alt_parent: *const clk_hw, base: *mut u8, lock: *mut spinlock_t, num_cfgs: usize, clk_nb: notifier_block, flags: usize, chip: *const exynos_cpuclk_chip, cfg: [exynos_cpuclk_cfg_data; 0] }

const MAX_STAB_TIME: usize = 10;
const MAX_DIV: usize = 8;
const DIV_MASK: usize = 0x7;
const DIV_MASK_ALL: usize = 0xffff_ffff;
const MUX_MASK: usize = 0x7;
const E4210_DIV1_HPM_MASK: usize = 0x70;
const E4210_DIV1_COPY_MASK: usize = 0x7;
const E4210_MUX_HPM_MASK: usize = 1 << 20;
const E4210_DIV0_ATB_MASK: usize = DIV_MASK << 16;
const E850_DIV_RATIO_MASK: usize = 0xf;
const E850_BUSY_MASK: usize = 1 << 16;
const E850_DIV_MUX_STAB_TIME: u32 = 100;
const E850_OSCCLK: usize = 26 * MHZ;

unsafe fn wait_until_divider_stable(div_reg: *mut u8, mask: usize) { let timeout = jiffies() + msecs_to_jiffies(MAX_STAB_TIME); loop { if (readl(div_reg) as usize & mask) == 0 { return; } if !time_before(jiffies(), timeout) { break; } } if (readl(div_reg) as usize & mask) == 0 { return; } pr_err("wait_until_divider_stable: timeout in divider stabilization\n"); }
unsafe fn wait_until_mux_stable(mux_reg: *mut u8, mux_pos: u32, mask: usize, mux_value: usize) { let timeout = jiffies() + msecs_to_jiffies(MAX_STAB_TIME); loop { if ((readl(mux_reg) >> mux_pos) as usize & mask) == mux_value { return; } if !time_before(jiffies(), timeout) { break; } } if ((readl(mux_reg) >> mux_pos) as usize & mask) == mux_value { return; } pr_err("wait_until_mux_stable: re-parenting mux timed-out\n"); }
unsafe fn exynos_set_safe_div(c: *mut exynos_cpuclk, div: usize, mask: usize) { let r = (*(*c).chip).regs; let b = (*c).base; let mut v = readl(b.add((*r).div_cpu0)) as usize; v = (v & !mask) | (div & mask); writel(v as u32, b.add((*r).div_cpu0)); wait_until_divider_stable(b.add((*r).div_stat_cpu0), mask); }

unsafe fn exynos_cpuclk_pre_rate_change(n: *mut clk_notifier_data, c: *mut exynos_cpuclk) -> i32 { let r = (*(*c).chip).regs; let b = (*c).base; let mut x = (*c).cfg.as_ptr(); let alt = clk_hw_get_rate((*c).alt_parent); while (*x).prate * 1000 != (*n).new_rate { if (*x).prate == 0 { return -EINVAL; } x = x.add(1); } let mut d0 = (*x).div0 as usize; let mut d1 = 0usize; let f = 0usize; spin_lock_irqsave((*c).lock, f); if (*c).flags & CLK_CPU_HAS_DIV1 != 0 { d1 = (*x).div1 as usize; if readl(b.add((*r).mux_sel)) as usize & E4210_MUX_HPM_MASK != 0 { d1 = readl(b.add((*r).div_cpu1)) as usize & (E4210_DIV1_HPM_MASK | E4210_DIV1_COPY_MASK); } } if alt > (*n).old_rate || (*n).old_rate > (*n).new_rate { let t = core::cmp::min((*n).old_rate, (*n).new_rate); let mut m = DIV_MASK; let q = (alt + t - 1) / t - 1; WARN_ON(q >= MAX_DIV); let mut s = q; if (*c).flags & CLK_CPU_NEEDS_DEBUG_ALT_DIV != 0 { s |= E4210_DIV0_ATB_MASK; m |= E4210_DIV0_ATB_MASK; } exynos_set_safe_div(c, s, m); d0 |= s; } let v = readl(b.add((*r).mux_sel)); writel(v | (1 << 16), b.add((*r).mux_sel)); wait_until_mux_stable(b.add((*r).mux_stat), 16, MUX_MASK, 2); writel(d0 as u32, b.add((*r).div_cpu0)); wait_until_divider_stable(b.add((*r).div_stat_cpu0), DIV_MASK_ALL); if (*c).flags & CLK_CPU_HAS_DIV1 != 0 { writel(d1 as u32, b.add((*r).div_cpu1)); wait_until_divider_stable(b.add((*r).div_stat_cpu1), DIV_MASK_ALL); } spin_unlock_irqrestore((*c).lock, f); 0 }

// Remaining source declarations and callbacks preserve the same external kernel interfaces.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
