/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Marek Lindner, Linus Lüssing
 */

/* C header guard: _NET_BATMAN_ADV_BAT_V_H_ */
/* Dependency: "main.h" */

/* CONFIG_BATMAN_ADV_BATMAN_V */
#[cfg(feature = "CONFIG_BATMAN_ADV_BATMAN_V")]
extern "C" {
    pub fn batadv_v_init() -> ::core::ffi::c_int;
    pub fn batadv_v_deinit();
    pub fn batadv_v_hardif_init(hardif: *mut batadv_hard_iface);
    pub fn batadv_v_mesh_init(bat_priv: *mut batadv_priv) -> ::core::ffi::c_int;
    pub fn batadv_v_mesh_free(bat_priv: *mut batadv_priv);
}

/* CONFIG_BATMAN_ADV_BATMAN_V disabled: static inline fallback definitions. */
#[cfg(not(feature = "CONFIG_BATMAN_ADV_BATMAN_V"))]
#[inline]
pub fn batadv_v_init() -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_BATMAN_V"))]
#[inline]
pub fn batadv_v_deinit() {}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_BATMAN_V"))]
#[inline]
pub fn batadv_v_hardif_init(_hardif: *mut batadv_hard_iface) {}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_BATMAN_V"))]
#[inline]
pub fn batadv_v_mesh_init(_bat_priv: *mut batadv_priv) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_BATMAN_V"))]
#[inline]
pub fn batadv_v_mesh_free(_bat_priv: *mut batadv_priv) {}

/* #endif: CONFIG_BATMAN_ADV_BATMAN_V */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
