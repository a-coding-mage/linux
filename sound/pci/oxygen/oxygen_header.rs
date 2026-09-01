/* SPDX-License-Identifier: GPL-2.0 */

// Translated from pci/oxygen/oxygen.h.
// C include dependencies: linux/mutex.h, linux/spinlock.h, linux/wait.h,
// linux/workqueue.h, and oxygen_regs.h.

use core::ffi::{c_char, c_int, c_ulong, c_void};

pub type u8 = u8;
pub type u16 = u16;
pub type u32 = u32;
pub type size_t = usize;
pub type bool_ = bool;
pub type __le16 = u16;
pub type __le32 = u32;

// External C types declared by this header or supplied by included headers.
#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pci_device_id {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hardware {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_rawmidi {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}
#[repr(C)]
pub struct module {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

/* 1 << PCM_x == OXYGEN_CHANNEL_x */
pub const PCM_A: usize = 0;
pub const PCM_B: usize = 1;
pub const PCM_C: usize = 2;
pub const PCM_SPDIF: usize = 3;
pub const PCM_MULTICH: usize = 4;
pub const PCM_AC97: usize = 5;
pub const PCM_COUNT: usize = 6;

// C macro OXYGEN_MCLKS(f_single, f_double, f_quad) token-pastes MCLK_ names.
// Pass the already resolved MCLK_* values to preserve the arithmetic behavior.
#[inline]
pub const fn OXYGEN_MCLKS(f_single: u8, f_double: u8, f_quad: u8) -> u8 {
    ((f_single as u8) << 0) | ((f_double as u8) << 2) | ((f_quad as u8) << 4)
}

pub const OXYGEN_IO_SIZE: usize = 0x100;

pub const OXYGEN_EEPROM_ID: u16 = 0x434d; /* "CM" */

/* model-specific configuration of outputs/inputs */
pub const PLAYBACK_0_TO_I2S: u32 = 0x0001;
/* PLAYBACK_0_TO_AC97_0		not implemented */
pub const PLAYBACK_1_TO_SPDIF: u32 = 0x0004;
pub const PLAYBACK_2_TO_AC97_1: u32 = 0x0008;
pub const CAPTURE_0_FROM_I2S_1: u32 = 0x0010;
pub const CAPTURE_0_FROM_I2S_2: u32 = 0x0020;
/* CAPTURE_0_FROM_AC97_0		not implemented */
pub const CAPTURE_1_FROM_SPDIF: u32 = 0x0080;
pub const CAPTURE_2_FROM_I2S_2: u32 = 0x0100;
pub const CAPTURE_2_FROM_AC97_1: u32 = 0x0200;
pub const CAPTURE_3_FROM_I2S_3: u32 = 0x0400;
pub const MIDI_OUTPUT: u32 = 0x0800;
pub const MIDI_INPUT: u32 = 0x1000;
pub const AC97_CD_INPUT: u32 = 0x2000;
pub const AC97_FMIC_SWITCH: u32 = 0x4000;

pub const CONTROL_SPDIF_PCM: usize = 0;
pub const CONTROL_SPDIF_INPUT_BITS: usize = 1;
pub const CONTROL_MIC_CAPTURE_SWITCH: usize = 2;
pub const CONTROL_LINE_CAPTURE_SWITCH: usize = 3;
pub const CONTROL_CD_CAPTURE_SWITCH: usize = 4;
pub const CONTROL_AUX_CAPTURE_SWITCH: usize = 5;
pub const CONTROL_COUNT: usize = 6;

// C initializer-fragment macros:
// #define OXYGEN_PCI_SUBID(sv, sd) .vendor = PCI_VENDOR_ID_CMEDIA, .device = 0x8788, .subvendor = sv, .subdevice = sd
// #define OXYGEN_PCI_SUBID_BROKEN_EEPROM OXYGEN_PCI_SUBID(PCI_VENDOR_ID_CMEDIA, 0x8788), .driver_data = BROKEN_EEPROM_DRIVER_DATA
pub const BROKEN_EEPROM_DRIVER_DATA: c_ulong = !0 as c_ulong;

#[repr(C)]
pub struct oxygen_model {
    pub shortname: *const c_char,
    pub longname: *const c_char,
    pub chip: *const c_char,
    pub init: Option<unsafe extern "C" fn(chip: *mut oxygen)>,
    pub control_filter: Option<unsafe extern "C" fn(template: *mut snd_kcontrol_new) -> c_int>,
    pub mixer_init: Option<unsafe extern "C" fn(chip: *mut oxygen) -> c_int>,
    pub cleanup: Option<unsafe extern "C" fn(chip: *mut oxygen)>,
    pub suspend: Option<unsafe extern "C" fn(chip: *mut oxygen)>,
    pub resume: Option<unsafe extern "C" fn(chip: *mut oxygen)>,
    pub pcm_hardware_filter:
        Option<unsafe extern "C" fn(channel: u32, hardware: *mut snd_pcm_hardware)>,
    pub set_dac_params:
        Option<unsafe extern "C" fn(chip: *mut oxygen, params: *mut snd_pcm_hw_params)>,
    pub set_adc_params:
        Option<unsafe extern "C" fn(chip: *mut oxygen, params: *mut snd_pcm_hw_params)>,
    pub update_dac_volume: Option<unsafe extern "C" fn(chip: *mut oxygen)>,
    pub update_dac_mute: Option<unsafe extern "C" fn(chip: *mut oxygen)>,
    pub update_center_lfe_mix: Option<unsafe extern "C" fn(chip: *mut oxygen, mixed: bool_)>,
    pub adjust_dac_routing:
        Option<unsafe extern "C" fn(chip: *mut oxygen, play_routing: u32) -> u32>,
    pub gpio_changed: Option<unsafe extern "C" fn(chip: *mut oxygen)>,
    pub uart_input: Option<unsafe extern "C" fn(chip: *mut oxygen)>,
    pub ac97_switch: Option<unsafe extern "C" fn(chip: *mut oxygen, reg: u32, mute: u32)>,
    pub dump_registers:
        Option<unsafe extern "C" fn(chip: *mut oxygen, buffer: *mut snd_info_buffer)>,
    pub dac_tlv: *const u32,
    pub model_data_size: size_t,
    pub device_config: u32,
    pub dac_channels_pcm: u8,
    pub dac_channels_mixer: u8,
    pub dac_volume_min: u8,
    pub dac_volume_max: u8,
    pub misc_flags: u8,
    pub function_flags: u8,
    pub dac_mclks: u8,
    pub adc_mclks: u8,
    pub dac_i2s_format: u16,
    pub adc_i2s_format: u16,
}

#[repr(C)]
pub union oxygen_saved_registers {
    pub _8: [u8; OXYGEN_IO_SIZE],
    pub _16: [__le16; OXYGEN_IO_SIZE / 2],
    pub _32: [__le32; OXYGEN_IO_SIZE / 4],
}

#[repr(C)]
pub struct oxygen {
    pub addr: c_ulong,
    pub reg_lock: spinlock_t,
    pub mutex: mutex,
    pub card: *mut snd_card,
    pub pci: *mut pci_dev,
    pub midi: *mut snd_rawmidi,
    pub irq: c_int,
    pub model_data: *mut c_void,
    pub interrupt_mask: u32,
    pub dac_volume: [u8; 8],
    pub dac_mute: u8,
    pub pcm_active: u8,
    pub pcm_running: u8,
    pub dac_routing: u8,
    pub spdif_playback_enable: u8,
    pub has_ac97_0: u8,
    pub has_ac97_1: u8,
    pub spdif_bits: u32,
    pub spdif_pcm_bits: u32,
    pub streams: [*mut snd_pcm_substream; PCM_COUNT],
    pub controls: [*mut snd_kcontrol; CONTROL_COUNT],
    pub spdif_input_bits_work: work_struct,
    pub gpio_work: work_struct,
    pub ac97_waitqueue: wait_queue_head_t,
    pub saved_registers: oxygen_saved_registers,
    pub saved_ac97_registers: [[u16; 0x40]; 2],
    pub uart_input_count: u32,
    pub uart_input: [u8; 32],
    pub model: oxygen_model,
}

unsafe extern "C" {
    /* oxygen_lib.c */

    pub fn oxygen_pci_probe(
        pci: *mut pci_dev,
        index: c_int,
        id: *mut c_char,
        owner: *mut module,
        ids: *const pci_device_id,
        get_model: Option<unsafe extern "C" fn(chip: *mut oxygen, id: *const pci_device_id) -> c_int>,
    ) -> c_int;
    pub static oxygen_pci_pm: dev_pm_ops;
    pub fn oxygen_pci_shutdown(pci: *mut pci_dev);

    /* oxygen_mixer.c */

    pub fn oxygen_mixer_init(chip: *mut oxygen) -> c_int;
    pub fn oxygen_update_dac_routing(chip: *mut oxygen);
    pub fn oxygen_update_spdif_source(chip: *mut oxygen);

    /* oxygen_pcm.c */

    pub fn oxygen_pcm_init(chip: *mut oxygen) -> c_int;

    /* oxygen_io.c */

    pub fn oxygen_read8(chip: *mut oxygen, reg: u32) -> u8;
    pub fn oxygen_read16(chip: *mut oxygen, reg: u32) -> u16;
    pub fn oxygen_read32(chip: *mut oxygen, reg: u32) -> u32;
    pub fn oxygen_write8(chip: *mut oxygen, reg: u32, value: u8);
    pub fn oxygen_write16(chip: *mut oxygen, reg: u32, value: u16);
    pub fn oxygen_write32(chip: *mut oxygen, reg: u32, value: u32);
    pub fn oxygen_write8_masked(chip: *mut oxygen, reg: u32, value: u8, mask: u8);
    pub fn oxygen_write16_masked(chip: *mut oxygen, reg: u32, value: u16, mask: u16);
    pub fn oxygen_write32_masked(chip: *mut oxygen, reg: u32, value: u32, mask: u32);

    pub fn oxygen_read_ac97(chip: *mut oxygen, codec: u32, index: u32) -> u16;
    pub fn oxygen_write_ac97(chip: *mut oxygen, codec: u32, index: u32, data: u16);
    pub fn oxygen_write_ac97_masked(
        chip: *mut oxygen,
        codec: u32,
        index: u32,
        data: u16,
        mask: u16,
    );

    pub fn oxygen_write_spi(chip: *mut oxygen, control: u8, data: u32) -> c_int;
    pub fn oxygen_write_i2c(chip: *mut oxygen, device: u8, map: u8, data: u8);

    pub fn oxygen_reset_uart(chip: *mut oxygen);
    pub fn oxygen_write_uart(chip: *mut oxygen, data: u8);

    pub fn oxygen_read_eeprom(chip: *mut oxygen, index: u32) -> u16;
    pub fn oxygen_write_eeprom(chip: *mut oxygen, index: u32, value: u16);
}

#[inline]
pub unsafe fn oxygen_set_bits8(chip: *mut oxygen, reg: u32, value: u8) {
    unsafe { oxygen_write8_masked(chip, reg, value, value) };
}

#[inline]
pub unsafe fn oxygen_set_bits16(chip: *mut oxygen, reg: u32, value: u16) {
    unsafe { oxygen_write16_masked(chip, reg, value, value) };
}

#[inline]
pub unsafe fn oxygen_set_bits32(chip: *mut oxygen, reg: u32, value: u32) {
    unsafe { oxygen_write32_masked(chip, reg, value, value) };
}

#[inline]
pub unsafe fn oxygen_clear_bits8(chip: *mut oxygen, reg: u32, value: u8) {
    unsafe { oxygen_write8_masked(chip, reg, 0, value) };
}

#[inline]
pub unsafe fn oxygen_clear_bits16(chip: *mut oxygen, reg: u32, value: u16) {
    unsafe { oxygen_write16_masked(chip, reg, 0, value) };
}

#[inline]
pub unsafe fn oxygen_clear_bits32(chip: *mut oxygen, reg: u32, value: u32) {
    unsafe { oxygen_write32_masked(chip, reg, 0, value) };
}

#[inline]
pub unsafe fn oxygen_ac97_set_bits(chip: *mut oxygen, codec: u32, index: u32, value: u16) {
    unsafe { oxygen_write_ac97_masked(chip, codec, index, value, value) };
}

#[inline]
pub unsafe fn oxygen_ac97_clear_bits(chip: *mut oxygen, codec: u32, index: u32, value: u16) {
    unsafe { oxygen_write_ac97_masked(chip, codec, index, 0, value) };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
