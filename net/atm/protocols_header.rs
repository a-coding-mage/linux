/* SPDX-License-Identifier: GPL-2.0 */
/* net/atm/protocols.h - ATM protocol handler entry points */

/* Written 1995-1997 by Werner Almesberger, EPFL LRC */

/* C header guard: NET_ATM_PROTOCOLS_H */

#[repr(C)]
pub struct atm_vcc {
    _private: [u8; 0],
}

extern "C" {
    pub fn atm_init_aal0(vcc: *mut atm_vcc) -> ::std::os::raw::c_int; /* "raw" AAL0 */
    pub fn atm_init_aal5(vcc: *mut atm_vcc) -> ::std::os::raw::c_int; /* "raw" AAL5 transport */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
