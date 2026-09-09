/*
 * Copyright 2014 Advanced Micro Devices, Inc.
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

// Dependency equivalent of: #include <linux/types.h>

#[repr(C)]
pub struct cik_ih_ring_entry {
    pub source_id: u32,
    pub data: u32,
    pub ring_id: u32,
    pub reserved: u32,
}

pub const CIK_INTSRC_CP_END_OF_PIPE: u32 = 0xB5;
pub const CIK_INTSRC_CP_BAD_OPCODE: u32 = 0xB7;
pub const CIK_INTSRC_SDMA_TRAP: u32 = 0xE0;
pub const CIK_INTSRC_SQ_INTERRUPT_MSG: u32 = 0xEF;
pub const CIK_INTSRC_GFX_PAGE_INV_FAULT: u32 = 0x92;
pub const CIK_INTSRC_GFX_MEM_PROT_FAULT: u32 = 0x93;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
