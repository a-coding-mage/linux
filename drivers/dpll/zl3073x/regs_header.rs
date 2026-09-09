/* SPDX-License-Identifier: GPL-2.0-only */

// Translated from regs.h. Linux bitfield helpers are represented locally.
pub const fn bit(n: u32) -> u32 { 1u32 << n }
pub const fn genmask(high: u32, low: u32) -> u32 {
    if high == 31 { u32::MAX << low } else { ((1u32 << (high + 1)) - 1) & (!0u32 << low) }
}
pub const fn field_get(mask: u32, value: u32) -> u32 { (value & mask) >> mask.trailing_zeros() }
pub const fn field_prep(mask: u32, value: u32) -> u32 { (value << mask.trailing_zeros()) & mask }

/* Hardware limits for ZL3073x chip family */
pub const ZL3073X_MAX_CHANNELS: u32 = 5;
pub const ZL3073X_NUM_REFS: u32 = 10;
pub const ZL3073X_NUM_OUTS: u32 = 10;
pub const ZL3073X_NUM_SYNTHS: u32 = 5;
pub const ZL3073X_NUM_INPUT_PINS: u32 = ZL3073X_NUM_REFS;
pub const ZL3073X_NUM_OUTPUT_PINS: u32 = ZL3073X_NUM_OUTS * 2;
pub const ZL3073X_NUM_PINS: u32 = ZL3073X_NUM_INPUT_PINS + ZL3073X_NUM_OUTPUT_PINS;
pub const ZL3073X_NCO_PIN_ID: u32 = ZL3073X_NUM_PINS;

pub const ZL_REG_OFFSET_MASK: u32 = genmask(6, 0);
pub const ZL_REG_PAGE_MASK: u32 = genmask(15, 7);
pub const ZL_REG_SIZE_MASK: u32 = genmask(18, 16);
pub const ZL_REG_MAX_OFFSET_MASK: u32 = genmask(25, 19);
pub const ZL_REG_ADDR_MASK: u32 = genmask(15, 0);
pub const fn zl_reg_offset(reg: u32) -> u32 { field_get(ZL_REG_OFFSET_MASK, reg) }
pub const fn zl_reg_page(reg: u32) -> u32 { field_get(ZL_REG_PAGE_MASK, reg) }
pub const fn zl_reg_max_offset(reg: u32) -> u32 { field_get(ZL_REG_MAX_OFFSET_MASK, reg) }
pub const fn zl_reg_size(reg: u32) -> u32 { field_get(ZL_REG_SIZE_MASK, reg) }
pub const fn zl_reg_addr(reg: u32) -> u32 { field_get(ZL_REG_ADDR_MASK, reg) }
pub const fn zl_reg_idx(idx: u32, page: u32, offset: u32, size: u32, items: u32, stride: u32) -> u32 {
    field_prep(ZL_REG_OFFSET_MASK, offset + idx * stride)
        | field_prep(ZL_REG_PAGE_MASK, page << 7)
        | field_prep(ZL_REG_SIZE_MASK, size << 16)
        | field_prep(ZL_REG_MAX_OFFSET_MASK, offset + (items - 1) * stride)
}
pub const fn zl_reg(page: u32, offset: u32, size: u32) -> u32 { zl_reg_idx(0, page, offset, size, 1, 0) }

pub const ZL_REG_INFO: u32 = zl_reg(0, 0x00, 1); pub const ZL_INFO_READY: u32 = bit(7);
pub const ZL_REG_ID: u32 = zl_reg(0, 0x01, 2); pub const ZL_REG_REVISION: u32 = zl_reg(0, 0x03, 2);
pub const ZL_REG_FW_VER: u32 = zl_reg(0, 0x05, 2); pub const ZL_REG_CUSTOM_CONFIG_VER: u32 = zl_reg(0, 0x07, 4);
pub const ZL_REG_RESET_STATUS: u32 = zl_reg(0, 0x18, 1); pub const ZL_REG_RESET_STATUS_RESET: u32 = bit(0);
pub const ZL_REG_DIE_TEMP_STATUS: u32 = zl_reg(0, 0x44, 2);

