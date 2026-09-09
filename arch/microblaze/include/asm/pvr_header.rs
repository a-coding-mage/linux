/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Support for the MicroBlaze PVR (Processor Version Register)
 *
 * Copyright (C) 2009 - 2011 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2007 John Williams <john.williams@petalogix.com>
 * Copyright (C) 2007 - 2011 PetaLogix
 */

pub const PVR_MSR_BIT: u32 = 0x400;

#[repr(C)]
pub struct pvr_s {
    pub pvr: [u32; 12],
}

pub const PVR0_PVR_FULL_MASK: u32 = 0x80000000;
pub const PVR0_USE_BARREL_MASK: u32 = 0x40000000;
pub const PVR0_USE_DIV_MASK: u32 = 0x20000000;
pub const PVR0_USE_HW_MUL_MASK: u32 = 0x10000000;
pub const PVR0_USE_FPU_MASK: u32 = 0x08000000;
pub const PVR0_USE_EXC_MASK: u32 = 0x04000000;
pub const PVR0_USE_ICACHE_MASK: u32 = 0x02000000;
pub const PVR0_USE_DCACHE_MASK: u32 = 0x01000000;
pub const PVR0_USE_MMU: u32 = 0x00800000;
pub const PVR0_USE_BTC: u32 = 0x00400000;
pub const PVR0_ENDI: u32 = 0x00200000;
pub const PVR0_VERSION_MASK: u32 = 0x0000FF00;
pub const PVR0_USER1_MASK: u32 = 0x000000FF;
pub const PVR1_USER2_MASK: u32 = 0xFFFFFFFF;

pub const PVR2_D_OPB_MASK: u32 = 0x80000000;
pub const PVR2_D_LMB_MASK: u32 = 0x40000000;
pub const PVR2_I_OPB_MASK: u32 = 0x20000000;
pub const PVR2_I_LMB_MASK: u32 = 0x10000000;
pub const PVR2_INTERRUPT_IS_EDGE_MASK: u32 = 0x08000000;
pub const PVR2_EDGE_IS_POSITIVE_MASK: u32 = 0x04000000;
pub const PVR2_D_PLB_MASK: u32 = 0x02000000;
pub const PVR2_I_PLB_MASK: u32 = 0x01000000;
pub const PVR2_INTERCONNECT: u32 = 0x00800000;
pub const PVR2_USE_EXTEND_FSL: u32 = 0x00080000;
pub const PVR2_USE_FSL_EXC: u32 = 0x00040000;
pub const PVR2_USE_MSR_INSTR: u32 = 0x00020000;
pub const PVR2_USE_PCMP_INSTR: u32 = 0x00010000;
pub const PVR2_AREA_OPTIMISED: u32 = 0x00008000;
pub const PVR2_USE_BARREL_MASK: u32 = 0x00004000;
pub const PVR2_USE_DIV_MASK: u32 = 0x00002000;
pub const PVR2_USE_HW_MUL_MASK: u32 = 0x00001000;
pub const PVR2_USE_FPU_MASK: u32 = 0x00000800;
pub const PVR2_USE_MUL64_MASK: u32 = 0x00000400;
pub const PVR2_USE_FPU2_MASK: u32 = 0x00000200;
pub const PVR2_USE_IPLBEXC: u32 = 0x00000100;
pub const PVR2_USE_DPLBEXC: u32 = 0x00000080;
pub const PVR2_OPCODE_0x0_ILL_MASK: u32 = 0x00000040;
pub const PVR2_UNALIGNED_EXC_MASK: u32 = 0x00000020;
pub const PVR2_ILL_OPCODE_EXC_MASK: u32 = 0x00000010;
pub const PVR2_IOPB_BUS_EXC_MASK: u32 = 0x00000008;
pub const PVR2_DOPB_BUS_EXC_MASK: u32 = 0x00000004;
pub const PVR2_DIV_ZERO_EXC_MASK: u32 = 0x00000002;
pub const PVR2_FPU_EXC_MASK: u32 = 0x00000001;

