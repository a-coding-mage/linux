/*
 * Copyright 2019 Advanced Micro Devices, Inc.
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
 * Authors: AMD
 *
 */

pub const NBIF_7_4__SRCID__CHIP_ERR_INT_EVENT: u32 = 0x5E; // Error generated
pub const NBIF_7_4__SRCID__DOORBELL_INTERRUPT: u32 = 0x5F; // Interrupt for doorbell event during VDDGFX off
pub const NBIF_7_4__SRCID__RAS_CONTROLLER_INTERRUPT: u32 = 0x60; // Interrupt for ras_intr_valid from RAS controller
pub const NBIF_7_4__SRCID__ERREVENT_ATHUB_INTERRUPT: u32 = 0x61; // Interrupt for SDP ErrEvent received from ATHUB
pub const NBIF_7_4__SRCID__PF_VF_MSGBUF_VALID: u32 = 0x87; // Valid message in PF->VF mailbox message buffer (The interrupt is sent on behalf of PF)
pub const NBIF_7_4__SRCID__PF_VF_MSGBUF_ACK: u32 = 0x88; // Acknowledge message in PF->VF mailbox message buffer (The interrupt is sent on behalf of VF)
pub const NBIF_7_4__SRCID__VF_PF_MSGBUF_VALID: u32 = 0x89; // Valid message in VF->PF mailbox message buffer (The interrupt is sent on behalf of VF)
pub const NBIF_7_4__SRCID__VF_PF_MSGBUF_ACK: u32 = 0x8A; // Acknowledge message in VF->PF mailbox message buffer (The interrupt is sent on behalf of PF)
pub const NBIF_7_4__SRCID__CHIP_DPA_INT_EVENT: u32 = 0xA0; // BIF_CHIP_DPA_INT_EVENT
pub const NBIF_7_4__SRCID__CHIP_SLOT_POWER_CHG_INT_EVENT: u32 = 0xA1; // BIF_CHIP_SLOT_POWER_CHG_INT_EVENT
pub const NBIF_7_4__SRCID__ATOMIC_UR_OPCODE: u32 = 0xCE; // BIF receives unsupported atomic opcode from MC
pub const NBIF_7_4__SRCID__ATOMIC_REQESTEREN_LOW: u32 = 0xCF; // BIF receive atomic request from MC while AtomicOp Requester is not enabled in PCIE config space

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