pub const fn zl_reg_ref_mon_status(i: u32) -> u32 { zl_reg_idx(i, 2, 0x02, 1, ZL3073X_NUM_REFS, 1) }
pub const ZL_REF_MON_STATUS_OK: u32 = 0; pub const ZL_REF_MON_STATUS_LOS: u32 = bit(0); pub const ZL_REF_MON_STATUS_SCM: u32 = bit(1); pub const ZL_REF_MON_STATUS_CFM: u32 = bit(2); pub const ZL_REF_MON_STATUS_GST: u32 = bit(3); pub const ZL_REF_MON_STATUS_PFM: u32 = bit(4); pub const ZL_REF_MON_STATUS_ESYNC: u32 = bit(6); pub const ZL_REF_MON_STATUS_SPLIT_XO: u32 = bit(7);
pub const fn zl_reg_dpll_mon_status(i: u32) -> u32 { zl_reg_idx(i, 2, 0x10, 1, ZL3073X_MAX_CHANNELS, 1) }
pub const ZL_DPLL_MON_STATUS_STATE: u32 = genmask(1,0); pub const ZL_DPLL_MON_STATUS_STATE_ACQUIRING: u32=0; pub const ZL_DPLL_MON_STATUS_STATE_LOCK:u32=1; pub const ZL_DPLL_MON_STATUS_STATE_HOLDOVER:u32=2; pub const ZL_DPLL_MON_STATUS_HO_READY:u32=bit(2);
pub const fn zl_reg_dpll_refsel_status(i:u32)->u32 { zl_reg_idx(i,2,0x30,1,ZL3073X_MAX_CHANNELS,1) }
pub const ZL_DPLL_REFSEL_STATUS_REFSEL:u32=genmask(3,0); pub const ZL_DPLL_REFSEL_STATUS_STATE:u32=genmask(6,4); pub const ZL_DPLL_REFSEL_STATUS_STATE_LOCK:u32=4;
pub const fn zl_reg_ref_freq(i:u32)->u32 { zl_reg_idx(i,2,0x44,4,ZL3073X_NUM_REFS,4) }

// Remaining register definitions retain the C macro names as const functions.
pub const ZL_REG_REF_PHASE_ERR_READ_RQST:u32=zl_reg(4,0x0f,1); pub const ZL_REF_PHASE_ERR_READ_RQST_RD:u32=bit(0);
pub const ZL_REG_REF_FREQ_MEAS_CTRL:u32=zl_reg(4,0x1c,1); pub const ZL_REF_FREQ_MEAS_CTRL_MASK:u32=genmask(1,0); pub const ZL_REF_FREQ_MEAS_CTRL_REF_FREQ:u32=1; pub const ZL_REF_FREQ_MEAS_CTRL_REF_FREQ_OFF:u32=2; pub const ZL_REF_FREQ_MEAS_CTRL_DPLL_FREQ_OFF:u32=3;
pub const ZL_REG_REF_FREQ_MEAS_MASK_3_0:u32=zl_reg(4,0x1d,1); pub const fn zl_reg_ref_freq_meas_mask_3_0(r:u32)->u32 { bit(r) }
pub const ZL_REG_REF_FREQ_MEAS_MASK_4:u32=zl_reg(4,0x1e,1); pub const fn zl_reg_ref_freq_meas_mask_4(r:u32)->u32 { bit(r-8) }
pub const ZL_REG_DPLL_MEAS_REF_FREQ_CTRL:u32=zl_reg(4,0x1f,1); pub const ZL_DPLL_MEAS_REF_FREQ_CTRL_EN:u32=bit(0); pub const ZL_DPLL_MEAS_REF_FREQ_CTRL_IDX:u32=genmask(6,4);
pub const fn zl_reg_ref_phase(i:u32)->u32 { zl_reg_idx(i,4,0x20,6,ZL3073X_NUM_REFS,6) }

