/* SPDX-License-Identifier: GPL-2.0-only */
/* Common definitions for Cirrus Logic CS35L56 smart amp. */

// Kernel headers and external types are supplied by the surrounding translation.

pub const CS35L56_DEVID: u32 = 0x0000000;
pub const CS35L56_REVID: u32 = 0x0000004;
pub const CS35L56_RELID: u32 = 0x000000C;
pub const CS35L56_OTPID: u32 = 0x0000010;
pub const CS35L56_SFT_RESET: u32 = 0x0000020;
pub const CS35L56_GLOBAL_ENABLES: u32 = 0x0002014;
pub const CS35L56_BLOCK_ENABLES: u32 = 0x0002018;
pub const CS35L56_BLOCK_ENABLES2: u32 = 0x000201C;
pub const CS35L56_SYNC_GPIO1_CFG: u32 = 0x0002410;
pub const CS35L56_ASP2_DIO_GPIO13_CFG: u32 = 0x0002440;
pub const CS35L56_UPDATE_REGS: u32 = 0x0002A0C;
pub const CS35L56_REFCLK_INPUT: u32 = 0x0002C04;
pub const CS35L56_GLOBAL_SAMPLE_RATE: u32 = 0x0002C0C;
pub const CS35L56_ASP1_ENABLES1: u32 = 0x0004800;
pub const CS35L56_ASP1_CONTROL1: u32 = 0x0004804;
pub const CS35L56_ASP1_CONTROL2: u32 = 0x0004808;
pub const CS35L56_ASP1_CONTROL3: u32 = 0x000480C;
pub const CS35L56_ASP1_FRAME_CONTROL1: u32 = 0x0004810;
pub const CS35L56_ASP1_FRAME_CONTROL5: u32 = 0x0004820;
pub const CS35L56_ASP1_DATA_CONTROL1: u32 = 0x0004830;
pub const CS35L56_ASP1_DATA_CONTROL5: u32 = 0x0004840;
pub const CS35L56_DACPCM1_INPUT: u32 = 0x0004C00;
pub const CS35L56_DACPCM2_INPUT: u32 = 0x0004C08;
pub const CS35L56_ASP1TX1_INPUT: u32 = 0x0004C20;
pub const CS35L56_ASP1TX2_INPUT: u32 = 0x0004C24;
pub const CS35L56_ASP1TX3_INPUT: u32 = 0x0004C28;
pub const CS35L56_ASP1TX4_INPUT: u32 = 0x0004C2C;
pub const CS35L56_DSP1RX1_INPUT: u32 = 0x0004C40;
pub const CS35L56_DSP1RX2_INPUT: u32 = 0x0004C44;
pub const CS35L56_SWIRE_DP3_CH1_INPUT: u32 = 0x0004C70;
pub const CS35L56_SWIRE_DP3_CH2_INPUT: u32 = 0x0004C74;
pub const CS35L56_SWIRE_DP3_CH3_INPUT: u32 = 0x0004C78;
pub const CS35L56_SWIRE_DP3_CH4_INPUT: u32 = 0x0004C7C;
pub const CS35L56_IRQ1_CFG: u32 = 0x000E000;
pub const CS35L56_IRQ1_STATUS: u32 = 0x000E004;
pub const CS35L56_IRQ1_EINT_1: u32 = 0x000E010;
pub const CS35L56_IRQ1_EINT_2: u32 = 0x000E014;
pub const CS35L56_IRQ1_EINT_4: u32 = 0x000E01C;
pub const CS35L56_IRQ1_EINT_8: u32 = 0x000E02C;
pub const CS35L56_IRQ1_EINT_18: u32 = 0x000E054;
pub const CS35L56_IRQ1_EINT_20: u32 = 0x000E05C;
pub const CS35L56_IRQ1_MASK_1: u32 = 0x000E090;
pub const CS35L56_IRQ1_MASK_2: u32 = 0x000E094;
pub const CS35L56_IRQ1_MASK_4: u32 = 0x000E09C;
pub const CS35L56_IRQ1_MASK_8: u32 = 0x000E0AC;
pub const CS35L56_IRQ1_MASK_18: u32 = 0x000E0D4;
pub const CS35L56_IRQ1_MASK_20: u32 = 0x000E0DC;
pub const CS35L56_GPIO_STATUS1: u32 = 0x000F000;
pub const CS35L56_GPIO1_CTRL1: u32 = 0x000F008;
pub const CS35L56_GPIO13_CTRL1: u32 = 0x000F038;
pub const CS35L56_MIXER_NGATE_CH1_CFG: u32 = 0x0010004;
pub const CS35L56_MIXER_NGATE_CH2_CFG: u32 = 0x0010008;
pub const CS35L56_DSP_MBOX_1_RAW: u32 = 0x0011000;
pub const CS35L56_DSP_VIRTUAL1_MBOX_1: u32 = 0x0011020;
pub const CS35L56_DSP_VIRTUAL1_MBOX_2: u32 = 0x0011024;
pub const CS35L56_DSP_VIRTUAL1_MBOX_3: u32 = 0x0011028;
pub const CS35L56_DSP_VIRTUAL1_MBOX_4: u32 = 0x001102C;
pub const CS35L56_DSP_VIRTUAL1_MBOX_5: u32 = 0x0011030;
pub const CS35L56_DSP_VIRTUAL1_MBOX_6: u32 = 0x0011034;
pub const CS35L56_DSP_VIRTUAL1_MBOX_7: u32 = 0x0011038;
pub const CS35L56_DSP_VIRTUAL1_MBOX_8: u32 = 0x001103C;
pub const CS35L56_DIE_STS1: u32 = 0x0017040;
pub const CS35L56_DIE_STS2: u32 = 0x0017044;
pub const CS35L56_DSP_RESTRICT_STS1: u32 = 0x00190F0;
pub const CS35L56_OTP_MEM_53: u32 = 0x00300D4;
pub const CS35L56_OTP_MEM_54: u32 = 0x00300D8;
pub const CS35L56_OTP_MEM_55: u32 = 0x00300DC;
pub const CS35L56_DSP1_XMEM_PACKED_0: u32 = 0x2000000;
pub const CS35L56_DSP1_XMEM_PACKED_6143: u32 = 0x2005FFC;
pub const CS35L56_DSP1_XMEM_UNPACKED32_0: u32 = 0x2400000;
pub const CS35L56_DSP1_XMEM_UNPACKED32_4095: u32 = 0x2403FFC;
pub const CS35L56_DSP1_SYS_INFO_ID: u32 = 0x25E0000;
pub const CS35L56_DSP1_SYS_INFO_END: u32 = 0x25E004C;
pub const CS35L56_DSP1_AHBM_WINDOW_DEBUG_0: u32 = 0x25E2040;
pub const CS35L56_DSP1_AHBM_WINDOW_DEBUG_1: u32 = 0x25E2044;
pub const CS35L56_DSP1_XMEM_UNPACKED24_0: u32 = 0x2800000;
pub const CS35L56_DSP1_FW_VER: u32 = 0x2800010;
pub const CS35L56_DSP1_HALO_STATE: u32 = 0x28021E0;
pub const CS35L56_B2_DSP1_HALO_STATE: u32 = 0x2803D20;
pub const CS35L56_DSP1_PM_CUR_STATE: u32 = 0x2804308;
pub const CS35L56_B2_DSP1_PM_CUR_STATE: u32 = 0x2804678;
pub const CS35L56_DSP1_XMEM_UNPACKED24_8191: u32 = 0x2807FFC;
pub const CS35L56_DSP1_CORE_BASE: u32 = 0x2B80000;
pub const CS35L56_DSP1_SCRATCH1: u32 = 0x2B805C0;
pub const CS35L56_DSP1_SCRATCH2: u32 = 0x2B805C8;
pub const CS35L56_DSP1_SCRATCH3: u32 = 0x2B805D0;
pub const CS35L56_DSP1_SCRATCH4: u32 = 0x2B805D8;
pub const CS35L56_DSP1_YMEM_PACKED_0: u32 = 0x2C00000;
pub const CS35L56_DSP1_YMEM_PACKED_4604: u32 = 0x2C047F0;
pub const CS35L56_DSP1_YMEM_UNPACKED32_0: u32 = 0x3000000;
pub const CS35L56_DSP1_YMEM_UNPACKED32_3070: u32 = 0x3002FF8;
pub const CS35L56_DSP1_YMEM_UNPACKED24_0: u32 = 0x3400000;
pub const CS35L56_MAIN_RENDER_USER_MUTE: u32 = 0x3400024;
pub const CS35L56_MAIN_RENDER_USER_VOLUME: u32 = 0x340002C;
pub const CS35L56_MAIN_POSTURE_NUMBER: u32 = 0x3400094;
pub const CS35L56_PROTECTION_STATUS: u32 = 0x34000D8;
pub const CS35L56_TRANSDUCER_ACTUAL_PS: u32 = 0x3400150;
pub const CS35L56_DSP1_YMEM_UNPACKED24_6141: u32 = 0x3405FF4;
pub const CS35L56_DSP1_PMEM_0: u32 = 0x3800000;
pub const CS35L56_DSP1_PMEM_5114: u32 = 0x3804FE8;

