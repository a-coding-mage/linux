/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Copyright (C) 2008-2009 ST-Ericsson AB
 *
 * Author: Rickard Andersson <rickard.andersson@stericsson.com>
 * Author: Linus Walleij <linus.walleij@stericsson.com>
 */

/* CONFIG_HAVE_TCM controls whether TCM support is available. */

#[cfg(feature = "config_have_tcm")]
extern "C" {
    /* __tcmdata: __section(".tcm.data") */
    /* __tcmconst: __section(".tcm.rodata") */
    /* __tcmfunc: __attribute__((long_call)) __section(".tcm.text") noinline */
    /* __tcmlocalfunc: __section(".tcm.text") */

    pub fn tcm_alloc(len: usize) -> *mut core::ffi::c_void;
    pub fn tcm_free(addr: *mut core::ffi::c_void, len: usize);
    pub fn tcm_dtcm_present() -> bool;
    pub fn tcm_itcm_present() -> bool;

    /* __init */
    pub fn tcm_init();
}

/* No TCM support, just a blank inline to be optimized out. */
#[cfg(not(feature = "config_have_tcm"))]
#[inline]
pub fn tcm_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
