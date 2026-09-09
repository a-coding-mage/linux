/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Register operations for the EMU8000
 *
 * Copyright (C) 1999 Steve Ratcliffe
 * Based on awe_wave.c by Takashi Iwai
 */

/* Data port addresses relative to the EMU base. */
macro_rules! EMU8000_DATA0 { ($e:expr) => { ($e).port1 }; }
macro_rules! EMU8000_DATA1 { ($e:expr) => { ($e).port2 }; }
macro_rules! EMU8000_DATA2 { ($e:expr) => { ($e).port2 + 2 }; }
macro_rules! EMU8000_DATA3 { ($e:expr) => { ($e).port3 }; }
macro_rules! EMU8000_PTR { ($e:expr) => { ($e).port3 + 2 }; }

/* Make a command from a register and channel. */
macro_rules! EMU8000_CMD { ($reg:expr, $chan:expr) => { (($reg) << 5 | ($chan)) }; }

/* Commands to read and write the EMU8000 registers. */
macro_rules! EMU8000_CPF_READ { ($emu:expr, $chan:expr) => { snd_emu8000_peek_dw($emu, EMU8000_DATA0!($emu), EMU8000_CMD!(0, $chan)) }; }
macro_rules! EMU8000_PTRX_READ { ($emu:expr, $chan:expr) => { snd_emu8000_peek_dw($emu, EMU8000_DATA0!($emu), EMU8000_CMD!(1, $chan)) }; }
macro_rules! EMU8000_CVCF_READ { ($emu:expr, $chan:expr) => { snd_emu8000_peek_dw($emu, EMU8000_DATA0!($emu), EMU8000_CMD!(2, $chan)) }; }
macro_rules! EMU8000_VTFT_READ { ($emu:expr, $chan:expr) => { snd_emu8000_peek_dw($emu, EMU8000_DATA0!($emu), EMU8000_CMD!(3, $chan)) }; }
macro_rules! EMU8000_PSST_READ { ($emu:expr, $chan:expr) => { snd_emu8000_peek_dw($emu, EMU8000_DATA0!($emu), EMU8000_CMD!(6, $chan)) }; }
macro_rules! EMU8000_CSL_READ { ($emu:expr, $chan:expr) => { snd_emu8000_peek_dw($emu, EMU8000_DATA0!($emu), EMU8000_CMD!(7, $chan)) }; }
macro_rules! EMU8000_CCCA_READ { ($emu:expr, $chan:expr) => { snd_emu8000_peek_dw($emu, EMU8000_DATA1!($emu), EMU8000_CMD!(0, $chan)) }; }

macro_rules! emu8000_read_dw_fixed { ($name:ident, $data:ident, $reg:expr, $chan:expr) => { macro_rules! $name { ($emu:expr) => { snd_emu8000_peek_dw($emu, $data!($emu), EMU8000_CMD!($reg, $chan)) }; } }; }
emu8000_read_dw_fixed!(EMU8000_HWCF4_READ, EMU8000_DATA1, 1, 9);
emu8000_read_dw_fixed!(EMU8000_HWCF5_READ, EMU8000_DATA1, 1, 10);
emu8000_read_dw_fixed!(EMU8000_HWCF6_READ, EMU8000_DATA1, 1, 13);
emu8000_read_dw_fixed!(EMU8000_SMALR_READ, EMU8000_DATA1, 1, 20);
emu8000_read_dw_fixed!(EMU8000_SMARR_READ, EMU8000_DATA1, 1, 21);
emu8000_read_dw_fixed!(EMU8000_SMALW_READ, EMU8000_DATA1, 1, 22);
emu8000_read_dw_fixed!(EMU8000_SMARW_READ, EMU8000_DATA1, 1, 23);

macro_rules! EMU8000_SMLD_READ { ($emu:expr) => { snd_emu8000_peek($emu, EMU8000_DATA1!($emu), EMU8000_CMD!(1, 26)) }; }
macro_rules! EMU8000_SMRD_READ { ($emu:expr) => { snd_emu8000_peek($emu, EMU8000_DATA2!($emu), EMU8000_CMD!(1, 26)) }; }
macro_rules! EMU8000_WC_READ { ($emu:expr) => { snd_emu8000_peek($emu, EMU8000_DATA2!($emu), EMU8000_CMD!(1, 27)) }; }
macro_rules! EMU8000_HWCF1_READ { ($emu:expr) => { snd_emu8000_peek($emu, EMU8000_DATA1!($emu), EMU8000_CMD!(1, 29)) }; }
macro_rules! EMU8000_HWCF2_READ { ($emu:expr) => { snd_emu8000_peek($emu, EMU8000_DATA1!($emu), EMU8000_CMD!(1, 30)) }; }
macro_rules! EMU8000_HWCF3_READ { ($emu:expr) => { snd_emu8000_peek($emu, EMU8000_DATA1!($emu), EMU8000_CMD!(1, 31)) }; }

