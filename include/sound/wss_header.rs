/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Definitions for CS4231 & InterWave chips & compatible chips. */

// Dependencies supplied by the surrounding kernel translation.

pub const WSS_MODE_NONE: u16 = 0x0000;
pub const WSS_MODE_PLAY: u16 = 0x0001;
pub const WSS_MODE_RECORD: u16 = 0x0002;
pub const WSS_MODE_TIMER: u16 = 0x0004;
pub const WSS_MODE_OPEN: u16 = WSS_MODE_PLAY | WSS_MODE_RECORD | WSS_MODE_TIMER;

pub const WSS_HW_DETECT: u16 = 0x0000;
pub const WSS_HW_DETECT3: u16 = 0x0001;
pub const WSS_HW_TYPE_MASK: u16 = 0xff00;
pub const WSS_HW_CS4231_MASK: u16 = 0x0100;
pub const WSS_HW_CS4231: u16 = 0x0100;
pub const WSS_HW_CS4231A: u16 = 0x0101;
pub const WSS_HW_AD1845: u16 = 0x0102;
pub const WSS_HW_CS4232_MASK: u16 = 0x0200;
pub const WSS_HW_CS4232: u16 = 0x0200;
pub const WSS_HW_CS4232A: u16 = 0x0201;
pub const WSS_HW_CS4236: u16 = 0x0202;
pub const WSS_HW_CS4236B_MASK: u16 = 0x0400;
pub const WSS_HW_CS4235: u16 = 0x0400;
pub const WSS_HW_CS4236B: u16 = 0x0401;
pub const WSS_HW_CS4237B: u16 = 0x0402;
pub const WSS_HW_CS4238B: u16 = 0x0403;
pub const WSS_HW_CS4239: u16 = 0x0404;
pub const WSS_HW_AD1848_MASK: u16 = 0x0800;
pub const WSS_HW_AD1847: u16 = 0x0801;
pub const WSS_HW_AD1848: u16 = 0x0802;
pub const WSS_HW_CS4248: u16 = 0x0803;
pub const WSS_HW_CMI8330: u16 = 0x0804;
pub const WSS_HW_THINKPAD: u16 = 0x0805;
pub const WSS_HW_INTERWAVE: u16 = 0x1000;
pub const WSS_HW_OPL3SA2: u16 = 0x1101;
pub const WSS_HW_OPTI93X: u16 = 0x1102;

pub const WSS_HWSHARE_IRQ: u16 = 1 << 0;
pub const WSS_HWSHARE_DMA1: u16 = 1 << 1;
pub const WSS_HWSHARE_DMA2: u16 = 1 << 2;

pub const AD1848_THINKPAD_CTL_PORT1: u16 = 0x15e8;
pub const AD1848_THINKPAD_CTL_PORT2: u16 = 0x15e9;
pub const AD1848_THINKPAD_CS4248_ENABLE_BIT: u8 = 0x02;

#[repr(C)]
pub struct snd_wss {
    pub port: ::core::ffi::c_ulong,
    pub res_port: *mut resource,
    pub cport: ::core::ffi::c_ulong,
    pub res_cport: *mut resource,
    pub irq: ::core::ffi::c_int,
    pub dma1: ::core::ffi::c_int,
    pub dma2: ::core::ffi::c_int,
    pub version: u16,
    pub mode: u16,
    pub hardware: u16,
    pub hwshare: u16,
    pub single_dma: u16,
    pub ebus_flag: u16,
    pub thinkpad_flag: u16,
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub timer: *mut snd_timer,
    pub image: [u8; 32],
    pub eimage: [u8; 32],
    pub cimage: [u8; 16],
    pub mce_bit: ::core::ffi::c_int,
    pub calibrate_mute: ::core::ffi::c_int,
    pub sw_3d_bit: ::core::ffi::c_int,
    pub p_dma_size: ::core::ffi::c_uint,
    pub c_dma_size: ::core::ffi::c_uint,
    pub reg_lock: spinlock_t,
    pub mce_mutex: mutex,
    pub open_mutex: mutex,
    pub rate_constraint: Option<unsafe extern "C" fn(*mut snd_pcm_runtime) -> ::core::ffi::c_int>,
    pub set_playback_format: Option<unsafe extern "C" fn(*mut snd_wss, *mut snd_pcm_hw_params, u8)>,
    pub set_capture_format: Option<unsafe extern "C" fn(*mut snd_wss, *mut snd_pcm_hw_params, u8)>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_wss, ::core::ffi::c_uint, ::core::ffi::c_int)>,
    #[cfg(CONFIG_PM)]
    pub suspend: Option<unsafe extern "C" fn(*mut snd_wss)>,
    #[cfg(CONFIG_PM)]
    pub resume: Option<unsafe extern "C" fn(*mut snd_wss)>,
    pub dma_private_data: *mut ::core::ffi::c_void,
    pub claim_dma: Option<unsafe extern "C" fn(*mut snd_wss, *mut ::core::ffi::c_void, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub release_dma: Option<unsafe extern "C" fn(*mut snd_wss, *mut ::core::ffi::c_void, ::core::ffi::c_int) -> ::core::ffi::c_int>,
}

