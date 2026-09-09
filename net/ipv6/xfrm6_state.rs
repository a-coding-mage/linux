// SPDX-License-Identifier: GPL-2.0
/*
 * xfrm6_state.c: based on xfrm4_state.c
 *
 * Authors:
 *	Mitsuru KANDA @USAGI
 *	Kazunori MIYAZAWA @USAGI
 *	Kunihiro Ishiguro <kunihiro@ipinfusion.com>
 *		IPv6 support
 *	YOSHIFUJI Hideaki @USAGI
 *		Split up af-specific portion
 *
 */

// C dependency: <net/xfrm.h>

extern "C" {
    fn xfrm_state_register_afinfo(afinfo: *mut xfrm_state_afinfo) -> i32;
    fn xfrm_state_unregister_afinfo(afinfo: *mut xfrm_state_afinfo);

    fn xfrm6_output();
    fn xfrm6_transport_finish();
    fn xfrm6_local_error();
}

#[repr(C)]
pub struct xfrm_state_afinfo {
    pub family: i32,
    pub proto: i32,
    pub output: Option<unsafe extern "C" fn()>,
    pub transport_finish: Option<unsafe extern "C" fn()>,
    pub local_error: Option<unsafe extern "C" fn()>,
}

static mut xfrm6_state_afinfo: xfrm_state_afinfo = xfrm_state_afinfo {
    family: AF_INET6,
    proto: IPPROTO_IPV6,
    output: Some(xfrm6_output),
    transport_finish: Some(xfrm6_transport_finish),
    local_error: Some(xfrm6_local_error),
};

// Build-time kernel constants supplied by the surrounding bindings.
extern "C" {
    static AF_INET6: i32;
    static IPPROTO_IPV6: i32;
}

pub unsafe extern "C" fn xfrm6_state_init() -> i32 {
    xfrm_state_register_afinfo(&mut xfrm6_state_afinfo)
}

pub unsafe extern "C" fn xfrm6_state_fini() {
    xfrm_state_unregister_afinfo(&mut xfrm6_state_afinfo);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
