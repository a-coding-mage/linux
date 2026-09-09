/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Witness Service client for CIFS
 *
 * Copyright (c) 2020 Samuel Cabrero <scabrero@suse.de>
 */

// Dependency supplied by cifsglob.h in the C source is intentionally left external.

#[cfg(feature = "CONFIG_CIFS_SWN_UPCALL")]
extern "C" {
    pub fn cifs_swn_register(tcon: *mut cifs_tcon) -> ::core::ffi::c_int;

    pub fn cifs_swn_unregister(tcon: *mut cifs_tcon) -> ::core::ffi::c_int;

    pub fn cifs_swn_notify(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> ::core::ffi::c_int;

    pub fn cifs_swn_dump(m: *mut seq_file);

    pub fn cifs_swn_check();
}

#[cfg(feature = "CONFIG_CIFS_SWN_UPCALL")]
#[inline]
pub unsafe fn cifs_swn_set_server_dstaddr(server: *mut TCP_Server_Info) -> bool {
    if (*server).use_swn_dstaddr {
        (*server).dstaddr = (*server).swn_dstaddr;
        return true;
    }
    false
}

#[cfg(feature = "CONFIG_CIFS_SWN_UPCALL")]
#[inline]
pub unsafe fn cifs_swn_reset_server_dstaddr(server: *mut TCP_Server_Info) {
    (*server).use_swn_dstaddr = false;
}

#[cfg(not(feature = "CONFIG_CIFS_SWN_UPCALL"))]
#[inline]
pub unsafe fn cifs_swn_register(_tcon: *mut cifs_tcon) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_CIFS_SWN_UPCALL"))]
#[inline]
pub unsafe fn cifs_swn_unregister(_tcon: *mut cifs_tcon) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_CIFS_SWN_UPCALL"))]
#[inline]
pub unsafe fn cifs_swn_notify(
    _s: *mut sk_buff,
    _i: *mut genl_info,
) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_CIFS_SWN_UPCALL"))]
#[inline]
pub unsafe fn cifs_swn_dump(_m: *mut seq_file) {}

#[cfg(not(feature = "CONFIG_CIFS_SWN_UPCALL"))]
#[inline]
pub unsafe fn cifs_swn_check() {}

#[cfg(not(feature = "CONFIG_CIFS_SWN_UPCALL"))]
#[inline]
pub unsafe fn cifs_swn_set_server_dstaddr(_server: *mut TCP_Server_Info) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_CIFS_SWN_UPCALL"))]
#[inline]
pub unsafe fn cifs_swn_reset_server_dstaddr(_server: *mut TCP_Server_Info) {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
