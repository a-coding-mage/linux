/* Rust translation of ni_stc.h */
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

#[inline(always)]
pub const fn BIT(n: u32) -> u32 { 1u32 << n }

/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Register descriptions for NI DAQ-STC chip
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1998-9 David A. Schleef <ds@schleef.org>
 */

/*
 * References:
 *   DAQ-STC Technical Reference Manual
 */


// dependency: #include "ni_tio.h"
// dependency: #include "ni_routes.h"

/*
 * Registers in the National Instruments DAQ-STC chip
 */

pub const NISTC_INTA_ACK_REG: u32 = 2;
pub const NISTC_INTA_ACK_G0_GATE: u32 = BIT(15);
pub const NISTC_INTA_ACK_G0_TC: u32 = BIT(14);
pub const NISTC_INTA_ACK_AI_ERR: u32 = BIT(13);
pub const NISTC_INTA_ACK_AI_STOP: u32 = BIT(12);
pub const NISTC_INTA_ACK_AI_START: u32 = BIT(11);
pub const NISTC_INTA_ACK_AI_START2: u32 = BIT(10);
pub const NISTC_INTA_ACK_AI_START1: u32 = BIT(9);
pub const NISTC_INTA_ACK_AI_SC_TC: u32 = BIT(8);
pub const NISTC_INTA_ACK_AI_SC_TC_ERR: u32 = BIT(7);
pub const NISTC_INTA_ACK_G0_TC_ERR: u32 = BIT(6);
pub const NISTC_INTA_ACK_G0_GATE_ERR: u32 = BIT(5);
pub const NISTC_INTA_ACK_AI_ALL: u32 = (NISTC_INTA_ACK_AI_ERR |;
					 NISTC_INTA_ACK_AI_STOP |	\
					 NISTC_INTA_ACK_AI_START |	\
					 NISTC_INTA_ACK_AI_START2 |	\
					 NISTC_INTA_ACK_AI_START1 |	\
					 NISTC_INTA_ACK_AI_SC_TC |	\
					 NISTC_INTA_ACK_AI_SC_TC_ERR)

pub const NISTC_INTB_ACK_REG: u32 = 3;
pub const NISTC_INTB_ACK_G1_GATE: u32 = BIT(15);
pub const NISTC_INTB_ACK_G1_TC: u32 = BIT(14);
pub const NISTC_INTB_ACK_AO_ERR: u32 = BIT(13);
pub const NISTC_INTB_ACK_AO_STOP: u32 = BIT(12);
pub const NISTC_INTB_ACK_AO_START: u32 = BIT(11);
pub const NISTC_INTB_ACK_AO_UPDATE: u32 = BIT(10);
pub const NISTC_INTB_ACK_AO_START1: u32 = BIT(9);
pub const NISTC_INTB_ACK_AO_BC_TC: u32 = BIT(8);
pub const NISTC_INTB_ACK_AO_UC_TC: u32 = BIT(7);
pub const NISTC_INTB_ACK_AO_UI2_TC: u32 = BIT(6);
pub const NISTC_INTB_ACK_AO_UI2_TC_ERR: u32 = BIT(5);
pub const NISTC_INTB_ACK_AO_BC_TC_ERR: u32 = BIT(4);
pub const NISTC_INTB_ACK_AO_BC_TC_TRIG_ERR: u32 = BIT(3);
pub const NISTC_INTB_ACK_G1_TC_ERR: u32 = BIT(2);
pub const NISTC_INTB_ACK_G1_GATE_ERR: u32 = BIT(1);
pub const NISTC_INTB_ACK_AO_ALL: u32 = (NISTC_INTB_ACK_AO_ERR |;
					 NISTC_INTB_ACK_AO_STOP |	\
					 NISTC_INTB_ACK_AO_START |	\
					 NISTC_INTB_ACK_AO_UPDATE |	\
					 NISTC_INTB_ACK_AO_START1 |	\
					 NISTC_INTB_ACK_AO_BC_TC |	\
					 NISTC_INTB_ACK_AO_UC_TC |	\
					 NISTC_INTB_ACK_AO_BC_TC_ERR |	\
					 NISTC_INTB_ACK_AO_BC_TC_TRIG_ERR)

pub const NISTC_AI_CMD2_REG: u32 = 4;
pub const NISTC_AI_CMD2_END_ON_SC_TC: u32 = BIT(15);
pub const NISTC_AI_CMD2_END_ON_EOS: u32 = BIT(14);
pub const NISTC_AI_CMD2_START1_DISABLE: u32 = BIT(11);
pub const NISTC_AI_CMD2_SC_SAVE_TRACE: u32 = BIT(10);
pub const NISTC_AI_CMD2_SI_SW_ON_SC_TC: u32 = BIT(9);
pub const NISTC_AI_CMD2_SI_SW_ON_STOP: u32 = BIT(8);
pub const NISTC_AI_CMD2_SI_SW_ON_TC: u32 = BIT(7);
pub const NISTC_AI_CMD2_SC_SW_ON_TC: u32 = BIT(4);
pub const NISTC_AI_CMD2_STOP_PULSE: u32 = BIT(3);
pub const NISTC_AI_CMD2_START_PULSE: u32 = BIT(2);
pub const NISTC_AI_CMD2_START2_PULSE: u32 = BIT(1);
pub const NISTC_AI_CMD2_START1_PULSE: u32 = BIT(0);

pub const NISTC_AO_CMD2_REG: u32 = 5;
macro_rules! NISTC_AO_CMD2_END_ON_BC_TC {
    () => { (x) (((x) & 0x3) << 14) };
}

pub const NISTC_AO_CMD2_START_STOP_GATE_ENA: u32 = BIT(13);
pub const NISTC_AO_CMD2_UC_SAVE_TRACE: u32 = BIT(12);
pub const NISTC_AO_CMD2_BC_GATE_ENA: u32 = BIT(11);
pub const NISTC_AO_CMD2_BC_SAVE_TRACE: u32 = BIT(10);
pub const NISTC_AO_CMD2_UI_SW_ON_BC_TC: u32 = BIT(9);
pub const NISTC_AO_CMD2_UI_SW_ON_STOP: u32 = BIT(8);
pub const NISTC_AO_CMD2_UI_SW_ON_TC: u32 = BIT(7);
pub const NISTC_AO_CMD2_UC_SW_ON_BC_TC: u32 = BIT(6);
pub const NISTC_AO_CMD2_UC_SW_ON_TC: u32 = BIT(5);
pub const NISTC_AO_CMD2_BC_SW_ON_TC: u32 = BIT(4);
pub const NISTC_AO_CMD2_MUTE_B: u32 = BIT(3);
pub const NISTC_AO_CMD2_MUTE_A: u32 = BIT(2);
pub const NISTC_AO_CMD2_UPDATE2_PULSE: u32 = BIT(1);
pub const NISTC_AO_CMD2_START1_PULSE: u32 = BIT(0);

pub const NISTC_G0_CMD_REG: u32 = 6;
pub const NISTC_G1_CMD_REG: u32 = 7;

pub const NISTC_AI_CMD1_REG: u32 = 8;
pub const NISTC_AI_CMD1_ATRIG_RESET: u32 = BIT(14);
pub const NISTC_AI_CMD1_DISARM: u32 = BIT(13);
pub const NISTC_AI_CMD1_SI2_ARM: u32 = BIT(12);
pub const NISTC_AI_CMD1_SI2_LOAD: u32 = BIT(11);
pub const NISTC_AI_CMD1_SI_ARM: u32 = BIT(10);
pub const NISTC_AI_CMD1_SI_LOAD: u32 = BIT(9);
pub const NISTC_AI_CMD1_DIV_ARM: u32 = BIT(8);
pub const NISTC_AI_CMD1_DIV_LOAD: u32 = BIT(7);
pub const NISTC_AI_CMD1_SC_ARM: u32 = BIT(6);
pub const NISTC_AI_CMD1_SC_LOAD: u32 = BIT(5);
pub const NISTC_AI_CMD1_SCAN_IN_PROG_PULSE: u32 = BIT(4);
pub const NISTC_AI_CMD1_EXTMUX_CLK_PULSE: u32 = BIT(3);
pub const NISTC_AI_CMD1_LOCALMUX_CLK_PULSE: u32 = BIT(2);
pub const NISTC_AI_CMD1_SC_TC_PULSE: u32 = BIT(1);
pub const NISTC_AI_CMD1_CONVERT_PULSE: u32 = BIT(0);

pub const NISTC_AO_CMD1_REG: u32 = 9;
pub const NISTC_AO_CMD1_ATRIG_RESET: u32 = BIT(15);
pub const NISTC_AO_CMD1_START_PULSE: u32 = BIT(14);
pub const NISTC_AO_CMD1_DISARM: u32 = BIT(13);
pub const NISTC_AO_CMD1_UI2_ARM_DISARM: u32 = BIT(12);
pub const NISTC_AO_CMD1_UI2_LOAD: u32 = BIT(11);
pub const NISTC_AO_CMD1_UI_ARM: u32 = BIT(10);
pub const NISTC_AO_CMD1_UI_LOAD: u32 = BIT(9);
pub const NISTC_AO_CMD1_UC_ARM: u32 = BIT(8);
pub const NISTC_AO_CMD1_UC_LOAD: u32 = BIT(7);
pub const NISTC_AO_CMD1_BC_ARM: u32 = BIT(6);
pub const NISTC_AO_CMD1_BC_LOAD: u32 = BIT(5);
pub const NISTC_AO_CMD1_DAC1_UPDATE_MODE: u32 = BIT(4);
pub const NISTC_AO_CMD1_LDAC1_SRC_SEL: u32 = BIT(3);
pub const NISTC_AO_CMD1_DAC0_UPDATE_MODE: u32 = BIT(2);
pub const NISTC_AO_CMD1_LDAC0_SRC_SEL: u32 = BIT(1);
pub const NISTC_AO_CMD1_UPDATE_PULSE: u32 = BIT(0);

pub const NISTC_DIO_OUT_REG: u32 = 10;
macro_rules! NISTC_DIO_OUT_SERIAL {
    () => { (x) (((x) & 0xff) << 8) };
}

pub const NISTC_DIO_OUT_SERIAL_MASK: u32 = NISTC_DIO_OUT_SERIAL(0xff);
macro_rules! NISTC_DIO_OUT_PARALLEL {
    () => { (x) ((x) & 0xff) };
}

pub const NISTC_DIO_OUT_PARALLEL_MASK: u32 = NISTC_DIO_OUT_PARALLEL(0xff);
pub const NISTC_DIO_SDIN: u32 = BIT(4);
pub const NISTC_DIO_SDOUT: u32 = BIT(0);

pub const NISTC_DIO_CTRL_REG: u32 = 11;
pub const NISTC_DIO_SDCLK: u32 = BIT(11);
pub const NISTC_DIO_CTRL_HW_SER_TIMEBASE: u32 = BIT(10);
pub const NISTC_DIO_CTRL_HW_SER_ENA: u32 = BIT(9);
pub const NISTC_DIO_CTRL_HW_SER_START: u32 = BIT(8);
macro_rules! NISTC_DIO_CTRL_DIR {
    () => { (x)  ((x) & 0xff) };
}

