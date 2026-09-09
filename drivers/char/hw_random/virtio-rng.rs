// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Randomness driver for virtio
 *  Copyright (C) 2007, 2008 Rusty Russell IBM Corporation
 */

// Dependencies supplied by the surrounding kernel translation unit are intentionally external.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut rng_index_ida: Ida;
    fn virtqueue_get_buf(vq: *mut Virtqueue, len: *mut u32) -> *mut c_void;
    fn smp_store_release(ptr: *mut u32, value: u32);
    fn complete(completion: *mut Completion);
    fn reinit_completion(completion: *mut Completion);
    fn sg_init_one(sg: *mut Scatterlist, buf: *mut u8, size: usize);
    fn virtqueue_add_inbuf(vq: *mut Virtqueue, sg: *mut Scatterlist, num: u32,
                           data: *mut c_void, gfp: u32) -> c_int;
    fn virtqueue_kick(vq: *mut Virtqueue) -> bool;
    fn wait_for_completion_killable(completion: *mut Completion) -> c_int;
    fn ida_alloc(ida: *mut Ida, gfp: u32) -> c_int;
    fn ida_free(ida: *mut Ida, index: c_int);
    fn init_completion(completion: *mut Completion);
    fn virtio_find_single_vq(vdev: *mut VirtioDevice, callback: Option<unsafe extern "C" fn(*mut Virtqueue)>,
                             name: *const c_char) -> *mut Virtqueue;
    fn virtio_device_ready(vdev: *mut VirtioDevice);
    fn hwrng_register(rng: *mut Hwrng) -> c_int;
    fn hwrng_unregister(rng: *mut Hwrng);
    fn virtio_reset_device(vdev: *mut VirtioDevice);
    fn kfree(ptr: *mut c_void);
    fn kzalloc(size: usize, gfp: u32) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, size: usize) -> *mut c_void;
}

const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const GFP_KERNEL: u32 = 0;
const VIRTIO_ID_RNG: u16 = 4;
const VIRTIO_DEV_ANY_ID: u16 = 0xffff;

#[repr(C)]
pub struct Ida { _private: [u8; 0] }
#[repr(C)]
pub struct Completion { _private: [u8; 0] }
#[repr(C)]
pub struct Scatterlist { _private: [u8; 0] }
#[repr(C)]
pub struct Virtqueue { pub vdev: *mut VirtioDevice }
#[repr(C)]
pub struct VirtioConfig { pub del_vqs: Option<unsafe extern "C" fn(*mut VirtioDevice)> }
#[repr(C)]
pub struct VirtioDevice { pub priv_: *mut VirtrngInfo, pub config: *mut VirtioConfig }
#[repr(C)]
pub struct Hwrng {
    pub read: Option<unsafe extern "C" fn(*mut Hwrng, *mut c_void, usize, bool) -> c_int>,
    pub cleanup: Option<unsafe extern "C" fn(*mut Hwrng)>,
    pub priv_: usize,
    pub name: *mut c_char,
}
#[repr(C)]
pub struct VirtioDeviceId { pub device: u16, pub vendor: u16 }
#[repr(C)]
pub struct VirtioDriver {
    pub name: *const c_char,
    pub id_table: *const VirtioDeviceId,
    pub probe: Option<unsafe extern "C" fn(*mut VirtioDevice) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut VirtioDevice)>,
    pub scan: Option<unsafe extern "C" fn(*mut VirtioDevice)>,
    pub freeze: Option<unsafe extern "C" fn(*mut VirtioDevice) -> c_int>,
    pub restore: Option<unsafe extern "C" fn(*mut VirtioDevice) -> c_int>,
}

#[repr(C)]
pub struct VirtrngInfo {
    pub hwrng: Hwrng,
    pub vq: *mut Virtqueue,
    pub name: [c_char; 25],
    pub index: c_int,
    pub hwrng_register_done: bool,
    pub hwrng_removed: bool,
    pub have_data: Completion,
    pub data_avail: u32,
    pub data_idx: u32,
    // Sized to the minimum buffer returned by rng_buffer_size(); SMP_CACHE_BYTES is build-time.
    pub data: [u8; 32],
}

unsafe extern "C" fn random_recv_done(vq: *mut Virtqueue) {
    let vi = (*(*vq).vdev).priv_;
    let mut len = 0u32;
    if virtqueue_get_buf((*vi).vq, &mut len).is_null() { return; }
    smp_store_release(&mut (*vi).data_avail, len);
    complete(&mut (*vi).have_data);
}

unsafe fn request_entropy(vi: *mut VirtrngInfo) {
    let mut sg = core::mem::MaybeUninit::<Scatterlist>::uninit();
    reinit_completion(&mut (*vi).have_data);
    (*vi).data_idx = 0;
    sg_init_one(sg.as_mut_ptr(), (*vi).data.as_mut_ptr(), core::mem::size_of_val(&(*vi).data));
    virtqueue_add_inbuf((*vi).vq, sg.as_mut_ptr(), 1, (*vi).data.as_mut_ptr() as *mut c_void, GFP_KERNEL);
    virtqueue_kick((*vi).vq);
}