pub const CS35L63_DSP1_FW_VER: u32 = CS35L56_DSP1_FW_VER;
pub const CS35L63_DSP1_HALO_STATE: u32 = 0x2803C04;
pub const CS35L63_DSP1_PM_CUR_STATE: u32 = 0x2804518;
pub const CS35L63_PROTECTION_STATUS: u32 = 0x340009C;
pub const CS35L63_TRANSDUCER_ACTUAL_PS: u32 = 0x34000F4;
pub const CS35L63_MAIN_RENDER_USER_MUTE: u32 = 0x3400020;
pub const CS35L63_MAIN_RENDER_USER_VOLUME: u32 = 0x3400028;
pub const CS35L63_MAIN_POSTURE_NUMBER: u32 = 0x3400068;

pub const CS35L56_DEVID_MASK: u32 = 0x00FFFFFF;
pub const CS35L56_AREVID_MASK: u32 = 0x000000F0;
pub const CS35L56_MTLREVID_MASK: u32 = 0x0000000F;
pub const CS35L56_REVID_B0: u32 = 0x000000B0;
pub const CS35L56_PAD_GPIO_PULL_MASK: u32 = 0xC;
pub const CS35L56_PAD_GPIO_IE: u32 = 1;
pub const CS35L56_PAD_PULL_NONE: u32 = 0;
pub const CS35L56_PAD_PULL_UP: u32 = 1;
pub const CS35L56_PAD_PULL_DOWN: u32 = 2;
pub const CS35L56_UPDT_GPIO_PRES: u32 = 1 << 6;
pub const CS35L56_ASP_RX2_EN_SHIFT: u32 = 17;
pub const CS35L56_ASP_RX1_EN_SHIFT: u32 = 16;
pub const CS35L56_ASP_TX4_EN_SHIFT: u32 = 3;
pub const CS35L56_ASP_TX3_EN_SHIFT: u32 = 2;
pub const CS35L56_ASP_TX2_EN_SHIFT: u32 = 1;
pub const CS35L56_ASP_TX1_EN_SHIFT: u32 = 0;
pub const CS35L56_ASP_BCLK_FREQ_MASK: u32 = 0x3F;
pub const CS35L56_ASP_BCLK_FREQ_SHIFT: u32 = 0;
pub const CS35L56_ASP_RX_WIDTH_MASK: u32 = 0xFF000000;
pub const CS35L56_ASP_RX_WIDTH_SHIFT: u32 = 24;
pub const CS35L56_ASP_TX_WIDTH_MASK: u32 = 0x00FF0000;
pub const CS35L56_ASP_TX_WIDTH_SHIFT: u32 = 16;
pub const CS35L56_ASP_FMT_MASK: u32 = 0x700;
pub const CS35L56_ASP_FMT_SHIFT: u32 = 8;
pub const CS35L56_ASP_BCLK_INV_MASK: u32 = 0x40;
pub const CS35L56_ASP_FSYNC_INV_MASK: u32 = 0x04;
pub const CS35L56_ASP1_DOUT_HIZ_CTRL_MASK: u32 = 0x3;
pub const CS35L56_ASP_TX_WL_MASK: u32 = 0x3F;
pub const CS35L56_ASP_RX_WL_MASK: u32 = 0x3F;
pub const CS35L56_ASP_TXN_SRC_MASK: u32 = 0x7F;
pub const CS35L56_SWIRETXN_SRC_MASK: u32 = 0x7F;
pub const CS35L56_IRQ1_STS_MASK: u32 = 1;
pub const CS35L56_AMP_SHORT_ERR_EINT1_MASK: u32 = 0x80000000;
pub const CS35L56_DSP_VIRTUAL2_MBOX_WR_EINT1_MASK: u32 = 0x00200000;
pub const CS35L56_OTP_BOOT_DONE_MASK: u32 = 2;
pub const CS35L56_TEMP_ERR_EINT1_MASK: u32 = 0x80000000;
pub const CS35L56_AUX_NGATE_CHN_EN: u32 = 1;
pub const CS35L56_GPIO_DIR_MASK: u32 = 1 << 31;
pub const CS35L56_GPIO_FN_MASK: u32 = 7;
pub const CS35L56_GPIO_FN_GPIO: u32 = 1;
pub const CS35L56_INPUT_SRC_NONE: u32 = 0;
pub const CS35L56_INPUT_SRC_ASP1RX1: u32 = 8;
pub const CS35L56_INPUT_SRC_ASP1RX2: u32 = 9;
pub const CS35L56_INPUT_SRC_VMON: u32 = 0x18;
pub const CS35L56_INPUT_SRC_IMON: u32 = 0x19;
pub const CS35L56_INPUT_SRC_ERR_VOL: u32 = 0x20;
pub const CS35L56_INPUT_SRC_CLASSH: u32 = 0x21;
pub const CS35L56_INPUT_SRC_VDDBMON: u32 = 0x28;
pub const CS35L56_INPUT_SRC_VBSTMON: u32 = 0x29;
pub const CS35L56_INPUT_SRC_DSP1TX1: u32 = 0x32;
pub const CS35L56_INPUT_SRC_DSP1TX2: u32 = 0x33;
pub const CS35L56_INPUT_SRC_DSP1TX3: u32 = 0x34;
pub const CS35L56_INPUT_SRC_DSP1TX4: u32 = 0x35;
pub const CS35L56_INPUT_SRC_DSP1TX5: u32 = 0x36;
pub const CS35L56_INPUT_SRC_DSP1TX6: u32 = 0x37;
pub const CS35L56_INPUT_SRC_DSP1TX7: u32 = 0x38;
pub const CS35L56_INPUT_SRC_DSP1TX8: u32 = 0x39;
pub const CS35L56_INPUT_SRC_TEMPMON: u32 = 0x3A;
pub const CS35L56_INPUT_SRC_INTERPOLATOR: u32 = 0x40;
pub const CS35L56_INPUT_SRC_SWIRE_DP1_CHANNEL1: u32 = 0x44;
pub const CS35L56_INPUT_SRC_SWIRE_DP1_CHANNEL2: u32 = 0x45;
pub const CS35L56_INPUT_MASK: u32 = 0x7F;
pub const CS35L56_NUM_INPUT_SRC: usize = 21;
pub const CS35L56_ASP_FMT_DSP_A: u32 = 0;
pub const CS35L56_ASP_FMT_I2S: u32 = 2;
pub const CS35L56_ASP_UNUSED_HIZ_OFF_HIZ: u32 = 3;
pub const CS35L56_PS0: u32 = 0;
pub const CS35L56_PS3: u32 = 3;
pub const CS35L56_RESTRICTED_MASK: u32 = 7;
pub const CS35L56_MAIN_RENDER_USER_MUTE_MASK: u32 = 1;
pub const CS35L56_MAIN_RENDER_USER_VOLUME_MIN: i32 = -400;
pub const CS35L56_MAIN_RENDER_USER_VOLUME_MAX: i32 = 48;
pub const CS35L56_MAIN_RENDER_USER_VOLUME_MASK: u32 = 0x0000FFC0;
pub const CS35L56_MAIN_RENDER_USER_VOLUME_SHIFT: u32 = 6;
pub const CS35L56_MAIN_RENDER_USER_VOLUME_SIGNBIT: u32 = 9;
pub const CS35L56_MAIN_POSTURE_MIN: u32 = 0;
pub const CS35L56_MAIN_POSTURE_MAX: u32 = 255;
pub const CS35L56_MAIN_POSTURE_MASK: u32 = CS35L56_MAIN_POSTURE_MAX;
pub const CS35L56_FIRMWARE_MISSING: u32 = 1;
pub const CS35L56_HALO_STATE_SHUTDOWN: u32 = 1;
pub const CS35L56_HALO_STATE_BOOT_DONE: u32 = 2;
pub const CS35L56_MBOX_CMD_PING: u32 = 0x0A000000;
pub const CS35L56_MBOX_CMD_AUDIO_PLAY: u32 = 0x0B000001;
pub const CS35L56_MBOX_CMD_AUDIO_PAUSE: u32 = 0x0B000002;
pub const CS35L56_MBOX_CMD_AUDIO_REINIT: u32 = 0x0B000003;
pub const CS35L56_MBOX_CMD_AUDIO_CALIBRATION: u32 = 0x0B000006;
pub const CS35L56_MBOX_CMD_HIBERNATE_NOW: u32 = 0x02000001;
pub const CS35L56_MBOX_CMD_WAKEUP: u32 = 0x02000002;
pub const CS35L56_MBOX_CMD_PREVENT_AUTO_HIBERNATE: u32 = 0x02000003;
pub const CS35L56_MBOX_CMD_ALLOW_AUTO_HIBERNATE: u32 = 0x02000004;
pub const CS35L56_MBOX_CMD_SHUTDOWN: u32 = 0x02000005;
pub const CS35L56_MBOX_CMD_SYSTEM_RESET: u32 = 0x02000007;
pub const CS35L56_MBOX_TIMEOUT_US: u32 = 5000;
pub const CS35L56_MBOX_POLL_US: u32 = 250;
pub const CS35L56_FW_REQ_ACTIVE_TIMEOUT_MS: u32 = 250;
pub const CS35L56_PS0_POLL_US: u32 = 500;
pub const CS35L56_PS0_TIMEOUT_US: u32 = 50000;
pub const CS35L56_PS3_POLL_US: u32 = 500;
pub const CS35L56_PS3_TIMEOUT_US: u32 = 300000;
pub const CS35L56_CAL_STATUS_SUCCESS: u32 = 1;
pub const CS35L56_CAL_STATUS_OUT_OF_RANGE: u32 = 3;
pub const CS35L56_CAL_SET_STATUS_UNKNOWN: u32 = 0;
pub const CS35L56_CAL_SET_STATUS_DEFAULT: u32 = 1;
pub const CS35L56_CAL_SET_STATUS_SET: u32 = 2;
pub const CS35L56_CONTROL_PORT_READY_US: u32 = 2200;
pub const CS35L56_HALO_STATE_POLL_US: u32 = 1000;
pub const CS35L56_HALO_STATE_TIMEOUT_US: u32 = 250000;
pub const CS35L56_RESET_PULSE_MIN_US: u32 = 1100;
pub const CS35L56_WAKE_HOLD_TIME_US: u32 = 1000;
pub const CS35L56_PAD_PULL_SETTLE_US: u32 = 10;
pub const CS35L56_CALIBRATION_POLL_US: u32 = 100 * 1000;
pub const CS35L56_CALIBRATION_TIMEOUT_US: u32 = 5 * 1_000_000;
pub const CS35L56_SDW1_PLAYBACK_PORT: u32 = 1;
pub const CS35L56_SDW1_CAPTURE_PORT: u32 = 3;
pub const CS35L56_NUM_BULK_SUPPLIES: usize = 3;
pub const CS35L56_NUM_DSP_REGIONS: usize = 5;
pub const CS35L56_MAX_GPIO: usize = 13;
pub const CS35L63_MAX_GPIO: usize = 9;
pub const CS35L56_SPI_RESET_TO_PORT_READY_US: u32 = CS35L56_CONTROL_PORT_READY_US + 2500;

