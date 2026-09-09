/* SPDX-License-Identifier: GPL-2.0 */
/* Portions Copyright (C) 2022 - 2025 Intel Corporation */
/* C header dependencies: linux/once_lite.h, net/cfg80211.h */

/* These configuration values are supplied by the build configuration. */
pub const MAC80211_OCB_DEBUG: bool = cfg!(CONFIG_MAC80211_OCB_DEBUG);
pub const MAC80211_IBSS_DEBUG: bool = cfg!(CONFIG_MAC80211_IBSS_DEBUG);
pub const MAC80211_PS_DEBUG: bool = cfg!(CONFIG_MAC80211_PS_DEBUG);
pub const MAC80211_HT_DEBUG: bool = cfg!(CONFIG_MAC80211_HT_DEBUG);
pub const MAC80211_MPL_DEBUG: bool = cfg!(CONFIG_MAC80211_MPL_DEBUG);
pub const MAC80211_MPATH_DEBUG: bool = cfg!(CONFIG_MAC80211_MPATH_DEBUG);
pub const MAC80211_MHWMP_DEBUG: bool = cfg!(CONFIG_MAC80211_MHWMP_DEBUG);
pub const MAC80211_MESH_SYNC_DEBUG: bool = cfg!(CONFIG_MAC80211_MESH_SYNC_DEBUG);
pub const MAC80211_MESH_CSA_DEBUG: bool = cfg!(CONFIG_MAC80211_MESH_CSA_DEBUG);
pub const MAC80211_MESH_PS_DEBUG: bool = cfg!(CONFIG_MAC80211_MESH_PS_DEBUG);
pub const MAC80211_TDLS_DEBUG: bool = cfg!(CONFIG_MAC80211_TDLS_DEBUG);
pub const MAC80211_STA_DEBUG: bool = cfg!(CONFIG_MAC80211_STA_DEBUG);
pub const MAC80211_MLME_DEBUG: bool = cfg!(CONFIG_MAC80211_MLME_DEBUG);

#[cfg(CONFIG_MAC80211_MESSAGE_TRACING)]
extern "C" {
    pub fn __sdata_info(fmt: *const core::ffi::c_char, ...);
    pub fn __sdata_dbg(print: bool, fmt: *const core::ffi::c_char, ...);
    pub fn __sdata_err(fmt: *const core::ffi::c_char, ...);
    pub fn __wiphy_dbg(wiphy: *mut wiphy, print: bool, fmt: *const core::ffi::c_char, ...);
}

#[cfg(CONFIG_MAC80211_MESSAGE_TRACING)]
#[macro_export]
macro_rules! _sdata_info {
    ($sdata:expr, $fmt:expr $(, $arg:expr)*) => {
        unsafe { $crate::__sdata_info(concat!("%s: ", $fmt, "\0").as_ptr() as _, $sdata.name.as_ptr() $(, $arg)*) }
    };
}
#[cfg(not(CONFIG_MAC80211_MESSAGE_TRACING))]
#[macro_export]
macro_rules! _sdata_info { ($sdata:expr, $fmt:expr $(, $arg:expr)*) => { unsafe { pr_info!(concat!("%s: ", $fmt), $sdata.name $(, $arg)*) } }; }

#[macro_export]
macro_rules! sdata_info { ($sdata:expr, $fmt:expr $(, $arg:expr)*) => { $crate::_sdata_info!($sdata, $fmt $(, $arg)*) }; }
#[macro_export]
macro_rules! sdata_err { ($sdata:expr, $fmt:expr $(, $arg:expr)*) => { $crate::_sdata_err!($sdata, $fmt $(, $arg)*) }; }
#[macro_export]
macro_rules! sdata_dbg { ($sdata:expr, $fmt:expr $(, $arg:expr)*) => { $crate::_sdata_dbg!(true, $sdata, $fmt $(, $arg)*) }; }

/* The following macros preserve the original C logging interface and argument evaluation. */
#[macro_export]
macro_rules! _sdata_dbg { ($print:expr, $sdata:expr, $fmt:expr $(, $arg:expr)*) => { if $print { pr_debug!(concat!("%s: ", $fmt), $sdata.name $(, $arg)*); } }; }
#[macro_export]
macro_rules! _sdata_err { ($sdata:expr, $fmt:expr $(, $arg:expr)*) => { pr_err!(concat!("%s: ", $fmt), $sdata.name $(, $arg)*); }; }
#[macro_export]
macro_rules! _wiphy_dbg { ($print:expr, $wiphy:expr, $fmt:expr $(, $arg:expr)*) => { if $print { wiphy_dbg!($wiphy, $fmt $(, $arg)*); } }; }