pub const PVR3_DEBUG_ENABLED_MASK: u32 = 0x80000000;
pub const PVR3_NUMBER_OF_PC_BRK_MASK: u32 = 0x1E000000;
pub const PVR3_NUMBER_OF_RD_ADDR_BRK_MASK: u32 = 0x00380000;
pub const PVR3_NUMBER_OF_WR_ADDR_BRK_MASK: u32 = 0x0000E000;
pub const PVR3_FSL_LINKS_MASK: u32 = 0x00000380;
pub const PVR4_USE_ICACHE_MASK: u32 = 0x80000000;
pub const PVR4_ICACHE_ADDR_TAG_BITS_MASK: u32 = 0x7C000000;
pub const PVR4_ICACHE_ALLOW_WR_MASK: u32 = 0x01000000;
pub const PVR4_ICACHE_LINE_LEN_MASK: u32 = 0x00E00000;
pub const PVR4_ICACHE_BYTE_SIZE_MASK: u32 = 0x001F0000;
pub const PVR4_ICACHE_ALWAYS_USED: u32 = 0x00008000;
pub const PVR4_ICACHE_INTERFACE: u32 = 0x00002000;
pub const PVR5_USE_DCACHE_MASK: u32 = 0x80000000;
pub const PVR5_DCACHE_ADDR_TAG_BITS_MASK: u32 = 0x7C000000;
pub const PVR5_DCACHE_ALLOW_WR_MASK: u32 = 0x01000000;
pub const PVR5_DCACHE_LINE_LEN_MASK: u32 = 0x00E00000;
pub const PVR5_DCACHE_BYTE_SIZE_MASK: u32 = 0x001F0000;
pub const PVR5_DCACHE_ALWAYS_USED: u32 = 0x00008000;
pub const PVR5_DCACHE_USE_WRITEBACK: u32 = 0x00004000;
pub const PVR5_DCACHE_INTERFACE: u32 = 0x00002000;
pub const PVR6_ICACHE_BASEADDR_MASK: u32 = 0xFFFFFFFF;
pub const PVR7_ICACHE_HIGHADDR_MASK: u32 = 0xFFFFFFFF;
pub const PVR8_DCACHE_BASEADDR_MASK: u32 = 0xFFFFFFFF;
pub const PVR9_DCACHE_HIGHADDR_MASK: u32 = 0xFFFFFFFF;
pub const PVR10_TARGET_FAMILY_MASK: u32 = 0xFF000000;
pub const PVR11_USE_MMU: u32 = 0xC0000000;
pub const PVR11_MMU_ITLB_SIZE: u32 = 0x38000000;
pub const PVR11_MMU_DTLB_SIZE: u32 = 0x07000000;
pub const PVR11_MMU_TLB_ACCESS: u32 = 0x00C00000;
pub const PVR11_MMU_ZONES: u32 = 0x003C0000;
pub const PVR11_MMU_PRIVINS: u32 = 0x00010000;
pub const PVR11_MSR_RESET_VALUE_MASK: u32 = 0x000007FF;

