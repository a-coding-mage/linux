/*
 * Copyright 2017 Advanced Micro Devices, Inc.
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

pub const SDMA0_4_0__SRCID__SDMA_ATOMIC_RTN_DONE: i32 = 217; // 0xD9 SDMA atomic*_rtn ops complete
pub const SDMA0_4_0__SRCID__SDMA_ATOMIC_TIMEOUT: i32 = 218; // 0xDA SDMA atomic CMPSWAP loop timeout
pub const SDMA0_4_0__SRCID__SDMA_IB_PREEMPT: i32 = 219; // 0xDB sdma mid-command buffer preempt interrupt
pub const SDMA0_4_0__SRCID__SDMA_ECC: i32 = 220; // 0xDC ECC  Error
pub const SDMA0_4_0__SRCID__SDMA_PAGE_FAULT: i32 = 221; // 0xDD Page Fault Error from UTCL2 when nack=3
pub const SDMA0_4_0__SRCID__SDMA_PAGE_NULL: i32 = 222; // 0xDE Page Null from UTCL2 when nack=2
pub const SDMA0_4_0__SRCID__SDMA_XNACK: i32 = 223; // 0xDF Page retry  timeout after UTCL2 return nack=1
pub const SDMA0_4_0__SRCID__SDMA_TRAP: i32 = 224; // 0xE0 Trap
pub const SDMA0_4_0__SRCID__SDMA_SEM_INCOMPLETE_TIMEOUT: i32 = 225; // 0xE1 0xDAGPF (Sem incomplete timeout)
pub const SDMA0_4_0__SRCID__SDMA_SEM_WAIT_FAIL_TIMEOUT: i32 = 226; // 0xE2 Semaphore wait fail timeout
pub const SDMA0_4_0__SRCID__SDMA_SRAM_ECC: i32 = 228; // 0xE4 SRAM ECC Error
pub const SDMA0_4_0__SRCID__SDMA_PREEMPT: i32 = 240; // 0xF0 SDMA New Run List
pub const SDMA0_4_0__SRCID__SDMA_VM_HOLE: i32 = 242; // 0xF2 MC or SEM address in VM hole
pub const SDMA0_4_0__SRCID__SDMA_CTXEMPTY: i32 = 243; // 0xF3 Context Empty
pub const SDMA0_4_0__SRCID__SDMA_DOORBELL_INVALID: i32 = 244; // 0xF4 Doorbell BE invalid
pub const SDMA0_4_0__SRCID__SDMA_FROZEN: i32 = 245; // 0xF5 SDMA Frozen
pub const SDMA0_4_0__SRCID__SDMA_POLL_TIMEOUT: i32 = 246; // 0xF6 SRBM read poll timeout
pub const SDMA0_4_0__SRCID__SDMA_SRBMWRITE: i32 = 247; // 0xF7 SRBM write Protection

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
