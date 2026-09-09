/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
    msp3400.h - definition for msp3400 inputs and outputs

    Copyright (C) 2006 Hans Verkuil (hverkuil@kernel.org)

*/

/* msp3400 routing
   ===============

   The msp3400 has a complicated routing scheme with many possible
   combinations. The details are all in the datasheets but I will try
   to give a short description here.

   Inputs
   ======

   There are 1) tuner inputs, 2) I2S inputs, 3) SCART inputs. You will have
   to select which tuner input to use and which SCART input to use. The
   selected tuner input, the selected SCART input and all I2S inputs go to
   the DSP (the tuner input first goes through the demodulator).

   The DSP handles things like volume, bass/treble, balance, and some chips
   have support for surround sound. It has several outputs: MAIN, AUX, I2S
   and SCART1/2. Each output can select which DSP input to use. So the MAIN
   output can select the tuner input while at the same time the SCART1 output
   uses the I2S input.

   Outputs
   =======

   Most DSP outputs are also the outputs of the msp3400. However, the SCART
   outputs of the msp3400 can select which input to use: either the SCART1 or
   SCART2 output from the DSP, or the msp3400 SCART inputs, thus completely
   bypassing the DSP.

   Summary
   =======

   So to specify a complete routing scheme for the msp3400 you will have to
   specify in the 'input' arg of the s_routing function:

   1) which tuner input to use
   2) which SCART input to use
   3) which DSP input to use for each DSP output

   And in the 'output' arg of the s_routing function you specify:

   1) which SCART input to use for each SCART output

   Depending on how the msp is wired to the other components you can
   ignore or mute certain inputs or outputs.

   Also, depending on the msp version only a subset of the inputs or
   outputs may be present. At the end of this header some tables are
   added containing a list of what is available for each msp version.
 */

/* Inputs to the DSP unit: two independent selections have to be made:
   1) the tuner (SIF) input
   2) the SCART input
   Bits 0-2 are used for the SCART input select, bit 3 is used for the tuner
   input, bits 4-7 are reserved.
 */

/* SCART input to DSP selection */
pub const MSP_IN_SCART1: u32 = 0; /* Pin SC1_IN */
pub const MSP_IN_SCART2: u32 = 1; /* Pin SC2_IN */
pub const MSP_IN_SCART3: u32 = 2; /* Pin SC3_IN */
pub const MSP_IN_SCART4: u32 = 3; /* Pin SC4_IN */
pub const MSP_IN_MONO: u32 = 6; /* Pin MONO_IN */
pub const MSP_IN_MUTE: u32 = 7; /* Mute DSP input */
#[inline]
pub const fn MSP_SCART_TO_DSP(input: u32) -> u32 { input }

/* Tuner input to demodulator and DSP selection */
pub const MSP_IN_TUNER1: u32 = 0; /* Analog Sound IF input pin ANA_IN1 */
pub const MSP_IN_TUNER2: u32 = 1; /* Analog Sound IF input pin ANA_IN2 */
#[inline]
pub const fn MSP_TUNER_TO_DSP(input: u32) -> u32 { input << 3 }

/* The msp has up to 5 DSP outputs, each output can independently select
   a DSP input.

   The DSP outputs are: loudspeaker output (aka MAIN), headphones output
   (aka AUX), SCART1 DA output, SCART2 DA output and an I2S output.
   There also is a quasi-peak detector output, but that is not used by
   this driver and is set to the same input as the loudspeaker output.
   Not all outputs are supported by all msp models. Setting the input
   of an unsupported output will be ignored by the driver.

   There are up to 16 DSP inputs to choose from, so each output is
   assigned 4 bits.

   Note: the 44x8G can mix two inputs and feed the result back to the
   DSP. This is currently not implemented. Also not implemented is the
   multi-channel capable I2S3 input of the 44x0G. If someone can demonstrate
   a need for one of those features then additional support can be added. */
pub const MSP_DSP_IN_TUNER: u32 = 0; /* Tuner DSP input */
pub const MSP_DSP_IN_SCART: u32 = 2; /* SCART DSP input */
pub const MSP_DSP_IN_I2S1: u32 = 5; /* I2S1 DSP input */
pub const MSP_DSP_IN_I2S2: u32 = 6; /* I2S2 DSP input */
pub const MSP_DSP_IN_I2S3: u32 = 7; /* I2S3 DSP input */
pub const MSP_DSP_IN_MAIN_AVC: u32 = 11; /* MAIN AVC processed DSP input */
pub const MSP_DSP_IN_MAIN: u32 = 12; /* MAIN DSP input */
pub const MSP_DSP_IN_AUX: u32 = 13; /* AUX DSP input */
#[inline] pub const fn MSP_DSP_TO_MAIN(input: u32) -> u32 { input << 4 }
#[inline] pub const fn MSP_DSP_TO_AUX(input: u32) -> u32 { input << 8 }
#[inline] pub const fn MSP_DSP_TO_SCART1(input: u32) -> u32 { input << 12 }
#[inline] pub const fn MSP_DSP_TO_SCART2(input: u32) -> u32 { input << 16 }
#[inline] pub const fn MSP_DSP_TO_I2S(input: u32) -> u32 { input << 20 }