pub const NISTC_DIO_CTRL_DIR_MASK: u32 = NISTC_DIO_CTRL_DIR(0xff);

pub const NISTC_AI_MODE1_REG: u32 = 12;
macro_rules! NISTC_AI_MODE1_CONVERT_SRC {
    () => { (x) (((x) & 0x1f) << 11) };
}

macro_rules! NISTC_AI_MODE1_SI_SRC {
    () => { (x) (((x) & 0x1f) << 6) };
}

pub const NISTC_AI_MODE1_CONVERT_POLARITY: u32 = BIT(5);
pub const NISTC_AI_MODE1_SI_POLARITY: u32 = BIT(4);
pub const NISTC_AI_MODE1_START_STOP: u32 = BIT(3);
pub const NISTC_AI_MODE1_RSVD: u32 = BIT(2);
pub const NISTC_AI_MODE1_CONTINUOUS: u32 = BIT(1);
pub const NISTC_AI_MODE1_TRIGGER_ONCE: u32 = BIT(0);

pub const NISTC_AI_MODE2_REG: u32 = 13;
pub const NISTC_AI_MODE2_SC_GATE_ENA: u32 = BIT(15);
pub const NISTC_AI_MODE2_START_STOP_GATE_ENA: u32 = BIT(14);
pub const NISTC_AI_MODE2_PRE_TRIGGER: u32 = BIT(13);
pub const NISTC_AI_MODE2_EXTMUX_PRESENT: u32 = BIT(12);
pub const NISTC_AI_MODE2_SI2_INIT_LOAD_SRC: u32 = BIT(9);
pub const NISTC_AI_MODE2_SI2_RELOAD_MODE: u32 = BIT(8);
pub const NISTC_AI_MODE2_SI_INIT_LOAD_SRC: u32 = BIT(7);
macro_rules! NISTC_AI_MODE2_SI_RELOAD_MODE {
    () => { (x) (((x) & 0x7) << 4) };
}

pub const NISTC_AI_MODE2_SI_WR_SWITCH: u32 = BIT(3);
pub const NISTC_AI_MODE2_SC_INIT_LOAD_SRC: u32 = BIT(2);
pub const NISTC_AI_MODE2_SC_RELOAD_MODE: u32 = BIT(1);
pub const NISTC_AI_MODE2_SC_WR_SWITCH: u32 = BIT(0);

pub const NISTC_AI_SI_LOADA_REG: u32 = 14;
pub const NISTC_AI_SI_LOADB_REG: u32 = 16;
pub const NISTC_AI_SC_LOADA_REG: u32 = 18;
pub const NISTC_AI_SC_LOADB_REG: u32 = 20;
pub const NISTC_AI_SI2_LOADA_REG: u32 = 23;
pub const NISTC_AI_SI2_LOADB_REG: u32 = 25;

pub const NISTC_G0_MODE_REG: u32 = 26;
pub const NISTC_G1_MODE_REG: u32 = 27;
pub const NISTC_G0_LOADA_REG: u32 = 28;
pub const NISTC_G0_LOADB_REG: u32 = 30;
pub const NISTC_G1_LOADA_REG: u32 = 32;
pub const NISTC_G1_LOADB_REG: u32 = 34;
pub const NISTC_G0_INPUT_SEL_REG: u32 = 36;
pub const NISTC_G1_INPUT_SEL_REG: u32 = 37;

pub const NISTC_AO_MODE1_REG: u32 = 38;
macro_rules! NISTC_AO_MODE1_UPDATE_SRC {
    () => { (x) (((x) & 0x1f) << 11) };
}

pub const NISTC_AO_MODE1_UPDATE_SRC_MASK: u32 = NISTC_AO_MODE1_UPDATE_SRC(0x1f);
macro_rules! NISTC_AO_MODE1_UI_SRC {
    () => { (x) (((x) & 0x1f) << 6) };
}

pub const NISTC_AO_MODE1_UI_SRC_MASK: u32 = NISTC_AO_MODE1_UI_SRC(0x1f);
pub const NISTC_AO_MODE1_MULTI_CHAN: u32 = BIT(5);
pub const NISTC_AO_MODE1_UPDATE_SRC_POLARITY: u32 = BIT(4);
pub const NISTC_AO_MODE1_UI_SRC_POLARITY: u32 = BIT(3);
pub const NISTC_AO_MODE1_UC_SW_EVERY_TC: u32 = BIT(2);
pub const NISTC_AO_MODE1_CONTINUOUS: u32 = BIT(1);
pub const NISTC_AO_MODE1_TRIGGER_ONCE: u32 = BIT(0);

pub const NISTC_AO_MODE2_REG: u32 = 39;
macro_rules! NISTC_AO_MODE2_FIFO_MODE {
    () => { (x) (((x) & 0x3) << 14) };
}

pub const NISTC_AO_MODE2_FIFO_MODE_MASK: u32 = NISTC_AO_MODE2_FIFO_MODE(3);
pub const NISTC_AO_MODE2_FIFO_MODE_E: u32 = NISTC_AO_MODE2_FIFO_MODE(0);
pub const NISTC_AO_MODE2_FIFO_MODE_HF: u32 = NISTC_AO_MODE2_FIFO_MODE(1);
pub const NISTC_AO_MODE2_FIFO_MODE_F: u32 = NISTC_AO_MODE2_FIFO_MODE(2);
pub const NISTC_AO_MODE2_FIFO_MODE_HF_F: u32 = NISTC_AO_MODE2_FIFO_MODE(3);
pub const NISTC_AO_MODE2_FIFO_REXMIT_ENA: u32 = BIT(13);
pub const NISTC_AO_MODE2_START1_DISABLE: u32 = BIT(12);
pub const NISTC_AO_MODE2_UC_INIT_LOAD_SRC: u32 = BIT(11);
pub const NISTC_AO_MODE2_UC_WR_SWITCH: u32 = BIT(10);
pub const NISTC_AO_MODE2_UI2_INIT_LOAD_SRC: u32 = BIT(9);
pub const NISTC_AO_MODE2_UI2_RELOAD_MODE: u32 = BIT(8);
pub const NISTC_AO_MODE2_UI_INIT_LOAD_SRC: u32 = BIT(7);
macro_rules! NISTC_AO_MODE2_UI_RELOAD_MODE {
    () => { (x) (((x) & 0x7) << 4) };
}

pub const NISTC_AO_MODE2_UI_WR_SWITCH: u32 = BIT(3);
pub const NISTC_AO_MODE2_BC_INIT_LOAD_SRC: u32 = BIT(2);
pub const NISTC_AO_MODE2_BC_RELOAD_MODE: u32 = BIT(1);
pub const NISTC_AO_MODE2_BC_WR_SWITCH: u32 = BIT(0);

pub const NISTC_AO_UI_LOADA_REG: u32 = 40;
pub const NISTC_AO_UI_LOADB_REG: u32 = 42;
pub const NISTC_AO_BC_LOADA_REG: u32 = 44;
pub const NISTC_AO_BC_LOADB_REG: u32 = 46;
pub const NISTC_AO_UC_LOADA_REG: u32 = 48;
pub const NISTC_AO_UC_LOADB_REG: u32 = 50;

pub const NISTC_CLK_FOUT_REG: u32 = 56;
pub const NISTC_CLK_FOUT_ENA: u32 = BIT(15);
pub const NISTC_CLK_FOUT_TIMEBASE_SEL: u32 = BIT(14);
pub const NISTC_CLK_FOUT_DIO_SER_OUT_DIV2: u32 = BIT(13);
pub const NISTC_CLK_FOUT_SLOW_DIV2: u32 = BIT(12);
pub const NISTC_CLK_FOUT_SLOW_TIMEBASE: u32 = BIT(11);
pub const NISTC_CLK_FOUT_G_SRC_DIV2: u32 = BIT(10);
pub const NISTC_CLK_FOUT_TO_BOARD_DIV2: u32 = BIT(9);
pub const NISTC_CLK_FOUT_TO_BOARD: u32 = BIT(8);
pub const NISTC_CLK_FOUT_AI_OUT_DIV2: u32 = BIT(7);
pub const NISTC_CLK_FOUT_AI_SRC_DIV2: u32 = BIT(6);
pub const NISTC_CLK_FOUT_AO_OUT_DIV2: u32 = BIT(5);
pub const NISTC_CLK_FOUT_AO_SRC_DIV2: u32 = BIT(4);
macro_rules! NISTC_CLK_FOUT_DIVIDER {
    () => { (x) (((x) & 0xf) << 0) };
}

macro_rules! NISTC_CLK_FOUT_TO_DIVIDER {
    () => { (x) (((x) >> 0) & 0xf) };
}

pub const NISTC_CLK_FOUT_DIVIDER_MASK: u32 = NISTC_CLK_FOUT_DIVIDER(0xf);

pub const NISTC_IO_BIDIR_PIN_REG: u32 = 57;

pub const NISTC_RTSI_TRIG_DIR_REG: u32 = 58;
pub const NISTC_RTSI_TRIG_OLD_CLK_CHAN: u32 = 7;
macro_rules! NISTC_RTSI_TRIG_NUM_CHAN {
    () => { (_m) ((_m) ? 8 : 7) };
}

macro_rules! NISTC_RTSI_TRIG_DIR {
    () => { (_c, _m) ((_m) ? BIT(8 + (_c)) : BIT(7 + (_c))) };
}

pub const NISTC_RTSI_TRIG_DIR_SUB_SEL1: u32 = BIT(2) /* only for M-Series */;
pub const NISTC_RTSI_TRIG_DIR_SUB_SEL1_SHIFT: u32 = 2 /* only for M-Series */;
pub const NISTC_RTSI_TRIG_USE_CLK: u32 = BIT(1);
pub const NISTC_RTSI_TRIG_DRV_CLK: u32 = BIT(0);

pub const NISTC_INT_CTRL_REG: u32 = 59;
pub const NISTC_INT_CTRL_INTB_ENA: u32 = BIT(15);
macro_rules! NISTC_INT_CTRL_INTB_SEL {
    () => { (x) (((x) & 0x7) << 12) };
}

pub const NISTC_INT_CTRL_INTA_ENA: u32 = BIT(11);
macro_rules! NISTC_INT_CTRL_INTA_SEL {
    () => { (x) (((x) & 0x7) << 8) };
}

pub const NISTC_INT_CTRL_PASSTHRU0_POL: u32 = BIT(3);
pub const NISTC_INT_CTRL_PASSTHRU1_POL: u32 = BIT(2);
pub const NISTC_INT_CTRL_3PIN_INT: u32 = BIT(1);
pub const NISTC_INT_CTRL_INT_POL: u32 = BIT(0);

pub const NISTC_AI_OUT_CTRL_REG: u32 = 60;
pub const NISTC_AI_OUT_CTRL_START_SEL: u32 = BIT(10);
macro_rules! NISTC_AI_OUT_CTRL_SCAN_IN_PROG_SEL {
    () => { (x) (((x) & 0x3) << 8) };
}

macro_rules! NISTC_AI_OUT_CTRL_EXTMUX_CLK_SEL {
    () => { (x) (((x) & 0x3) << 6) };
}

macro_rules! NISTC_AI_OUT_CTRL_LOCALMUX_CLK_SEL {
    () => { (x) (((x) & 0x3) << 4) };
}