#[repr(C, packed)]
pub struct Cs35l56SpiPayload { pub addr: u32, pub pad: u16, pub value: u32 }

#[repr(C)]
pub struct Cs35l56FwReg {
    pub fw_ver: u32, pub halo_state: u32, pub pm_cur_stat: u32, pub prot_sts: u32,
    pub transducer_actual_ps: u32, pub user_mute: u32, pub user_volume: u32, pub posture_number: u32,
}

// The following structures retain the C layout and external kernel types.
#[repr(C)]
pub struct Cs35l56CalDebugfsFops { pub calibrate: debugfs_short_fops, pub cal_temperature: debugfs_short_fops, pub cal_data: debugfs_short_fops }

#[repr(C)]
pub struct Cs35l56Base {
    pub dev: *mut device, pub regmap: *mut regmap, pub dsp: *mut cs_dsp, pub irq: i32,
    pub irq_lock: mutex, pub type_: u8, pub rev: u8, pub init_done: bool, pub fw_patched: bool,
    pub secured: bool, pub can_hibernate: bool, pub cal_data_valid: bool, pub cal_index: i8,
    pub num_amps: u8, pub cal_data: cirrus_amp_cal_data, pub reset_gpio: *mut gpio_desc,
    pub spi_payload_buf: *mut Cs35l56SpiPayload, pub fw_reg: *const Cs35l56FwReg,
    pub calibration_controls: *const cirrus_amp_cal_controls, pub debugfs: *mut dentry,
    pub silicon_uid: u64, pub onchip_spkid_gpios: [u8; 5], pub num_onchip_spkid_gpios: u8,
    pub onchip_spkid_pulls: [u8; 5], pub num_onchip_spkid_pulls: u8,
}

