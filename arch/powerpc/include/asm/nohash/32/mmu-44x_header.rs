/* SPDX-License-Identifier: GPL-2.0 */
/* PPC440 support */

pub const PPC44x_MMUCR_TID: u32 = 0x000000ff;
pub const PPC44x_MMUCR_STS: u32 = 0x00010000;

pub const PPC44x_TLB_PAGEID: u32 = 0;
pub const PPC44x_TLB_XLAT: u32 = 1;
pub const PPC44x_TLB_ATTRIB: u32 = 2;

/* Page identification fields */
pub const PPC44x_TLB_EPN_MASK: u32 = 0xfffffc00; /* Effective Page Number */
pub const PPC44x_TLB_VALID: u32 = 0x00000200; /* Valid flag */
pub const PPC44x_TLB_TS: u32 = 0x00000100; /* Translation address space */
pub const PPC44x_TLB_1K: u32 = 0x00000000; /* Page sizes */
pub const PPC44x_TLB_4K: u32 = 0x00000010;
pub const PPC44x_TLB_16K: u32 = 0x00000020;
pub const PPC44x_TLB_64K: u32 = 0x00000030;
pub const PPC44x_TLB_256K: u32 = 0x00000040;
pub const PPC44x_TLB_1M: u32 = 0x00000050;
pub const PPC44x_TLB_16M: u32 = 0x00000070;
pub const PPC44x_TLB_256M: u32 = 0x00000090;

/* Translation fields */
pub const PPC44x_TLB_RPN_MASK: u32 = 0xfffffc00; /* Real Page Number */
pub const PPC44x_TLB_ERPN_MASK: u32 = 0x0000000f;

/* Storage attribute and access control fields */
pub const PPC44x_TLB_ATTR_MASK: u32 = 0x0000ff80;
pub const PPC44x_TLB_U0: u32 = 0x00008000; /* User 0 */
pub const PPC44x_TLB_U1: u32 = 0x00004000; /* User 1 */
pub const PPC44x_TLB_U2: u32 = 0x00002000; /* User 2 */
pub const PPC44x_TLB_U3: u32 = 0x00001000; /* User 3 */
pub const PPC44x_TLB_W: u32 = 0x00000800; /* Caching is write-through */
pub const PPC44x_TLB_I: u32 = 0x00000400; /* Caching is inhibited */
pub const PPC44x_TLB_M: u32 = 0x00000200; /* Memory is coherent */
pub const PPC44x_TLB_G: u32 = 0x00000100; /* Memory is guarded */
pub const PPC44x_TLB_E: u32 = 0x00000080; /* Memory is little endian */
pub const PPC44x_TLB_PERM_MASK: u32 = 0x0000003f;
pub const PPC44x_TLB_UX: u32 = 0x00000020; /* User execution */
pub const PPC44x_TLB_UW: u32 = 0x00000010; /* User write */
pub const PPC44x_TLB_UR: u32 = 0x00000008; /* User read */
pub const PPC44x_TLB_SX: u32 = 0x00000004; /* Super execution */
pub const PPC44x_TLB_SW: u32 = 0x00000002; /* Super write */
pub const PPC44x_TLB_SR: u32 = 0x00000001; /* Super read */
pub const PPC44x_TLB_SIZE: u32 = 64;

/* 47x bits */
pub const PPC47x_MMUCR_TID: u32 = 0x0000ffff;
pub const PPC47x_MMUCR_STS: u32 = 0x00010000;
pub const PPC47x_TLB0_EPN_MASK: u32 = 0xfffff000; /* Effective Page Number */
pub const PPC47x_TLB0_VALID: u32 = 0x00000800; /* Valid flag */
pub const PPC47x_TLB0_TS: u32 = 0x00000400; /* Translation address space */
pub const PPC47x_TLB0_4K: u32 = 0x00000000;
pub const PPC47x_TLB0_16K: u32 = 0x00000010;
pub const PPC47x_TLB0_64K: u32 = 0x00000030;
pub const PPC47x_TLB0_1M: u32 = 0x00000070;
pub const PPC47x_TLB0_16M: u32 = 0x000000f0;
pub const PPC47x_TLB0_256M: u32 = 0x000001f0;
pub const PPC47x_TLB0_1G: u32 = 0x000003f0;
pub const PPC47x_TLB0_BOLTED_R: u32 = 0x00000008; /* tlbre only */
pub const PPC47x_TLB1_RPN_MASK: u32 = 0xfffff000; /* Real Page Number */
pub const PPC47x_TLB1_ERPN_MASK: u32 = 0x000003ff;
pub const PPC47x_TLB2_ATTR_MASK: u32 = 0x0003ff80;
pub const PPC47x_TLB2_IL1I: u32 = 0x00020000; /* Memory is guarded */
pub const PPC47x_TLB2_IL1D: u32 = 0x00010000; /* Memory is guarded */
pub const PPC47x_TLB2_U0: u32 = 0x00008000; /* User 0 */
pub const PPC47x_TLB2_U1: u32 = 0x00004000; /* User 1 */
pub const PPC47x_TLB2_U2: u32 = 0x00002000; /* User 2 */
pub const PPC47x_TLB2_U3: u32 = 0x00001000; /* User 3 */
pub const PPC47x_TLB2_W: u32 = 0x00000800; /* Caching is write-through */
pub const PPC47x_TLB2_I: u32 = 0x00000400; /* Caching is inhibited */
pub const PPC47x_TLB2_M: u32 = 0x00000200; /* Memory is coherent */
pub const PPC47x_TLB2_G: u32 = 0x00000100; /* Memory is guarded */
pub const PPC47x_TLB2_E: u32 = 0x00000080; /* Memory is little endian */
pub const PPC47x_TLB2_PERM_MASK: u32 = 0x0000003f;
pub const PPC47x_TLB2_UX: u32 = 0x00000020; /* User execution */
pub const PPC47x_TLB2_UW: u32 = 0x00000010; /* User write */
pub const PPC47x_TLB2_UR: u32 = 0x00000008; /* User read */
pub const PPC47x_TLB2_SX: u32 = 0x00000004; /* Super execution */
pub const PPC47x_TLB2_SW: u32 = 0x00000002; /* Super write */
pub const PPC47x_TLB2_SR: u32 = 0x00000001; /* Super read */
pub const PPC47x_TLB2_U_RWX: u32 = PPC47x_TLB2_UX | PPC47x_TLB2_UW | PPC47x_TLB2_UR;
pub const PPC47x_TLB2_S_RWX: u32 = PPC47x_TLB2_SX | PPC47x_TLB2_SW | PPC47x_TLB2_SR;
pub const PPC47x_TLB2_S_RW: u32 = PPC47x_TLB2_SW | PPC47x_TLB2_SR;
pub const PPC47x_TLB2_IMG: u32 = PPC47x_TLB2_I | PPC47x_TLB2_M | PPC47x_TLB2_G;