macro_rules! NISTC_AI_OUT_CTRL_SC_TC_SEL {
    () => { (x)  (((x) & 0x3) << 2) };
}

macro_rules! NISTC_AI_OUT_CTRL_CONVERT_SEL {
    () => { (x) (((x) & 0x3) << 0) };
}

pub const NISTC_AI_OUT_CTRL_CONVERT_HIGH_Z: u32 = NISTC_AI_OUT_CTRL_CONVERT_SEL(0);
pub const NISTC_AI_OUT_CTRL_CONVERT_GND: u32 = NISTC_AI_OUT_CTRL_CONVERT_SEL(1);
pub const NISTC_AI_OUT_CTRL_CONVERT_LOW: u32 = NISTC_AI_OUT_CTRL_CONVERT_SEL(2);
pub const NISTC_AI_OUT_CTRL_CONVERT_HIGH: u32 = NISTC_AI_OUT_CTRL_CONVERT_SEL(3);

pub const NISTC_ATRIG_ETC_REG: u32 = 61;
pub const NISTC_ATRIG_ETC_GPFO_1_ENA: u32 = BIT(15);
pub const NISTC_ATRIG_ETC_GPFO_0_ENA: u32 = BIT(14);
macro_rules! NISTC_ATRIG_ETC_GPFO_0_SEL {
    () => { (x) (((x) & 0x7) << 11) };
}

macro_rules! NISTC_ATRIG_ETC_GPFO_0_SEL_TO_SRC {
    () => { (x) (((x) >> 11) & 0x7) };
}

pub const NISTC_ATRIG_ETC_GPFO_1_SEL: u32 = BIT(7);
macro_rules! NISTC_ATRIG_ETC_GPFO_1_SEL_TO_SRC {
    () => { (x) (((x) >> 7) & 0x1) };
}

pub const NISTC_ATRIG_ETC_DRV: u32 = BIT(4);
pub const NISTC_ATRIG_ETC_ENA: u32 = BIT(3);
macro_rules! NISTC_ATRIG_ETC_MODE {
    () => { (x)  (((x) & 0x7) << 0) };
}

pub const NISTC_GPFO_0_G_OUT: u32 = 0 /* input to GPFO_0_SEL for Ctr0Out */;
pub const NISTC_GPFO_1_G_OUT: u32 = 0 /* input to GPFO_1_SEL for Ctr1Out */;

pub const NISTC_AI_START_STOP_REG: u32 = 62;
pub const NISTC_AI_START_POLARITY: u32 = BIT(15);
pub const NISTC_AI_STOP_POLARITY: u32 = BIT(14);
pub const NISTC_AI_STOP_SYNC: u32 = BIT(13);
pub const NISTC_AI_STOP_EDGE: u32 = BIT(12);
macro_rules! NISTC_AI_STOP_SEL {
    () => { (x)  (((x) & 0x1f) << 7) };
}

pub const NISTC_AI_START_SYNC: u32 = BIT(6);
pub const NISTC_AI_START_EDGE: u32 = BIT(5);
macro_rules! NISTC_AI_START_SEL {
    () => { (x)  (((x) & 0x1f) << 0) };
}


pub const NISTC_AI_TRIG_SEL_REG: u32 = 63;
pub const NISTC_AI_TRIG_START1_POLARITY: u32 = BIT(15);
pub const NISTC_AI_TRIG_START2_POLARITY: u32 = BIT(14);
pub const NISTC_AI_TRIG_START2_SYNC: u32 = BIT(13);
pub const NISTC_AI_TRIG_START2_EDGE: u32 = BIT(12);
macro_rules! NISTC_AI_TRIG_START2_SEL {
    () => { (x) (((x) & 0x1f) << 7) };
}

pub const NISTC_AI_TRIG_START1_SYNC: u32 = BIT(6);
pub const NISTC_AI_TRIG_START1_EDGE: u32 = BIT(5);
macro_rules! NISTC_AI_TRIG_START1_SEL {
    () => { (x) (((x) & 0x1f) << 0) };
}


pub const NISTC_AI_DIV_LOADA_REG: u32 = 64;

pub const NISTC_AO_START_SEL_REG: u32 = 66;
pub const NISTC_AO_START_UI2_SW_GATE: u32 = BIT(15);
pub const NISTC_AO_START_UI2_EXT_GATE_POL: u32 = BIT(14);
pub const NISTC_AO_START_POLARITY: u32 = BIT(13);
pub const NISTC_AO_START_AOFREQ_ENA: u32 = BIT(12);
macro_rules! NISTC_AO_START_UI2_EXT_GATE_SEL {
    () => { (x) (((x) & 0x1f) << 7) };
}

pub const NISTC_AO_START_SYNC: u32 = BIT(6);
pub const NISTC_AO_START_EDGE: u32 = BIT(5);
macro_rules! NISTC_AO_START_SEL {
    () => { (x)  (((x) & 0x1f) << 0) };
}


pub const NISTC_AO_TRIG_SEL_REG: u32 = 67;
pub const NISTC_AO_TRIG_UI2_EXT_GATE_ENA: u32 = BIT(15);
pub const NISTC_AO_TRIG_DELAYED_START1: u32 = BIT(14);
pub const NISTC_AO_TRIG_START1_POLARITY: u32 = BIT(13);
pub const NISTC_AO_TRIG_UI2_SRC_POLARITY: u32 = BIT(12);
macro_rules! NISTC_AO_TRIG_UI2_SRC_SEL {
    () => { (x) (((x) & 0x1f) << 7) };
}

pub const NISTC_AO_TRIG_START1_SYNC: u32 = BIT(6);
pub const NISTC_AO_TRIG_START1_EDGE: u32 = BIT(5);
macro_rules! NISTC_AO_TRIG_START1_SEL {
    () => { (x) (((x) & 0x1f) << 0) };
}

pub const NISTC_AO_TRIG_START1_SEL_MASK: u32 = NISTC_AO_TRIG_START1_SEL(0x1f);

pub const NISTC_G0_AUTOINC_REG: u32 = 68;
pub const NISTC_G1_AUTOINC_REG: u32 = 69;

pub const NISTC_AO_MODE3_REG: u32 = 70;
pub const NISTC_AO_MODE3_UI2_SW_NEXT_TC: u32 = BIT(13);
pub const NISTC_AO_MODE3_UC_SW_EVERY_BC_TC: u32 = BIT(12);
pub const NISTC_AO_MODE3_TRIG_LEN: u32 = BIT(11);
pub const NISTC_AO_MODE3_STOP_ON_OVERRUN_ERR: u32 = BIT(5);
pub const NISTC_AO_MODE3_STOP_ON_BC_TC_TRIG_ERR: u32 = BIT(4);
pub const NISTC_AO_MODE3_STOP_ON_BC_TC_ERR: u32 = BIT(3);
pub const NISTC_AO_MODE3_NOT_AN_UPDATE: u32 = BIT(2);
pub const NISTC_AO_MODE3_SW_GATE: u32 = BIT(1);
pub const NISTC_AO_MODE3_LAST_GATE_DISABLE: u32 = BIT(0) /* M-Series only */;

pub const NISTC_RESET_REG: u32 = 72;
pub const NISTC_RESET_SOFTWARE: u32 = BIT(11);
pub const NISTC_RESET_AO_CFG_END: u32 = BIT(9);
pub const NISTC_RESET_AI_CFG_END: u32 = BIT(8);
pub const NISTC_RESET_AO_CFG_START: u32 = BIT(5);
pub const NISTC_RESET_AI_CFG_START: u32 = BIT(4);
pub const NISTC_RESET_G1: u32 = BIT(3);
pub const NISTC_RESET_G0: u32 = BIT(2);
pub const NISTC_RESET_AO: u32 = BIT(1);
pub const NISTC_RESET_AI: u32 = BIT(0);

pub const NISTC_INTA_ENA_REG: u32 = 73;
pub const NISTC_INTA2_ENA_REG: u32 = 74;
pub const NISTC_INTA_ENA_PASSTHRU0: u32 = BIT(9);
pub const NISTC_INTA_ENA_G0_GATE: u32 = BIT(8);
pub const NISTC_INTA_ENA_AI_FIFO: u32 = BIT(7);
pub const NISTC_INTA_ENA_G0_TC: u32 = BIT(6);
pub const NISTC_INTA_ENA_AI_ERR: u32 = BIT(5);
pub const NISTC_INTA_ENA_AI_STOP: u32 = BIT(4);
pub const NISTC_INTA_ENA_AI_START: u32 = BIT(3);
pub const NISTC_INTA_ENA_AI_START2: u32 = BIT(2);
pub const NISTC_INTA_ENA_AI_START1: u32 = BIT(1);
pub const NISTC_INTA_ENA_AI_SC_TC: u32 = BIT(0);
pub const NISTC_INTA_ENA_AI_MASK: u32 = (NISTC_INTA_ENA_AI_FIFO |;
					 NISTC_INTA_ENA_AI_ERR |	\
					 NISTC_INTA_ENA_AI_STOP |	\
					 NISTC_INTA_ENA_AI_START |	\
					 NISTC_INTA_ENA_AI_START2 |	\
					 NISTC_INTA_ENA_AI_START1 |	\
					 NISTC_INTA_ENA_AI_SC_TC)

pub const NISTC_INTB_ENA_REG: u32 = 75;
pub const NISTC_INTB2_ENA_REG: u32 = 76;
pub const NISTC_INTB_ENA_PASSTHRU1: u32 = BIT(11);
pub const NISTC_INTB_ENA_G1_GATE: u32 = BIT(10);
pub const NISTC_INTB_ENA_G1_TC: u32 = BIT(9);
pub const NISTC_INTB_ENA_AO_FIFO: u32 = BIT(8);
pub const NISTC_INTB_ENA_AO_UI2_TC: u32 = BIT(7);
pub const NISTC_INTB_ENA_AO_UC_TC: u32 = BIT(6);
pub const NISTC_INTB_ENA_AO_ERR: u32 = BIT(5);
pub const NISTC_INTB_ENA_AO_STOP: u32 = BIT(4);
pub const NISTC_INTB_ENA_AO_START: u32 = BIT(3);
pub const NISTC_INTB_ENA_AO_UPDATE: u32 = BIT(2);
pub const NISTC_INTB_ENA_AO_START1: u32 = BIT(1);
pub const NISTC_INTB_ENA_AO_BC_TC: u32 = BIT(0);

pub const NISTC_AI_PERSONAL_REG: u32 = 77;
pub const NISTC_AI_PERSONAL_SHIFTIN_PW: u32 = BIT(15);
pub const NISTC_AI_PERSONAL_EOC_POLARITY: u32 = BIT(14);
pub const NISTC_AI_PERSONAL_SOC_POLARITY: u32 = BIT(13);
pub const NISTC_AI_PERSONAL_SHIFTIN_POL: u32 = BIT(12);
pub const NISTC_AI_PERSONAL_CONVERT_TIMEBASE: u32 = BIT(11);
pub const NISTC_AI_PERSONAL_CONVERT_PW: u32 = BIT(10);
pub const NISTC_AI_PERSONAL_CONVERT_ORIG_PULSE: u32 = BIT(9);
pub const NISTC_AI_PERSONAL_FIFO_FLAGS_POL: u32 = BIT(8);
pub const NISTC_AI_PERSONAL_OVERRUN_MODE: u32 = BIT(7);
pub const NISTC_AI_PERSONAL_EXTMUX_CLK_PW: u32 = BIT(6);
pub const NISTC_AI_PERSONAL_LOCALMUX_CLK_PW: u32 = BIT(5);
pub const NISTC_AI_PERSONAL_AIFREQ_POL: u32 = BIT(4);

