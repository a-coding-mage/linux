/* SPDX-License-Identifier: MIT */
/* Copyright © 2026 Intel Corporation */

pub const I915_PM_INTERRUPT: u32 = 1 << 31;
pub const I915_ISP_INTERRUPT: u32 = 1 << 22;
pub const I915_LPE_PIPE_B_INTERRUPT: u32 = 1 << 21;
pub const I915_LPE_PIPE_A_INTERRUPT: u32 = 1 << 20;
pub const I915_MIPIC_INTERRUPT: u32 = 1 << 19;
pub const I915_MIPIA_INTERRUPT: u32 = 1 << 18;
pub const I915_PIPE_CONTROL_NOTIFY_INTERRUPT: u32 = 1 << 18;
pub const I915_DISPLAY_PORT_INTERRUPT: u32 = 1 << 17;
pub const I915_DISPLAY_PIPE_C_HBLANK_INTERRUPT: u32 = 1 << 16;
pub const I915_MASTER_ERROR_INTERRUPT: u32 = 1 << 15;
pub const I915_DISPLAY_PIPE_B_HBLANK_INTERRUPT: u32 = 1 << 14;
pub const I915_GMCH_THERMAL_SENSOR_EVENT_INTERRUPT: u32 = 1 << 14; /* p-state */
pub const I915_DISPLAY_PIPE_A_HBLANK_INTERRUPT: u32 = 1 << 13;
pub const I915_HWB_OOM_INTERRUPT: u32 = 1 << 13;
pub const I915_LPE_PIPE_C_INTERRUPT: u32 = 1 << 12;
pub const I915_SYNC_STATUS_INTERRUPT: u32 = 1 << 12;
pub const I915_MISC_INTERRUPT: u32 = 1 << 11;
pub const I915_DISPLAY_PLANE_A_FLIP_PENDING_INTERRUPT: u32 = 1 << 11;
pub const I915_DISPLAY_PIPE_C_VBLANK_INTERRUPT: u32 = 1 << 10;
pub const I915_DISPLAY_PLANE_B_FLIP_PENDING_INTERRUPT: u32 = 1 << 10;
pub const I915_DISPLAY_PIPE_C_EVENT_INTERRUPT: u32 = 1 << 9;
pub const I915_OVERLAY_PLANE_FLIP_PENDING_INTERRUPT: u32 = 1 << 9;
pub const I915_DISPLAY_PIPE_C_DPBM_INTERRUPT: u32 = 1 << 8;
pub const I915_DISPLAY_PLANE_C_FLIP_PENDING_INTERRUPT: u32 = 1 << 8;
pub const I915_DISPLAY_PIPE_A_VBLANK_INTERRUPT: u32 = 1 << 7;
pub const I915_DISPLAY_PIPE_A_EVENT_INTERRUPT: u32 = 1 << 6;
pub const I915_DISPLAY_PIPE_B_VBLANK_INTERRUPT: u32 = 1 << 5;
pub const I915_DISPLAY_PIPE_B_EVENT_INTERRUPT: u32 = 1 << 4;
pub const I915_DISPLAY_PIPE_A_DPBM_INTERRUPT: u32 = 1 << 3;
pub const I915_DISPLAY_PIPE_B_DPBM_INTERRUPT: u32 = 1 << 2;
pub const I915_DEBUG_INTERRUPT: u32 = 1 << 2;
pub const I915_WINVALID_INTERRUPT: u32 = 1 << 1;
pub const I915_USER_INTERRUPT: u32 = 1 << 1;
pub const I915_ASLE_INTERRUPT: u32 = 1 << 0;
pub const I915_BSD_USER_INTERRUPT: u32 = 1 << 25;

pub const GEN8_MASTER_IRQ: u32 = _MMIO(0x44200);
pub const GEN8_MASTER_IRQ_CONTROL: u32 = 1 << 31;
pub const GEN8_PCU_IRQ: u32 = 1 << 30;
pub const GEN8_DE_PCH_IRQ: u32 = 1 << 23;
pub const GEN8_DE_MISC_IRQ: u32 = 1 << 22;
pub const GEN8_DE_PORT_IRQ: u32 = 1 << 20;
pub const GEN8_DE_PIPE_C_IRQ: u32 = 1 << 18;
pub const GEN8_DE_PIPE_B_IRQ: u32 = 1 << 17;
pub const GEN8_DE_PIPE_A_IRQ: u32 = 1 << 16;
macro_rules! GEN8_DE_PIPE_IRQ { ($pipe:expr) => { 1u32 << (16 + ($pipe)) }; }
pub const GEN8_GT_VECS_IRQ: u32 = 1 << 6;
pub const GEN8_GT_GUC_IRQ: u32 = 1 << 5;
pub const GEN8_GT_PM_IRQ: u32 = 1 << 4;
pub const GEN8_GT_VCS1_IRQ: u32 = 1 << 3; /* NB: VCS2 in bspec! */
pub const GEN8_GT_VCS0_IRQ: u32 = 1 << 2; /* NB: VCS1 in bpsec! */
pub const GEN8_GT_BCS_IRQ: u32 = 1 << 1;
pub const GEN8_GT_RCS_IRQ: u32 = 1 << 0;

pub const GEN11_GU_MISC_ISR: u32 = _MMIO(0x444f0);
pub const GEN11_GU_MISC_IMR: u32 = _MMIO(0x444f4);
pub const GEN11_GU_MISC_IIR: u32 = _MMIO(0x444f8);
pub const GEN11_GU_MISC_IER: u32 = _MMIO(0x444fc);
pub const GEN11_GU_MISC_GSE: u32 = 1 << 27;
pub const GEN11_GU_MISC_IRQ_REGS: _ = I915_IRQ_REGS!(GEN11_GU_MISC_IMR, GEN11_GU_MISC_IER, GEN11_GU_MISC_IIR);

pub const GEN11_GFX_MSTR_IRQ: u32 = _MMIO(0x190010);
pub const GEN11_MASTER_IRQ: u32 = 1 << 31;
pub const GEN11_PCU_IRQ: u32 = 1 << 30;
pub const GEN11_GU_MISC_IRQ: u32 = 1 << 29;
pub const GEN11_DISPLAY_IRQ: u32 = 1 << 16;
macro_rules! GEN11_GT_DW_IRQ { ($x:expr) => { 1u32 << ($x) }; }
pub const GEN11_GT_DW1_IRQ: u32 = 1 << 1;
pub const GEN11_GT_DW0_IRQ: u32 = 1 << 0;

pub const SCPD0: u32 = _MMIO(0x209c); /* 915+ only */
pub const SCPD_FBC_IGNORE_3D: u32 = 1 << 6;
pub const CSTATE_RENDER_CLOCK_GATE_DISABLE: u32 = 1 << 5;

pub const VLV_IIR_RW: u32 = _MMIO(VLV_DISPLAY_BASE + 0x2084);
pub const VLV_IER: u32 = _MMIO(VLV_DISPLAY_BASE + 0x20a0);
pub const VLV_IIR: u32 = _MMIO(VLV_DISPLAY_BASE + 0x20a4);
pub const VLV_IMR: u32 = _MMIO(VLV_DISPLAY_BASE + 0x20a8);
pub const VLV_ISR: u32 = _MMIO(VLV_DISPLAY_BASE + 0x20ac);
pub const VLV_PCBR: u32 = _MMIO(VLV_DISPLAY_BASE + 0x2120);
pub const VLV_PCBR_ADDR_SHIFT: u32 = 12;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
