/* SPDX-License-Identifier: GPL-2.0-only */

/* Dependencies supplied by the surrounding translation unit: chan.h, out.h,
 * ref.h, regs.h, and synth.h. */

pub const ZL_POLL_DF_READ_TIMEOUT_US: u32 = 25 * USEC_PER_MSEC;
pub const ZL_POLL_FREQ_MEAS_TIMEOUT_US: u32 = 50 * USEC_PER_MSEC;
pub const ZL_POLL_HWREG_TIMEOUT_US: u32 = 50 * USEC_PER_MSEC;
pub const ZL_POLL_MB_TIMEOUT_US: u32 = 30 * USEC_PER_MSEC;
pub const ZL_POLL_PHASE_ERR_TIMEOUT_US: u32 = 50 * USEC_PER_MSEC;
pub const ZL_POLL_PHASE_STEP_TIMEOUT_US: u32 = 3000 * USEC_PER_MSEC;
pub const ZL_POLL_TIE_WR_TIMEOUT_US: u32 = 1000 * USEC_PER_MSEC;
pub const ZL_POLL_TOD_RD_TIMEOUT_US: u32 = 30 * USEC_PER_MSEC;
pub const ZL_POLL_TOD_WR_TIMEOUT_US: u32 = 1000 * USEC_PER_MSEC;

pub const ZL3073X_FLAG_REF_PHASE_COMP_32_BIT: usize = 0;
pub const ZL3073X_FLAG_DIE_TEMP_BIT: usize = 1;
pub const ZL3073X_FLAGS_NBITS: usize = 2;
pub const ZL3073X_FLAG_REF_PHASE_COMP_32: usize = 1 << ZL3073X_FLAG_REF_PHASE_COMP_32_BIT;
pub const ZL3073X_FLAG_DIE_TEMP: usize = 1 << ZL3073X_FLAG_DIE_TEMP_BIT;

#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct regmap { _private: [u8; 0] }

#[repr(C)]
pub struct zl3073x_chip_info {
    pub id: u16,
    pub num_channels: u8,
    pub flags: usize,
}

#[repr(C)]
pub struct zl3073x_dev {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub info: *const zl3073x_chip_info,
    pub multiop_lock: mutex,
    pub tie_lock: mutex,
    pub phase_step_lock: mutex,
    pub ref_: [zl3073x_ref; ZL3073X_NUM_REFS],
    pub out: [zl3073x_out; ZL3073X_NUM_OUTS],
    pub synth: [zl3073x_synth; ZL3073X_NUM_SYNTHS],
    pub chan: [zl3073x_chan; ZL3073X_MAX_CHANNELS],
    pub dplls: list_head,
    pub kworker: *mut kthread_worker,
    pub work: kthread_delayed_work,
    pub clock_id: u64,
    pub out_step_time_mask: u16,
    pub phase_avg_factor: u8,
    pub freq_monitor: bool,
}

extern "C" {
    pub static zl3073x_regmap_config: regmap_config;
    pub fn zl3073x_devm_alloc(dev: *mut device) -> *mut zl3073x_dev;
    pub fn zl3073x_dev_probe(zldev: *mut zl3073x_dev) -> i32;
    pub fn zl3073x_dev_start(zldev: *mut zl3073x_dev, full: bool) -> i32;
    pub fn zl3073x_dev_stop(zldev: *mut zl3073x_dev);
}

#[inline]
pub unsafe fn zl3073x_dev_phase_avg_factor_get(zldev: *mut zl3073x_dev) -> u8 {
    (*zldev).phase_avg_factor
}
extern "C" { pub fn zl3073x_dev_phase_avg_factor_set(zldev: *mut zl3073x_dev, factor: u8) -> i32; }

#[repr(C)]
pub struct zl3073x_hwreg_seq_item { pub addr: u32, pub value: u32, pub mask: u32, pub wait: u32 }

#[macro_export]
macro_rules! HWREG_SEQ_ITEM {
    ($addr:expr, $value:expr, $mask:expr, $wait:expr) => {
        zl3073x_hwreg_seq_item { addr: $addr, value: (($value << (($mask).trailing_zeros())) & $mask), mask: $mask, wait: $wait }
    };
}

