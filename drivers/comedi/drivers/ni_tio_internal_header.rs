/* SPDX-License-Identifier: GPL-2.0+ */
/* Rust translation of ni_tio_internal.h.  Dependencies are supplied by ni_tio. */

#[inline(always)]
pub const fn BIT(x: u32) -> u32 { 1u32 << x }

pub const fn NITIO_AUTO_INC_REG(x: u32) -> u32 { NITIO_G0_AUTO_INC + x }
pub const GI_AUTO_INC_MASK: u32 = 0xff;
pub const fn NITIO_CMD_REG(x: u32) -> u32 { NITIO_G0_CMD + x }
pub const GI_ARM: u32 = BIT(0); pub const GI_SAVE_TRACE: u32 = BIT(1);
pub const GI_LOAD: u32 = BIT(2); pub const GI_DISARM: u32 = BIT(4);
pub const fn GI_CNT_DIR(x: u32) -> u32 { (x & 0x3) << 5 }
pub const GI_CNT_DIR_MASK: u32 = GI_CNT_DIR(3);
pub const GI_WRITE_SWITCH: u32 = BIT(7); pub const GI_SYNC_GATE: u32 = BIT(8);
pub const GI_LITTLE_BIG_ENDIAN: u32 = BIT(9); pub const GI_BANK_SWITCH_START: u32 = BIT(10);
pub const GI_BANK_SWITCH_MODE: u32 = BIT(11); pub const GI_BANK_SWITCH_ENABLE: u32 = BIT(12);
pub const GI_ARM_COPY: u32 = BIT(13); pub const GI_SAVE_TRACE_COPY: u32 = BIT(14);
pub const GI_DISARM_COPY: u32 = BIT(15);
pub const fn NITIO_HW_SAVE_REG(x: u32) -> u32 { NITIO_G0_HW_SAVE + x }
pub const fn NITIO_SW_SAVE_REG(x: u32) -> u32 { NITIO_G0_SW_SAVE + x }
pub const fn NITIO_MODE_REG(x: u32) -> u32 { NITIO_G0_MODE + x }
pub const fn GI_GATING_MODE(x: u32) -> u32 { (x & 3) << 0 }
pub const GI_GATING_DISABLED: u32 = GI_GATING_MODE(0); pub const GI_LEVEL_GATING: u32 = GI_GATING_MODE(1);
pub const GI_RISING_EDGE_GATING: u32 = GI_GATING_MODE(2); pub const GI_FALLING_EDGE_GATING: u32 = GI_GATING_MODE(3);
pub const GI_GATING_MODE_MASK: u32 = GI_GATING_MODE(3); pub const GI_GATE_ON_BOTH_EDGES: u32 = BIT(2);
pub const fn GI_EDGE_GATE_MODE(x: u32) -> u32 { (x & 3) << 3 }
pub const GI_EDGE_GATE_STARTS_STOPS: u32 = GI_EDGE_GATE_MODE(0); pub const GI_EDGE_GATE_STOPS_STARTS: u32 = GI_EDGE_GATE_MODE(1);
pub const GI_EDGE_GATE_STARTS: u32 = GI_EDGE_GATE_MODE(2); pub const GI_EDGE_GATE_NO_STARTS_OR_STOPS: u32 = GI_EDGE_GATE_MODE(3);
pub const GI_EDGE_GATE_MODE_MASK: u32 = GI_EDGE_GATE_MODE(3);
pub const fn GI_STOP_MODE(x: u32) -> u32 { (x & 3) << 5 }
pub const GI_STOP_ON_GATE: u32 = GI_STOP_MODE(0); pub const GI_STOP_ON_GATE_OR_TC: u32 = GI_STOP_MODE(1);
pub const GI_STOP_ON_GATE_OR_SECOND_TC: u32 = GI_STOP_MODE(2); pub const GI_STOP_MODE_MASK: u32 = GI_STOP_MODE(3);
pub const GI_LOAD_SRC_SEL: u32 = BIT(7); pub const fn GI_OUTPUT_MODE(x: u32) -> u32 { (x & 3) << 8 }
pub const GI_OUTPUT_TC_PULSE: u32 = GI_OUTPUT_MODE(1); pub const GI_OUTPUT_TC_TOGGLE: u32 = GI_OUTPUT_MODE(2);
pub const GI_OUTPUT_TC_OR_GATE_TOGGLE: u32 = GI_OUTPUT_MODE(3); pub const GI_OUTPUT_MODE_MASK: u32 = GI_OUTPUT_MODE(3);
pub const fn GI_COUNTING_ONCE(x: u32) -> u32 { (x & 3) << 10 }
pub const GI_NO_HARDWARE_DISARM: u32 = GI_COUNTING_ONCE(0); pub const GI_DISARM_AT_TC: u32 = GI_COUNTING_ONCE(1);
pub const GI_DISARM_AT_GATE: u32 = GI_COUNTING_ONCE(2); pub const GI_DISARM_AT_TC_OR_GATE: u32 = GI_COUNTING_ONCE(3);
pub const GI_COUNTING_ONCE_MASK: u32 = GI_COUNTING_ONCE(3); pub const GI_LOADING_ON_TC: u32 = BIT(12);
pub const GI_GATE_POL_INVERT: u32 = BIT(13); pub const GI_LOADING_ON_GATE: u32 = BIT(14); pub const GI_RELOAD_SRC_SWITCHING: u32 = BIT(15);
pub const fn NITIO_LOADA_REG(x: u32) -> u32 { NITIO_G0_LOADA + x }
pub const fn NITIO_LOADB_REG(x: u32) -> u32 { NITIO_G0_LOADB + x }
pub const fn NITIO_INPUT_SEL_REG(x: u32) -> u32 { NITIO_G0_INPUT_SEL + x }
pub const GI_READ_ACKS_IRQ: u32 = BIT(0); pub const GI_WRITE_ACKS_IRQ: u32 = BIT(1);
pub const fn GI_BITS_TO_SRC(x: u32) -> u32 { (x >> 2) & 0x1f }
pub const fn GI_SRC_SEL(x: u32) -> u32 { (x & 0x1f) << 2 }
pub const GI_SRC_SEL_MASK: u32 = GI_SRC_SEL(0x1f); pub const fn GI_BITS_TO_GATE(x: u32) -> u32 { (x >> 7) & 0x1f }
pub const fn GI_GATE_SEL(x: u32) -> u32 { (x & 0x1f) << 7 }
pub const GI_GATE_SEL_MASK: u32 = GI_GATE_SEL(0x1f); pub const GI_GATE_SEL_LOAD_SRC: u32 = BIT(12);
pub const GI_OR_GATE: u32 = BIT(13); pub const GI_OUTPUT_POL_INVERT: u32 = BIT(14); pub const GI_SRC_POL_INVERT: u32 = BIT(15);
pub const fn NITIO_CNT_MODE_REG(x: u32) -> u32 { NITIO_G0_CNT_MODE + x }
pub const fn GI_CNT_MODE(x: u32) -> u32 { (x & 7) << 0 }
pub const GI_CNT_MODE_NORMAL: u32 = GI_CNT_MODE(0); pub const GI_CNT_MODE_QUADX1: u32 = GI_CNT_MODE(1);
pub const GI_CNT_MODE_QUADX2: u32 = GI_CNT_MODE(2); pub const GI_CNT_MODE_QUADX4: u32 = GI_CNT_MODE(3);
pub const GI_CNT_MODE_TWO_PULSE: u32 = GI_CNT_MODE(4); pub const GI_CNT_MODE_SYNC_SRC: u32 = GI_CNT_MODE(6);
pub const GI_CNT_MODE_MASK: u32 = GI_CNT_MODE(7); pub const GI_INDEX_MODE: u32 = BIT(4);
pub const fn GI_INDEX_PHASE(x: u32) -> u32 { (x & 3) << 5 }
pub const GI_INDEX_PHASE_MASK: u32 = GI_INDEX_PHASE(3); pub const GI_HW_ARM_ENA: u32 = BIT(7);
pub const fn GI_HW_ARM_SEL(x: u32) -> u32 { x << 8 }
pub const GI_660X_HW_ARM_SEL_MASK: u32 = GI_HW_ARM_SEL(7); pub const GI_M_HW_ARM_SEL_MASK: u32 = GI_HW_ARM_SEL(0x1f);
pub const GI_660X_PRESCALE_X8: u32 = BIT(12); pub const GI_M_PRESCALE_X8: u32 = BIT(13);
pub const GI_660X_ALT_SYNC: u32 = BIT(13); pub const GI_M_ALT_SYNC: u32 = BIT(14);
pub const GI_660X_PRESCALE_X2: u32 = BIT(14); pub const GI_M_PRESCALE_X2: u32 = BIT(15);
pub const fn NITIO_GATE2_REG(x: u32) -> u32 { NITIO_G0_GATE2 + x }
pub const GI_GATE2_MODE: u32 = BIT(0); pub const fn GI_BITS_TO_GATE2(x: u32) -> u32 { (x >> 7) & 0x1f }
pub const fn GI_GATE2_SEL(x: u32) -> u32 { (x & 0x1f) << 7 }
pub const GI_GATE2_SEL_MASK: u32 = GI_GATE2_SEL(0x1f); pub const GI_GATE2_POL_INVERT: u32 = BIT(13);
pub const GI_GATE2_SUBSEL: u32 = BIT(14); pub const GI_SRC_SUBSEL: u32 = BIT(15);
pub const fn NITIO_SHARED_STATUS_REG(x: u32) -> u32 { NITIO_G01_STATUS + x / 2 }
pub const fn GI_SAVE(x: u32) -> u32 { if x % 2 != 0 { BIT(1) } else { BIT(0) } }
pub const fn GI_COUNTING(x: u32) -> u32 { if x % 2 != 0 { BIT(3) } else { BIT(2) } }
pub const fn GI_NEXT_LOAD_SRC(x: u32) -> u32 { if x % 2 != 0 { BIT(5) } else { BIT(4) } }
pub const fn GI_STALE_DATA(x: u32) -> u32 { if x % 2 != 0 { BIT(7) } else { BIT(6) } }
pub const fn GI_ARMED(x: u32) -> u32 { if x % 2 != 0 { BIT(9) } else { BIT(8) } }
pub const fn GI_NO_LOAD_BETWEEN_GATES(x: u32) -> u32 { if x % 2 != 0 { BIT(11) } else { BIT(10) } }
pub const fn GI_TC_ERROR(x: u32) -> u32 { if x % 2 != 0 { BIT(13) } else { BIT(12) } }
pub const fn GI_GATE_ERROR(x: u32) -> u32 { if x % 2 != 0 { BIT(15) } else { BIT(14) } }
pub const fn NITIO_RESET_REG(x: u32) -> u32 { NITIO_G01_RESET + x / 2 }
pub const fn GI_RESET(x: u32) -> u32 { BIT(2 + x % 2) }
pub const fn NITIO_STATUS1_REG(x: u32) -> u32 { NITIO_G01_STATUS1 + x / 2 }
pub const fn NITIO_STATUS2_REG(x: u32) -> u32 { NITIO_G01_STATUS2 + x / 2 }
pub const fn GI_OUTPUT(x: u32) -> u32 { if x % 2 != 0 { BIT(1) } else { BIT(0) } }
pub const fn GI_HW_SAVE(x: u32) -> u32 { if x % 2 != 0 { BIT(13) } else { BIT(12) } }
pub const fn GI_PERMANENT_STALE(x: u32) -> u32 { if x % 2 != 0 { BIT(15) } else { BIT(14) } }
pub const fn NITIO_DMA_CFG_REG(x: u32) -> u32 { NITIO_G0_DMA_CFG + x }
pub const GI_DMA_ENABLE: u32 = BIT(0); pub const GI_DMA_WRITE: u32 = BIT(1); pub const GI_DMA_INT_ENA: u32 = BIT(2);
pub const GI_DMA_RESET: u32 = BIT(3); pub const GI_DMA_BANKSW_ERROR: u32 = BIT(4);
pub const fn NITIO_DMA_STATUS_REG(x: u32) -> u32 { NITIO_G0_DMA_STATUS + x }
pub const GI_DMA_READBANK: u32 = BIT(13); pub const GI_DRQ_ERROR: u32 = BIT(14); pub const GI_DRQ_STATUS: u32 = BIT(15);
pub const fn NITIO_ABZ_REG(x: u32) -> u32 { NITIO_G0_ABZ + x }
pub const fn NITIO_INT_ACK_REG(x: u32) -> u32 { NITIO_G0_INT_ACK + x }
pub const fn GI_GATE_ERROR_CONFIRM(x: u32) -> u32 { if x % 2 != 0 { BIT(1) } else { BIT(5) } }
pub const fn GI_TC_ERROR_CONFIRM(x: u32) -> u32 { if x % 2 != 0 { BIT(2) } else { BIT(6) } }
pub const GI_TC_INTERRUPT_ACK: u32 = BIT(14); pub const GI_GATE_INTERRUPT_ACK: u32 = BIT(15);
pub const fn NITIO_STATUS_REG(x: u32) -> u32 { NITIO_G0_STATUS + x }
pub const GI_GATE_INTERRUPT: u32 = BIT(2); pub const GI_TC: u32 = BIT(3); pub const GI_INTERRUPT: u32 = BIT(15);
pub const fn NITIO_INT_ENA_REG(x: u32) -> u32 { NITIO_G0_INT_ENA + x }
pub const fn GI_TC_INTERRUPT_ENABLE(x: u32) -> u32 { if x % 2 != 0 { BIT(9) } else { BIT(6) } }
pub const fn GI_GATE_INTERRUPT_ENABLE(x: u32) -> u32 { if x % 2 != 0 { BIT(10) } else { BIT(8) } }