/* Per-channel byte-register reads. */
macro_rules! emu8000_read_byte { ($name:ident, $data:ident, $reg:expr) => { macro_rules! $name { ($emu:expr, $chan:expr) => { snd_emu8000_peek($emu, $data!($emu), EMU8000_CMD!($reg, $chan)) }; } }; }
emu8000_read_byte!(EMU8000_INIT1_READ, EMU8000_DATA1, 2);
emu8000_read_byte!(EMU8000_INIT2_READ, EMU8000_DATA2, 2);
emu8000_read_byte!(EMU8000_INIT3_READ, EMU8000_DATA1, 3);
emu8000_read_byte!(EMU8000_INIT4_READ, EMU8000_DATA2, 3);
emu8000_read_byte!(EMU8000_ENVVOL_READ, EMU8000_DATA1, 4);
emu8000_read_byte!(EMU8000_DCYSUSV_READ, EMU8000_DATA1, 5);
emu8000_read_byte!(EMU8000_ENVVAL_READ, EMU8000_DATA1, 6);
emu8000_read_byte!(EMU8000_DCYSUS_READ, EMU8000_DATA1, 7);
emu8000_read_byte!(EMU8000_ATKHLDV_READ, EMU8000_DATA2, 4);
emu8000_read_byte!(EMU8000_LFO1VAL_READ, EMU8000_DATA2, 5);
emu8000_read_byte!(EMU8000_ATKHLD_READ, EMU8000_DATA2, 6);
emu8000_read_byte!(EMU8000_LFO2VAL_READ, EMU8000_DATA2, 7);
emu8000_read_byte!(EMU8000_IP_READ, EMU8000_DATA3, 0);
emu8000_read_byte!(EMU8000_IFATN_READ, EMU8000_DATA3, 1);
emu8000_read_byte!(EMU8000_PEFE_READ, EMU8000_DATA3, 2);
emu8000_read_byte!(EMU8000_FMMOD_READ, EMU8000_DATA3, 3);
emu8000_read_byte!(EMU8000_TREMFRQ_READ, EMU8000_DATA3, 4);
emu8000_read_byte!(EMU8000_FM2FRQ2_READ, EMU8000_DATA3, 5);

macro_rules! emu8000_write_dw { ($name:ident, $data:ident, $reg:expr) => { macro_rules! $name { ($emu:expr, $chan:expr, $val:expr) => { snd_emu8000_poke_dw($emu, $data!($emu), EMU8000_CMD!($reg, $chan), $val) }; } }; }
emu8000_write_dw!(EMU8000_CPF_WRITE, EMU8000_DATA0, 0);
emu8000_write_dw!(EMU8000_PTRX_WRITE, EMU8000_DATA0, 1);
emu8000_write_dw!(EMU8000_CVCF_WRITE, EMU8000_DATA0, 2);
emu8000_write_dw!(EMU8000_VTFT_WRITE, EMU8000_DATA0, 3);
emu8000_write_dw!(EMU8000_PSST_WRITE, EMU8000_DATA0, 6);
emu8000_write_dw!(EMU8000_CSL_WRITE, EMU8000_DATA0, 7);
emu8000_write_dw!(EMU8000_CCCA_WRITE, EMU8000_DATA1, 0);

