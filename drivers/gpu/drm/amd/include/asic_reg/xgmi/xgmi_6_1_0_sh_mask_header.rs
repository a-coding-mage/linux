/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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

// PCS_XGMI3X16_PCS_ERROR_STATUS
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__DataLossErr__SHIFT: u32 = 0x0;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__TrainingErr__SHIFT: u32 = 0x1;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__FlowCtrlAckErr__SHIFT: u32 = 0x2;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RxFifoUnderflowErr__SHIFT: u32 = 0x3;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RxFifoOverflowErr__SHIFT: u32 = 0x4;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__CRCErr__SHIFT: u32 = 0x5;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__BERExceededErr__SHIFT: u32 = 0x6;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__TxVcidDataErr__SHIFT: u32 = 0x7;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ReplayBufParityErr__SHIFT: u32 = 0x8;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__DataParityErr__SHIFT: u32 = 0x9;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ReplayFifoOverflowErr__SHIFT: u32 = 0xa;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ReplayFifoUnderflowErr__SHIFT: u32 = 0xb;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ElasticFifoOverflowErr__SHIFT: u32 = 0xc;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__DeskewErr__SHIFT: u32 = 0xd;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__FlowCtrlCRCErr__SHIFT: u32 = 0xe;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__DataStartupLimitErr__SHIFT: u32 = 0xf;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__FCInitTimeoutErr__SHIFT: u32 = 0x10;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RecoveryTimeoutErr__SHIFT: u32 = 0x11;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ReadySerialTimeoutErr__SHIFT: u32 = 0x12;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ReadySerialAttemptErr__SHIFT: u32 = 0x13;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RecoveryAttemptErr__SHIFT: u32 = 0x14;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RecoveryRelockAttemptErr__SHIFT: u32 = 0x15;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ReplayAttemptErr__SHIFT: u32 = 0x16;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__SyncHdrErr__SHIFT: u32 = 0x17;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__TxReplayTimeoutErr__SHIFT: u32 = 0x18;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RxReplayTimeoutErr__SHIFT: u32 = 0x19;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__LinkSubTxTimeoutErr__SHIFT: u32 = 0x1a;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__LinkSubRxTimeoutErr__SHIFT: u32 = 0x1b;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RxCMDPktErr__SHIFT: u32 = 0x1c;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__DataLossErr_MASK: u32 = 0x00000001;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__TrainingErr_MASK: u32 = 0x00000002;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__FlowCtrlAckErr_MASK: u32 = 0x00000004;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RxFifoUnderflowErr_MASK: u32 = 0x00000008;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RxFifoOverflowErr_MASK: u32 = 0x00000010;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__CRCErr_MASK: u32 = 0x00000020;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__BERExceededErr_MASK: u32 = 0x00000040;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__TxVcidDataErr_MASK: u32 = 0x00000080;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ReplayBufParityErr_MASK: u32 = 0x00000100;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__DataParityErr_MASK: u32 = 0x00000200;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ReplayFifoOverflowErr_MASK: u32 = 0x00000400;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ReplayFifoUnderflowErr_MASK: u32 = 0x00000800;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ElasticFifoOverflowErr_MASK: u32 = 0x00001000;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__DeskewErr_MASK: u32 = 0x00002000;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__FlowCtrlCRCErr_MASK: u32 = 0x00004000;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__DataStartupLimitErr_MASK: u32 = 0x00008000;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__FCInitTimeoutErr_MASK: u32 = 0x00010000;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RecoveryTimeoutErr_MASK: u32 = 0x00020000;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ReadySerialTimeoutErr_MASK: u32 = 0x00040000;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ReadySerialAttemptErr_MASK: u32 = 0x00080000;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RecoveryAttemptErr_MASK: u32 = 0x00100000;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RecoveryRelockAttemptErr_MASK: u32 = 0x00200000;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ReplayAttemptErr_MASK: u32 = 0x00400000;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__SyncHdrErr_MASK: u32 = 0x00800000;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__TxReplayTimeoutErr_MASK: u32 = 0x01000000;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RxReplayTimeoutErr_MASK: u32 = 0x02000000;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__LinkSubTxTimeoutErr_MASK: u32 = 0x04000000;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__LinkSubRxTimeoutErr_MASK: u32 = 0x08000000;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RxCMDPktErr_MASK: u32 = 0x10000000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
