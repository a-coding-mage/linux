/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1992 - 1997 Silicon Graphics, Inc.
 */

/*
 * Macros to manipulate the interrupt register on the calling hub chip.
 */

macro_rules! LOCAL_HUB_SEND_INTR {
    ($level:expr) => {
        LOCAL_HUB_S(PI_INT_PEND_MOD, (0x100 | ($level)))
    };
}

macro_rules! REMOTE_HUB_SEND_INTR {
    ($hub:expr, $level:expr) => {
        REMOTE_HUB_S(($hub), PI_INT_PEND_MOD, (0x100 | ($level)))
    };
}

/*
 * When clearing the interrupt, make sure this clear does make it
 * to the hub. Otherwise we could end up losing interrupts.
 * We do an uncached load of the int_pend0 register to ensure this.
 */

macro_rules! LOCAL_HUB_CLR_INTR {
    ($level:expr) => {{
        LOCAL_HUB_S(PI_INT_PEND_MOD, ($level));
        LOCAL_HUB_L(PI_INT_PEND0);
    }};
}

macro_rules! REMOTE_HUB_CLR_INTR {
    ($hub:expr, $level:expr) => {{
        let __hub = $hub;

        REMOTE_HUB_S(__hub, PI_INT_PEND_MOD, ($level));
        REMOTE_HUB_L(__hub, PI_INT_PEND0);
    }};
}

/*
 * Hard-coded interrupt levels:
 */

/*
 *  L0 = SW1
 *  L1 = SW2
 *  L2 = INT_PEND0
 *  L3 = INT_PEND1
 *  L4 = RTC
 *  L5 = Profiling Timer
 *  L6 = Hub Errors
 *  L7 = Count/Compare (T5 counters)
 */

/*
 * INT_PEND0 hard-coded bits.
 */

/*
 * INT_PEND0 bits determined by hardware:
 */
pub const RESERVED_INTR: i32 = 0; /* What is this bit? */
pub const GFX_INTR_A: i32 = 1;
pub const GFX_INTR_B: i32 = 2;
pub const PG_MIG_INTR: i32 = 3;
pub const UART_INTR: i32 = 4;
pub const CC_PEND_A: i32 = 5;
pub const CC_PEND_B: i32 = 6;

/*
 * INT_PEND0 used by the kernel for itself ...
 */
pub const CPU_RESCHED_A_IRQ: i32 = 7;
pub const CPU_RESCHED_B_IRQ: i32 = 8;
pub const CPU_CALL_A_IRQ: i32 = 9;
pub const CPU_CALL_B_IRQ: i32 = 10;

/*
 * INT_PEND1 hard-coded bits:
 */
pub const NI_BRDCAST_ERR_A: i32 = 39;
pub const NI_BRDCAST_ERR_B: i32 = 40;

pub const LLP_PFAIL_INTR_A: i32 = 41; /* see ml/SN/SN0/sysctlr.c */
pub const LLP_PFAIL_INTR_B: i32 = 42;

pub const TLB_INTR_A: i32 = 43; /* used for tlb flush random */
pub const TLB_INTR_B: i32 = 44;

pub const IP27_INTR_0: i32 = 45; /* Reserved for PROM use */
pub const IP27_INTR_1: i32 = 46; /* do not use in Kernel */
pub const IP27_INTR_2: i32 = 47;
pub const IP27_INTR_3: i32 = 48;
pub const IP27_INTR_4: i32 = 49;
pub const IP27_INTR_5: i32 = 50;
pub const IP27_INTR_6: i32 = 51;
pub const IP27_INTR_7: i32 = 52;

pub const BRIDGE_ERROR_INTR: i32 = 53; /* Setup by PROM to catch */
/* Bridge Errors */
pub const DEBUG_INTR_A: i32 = 54;
pub const DEBUG_INTR_B: i32 = 55; /* Used by symmon to stop all cpus */
pub const IO_ERROR_INTR: i32 = 57; /* Setup by PROM */
pub const CLK_ERR_INTR: i32 = 58;
pub const COR_ERR_INTR_A: i32 = 59;
pub const COR_ERR_INTR_B: i32 = 60;
pub const MD_COR_ERR_INTR: i32 = 61;
pub const NI_ERROR_INTR: i32 = 62;
pub const MSC_PANIC_INTR: i32 = 63;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
