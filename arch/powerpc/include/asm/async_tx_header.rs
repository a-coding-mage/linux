/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2008-2009 DENX Software Engineering.
 *
 * Author: Yuri Tikhonov <yur@emcraft.com>
 */

/*
 * The original header selects this interface when CONFIG_440SPe or
 * CONFIG_440SP is enabled.  These cfg flags preserve that build-time intent.
 */
#[cfg(any(feature = "CONFIG_440SPe", feature = "CONFIG_440SP"))]
unsafe extern "C" {
    pub fn ppc440spe_async_tx_find_best_channel(
        cap: dma_transaction_type,
        dst_lst: *mut *mut page,
        dst_cnt: core::ffi::c_int,
        src_lst: *mut *mut page,
        src_cnt: core::ffi::c_int,
        src_sz: usize,
    ) -> *mut dma_chan;
}

#[cfg(any(feature = "CONFIG_440SPe", feature = "CONFIG_440SP"))]
macro_rules! async_tx_find_channel {
    ($dep:expr, $cap:expr, $dst_lst:expr, $dst_cnt:expr, $src_lst:expr,
     $src_cnt:expr, $src_sz:expr) => {
        unsafe {
            ppc440spe_async_tx_find_best_channel(
                $cap, $dst_lst, $dst_cnt, $src_lst, $src_cnt, $src_sz,
            )
        }
    };
}

#[cfg(not(any(feature = "CONFIG_440SPe", feature = "CONFIG_440SP")))]
macro_rules! async_tx_find_channel {
    ($dep:expr, $type:expr, $dst:expr, $dst_count:expr, $src:expr,
     $src_count:expr, $len:expr) => {
        unsafe { __async_tx_find_channel($dep, $type) }
    };
}

#[cfg(not(any(feature = "CONFIG_440SPe", feature = "CONFIG_440SP")))]
unsafe extern "C" {
    pub fn __async_tx_find_channel(
        submit: *mut async_submit_ctl,
        tx_type: dma_transaction_type,
    ) -> *mut dma_chan;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