unsafe fn copy_data(vi: *mut VirtrngInfo, buf: *mut u8, mut size: u32) -> u32 {
    let avail = core::cmp::min((*vi).data_avail, core::mem::size_of_val(&(*vi).data) as u32);
    if (*vi).data_idx >= avail {
        (*vi).data_avail = 0;
        request_entropy(vi);
        return 0;
    }
    size = core::cmp::min(size, avail - (*vi).data_idx);
    let idx = (*vi).data_idx;
    memcpy(buf as *mut c_void, (*vi).data.as_ptr().add(idx as usize) as *const c_void, size as usize);
    (*vi).data_idx = (*vi).data_idx.wrapping_add(size);
    (*vi).data_avail = (*vi).data_avail.wrapping_sub(size);
    if (*vi).data_avail == 0 { request_entropy(vi); }
    size
}

unsafe extern "C" fn virtio_read(rng: *mut Hwrng, buf: *mut c_void, mut size: usize, wait: bool) -> c_int {
    let vi = (*rng).priv_ as *mut VirtrngInfo;
    if (*vi).hwrng_removed { return -ENODEV; }
    let mut read = 0usize;
    if (*vi).data_avail != 0 {
        let chunk = copy_data(vi, buf as *mut u8, size as u32) as usize;
        size -= chunk; read += chunk;
    }
    if !wait { return read as c_int; }
    while size != 0 {
        let ret = wait_for_completion_killable(&mut (*vi).have_data);
        if ret < 0 { return ret; }
        if (*vi).data_avail == 0 { return read as c_int; }
        let chunk = copy_data(vi, (buf as *mut u8).add(read), size as u32) as usize;
        size -= chunk; read += chunk;
    }
    read as c_int
}

unsafe extern "C" fn virtio_cleanup(rng: *mut Hwrng) {
    let vi = (*rng).priv_ as *mut VirtrngInfo;
    complete(&mut (*vi).have_data);
}

unsafe extern "C" fn probe_common(vdev: *mut VirtioDevice) -> c_int {
    let vi = kzalloc(core::mem::size_of::<VirtrngInfo>(), GFP_KERNEL) as *mut VirtrngInfo;
    if vi.is_null() { return -ENOMEM; }
    let index = ida_alloc(&mut rng_index_ida, GFP_KERNEL);
    if index < 0 { kfree(vi as *mut c_void); return index; }
    (*vi).index = index;
    let prefix = b"virtio_rng.";
    for (i, b) in prefix.iter().enumerate() { (*vi).name[i] = *b as c_char; }
    let digit = (index % 10) as u8;
    (*vi).name[prefix.len()] = (b'0' + digit) as c_char;
    init_completion(&mut (*vi).have_data);
    (*vi).hwrng = Hwrng { read: Some(virtio_read), cleanup: Some(virtio_cleanup),
        priv_: vi as usize, name: (*vi).name.as_mut_ptr() };
    (*vdev).priv_ = vi;
    (*vi).vq = virtio_find_single_vq(vdev, Some(random_recv_done), b"input\0".as_ptr() as *const c_char);
    if (*vi).vq.is_null() { ida_free(&mut rng_index_ida, index); kfree(vi as *mut c_void); return -ENODEV; }
    virtio_device_ready(vdev);
    request_entropy(vi);
    0
}

unsafe extern "C" fn remove_common(vdev: *mut VirtioDevice) {
    let vi = (*vdev).priv_;
    (*vi).hwrng_removed = true;
    (*vi).data_avail = 0;
    (*vi).data_idx = 0;
    complete(&mut (*vi).have_data);
    if (*vi).hwrng_register_done { hwrng_unregister(&mut (*vi).hwrng); }
    virtio_reset_device(vdev);
    if !(*vdev).config.is_null() {
        if let Some(del_vqs) = (*(*vdev).config).del_vqs { del_vqs(vdev); }
    }
    ida_free(&mut rng_index_ida, (*vi).index);
    kfree(vi as *mut c_void);
}
unsafe extern "C" fn virtrng_probe(vdev: *mut VirtioDevice) -> c_int { probe_common(vdev) }
unsafe extern "C" fn virtrng_remove(vdev: *mut VirtioDevice) { remove_common(vdev) }
unsafe extern "C" fn virtrng_scan(vdev: *mut VirtioDevice) {
    let vi = (*vdev).priv_;
    if hwrng_register(&mut (*vi).hwrng) == 0 { (*vi).hwrng_register_done = true; }
}
unsafe extern "C" fn virtrng_freeze(vdev: *mut VirtioDevice) -> c_int { remove_common(vdev); 0 }
unsafe extern "C" fn virtrng_restore(vdev: *mut VirtioDevice) -> c_int {
    let err = probe_common(vdev);
    if err == 0 {
        let vi = (*vdev).priv_;
        (*vi).hwrng_removed = true;
        let register_err = hwrng_register(&mut (*vi).hwrng);
        if register_err == 0 { (*vi).hwrng_register_done = true; (*vi).hwrng_removed = false; }
        return register_err;
    }
    err
}

#[no_mangle]
pub static mut id_table: [VirtioDeviceId; 2] = [
    VirtioDeviceId { device: VIRTIO_ID_RNG, vendor: VIRTIO_DEV_ANY_ID },
    VirtioDeviceId { device: 0, vendor: 0 },
];

#[no_mangle]
pub static mut virtio_rng_driver: VirtioDriver = VirtioDriver {
    name: b"virtio_rng\0".as_ptr() as *const c_char,
    id_table: id_table.as_ptr(),
    probe: Some(virtrng_probe),
    remove: Some(virtrng_remove),
    scan: Some(virtrng_scan),
    freeze: Some(virtrng_freeze),
    restore: Some(virtrng_restore),
};

// module_virtio_driver(virtio_rng_driver);
// MODULE_DEVICE_TABLE(virtio, id_table);
// MODULE_DESCRIPTION("Virtio random number driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
