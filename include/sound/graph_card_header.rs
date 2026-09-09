/* SPDX-License-Identifier: GPL-2.0
 *
 * ASoC audio graph card support
 *
 */

use core::ffi::c_int;

/* Types supplied by <sound/simple_card_utils.h>. */
#[repr(C)]
pub struct simple_util_priv {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct link_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

pub type GRAPH2_CUSTOM = unsafe extern "C" fn(
    priv_: *mut simple_util_priv,
    lnk: *mut device_node,
    li: *mut link_info,
) -> c_int;

#[repr(C)]
pub struct graph2_custom_hooks {
    pub hook_pre: Option<unsafe extern "C" fn(priv_: *mut simple_util_priv) -> c_int>,
    pub hook_post: Option<unsafe extern "C" fn(priv_: *mut simple_util_priv) -> c_int>,
    pub custom_normal: Option<GRAPH2_CUSTOM>,
    pub custom_dpcm: Option<GRAPH2_CUSTOM>,
    pub custom_c2c: Option<GRAPH2_CUSTOM>,
}

unsafe extern "C" {
    pub fn audio_graph_parse_of(priv_: *mut simple_util_priv, dev: *mut device) -> c_int;
    pub fn audio_graph2_parse_of(
        priv_: *mut simple_util_priv,
        dev: *mut device,
        hooks: *mut graph2_custom_hooks,
    ) -> c_int;

    pub fn audio_graph2_link_normal(
        priv_: *mut simple_util_priv,
        lnk: *mut device_node,
        li: *mut link_info,
    ) -> c_int;
    pub fn audio_graph2_link_dpcm(
        priv_: *mut simple_util_priv,
        lnk: *mut device_node,
        li: *mut link_info,
    ) -> c_int;
    pub fn audio_graph2_link_c2c(
        priv_: *mut simple_util_priv,
        lnk: *mut device_node,
        li: *mut link_info,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