extern "C" {
    pub fn zl3073x_mb_op(zldev: *mut zl3073x_dev, op_reg: u32, op_val: u8, mask_reg: u32, mask_val: u16) -> i32;
    pub fn zl3073x_poll_zero_u8(zldev: *mut zl3073x_dev, reg: u32, mask: u8, timeout_us: u32) -> i32;
    pub fn zl3073x_read_u8(zldev: *mut zl3073x_dev, reg: u32, val: *mut u8) -> i32;
    pub fn zl3073x_read_u16(zldev: *mut zl3073x_dev, reg: u32, val: *mut u16) -> i32;
    pub fn zl3073x_read_u32(zldev: *mut zl3073x_dev, reg: u32, val: *mut u32) -> i32;
    pub fn zl3073x_read_u48(zldev: *mut zl3073x_dev, reg: u32, val: *mut u64) -> i32;
    pub fn zl3073x_write_u8(zldev: *mut zl3073x_dev, reg: u32, val: u8) -> i32;
    pub fn zl3073x_write_u16(zldev: *mut zl3073x_dev, reg: u32, val: u16) -> i32;
    pub fn zl3073x_write_u32(zldev: *mut zl3073x_dev, reg: u32, val: u32) -> i32;
    pub fn zl3073x_write_u48(zldev: *mut zl3073x_dev, reg: u32, val: u64) -> i32;
    pub fn zl3073x_read_hwreg(zldev: *mut zl3073x_dev, addr: u32, value: *mut u32) -> i32;
    pub fn zl3073x_write_hwreg(zldev: *mut zl3073x_dev, addr: u32, value: u32) -> i32;
    pub fn zl3073x_update_hwreg(zldev: *mut zl3073x_dev, addr: u32, value: u32, mask: u32) -> i32;
    pub fn zl3073x_write_hwreg_seq(zldev: *mut zl3073x_dev, seq: *const zl3073x_hwreg_seq_item, num_items: usize) -> i32;
    pub fn zl3073x_ref_phase_offsets_update(zldev: *mut zl3073x_dev, channel: i32) -> i32;
}

#[inline] pub unsafe fn zl3073x_dev_is_ref_phase_comp_32bit(z: *mut zl3073x_dev) -> bool { ((*(*z).info).flags & ZL3073X_FLAG_REF_PHASE_COMP_32) != 0 }
#[inline] pub fn zl3073x_is_n_pin(id: u8) -> bool { (id & 1) != 0 }
#[inline] pub fn zl3073x_is_p_pin(id: u8) -> bool { !zl3073x_is_n_pin(id) }
#[inline] pub fn zl3073x_input_pin_ref_get(id: u8) -> u8 { id }
#[inline] pub fn zl3073x_output_pin_out_get(id: u8) -> u8 { id / 2 }

#[inline] pub unsafe fn zl3073x_dev_ref_freq_get(z: *mut zl3073x_dev, i: u8) -> u32 { zl3073x_ref_freq_get(zl3073x_ref_state_get(z, i)) }
#[inline] pub unsafe fn zl3073x_dev_ref_is_diff(z: *mut zl3073x_dev, i: u8) -> bool { zl3073x_ref_is_diff(zl3073x_ref_state_get(z, i)) }
#[inline] pub unsafe fn zl3073x_dev_ref_is_status_ok(z: *mut zl3073x_dev, i: u8) -> bool { zl3073x_ref_is_status_ok(zl3073x_ref_state_get(z, i)) }
#[inline] pub unsafe fn zl3073x_dev_synth_freq_get(z: *mut zl3073x_dev, i: u8) -> u32 { zl3073x_synth_freq_get(zl3073x_synth_state_get(z, i)) }
#[inline] pub unsafe fn zl3073x_dev_out_synth_get(z: *mut zl3073x_dev, i: u8) -> u8 { zl3073x_out_synth_get(zl3073x_out_state_get(z, i)) }
#[inline] pub unsafe fn zl3073x_dev_out_is_enabled(z: *mut zl3073x_dev, i: u8) -> bool { let o=zl3073x_out_state_get(z,i); let s=zl3073x_synth_state_get(z,zl3073x_out_synth_get(o)); zl3073x_synth_is_enabled(s) && zl3073x_out_is_enabled(o) }
#[inline] pub unsafe fn zl3073x_dev_out_is_stepped(z: *mut zl3073x_dev, i: u8) -> bool { ((*z).out_step_time_mask & (1u16 << i)) != 0 }
#[inline] pub unsafe fn zl3073x_dev_out_dpll_get(z: *mut zl3073x_dev, i: u8) -> u8 { let o=zl3073x_out_state_get(z,i); zl3073x_synth_dpll_get(zl3073x_synth_state_get(z,zl3073x_out_synth_get(o))) }
#[inline] pub unsafe fn zl3073x_dev_output_pin_freq_get(z: *mut zl3073x_dev, id: u8) -> u32 { let o=zl3073x_out_state_get(z,zl3073x_output_pin_out_get(id)); let s=zl3073x_synth_state_get(z,zl3073x_out_synth_get(o)); let mut f=zl3073x_synth_freq_get(s)/(*o).div; if zl3073x_out_is_ndiv(o)&&zl3073x_is_n_pin(id){f/=(*o).esync_n_period;} f }
#[inline] pub unsafe fn zl3073x_dev_out_is_diff(z: *mut zl3073x_dev, i: u8) -> bool { zl3073x_out_is_diff(zl3073x_out_state_get(z,i)) }

#[inline]
pub unsafe fn zl3073x_dev_output_pin_is_enabled(z: *mut zl3073x_dev, id: u8) -> bool {
    let o = zl3073x_out_state_get(z, zl3073x_output_pin_out_get(id));
    if !zl3073x_dev_out_is_enabled(z, zl3073x_output_pin_out_get(id)) { return false; }
    match zl3073x_out_signal_format_get(o) {
        ZL_OUTPUT_MODE_SIGNAL_FORMAT_DISABLED => false,
        ZL_OUTPUT_MODE_SIGNAL_FORMAT_1P => !zl3073x_is_n_pin(id),
        ZL_OUTPUT_MODE_SIGNAL_FORMAT_1N => !zl3073x_is_p_pin(id),
        _ => true,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