extern "C" {
    pub fn ni_tio_write(counter: *mut ni_gpct, value: ::core::ffi::c_uint, reg: ni_gpct_register);
    pub fn ni_tio_read(counter: *mut ni_gpct, reg: ni_gpct_register) -> ::core::ffi::c_uint;
    pub fn ni_tio_set_bits(counter: *mut ni_gpct, reg: ni_gpct_register, mask: ::core::ffi::c_uint, value: ::core::ffi::c_uint);
    pub fn ni_tio_get_soft_copy(counter: *const ni_gpct, reg: ni_gpct_register) -> ::core::ffi::c_uint;
    pub fn ni_tio_arm(counter: *mut ni_gpct, arm: bool, start_trigger: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn ni_tio_set_gate_src(counter: *mut ni_gpct, gate: ::core::ffi::c_uint, src: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn ni_tio_set_gate_src_raw(counter: *mut ni_gpct, gate: ::core::ffi::c_uint, src: ::core::ffi::c_uint) -> ::core::ffi::c_int;
}

#[inline]
pub fn ni_tio_counting_mode_registers_present(counter_dev: *const ni_gpct_device) -> bool {
    // m series and 660x variants have counting mode registers
    unsafe { (*counter_dev).variant != ni_gpct_variant_e_series }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
