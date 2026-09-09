/*
 * Copyright 2015 Advanced Micro Devices, Inc.
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
 *
 */

/* We need to fill in the default values. */

pub const SMU7_VOTINGRIGHTSCLIENTS_DFLT0: u32 = 0x3FFFC102;
pub const SMU7_VOTINGRIGHTSCLIENTS_DFLT1: u32 = 0x000400;
pub const SMU7_VOTINGRIGHTSCLIENTS_DFLT2: u32 = 0xC00080;
pub const SMU7_VOTINGRIGHTSCLIENTS_DFLT3: u32 = 0xC00200;
pub const SMU7_VOTINGRIGHTSCLIENTS_DFLT4: u32 = 0xC01680;
pub const SMU7_VOTINGRIGHTSCLIENTS_DFLT5: u32 = 0xC00033;
pub const SMU7_VOTINGRIGHTSCLIENTS_DFLT6: u32 = 0xC00033;
pub const SMU7_VOTINGRIGHTSCLIENTS_DFLT7: u32 = 0x3FFFC000;

pub const SMU7_THERMALPROTECTCOUNTER_DFLT: u32 = 0x200;
pub const SMU7_STATICSCREENTHRESHOLDUNIT_DFLT: u32 = 0;
pub const SMU7_STATICSCREENTHRESHOLD_DFLT: u32 = 0x00C8;
pub const SMU7_GFXIDLECLOCKSTOPTHRESHOLD_DFLT: u32 = 0x200;
pub const SMU7_REFERENCEDIVIDER_DFLT: u32 = 4;

pub const SMU7_ULVVOLTAGECHANGEDELAY_DFLT: u32 = 1687;

pub const SMU7_CGULVPARAMETER_DFLT: u32 = 0x00040035;
pub const SMU7_CGULVCONTROL_DFLT: u32 = 0x00007450;
pub const SMU7_TARGETACTIVITY_DFLT: u32 = 50;
pub const SMU7_MCLK_TARGETACTIVITY_DFLT: u32 = 10;
pub const SMU7_SCLK_TARGETACTIVITY_DFLT: u32 = 30;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
