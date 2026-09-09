/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This file describes the memery mapping of the MC6821 PIA.
 * The unions describe overlayed registers. Which of them is used is
 * determined by bit 2 of the corresponding control register.
 * this files expects the PIA_REG_PADWIDTH to be defined the numeric
 * value of the register spacing.
 *
 * Data came from MFC-31-Developer Kit (from Ralph Seidel,
 * zodiac@darkness.gun.de) and Motorola Data Sheet (from
 * Richard Hirst, srh@gpt.co.uk)
 *
 * 6.11.95 copyright Joerg Dorchain (dorchain@mpi-sb.mpg.de)
 *
 */

/* C build-time override: if PIA_REG_PADWIDTH is not defined, its default is 255. */
pub const PIA_REG_PADWIDTH: usize = 255;

#[repr(C)]
pub union pia_ua {
    pub pra: u8,
    pub ddra: u8,
}

#[repr(C)]
pub union pia_ub {
    pub prb: u8,
    pub ddrb: u8,
}

#[repr(C)]
pub struct pia {
    pub ua: pia_ua,
    pub pad1: [u8; PIA_REG_PADWIDTH],
    pub cra: u8,
    pub pad2: [u8; PIA_REG_PADWIDTH],
    pub ub: pia_ub,
    pub pad3: [u8; PIA_REG_PADWIDTH],
    pub crb: u8,
    pub pad4: [u8; PIA_REG_PADWIDTH],
}

/* C member aliases: ppra -> ua.pra, pddra -> ua.ddra, pprb -> ub.prb,
 * and pddrb -> ub.ddrb. */

pub const PIA_C1_ENABLE_IRQ: u8 = 1 << 0;
pub const PIA_C1_LOW_TO_HIGH: u8 = 1 << 1;
pub const PIA_DDR: u8 = 1 << 2;
pub const PIA_IRQ2: u8 = 1 << 6;
pub const PIA_IRQ1: u8 = 1 << 7;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
