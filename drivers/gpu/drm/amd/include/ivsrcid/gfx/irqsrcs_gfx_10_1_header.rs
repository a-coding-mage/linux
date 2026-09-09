/*
 * Copyright 2018 Advanced Micro Devices, Inc.
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

pub const GFX_10_1__SRCID__CP_RB_INTERRUPT_PKT: u32 = 176; // B0 CP_INTERRUPT pkt in RB
pub const GFX_10_1__SRCID__CP_GENERIC_INT: u32 = 177; // B1 MES GENERIC INT
pub const GFX_10_1__SRCID__CP_IB1_INTERRUPT_PKT: u32 = 177; // B1 CP_INTERRUPT pkt in IB1
pub const GFX_10_1__SRCID__CP_IB2_INTERRUPT_PKT: u32 = 178; // B2 CP_INTERRUPT pkt in IB2
pub const GFX_10_1__SRCID__CP_PM4_PKT_RSVD_BIT_ERROR: u32 = 180; // B4 PM4 Pkt Rsvd Bits Error
pub const GFX_10_1__SRCID__CP_EOP_INTERRUPT: u32 = 181; // B5 End-of-Pipe Interrupt
pub const GFX_10_1__SRCID__CP_BAD_OPCODE_ERROR: u32 = 183; // B7 Bad Opcode Error
pub const GFX_10_1__SRCID__CP_PRIV_REG_FAULT: u32 = 184; // B8 Privileged Register Fault
pub const GFX_10_1__SRCID__CP_PRIV_INSTR_FAULT: u32 = 185; // B9 Privileged Instr Fault
pub const GFX_10_1__SRCID__CP_WAIT_MEM_SEM_FAULT: u32 = 186; // BA Wait Memory Semaphore Fault (Synchronization Object Fault)
pub const GFX_10_1__SRCID__CP_CTX_EMPTY_INTERRUPT: u32 = 187; // BB Context Empty Interrupt
pub const GFX_10_1__SRCID__CP_CTX_BUSY_INTERRUPT: u32 = 188; // BC Context Busy Interrupt
pub const GFX_10_1__SRCID__CP_ME_WAIT_REG_MEM_POLL_TIMEOUT: u32 = 192; // C0 CP.ME Wait_Reg_Mem Poll Timeout
pub const GFX_10_1__SRCID__CP_SIG_INCOMPLETE: u32 = 193; // C1 "Surface Probe Fault Signal Incomplete"
pub const GFX_10_1__SRCID__CP_PREEMPT_ACK: u32 = 194; // C2 Preemption Ack-wledge
pub const GFX_10_1__SRCID__CP_GPF: u32 = 195; // C3 General Protection Fault (GPF)
pub const GFX_10_1__SRCID__CP_GDS_ALLOC_ERROR: u32 = 196; // C4 GDS Alloc Error
pub const GFX_10_1__SRCID__CP_ECC_ERROR: u32 = 197; // C5 ECC  Error
pub const GFX_10_1__SRCID__CP_COMPUTE_QUERY_STATUS: u32 = 199; // C7 Compute query status
pub const GFX_10_1__SRCID__CP_VM_DOORBELL: u32 = 200; // C8 Unattached VM Doorbell Received
pub const GFX_10_1__SRCID__CP_FUE_ERROR: u32 = 201; // C9 ECC FUE Error
pub const GFX_10_1__SRCID__RLC_STRM_PERF_MONITOR_INTERRUPT: u32 = 202; // CA Streaming Perf Monitor Interrupt
pub const GFX_10_1__SRCID__GRBM_RD_TIMEOUT_ERROR: u32 = 232; // E8 CRead timeout error
pub const GFX_10_1__SRCID__GRBM_REG_GUI_IDLE: u32 = 233; // E9 Register GUI Idle
pub const GFX_10_1__SRCID__SQ_INTERRUPT_ID: u32 = 239; // EF SQ Interrupt (ttrace wrap, errors)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
