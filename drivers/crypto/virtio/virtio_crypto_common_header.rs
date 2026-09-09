/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Common header for Virtio crypto device.
 *
 * Copyright 2016 HUAWEI TECHNOLOGIES CO., LTD.
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct data_queue {
    // Virtqueue associated with this send _queue
    pub vq: *mut virtqueue,
    // To protect the vq operations for the dataq
    pub lock: spinlock_t,
    // Name of the tx queue: dataq.$index
    pub name: [core::ffi::c_char; 32],
    pub engine: *mut crypto_engine,
    pub done_work: work_struct,
}

#[repr(C)]
pub struct virtio_crypto {
    pub vdev: *mut virtio_device,
    pub ctrl_vq: *mut virtqueue,
    pub data_vq: *mut data_queue,
    // Work struct for config space updates
    pub config_work: work_struct,
    // To protect the vq operations for the controlq
    pub ctrl_lock: spinlock_t,
    // Maximum of data queues supported by the device
    pub max_data_queues: u32,
    // Number of queue currently used by the driver
    pub curr_queue: u32,
    /*
     * Specifies the services mask which the device support,
     * see VIRTIO_CRYPTO_SERVICE_*
     */
    pub crypto_services: u32,
    // Detailed algorithms mask
    pub cipher_algo_l: u32,
    pub cipher_algo_h: u32,
    pub hash_algo: u32,
    pub mac_algo_l: u32,
    pub mac_algo_h: u32,
    pub aead_algo: u32,
    pub akcipher_algo: u32,
    // Maximum length of cipher key
    pub max_cipher_key_len: u32,
    // Maximum length of authenticated key
    pub max_auth_key_len: u32,
    // Maximum size of per request
    pub max_size: u64,
    pub status: core::ffi::c_ulong,
    pub ref_count: atomic_t,
    pub list: list_head,
    pub owner: *mut module,
    pub dev_id: u8,
    // Does the affinity hint is set for virtqueues?
    pub affinity_hint_set: bool,
}

#[repr(C)]
pub struct virtio_crypto_sym_session_info {
    // Backend session id, which come from the host side
    pub session_id: u64,
}

/*
 * Note: there are padding fields in request, clear them to zero before
 *       sending to host to avoid to divulge any information.
 * Ex, virtio_crypto_ctrl_request::ctrl::u::destroy_session::padding[48]
 */
#[repr(C)]
pub struct virtio_crypto_ctrl_request {
    pub ctrl: virtio_crypto_op_ctrl_req,
    pub input: virtio_crypto_session_input,
    pub ctrl_status: virtio_crypto_inhdr,
    pub compl: completion,
}

pub struct virtio_crypto_request;

pub type virtio_crypto_data_callback = unsafe extern "C" fn(
    vc_req: *mut virtio_crypto_request,
    len: core::ffi::c_int,
);

#[repr(C)]
pub struct virtio_crypto_request {
    pub status: u8,
    pub req_data: *mut virtio_crypto_op_data_req,
    pub sgs: *mut *mut scatterlist,
    pub dataq: *mut data_queue,
    pub alg_cb: virtio_crypto_data_callback,
}

extern "C" {
    pub fn virtcrypto_devmgr_add_dev(vcrypto_dev: *mut virtio_crypto) -> core::ffi::c_int;
    pub fn virtcrypto_devmgr_get_head() -> *mut list_head;
    pub fn virtcrypto_devmgr_rm_dev(vcrypto_dev: *mut virtio_crypto);
    pub fn virtcrypto_dev_get(vcrypto_dev: *mut virtio_crypto) -> core::ffi::c_int;
    pub fn virtcrypto_dev_put(vcrypto_dev: *mut virtio_crypto);
    pub fn virtcrypto_dev_started(vcrypto_dev: *mut virtio_crypto) -> core::ffi::c_int;
    pub fn virtcrypto_algo_is_supported(
        vcrypto_dev: *mut virtio_crypto,
        service: u32,
        algo: u32,
    ) -> bool;
    pub fn virtcrypto_get_dev_node(node: core::ffi::c_int, service: u32, algo: u32)
        -> *mut virtio_crypto;
    pub fn virtcrypto_dev_start(vcrypto: *mut virtio_crypto) -> core::ffi::c_int;
    pub fn virtcrypto_dev_stop(vcrypto: *mut virtio_crypto);
    pub fn virtio_crypto_skcipher_crypt_req(
        engine: *mut crypto_engine,
        vreq: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    pub fn virtcrypto_clear_request(vc_req: *mut virtio_crypto_request);
    pub fn virtio_crypto_skcipher_algs_register(vcrypto: *mut virtio_crypto) -> core::ffi::c_int;
    pub fn virtio_crypto_skcipher_algs_unregister(vcrypto: *mut virtio_crypto);
    pub fn virtio_crypto_akcipher_algs_register(vcrypto: *mut virtio_crypto) -> core::ffi::c_int;
    pub fn virtio_crypto_akcipher_algs_unregister(vcrypto: *mut virtio_crypto);
    pub fn virtio_crypto_ctrl_vq_request(
        vcrypto: *mut virtio_crypto,
        sgs: *mut *mut scatterlist,
        out_sgs: core::ffi::c_uint,
        in_sgs: core::ffi::c_uint,
        vc_ctrl_req: *mut virtio_crypto_ctrl_request,
    ) -> core::ffi::c_int;
}

#[inline]
pub unsafe fn virtio_crypto_get_current_node() -> core::ffi::c_int {
    let cpu: core::ffi::c_int = get_cpu();
    let node: core::ffi::c_int = cpu_to_node(cpu);
    put_cpu();
    node
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
