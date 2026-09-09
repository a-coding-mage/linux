/* SPDX-License-Identifier: GPL-2.0-only */
/* Register bitfield descriptions for Pondicherry2 memory controller. */

// C bitfields are represented by their underlying register word.  The masks
// below preserve the source field layout and permit the same raw operations.
pub type u32_t = u32;
pub type u64_t = u64;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct b_cr_touud_lo_pci { pub value: u32 }
pub const b_cr_touud_lo_pci_port: u32 = 0x4c;
pub const b_cr_touud_lo_pci_offset: u32 = 0xa8;
pub const b_cr_touud_lo_pci_r_opcode: u32 = 0x04;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct b_cr_touud_hi_pci { pub value: u32 }
pub const b_cr_touud_hi_pci_port: u32 = 0x4c;
pub const b_cr_touud_hi_pci_offset: u32 = 0xac;
pub const b_cr_touud_hi_pci_r_opcode: u32 = 0x04;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct b_cr_tolud_pci { pub value: u32 }
pub const b_cr_tolud_pci_port: u32 = 0x4c;
pub const b_cr_tolud_pci_offset: u32 = 0xbc;
pub const b_cr_tolud_pci_r_opcode: u32 = 0x04;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct b_cr_mchbar_lo_pci { pub value: u32 }
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct b_cr_mchbar_hi_pci { pub value: u32 }

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct b_cr_slice_channel_hash { pub value: u64 }
pub const b_cr_slice_channel_hash_port: u32 = 0x4c;
pub const b_cr_slice_channel_hash_offset: u32 = 0x4c58;
pub const b_cr_slice_channel_hash_r_opcode: u32 = 0x06;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct b_cr_mot_out_base_mchbar { pub value: u32 }
pub const b_cr_mot_out_base_mchbar_port: u32 = 0x4c;
pub const b_cr_mot_out_base_mchbar_offset: u32 = 0x6af0;
pub const b_cr_mot_out_base_mchbar_r_opcode: u32 = 0x00;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct b_cr_mot_out_mask_mchbar { pub value: u32 }
pub const b_cr_mot_out_mask_mchbar_port: u32 = 0x4c;
pub const b_cr_mot_out_mask_mchbar_offset: u32 = 0x6af4;
pub const b_cr_mot_out_mask_mchbar_r_opcode: u32 = 0x00;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct b_cr_asym_mem_region0_mchbar { pub value: u32 }
pub const b_cr_asym_mem_region0_mchbar_port: u32 = 0x4c;
pub const b_cr_asym_mem_region0_mchbar_offset: u32 = 0x6e40;
pub const b_cr_asym_mem_region0_mchbar_r_opcode: u32 = 0x00;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct b_cr_asym_mem_region1_mchbar { pub value: u32 }
pub const b_cr_asym_mem_region1_mchbar_port: u32 = 0x4c;
pub const b_cr_asym_mem_region1_mchbar_offset: u32 = 0x6e44;
pub const b_cr_asym_mem_region1_mchbar_r_opcode: u32 = 0x00;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct b_cr_asym_mem_region_denverton { pub value: u32 }
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct b_cr_asym_2way_mem_region_mchbar { pub value: u32 }
pub const b_cr_asym_2way_mem_region_mchbar_port: u32 = 0x4c;
pub const b_cr_asym_2way_mem_region_mchbar_offset: u32 = 0x6e50;
pub const b_cr_asym_2way_mem_region_mchbar_r_opcode: u32 = 0x00;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct d_cr_drp0 { pub value: u32 }
pub const d_cr_drp0_offset: u32 = 0x1400;
pub const d_cr_drp0_r_opcode: u32 = 0x00;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct d_cr_dsch { pub value: u32 }
pub const d_cr_dsch_port: u32 = 0x16;
pub const d_cr_dsch_offset: u32 = 0x0;
pub const d_cr_dsch_r_opcode: u32 = 0x0;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct d_cr_ecc_ctrl { pub value: u32 }
pub const d_cr_ecc_ctrl_offset: u32 = 0x180;
pub const d_cr_ecc_ctrl_r_opcode: u32 = 0x0;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct d_cr_drp { pub value: u32 }
pub const d_cr_drp_offset: u32 = 0x158;
pub const d_cr_drp_r_opcode: u32 = 0x0;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct d_cr_dmap { pub value: u32 }
pub const d_cr_dmap_offset: u32 = 0x174;
pub const d_cr_dmap_r_opcode: u32 = 0x0;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct d_cr_dmap1 { pub value: u32 }
pub const d_cr_dmap1_offset: u32 = 0xb4;
pub const d_cr_dmap1_r_opcode: u32 = 0x0;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct d_cr_dmap2 { pub value: u32 }
pub const d_cr_dmap2_offset: u32 = 0x148;
pub const d_cr_dmap2_r_opcode: u32 = 0x0;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct d_cr_dmap3 { pub value: u32 }
pub const d_cr_dmap3_offset: u32 = 0x14c;
pub const d_cr_dmap3_r_opcode: u32 = 0x0;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct d_cr_dmap4 { pub value: u32 }
pub const d_cr_dmap4_offset: u32 = 0x150;
pub const d_cr_dmap4_r_opcode: u32 = 0x0;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct d_cr_dmap5 { pub value: u32 }
pub const d_cr_dmap5_offset: u32 = 0x154;
pub const d_cr_dmap5_r_opcode: u32 = 0x0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
