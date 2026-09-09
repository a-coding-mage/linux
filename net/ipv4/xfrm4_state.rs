// SPDX-License-Identifier: GPL-2.0
/*
 * xfrm4_state.c
 *
 * Changes:
 * 	YOSHIFUJI Hideaki @USAGI
 * 		Split up af-specific portion
 *
 */

// Dependency declarations supplied by the surrounding kernel translation.

static mut xfrm4_state_afinfo: xfrm_state_afinfo = xfrm_state_afinfo {
    family: AF_INET,
    proto: IPPROTO_IPIP,
    output: xfrm4_output,
    transport_finish: xfrm4_transport_finish,
    local_error: xfrm4_local_error,
};

#[no_mangle]
pub unsafe extern "C" fn xfrm4_state_init() {
    xfrm_state_register_afinfo(&mut xfrm4_state_afinfo);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
