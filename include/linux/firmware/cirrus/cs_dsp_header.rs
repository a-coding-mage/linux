/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * cs_dsp.h  --  Cirrus Logic DSP firmware support
 *
 * Based on sound/soc/codecs/wm_adsp.h
 */

// External kernel types referenced by this header are supplied by other translated files.

pub const CS_ADSP2_REGION_0: u32 = 1 << 0;
pub const CS_ADSP2_REGION_1: u32 = 1 << 1;
pub const CS_ADSP2_REGION_2: u32 = 1 << 2;
pub const CS_ADSP2_REGION_3: u32 = 1 << 3;
pub const CS_ADSP2_REGION_4: u32 = 1 << 4;
pub const CS_ADSP2_REGION_5: u32 = 1 << 5;
pub const CS_ADSP2_REGION_6: u32 = 1 << 6;
pub const CS_ADSP2_REGION_7: u32 = 1 << 7;
pub const CS_ADSP2_REGION_8: u32 = 1 << 8;
pub const CS_ADSP2_REGION_9: u32 = 1 << 9;
pub const CS_ADSP2_REGION_1_9: u32 = CS_ADSP2_REGION_1 | CS_ADSP2_REGION_2 |
    CS_ADSP2_REGION_3 | CS_ADSP2_REGION_4 | CS_ADSP2_REGION_5 |
    CS_ADSP2_REGION_6 | CS_ADSP2_REGION_7 | CS_ADSP2_REGION_8 | CS_ADSP2_REGION_9;
pub const CS_ADSP2_REGION_ALL: u32 = CS_ADSP2_REGION_0 | CS_ADSP2_REGION_1_9;

pub const CS_DSP_DATA_WORD_SIZE: u32 = 3;
pub const CS_DSP_DATA_WORD_BITS: u32 = 3 * 8;
pub const CS_DSP_ACKED_CTL_TIMEOUT_MS: u32 = 100;
pub const CS_DSP_ACKED_CTL_N_QUICKPOLLS: u32 = 10;
pub const CS_DSP_ACKED_CTL_MIN_VALUE: u32 = 0;
pub const CS_DSP_ACKED_CTL_MAX_VALUE: u32 = 0xFFFFFF;

pub const CS_DSP_WSEQ_FULL: u8 = 0x00;
pub const CS_DSP_WSEQ_ADDR8: u8 = 0x02;
pub const CS_DSP_WSEQ_L16: u8 = 0x04;
pub const CS_DSP_WSEQ_H16: u8 = 0x05;
pub const CS_DSP_WSEQ_UNLOCK: u8 = 0xFD;
pub const CS_DSP_WSEQ_END: u8 = 0xFF;

#[repr(C)]
pub struct cs_dsp_region {
    pub r#type: i32,
    pub base: u32,
}

#[repr(C)]
pub struct cs_dsp_alg_region {
    pub alg: u32,
    pub ver: u32,
    pub r#type: i32,
    pub base: u32,
}