// Register Page 5, DPLL
pub const fn zl_reg_dpll_mode_refsel(i:u32)->u32 { zl_reg_idx(i,5,4,1,ZL3073X_MAX_CHANNELS,4) }
pub const ZL_DPLL_MODE_REFSEL_MODE:u32=genmask(2,0); pub const ZL_DPLL_MODE_REFSEL_MODE_FREERUN:u32=0; pub const ZL_DPLL_MODE_REFSEL_MODE_HOLDOVER:u32=1; pub const ZL_DPLL_MODE_REFSEL_MODE_REFLOCK:u32=2; pub const ZL_DPLL_MODE_REFSEL_MODE_AUTO:u32=3; pub const ZL_DPLL_MODE_REFSEL_MODE_NCO:u32=4; pub const ZL_DPLL_MODE_REFSEL_REF:u32=genmask(7,4);
pub const fn zl_reg_dpll_ctrl(i:u32)->u32 { zl_reg_idx(i,5,5,1,ZL3073X_MAX_CHANNELS,4) }
pub const ZL_DPLL_CTRL_TIE_CLEAR:u32=bit(0); pub const ZL_DPLL_CTRL_TOD_STEP_RST:u32=bit(2); pub const ZL_DPLL_CTRL_NCO_AUTO_READ:u32=bit(7);
pub const fn zl_reg_dpll_df_read(i:u32)->u32 { zl_reg_idx(i,5,0x28,1,ZL3073X_MAX_CHANNELS,1) }
pub const ZL_DPLL_DF_READ_SEM:u32=bit(4); pub const ZL_DPLL_DF_READ_REF_OFST:u32=bit(3); pub const ZL_DPLL_DF_READ_CMD:u32=genmask(2,0); pub const ZL_DPLL_DF_READ_CMD_ACC_I:u32=4;