pub const NISTC_AO_PERSONAL_REG: u32 = 78;
pub const NISTC_AO_PERSONAL_MULTI_DACS: u32 = BIT(15) /* M-Series only */;
pub const NISTC_AO_PERSONAL_NUM_DAC: u32 = BIT(14) /* 1:single; 0:dual */;
pub const NISTC_AO_PERSONAL_FAST_CPU: u32 = BIT(13) /* M-Series reserved */;
pub const NISTC_AO_PERSONAL_TMRDACWR_PW: u32 = BIT(12);
pub const NISTC_AO_PERSONAL_FIFO_FLAGS_POL: u32 = BIT(11) /* M-Series reserved */;
pub const NISTC_AO_PERSONAL_FIFO_ENA: u32 = BIT(10);
pub const NISTC_AO_PERSONAL_AOFREQ_POL: u32 = BIT(9) /* M-Series reserved */;
pub const NISTC_AO_PERSONAL_DMA_PIO_CTRL: u32 = BIT(8) /* M-Series reserved */;
pub const NISTC_AO_PERSONAL_UPDATE_ORIG_PULSE: u32 = BIT(7);
pub const NISTC_AO_PERSONAL_UPDATE_TIMEBASE: u32 = BIT(6);
pub const NISTC_AO_PERSONAL_UPDATE_PW: u32 = BIT(5);
pub const NISTC_AO_PERSONAL_BC_SRC_SEL: u32 = BIT(4);
pub const NISTC_AO_PERSONAL_INTERVAL_BUFFER_MODE: u32 = BIT(3);

pub const NISTC_RTSI_TRIGA_OUT_REG: u32 = 79;
pub const NISTC_RTSI_TRIGB_OUT_REG: u32 = 80;
pub const NISTC_RTSI_TRIGB_SUB_SEL1: u32 = BIT(15) /* not for M-Series */;
pub const NISTC_RTSI_TRIGB_SUB_SEL1_SHIFT: u32 = 15 /* not for M-Series */;
macro_rules! NISTC_RTSI_TRIG {
    () => { (_c, _s)  (((_s) & 0xf) << (((_c) % 4) * 4)) };
}

macro_rules! NISTC_RTSI_TRIG_MASK {
    () => { (_c) NISTC_RTSI_TRIG((_c), 0xf) };
}

macro_rules! NISTC_RTSI_TRIG_TO_SRC {
    () => { (_c, _b) (((_b) >> (((_c) % 4) * 4)) & 0xf) };
}


pub const NISTC_RTSI_BOARD_REG: u32 = 81;

pub const NISTC_CFG_MEM_CLR_REG: u32 = 82;
pub const NISTC_ADC_FIFO_CLR_REG: u32 = 83;
pub const NISTC_DAC_FIFO_CLR_REG: u32 = 84;
pub const NISTC_WR_STROBE3_REG: u32 = 85;

pub const NISTC_AO_OUT_CTRL_REG: u32 = 86;
pub const NISTC_AO_OUT_CTRL_EXT_GATE_ENA: u32 = BIT(15);
macro_rules! NISTC_AO_OUT_CTRL_EXT_GATE_SEL {
    () => { (x) (((x) & 0x1f) << 10) };
}

macro_rules! NISTC_AO_OUT_CTRL_CHANS {
    () => { (x)  (((x) & 0xf) << 6) };
}

macro_rules! NISTC_AO_OUT_CTRL_UPDATE2_SEL {
    () => { (x) (((x) & 0x3) << 4) };
}

pub const NISTC_AO_OUT_CTRL_EXT_GATE_POL: u32 = BIT(3);
pub const NISTC_AO_OUT_CTRL_UPDATE2_TOGGLE: u32 = BIT(2);
macro_rules! NISTC_AO_OUT_CTRL_UPDATE_SEL {
    () => { (x)  (((x) & 0x3) << 0) };
}

pub const NISTC_AO_OUT_CTRL_UPDATE_SEL_HIGHZ: u32 = NISTC_AO_OUT_CTRL_UPDATE_SEL(0);
pub const NISTC_AO_OUT_CTRL_UPDATE_SEL_GND: u32 = NISTC_AO_OUT_CTRL_UPDATE_SEL(1);
pub const NISTC_AO_OUT_CTRL_UPDATE_SEL_LOW: u32 = NISTC_AO_OUT_CTRL_UPDATE_SEL(2);
pub const NISTC_AO_OUT_CTRL_UPDATE_SEL_HIGH: u32 = NISTC_AO_OUT_CTRL_UPDATE_SEL(3);

pub const NISTC_AI_MODE3_REG: u32 = 87;
pub const NISTC_AI_MODE3_TRIG_LEN: u32 = BIT(15);
pub const NISTC_AI_MODE3_DELAY_START: u32 = BIT(14);
pub const NISTC_AI_MODE3_SOFTWARE_GATE: u32 = BIT(13);
pub const NISTC_AI_MODE3_SI_TRIG_DELAY: u32 = BIT(12);
pub const NISTC_AI_MODE3_SI2_SRC_SEL: u32 = BIT(11);
pub const NISTC_AI_MODE3_DELAYED_START2: u32 = BIT(10);
pub const NISTC_AI_MODE3_DELAYED_START1: u32 = BIT(9);
pub const NISTC_AI_MODE3_EXT_GATE_MODE: u32 = BIT(8);
macro_rules! NISTC_AI_MODE3_FIFO_MODE {
    () => { (x) (((x) & 0x3) << 6) };
}

pub const NISTC_AI_MODE3_FIFO_MODE_NE: u32 = NISTC_AI_MODE3_FIFO_MODE(0);
pub const NISTC_AI_MODE3_FIFO_MODE_HF: u32 = NISTC_AI_MODE3_FIFO_MODE(1);
pub const NISTC_AI_MODE3_FIFO_MODE_F: u32 = NISTC_AI_MODE3_FIFO_MODE(2);
pub const NISTC_AI_MODE3_FIFO_MODE_HF_E: u32 = NISTC_AI_MODE3_FIFO_MODE(3);
pub const NISTC_AI_MODE3_EXT_GATE_POL: u32 = BIT(5);
macro_rules! NISTC_AI_MODE3_EXT_GATE_SEL {
    () => { (x) (((x) & 0x1f) << 0) };
}


pub const NISTC_AI_STATUS1_REG: u32 = 2;
pub const NISTC_AI_STATUS1_INTA: u32 = BIT(15);
pub const NISTC_AI_STATUS1_FIFO_F: u32 = BIT(14);
pub const NISTC_AI_STATUS1_FIFO_HF: u32 = BIT(13);
pub const NISTC_AI_STATUS1_FIFO_E: u32 = BIT(12);
pub const NISTC_AI_STATUS1_OVERRUN: u32 = BIT(11);
pub const NISTC_AI_STATUS1_OVERFLOW: u32 = BIT(10);
pub const NISTC_AI_STATUS1_SC_TC_ERR: u32 = BIT(9);
pub const NISTC_AI_STATUS1_OVER: u32 = (NISTC_AI_STATUS1_OVERRUN |;
					 NISTC_AI_STATUS1_OVERFLOW)
pub const NISTC_AI_STATUS1_ERR: u32 = (NISTC_AI_STATUS1_OVER |;
					 NISTC_AI_STATUS1_SC_TC_ERR)
pub const NISTC_AI_STATUS1_START2: u32 = BIT(8);
pub const NISTC_AI_STATUS1_START1: u32 = BIT(7);
pub const NISTC_AI_STATUS1_SC_TC: u32 = BIT(6);
pub const NISTC_AI_STATUS1_START: u32 = BIT(5);
pub const NISTC_AI_STATUS1_STOP: u32 = BIT(4);
pub const NISTC_AI_STATUS1_G0_TC: u32 = BIT(3);
pub const NISTC_AI_STATUS1_G0_GATE: u32 = BIT(2);
pub const NISTC_AI_STATUS1_FIFO_REQ: u32 = BIT(1);
pub const NISTC_AI_STATUS1_PASSTHRU0: u32 = BIT(0);

pub const NISTC_AO_STATUS1_REG: u32 = 3;
pub const NISTC_AO_STATUS1_INTB: u32 = BIT(15);
pub const NISTC_AO_STATUS1_FIFO_F: u32 = BIT(14);
pub const NISTC_AO_STATUS1_FIFO_HF: u32 = BIT(13);
pub const NISTC_AO_STATUS1_FIFO_E: u32 = BIT(12);
pub const NISTC_AO_STATUS1_BC_TC_ERR: u32 = BIT(11);
pub const NISTC_AO_STATUS1_START: u32 = BIT(10);
pub const NISTC_AO_STATUS1_OVERRUN: u32 = BIT(9);
pub const NISTC_AO_STATUS1_START1: u32 = BIT(8);
pub const NISTC_AO_STATUS1_BC_TC: u32 = BIT(7);
pub const NISTC_AO_STATUS1_UC_TC: u32 = BIT(6);
pub const NISTC_AO_STATUS1_UPDATE: u32 = BIT(5);
pub const NISTC_AO_STATUS1_UI2_TC: u32 = BIT(4);
pub const NISTC_AO_STATUS1_G1_TC: u32 = BIT(3);
pub const NISTC_AO_STATUS1_G1_GATE: u32 = BIT(2);
pub const NISTC_AO_STATUS1_FIFO_REQ: u32 = BIT(1);
pub const NISTC_AO_STATUS1_PASSTHRU1: u32 = BIT(0);

pub const NISTC_G01_STATUS_REG: u32 = 4;

pub const NISTC_AI_STATUS2_REG: u32 = 5;

pub const NISTC_AO_STATUS2_REG: u32 = 6;

pub const NISTC_DIO_IN_REG: u32 = 7;

pub const NISTC_G0_HW_SAVE_REG: u32 = 8;
pub const NISTC_G1_HW_SAVE_REG: u32 = 10;

pub const NISTC_G0_SAVE_REG: u32 = 12;
pub const NISTC_G1_SAVE_REG: u32 = 14;

pub const NISTC_AO_UI_SAVE_REG: u32 = 16;
pub const NISTC_AO_BC_SAVE_REG: u32 = 18;
pub const NISTC_AO_UC_SAVE_REG: u32 = 20;

pub const NISTC_STATUS1_REG: u32 = 27;
pub const NISTC_STATUS1_SERIO_IN_PROG: u32 = BIT(12);

pub const NISTC_DIO_SERIAL_IN_REG: u32 = 28;

pub const NISTC_STATUS2_REG: u32 = 29;
pub const NISTC_STATUS2_AO_TMRDACWRS_IN_PROGRESS: u32 = BIT(5);

pub const NISTC_AI_SI_SAVE_REG: u32 = 64;
pub const NISTC_AI_SC_SAVE_REG: u32 = 66;

