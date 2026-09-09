/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* atmppp.h - RFC2364 PPPoATM */

/* Written 2000 by Mitchell Blank Jr */

/* The C header includes <linux/atm.h>; atm_backend_t is supplied by that dependency. */

pub const PPPOATM_ENCAPS_AUTODETECT: i32 = 0;
pub const PPPOATM_ENCAPS_VC: i32 = 1;
pub const PPPOATM_ENCAPS_LLC: i32 = 2;

/*
 * This is for the ATM_SETBACKEND call - these are like socket families:
 * the first element of the structure is the backend number and the rest
 * is per-backend specific
 */
#[repr(C)]
pub struct atm_backend_ppp {
    pub backend_num: atm_backend_t, /* ATM_BACKEND_PPP */
    pub encaps: i32,                /* PPPOATM_ENCAPS_* */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