pub const ZL_REG_DPLL_TIE_CTRL:u32=zl_reg(5,0x30,1); pub const ZL_DPLL_TIE_CTRL_OP:u32=genmask(2,0); pub const ZL_DPLL_TIE_CTRL_OP_WR:u32=4; pub const ZL_REG_DPLL_TIE_CTRL_MASK:u32=zl_reg(5,0x31,1);
pub const fn zl_reg_dpll_tod_ctrl(i:u32)->u32 { zl_reg_idx(i,5,0x38,1,ZL3073X_MAX_CHANNELS,1) } pub const ZL_DPLL_TOD_CTRL_SEM:u32=bit(4); pub const ZL_DPLL_TOD_CTRL_CMD:u32=genmask(3,0); pub const ZL_DPLL_TOD_CTRL_CMD_WR_NEXT_1HZ:u32=1; pub const ZL_DPLL_TOD_CTRL_CMD_RD_CURRENT:u32=8; pub const ZL_DPLL_TOD_CTRL_CMD_RD_NEXT_1HZ:u32=9;
pub const ZL_REG_DPLL_MEAS_CTRL:u32=zl_reg(5,0x50,1); pub const ZL_DPLL_MEAS_CTRL_EN:u32=bit(0); pub const ZL_DPLL_MEAS_CTRL_AVG_FACTOR:u32=genmask(7,4); pub const ZL_REG_DPLL_MEAS_IDX:u32=zl_reg(5,0x51,1); pub const ZL_DPLL_MEAS_IDX:u32=genmask(2,0); pub const ZL_REG_DPLL_PHASE_ERR_READ_MASK:u32=zl_reg(5,0x54,1);
pub const fn zl_reg_dpll_phase_err_data(i:u32)->u32 { zl_reg_idx(i,5,0x55,6,ZL3073X_MAX_CHANNELS,6) }
pub const fn zl_reg_dpll_df_offset_03(i:u32)->u32 { zl_reg_idx(i,6,0,6,4,0x20) } pub const ZL_REG_DPLL_DF_OFFSET_4:u32=zl_reg(7,0,6); pub const fn zl_reg_dpll_df_offset(i:u32)->u32 { if i<4 { zl_reg_dpll_df_offset_03(i) } else { ZL_REG_DPLL_DF_OFFSET_4 } } pub const ZL_DPLL_DF_OFFSET_UNKNOWN:i64=i64::MIN;
pub const fn zl_reg_dpll_tie_data_03(i:u32)->u32 { zl_reg_idx(i,6,0x0c,6,4,0x20) } pub const ZL_REG_DPLL_TIE_DATA_4:u32=zl_reg(7,0x0c,6); pub const fn zl_reg_dpll_tie_data(i:u32)->u32 { if i<4 {zl_reg_dpll_tie_data_03(i)} else {ZL_REG_DPLL_TIE_DATA_4} }
pub const fn zl_reg_dpll_tod_sec_03(i:u32)->u32 { zl_reg_idx(i,6,0x12,6,4,0x20) } pub const ZL_REG_DPLL_TOD_SEC_4:u32=zl_reg(7,0x12,6); pub const fn zl_reg_dpll_tod_sec(i:u32)->u32 {if i<4{zl_reg_dpll_tod_sec_03(i)}else{ZL_REG_DPLL_TOD_SEC_4}}
pub const fn zl_reg_dpll_tod_ns_03(i:u32)->u32 { zl_reg_idx(i,6,0x18,4,4,0x20) } pub const ZL_REG_DPLL_TOD_NS_4:u32=zl_reg(7,0x18,4); pub const fn zl_reg_dpll_tod_ns(i:u32)->u32 {if i<4{zl_reg_dpll_tod_ns_03(i)}else{ZL_REG_DPLL_TOD_NS_4}}
pub const fn zl_reg_synth_ctrl(i:u32)->u32 {zl_reg_idx(i,9,0,1,ZL3073X_NUM_SYNTHS,1)} pub const ZL_SYNTH_CTRL_EN:u32=bit(0); pub const ZL_SYNTH_CTRL_DPLL_SEL:u32=genmask(6,4);
pub const ZL_REG_SYNTH_PHASE_SHIFT_CTRL:u32=zl_reg(9,0x1e,1); pub const ZL_REG_SYNTH_PHASE_SHIFT_MASK:u32=zl_reg(9,0x1f,1); pub const ZL_REG_SYNTH_PHASE_SHIFT_INTVL:u32=zl_reg(9,0x20,1); pub const ZL_REG_SYNTH_PHASE_SHIFT_DATA:u32=zl_reg(9,0x21,2);
pub const fn zl_reg_output_ctrl(i:u32)->u32 {zl_reg_idx(i,9,0x28,1,ZL3073X_NUM_OUTS,1)} pub const ZL_OUTPUT_CTRL_EN:u32=bit(0); pub const ZL_OUTPUT_CTRL_SYNTH_SEL:u32=genmask(6,4); pub const ZL_REG_OUTPUT_STEP_TIME_MASK:u32=zl_reg(9,0x36,2);
pub const ZL_REG_OUTPUT_PHASE_STEP_CTRL:u32=zl_reg(9,0x38,1); pub const ZL_OUTPUT_PHASE_STEP_CTRL_DPLL:u32=genmask(6,4); pub const ZL_OUTPUT_PHASE_STEP_CTRL_TOD_STEP:u32=bit(3); pub const ZL_OUTPUT_PHASE_STEP_CTRL_OP:u32=genmask(1,0); pub const ZL_OUTPUT_PHASE_STEP_CTRL_OP_NONE:u32=0; pub const ZL_OUTPUT_PHASE_STEP_CTRL_OP_RESET:u32=1; pub const ZL_OUTPUT_PHASE_STEP_CTRL_OP_READ:u32=2; pub const ZL_OUTPUT_PHASE_STEP_CTRL_OP_WRITE:u32=3; pub const ZL_REG_OUTPUT_PHASE_STEP_NUMBER:u32=zl_reg(9,0x39,1); pub const ZL_REG_OUTPUT_PHASE_STEP_MASK:u32=zl_reg(9,0x3a,2); pub const ZL_REG_OUTPUT_PHASE_STEP_DATA:u32=zl_reg(9,0x3c,4);

