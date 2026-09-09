/* SPDX-License-Identifier: GPL-2.0 */
/*
 * The contents of this file are private to DMA engine drivers, and is not
 * part of the API to be used by DMA engine users.
 *
 * Translated from dmaengine.h. Types and symbols supplied by the Linux DMA
 * engine headers are intentionally left as external dependencies.
 */

pub unsafe fn dma_cookie_init(chan: *mut dma_chan) {
    (*chan).cookie = DMA_MIN_COOKIE;
    (*chan).completed_cookie = DMA_MIN_COOKIE;
}

pub unsafe fn dma_cookie_assign(tx: *mut dma_async_tx_descriptor) -> dma_cookie_t {
    let chan = (*tx).chan;
    let mut cookie = (*chan).cookie + 1;
    if cookie < DMA_MIN_COOKIE {
        cookie = DMA_MIN_COOKIE;
    }
    (*tx).cookie = cookie;
    (*chan).cookie = cookie;
    cookie
}

pub unsafe fn dma_cookie_complete(tx: *mut dma_async_tx_descriptor) {
    // Equivalent of BUG_ON(tx->cookie < DMA_MIN_COOKIE).
    if (*tx).cookie < DMA_MIN_COOKIE {
        panic!("BUG_ON: DMA cookie is below DMA_MIN_COOKIE");
    }
    (*tx).chan.as_mut().unwrap().completed_cookie = (*tx).cookie;
    (*tx).cookie = 0;
}

pub unsafe fn dma_cookie_status(
    chan: *mut dma_chan,
    cookie: dma_cookie_t,
    state: *mut dma_tx_state,
) -> dma_status {
    let used = (*chan).cookie;
    let complete = (*chan).completed_cookie;
    barrier();
    if !state.is_null() {
        (*state).last = complete;
        (*state).used = used;
        (*state).residue = 0;
        (*state).in_flight_bytes = 0;
    }
    dma_async_is_complete(cookie, complete, used)
}

pub unsafe fn dma_set_residue(state: *mut dma_tx_state, residue: u32) {
    if !state.is_null() {
        (*state).residue = residue;
    }
}

pub unsafe fn dma_set_in_flight_bytes(state: *mut dma_tx_state, in_flight_bytes: u32) {
    if !state.is_null() {
        (*state).in_flight_bytes = in_flight_bytes;
    }
}

#[repr(C)]
pub struct dmaengine_desc_callback {
    pub callback: dma_async_tx_callback,
    pub callback_result: dma_async_tx_callback_result,
    pub callback_param: *mut core::ffi::c_void,
}

pub unsafe fn dmaengine_desc_get_callback(
    tx: *mut dma_async_tx_descriptor,
    cb: *mut dmaengine_desc_callback,
) {
    (*cb).callback = (*tx).callback;
    (*cb).callback_result = (*tx).callback_result;
    (*cb).callback_param = (*tx).callback_param;
}

pub unsafe fn dmaengine_desc_callback_invoke(
    cb: *mut dmaengine_desc_callback,
    mut result: *const dmaengine_result,
) {
    let dummy_result = dmaengine_result {
        result: DMA_TRANS_NOERROR,
        residue: 0,
    };

    if let Some(callback_result) = (*cb).callback_result {
        if result.is_null() {
            result = &dummy_result;
        }
        callback_result((*cb).callback_param, result);
    } else if let Some(callback) = (*cb).callback {
        callback((*cb).callback_param);
    }
}

pub unsafe fn dmaengine_desc_get_callback_invoke(
    tx: *mut dma_async_tx_descriptor,
    result: *const dmaengine_result,
) {
    let mut cb = core::mem::MaybeUninit::<dmaengine_desc_callback>::uninit();
    dmaengine_desc_get_callback(tx, cb.as_mut_ptr());
    dmaengine_desc_callback_invoke(cb.as_mut_ptr(), result);
}

pub unsafe fn dmaengine_desc_callback_valid(cb: *mut dmaengine_desc_callback) -> bool {
    (*cb).callback.is_some() || (*cb).callback_result.is_some()
}

extern "C" {
    pub fn dma_get_slave_channel(chan: *mut dma_chan) -> *mut dma_chan;
    pub fn dma_get_any_slave_channel(device: *mut dma_device) -> *mut dma_chan;
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
pub unsafe fn dmaengine_get_debugfs_root(dma_dev: *mut dma_device) -> *mut dentry {
    (*dma_dev).dbg_dev_root
}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn dmaengine_get_debugfs_root(_dma_dev: *mut dma_device) -> *mut dentry {
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
