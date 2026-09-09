/* SPDX-License-Identifier: GPL-2.0-only */
/* Translation of linux/tpm.h. */

// Dependencies supplied by the surrounding kernel translation.

pub enum trusted_key_payload {}
pub enum trusted_key_options {}
pub enum tpm2_auth {}
pub enum device {}
pub enum cdev {}
pub enum rw_semaphore {}
pub enum mutex {}
pub enum dentry {}
pub enum attribute_group {}
pub enum hwrng {}
pub enum seq_operations {}
pub enum tpm_bank_info {}
pub enum tpm_buf {}
pub enum tpm_digest {}

pub type acpi_handle = *mut core::ffi::c_void;

#[repr(C)]
pub struct TpmOpsFlags;
pub const TPM_OPS_AUTO_STARTUP: u32 = 1 << 0;

#[repr(C)]
pub struct tpm_class_ops {
    pub flags: u32,
    pub req_complete_mask: u8,
    pub req_complete_val: u8,
    pub req_canceled: Option<unsafe extern "C" fn(*mut tpm_chip, u8) -> bool>,
    pub recv: Option<unsafe extern "C" fn(*mut tpm_chip, *mut u8, usize) -> i32>,
    pub send: Option<unsafe extern "C" fn(*mut tpm_chip, *mut u8, usize, usize) -> i32>,
    pub cancel: Option<unsafe extern "C" fn(*mut tpm_chip)>,
    pub status: Option<unsafe extern "C" fn(*mut tpm_chip) -> u8>,
    pub update_timeouts: Option<unsafe extern "C" fn(*mut tpm_chip, *mut usize)>,
    pub update_durations: Option<unsafe extern "C" fn(*mut tpm_chip, *mut usize)>,
    pub go_idle: Option<unsafe extern "C" fn(*mut tpm_chip) -> i32>,
    pub cmd_ready: Option<unsafe extern "C" fn(*mut tpm_chip) -> i32>,
    pub request_locality: Option<unsafe extern "C" fn(*mut tpm_chip, i32) -> i32>,
    pub relinquish_locality: Option<unsafe extern "C" fn(*mut tpm_chip, i32) -> i32>,
    pub clk_enable: Option<unsafe extern "C" fn(*mut tpm_chip, bool)>,
}

pub const TPM_NUM_EVENT_LOG_FILES: usize = 3;
pub const TPM_SHORT: usize = 0;
pub const TPM_MEDIUM: usize = 1;
pub const TPM_LONG: usize = 2;
pub const TPM_LONG_LONG: usize = 3;
pub const TPM_UNDEFINED: usize = 4;
pub const TPM_NUM_DURATIONS: usize = TPM_UNDEFINED;
pub const TPM_PPI_VERSION_LEN: usize = 3;
pub const EC_PT_SZ: usize = 32;
pub const TPM2_NAME_SIZE: usize = 34;
pub const TPM2_MAX_CONTEXT_SIZE: usize = 4096;

#[repr(C)]
pub struct tpm_space {
    pub context_tbl: [u32; 3], pub context_buf: *mut u8,
    pub session_tbl: [u32; 3], pub session_buf: *mut u8, pub buf_size: u32,
}
#[repr(C)]
pub struct tpm_bios_log { pub bios_event_log: *mut core::ffi::c_void, pub bios_event_log_end: *mut core::ffi::c_void }
#[repr(C)]
pub struct tpm_chip_seqops { pub chip: *mut tpm_chip, pub seqops: *const seq_operations }

#[repr(C)]
pub struct tpm_chip {
    pub dev: device, pub devs: device, pub cdev: cdev, pub cdevs: cdev,
    pub ops_sem: rw_semaphore, pub ops: *const tpm_class_ops,
    pub log: tpm_bios_log, pub bin_log_seqops: tpm_chip_seqops, pub ascii_log_seqops: tpm_chip_seqops,
    pub flags: u32, pub dev_num: i32, pub is_open: usize, pub hwrng_name: [i8; 64], pub hwrng: hwrng,
    pub tpm_mutex: mutex, pub timeout_a: usize, pub timeout_b: usize, pub timeout_c: usize, pub timeout_d: usize,
    pub timeout_adjusted: bool, pub duration: [usize; TPM_NUM_DURATIONS], pub duration_adjusted: bool,
    pub bios_dir: *mut dentry, pub groups: [*const attribute_group; 3 + 32], pub groups_cnt: u32,
    pub nr_allocated_banks: u32, pub allocated_banks: [tpm_bank_info; 16],
    // CONFIG_ACPI conditionally adds acpi_dev_handle and ppi_version.
    pub work_space: tpm_space, pub last_cc: u32, pub nr_commands: u32, pub cc_attrs_tbl: *mut u32,
    pub locality: i32,
    // CONFIG_TCG_TPM2_HMAC conditionally adds the NULL-seed contexts and auth pointer.
}