macro_rules! emu8000_write_dw_fixed { ($name:ident, $data:ident, $reg:expr, $chan:expr) => { macro_rules! $name { ($emu:expr, $val:expr) => { snd_emu8000_poke_dw($emu, $data!($emu), EMU8000_CMD!($reg, $chan), $val) }; } }; }
emu8000_write_dw_fixed!(EMU8000_HWCF4_WRITE, EMU8000_DATA1, 1, 9);
emu8000_write_dw_fixed!(EMU8000_HWCF5_WRITE, EMU8000_DATA1, 1, 10);
emu8000_write_dw_fixed!(EMU8000_HWCF6_WRITE, EMU8000_DATA1, 1, 13);
/* this register is not documented */
emu8000_write_dw_fixed!(EMU8000_HWCF7_WRITE, EMU8000_DATA1, 1, 14);
emu8000_write_dw_fixed!(EMU8000_SMALR_WRITE, EMU8000_DATA1, 1, 20);
emu8000_write_dw_fixed!(EMU8000_SMARR_WRITE, EMU8000_DATA1, 1, 21);
emu8000_write_dw_fixed!(EMU8000_SMALW_WRITE, EMU8000_DATA1, 1, 22);
emu8000_write_dw_fixed!(EMU8000_SMARW_WRITE, EMU8000_DATA1, 1, 23);

macro_rules! emu8000_write_byte_fixed { ($name:ident, $data:ident, $reg:expr, $chan:expr) => { macro_rules! $name { ($emu:expr, $val:expr) => { snd_emu8000_poke($emu, $data!($emu), EMU8000_CMD!($reg, $chan), $val) }; } }; }
emu8000_write_byte_fixed!(EMU8000_SMLD_WRITE, EMU8000_DATA1, 1, 26);
emu8000_write_byte_fixed!(EMU8000_SMRD_WRITE, EMU8000_DATA2, 1, 26);
emu8000_write_byte_fixed!(EMU8000_WC_WRITE, EMU8000_DATA2, 1, 27);
emu8000_write_byte_fixed!(EMU8000_HWCF1_WRITE, EMU8000_DATA1, 1, 29);
emu8000_write_byte_fixed!(EMU8000_HWCF2_WRITE, EMU8000_DATA1, 1, 30);
emu8000_write_byte_fixed!(EMU8000_HWCF3_WRITE, EMU8000_DATA1, 1, 31);

macro_rules! emu8000_write_byte { ($name:ident, $data:ident, $reg:expr) => { macro_rules! $name { ($emu:expr, $chan:expr, $val:expr) => { snd_emu8000_poke($emu, $data!($emu), EMU8000_CMD!($reg, $chan), $val) }; } }; }
emu8000_write_byte!(EMU8000_INIT1_WRITE, EMU8000_DATA1, 2);
emu8000_write_byte!(EMU8000_INIT2_WRITE, EMU8000_DATA2, 2);
emu8000_write_byte!(EMU8000_INIT3_WRITE, EMU8000_DATA1, 3);
emu8000_write_byte!(EMU8000_INIT4_WRITE, EMU8000_DATA2, 3);
emu8000_write_byte!(EMU8000_ENVVOL_WRITE, EMU8000_DATA1, 4);
emu8000_write_byte!(EMU8000_DCYSUSV_WRITE, EMU8000_DATA1, 5);
emu8000_write_byte!(EMU8000_ENVVAL_WRITE, EMU8000_DATA1, 6);
emu8000_write_byte!(EMU8000_DCYSUS_WRITE, EMU8000_DATA1, 7);
emu8000_write_byte!(EMU8000_ATKHLDV_WRITE, EMU8000_DATA2, 4);
emu8000_write_byte!(EMU8000_LFO1VAL_WRITE, EMU8000_DATA2, 5);
emu8000_write_byte!(EMU8000_ATKHLD_WRITE, EMU8000_DATA2, 6);
emu8000_write_byte!(EMU8000_LFO2VAL_WRITE, EMU8000_DATA2, 7);
emu8000_write_byte!(EMU8000_IP_WRITE, EMU8000_DATA3, 0);
emu8000_write_byte!(EMU8000_IFATN_WRITE, EMU8000_DATA3, 1);
emu8000_write_byte!(EMU8000_PEFE_WRITE, EMU8000_DATA3, 2);
emu8000_write_byte!(EMU8000_FMMOD_WRITE, EMU8000_DATA3, 3);
emu8000_write_byte!(EMU8000_TREMFRQ_WRITE, EMU8000_DATA3, 4);
emu8000_write_byte!(EMU8000_FM2FRQ2_WRITE, EMU8000_DATA3, 5);

emu8000_write_dw!(EMU8000_0080_WRITE, EMU8000_DATA0, 4);
emu8000_write_dw!(EMU8000_00A0_WRITE, EMU8000_DATA0, 5);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