/* Output SCART select: the SCART outputs can select which input
   to use. */
pub const MSP_SC_IN_SCART1: u32 = 0; /* SCART1 input, bypassing the DSP */
pub const MSP_SC_IN_SCART2: u32 = 1; /* SCART2 input, bypassing the DSP */
pub const MSP_SC_IN_SCART3: u32 = 2; /* SCART3 input, bypassing the DSP */
pub const MSP_SC_IN_SCART4: u32 = 3; /* SCART4 input, bypassing the DSP */
pub const MSP_SC_IN_DSP_SCART1: u32 = 4; /* DSP SCART1 input */
pub const MSP_SC_IN_DSP_SCART2: u32 = 5; /* DSP SCART2 input */
pub const MSP_SC_IN_MONO: u32 = 6; /* MONO input, bypassing the DSP */
pub const MSP_SC_IN_MUTE: u32 = 7; /* MUTE output */
#[inline] pub const fn MSP_SC_TO_SCART1(input: u32) -> u32 { input }
#[inline] pub const fn MSP_SC_TO_SCART2(input: u32) -> u32 { input << 4 }

/* Shortcut macros */
#[inline]
pub const fn MSP_INPUT(sc: u32, t: u32, main_aux_src: u32, sc_i2s_src: u32) -> u32 {
    MSP_SCART_TO_DSP(sc) | MSP_TUNER_TO_DSP(t) |
    MSP_DSP_TO_MAIN(main_aux_src) | MSP_DSP_TO_AUX(main_aux_src) |
    MSP_DSP_TO_SCART1(sc_i2s_src) | MSP_DSP_TO_SCART2(sc_i2s_src) |
    MSP_DSP_TO_I2S(sc_i2s_src)
}
pub const MSP_INPUT_DEFAULT: u32 = MSP_INPUT(MSP_IN_SCART1, MSP_IN_TUNER1,
                                              MSP_DSP_IN_TUNER, MSP_DSP_IN_TUNER);
#[inline]
pub const fn MSP_OUTPUT(sc: u32) -> u32 { MSP_SC_TO_SCART1(sc) | MSP_SC_TO_SCART2(sc) }
/* This equals the RESET position of the msp3400 ACB register */
pub const MSP_OUTPUT_DEFAULT: u32 = MSP_SC_TO_SCART1(MSP_SC_IN_SCART3) |
                                    MSP_SC_TO_SCART2(MSP_SC_IN_DSP_SCART1);

/* Tuner inputs vs. msp version */
/* Chip      TUNER_1   TUNER_2
   -------------------------
   msp34x0b  y         y
   msp34x0c  y         y
   msp34x0d  y         y
   msp34x5d  y         n
   msp34x7d  y         n
   msp34x0g  y         y
   msp34x1g  y         y
   msp34x2g  y         y
   msp34x5g  y         n
   msp34x7g  y         n
   msp44x0g  y         y
   msp44x8g  y         y
 */

/* SCART inputs vs. msp version */
/* Chip      SC1 SC2 SC3 SC4
   -------------------------
   msp34x0b  y   y   y   n
   msp34x0c  y   y   y   n
   msp34x0d  y   y   y   y
   msp34x5d  y   y   n   n
   msp34x7d  y   n   n   n
   msp34x0g  y   y   y   y
   msp34x1g  y   y   y   y
   msp34x2g  y   y   y   y
   msp34x5g  y   y   n   n
   msp34x7g  y   n   n   n
   msp44x0g  y   y   y   y
   msp44x8g  y   y   y   y
 */

/* DSP inputs vs. msp version (tuner and SCART inputs are always available) */
/* Chip      I2S1 I2S2 I2S3 MAIN_AVC MAIN AUX
   ------------------------------------------
   msp34x0b  y    n    n    n        n    n
   msp34x0c  y    y    n    n        n    n
   msp34x0d  y    y    n    n        n    n
   msp34x5d  y    y    n    n        n    n
   msp34x7d  n    n    n    n        n    n
   msp34x0g  y    y    n    n        n    n
   msp34x1g  y    y    n    n        n    n
   msp34x2g  y    y    n    y        y    y
   msp34x5g  y    y    n    n        n    n
   msp34x7g  n    n    n    n        n    n
   msp44x0g  y    y    y    y        y    y
   msp44x8g  y    y    y    n        n    n
 */

/* DSP outputs vs. msp version */
/* Chip      MAIN AUX SCART1 SCART2 I2S
   ------------------------------------
   msp34x0b  y    y   y      n      y
   msp34x0c  y    y   y      n      y
   msp34x0d  y    y   y      y      y
   msp34x5d  y    n   y      n      y
   msp34x7d  y    n   y      n      n
   msp34x0g  y    y   y      y      y
   msp34x1g  y    y   y      y      y
   msp34x2g  y    y   y      y      y
   msp34x5g  y    n   y      n      y
   msp34x7g  y    n   y      n      n
   msp44x0g  y    y   y      y      y
   msp44x8g  y    y   y      y      y
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