#[inline]
pub const fn tpm2_handle_mso(handle: u32) -> u32 { handle >> 24 }
pub const TPM_VID_INTEL: u32 = 0x8086;
pub const TPM_VID_WINBOND: u32 = 0x1050;
pub const TPM_VID_STM: u32 = 0x104A;
pub const TPM_VID_ATML: u32 = 0x1114;
pub const TPM_VID_IFX: u32 = 0x15D1;
pub const TPM_CHIP_FLAG_BOOTSTRAPPED: u32 = 1 << 0;
pub const TPM_CHIP_FLAG_TPM2: u32 = 1 << 1;
pub const TPM_CHIP_FLAG_IRQ: u32 = 1 << 2;
pub const TPM_CHIP_FLAG_VIRTUAL: u32 = 1 << 3;
pub const TPM_CHIP_FLAG_HAVE_TIMEOUTS: u32 = 1 << 4;
pub const TPM_CHIP_FLAG_ALWAYS_POWERED: u32 = 1 << 5;
pub const TPM_CHIP_FLAG_FIRMWARE_POWER_MANAGED: u32 = 1 << 6;
pub const TPM_CHIP_FLAG_FIRMWARE_UPGRADE: u32 = 1 << 7;
pub const TPM_CHIP_FLAG_SUSPENDED: u32 = 1 << 8;
pub const TPM_CHIP_FLAG_HWRNG_DISABLED: u32 = 1 << 9;
pub const TPM_CHIP_FLAG_DISABLE: u32 = 1 << 10;
pub const TPM_CHIP_FLAG_SYNC: u32 = 1 << 11;

#[repr(C)] pub struct tpm2_hash { pub crypto_id: u32, pub tpm_id: u32 }
#[inline] pub unsafe fn tpm_is_firmware_upgrade(chip: *mut tpm_chip) -> bool { ((*chip).flags & TPM_CHIP_FLAG_FIRMWARE_UPGRADE) != 0 }
#[inline] pub const fn tpm2_rc_value(rc: u32) -> u32 { if rc & (1 << 7) != 0 { rc & 0xbf } else { rc } }
#[inline] pub unsafe fn tpm_ret_to_err(ret: isize) -> isize {
    if ret < 0 { return ret; }
    match tpm2_rc_value(ret as u32) { TPM2_RC_SUCCESS => 0, TPM2_RC_SESSION_MEMORY => -12, TPM2_RC_HASH => -22, _ => -1 }
}

extern "C" {
    pub fn tpm_is_tpm2(chip: *mut tpm_chip) -> i32;
    pub fn tpm_try_get_ops(chip: *mut tpm_chip) -> i32;
    pub fn tpm_put_ops(chip: *mut tpm_chip);
    pub fn tpm_transmit_cmd(chip: *mut tpm_chip, buf: *mut tpm_buf, min_rsp_body_length: usize, desc: *const i8) -> isize;
    pub fn tpm_pcr_read(chip: *mut tpm_chip, pcr_idx: u32, digest: *mut tpm_digest) -> i32;
    pub fn tpm_pcr_extend(chip: *mut tpm_chip, pcr_idx: u32, digests: *mut tpm_digest) -> i32;
    pub fn tpm_get_random(chip: *mut tpm_chip, data: *mut u8, max: usize) -> i32;
    pub fn tpm_default_chip() -> *mut tpm_chip;
    pub fn tpm2_flush_context(chip: *mut tpm_chip, handle: u32);
    pub fn tpm2_find_hash_alg(crypto_id: u32) -> i32;
    pub fn tpm_buf_append_empty_auth(buf: *mut tpm_buf, handle: u32);
    pub fn tpm_buf_append_name(chip: *mut tpm_chip, buf: *mut tpm_buf, handle: u32, name: *mut u8) -> i32;
    pub fn tpm_buf_append_hmac_session(chip: *mut tpm_chip, buf: *mut tpm_buf, attributes: u8, passphrase: *mut u8, passphraselen: i32);
    pub fn tpm_buf_append_auth(chip: *mut tpm_chip, buf: *mut tpm_buf, passphrase: *mut u8, passphraselen: i32);
}

// Values supplied by the TPM command translation.
extern "C" { static TPM2_RC_SUCCESS: u32; static TPM2_RC_SESSION_MEMORY: u32; static TPM2_RC_HASH: u32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