/*
 * PCI E Series Registers
 */
pub const NI_E_STC_WINDOW_ADDR_REG: u32 = 0x00 /* rw16 */;
pub const NI_E_STC_WINDOW_DATA_REG: u32 = 0x02 /* rw16 */;

pub const NI_E_STATUS_REG: u32 = 0x01 /* r8 */;
pub const NI_E_STATUS_AI_FIFO_LOWER_NE: u32 = BIT(3);
pub const NI_E_STATUS_PROMOUT: u32 = BIT(0);

pub const NI_E_DMA_AI_AO_SEL_REG: u32 = 0x09 /* w8 */;
macro_rules! NI_E_DMA_AI_SEL {
    () => { (x)  (((x) & 0xf) << 0) };
}

pub const NI_E_DMA_AI_SEL_MASK: u32 = NI_E_DMA_AI_SEL(0xf);
macro_rules! NI_E_DMA_AO_SEL {
    () => { (x)  (((x) & 0xf) << 4) };
}

pub const NI_E_DMA_AO_SEL_MASK: u32 = NI_E_DMA_AO_SEL(0xf);

pub const NI_E_DMA_G0_G1_SEL_REG: u32 = 0x0b /* w8 */;
macro_rules! NI_E_DMA_G0_G1_SEL {
    () => { (_g, _c) (((_c) & 0xf) << ((_g) * 4)) };
}

macro_rules! NI_E_DMA_G0_G1_SEL_MASK {
    () => { (_g) NI_E_DMA_G0_G1_SEL((_g), 0xf) };
}


pub const NI_E_SERIAL_CMD_REG: u32 = 0x0d /* w8 */;
macro_rules! NI_E_SERIAL_CMD_DAC_LD {
    () => { (x) BIT(3 + (x)) };
}

pub const NI_E_SERIAL_CMD_EEPROM_CS: u32 = BIT(2);
pub const NI_E_SERIAL_CMD_SDATA: u32 = BIT(1);
pub const NI_E_SERIAL_CMD_SCLK: u32 = BIT(0);

pub const NI_E_MISC_CMD_REG: u32 = 0x0f /* w8 */;
macro_rules! NI_E_MISC_CMD_INTEXT_ATRIG {
    () => { (x) (((x) & 0x1) << 7) };
}

pub const NI_E_MISC_CMD_EXT_ATRIG: u32 = NI_E_MISC_CMD_INTEXT_ATRIG(0);
pub const NI_E_MISC_CMD_INT_ATRIG: u32 = NI_E_MISC_CMD_INTEXT_ATRIG(1);

pub const NI_E_AI_CFG_LO_REG: u32 = 0x10 /* w16 */;
pub const NI_E_AI_CFG_LO_LAST_CHAN: u32 = BIT(15);
pub const NI_E_AI_CFG_LO_GEN_TRIG: u32 = BIT(12);
pub const NI_E_AI_CFG_LO_DITHER: u32 = BIT(9);
pub const NI_E_AI_CFG_LO_UNI: u32 = BIT(8);
macro_rules! NI_E_AI_CFG_LO_GAIN {
    () => { (x)  ((x) << 0) };
}


pub const NI_E_AI_CFG_HI_REG: u32 = 0x12 /* w16 */;
macro_rules! NI_E_AI_CFG_HI_TYPE {
    () => { (x)  (((x) & 0x7) << 12) };
}

pub const NI_E_AI_CFG_HI_TYPE_DIFF: u32 = NI_E_AI_CFG_HI_TYPE(1);
pub const NI_E_AI_CFG_HI_TYPE_COMMON: u32 = NI_E_AI_CFG_HI_TYPE(2);
pub const NI_E_AI_CFG_HI_TYPE_GROUND: u32 = NI_E_AI_CFG_HI_TYPE(3);
pub const NI_E_AI_CFG_HI_AC_COUPLE: u32 = BIT(11);
macro_rules! NI_E_AI_CFG_HI_CHAN {
    () => { (x)  (((x) & 0x3f) << 0) };
}


pub const NI_E_AO_CFG_REG: u32 = 0x16 /* w16 */;
macro_rules! NI_E_AO_DACSEL {
    () => { (x)  ((x) << 8) };
}

pub const NI_E_AO_GROUND_REF: u32 = BIT(3);
pub const NI_E_AO_EXT_REF: u32 = BIT(2);
pub const NI_E_AO_DEGLITCH: u32 = BIT(1);
pub const NI_E_AO_CFG_BIP: u32 = BIT(0);

macro_rules! NI_E_DAC_DIRECT_DATA_REG {
    () => { (x) (0x18 + ((x) * 2)) /* w16 */ };
}


pub const NI_E_8255_BASE: u32 = 0x19 /* rw8 */;

pub const NI_E_AI_FIFO_DATA_REG: u32 = 0x1c /* r16 */;

pub const NI_E_AO_FIFO_DATA_REG: u32 = 0x1e /* w16 */;

/*
 * 611x registers (these boards differ from the e-series)
 */
pub const NI611X_MAGIC_REG: u32 = 0x19 /* w8 (new) */;
pub const NI611X_CALIB_CHAN_SEL_REG: u32 = 0x1a /* w16 (new) */;
pub const NI611X_AI_FIFO_DATA_REG: u32 = 0x1c /* r32 (incompatible) */;
pub const NI611X_AI_FIFO_OFFSET_LOAD_REG: u32 = 0x05 /* r8 (new) */;
pub const NI611X_AO_FIFO_DATA_REG: u32 = 0x14 /* w32 (incompatible) */;
pub const NI611X_CAL_GAIN_SEL_REG: u32 = 0x05 /* w8 (new) */;

pub const NI611X_AO_WINDOW_ADDR_REG: u32 = 0x18;
pub const NI611X_AO_WINDOW_DATA_REG: u32 = 0x1e;

/*
 * 6143 registers
 */
pub const NI6143_MAGIC_REG: u32 = 0x19 /* w8 */;
pub const NI6143_DMA_G0_G1_SEL_REG: u32 = 0x0b /* w8 */;
pub const NI6143_PIPELINE_DELAY_REG: u32 = 0x1f /* w8 */;
pub const NI6143_EOC_SET_REG: u32 = 0x1d /* w8 */;
pub const NI6143_DMA_AI_SEL_REG: u32 = 0x09 /* w8 */;
pub const NI6143_AI_FIFO_DATA_REG: u32 = 0x8c /* r32 */;
pub const NI6143_AI_FIFO_FLAG_REG: u32 = 0x84 /* w32 */;
pub const NI6143_AI_FIFO_CTRL_REG: u32 = 0x88 /* w32 */;
pub const NI6143_AI_FIFO_STATUS_REG: u32 = 0x88 /* r32 */;
pub const NI6143_AI_FIFO_DMA_THRESH_REG: u32 = 0x90 /* w32 */;
pub const NI6143_AI_FIFO_WORDS_AVAIL_REG: u32 = 0x94 /* w32 */;

pub const NI6143_CALIB_CHAN_REG: u32 = 0x42 /* w16 */;
pub const NI6143_CALIB_CHAN_RELAY_ON: u32 = BIT(15);
pub const NI6143_CALIB_CHAN_RELAY_OFF: u32 = BIT(14);
macro_rules! NI6143_CALIB_CHAN {
    () => { (x)  (((x) & 0xf) << 0) };
}

pub const NI6143_CALIB_CHAN_GND_GND: u32 = NI6143_CALIB_CHAN(0) /* Offset Cal */;
pub const NI6143_CALIB_CHAN_2V5_GND: u32 = NI6143_CALIB_CHAN(2) /* 2.5V ref */;
pub const NI6143_CALIB_CHAN_PWM_GND: u32 = NI6143_CALIB_CHAN(5) /* +-5V Self Cal */;
pub const NI6143_CALIB_CHAN_2V5_PWM: u32 = NI6143_CALIB_CHAN(10) /* PWM Cal */;
pub const NI6143_CALIB_CHAN_PWM_PWM: u32 = NI6143_CALIB_CHAN(13) /* CMRR */;
pub const NI6143_CALIB_CHAN_GND_PWM: u32 = NI6143_CALIB_CHAN(14) /* PWM Cal */;
pub const NI6143_CALIB_LO_TIME_REG: u32 = 0x20 /* w16 */;
pub const NI6143_CALIB_HI_TIME_REG: u32 = 0x22 /* w16 */;
pub const NI6143_RELAY_COUNTER_LOAD_REG: u32 = 0x4c /* w32 */;
pub const NI6143_SIGNATURE_REG: u32 = 0x50 /* w32 */;
pub const NI6143_RELEASE_DATE_REG: u32 = 0x54 /* w32 */;
pub const NI6143_RELEASE_OLDEST_DATE_REG: u32 = 0x58 /* w32 */;

/*
 * 671x, 611x windowed ao registers
 */
macro_rules! NI671X_DAC_DIRECT_DATA_REG {
    () => { (x) (0x00 + (x)) /* w16 */ };
}

pub const NI611X_AO_TIMED_REG: u32 = 0x10 /* w16 */;
pub const NI671X_AO_IMMEDIATE_REG: u32 = 0x11 /* w16 */;
pub const NI611X_AO_FIFO_OFFSET_LOAD_REG: u32 = 0x13 /* w32 */;
pub const NI67XX_AO_SP_UPDATES_REG: u32 = 0x14 /* w16 */;
pub const NI611X_AO_WAVEFORM_GEN_REG: u32 = 0x15 /* w16 */;
pub const NI611X_AO_MISC_REG: u32 = 0x16 /* w16 */;
pub const NI611X_AO_MISC_CLEAR_WG: u32 = BIT(0);
pub const NI67XX_AO_CAL_CHAN_SEL_REG: u32 = 0x17 /* w16 */;
pub const NI67XX_AO_CFG2_REG: u32 = 0x18 /* w16 */;
pub const NI67XX_CAL_CMD_REG: u32 = 0x19 /* w16 */;
pub const NI67XX_CAL_STATUS_REG: u32 = 0x1a /* r8 */;
pub const NI67XX_CAL_STATUS_BUSY: u32 = BIT(0);
pub const NI67XX_CAL_STATUS_OSC_DETECT: u32 = BIT(1);
pub const NI67XX_CAL_STATUS_OVERRANGE: u32 = BIT(2);
pub const NI67XX_CAL_DATA_REG: u32 = 0x1b /* r16 */;
pub const NI67XX_CAL_CFG_HI_REG: u32 = 0x1c /* rw16 */;
pub const NI67XX_CAL_CFG_LO_REG: u32 = 0x1d /* rw16 */;

pub const CS5529_CMD_CB: u32 = BIT(7);
pub const CS5529_CMD_SINGLE_CONV: u32 = BIT(6);
pub const CS5529_CMD_CONT_CONV: u32 = BIT(5);
pub const CS5529_CMD_READ: u32 = BIT(4);
macro_rules! CS5529_CMD_REG {
    () => { (x)  (((x) & 0x7) << 1) };
}

pub const CS5529_CMD_REG_MASK: u32 = CS5529_CMD_REG(7);
pub const CS5529_CMD_PWR_SAVE: u32 = BIT(0);