pub const ZL_REG_REF_MB_MASK:u32=zl_reg(10,2,2); pub const ZL_REG_REF_MB_SEM:u32=zl_reg(10,4,1); pub const ZL_REF_MB_SEM_WR:u32=bit(0); pub const ZL_REF_MB_SEM_RD:u32=bit(1); pub const ZL_REG_REF_FREQ_BASE:u32=zl_reg(10,5,2); pub const ZL_REG_REF_FREQ_MULT:u32=zl_reg(10,7,2); pub const ZL_REG_REF_RATIO_M:u32=zl_reg(10,9,2); pub const ZL_REG_REF_RATIO_N:u32=zl_reg(10,0xb,2); pub const ZL_REG_REF_CONFIG:u32=zl_reg(10,0xd,1); pub const ZL_REF_CONFIG_ENABLE:u32=bit(0); pub const ZL_REF_CONFIG_DIFF_EN:u32=bit(2); pub const ZL_REG_REF_PHASE_OFFSET_COMP:u32=zl_reg(10,0x28,6); pub const ZL_REG_REF_PHASE_OFFSET_COMP_32:u32=zl_reg(10,0x28,4); pub const ZL_REG_REF_SYNC_CTRL:u32=zl_reg(10,0x2e,1); pub const ZL_REF_SYNC_CTRL_MODE:u32=genmask(2,0); pub const ZL_REF_SYNC_CTRL_MODE_REFSYNC_PAIR_OFF:u32=0; pub const ZL_REF_SYNC_CTRL_MODE_REFSYNC_PAIR:u32=1; pub const ZL_REF_SYNC_CTRL_MODE_50_50_ESYNC_25_75:u32=2; pub const ZL_REF_SYNC_CTRL_PAIR:u32=genmask(7,4); pub const ZL_REG_REF_ESYNC_DIV:u32=zl_reg(10,0x30,4); pub const ZL_REF_ESYNC_DIV_1HZ:u32=0;

pub const ZL_REG_DPLL_MB_MASK:u32=zl_reg(12,2,2); pub const ZL_REG_DPLL_MB_SEM:u32=zl_reg(12,4,1); pub const ZL_DPLL_MB_SEM_WR:u32=bit(0); pub const ZL_DPLL_MB_SEM_RD:u32=bit(1); pub const fn zl_reg_dpll_ref_prio(i:u32)->u32 {zl_reg_idx(i,12,0x52,1,ZL3073X_NUM_REFS/2,1)} pub const ZL_DPLL_REF_PRIO_REF_P:u32=genmask(3,0); pub const ZL_DPLL_REF_PRIO_REF_N:u32=genmask(7,4); pub const ZL_DPLL_REF_PRIO_MAX:u32=14; pub const ZL_DPLL_REF_PRIO_NONE:u32=15;

pub const ZL_REG_SYNTH_MB_MASK:u32=zl_reg(13,2,2); pub const ZL_REG_SYNTH_MB_SEM:u32=zl_reg(13,4,1); pub const ZL_SYNTH_MB_SEM_WR:u32=bit(0); pub const ZL_SYNTH_MB_SEM_RD:u32=bit(1); pub const ZL_REG_SYNTH_FREQ_BASE:u32=zl_reg(13,6,2); pub const ZL_REG_SYNTH_FREQ_MULT:u32=zl_reg(13,8,4); pub const ZL_REG_SYNTH_FREQ_M:u32=zl_reg(13,0xc,2); pub const ZL_REG_SYNTH_FREQ_N:u32=zl_reg(13,0xe,2);

