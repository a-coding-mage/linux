/* SPDX-License-Identifier: GPL-2.0+
 *
 * Universal interface for Audio Codec '97
 */

/* C header dependencies are supplied by other translated units. */

pub const AC97_BUS_MAX_DEVICES: usize = 4;

pub const AC97_SIGMATEL_OUTSEL: u16 = 0x64;
pub const AC97_SIGMATEL_INSEL: u16 = 0x66;
pub const AC97_SIGMATEL_IOMISC: u16 = 0x68;
pub const AC97_SIGMATEL_ANALOG: u16 = 0x6c;
pub const AC97_SIGMATEL_DAC2INVERT: u16 = 0x6e;
pub const AC97_SIGMATEL_BIAS1: u16 = 0x70;
pub const AC97_SIGMATEL_BIAS2: u16 = 0x72;
pub const AC97_SIGMATEL_VARIOUS: u16 = 0x72;
pub const AC97_SIGMATEL_MULTICHN: u16 = 0x74;
pub const AC97_SIGMATEL_CIC1: u16 = 0x76;
pub const AC97_SIGMATEL_CIC2: u16 = 0x78;

pub const AC97_AD_TEST: u16 = 0x5a;
pub const AC97_AD_TEST2: u16 = 0x5c;
pub const AC97_AD_HPFD_SHIFT: u32 = 12;
pub const AC97_AD_CODEC_CFG: u16 = 0x70;
pub const AC97_AD_JACK_SPDIF: u16 = 0x72;
pub const AC97_AD_SERIAL_CFG: u16 = 0x74;
pub const AC97_AD_MISC: u16 = 0x76;
pub const AC97_AD_VREFD_SHIFT: u32 = 2;

pub const AC97_CSR_ACMODE: u16 = 0x5e;
pub const AC97_CSR_MISC_CRYSTAL: u16 = 0x60;
pub const AC97_CSR_SPDIF: u16 = 0x68;
pub const AC97_CSR_SERIAL: u16 = 0x6a;
pub const AC97_CSR_SPECF_ADDR: u16 = 0x6c;
pub const AC97_CSR_SPECF_DATA: u16 = 0x6e;
pub const AC97_CSR_BDI_STATUS: u16 = 0x7a;

pub const AC97_CXR_AUDIO_MISC: u16 = 0x5c;
pub const AC97_CXR_SPDIFEN: u16 = 1 << 3;
pub const AC97_CXR_COPYRGT: u16 = 1 << 2;
pub const AC97_CXR_SPDIF_MASK: u16 = 3 << 0;
pub const AC97_CXR_SPDIF_PCM: u16 = 0x0;
pub const AC97_CXR_SPDIF_AC3: u16 = 0x2;

pub const AC97_ALC650_SPDIF_INPUT_STATUS1: u16 = 0x60;
pub const AC97_ALC650_PRO: u16 = 0x0001;
pub const AC97_ALC650_NAUDIO: u16 = 0x0002;
pub const AC97_ALC650_COPY: u16 = 0x0004;
pub const AC97_ALC650_PRE: u16 = 0x0038;
pub const AC97_ALC650_PRE_SHIFT: u32 = 3;
pub const AC97_ALC650_MODE: u16 = 0x00c0;
pub const AC97_ALC650_MODE_SHIFT: u32 = 6;
pub const AC97_ALC650_CC_MASK: u16 = 0x7f00;
pub const AC97_ALC650_CC_SHIFT: u32 = 8;
pub const AC97_ALC650_L: u16 = 0x8000;
pub const AC97_ALC650_SPDIF_INPUT_STATUS2: u16 = 0x62;
pub const AC97_ALC650_SOUCE_MASK: u16 = 0x000f;
pub const AC97_ALC650_CHANNEL_MASK: u16 = 0x00f0;
pub const AC97_ALC650_CHANNEL_SHIFT: u32 = 4;
pub const AC97_ALC650_SPSR_MASK: u16 = 0x0f00;
pub const AC97_ALC650_SPSR_SHIFT: u32 = 8;
pub const AC97_ALC650_SPSR_44K: u16 = 0x0000;
pub const AC97_ALC650_SPSR_48K: u16 = 0x0200;
pub const AC97_ALC650_SPSR_32K: u16 = 0x0300;
pub const AC97_ALC650_CLOCK_ACCURACY: u16 = 0x3000;
pub const AC97_ALC650_CLOCK_SHIFT: u32 = 12;
pub const AC97_ALC650_CLOCK_LOCK: u16 = 0x4000;
pub const AC97_ALC650_V: u16 = 0x8000;
pub const AC97_ALC650_SURR_DAC_VOL: u16 = 0x64;
pub const AC97_ALC650_LFE_DAC_VOL: u16 = 0x66;
pub const AC97_ALC650_UNKNOWN1: u16 = 0x68;
pub const AC97_ALC650_MULTICH: u16 = 0x6a;
pub const AC97_ALC650_UNKNOWN2: u16 = 0x6c;
pub const AC97_ALC650_REVISION: u16 = 0x6e;
pub const AC97_ALC650_UNKNOWN3: u16 = 0x70;
pub const AC97_ALC650_UNKNOWN4: u16 = 0x72;
pub const AC97_ALC650_MISC: u16 = 0x74;
pub const AC97_ALC650_GPIO_SETUP: u16 = 0x76;
pub const AC97_ALC650_GPIO_STATUS: u16 = 0x78;
pub const AC97_ALC650_CLOCK: u16 = 0x7a;