pub const CS5529_OFFSET_REG: u32 = CS5529_CMD_REG(0);
pub const CS5529_GAIN_REG: u32 = CS5529_CMD_REG(1);
pub const CS5529_CONV_DATA_REG: u32 = CS5529_CMD_REG(3);
pub const CS5529_SETUP_REG: u32 = CS5529_CMD_REG(4);

pub const CS5529_CFG_REG: u32 = CS5529_CMD_REG(2);
macro_rules! CS5529_CFG_AOUT {
    () => { (x)  BIT(22 + (x)) };
}

macro_rules! CS5529_CFG_DOUT {
    () => { (x)  BIT(18 + (x)) };
}

pub const CS5529_CFG_LOW_PWR_MODE: u32 = BIT(16);
macro_rules! CS5529_CFG_WORD_RATE {
    () => { (x)  (((x) & 0x7) << 13) };
}

pub const CS5529_CFG_WORD_RATE_MASK: u32 = CS5529_CFG_WORD_RATE(0x7);
pub const CS5529_CFG_WORD_RATE_2180: u32 = CS5529_CFG_WORD_RATE(0);
pub const CS5529_CFG_WORD_RATE_1092: u32 = CS5529_CFG_WORD_RATE(1);
pub const CS5529_CFG_WORD_RATE_532: u32 = CS5529_CFG_WORD_RATE(2);
pub const CS5529_CFG_WORD_RATE_388: u32 = CS5529_CFG_WORD_RATE(3);
pub const CS5529_CFG_WORD_RATE_324: u32 = CS5529_CFG_WORD_RATE(4);
pub const CS5529_CFG_WORD_RATE_17444: u32 = CS5529_CFG_WORD_RATE(5);
pub const CS5529_CFG_WORD_RATE_8724: u32 = CS5529_CFG_WORD_RATE(6);
pub const CS5529_CFG_WORD_RATE_4364: u32 = CS5529_CFG_WORD_RATE(7);
pub const CS5529_CFG_UNIPOLAR: u32 = BIT(12);
pub const CS5529_CFG_RESET: u32 = BIT(7);
pub const CS5529_CFG_RESET_VALID: u32 = BIT(6);
pub const CS5529_CFG_PORT_FLAG: u32 = BIT(5);
pub const CS5529_CFG_PWR_SAVE_SEL: u32 = BIT(4);
pub const CS5529_CFG_DONE_FLAG: u32 = BIT(3);
macro_rules! CS5529_CFG_CALIB {
    () => { (x)  (((x) & 0x7) << 0) };
}

pub const CS5529_CFG_CALIB_NONE: u32 = CS5529_CFG_CALIB(0);
pub const CS5529_CFG_CALIB_OFFSET_SELF: u32 = CS5529_CFG_CALIB(1);
pub const CS5529_CFG_CALIB_GAIN_SELF: u32 = CS5529_CFG_CALIB(2);
pub const CS5529_CFG_CALIB_BOTH_SELF: u32 = CS5529_CFG_CALIB(3);
pub const CS5529_CFG_CALIB_OFFSET_SYS: u32 = CS5529_CFG_CALIB(5);
pub const CS5529_CFG_CALIB_GAIN_SYS: u32 = CS5529_CFG_CALIB(6);

/*
 * M-Series specific registers not handled by the DAQ-STC and GPCT register
 * remapping.
 */
pub const NI_M_CDIO_DMA_SEL_REG: u32 = 0x007;
macro_rules! NI_M_CDIO_DMA_SEL_CDO {
    () => { (x) (((x) & 0xf) << 4) };
}

pub const NI_M_CDIO_DMA_SEL_CDO_MASK: u32 = NI_M_CDIO_DMA_SEL_CDO(0xf);
macro_rules! NI_M_CDIO_DMA_SEL_CDI {
    () => { (x) (((x) & 0xf) << 0) };
}

pub const NI_M_CDIO_DMA_SEL_CDI_MASK: u32 = NI_M_CDIO_DMA_SEL_CDI(0xf);
pub const NI_M_SCXI_STATUS_REG: u32 = 0x007;
pub const NI_M_AI_AO_SEL_REG: u32 = 0x009;
pub const NI_M_G0_G1_SEL_REG: u32 = 0x00b;
pub const NI_M_MISC_CMD_REG: u32 = 0x00f;
pub const NI_M_SCXI_SER_DO_REG: u32 = 0x011;
pub const NI_M_SCXI_CTRL_REG: u32 = 0x013;
pub const NI_M_SCXI_OUT_ENA_REG: u32 = 0x015;
pub const NI_M_AI_FIFO_DATA_REG: u32 = 0x01c;
pub const NI_M_DIO_REG: u32 = 0x024;
pub const NI_M_DIO_DIR_REG: u32 = 0x028;
pub const NI_M_CAL_PWM_REG: u32 = 0x040;
macro_rules! NI_M_CAL_PWM_HIGH_TIME {
    () => { (x) (((x) & 0xffff) << 16) };
}

macro_rules! NI_M_CAL_PWM_LOW_TIME {
    () => { (x) (((x) & 0xffff) << 0) };
}

macro_rules! NI_M_GEN_PWM_REG {
    () => { (x)  (0x044 + ((x) * 2)) };
}

pub const NI_M_AI_CFG_FIFO_DATA_REG: u32 = 0x05e;
pub const NI_M_AI_CFG_LAST_CHAN: u32 = BIT(14);
pub const NI_M_AI_CFG_DITHER: u32 = BIT(13);
pub const NI_M_AI_CFG_POLARITY: u32 = BIT(12);
macro_rules! NI_M_AI_CFG_GAIN {
    () => { (x)  (((x) & 0x7) << 9) };
}

macro_rules! NI_M_AI_CFG_CHAN_TYPE {
    () => { (x) (((x) & 0x7) << 6) };
}

pub const NI_M_AI_CFG_CHAN_TYPE_MASK: u32 = NI_M_AI_CFG_CHAN_TYPE(7);
pub const NI_M_AI_CFG_CHAN_TYPE_CALIB: u32 = NI_M_AI_CFG_CHAN_TYPE(0);
pub const NI_M_AI_CFG_CHAN_TYPE_DIFF: u32 = NI_M_AI_CFG_CHAN_TYPE(1);
pub const NI_M_AI_CFG_CHAN_TYPE_COMMON: u32 = NI_M_AI_CFG_CHAN_TYPE(2);
pub const NI_M_AI_CFG_CHAN_TYPE_GROUND: u32 = NI_M_AI_CFG_CHAN_TYPE(3);
pub const NI_M_AI_CFG_CHAN_TYPE_AUX: u32 = NI_M_AI_CFG_CHAN_TYPE(5);
pub const NI_M_AI_CFG_CHAN_TYPE_GHOST: u32 = NI_M_AI_CFG_CHAN_TYPE(7);
macro_rules! NI_M_AI_CFG_BANK_SEL {
    () => { (x)  ((((x) & 0x40) << 4) | ((x) & 0x30)) };
}

macro_rules! NI_M_AI_CFG_CHAN_SEL {
    () => { (x)  (((x) & 0xf) << 0) };
}

pub const NI_M_INTC_ENA_REG: u32 = 0x088;
pub const NI_M_INTC_ENA: u32 = BIT(0);
pub const NI_M_INTC_STATUS_REG: u32 = 0x088;
pub const NI_M_INTC_STATUS: u32 = BIT(0);
pub const NI_M_ATRIG_CTRL_REG: u32 = 0x08c;
pub const NI_M_AO_SER_INT_ENA_REG: u32 = 0x0a0;
pub const NI_M_AO_SER_INT_ACK_REG: u32 = 0x0a1;
pub const NI_M_AO_SER_INT_STATUS_REG: u32 = 0x0a1;
pub const NI_M_AO_CALIB_REG: u32 = 0x0a3;
pub const NI_M_AO_FIFO_DATA_REG: u32 = 0x0a4;
pub const NI_M_PFI_FILTER_REG: u32 = 0x0b0;
macro_rules! NI_M_PFI_FILTER_SEL {
    () => { (_c, _f) (((_f) & 0x3) << ((_c) * 2)) };
}

macro_rules! NI_M_PFI_FILTER_SEL_MASK {
    () => { (_c) NI_M_PFI_FILTER_SEL((_c), 0x3) };
}

pub const NI_M_RTSI_FILTER_REG: u32 = 0x0b4;
pub const NI_M_SCXI_LEGACY_COMPAT_REG: u32 = 0x0bc;
macro_rules! NI_M_DAC_DIRECT_DATA_REG {
    () => { (x) (0x0c0 + ((x) * 4)) };
}

macro_rules! NI_M_AO_WAVEFORM_ORDER_REG {
    () => { (x) (0x0c2 + ((x) * 4)) };
}

macro_rules! NI_M_AO_CFG_BANK_REG {
    () => { (x)  (0x0c3 + ((x) * 4)) };
}

pub const NI_M_AO_CFG_BANK_BIPOLAR: u32 = BIT(7);
pub const NI_M_AO_CFG_BANK_UPDATE_TIMED: u32 = BIT(6);
macro_rules! NI_M_AO_CFG_BANK_REF {
    () => { (x)  (((x) & 0x7) << 3) };
}

pub const NI_M_AO_CFG_BANK_REF_MASK: u32 = NI_M_AO_CFG_BANK_REF(7);
pub const NI_M_AO_CFG_BANK_REF_INT_10V: u32 = NI_M_AO_CFG_BANK_REF(0);
pub const NI_M_AO_CFG_BANK_REF_INT_5V: u32 = NI_M_AO_CFG_BANK_REF(1);
macro_rules! NI_M_AO_CFG_BANK_OFFSET {
    () => { (x) (((x) & 0x7) << 0) };
}

pub const NI_M_AO_CFG_BANK_OFFSET_MASK: u32 = NI_M_AO_CFG_BANK_OFFSET(7);
pub const NI_M_AO_CFG_BANK_OFFSET_0V: u32 = NI_M_AO_CFG_BANK_OFFSET(0);
pub const NI_M_AO_CFG_BANK_OFFSET_5V: u32 = NI_M_AO_CFG_BANK_OFFSET(1);
pub const NI_M_RTSI_SHARED_MUX_REG: u32 = 0x1a2;
pub const NI_M_CLK_FOUT2_REG: u32 = 0x1c4;
pub const NI_M_CLK_FOUT2_RTSI_10MHZ: u32 = BIT(7);
pub const NI_M_CLK_FOUT2_TIMEBASE3_PLL: u32 = BIT(6);
pub const NI_M_CLK_FOUT2_TIMEBASE1_PLL: u32 = BIT(5);
macro_rules! NI_M_CLK_FOUT2_PLL_SRC {
    () => { (x) (((x) & 0x1f) << 0) };
}

pub const NI_M_CLK_FOUT2_PLL_SRC_MASK: u32 = NI_M_CLK_FOUT2_PLL_SRC(0x1f);
pub const NI_M_MAX_RTSI_CHAN: u32 = 7;
macro_rules! NI_M_CLK_FOUT2_PLL_SRC_RTSI {
    () => { (x) (((x) == NI_M_MAX_RTSI_CHAN) };
}

					 ? NI_M_CLK_FOUT2_PLL_SRC(0x1b)	\
					 : NI_M_CLK_FOUT2_PLL_SRC(0xb + (x)))
