/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2016,2017 IBM Corporation.
 */

/*
 * "magic" Event State Buffer (ESB) MMIO offsets.
 *
 * Each interrupt source has a 2-bit state machine called ESB
 * which can be controlled by MMIO. It's made of 2 bits, P and
 * Q. P indicates that an interrupt is pending (has been sent
 * to a queue and is waiting for an EOI). Q indicates that the
 * interrupt has been triggered while pending.
 *
 * This acts as a coalescing mechanism in order to guarantee
 * that a given interrupt only occurs at most once in a queue.
 *
 * When doing an EOI, the Q bit will indicate if the interrupt
 * needs to be re-triggered.
 *
 * The following offsets into the ESB MMIO allow to read or
 * manipulate the PQ bits. They must be used with an 8-bytes
 * load instruction. They all return the previous state of the
 * interrupt (atomically).
 *
 * Additionally, some ESB pages support doing an EOI via a
 * store at 0 and some ESBs support doing a trigger via a
 * separate trigger page.
 */
pub const XIVE_ESB_STORE_EOI: u32 = 0x400; // Store
pub const XIVE_ESB_LOAD_EOI: u32 = 0x000; // Load
pub const XIVE_ESB_GET: u32 = 0x800; // Load
pub const XIVE_ESB_SET_PQ_00: u32 = 0xc00; // Load
pub const XIVE_ESB_SET_PQ_01: u32 = 0xd00; // Load
pub const XIVE_ESB_SET_PQ_10: u32 = 0xe00; // Load
pub const XIVE_ESB_SET_PQ_11: u32 = 0xf00; // Load

/* Load-after-store ordering. */
pub const XIVE_ESB_LD_ST_MO: u32 = 0x40; // Load-after-store ordering

pub const XIVE_ESB_VAL_P: u8 = 0x2;
pub const XIVE_ESB_VAL_Q: u8 = 0x1;
pub const XIVE_ESB_INVALID: u8 = 0xFF;

/* Thread Management (aka "TM") registers. */

/* TM register offsets */
pub const TM_QW0_USER: u32 = 0x000; // All rings
pub const TM_QW1_OS: u32 = 0x010; // Ring 0..2
pub const TM_QW2_HV_POOL: u32 = 0x020; // Ring 0..1
pub const TM_QW3_HV_PHYS: u32 = 0x030; // Ring 0..1

/* Byte offsets inside a QW             QW0 QW1 QW2 QW3 */
pub const TM_NSR: u32 = 0x0; //  +   +   -   +
pub const TM_CPPR: u32 = 0x1; //  -   +   -   +
pub const TM_IPB: u32 = 0x2; //  -   +   +   +
pub const TM_LSMFB: u32 = 0x3; //  -   +   +   +
pub const TM_ACK_CNT: u32 = 0x4; //  -   +   -   -
pub const TM_INC: u32 = 0x5; //  -   +   -   +
pub const TM_AGE: u32 = 0x6; //  -   +   -   +
pub const TM_PIPR: u32 = 0x7; //  -   +   -   +

pub const TM_WORD0: u32 = 0x0;
pub const TM_WORD1: u32 = 0x4;

/* QW word 2 contains the valid bit at the top and other fields depending on the QW. */
pub const TM_WORD2: u32 = 0x8;
pub const TM_QW0W2_VU: u32 = 1u32 << 31;
pub const TM_QW0W2_LOGIC_SERV: u32 = 0x7fff_ffff; // XX 2,31 ?
pub const TM_QW1W2_VO: u32 = 1u32 << 31;
pub const TM_QW1W2_HO: u32 = 1u32 << 30; // P10 XIVE2
pub const TM_QW1W2_OS_CAM: u32 = 0x00ff_ffff;
pub const TM_QW2W2_VP: u32 = 1u32 << 31;
pub const TM_QW2W2_HP: u32 = 1u32 << 30; // P10 XIVE2
pub const TM_QW2W2_POOL_CAM: u32 = 0x00ff_ffff;
pub const TM_QW3W2_VT: u32 = 1u32 << 31;
pub const TM_QW3W2_HT: u32 = 1u32 << 30; // P10 XIVE2
pub const TM_QW3W2_LP: u32 = 1u32 << 25;
pub const TM_QW3W2_LE: u32 = 1u32 << 24;
pub const TM_QW3W2_T: u32 = 1u32;

/* Special CI operations and NSR fields. */
pub const TM_SPC_ACK_EBB: u32 = 0x800; // Load8 ack EBB to reg
pub const TM_SPC_ACK_OS_REG: u32 = 0x810; // Load16 ack OS irq to reg
pub const TM_SPC_PUSH_USR_CTX: u32 = 0x808; // Store32 Push/Validate user context
pub const TM_SPC_PULL_USR_CTX: u32 = 0x808; // Load32 Pull/Invalidate user context
pub const TM_SPC_SET_OS_PENDING: u32 = 0x812; // Store8 Set OS irq pending bit
pub const TM_SPC_PULL_OS_CTX: u32 = 0x818; // Load32/Load64 Pull/Invalidate OS context to reg
pub const TM_SPC_PULL_POOL_CTX: u32 = 0x828; // Load32/Load64 Pull/Invalidate Pool context to reg
pub const TM_SPC_ACK_HV_REG: u32 = 0x830; // Load16 ack HV irq to reg
pub const TM_SPC_PULL_USR_CTX_OL: u32 = 0xc08; // Store8 Pull/Inval usr ctx to odd line
pub const TM_SPC_ACK_OS_EL: u32 = 0xc10; // Store8 ack OS irq to even line
pub const TM_SPC_ACK_HV_POOL_EL: u32 = 0xc20; // Store8 ack HV evt pool to even line
pub const TM_SPC_ACK_HV_EL: u32 = 0xc30; // Store8 ack HV irq to even line

pub const TM_QW0_NSR_EB: u8 = 1u8 << 7;
pub const TM_QW1_NSR_EO: u8 = 1u8 << 7;
pub const TM_QW3_NSR_HE: u8 = 0xc0;
pub const TM_QW3_NSR_HE_NONE: u8 = 0;
pub const TM_QW3_NSR_HE_POOL: u8 = 1;
pub const TM_QW3_NSR_HE_PHYS: u8 = 2;
pub const TM_QW3_NSR_HE_LSI: u8 = 3;
pub const TM_QW3_NSR_I: u8 = 1u8 << 5;
pub const TM_QW3_NSR_GRP_LVL: u8 = 0x1f;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