pub const AC97_YMF7X3_DIT_CTRL: u16 = 0x66;
pub const AC97_YMF7X3_3D_MODE_SEL: u16 = 0x68;
pub const AC97_CM9738_VENDOR_CTRL: u16 = 0x5a;
pub const AC97_CM9739_MULTI_CHAN: u16 = 0x64;
pub const AC97_CM9739_SPDIF_IN_STATUS: u16 = 0x68;
pub const AC97_CM9739_SPDIF_CTRL: u16 = 0x6c;
pub const AC97_WM97XX_FMIXER_VOL: u16 = 0x72;
pub const AC97_WM9704_RMIXER_VOL: u16 = 0x74;
pub const AC97_WM9704_TEST: u16 = 0x5a;
pub const AC97_WM9704_RPCM_VOL: u16 = 0x70;
pub const AC97_WM9711_OUT3VOL: u16 = 0x16;

pub const AC97_SCAP_AUDIO: u32 = 1 << 0;
pub const AC97_SCAP_MODEM: u32 = 1 << 1;
pub const AC97_SCAP_SURROUND_DAC: u32 = 1 << 2;
pub const AC97_SCAP_CENTER_LFE_DAC: u32 = 1 << 3;
pub const AC97_SCAP_SKIP_AUDIO: u32 = 1 << 4;
pub const AC97_SCAP_SKIP_MODEM: u32 = 1 << 5;
pub const AC97_SCAP_INDEP_SDIN: u32 = 1 << 6;
pub const AC97_SCAP_INV_EAPD: u32 = 1 << 7;
pub const AC97_SCAP_DETECT_BY_VENDOR: u32 = 1 << 8;
pub const AC97_SCAP_NO_SPDIF: u32 = 1 << 9;
pub const AC97_SCAP_EAPD_LED: u32 = 1 << 10;
pub const AC97_SCAP_POWER_SAVE: u32 = 1 << 11;
pub const AC97_HAS_PC_BEEP: u32 = 1 << 0;
pub const AC97_AD_MULTI: u32 = 1 << 1;
pub const AC97_CS_SPDIF: u32 = 1 << 2;
pub const AC97_CX_SPDIF: u32 = 1 << 3;
pub const AC97_STEREO_MUTES: u32 = 1 << 4;
pub const AC97_DOUBLE_RATE: u32 = 1 << 5;
pub const AC97_HAS_NO_MASTER_VOL: u32 = 1 << 6;
pub const AC97_HAS_NO_PCM_VOL: u32 = 1 << 7;
pub const AC97_DEFAULT_POWER_OFF: u32 = 1 << 8;
pub const AC97_MODEM_PATCH: u32 = 1 << 9;
pub const AC97_HAS_NO_REC_GAIN: u32 = 1 << 10;
pub const AC97_HAS_NO_PHONE: u32 = 1 << 11;
pub const AC97_HAS_NO_PC_BEEP: u32 = 1 << 12;
pub const AC97_HAS_NO_VIDEO: u32 = 1 << 13;
pub const AC97_HAS_NO_CD: u32 = 1 << 14;
pub const AC97_HAS_NO_MIC: u32 = 1 << 15;
pub const AC97_HAS_NO_TONE: u32 = 1 << 16;
pub const AC97_HAS_NO_STD_PCM: u32 = 1 << 17;
pub const AC97_HAS_NO_AUX: u32 = 1 << 18;
pub const AC97_HAS_8CH: u32 = 1 << 19;

pub const AC97_RATES_FRONT_DAC: usize = 0;
pub const AC97_RATES_SURR_DAC: usize = 1;
pub const AC97_RATES_LFE_DAC: usize = 2;
pub const AC97_RATES_ADC: usize = 3;
pub const AC97_RATES_MIC_ADC: usize = 4;
pub const AC97_RATES_SPDIF: usize = 5;
pub const AC97_NUM_GPIOS: usize = 16;