#[inline]
pub const unsafe fn cs35l56_is_otp_register(reg: u32) -> bool { (reg >> 16) == 3 }

// Direct equivalents of the header's inline helpers; allocator/configuration
// primitives are supplied by the kernel-facing translation.
extern "C" {
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
}
pub const GFP_KERNEL: u32 = 0;
pub const GFP_DMA: u32 = 0;
pub const ENOMEM: i32 = 12;

#[inline]
pub unsafe fn cs35l56_init_config_for_spi(cs35l56: *mut Cs35l56Base, spi: *mut spi_device) -> i32 {
    (*cs35l56).spi_payload_buf = devm_kzalloc(&mut (*spi).dev, core::mem::size_of::<Cs35l56SpiPayload>(), GFP_KERNEL | GFP_DMA) as *mut Cs35l56SpiPayload;
    if (*cs35l56).spi_payload_buf.is_null() { return -ENOMEM; }
    0
}

#[inline]
pub unsafe fn cs35l56_is_spi(cs35l56: *mut Cs35l56Base) -> bool {
    (*cs35l56).spi_payload_buf != core::ptr::null_mut()
}

// Declaration-only external interfaces from the header.
extern "C" {
    pub static cs35l56_regmap_i2c: regmap_config;
    pub static cs35l56_regmap_spi: regmap_config;
    pub static cs35l56_regmap_sdw: regmap_config;
    pub static cs35l63_regmap_i2c: regmap_config;
    pub static cs35l63_regmap_sdw: regmap_config;
    pub static cs35l56_calibration_controls: cirrus_amp_cal_controls;
    pub static cs35l56_cal_set_status_text: [*const core::ffi::c_char; 3];
    pub static cs35l56_tx_input_texts: [*const core::ffi::c_char; CS35L56_NUM_INPUT_SRC];
    pub static cs35l56_tx_input_values: [u32; CS35L56_NUM_INPUT_SRC];
}