#[repr(C)]
pub struct cs_dsp_coeff_ctl {
    pub list: list_head,
    pub dsp: *mut cs_dsp,
    pub cache: *mut core::ffi::c_void,
    pub fw_name: *const core::ffi::c_char,
    pub subname: *const core::ffi::c_char,
    pub subname_len: u32,
    pub offset: u32,
    pub len: u32,
    pub r#type: u32,
    pub flags: u32,
    pub set: u32,
    pub enabled: u32,
    pub alg_region: cs_dsp_alg_region,
    pub priv_: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct cs_dsp {
    pub name: *const core::ffi::c_char,
    pub rev: i32,
    pub num: i32,
    pub r#type: i32,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub ops: *const cs_dsp_ops,
    pub client_ops: *const cs_dsp_client_ops,
    pub base: u32,
    pub base_sysinfo: u32,
    pub sysclk_reg: u32,
    pub sysclk_mask: u32,
    pub sysclk_shift: u32,
    pub no_core_startstop: bool,
    pub alg_regions: list_head,
    pub fw_name: *const core::ffi::c_char,
    pub fw_id: u32,
    pub fw_id_version: u32,
    pub fw_vendor_id: u32,
    pub mem: *const cs_dsp_region,
    pub num_mems: i32,
    pub wmfw_ver: i32,
    pub booted: bool,
    pub running: bool,
    pub hibernating: bool,
    pub ctl_list: list_head,
    pub pwr_lock: mutex,
    pub lock_regions: u32,
    #[cfg(feature = "CONFIG_DEBUG_FS")]
    pub debugfs_root: *mut dentry,
    #[cfg(feature = "CONFIG_DEBUG_FS")]
    pub wmfw_file_name: *const core::ffi::c_char,
    #[cfg(feature = "CONFIG_DEBUG_FS")]
    pub bin_file_name: *const core::ffi::c_char,
}

pub struct cs_dsp_ops;
pub struct cs_dsp_client_ops;

#[repr(C)]
pub struct cs_dsp_client_ops {
    pub control_add: Option<unsafe extern "C" fn(*mut cs_dsp_coeff_ctl) -> i32>,
    pub control_remove: Option<unsafe extern "C" fn(*mut cs_dsp_coeff_ctl)>,
    pub pre_run: Option<unsafe extern "C" fn(*mut cs_dsp) -> i32>,
    pub post_run: Option<unsafe extern "C" fn(*mut cs_dsp) -> i32>,
    pub pre_stop: Option<unsafe extern "C" fn(*mut cs_dsp)>,
    pub post_stop: Option<unsafe extern "C" fn(*mut cs_dsp)>,
    pub watchdog_expired: Option<unsafe extern "C" fn(*mut cs_dsp)>,
}

extern "C" {
    pub fn cs_dsp_adsp1_init(dsp: *mut cs_dsp) -> i32;
    pub fn cs_dsp_adsp2_init(dsp: *mut cs_dsp) -> i32;
    pub fn cs_dsp_halo_init(dsp: *mut cs_dsp) -> i32;
    pub fn cs_dsp_adsp1_power_up(dsp: *mut cs_dsp, wmfw_firmware: *const firmware, wmfw_filename: *const core::ffi::c_char, coeff_firmware: *const firmware, coeff_filename: *const core::ffi::c_char, fw_name: *const core::ffi::c_char) -> i32;
    pub fn cs_dsp_adsp1_power_down(dsp: *mut cs_dsp);
    pub fn cs_dsp_power_up(dsp: *mut cs_dsp, wmfw_firmware: *const firmware, wmfw_filename: *const core::ffi::c_char, coeff_firmware: *const firmware, coeff_filename: *const core::ffi::c_char, fw_name: *const core::ffi::c_char) -> i32;
    pub fn cs_dsp_power_down(dsp: *mut cs_dsp);
    pub fn cs_dsp_run(dsp: *mut cs_dsp) -> i32;
    pub fn cs_dsp_stop(dsp: *mut cs_dsp);
    pub fn cs_dsp_remove(dsp: *mut cs_dsp);
    pub fn cs_dsp_set_dspclk(dsp: *mut cs_dsp, freq: u32) -> i32;
    pub fn cs_dsp_adsp2_bus_error(dsp: *mut cs_dsp);
    pub fn cs_dsp_halo_bus_error(dsp: *mut cs_dsp);
    pub fn cs_dsp_halo_wdt_expire(dsp: *mut cs_dsp);
    pub fn cs_dsp_init_debugfs(dsp: *mut cs_dsp, debugfs_root: *mut dentry);
    pub fn cs_dsp_cleanup_debugfs(dsp: *mut cs_dsp);
    pub fn cs_dsp_coeff_write_acked_control(ctl: *mut cs_dsp_coeff_ctl, event_id: u32) -> i32;
    pub fn cs_dsp_coeff_write_ctrl(ctl: *mut cs_dsp_coeff_ctl, off: u32, buf: *const core::ffi::c_void, len: usize) -> i32;
    pub fn cs_dsp_coeff_lock_and_write_ctrl(ctl: *mut cs_dsp_coeff_ctl, off: u32, buf: *const core::ffi::c_void, len: usize) -> i32;
    pub fn cs_dsp_coeff_read_ctrl(ctl: *mut cs_dsp_coeff_ctl, off: u32, buf: *mut core::ffi::c_void, len: usize) -> i32;
    pub fn cs_dsp_coeff_lock_and_read_ctrl(ctl: *mut cs_dsp_coeff_ctl, off: u32, buf: *mut core::ffi::c_void, len: usize) -> i32;
    pub fn cs_dsp_get_ctl(dsp: *mut cs_dsp, name: *const core::ffi::c_char, r#type: i32, alg: u32) -> *mut cs_dsp_coeff_ctl;
    pub fn cs_dsp_read_raw_data_block(dsp: *mut cs_dsp, mem_type: i32, mem_addr: u32, num_words: u32, data: *mut u32) -> i32;
    pub fn cs_dsp_read_data_word(dsp: *mut cs_dsp, mem_type: i32, mem_addr: u32, data: *mut u32) -> i32;
    pub fn cs_dsp_write_data_word(dsp: *mut cs_dsp, mem_type: i32, mem_addr: u32, data: u32) -> i32;
    pub fn cs_dsp_remove_padding(buf: *mut u32, nwords: i32);
    pub fn cs_dsp_find_alg_region(dsp: *mut cs_dsp, r#type: i32, id: u32) -> *mut cs_dsp_alg_region;
    pub fn cs_dsp_mem_region_name(r#type: u32) -> *const core::ffi::c_char;
}

#[repr(C)]
pub struct cs_dsp_wseq {
    pub ctl: *mut cs_dsp_coeff_ctl,
    pub ops: list_head,
}

extern "C" {
    pub fn cs_dsp_wseq_init(dsp: *mut cs_dsp, wseqs: *mut cs_dsp_wseq, num_wseqs: u32) -> i32;
    pub fn cs_dsp_wseq_write(dsp: *mut cs_dsp, wseq: *mut cs_dsp_wseq, addr: u32, data: u32, op_code: u8, update: bool) -> i32;
    pub fn cs_dsp_wseq_multi_write(dsp: *mut cs_dsp, wseq: *mut cs_dsp_wseq, reg_seq: *const reg_sequence, num_regs: i32, op_code: u8, update: bool) -> i32;
}

#[repr(C)]
pub struct cs_dsp_chunk {
    pub data: *mut u8,
    pub max: *mut u8,
    pub bytes: i32,
    pub cache: u32,
    pub cachebits: i32,
}

#[inline]
pub unsafe fn cs_dsp_chunk(data: *mut u8, size: i32) -> cs_dsp_chunk {
    cs_dsp_chunk { data, max: data.offset(size as isize), bytes: 0, cache: 0, cachebits: 0 }
}

#[inline]
pub unsafe fn cs_dsp_chunk_end(ch: *mut cs_dsp_chunk) -> bool { (*ch).data == (*ch).max }

#[inline]
pub unsafe fn cs_dsp_chunk_bytes(ch: *mut cs_dsp_chunk) -> i32 { (*ch).bytes }

#[inline]
pub unsafe fn cs_dsp_chunk_valid_addr(ch: *mut cs_dsp_chunk, addr: *mut core::ffi::c_void) -> bool {
    let addr = addr as *mut u8;
    addr >= (*ch).data && addr < (*ch).max
}

extern "C" {
    pub fn cs_dsp_chunk_write(ch: *mut cs_dsp_chunk, nbits: i32, val: u32) -> i32;
    pub fn cs_dsp_chunk_flush(ch: *mut cs_dsp_chunk) -> i32;
    pub fn cs_dsp_chunk_read(ch: *mut cs_dsp_chunk, nbits: i32) -> i32;
    pub fn cs_dsp_hibernate(dsp: *mut cs_dsp, hibernating: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