#[allow(non_camel_case_types)] pub type c_int = i32;
#[repr(C)] pub struct snd_card { _private: [u8; 0] }
#[repr(C)] pub struct snd_ac97_gpio_priv { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_chmap { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_runtime { _private: [u8; 0] }
#[repr(C)] pub struct snd_info_entry { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct device { pub platform_data: *mut core::ffi::c_void }
#[repr(C)] pub struct delayed_work { _private: [u8; 0] }
#[repr(C)] pub struct bus_type { _private: [u8; 0] }

#[repr(C)] pub struct snd_ac97_build_ops {
    pub build_3d: Option<unsafe extern "C" fn(*mut snd_ac97) -> c_int>,
    pub build_specific: Option<unsafe extern "C" fn(*mut snd_ac97) -> c_int>,
    pub build_spdif: Option<unsafe extern "C" fn(*mut snd_ac97) -> c_int>,
    pub build_post_spdif: Option<unsafe extern "C" fn(*mut snd_ac97) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_ac97)>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_ac97)>,
    pub update_jacks: Option<unsafe extern "C" fn(*mut snd_ac97)>,
}
#[repr(C)] pub struct snd_ac97_bus_ops {
    pub reset: Option<unsafe extern "C" fn(*mut snd_ac97)>,
    pub warm_reset: Option<unsafe extern "C" fn(*mut snd_ac97)>,
    pub write: Option<unsafe extern "C" fn(*mut snd_ac97, u16, u16)>,
    pub read: Option<unsafe extern "C" fn(*mut snd_ac97, u16) -> u16>,
    pub wait: Option<unsafe extern "C" fn(*mut snd_ac97)>,
    pub init: Option<unsafe extern "C" fn(*mut snd_ac97)>,
}
#[repr(C)] pub struct snd_ac97_bus {
    pub ops: *const snd_ac97_bus_ops, pub private_data: *mut core::ffi::c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_ac97_bus)>, pub card: *mut snd_card,
    pub num: u16, pub no_vra: u16, pub dra: u16, pub isdin: u16, pub clock: u32,
    pub bus_lock: spinlock_t, pub used_slots: [[u16; 4]; 2], pub pcms_count: u16,
    pub pcms: *mut ac97_pcm, pub codec: [*mut snd_ac97; 4], pub proc: *mut snd_info_entry,
}
#[repr(C)] pub struct snd_ac97_res_table { pub reg: u16, pub bits: u16 }
#[repr(C)] pub struct snd_ac97_template {
    pub private_data: *mut core::ffi::c_void, pub private_free: Option<unsafe extern "C" fn(*mut snd_ac97)>,
    pub pci: *mut pci_dev, pub num: u16, pub addr: u16, pub scaps: u32,
    pub res_table: *const snd_ac97_res_table,
}
#[repr(C)] pub struct ad18xx_spec {
    pub unchained: [u16; 3], pub chained: [u16; 3], pub id: [u16; 3], pub pcmreg: [u16; 3],
    pub codec_cfg: [u16; 3], pub swap_mic_linein: u8, pub lo_as_master: u8,
}
#[repr(C)] pub union snd_ac97_spec { pub ad18xx: ad18xx_spec, pub dev_flags: u32 }
#[repr(C)] pub struct snd_ac97 {
    pub build_ops: *const snd_ac97_build_ops, pub private_data: *mut core::ffi::c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_ac97)>, pub bus: *mut snd_ac97_bus,
    pub pci: *mut pci_dev, pub proc: *mut snd_info_entry, pub proc_regs: *mut snd_info_entry,
    pub subsystem_vendor: u16, pub subsystem_device: u16, pub reg_mutex: mutex, pub page_mutex: mutex,
    pub num: u16, pub addr: u16, pub id: u32, pub caps: u16, pub ext_id: u16, pub ext_mid: u16,
    pub res_table: *const snd_ac97_res_table, pub scaps: u32, pub flags: u32, pub rates: [u32; 6],
    pub spdif_status: u32, pub regs: [u16; 0x80], pub reg_accessed: [u64; 2], pub spec: snd_ac97_spec,
    pub indep_surround: u8, pub channel_mode: u8, pub power_up: u32, pub power_work: delayed_work,
    pub dev: device, pub gpio_priv: *mut snd_ac97_gpio_priv, pub chmaps: [*mut snd_pcm_chmap; 2],
}