extern "C" {
    pub fn cs35l56_set_asp_patch(base: *mut Cs35l56Base) -> i32;
    pub fn cs35l56_set_patch(base: *mut Cs35l56Base) -> i32;
    pub fn cs35l56_mbox_send(base: *mut Cs35l56Base, command: u32) -> i32;
    pub fn cs35l56_firmware_shutdown(base: *mut Cs35l56Base) -> i32;
    pub fn cs35l56_wait_for_firmware_boot(base: *mut Cs35l56Base) -> i32;
    pub fn cs35l56_wait_control_port_ready();
    pub fn cs35l56_wait_min_reset_pulse();
    pub fn cs35l56_system_reset(base: *mut Cs35l56Base, is_soundwire: bool);
    pub fn cs35l56_irq_request(base: *mut Cs35l56Base, irq: i32) -> i32;
    pub fn cs35l56_is_fw_reload_needed(base: *mut Cs35l56Base) -> i32;
    pub fn cs35l56_runtime_suspend_common(base: *mut Cs35l56Base) -> i32;
    pub fn cs35l56_runtime_resume_common(base: *mut Cs35l56Base, is_soundwire: bool) -> i32;
    pub fn cs35l56_init_cs_dsp(base: *mut Cs35l56Base, dsp: *mut cs_dsp);
    pub fn cs35l56_get_calibration(base: *mut Cs35l56Base) -> i32;
    pub fn cs35l56_stash_calibration(base: *mut Cs35l56Base, data: *const cirrus_amp_cal_data) -> i32;
    pub fn cs35l56_calibrate_debugfs_write(base: *mut Cs35l56Base, from: *const core::ffi::c_char, count: usize, ppos: *mut i64) -> isize;
    pub fn cs35l56_cal_ambient_debugfs_write(base: *mut Cs35l56Base, from: *const core::ffi::c_char, count: usize, ppos: *mut i64) -> isize;
    pub fn cs35l56_cal_data_debugfs_read(base: *mut Cs35l56Base, to: *mut core::ffi::c_char, count: usize, ppos: *mut i64) -> isize;
    pub fn cs35l56_cal_data_debugfs_write(base: *mut Cs35l56Base, from: *const core::ffi::c_char, count: usize, ppos: *mut i64) -> isize;
    pub fn cs35l56_factory_calibrate(base: *mut Cs35l56Base) -> i32;
    pub fn cs35l56_create_cal_debugfs(base: *mut Cs35l56Base, fops: *const Cs35l56CalDebugfsFops);
    pub fn cs35l56_remove_cal_debugfs(base: *mut Cs35l56Base);
    pub fn cs35l56_cal_set_status_get(base: *mut Cs35l56Base, uvalue: *mut snd_ctl_elem_value) -> i32;
    pub fn cs35l56_read_prot_status(base: *mut Cs35l56Base, fw_missing: *mut bool, fw_version: *mut u32) -> i32;
    pub fn cs35l56_warn_if_firmware_missing(base: *mut Cs35l56Base);
    pub fn cs35l56_log_tuning(base: *mut Cs35l56Base, dsp: *mut cs_dsp);
    pub fn cs35l56_hw_init(base: *mut Cs35l56Base) -> i32;
    pub fn cs35l56_get_speaker_id(base: *mut Cs35l56Base) -> i32;
    pub fn cs35l56_check_and_save_onchip_spkid_gpios(base: *mut Cs35l56Base, gpios: *const u32, num_gpios: i32, pulls: *const u32, num_pulls: i32) -> i32;
    pub fn cs35l56_configure_onchip_spkid_pads(base: *mut Cs35l56Base) -> i32;
    pub fn cs35l56_read_onchip_spkid(base: *mut Cs35l56Base) -> i32;
    pub fn cs35l56_get_bclk_freq_id(freq: u32) -> i32;
    pub fn cs35l56_fill_supply_names(data: *mut regulator_bulk_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
