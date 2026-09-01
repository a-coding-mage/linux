/* SPDX-License-Identifier: GPL-2.0 */

/* Rust translation of declarations from sound/soc.h dependencies. */
#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum wcd_clsh_event {
    WCD_CLSH_EVENT_PRE_DAC = 1,
    WCD_CLSH_EVENT_POST_PA = 2,
}

/*
 * Basic states for Class H state machine.
 * represented as a bit mask within a u8 data type
 * bit 0: EAR mode
 * bit 1: HPH Left mode
 * bit 2: HPH Right mode
 * bit 3: Lineout mode
 */
pub const WCD_CLSH_STATE_IDLE: u32 = 0;
pub const WCD_CLSH_STATE_EAR: u32 = 1u32 << 0;
pub const WCD_CLSH_STATE_HPHL: u32 = 1u32 << 1;
pub const WCD_CLSH_STATE_HPHR: u32 = 1u32 << 2;
pub const WCD_CLSH_STATE_LO: u32 = 1u32 << 3;
pub const WCD_CLSH_STATE_AUX: u32 = 1u32 << 4;
pub const WCD_CLSH_STATE_MAX: u32 = 4;
pub const WCD_CLSH_V3_STATE_MAX: u32 = 5;
pub const NUM_CLSH_STATES_V2: u32 = 1u32 << WCD_CLSH_STATE_MAX;
pub const NUM_CLSH_STATES_V3: u32 = 1u32 << WCD_CLSH_V3_STATE_MAX;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum wcd_clsh_mode {
    CLS_H_NORMAL = 0, /* Class-H Default */
    CLS_H_HIFI = 1, /* Class-H HiFi */
    CLS_H_LP = 2, /* Class-H Low Power */
    CLS_AB = 3, /* Class-AB */
    CLS_H_LOHIFI = 4, /* LoHIFI */
    CLS_H_ULP = 5, /* Ultra Low power */
    CLS_AB_HIFI = 6, /* Class-AB */
    CLS_AB_LP = 7, /* Class-AB Low Power */
    CLS_AB_LOHIFI = 8, /* Class-AB Low HIFI */
    CLS_NONE = 9, /* None of the above modes */
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum wcd_codec_version {
    WCD9335 = 0,
    WCD934X = 1,
    /* New CLSH after this */
    WCD937X = 2,
    WCD938X = 3,
    WCD939X = 4,
}

#[repr(C)]
pub struct wcd_clsh_ctrl {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn wcd_clsh_ctrl_alloc(
        comp: *mut snd_soc_component,
        version: ::std::os::raw::c_int,
    ) -> *mut wcd_clsh_ctrl;
    pub fn wcd_clsh_ctrl_free(ctrl: *mut wcd_clsh_ctrl);
    pub fn wcd_clsh_ctrl_get_state(ctrl: *mut wcd_clsh_ctrl) -> ::std::os::raw::c_int;
    pub fn wcd_clsh_ctrl_set_state(
        ctrl: *mut wcd_clsh_ctrl,
        clsh_event: wcd_clsh_event,
        nstate: ::std::os::raw::c_int,
        mode: wcd_clsh_mode,
    ) -> ::std::os::raw::c_int;
    pub fn wcd_clsh_set_hph_mode(ctrl: *mut wcd_clsh_ctrl, mode: ::std::os::raw::c_int);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