pub const ZL_REG_OUTPUT_MB_MASK:u32=zl_reg(14,2,2); pub const ZL_REG_OUTPUT_MB_SEM:u32=zl_reg(14,4,1); pub const ZL_OUTPUT_MB_SEM_WR:u32=bit(0); pub const ZL_OUTPUT_MB_SEM_RD:u32=bit(1); pub const ZL_REG_OUTPUT_MODE:u32=zl_reg(14,5,1); pub const ZL_OUTPUT_MODE_CLOCK_TYPE:u32=genmask(2,0); pub const ZL_OUTPUT_MODE_CLOCK_TYPE_NORMAL:u32=0; pub const ZL_OUTPUT_MODE_CLOCK_TYPE_ESYNC:u32=1; pub const ZL_OUTPUT_MODE_SIGNAL_FORMAT:u32=genmask(7,4); pub const ZL_OUTPUT_MODE_SIGNAL_FORMAT_DISABLED:u32=0; pub const ZL_OUTPUT_MODE_SIGNAL_FORMAT_LVDS:u32=1; pub const ZL_OUTPUT_MODE_SIGNAL_FORMAT_DIFF:u32=2; pub const ZL_OUTPUT_MODE_SIGNAL_FORMAT_LOWVCM:u32=3; pub const ZL_OUTPUT_MODE_SIGNAL_FORMAT_2:u32=4; pub const ZL_OUTPUT_MODE_SIGNAL_FORMAT_1P:u32=5; pub const ZL_OUTPUT_MODE_SIGNAL_FORMAT_1N:u32=6; pub const ZL_OUTPUT_MODE_SIGNAL_FORMAT_2_INV:u32=7; pub const ZL_OUTPUT_MODE_SIGNAL_FORMAT_2_NDIV:u32=12; pub const ZL_OUTPUT_MODE_SIGNAL_FORMAT_2_NDIV_INV:u32=15; pub const ZL_REG_OUTPUT_DIV:u32=zl_reg(14,0xc,4); pub const ZL_REG_OUTPUT_WIDTH:u32=zl_reg(14,0x10,4); pub const ZL_REG_OUTPUT_ESYNC_PERIOD:u32=zl_reg(14,0x14,4); pub const ZL_REG_OUTPUT_ESYNC_WIDTH:u32=zl_reg(14,0x18,4); pub const ZL_REG_OUTPUT_PHASE_COMP:u32=zl_reg(14,0x20,4);

pub const ZL_REG_HWREG_OP:u32=zl_reg(0xff,0,1); pub const ZL_HWREG_OP_WRITE:u32=0x28; pub const ZL_HWREG_OP_READ:u32=0x29; pub const ZL_HWREG_OP_PENDING:u32=bit(1); pub const ZL_REG_HWREG_ADDR:u32=zl_reg(0xff,4,4); pub const ZL_REG_HWREG_WRITE_DATA:u32=zl_reg(0xff,8,4); pub const ZL_REG_HWREG_READ_DATA:u32=zl_reg(0xff,0xc,4);
pub const ZL_REG_FLASH_HASH:u32=zl_reg(0,0x78,4); pub const ZL_REG_FLASH_FAMILY:u32=zl_reg(0,0x7c,1); pub const ZL_REG_FLASH_RELEASE:u32=zl_reg(0,0x7d,1); pub const ZL_REG_HOST_CONTROL:u32=zl_reg(1,2,1); pub const ZL_HOST_CONTROL_ENABLE:u32=bit(0); pub const ZL_REG_IMAGE_START_ADDR:u32=zl_reg(1,4,4); pub const ZL_REG_IMAGE_SIZE:u32=zl_reg(1,8,4); pub const ZL_REG_FLASH_INDEX_READ:u32=zl_reg(1,0xc,4); pub const ZL_REG_FLASH_INDEX_WRITE:u32=zl_reg(1,0x10,4); pub const ZL_REG_FILL_PATTERN:u32=zl_reg(1,0x14,4); pub const ZL_REG_WRITE_FLASH:u32=zl_reg(1,0x18,1); pub const ZL_WRITE_FLASH_OP:u32=genmask(2,0); pub const ZL_WRITE_FLASH_OP_DONE:u32=0; pub const ZL_WRITE_FLASH_OP_SECTORS:u32=2; pub const ZL_WRITE_FLASH_OP_PAGE:u32=3; pub const ZL_WRITE_FLASH_OP_COPY_PAGE:u32=4; pub const ZL_REG_FLASH_INFO:u32=zl_reg(2,0,1); pub const ZL_FLASH_INFO_SECTOR_SIZE:u32=genmask(3,0); pub const ZL_FLASH_INFO_SECTOR_4K:u32=0; pub const ZL_FLASH_INFO_SECTOR_64K:u32=1; pub const ZL_REG_ERROR_COUNT:u32=zl_reg(2,4,4); pub const ZL_REG_ERROR_CAUSE:u32=zl_reg(2,8,4); pub const ZL_REG_OP_STATE:u32=zl_reg(2,0x14,1); pub const ZL_OP_STATE_NO_COMMAND:u32=0; pub const ZL_OP_STATE_PENDING:u32=1; pub const ZL_OP_STATE_DONE:u32=2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