pub const NI_M_CLK_FOUT2_PLL_SRC_STAR: u32 = NI_M_CLK_FOUT2_PLL_SRC(0x14);
pub const NI_M_CLK_FOUT2_PLL_SRC_PXI10: u32 = NI_M_CLK_FOUT2_PLL_SRC(0x1d);
pub const NI_M_PLL_CTRL_REG: u32 = 0x1c6;
macro_rules! NI_M_PLL_CTRL_VCO_MODE {
    () => { (x) (((x) & 0x3) << 13) };
}

pub const NI_M_PLL_CTRL_VCO_MODE_200_325MHZ: u32 = NI_M_PLL_CTRL_VCO_MODE(0);
pub const NI_M_PLL_CTRL_VCO_MODE_175_225MHZ: u32 = NI_M_PLL_CTRL_VCO_MODE(1);
pub const NI_M_PLL_CTRL_VCO_MODE_100_225MHZ: u32 = NI_M_PLL_CTRL_VCO_MODE(2);
pub const NI_M_PLL_CTRL_VCO_MODE_75_150MHZ: u32 = NI_M_PLL_CTRL_VCO_MODE(3);
pub const NI_M_PLL_CTRL_ENA: u32 = BIT(12);
pub const NI_M_PLL_MAX_DIVISOR: u32 = 0x10;
macro_rules! NI_M_PLL_CTRL_DIVISOR {
    () => { (x) (((x) & 0xf) << 8) };
}

pub const NI_M_PLL_MAX_MULTIPLIER: u32 = 0x100;
macro_rules! NI_M_PLL_CTRL_MULTIPLIER {
    () => { (x) (((x) & 0xff) << 0) };
}

pub const NI_M_PLL_STATUS_REG: u32 = 0x1c8;
pub const NI_M_PLL_STATUS_LOCKED: u32 = BIT(0);
macro_rules! NI_M_PFI_OUT_SEL_REG {
    () => { (x)  (0x1d0 + ((x) * 2)) };
}

macro_rules! NI_M_PFI_CHAN {
    () => { (_c)  (((_c) % 3) * 5) };
}

macro_rules! NI_M_PFI_OUT_SEL {
    () => { (_c, _s) (((_s) & 0x1f) << NI_M_PFI_CHAN(_c)) };
}

macro_rules! NI_M_PFI_OUT_SEL_MASK {
    () => { (_c) (0x1f << NI_M_PFI_CHAN(_c)) };
}

macro_rules! NI_M_PFI_OUT_SEL_TO_SRC {
    () => { (_c, _b) (((_b) >> NI_M_PFI_CHAN(_c)) & 0x1f) };
}

pub const NI_M_PFI_DI_REG: u32 = 0x1dc;
pub const NI_M_PFI_DO_REG: u32 = 0x1de;
pub const NI_M_CFG_BYPASS_FIFO_REG: u32 = 0x218;
pub const NI_M_CFG_BYPASS_FIFO: u32 = BIT(31);
pub const NI_M_CFG_BYPASS_AI_POLARITY: u32 = BIT(22);
pub const NI_M_CFG_BYPASS_AI_DITHER: u32 = BIT(21);
macro_rules! NI_M_CFG_BYPASS_AI_GAIN {
    () => { (x) (((x) & 0x7) << 18) };
}

macro_rules! NI_M_CFG_BYPASS_AO_CAL {
    () => { (x) (((x) & 0xf) << 15) };
}

pub const NI_M_CFG_BYPASS_AO_CAL_MASK: u32 = NI_M_CFG_BYPASS_AO_CAL(0xf);
macro_rules! NI_M_CFG_BYPASS_AI_MODE_MUX {
    () => { (x) (((x) & 0x3) << 13) };
}

pub const NI_M_CFG_BYPASS_AI_MODE_MUX_MASK: u32 = NI_M_CFG_BYPASS_AI_MODE_MUX(3);
macro_rules! NI_M_CFG_BYPASS_AI_CAL_NEG {
    () => { (x) (((x) & 0x7) << 10) };
}

pub const NI_M_CFG_BYPASS_AI_CAL_NEG_MASK: u32 = NI_M_CFG_BYPASS_AI_CAL_NEG(7);
macro_rules! NI_M_CFG_BYPASS_AI_CAL_POS {
    () => { (x) (((x) & 0x7) << 7) };
}

pub const NI_M_CFG_BYPASS_AI_CAL_POS_MASK: u32 = NI_M_CFG_BYPASS_AI_CAL_POS(7);
pub const NI_M_CFG_BYPASS_AI_CAL_MASK: u32 = (NI_M_CFG_BYPASS_AI_CAL_POS_MASK |;
					 NI_M_CFG_BYPASS_AI_CAL_NEG_MASK | \
					 NI_M_CFG_BYPASS_AI_MODE_MUX_MASK | \
					 NI_M_CFG_BYPASS_AO_CAL_MASK)
macro_rules! NI_M_CFG_BYPASS_AI_BANK {
    () => { (x) (((x) & 0xf) << 3) };
}

pub const NI_M_CFG_BYPASS_AI_BANK_MASK: u32 = NI_M_CFG_BYPASS_AI_BANK(0xf);
macro_rules! NI_M_CFG_BYPASS_AI_CHAN {
    () => { (x) (((x) & 0x7) << 0) };
}

pub const NI_M_CFG_BYPASS_AI_CHAN_MASK: u32 = NI_M_CFG_BYPASS_AI_CHAN(7);
pub const NI_M_SCXI_DIO_ENA_REG: u32 = 0x21c;
pub const NI_M_CDI_FIFO_DATA_REG: u32 = 0x220;
pub const NI_M_CDO_FIFO_DATA_REG: u32 = 0x220;
pub const NI_M_CDIO_STATUS_REG: u32 = 0x224;
pub const NI_M_CDIO_STATUS_CDI_OVERFLOW: u32 = BIT(20);
pub const NI_M_CDIO_STATUS_CDI_OVERRUN: u32 = BIT(19);
pub const NI_M_CDIO_STATUS_CDI_ERROR: u32 = (NI_M_CDIO_STATUS_CDI_OVERFLOW |;
					 NI_M_CDIO_STATUS_CDI_OVERRUN)
pub const NI_M_CDIO_STATUS_CDI_FIFO_REQ: u32 = BIT(18);
pub const NI_M_CDIO_STATUS_CDI_FIFO_FULL: u32 = BIT(17);
pub const NI_M_CDIO_STATUS_CDI_FIFO_EMPTY: u32 = BIT(16);
pub const NI_M_CDIO_STATUS_CDO_UNDERFLOW: u32 = BIT(4);
pub const NI_M_CDIO_STATUS_CDO_OVERRUN: u32 = BIT(3);
pub const NI_M_CDIO_STATUS_CDO_ERROR: u32 = (NI_M_CDIO_STATUS_CDO_UNDERFLOW |;
					 NI_M_CDIO_STATUS_CDO_OVERRUN)
pub const NI_M_CDIO_STATUS_CDO_FIFO_REQ: u32 = BIT(2);
pub const NI_M_CDIO_STATUS_CDO_FIFO_FULL: u32 = BIT(1);
pub const NI_M_CDIO_STATUS_CDO_FIFO_EMPTY: u32 = BIT(0);
pub const NI_M_CDIO_CMD_REG: u32 = 0x224;
pub const NI_M_CDI_CMD_SW_UPDATE: u32 = BIT(20);
pub const NI_M_CDO_CMD_SW_UPDATE: u32 = BIT(19);
pub const NI_M_CDO_CMD_F_E_INT_ENA_CLR: u32 = BIT(17);
pub const NI_M_CDO_CMD_F_E_INT_ENA_SET: u32 = BIT(16);
pub const NI_M_CDI_CMD_ERR_INT_CONFIRM: u32 = BIT(15);
pub const NI_M_CDO_CMD_ERR_INT_CONFIRM: u32 = BIT(14);
pub const NI_M_CDI_CMD_F_REQ_INT_ENA_CLR: u32 = BIT(13);
pub const NI_M_CDI_CMD_F_REQ_INT_ENA_SET: u32 = BIT(12);
pub const NI_M_CDO_CMD_F_REQ_INT_ENA_CLR: u32 = BIT(11);
pub const NI_M_CDO_CMD_F_REQ_INT_ENA_SET: u32 = BIT(10);
pub const NI_M_CDI_CMD_ERR_INT_ENA_CLR: u32 = BIT(9);
pub const NI_M_CDI_CMD_ERR_INT_ENA_SET: u32 = BIT(8);
pub const NI_M_CDO_CMD_ERR_INT_ENA_CLR: u32 = BIT(7);
pub const NI_M_CDO_CMD_ERR_INT_ENA_SET: u32 = BIT(6);
pub const NI_M_CDI_CMD_RESET: u32 = BIT(5);
pub const NI_M_CDO_CMD_RESET: u32 = BIT(4);
pub const NI_M_CDI_CMD_ARM: u32 = BIT(3);
pub const NI_M_CDI_CMD_DISARM: u32 = BIT(2);
pub const NI_M_CDO_CMD_ARM: u32 = BIT(1);
pub const NI_M_CDO_CMD_DISARM: u32 = BIT(0);
pub const NI_M_CDI_MODE_REG: u32 = 0x228;
macro_rules! NI_M_CDI_MODE_DATA_LANE {
    () => { (x) (((x) & 0x3) << 12) };
}

pub const NI_M_CDI_MODE_DATA_LANE_MASK: u32 = NI_M_CDI_MODE_DATA_LANE(3);
pub const NI_M_CDI_MODE_DATA_LANE_0_15: u32 = NI_M_CDI_MODE_DATA_LANE(0);
pub const NI_M_CDI_MODE_DATA_LANE_16_31: u32 = NI_M_CDI_MODE_DATA_LANE(1);
pub const NI_M_CDI_MODE_DATA_LANE_0_7: u32 = NI_M_CDI_MODE_DATA_LANE(0);
pub const NI_M_CDI_MODE_DATA_LANE_8_15: u32 = NI_M_CDI_MODE_DATA_LANE(1);
pub const NI_M_CDI_MODE_DATA_LANE_16_23: u32 = NI_M_CDI_MODE_DATA_LANE(2);
pub const NI_M_CDI_MODE_DATA_LANE_24_31: u32 = NI_M_CDI_MODE_DATA_LANE(3);
pub const NI_M_CDI_MODE_FIFO_MODE: u32 = BIT(11);
pub const NI_M_CDI_MODE_POLARITY: u32 = BIT(10);
pub const NI_M_CDI_MODE_HALT_ON_ERROR: u32 = BIT(9);
macro_rules! NI_M_CDI_MODE_SAMPLE_SRC {
    () => { (x) (((x) & 0x3f) << 0) };
}

pub const NI_M_CDI_MODE_SAMPLE_SRC_MASK: u32 = NI_M_CDI_MODE_SAMPLE_SRC(0x3f);
pub const NI_M_CDO_MODE_REG: u32 = 0x22c;
macro_rules! NI_M_CDO_MODE_DATA_LANE {
    () => { (x) (((x) & 0x3) << 12) };
}

