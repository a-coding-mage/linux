/*
 *  BSD LICENSE
 *
 *  Copyright(c) 2017 Broadcom Corporation.  All rights reserved.
 *
 *  Redistribution and use in source and binary forms, with or without
 *  modification, are permitted provided that the following conditions
 *  are met:
 *
 *    * Redistributions of source code must retain the above copyright
 *      notice, this list of conditions and the following disclaimer.
 *    * Redistributions in binary form must reproduce the above copyright
 *      notice, this list of conditions and the following disclaimer in
 *      the documentation and/or other materials provided with the
 *      distribution.
 *    * Neither the name of Broadcom Corporation nor the names of its
 *      contributors may be used to endorse or promote products derived
 *      from this software without specific prior written permission.
 *
 *  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 *  "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 *  LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 *  A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 *  OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 *  SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 *  LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 *  DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 *  THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 *  (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 *  OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

/* Alternate functions available in MUX controller */
pub const MODE_NITRO: u32 = 0;
pub const MODE_NAND: u32 = 1;
pub const MODE_PNOR: u32 = 2;
pub const MODE_GPIO: u32 = 3;

/* Pad configuration attribute */
pub const PAD_SLEW_RATE_ENA: u32 = 1 << 0;
pub const PAD_SLEW_RATE_ENA_MASK: u32 = 1 << 0;

pub const PAD_DRIVE_STRENGTH_2_MA: u32 = 0 << 1;
pub const PAD_DRIVE_STRENGTH_4_MA: u32 = 1 << 1;
pub const PAD_DRIVE_STRENGTH_6_MA: u32 = 2 << 1;
pub const PAD_DRIVE_STRENGTH_8_MA: u32 = 3 << 1;
pub const PAD_DRIVE_STRENGTH_10_MA: u32 = 4 << 1;
pub const PAD_DRIVE_STRENGTH_12_MA: u32 = 5 << 1;
pub const PAD_DRIVE_STRENGTH_14_MA: u32 = 6 << 1;
pub const PAD_DRIVE_STRENGTH_16_MA: u32 = 7 << 1;
pub const PAD_DRIVE_STRENGTH_MASK: u32 = 7 << 1;

pub const PAD_PULL_UP_ENA: u32 = 1 << 4;
pub const PAD_PULL_UP_ENA_MASK: u32 = 1 << 4;

pub const PAD_PULL_DOWN_ENA: u32 = 1 << 5;
pub const PAD_PULL_DOWN_ENA_MASK: u32 = 1 << 5;

pub const PAD_INPUT_PATH_DIS: u32 = 1 << 6;
pub const PAD_INPUT_PATH_DIS_MASK: u32 = 1 << 6;

pub const PAD_HYSTERESIS_ENA: u32 = 1 << 7;
pub const PAD_HYSTERESIS_ENA_MASK: u32 = 1 << 7;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