#[inline] pub unsafe fn ac97_is_audio(ac97: *mut snd_ac97) -> c_int { ((*ac97).scaps & AC97_SCAP_AUDIO) as c_int }
#[inline] pub unsafe fn ac97_is_modem(ac97: *mut snd_ac97) -> c_int { ((*ac97).scaps & AC97_SCAP_MODEM) as c_int }
/* AC97_EI_REV_MASK and related definitions are supplied by sound/ac97/regs.h. */
#[inline] pub unsafe fn ac97_is_rev22(ac97: *mut snd_ac97) -> bool { ((*ac97).ext_id & AC97_EI_REV_MASK) >= AC97_EI_REV_22 }
#[inline] pub unsafe fn ac97_can_amap(ac97: *mut snd_ac97) -> bool { ((*ac97).ext_id & AC97_EI_AMAP) != 0 }
#[inline] pub unsafe fn ac97_can_spdif(ac97: *mut snd_ac97) -> bool { ((*ac97).ext_id & AC97_EI_SPDIF) != 0 }

pub const AC97_TUNE_DEFAULT: c_int = -1;
pub const AC97_TUNE_NONE: c_int = 0;
pub const AC97_TUNE_HP_ONLY: c_int = 1;
pub const AC97_TUNE_SWAP_HP: c_int = 2;
pub const AC97_TUNE_SWAP_SURROUND: c_int = 3;
pub const AC97_TUNE_AD_SHARING: c_int = 4;
pub const AC97_TUNE_ALC_JACK: c_int = 5;
pub const AC97_TUNE_INV_EAPD: c_int = 6;
pub const AC97_TUNE_MUTE_LED: c_int = 7;
pub const AC97_TUNE_HP_MUTE_LED: c_int = 8;

#[repr(C)] pub struct ac97_quirk { pub subvendor: u16, pub subdevice: u16, pub mask: u16, pub codec_id: u32, pub name: *const i8, pub r#type: c_int }
#[repr(C)] pub enum ac97_pcm_cfg { AC97_PCM_CFG_FRONT = 2, AC97_PCM_CFG_REAR = 10, AC97_PCM_CFG_LFE = 11, AC97_PCM_CFG_40 = 4, AC97_PCM_CFG_51 = 6, AC97_PCM_CFG_SPDIF = 20 }
#[repr(C)] pub struct ac97_pcm_r { pub slots: u16, pub rslots: [u16; 4], pub rate_table: [u8; 4], pub codec: [*mut snd_ac97; 4] }
#[repr(C)] pub struct ac97_pcm { pub bus: *mut snd_ac97_bus, pub stream: u32, pub exclusive: u32, pub copy_flag: u32, pub spdif: u32, pub aslots: u16, pub cur_dbl: u16, pub rates: u32, pub r: [ac97_pcm_r; 2], pub private_value: usize }

extern "C" {
    pub static ac97_bus_type: bus_type;
    pub fn snd_ac97_bus(card: *mut snd_card, num: c_int, ops: *const snd_ac97_bus_ops, private_data: *mut core::ffi::c_void, rbus: *mut *mut snd_ac97_bus) -> c_int;
    pub fn snd_ac97_mixer(bus: *mut snd_ac97_bus, template: *mut snd_ac97_template, rac97: *mut *mut snd_ac97) -> c_int;
    pub fn snd_ac97_get_short_name(ac97: *mut snd_ac97) -> *const i8;
    pub fn snd_ac97_write(ac97: *mut snd_ac97, reg: u16, value: u16);
    pub fn snd_ac97_read(ac97: *mut snd_ac97, reg: u16) -> u16;
    pub fn snd_ac97_write_cache(ac97: *mut snd_ac97, reg: u16, value: u16);
    pub fn snd_ac97_update(ac97: *mut snd_ac97, reg: u16, value: u16) -> c_int;
    pub fn snd_ac97_update_bits(ac97: *mut snd_ac97, reg: u16, mask: u16, value: u16) -> c_int;
    pub fn snd_ac97_reset(ac97: *mut snd_ac97, try_warm: bool, id: u32, id_mask: u32) -> c_int;
    pub fn snd_ac97_tune_hardware(ac97: *mut snd_ac97, quirk: *const ac97_quirk, override_: *const i8) -> c_int;
    pub fn snd_ac97_set_rate(ac97: *mut snd_ac97, reg: c_int, rate: u32) -> c_int;
    pub fn snd_ac97_pcm_assign(ac97: *mut snd_ac97_bus, pcms_count: u16, pcms: *const ac97_pcm) -> c_int;
    pub fn snd_ac97_pcm_open(pcm: *mut ac97_pcm, rate: u32, cfg: ac97_pcm_cfg, slots: u16) -> c_int;
    pub fn snd_ac97_pcm_close(pcm: *mut ac97_pcm) -> c_int;
    pub fn snd_ac97_pcm_double_rate_rules(runtime: *mut snd_pcm_runtime) -> c_int;
}

pub unsafe fn snd_ac97_dev_add_pdata(ac97: *mut snd_ac97, data: *mut core::ffi::c_void) { (*ac97).dev.platform_data = data; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