pub unsafe extern "C" {
    pub fn snd_wss_out(chip: *mut snd_wss, reg: u8, val: u8);
    pub fn snd_wss_in(chip: *mut snd_wss, reg: u8) -> u8;
    pub fn snd_cs4236_ext_out(chip: *mut snd_wss, reg: u8, val: u8);
    pub fn snd_cs4236_ext_in(chip: *mut snd_wss, reg: u8) -> u8;
    pub fn snd_wss_mce_up(chip: *mut snd_wss);
    pub fn snd_wss_mce_down(chip: *mut snd_wss);
    pub fn snd_wss_overrange(chip: *mut snd_wss);
    pub fn snd_wss_interrupt(irq: ::core::ffi::c_int, dev_id: *mut ::core::ffi::c_void) -> irqreturn_t;
    pub fn snd_wss_chip_id(chip: *mut snd_wss) -> *const ::core::ffi::c_char;
    pub fn snd_wss_create(card: *mut snd_card, port: ::core::ffi::c_ulong, cport: ::core::ffi::c_ulong, irq: ::core::ffi::c_int, dma1: ::core::ffi::c_int, dma2: ::core::ffi::c_int, hardware: u16, hwshare: u16, rchip: *mut *mut snd_wss) -> ::core::ffi::c_int;
    pub fn snd_wss_pcm(chip: *mut snd_wss, device: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn snd_wss_timer(chip: *mut snd_wss, device: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn snd_wss_mixer(chip: *mut snd_wss) -> ::core::ffi::c_int;
    pub fn snd_wss_get_pcm_ops(direction: ::core::ffi::c_int) -> *const snd_pcm_ops;
    pub fn snd_cs4236_create(card: *mut snd_card, port: ::core::ffi::c_ulong, cport: ::core::ffi::c_ulong, irq: ::core::ffi::c_int, dma1: ::core::ffi::c_int, dma2: ::core::ffi::c_int, hardware: u16, hwshare: u16, rchip: *mut *mut snd_wss) -> ::core::ffi::c_int;
    pub fn snd_cs4236_pcm(chip: *mut snd_wss, device: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn snd_cs4236_mixer(chip: *mut snd_wss) -> ::core::ffi::c_int;
    pub fn snd_wss_info_single(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> ::core::ffi::c_int;
    pub fn snd_wss_get_single(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> ::core::ffi::c_int;
    pub fn snd_wss_put_single(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> ::core::ffi::c_int;
    pub fn snd_wss_info_double(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> ::core::ffi::c_int;
    pub fn snd_wss_get_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> ::core::ffi::c_int;
    pub fn snd_wss_put_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> ::core::ffi::c_int;
}

// C bit-fields are represented as individual one-bit values in the source;
// the surrounding translation may replace these with its canonical bitfield type.
#[macro_export]
macro_rules! WSS_SINGLE {
    ($xname:expr, $xindex:expr, $reg:expr, $shift:expr, $mask:expr, $invert:expr) => {
        snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: $xname, index: $xindex,
            info: Some(snd_wss_info_single), get: Some(snd_wss_get_single), put: Some(snd_wss_put_single),
            private_value: ($reg | ($shift << 8) | ($mask << 16) | ($invert << 24)) }
    };
}

#[macro_export]
macro_rules! WSS_DOUBLE {
    ($xname:expr, $xindex:expr, $left_reg:expr, $right_reg:expr, $shift_left:expr, $shift_right:expr, $mask:expr, $invert:expr) => {
        snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: $xname, index: $xindex,
            info: Some(snd_wss_info_double), get: Some(snd_wss_get_double), put: Some(snd_wss_put_double),
            private_value: ($left_reg | ($right_reg << 8) | ($shift_left << 16) |
                ($shift_right << 19) | ($mask << 24) | ($invert << 22)) }
    };
}

#[macro_export]
macro_rules! WSS_SINGLE_TLV {
    ($xname:expr, $xindex:expr, $reg:expr, $shift:expr, $mask:expr, $invert:expr, $xtlv:expr) => {
        snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
            name: $xname, index: $xindex, info: Some(snd_wss_info_single),
            get: Some(snd_wss_get_single), put: Some(snd_wss_put_single),
            private_value: ($reg | ($shift << 8) | ($mask << 16) | ($invert << 24)),
            tlv: snd_ctl_elem_tlv { p: $xtlv } }
    };
}

#[macro_export]
macro_rules! WSS_DOUBLE_TLV {
    ($xname:expr, $xindex:expr, $left_reg:expr, $right_reg:expr, $shift_left:expr, $shift_right:expr, $mask:expr, $invert:expr, $xtlv:expr) => {
        snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
            name: $xname, index: $xindex, info: Some(snd_wss_info_double),
            get: Some(snd_wss_get_double), put: Some(snd_wss_put_double),
            private_value: ($left_reg | ($right_reg << 8) | ($shift_left << 16) |
                ($shift_right << 19) | ($mask << 24) | ($invert << 22)),
            tlv: snd_ctl_elem_tlv { p: $xtlv } }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
