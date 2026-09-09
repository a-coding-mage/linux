/*
 * OMAP Dual-Mode Timers
 *
 * Copyright (C) 2010 Texas Instruments Incorporated - https://www.ti.com/
 * Tarun Kanti DebBarma <tarun.kanti@ti.com>
 * Thara Gopinath <thara@ti.com>
 *
 * Platform device conversion and hwmod support.
 *
 * Copyright (C) 2005 Nokia Corporation
 * Author: Lauri Leukkunen <lauri.leukkunen@nokia.com>
 * PWM and clock framwork support by Timo Teras.
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the
 * Free Software Foundation; either version 2 of the License, or (at your
 * option) any later version.
 *
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED
 * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN
 * NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 675 Mass Ave, Cambridge, MA 02139, USA.
 */

/* clock sources */
pub const OMAP_TIMER_SRC_SYS_CLK: u32 = 0x00;
pub const OMAP_TIMER_SRC_32_KHZ: u32 = 0x01;
pub const OMAP_TIMER_SRC_EXT_CLK: u32 = 0x02;

/* timer interrupt enable bits */
pub const OMAP_TIMER_INT_CAPTURE: u32 = 1 << 2;
pub const OMAP_TIMER_INT_OVERFLOW: u32 = 1 << 1;
pub const OMAP_TIMER_INT_MATCH: u32 = 1 << 0;

/* trigger types */
pub const OMAP_TIMER_TRIGGER_NONE: u32 = 0x00;
pub const OMAP_TIMER_TRIGGER_OVERFLOW: u32 = 0x01;
pub const OMAP_TIMER_TRIGGER_OVERFLOW_AND_COMPARE: u32 = 0x02;

/* timer capabilities used in hwmod database */
pub const OMAP_TIMER_SECURE: u32 = 0x80000000;
pub const OMAP_TIMER_ALWON: u32 = 0x40000000;
pub const OMAP_TIMER_HAS_PWM: u32 = 0x20000000;
pub const OMAP_TIMER_NEEDS_RESET: u32 = 0x10000000;
pub const OMAP_TIMER_HAS_DSP_IRQ: u32 = 0x08000000;

#[repr(C)]
pub struct omap_dm_timer {}

extern "C" {
    pub fn omap_dm_timer_modify_idlect_mask(inputmask: u32) -> u32;
}

/*
 * Do not use the defines below, they are not needed. They should be only
 * used by dmtimer.c and sys_timer related code.
 */

/*
 * The interrupt registers are different between v1 and v2 ip.
 * These registers are offsets from timer->iobase.
 */
pub const OMAP_TIMER_ID_OFFSET: u32 = 0x00;
pub const OMAP_TIMER_OCP_CFG_OFFSET: u32 = 0x10;
pub const OMAP_TIMER_V1_SYS_STAT_OFFSET: u32 = 0x14;
pub const OMAP_TIMER_V1_STAT_OFFSET: u32 = 0x18;
pub const OMAP_TIMER_V1_INT_EN_OFFSET: u32 = 0x1c;
pub const OMAP_TIMER_V2_IRQSTATUS_RAW: u32 = 0x24;
pub const OMAP_TIMER_V2_IRQSTATUS: u32 = 0x28;
pub const OMAP_TIMER_V2_IRQENABLE_SET: u32 = 0x2c;
pub const OMAP_TIMER_V2_IRQENABLE_CLR: u32 = 0x30;

/*
 * The functional registers have a different base on v1 and v2 ip.
 * These registers are offsets from timer->func_base. The func_base
 * is samae as io_base for v1 and io_base + 0x14 for v2 ip.
 *
 */
pub const OMAP_TIMER_V2_FUNC_OFFSET: u32 = 0x14;
pub const _OMAP_TIMER_WAKEUP_EN_OFFSET: u32 = 0x20;
pub const _OMAP_TIMER_CTRL_OFFSET: u32 = 0x24;
pub const OMAP_TIMER_CTRL_GPOCFG: u32 = 1 << 14;
pub const OMAP_TIMER_CTRL_CAPTMODE: u32 = 1 << 13;
pub const OMAP_TIMER_CTRL_PT: u32 = 1 << 12;
pub const OMAP_TIMER_CTRL_TCM_LOWTOHIGH: u32 = 0x1 << 8;
pub const OMAP_TIMER_CTRL_TCM_HIGHTOLOW: u32 = 0x2 << 8;
pub const OMAP_TIMER_CTRL_TCM_BOTHEDGES: u32 = 0x3 << 8;
pub const OMAP_TIMER_CTRL_SCPWM: u32 = 1 << 7;
pub const OMAP_TIMER_CTRL_CE: u32 = 1 << 6; /* compare enable */
pub const OMAP_TIMER_CTRL_PRE: u32 = 1 << 5; /* prescaler enable */
pub const OMAP_TIMER_CTRL_PTV_SHIFT: u32 = 2; /* prescaler value shift */
pub const OMAP_TIMER_CTRL_POSTED: u32 = 1 << 2;
pub const OMAP_TIMER_CTRL_AR: u32 = 1 << 1; /* auto-reload enable */
pub const OMAP_TIMER_CTRL_ST: u32 = 1 << 0; /* start timer */
pub const _OMAP_TIMER_COUNTER_OFFSET: u32 = 0x28;
pub const _OMAP_TIMER_LOAD_OFFSET: u32 = 0x2c;
pub const _OMAP_TIMER_TRIGGER_OFFSET: u32 = 0x30;
pub const _OMAP_TIMER_WRITE_PEND_OFFSET: u32 = 0x34;
pub const WP_NONE: u32 = 0; /* no write pending bit */
pub const WP_TCLR: u32 = 1 << 0;
pub const WP_TCRR: u32 = 1 << 1;
pub const WP_TLDR: u32 = 1 << 2;
pub const WP_TTGR: u32 = 1 << 3;
pub const WP_TMAR: u32 = 1 << 4;
pub const WP_TPIR: u32 = 1 << 5;
pub const WP_TNIR: u32 = 1 << 6;
pub const WP_TCVR: u32 = 1 << 7;
pub const WP_TOCR: u32 = 1 << 8;
pub const WP_TOWR: u32 = 1 << 9;
pub const _OMAP_TIMER_MATCH_OFFSET: u32 = 0x38;
pub const _OMAP_TIMER_CAPTURE_OFFSET: u32 = 0x3c;
pub const _OMAP_TIMER_IF_CTRL_OFFSET: u32 = 0x40;
pub const _OMAP_TIMER_CAPTURE2_OFFSET: u32 = 0x44; /* TCAR2, 34xx only */
pub const _OMAP_TIMER_TICK_POS_OFFSET: u32 = 0x48; /* TPIR, 34xx only */
pub const _OMAP_TIMER_TICK_NEG_OFFSET: u32 = 0x4c; /* TNIR, 34xx only */
pub const _OMAP_TIMER_TICK_COUNT_OFFSET: u32 = 0x50; /* TCVR, 34xx only */
pub const _OMAP_TIMER_TICK_INT_MASK_SET_OFFSET: u32 = 0x54; /* TOCR, 34xx only */
pub const _OMAP_TIMER_TICK_INT_MASK_COUNT_OFFSET: u32 = 0x58; /* TOWR, 34xx only */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