extern "C" {
    pub static mut tlb_44x_hwater: u32;
    pub static mut tlb_44x_index: u32;
    pub static mut patch__tlb_44x_hwater_D: i32;
    pub static mut patch__tlb_44x_hwater_I: i32;
}

#[repr(C)]
pub struct mm_context_t {
    pub id: u32,
    pub active: u32,
    pub vdso: *mut core::ffi::c_void,
}

/* Build-time configuration conditionals from the C header are preserved below. */
#[cfg(not(feature = "CONFIG_PPC_EARLY_DEBUG_44x"))]
pub const PPC44x_EARLY_TLBS: u32 = 1;
#[cfg(feature = "CONFIG_PPC_EARLY_DEBUG_44x")]
pub const PPC44x_EARLY_TLBS: u32 = 2;
#[cfg(feature = "CONFIG_PPC_EARLY_DEBUG_44x")]
pub const PPC44x_EARLY_DEBUG_VIRTADDR: u32 = 0xf0000000 | (0u32 & 0xffff);

pub const PPC_PIN_SIZE: u32 = 1 << 28; /* 256M */

/* Page-size selection is build-time configuration; the original alternatives are retained. */
#[cfg(feature = "CONFIG_PPC_4K_PAGES")]
pub const PPC44x_TLBE_SIZE: u32 = PPC44x_TLB_4K;
#[cfg(feature = "CONFIG_PPC_4K_PAGES")]
pub const PPC47x_TLBE_SIZE: u32 = PPC47x_TLB0_4K;
#[cfg(feature = "CONFIG_PPC_4K_PAGES")]
pub const mmu_virtual_psize: u32 = MMU_PAGE_4K;
#[cfg(feature = "CONFIG_PPC_16K_PAGES")]
pub const PPC44x_TLBE_SIZE: u32 = PPC44x_TLB_16K;
#[cfg(feature = "CONFIG_PPC_16K_PAGES")]
pub const PPC47x_TLBE_SIZE: u32 = PPC47x_TLB0_16K;
#[cfg(feature = "CONFIG_PPC_16K_PAGES")]
pub const mmu_virtual_psize: u32 = MMU_PAGE_16K;
#[cfg(feature = "CONFIG_PPC_64K_PAGES")]
pub const PPC44x_TLBE_SIZE: u32 = PPC44x_TLB_64K;
#[cfg(feature = "CONFIG_PPC_64K_PAGES")]
pub const PPC47x_TLBE_SIZE: u32 = PPC47x_TLB0_64K;
#[cfg(feature = "CONFIG_PPC_64K_PAGES")]
pub const mmu_virtual_psize: u32 = MMU_PAGE_64K;
#[cfg(feature = "CONFIG_PPC_256K_PAGES")]
pub const PPC44x_TLBE_SIZE: u32 = PPC44x_TLB_256K;
#[cfg(feature = "CONFIG_PPC_256K_PAGES")]
pub const mmu_virtual_psize: u32 = MMU_PAGE_256K;

pub const mmu_linear_psize: u32 = MMU_PAGE_256M;
pub const PPC44x_PGD_OFF_SHIFT: u32 = 32 - PGDIR_SHIFT + PGD_T_LOG2;
pub const PPC44x_PGD_OFF_MASK_BIT: u32 = PGDIR_SHIFT - PGD_T_LOG2;
pub const PPC44x_PTE_ADD_SHIFT: u32 = 32 - PGDIR_SHIFT + PTE_SHIFT + PTE_T_LOG2;
pub const PPC44x_PTE_ADD_MASK_BIT: u32 = 32 - PTE_T_LOG2 - PTE_SHIFT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