#[macro_export]
macro_rules! link_info { ($link:expr, $fmt:expr $(, $arg:expr)*) => { if ieee80211_vif_is_mld!(&$link.sdata.vif) { $crate::_sdata_info!($link.sdata, concat!("[link %d] ", $fmt), $link.link_id $(, $arg)*); } else { $crate::_sdata_info!($link.sdata, $fmt $(, $arg)*); } }; }
#[macro_export]
macro_rules! link_err { ($link:expr, $fmt:expr $(, $arg:expr)*) => { if ieee80211_vif_is_mld!(&$link.sdata.vif) { $crate::_sdata_err!($link.sdata, concat!("[link %d] ", $fmt), $link.link_id $(, $arg)*); } else { $crate::_sdata_err!($link.sdata, $fmt $(, $arg)*); } }; }
#[macro_export]
macro_rules! link_err_once { ($link:expr, $fmt:expr $(, $arg:expr)*) => { do_once_lite!(link_err, $link, $fmt $(, $arg)*); }; }
#[macro_export]
macro_rules! link_id_info { ($sdata:expr, $link_id:expr, $fmt:expr $(, $arg:expr)*) => { if ieee80211_vif_is_mld!(&$sdata.vif) { $crate::_sdata_info!($sdata, concat!("[link %d] ", $fmt), $link_id $(, $arg)*); } else { $crate::_sdata_info!($sdata, $fmt $(, $arg)*); } }; }
#[macro_export]
macro_rules! link_dbg { ($link:expr, $fmt:expr $(, $arg:expr)*) => { $crate::_link_id_dbg!(true, $link.sdata, $link.link_id, $fmt $(, $arg)*); }; }
#[macro_export]
macro_rules! _link_id_dbg { ($print:expr, $sdata:expr, $link_id:expr, $fmt:expr $(, $arg:expr)*) => { if ieee80211_vif_is_mld!(&$sdata.vif) { $crate::_sdata_dbg!($print, $sdata, concat!("[link %d] ", $fmt), $link_id $(, $arg)*); } else { $crate::_sdata_dbg!($print, $sdata, $fmt $(, $arg)*); } }; }

#[macro_export]
macro_rules! _debug_wrapper { ($name:ident, $flag:expr) => { #[macro_export] macro_rules! $name { ($sdata:expr, $fmt:expr $(, $arg:expr)*) => { $crate::_sdata_dbg!($flag, $sdata, $fmt $(, $arg)*); } }; }; }
_debug_wrapper!(ht_dbg, MAC80211_HT_DEBUG);
_debug_wrapper!(ocb_dbg, MAC80211_OCB_DEBUG);
_debug_wrapper!(ibss_dbg, MAC80211_IBSS_DEBUG);
_debug_wrapper!(ps_dbg, MAC80211_PS_DEBUG);
#[macro_export]
macro_rules! ps_dbg_hw { ($hw:expr, $fmt:expr $(, $arg:expr)*) => { $crate::_wiphy_dbg!(MAC80211_PS_DEBUG, $hw.wiphy, $fmt $(, $arg)*); }; }
_debug_wrapper!(mpl_dbg, MAC80211_MPL_DEBUG);
_debug_wrapper!(mpath_dbg, MAC80211_MPATH_DEBUG);
_debug_wrapper!(mhwmp_dbg, MAC80211_MHWMP_DEBUG);
_debug_wrapper!(msync_dbg, MAC80211_MESH_SYNC_DEBUG);
_debug_wrapper!(mcsa_dbg, MAC80211_MESH_CSA_DEBUG);
_debug_wrapper!(mps_dbg, MAC80211_MESH_PS_DEBUG);
_debug_wrapper!(tdls_dbg, MAC80211_TDLS_DEBUG);
_debug_wrapper!(sta_dbg, MAC80211_STA_DEBUG);
_debug_wrapper!(mlme_dbg, MAC80211_MLME_DEBUG);

#[macro_export]
macro_rules! mlme_link_id_dbg { ($sdata:expr, $link_id:expr, $fmt:expr $(, $arg:expr)*) => { $crate::_link_id_dbg!(MAC80211_MLME_DEBUG, $sdata, $link_id, $fmt $(, $arg)*); }; }
#[macro_export]
macro_rules! ht_dbg_ratelimited { ($sdata:expr, $fmt:expr $(, $arg:expr)*) => { $crate::_sdata_dbg!(MAC80211_HT_DEBUG && net_ratelimit!(), $sdata, $fmt $(, $arg)*); }; }
#[macro_export]
macro_rules! ps_dbg_ratelimited { ($sdata:expr, $fmt:expr $(, $arg:expr)*) => { $crate::_sdata_dbg!(MAC80211_PS_DEBUG && net_ratelimit!(), $sdata, $fmt $(, $arg)*); }; }
#[macro_export]
macro_rules! mlme_dbg_ratelimited { ($sdata:expr, $fmt:expr $(, $arg:expr)*) => { $crate::_sdata_dbg!(MAC80211_MLME_DEBUG && net_ratelimit!(), $sdata, $fmt $(, $arg)*); }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
