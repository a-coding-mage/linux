/* SPDX-License-Identifier: GPL-2.0 */
/* TI sysc interconnect target module defines */

/* Generic sysc found on omap2 and later, also known as type1 */
pub const SYSC_OMAP2_CLOCKACTIVITY: u32 = 3u32 << 8;
pub const SYSC_OMAP2_EMUFREE: u32 = 1u32 << 5;
pub const SYSC_OMAP2_ENAWAKEUP: u32 = 1u32 << 2;
pub const SYSC_OMAP2_SOFTRESET: u32 = 1u32 << 1;
pub const SYSC_OMAP2_AUTOIDLE: u32 = 1u32 << 0;

/* Generic sysc found on omap4 and later, also known as type2 */
pub const SYSC_OMAP4_DMADISABLE: u32 = 1u32 << 16;
pub const SYSC_OMAP4_FREEEMU: u32 = 1u32 << 1; /* Also known as EMUFREE */
pub const SYSC_OMAP4_SOFTRESET: u32 = 1u32 << 0;

/* SmartReflex sysc found on 36xx and later */
pub const SYSC_OMAP3_SR_ENAWAKEUP: u32 = 1u32 << 26;

pub const SYSC_DRA7_MCAN_ENAWAKEUP: u32 = 1u32 << 4;

/* PRUSS sysc found on AM33xx/AM43xx/AM57xx */
pub const SYSC_PRUSS_SUB_MWAIT: u32 = 1u32 << 5;
pub const SYSC_PRUSS_STANDBY_INIT: u32 = 1u32 << 4;

/* SYSCONFIG STANDBYMODE/MIDLEMODE/SIDLEMODE supported by hardware */
pub const SYSC_IDLE_FORCE: u32 = 0;
pub const SYSC_IDLE_NO: u32 = 1;
pub const SYSC_IDLE_SMART: u32 = 2;
pub const SYSC_IDLE_SMART_WKUP: u32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
