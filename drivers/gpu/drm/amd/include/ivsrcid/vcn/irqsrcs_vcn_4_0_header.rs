/*
 * Copyright 2022 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

pub const VCN_4_0__SRCID__UVD_TRAP: u32 = 114; // 0x72 UVD_TRAP
pub const VCN_4_0__SRCID__UVD_ENC_GENERAL_PURPOSE: u32 = 119; // 0x77 Encoder General Purpose
pub const VCN_4_0__SRCID__UVD_ENC_LOW_LATENCY: u32 = 120; // 0x78 Encoder Low Latency
pub const VCN_4_0__SRCID__UVD_SYSTEM_MESSAGE_INTERRUPT: u32 = 124; // 0x7c UVD system message interrupt
pub const VCN_4_0__SRCID__JPEG_ENCODE: u32 = 151; // 0x97 JRBC Encode interrupt
pub const VCN_4_0__SRCID__JPEG_DECODE: u32 = 153; // 0x99 JRBC Decode interrupt

pub const VCN_4_0__SRCID__JPEG1_DECODE: u32 = 149; // 0x95 JRBC1 Decode interrupt
pub const VCN_4_0__SRCID__JPEG2_DECODE: u32 = VCN_4_0__SRCID__JPEG_ENCODE; // 0x97 JRBC2 Decode interrupt
pub const VCN_4_0__SRCID__JPEG3_DECODE: u32 = 171; // 0xab JRBC3 Decode interrupt
pub const VCN_4_0__SRCID__JPEG4_DECODE: u32 = 172; // 0xac JRBC4 Decode interrupt
pub const VCN_4_0__SRCID__JPEG5_DECODE: u32 = 173; // 0xad JRBC5 Decode interrupt
pub const VCN_4_0__SRCID__JPEG6_DECODE: u32 = 174; // 0xae JRBC6 Decode interrupt
pub const VCN_4_0__SRCID__JPEG7_DECODE: u32 = 175; // 0xaf JRBC7 Decode interrupt

pub const VCN_4_0__SRCID_UVD_POISON: u32 = 160;
pub const VCN_4_0__SRCID_DJPEG0_POISON: u32 = 161;
pub const VCN_4_0__SRCID_EJPEG0_POISON: u32 = 162;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
