/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_int;

pub const ADM_PATH_PLAYBACK: c_int = 0x1;
pub const ADM_PATH_LIVE_REC: c_int = 0x2;
pub const MAX_COPPS_PER_PORT: usize = 8;
pub const NULL_COPP_TOPOLOGY: c_int = 0x00010312;

/* multiple copp per stream. */
#[repr(C)]
pub struct route_payload {
    pub num_copps: c_int,
    pub session_id: c_int,
    pub copp_idx: [c_int; MAX_COPPS_PER_PORT],
    pub port_id: [c_int; MAX_COPPS_PER_PORT],
}

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct q6copp {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn q6adm_open(
        dev: *mut device,
        port_id: c_int,
        path: c_int,
        rate: c_int,
        channel_mode: c_int,
        topology: c_int,
        perf_mode: c_int,
        bit_width: u16,
        app_type: c_int,
        acdb_id: c_int,
    ) -> *mut q6copp;

    pub fn q6adm_close(dev: *mut device, copp: *mut q6copp) -> c_int;
    pub fn q6adm_get_copp_id(copp: *mut q6copp) -> c_int;
    pub fn q6adm_matrix_map(
        dev: *mut device,
        path: c_int,
        payload_map: route_payload,
        perf_mode: c_int,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