pub const NI_M_CDO_MODE_DATA_LANE_MASK: u32 = NI_M_CDO_MODE_DATA_LANE(3);
pub const NI_M_CDO_MODE_DATA_LANE_0_15: u32 = NI_M_CDO_MODE_DATA_LANE(0);
pub const NI_M_CDO_MODE_DATA_LANE_16_31: u32 = NI_M_CDO_MODE_DATA_LANE(1);
pub const NI_M_CDO_MODE_DATA_LANE_0_7: u32 = NI_M_CDO_MODE_DATA_LANE(0);
pub const NI_M_CDO_MODE_DATA_LANE_8_15: u32 = NI_M_CDO_MODE_DATA_LANE(1);
pub const NI_M_CDO_MODE_DATA_LANE_16_23: u32 = NI_M_CDO_MODE_DATA_LANE(2);
pub const NI_M_CDO_MODE_DATA_LANE_24_31: u32 = NI_M_CDO_MODE_DATA_LANE(3);
pub const NI_M_CDO_MODE_FIFO_MODE: u32 = BIT(11);
pub const NI_M_CDO_MODE_POLARITY: u32 = BIT(10);
pub const NI_M_CDO_MODE_HALT_ON_ERROR: u32 = BIT(9);
pub const NI_M_CDO_MODE_RETRANSMIT: u32 = BIT(8);
macro_rules! NI_M_CDO_MODE_SAMPLE_SRC {
    () => { (x) (((x) & 0x3f) << 0) };
}

pub const NI_M_CDO_MODE_SAMPLE_SRC_MASK: u32 = NI_M_CDO_MODE_SAMPLE_SRC(0x3f);
pub const NI_M_CDI_MASK_ENA_REG: u32 = 0x230;
pub const NI_M_CDO_MASK_ENA_REG: u32 = 0x234;
macro_rules! NI_M_STATIC_AI_CTRL_REG {
    () => { (x) ((x) ? (0x260 + (x)) : 0x064) };
}

macro_rules! NI_M_AO_REF_ATTENUATION_REG {
    () => { (x) (0x264 + (x)) };
}

pub const NI_M_AO_REF_ATTENUATION_X5: u32 = BIT(0);

enum {
	ai_gain_16 = 0,
	ai_gain_8,
	ai_gain_14,
	ai_gain_4,
	ai_gain_611x,
	ai_gain_622x,
	ai_gain_628x,
	ai_gain_6143
};

enum caldac_enum {
	caldac_none = 0,
	mb88341,
	dac8800,
	dac8043,
	ad8522,
	ad8804,
	ad8842,
	ad8804_debug
};

enum ni_reg_type {
	ni_reg_normal = 0x0,
	ni_reg_611x = 0x1,
	ni_reg_6711 = 0x2,
	ni_reg_6713 = 0x4,
	ni_reg_67xx_mask = 0x6,
	ni_reg_6xxx_mask = 0x7,
	ni_reg_622x = 0x8,
	ni_reg_625x = 0x10,
	ni_reg_628x = 0x18,
	ni_reg_m_series_mask = 0x18,
	ni_reg_6143 = 0x20
};

#[repr(C)]
struct ni_board_struct {
	const char *name;
	const char *alt_route_name;
	int device_id;
	int isapnp_id;

	int n_adchan;
	unsigned int ai_maxdata;

	int ai_fifo_depth;
	unsigned int alwaysdither:1;
	int gainlkup;
	int ai_speed;

	int n_aochan;
	unsigned int ao_maxdata;
	int ao_fifo_depth;
	const struct comedi_lrange *ao_range_table;
	unsigned int ao_speed;

	int reg_type;
	unsigned int has_8255:1;
	unsigned int has_32dio_chan:1;
	unsigned int dio_speed; /* not for e-series */

	enum caldac_enum caldac[3];
};

pub const MAX_N_CALDACS: u32 = 34;
pub const MAX_N_AO_CHAN: u32 = 8;
pub const NUM_GPCT: u32 = 2;

pub const NUM_PFI_OUTPUT_SELECT_REGS: u32 = 6;
pub const NUM_RTSI_SHARED_MUXS: u32 = (NI_RTSI_BRD(-1) - NI_RTSI_BRD(0) + 1);

pub const M_SERIES_EEPROM_SIZE: u32 = 1024;

#[repr(C)]
struct ni_private {
	unsigned short dio_output;
	unsigned short dio_control;
	int aimode;
	unsigned int ai_calib_source;
	unsigned int ai_calib_source_enabled;
	/* protects access to windowed registers */
	spinlock_t window_lock;
	/* protects interrupt/dma register access */
	spinlock_t soft_reg_copy_lock;
	/* protects mite DMA channel request/release */
	spinlock_t mite_channel_lock;

	int changain_state;
	unsigned int changain_spec;

	unsigned int caldac_maxdata_list[MAX_N_CALDACS];
	unsigned short caldacs[MAX_N_CALDACS];

	unsigned short ai_cmd2;

	unsigned short ao_conf[MAX_N_AO_CHAN];
	unsigned short ao_mode1;
	unsigned short ao_mode2;
	unsigned short ao_mode3;
	unsigned short ao_cmd1;
	unsigned short ao_cmd2;

	struct ni_gpct_device *counter_dev;
	unsigned short an_trig_etc_reg;

	unsigned int ai_offset[512];

	unsigned long serial_interval_ns;
	unsigned char serial_hw_mode;
	unsigned short clock_and_fout;
	unsigned short clock_and_fout2;

	unsigned short int_a_enable_reg;
	unsigned short int_b_enable_reg;
	unsigned short io_bidirection_pin_reg;
	unsigned short rtsi_trig_direction_reg;
	unsigned short rtsi_trig_a_output_reg;
	unsigned short rtsi_trig_b_output_reg;
	unsigned short pfi_output_select_reg[NUM_PFI_OUTPUT_SELECT_REGS];
	unsigned short ai_ao_select_reg;
	unsigned short g0_g1_select_reg;
	unsigned short cdio_dma_select_reg;

	unsigned int clock_ns;
	unsigned int clock_source;

	unsigned short pwm_up_count;
	unsigned short pwm_down_count;

	unsigned short ai_fifo_buffer[0x2000];
	u8 eeprom_buffer[M_SERIES_EEPROM_SIZE];

	struct mite *mite;
	struct mite_channel *ai_mite_chan;
	struct mite_channel *ao_mite_chan;
	struct mite_channel *cdo_mite_chan;
	struct mite_ring *ai_mite_ring;
	struct mite_ring *ao_mite_ring;
	struct mite_ring *cdo_mite_ring;
	struct mite_ring *gpct_mite_ring[NUM_GPCT];

	/* ni_pcimio board type flags (based on the boardinfo reg_type) */
	unsigned int is_m_series:1;
	unsigned int is_6xxx:1;
	unsigned int is_611x:1;
	unsigned int is_6143:1;
	unsigned int is_622x:1;
	unsigned int is_625x:1;
	unsigned int is_628x:1;
	unsigned int is_67xx:1;
	unsigned int is_6711:1;
	unsigned int is_6713:1;

	/*
	 * Boolean value of whether device needs to be armed.
	 *
	 * Currently, only NI AO devices are known to be needing arming, since
	 * the DAC registers must be preloaded before triggering.
	 * This variable should only be set true during a command operation
	 * (e.g ni_ao_cmd) and should then be set false by the arming
	 * function (e.g. ni_ao_arm).
	 *
	 * This variable helps to ensure that multiple DMA allocations are not
	 * possible.
	 */
	unsigned int ao_needs_arming:1;

	/* device signal route tables */
	struct ni_route_tables routing_tables;

	/*
	 * Number of clients (RTSI lines) for current RTSI MUX source.
	 *
	 * This allows resource management of RTSI board/shared mux lines by
	 * marking the RTSI line that is using a particular MUX.  Currently,
	 * these lines are only automatically allocated based on source of the
	 * route requested.  Furthermore, the only way that this auto-allocation
	 * and configuration works is via the globally-named ni signal/terminal
	 * names.
	 */
	u8 rtsi_shared_mux_usage[NUM_RTSI_SHARED_MUXS];

	/*
	 * softcopy register for rtsi shared mux/board lines.
	 * For e-series, the bit layout of this register is
	 * (docs: mhddk/nieseries/ChipObjects/tSTC.{h,ipp},
	 *        DAQ-STC, Jan 1999, 340934B-01):
	 *   bits 0:2  --  NI_RTSI_BRD(0) source selection
	 *   bits 3:5  --  NI_RTSI_BRD(1) source selection
	 *   bits 6:8  --  NI_RTSI_BRD(2) source selection
	 *   bits 9:11 --  NI_RTSI_BRD(3) source selection
	 *   bit  12   --  NI_RTSI_BRD(0) direction, 0:input, 1:output
	 *   bit  13   --  NI_RTSI_BRD(1) direction, 0:input, 1:output
	 *   bit  14   --  NI_RTSI_BRD(2) direction, 0:input, 1:output
	 *   bit  15   --  NI_RTSI_BRD(3) direction, 0:input, 1:output
	 *   According to DAQ-STC:
	 *     RTSI Board Interface--Configured as an input, each bidirectional
	 *     RTSI_BRD pin can drive any of the seven RTSI_TRIGGER pins.
	 *     RTSI_BRD<0..1> can also be driven by AI STOP and RTSI_BRD<2..3>
	 *     can also be driven by the AI START and SCAN_IN_PROG signals.
	 *     These pins provide a mechanism for additional board-level signals
	 *     to be sent on or received from the RTSI bus.
	 *   Couple of comments:
	 *   - Neither the DAQ-STC nor the MHDDK is clear on what the direction
	 *     of the RTSI_BRD pins actually means.  There does not appear to be
	 *     any clear indication on what "output" would mean, since the point
	 *     of the RTSI_BRD lines is to always drive one of the
	 *     RTSI_TRIGGER<0..6> lines.
	 *   - The DAQ-STC also indicates that the NI_RTSI_BRD lines can be
	 *     driven by any of the RTSI_TRIGGER<0..6> lines.
	 *     But, looking at valid device routes, as visually imported from
	 *     NI-MAX, there appears to be only one family (so far) that has the
	 *     ability to route a signal from one TRIGGER_LINE to another
	 *     TRIGGER_LINE: the 653x family of DIO devices.
	 *
	 * For m-series, the bit layout of this register is
	 * (docs: mhddk/nimseries/ChipObjects/tMSeries.{h,ipp}):
	 *   bits  0:3  --  NI_RTSI_BRD(0) source selection
	 *   bits  4:7  --  NI_RTSI_BRD(1) source selection
	 *   bits  8:11 --  NI_RTSI_BRD(2) source selection
	 *   bits 12:15 --  NI_RTSI_BRD(3) source selection
	 *   Note:  The m-series does not have any option to change direction of
	 *   NI_RTSI_BRD muxes.  Furthermore, there are no register values that
	 *   indicate the ability to have TRIGGER_LINES driving the output of
	 *   the NI_RTSI_BRD muxes.
	 */
	u16 rtsi_shared_mux_reg;

	/*
	 * Number of clients (RTSI lines) for current RGOUT0 path.
	 * Stored in part of in RTSI_TRIG_DIR or RTSI_TRIGB registers
	 */
	u8 rgout0_usage;
};


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
