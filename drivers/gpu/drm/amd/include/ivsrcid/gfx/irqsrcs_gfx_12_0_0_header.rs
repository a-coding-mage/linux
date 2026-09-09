/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2024 Advanced Micro Devices, Inc.
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

pub const GFX_12_0_0__SRCID__UTCL2_FAULT: u32 = 0; // UTCL2 has encountered a fault or retry scenario
pub const GFX_12_0_0__SRCID__UTCL2_DATA_POISONING: u32 = 1; // UTCL2 for data poisoning
pub const GFX_12_0_0__SRCID__MEM_ACCES_MON: u32 = 10; // 0x0A EA memory access monitor interrupt
pub const GFX_12_0_0__SRCID__SDMA_ATOMIC_RTN_DONE: u32 = 48; // 0x30 SDMA atomic*_rtn ops complete
pub const GFX_12_0_0__SRCID__SDMA_TRAP: u32 = 49; // 0x31 Trap
pub const GFX_12_0_0__SRCID__SDMA_SRBMWRITE: u32 = 50; // 0x32 SRBM write Protection
pub const GFX_12_0_0__SRCID__SDMA_CTXEMPTY: u32 = 51; // 0x33 Context Empty
pub const GFX_12_0_0__SRCID__SDMA_PREEMPT: u32 = 52; // 0x34 SDMA New Run List
pub const GFX_12_0_0__SRCID__SDMA_IB_PREEMPT: u32 = 53; // 0x35 sdma mid - command buffer preempt interrupt
pub const GFX_12_0_0__SRCID__SDMA_DOORBELL_INVALID: u32 = 54; // 0x36 Doorbell BE invalid
pub const GFX_12_0_0__SRCID__SDMA_QUEUE_HANG: u32 = 55; // 0x37 Queue hang or Command timeout
pub const GFX_12_0_0__SRCID__SDMA_ATOMIC_TIMEOUT: u32 = 56; // 0x38 SDMA atomic CMPSWAP loop timeout
pub const GFX_12_0_0__SRCID__SDMA_POLL_TIMEOUT: u32 = 57; // 0x39 SRBM read poll timeout
pub const GFX_12_0_0__SRCID__SDMA_PAGE_TIMEOUT: u32 = 58; // 0x3A Page retry  timeout after UTCL2 return nack = 1
pub const GFX_12_0_0__SRCID__SDMA_PAGE_NULL: u32 = 59; // 0x3B Page Null from UTCL2 when nack = 2
pub const GFX_12_0_0__SRCID__SDMA_PAGE_FAULT: u32 = 60; // 0x3C Page Fault Error from UTCL2 when nack = 3
pub const GFX_12_0_0__SRCID__SDMA_VM_HOLE: u32 = 61; // 0x3D MC or SEM address in VM hole
pub const GFX_12_0_0__SRCID__SDMA_ECC: u32 = 62; // 0x3E ECC Error
pub const GFX_12_0_0__SRCID__SDMA_FROZEN: u32 = 63; // 0x3F SDMA Frozen
pub const GFX_12_0_0__SRCID__SDMA_SRAM_ECC: u32 = 64; // 0x40 SRAM ECC Error
pub const GFX_12_0_0__SRCID__SDMA_SEM_INCOMPLETE_TIMEOUT: u32 = 65; // 0x41 GPF(Sem incomplete timeout)
pub const GFX_12_0_0__SRCID__SDMA_SEM_WAIT_FAIL_TIMEOUT: u32 = 66; // 0x42 Semaphore wait fail timeout
pub const GFX_12_0_0__SRCID__SDMA_FENCE: u32 = 70; // 0x46 User fence
pub const GFX_12_0_0__SRCID__RLC_GC_FED_INTERRUPT: u32 = 128; // 0x80 FED Interrupt (for data poisoning)
pub const GFX_12_0_0__SRCID__CP_GENERIC_INT: u32 = 177; // 0xB1 CP_GENERIC int
pub const GFX_12_0_0__SRCID__CP_PM4_PKT_RSVD_BIT_ERROR: u32 = 180; // 0xB4 PM4 Pkt Rsvd Bits Error
pub const GFX_12_0_0__SRCID__CP_EOP_INTERRUPT: u32 = 181; // 0xB5 End-of-Pipe Interrupt
pub const GFX_12_0_0__SRCID__CP_BAD_OPCODE_ERROR: u32 = 183; // 0xB7 Bad Opcode Error
pub const GFX_12_0_0__SRCID__CP_PRIV_REG_FAULT: u32 = 184; // 0xB8 Privileged Register Fault
pub const GFX_12_0_0__SRCID__CP_PRIV_INSTR_FAULT: u32 = 185; // 0xB9 Privileged Instr Fault
pub const GFX_12_0_0__SRCID__CP_WAIT_MEM_SEM_FAULT: u32 = 186; // 0xBA Wait Memory Semaphore Fault (Sync Object Fault)
pub const GFX_12_0_0__SRCID__CP_CTX_EMPTY_INTERRUPT: u32 = 187; // 0xBB Context Empty Interrupt
pub const GFX_12_0_0__SRCID__CP_CTX_BUSY_INTERRUPT: u32 = 188; // 0xBC Context Busy Interrupt
pub const GFX_12_0_0__SRCID__CP_ME_WAIT_REG_MEM_POLL_TIMEOUT: u32 = 192; // 0xC0 CP.ME Wait_Reg_Mem Poll Timeout
pub const GFX_12_0_0__SRCID__CP_SIG_INCOMPLETE: u32 = 193; // 0xC1 "Surface Probe Fault Signal Incomplete"
pub const GFX_12_0_0__SRCID__CP_PREEMPT_ACK: u32 = 194; // 0xC2 Preemption Ack-wledge
pub const GFX_12_0_0__SRCID__CP_GPF: u32 = 195; // 0xC3 General Protection Fault (GPF)
pub const GFX_12_0_0__SRCID__CP_GDS_ALLOC_ERROR: u32 = 196; // 0xC4 GDS Alloc Error
pub const GFX_12_0_0__SRCID__CP_ECC_ERROR: u32 = 197; // 0xC5 ECC  Error
pub const GFX_12_0_0__SRCID__CP_COMPUTE_QUERY_STATUS: u32 = 199; // 0xC7 Compute query status
pub const GFX_12_0_0__SRCID__CP_VM_DOORBELL: u32 = 200; // 0xC8 Unattached VM Doorbell Received
pub const GFX_12_0_0__SRCID__CP_FUE_ERROR: u32 = 201; // 0xC9 ECC FUE Error
pub const GFX_12_0_0__SRCID__RLC_STRM_PERF_MONITOR_INTERRUPT: u32 = 202; // 0xCA Streaming Perf Monitor Interrupt
pub const GFX_12_0_0__SRCID__GRBM_RD_TIMEOUT_ERROR: u32 = 232; // 0xE8 CRead timeout error
pub const GFX_12_0_0__SRCID__GRBM_REG_GUI_IDLE: u32 = 233; // 0xE9 Register GUI Idle
pub const GFX_12_0_0__SRCID__SQ_INTERRUPT_ID: u32 = 239; // 0xEF SQ Interrupt (ttrace wrap, errors)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
