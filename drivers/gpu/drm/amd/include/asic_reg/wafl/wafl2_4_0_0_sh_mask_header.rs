/*
 * Copyright 2020 Advanced Micro Devices, Inc.
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

// PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__DataLossErr__SHIFT: u32 = 0x0;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__TrainingErr__SHIFT: u32 = 0x1;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__CRCErr__SHIFT: u32 = 0x5;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__BERExceededErr__SHIFT: u32 = 0x6;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__TxMetaDataErr__SHIFT: u32 = 0x7;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__ReplayBufParityErr__SHIFT: u32 = 0x8;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__DataParityErr__SHIFT: u32 = 0x9;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__ReplayFifoOverflowErr__SHIFT: u32 = 0xa;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__ReplayFifoUnderflowErr__SHIFT: u32 = 0xb;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__ElasticFifoOverflowErr__SHIFT: u32 = 0xc;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__DeskewErr__SHIFT: u32 = 0xd;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__DataStartupLimitErr__SHIFT: u32 = 0xf;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__FCInitTimeoutErr__SHIFT: u32 = 0x10;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__RecoveryTimeoutErr__SHIFT: u32 = 0x11;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__ReadySerialTimeoutErr__SHIFT: u32 = 0x12;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__ReadySerialAttemptErr__SHIFT: u32 = 0x13;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__RecoveryAttemptErr__SHIFT: u32 = 0x14;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__RecoveryRelockAttemptErr__SHIFT: u32 = 0x15;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__ClearBERAccum__SHIFT: u32 = 0x17;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__BERAccumulator__SHIFT: u32 = 0x18;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__DataLossErr_MASK: u32 = 0x00000001;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__TrainingErr_MASK: u32 = 0x00000002;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__CRCErr_MASK: u32 = 0x00000020;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__BERExceededErr_MASK: u32 = 0x00000040;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__TxMetaDataErr_MASK: u32 = 0x00000080;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__ReplayBufParityErr_MASK: u32 = 0x00000100;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__DataParityErr_MASK: u32 = 0x00000200;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__ReplayFifoOverflowErr_MASK: u32 = 0x00000400;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__ReplayFifoUnderflowErr_MASK: u32 = 0x00000800;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__ElasticFifoOverflowErr_MASK: u32 = 0x00001000;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__DeskewErr_MASK: u32 = 0x00002000;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__DataStartupLimitErr_MASK: u32 = 0x00008000;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__FCInitTimeoutErr_MASK: u32 = 0x00010000;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__RecoveryTimeoutErr_MASK: u32 = 0x00020000;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__ReadySerialTimeoutErr_MASK: u32 = 0x00040000;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__ReadySerialAttemptErr_MASK: u32 = 0x00080000;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__RecoveryAttemptErr_MASK: u32 = 0x00100000;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__RecoveryRelockAttemptErr_MASK: u32 = 0x00200000;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__ClearBERAccum_MASK: u32 = 0x00800000;
pub const PCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS__BERAccumulator_MASK: u32 = 0xFF000000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
