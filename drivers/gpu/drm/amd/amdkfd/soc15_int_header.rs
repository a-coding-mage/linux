/*
 * Copyright 2016-2018 Advanced Micro Devices, Inc.
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

// Dependency supplied by the source header: soc15_ih_clientid.h

pub const SOC15_INTSRC_CP_END_OF_PIPE: u32 = 181;
pub const SOC15_INTSRC_CP_BAD_OPCODE: u32 = 183;
pub const SOC15_INTSRC_SQ_INTERRUPT_MSG: u32 = 239;
pub const SOC15_INTSRC_VMC_FAULT: u32 = 0;
pub const SOC15_INTSRC_VMC_UTCL2_POISON: u32 = 1;
pub const SOC15_INTSRC_SDMA_TRAP: u32 = 224;
pub const SOC15_INTSRC_SDMA_ECC: u32 = 220;
pub const SOC21_INTSRC_SDMA_TRAP: u32 = 49;
pub const SOC21_INTSRC_SDMA_ECC: u32 = 62;

#[macro_export]
macro_rules! SOC15_CLIENT_ID_FROM_IH_ENTRY {
    ($entry:expr) => { (le32_to_cpu($entry[0]) & 0xff) };
}

#[macro_export]
macro_rules! SOC15_SOURCE_ID_FROM_IH_ENTRY {
    ($entry:expr) => { ((le32_to_cpu($entry[0]) >> 8) & 0xff) };
}

#[macro_export]
macro_rules! SOC15_RING_ID_FROM_IH_ENTRY {
    ($entry:expr) => { ((le32_to_cpu($entry[0]) >> 16) & 0xff) };
}

#[macro_export]
macro_rules! SOC15_VMID_FROM_IH_ENTRY {
    ($entry:expr) => { ((le32_to_cpu($entry[0]) >> 24) & 0xf) };
}

#[macro_export]
macro_rules! SOC15_VMID_TYPE_FROM_IH_ENTRY {
    ($entry:expr) => { ((le32_to_cpu($entry[0]) >> 31) & 0x1) };
}

#[macro_export]
macro_rules! SOC15_PASID_FROM_IH_ENTRY {
    ($entry:expr) => { (le32_to_cpu($entry[3]) & 0xffff) };
}

#[macro_export]
macro_rules! SOC15_NODEID_FROM_IH_ENTRY {
    ($entry:expr) => { ((le32_to_cpu($entry[3]) >> 16) & 0xff) };
}

#[macro_export]
macro_rules! SOC15_CONTEXT_ID0_FROM_IH_ENTRY {
    ($entry:expr) => { le32_to_cpu($entry[4]) };
}

#[macro_export]
macro_rules! SOC15_CONTEXT_ID1_FROM_IH_ENTRY {
    ($entry:expr) => { le32_to_cpu($entry[5]) };
}

#[macro_export]
macro_rules! SOC15_CONTEXT_ID2_FROM_IH_ENTRY {
    ($entry:expr) => { le32_to_cpu($entry[6]) };
}

#[macro_export]
macro_rules! SOC15_CONTEXT_ID3_FROM_IH_ENTRY {
    ($entry:expr) => { le32_to_cpu($entry[7]) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