/* PVR access macros. */
macro_rules! pvr_access { ($name:ident, $index:expr, $expr:expr) => { #[macro_export] macro_rules! $name { ($pvr:expr) => { (($pvr).pvr[$index] $expr) }; } }; }

pub const PVR4_ICACHE_USE_FSL_MASK: u32 = 0;
pub const PVR5_DCACHE_USE_FSL_MASK: u32 = 0;

macro_rules! pvr_expr { ($name:ident, $body:expr) => { #[macro_export] macro_rules! $name { ($pvr:expr) => { $body }; } }; }

pvr_expr!(PVR_IS_FULL, (($pvr).pvr[0] & PVR0_PVR_FULL_MASK));
pvr_expr!(PVR_USE_BARREL, (($pvr).pvr[0] & PVR0_USE_BARREL_MASK));
pvr_expr!(PVR_USE_DIV, (($pvr).pvr[0] & PVR0_USE_DIV_MASK));
pvr_expr!(PVR_USE_HW_MUL, (($pvr).pvr[0] & PVR0_USE_HW_MUL_MASK));
pvr_expr!(PVR_USE_FPU, (($pvr).pvr[0] & PVR0_USE_FPU_MASK));
pvr_expr!(PVR_USE_FPU2, (($pvr).pvr[2] & PVR2_USE_FPU2_MASK));
pvr_expr!(PVR_USE_ICACHE, (($pvr).pvr[0] & PVR0_USE_ICACHE_MASK));
pvr_expr!(PVR_USE_DCACHE, (($pvr).pvr[0] & PVR0_USE_DCACHE_MASK));
pvr_expr!(PVR_VERSION, ((($pvr).pvr[0] & PVR0_VERSION_MASK) >> 8));
pvr_expr!(PVR_USER1, (($pvr).pvr[0] & PVR0_USER1_MASK));
pvr_expr!(PVR_USER2, (($pvr).pvr[1] & PVR1_USER2_MASK));

macro_rules! pvr2 { ($name:ident, $mask:ident) => { pvr_expr!($name, (($pvr).pvr[2] & $mask)); }; }
pvr2!(PVR_D_OPB, PVR2_D_OPB_MASK); pvr2!(PVR_D_LMB, PVR2_D_LMB_MASK); pvr2!(PVR_I_OPB, PVR2_I_OPB_MASK); pvr2!(PVR_I_LMB, PVR2_I_LMB_MASK);
pvr2!(PVR_INTERRUPT_IS_EDGE, PVR2_INTERRUPT_IS_EDGE_MASK); pvr2!(PVR_EDGE_IS_POSITIVE, PVR2_EDGE_IS_POSITIVE_MASK); pvr2!(PVR_USE_MSR_INSTR, PVR2_USE_MSR_INSTR); pvr2!(PVR_USE_PCMP_INSTR, PVR2_USE_PCMP_INSTR); pvr2!(PVR_AREA_OPTIMISED, PVR2_AREA_OPTIMISED); pvr2!(PVR_USE_MUL64, PVR2_USE_MUL64_MASK); pvr2!(PVR_OPCODE_0x0_ILLEGAL, PVR2_OPCODE_0x0_ILL_MASK); pvr2!(PVR_UNALIGNED_EXCEPTION, PVR2_UNALIGNED_EXC_MASK); pvr2!(PVR_ILL_OPCODE_EXCEPTION, PVR2_ILL_OPCODE_EXC_MASK); pvr2!(PVR_IOPB_BUS_EXCEPTION, PVR2_IOPB_BUS_EXC_MASK); pvr2!(PVR_DOPB_BUS_EXCEPTION, PVR2_DOPB_BUS_EXC_MASK); pvr2!(PVR_DIV_ZERO_EXCEPTION, PVR2_DIV_ZERO_EXC_MASK); pvr2!(PVR_FPU_EXCEPTION, PVR2_FPU_EXC_MASK); pvr2!(PVR_FSL_EXCEPTION, PVR2_USE_EXTEND_FSL);

pvr_expr!(PVR_DEBUG_ENABLED, (($pvr).pvr[3] & PVR3_DEBUG_ENABLED_MASK));
pvr_expr!(PVR_NUMBER_OF_PC_BRK, ((($pvr).pvr[3] & PVR3_NUMBER_OF_PC_BRK_MASK) >> 25));
pvr_expr!(PVR_NUMBER_OF_RD_ADDR_BRK, ((($pvr).pvr[3] & PVR3_NUMBER_OF_RD_ADDR_BRK_MASK) >> 19));
pvr_expr!(PVR_NUMBER_OF_WR_ADDR_BRK, ((($pvr).pvr[3] & PVR3_NUMBER_OF_WR_ADDR_BRK_MASK) >> 13));
pvr_expr!(PVR_FSL_LINKS, ((($pvr).pvr[3] & PVR3_FSL_LINKS_MASK) >> 7));
pvr_expr!(PVR_ICACHE_ADDR_TAG_BITS, ((($pvr).pvr[4] & PVR4_ICACHE_ADDR_TAG_BITS_MASK) >> 26));
pvr_expr!(PVR_ICACHE_USE_FSL, (($pvr).pvr[4] & PVR4_ICACHE_USE_FSL_MASK));
pvr_expr!(PVR_ICACHE_ALLOW_WR, (($pvr).pvr[4] & PVR4_ICACHE_ALLOW_WR_MASK));
pvr_expr!(PVR_ICACHE_LINE_LEN, (1u32 << ((($pvr).pvr[4] & PVR4_ICACHE_LINE_LEN_MASK) >> 21)));
pvr_expr!(PVR_ICACHE_BYTE_SIZE, (1u32 << ((($pvr).pvr[4] & PVR4_ICACHE_BYTE_SIZE_MASK) >> 16)));
pvr_expr!(PVR_DCACHE_ADDR_TAG_BITS, ((($pvr).pvr[5] & PVR5_DCACHE_ADDR_TAG_BITS_MASK) >> 26));
pvr_expr!(PVR_DCACHE_USE_FSL, (($pvr).pvr[5] & PVR5_DCACHE_USE_FSL_MASK));
pvr_expr!(PVR_DCACHE_ALLOW_WR, (($pvr).pvr[5] & PVR5_DCACHE_ALLOW_WR_MASK));
pvr_expr!(PVR_DCACHE_LINE_LEN, (1u32 << ((($pvr).pvr[5] & PVR5_DCACHE_LINE_LEN_MASK) >> 21)));
pvr_expr!(PVR_DCACHE_BYTE_SIZE, (1u32 << ((($pvr).pvr[5] & PVR5_DCACHE_BYTE_SIZE_MASK) >> 16)));
pvr_expr!(PVR_DCACHE_USE_WRITEBACK, ((($pvr).pvr[5] & PVR5_DCACHE_USE_WRITEBACK) >> 14));
pvr_expr!(PVR_ICACHE_BASEADDR, (($pvr).pvr[6] & PVR6_ICACHE_BASEADDR_MASK));
pvr_expr!(PVR_ICACHE_HIGHADDR, (($pvr).pvr[7] & PVR7_ICACHE_HIGHADDR_MASK));
pvr_expr!(PVR_DCACHE_BASEADDR, (($pvr).pvr[8] & PVR8_DCACHE_BASEADDR_MASK));
pvr_expr!(PVR_DCACHE_HIGHADDR, (($pvr).pvr[9] & PVR9_DCACHE_HIGHADDR_MASK));
pvr_expr!(PVR_TARGET_FAMILY, ((($pvr).pvr[10] & PVR10_TARGET_FAMILY_MASK) >> 24));
pvr_expr!(PVR_MSR_RESET_VALUE, (($pvr).pvr[11] & PVR11_MSR_RESET_VALUE_MASK));
pvr_expr!(PVR_USE_MMU, ((($pvr).pvr[11] & PVR11_USE_MMU) >> 30));
pvr_expr!(PVR_MMU_ITLB_SIZE, (($pvr).pvr[11] & PVR11_MMU_ITLB_SIZE));
pvr_expr!(PVR_MMU_DTLB_SIZE, (($pvr).pvr[11] & PVR11_MMU_DTLB_SIZE));
pvr_expr!(PVR_MMU_TLB_ACCESS, (($pvr).pvr[11] & PVR11_MMU_TLB_ACCESS));
pvr_expr!(PVR_MMU_ZONES, (($pvr).pvr[11] & PVR11_MMU_ZONES));
pvr_expr!(PVR_MMU_PRIVINS, (($pvr).pvr[11] & PVR11_MMU_PRIVINS));
pvr_expr!(PVR_ENDIAN, (($pvr).pvr[0] & PVR0_ENDI));

unsafe extern "C" {
    pub fn cpu_has_pvr() -> ::core::ffi::c_int;
    pub fn get_pvr(pvr: *mut pvr_s);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
