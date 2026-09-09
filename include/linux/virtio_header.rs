/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/virtio.h. Kernel-provided types and macros remain external. */

pub struct module;

#[repr(C)]
pub struct virtqueue {
    pub list: list_head,
    pub callback: Option<unsafe extern "C" fn(vq: *mut virtqueue)>,
    pub name: *const ::core::ffi::c_char,
    pub vdev: *mut virtio_device,
    pub index: ::core::ffi::c_uint,
    pub num_free: ::core::ffi::c_uint,
    pub num_max: ::core::ffi::c_uint,
    pub reset: bool,
    pub priv_: *mut ::core::ffi::c_void,
}

pub struct vduse_vq_group;

#[repr(C)]
pub union virtio_map {
    pub dma_dev: *mut device,
    pub group: *mut vduse_vq_group,
}

extern "C" {
    pub fn virtqueue_add_outbuf(vq: *mut virtqueue, sg: *mut scatterlist,
        num: ::core::ffi::c_uint, data: *mut ::core::ffi::c_void, gfp: gfp_t) -> ::core::ffi::c_int;
    pub fn virtqueue_add_inbuf(vq: *mut virtqueue, sg: *mut scatterlist,
        num: ::core::ffi::c_uint, data: *mut ::core::ffi::c_void, gfp: gfp_t) -> ::core::ffi::c_int;
    pub fn virtqueue_add_inbuf_cache_clean(vq: *mut virtqueue, sg: *mut scatterlist,
        num: ::core::ffi::c_uint, data: *mut ::core::ffi::c_void, gfp: gfp_t) -> ::core::ffi::c_int;
    pub fn virtqueue_add_inbuf_ctx(vq: *mut virtqueue, sg: *mut scatterlist,
        num: ::core::ffi::c_uint, data: *mut ::core::ffi::c_void, ctx: *mut ::core::ffi::c_void, gfp: gfp_t) -> ::core::ffi::c_int;
    pub fn virtqueue_add_inbuf_premapped(vq: *mut virtqueue, sg: *mut scatterlist,
        num: ::core::ffi::c_uint, data: *mut ::core::ffi::c_void, ctx: *mut ::core::ffi::c_void, gfp: gfp_t) -> ::core::ffi::c_int;
    pub fn virtqueue_add_outbuf_premapped(vq: *mut virtqueue, sg: *mut scatterlist,
        num: ::core::ffi::c_uint, data: *mut ::core::ffi::c_void, gfp: gfp_t) -> ::core::ffi::c_int;
    pub fn virtqueue_add_sgs(vq: *mut virtqueue, sgs: *mut *mut scatterlist,
        out_sgs: ::core::ffi::c_uint, in_sgs: ::core::ffi::c_uint,
        data: *mut ::core::ffi::c_void, gfp: gfp_t) -> ::core::ffi::c_int;
    pub fn virtqueue_dma_dev(vq: *mut virtqueue) -> *mut device;
    pub fn virtqueue_kick(vq: *mut virtqueue) -> bool;
    pub fn virtqueue_kick_prepare(vq: *mut virtqueue) -> bool;
    pub fn virtqueue_notify(vq: *mut virtqueue) -> bool;
    pub fn virtqueue_get_buf(vq: *mut virtqueue, len: *mut ::core::ffi::c_uint) -> *mut ::core::ffi::c_void;
    pub fn virtqueue_get_buf_ctx(vq: *mut virtqueue, len: *mut ::core::ffi::c_uint, ctx: *mut *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    pub fn virtqueue_disable_cb(vq: *mut virtqueue);
    pub fn virtqueue_enable_cb(vq: *mut virtqueue) -> bool;
    pub fn virtqueue_enable_cb_prepare(vq: *mut virtqueue) -> ::core::ffi::c_uint;
    pub fn virtqueue_poll(vq: *mut virtqueue, _: ::core::ffi::c_uint) -> bool;
    pub fn virtqueue_enable_cb_delayed(vq: *mut virtqueue) -> bool;
    pub fn virtqueue_detach_unused_buf(vq: *mut virtqueue) -> *mut ::core::ffi::c_void;
    pub fn virtqueue_get_vring_size(vq: *const virtqueue) -> ::core::ffi::c_uint;
    pub fn virtqueue_is_broken(vq: *const virtqueue) -> bool;
    pub fn virtqueue_get_vring(vq: *const virtqueue) -> *const vring;
    pub fn virtqueue_get_desc_addr(vq: *const virtqueue) -> dma_addr_t;
    pub fn virtqueue_get_avail_addr(vq: *const virtqueue) -> dma_addr_t;
    pub fn virtqueue_get_used_addr(vq: *const virtqueue) -> dma_addr_t;
    pub fn virtqueue_resize(vq: *mut virtqueue, num: u32,
        recycle: Option<unsafe extern "C" fn(*mut virtqueue, *mut ::core::ffi::c_void)>,
        recycle_done: Option<unsafe extern "C" fn(*mut virtqueue)>) -> ::core::ffi::c_int;
    pub fn virtqueue_reset(vq: *mut virtqueue,
        recycle: Option<unsafe extern "C" fn(*mut virtqueue, *mut ::core::ffi::c_void)>,
        recycle_done: Option<unsafe extern "C" fn(*mut virtqueue)>) -> ::core::ffi::c_int;
}

#[repr(C)]
pub struct virtio_admin_cmd {
    pub opcode: __le16,
    pub group_type: __le16,
    pub group_member_id: __le64,
    pub data_sg: *mut scatterlist,
    pub result_sg: *mut scatterlist,
    pub completion: completion,
    pub result_sg_size: u32,
    pub ret: ::core::ffi::c_int,
}

#[repr(C)]
pub struct virtio_device {
    pub index: ::core::ffi::c_int,
    pub failed: bool,
    pub config_core_enabled: bool,
    pub config_driver_disabled: bool,
    pub config_change_pending: bool,
    pub config_lock: spinlock_t,
    pub vqs_list_lock: spinlock_t,
    pub dev: device,
    pub id: virtio_device_id,
    pub config: *const virtio_config_ops,
    pub vringh_config: *const vringh_config_ops,
    pub map: *const virtio_map_ops,
    pub vqs: list_head,
    pub features: [u64; VIRTIO_FEATURES_U64S],
    pub priv_: *mut ::core::ffi::c_void,
    pub vmap: virtio_map,
    /* CONFIG_VIRTIO_DEBUG: debugfs_dir and debugfs_filter_features. */
}

#[macro_export]
macro_rules! dev_to_virtio { ($dev:expr) => { container_of_const!($dev, virtio_device, dev) }; }

extern "C" {
    pub fn virtio_add_status(dev: *mut virtio_device, status: ::core::ffi::c_uint);
    pub fn register_virtio_device(dev: *mut virtio_device) -> ::core::ffi::c_int;
    pub fn unregister_virtio_device(dev: *mut virtio_device);
    pub fn is_virtio_device(dev: *mut device) -> bool;
    pub fn virtio_break_device(dev: *mut virtio_device);
    pub fn __virtio_unbreak_device(dev: *mut virtio_device);
    pub fn __virtqueue_break(vq: *mut virtqueue);
    pub fn __virtqueue_unbreak(vq: *mut virtqueue);
    pub fn virtio_config_changed(dev: *mut virtio_device);
    pub fn virtio_config_driver_disable(dev: *mut virtio_device);
    pub fn virtio_config_driver_enable(dev: *mut virtio_device);
    pub fn virtio_reset_device(dev: *mut virtio_device);
    pub fn virtio_device_shutdown(dev: *mut virtio_device);
    pub fn virtio_device_reset_prepare(dev: *mut virtio_device) -> ::core::ffi::c_int;
    pub fn virtio_device_reset_done(dev: *mut virtio_device) -> ::core::ffi::c_int;
    pub fn virtio_max_dma_size(vdev: *const virtio_device) -> usize;
    #[cfg(CONFIG_PM_SLEEP)]
    pub fn virtio_device_freeze(dev: *mut virtio_device) -> ::core::ffi::c_int;
    #[cfg(CONFIG_PM_SLEEP)]
    pub fn virtio_device_restore(dev: *mut virtio_device) -> ::core::ffi::c_int;
}

#[repr(C)]
pub struct virtio_driver {
    pub driver: device_driver,
    pub id_table: *const virtio_device_id,
    pub feature_table: *const ::core::ffi::c_uint,
    pub feature_table_size: ::core::ffi::c_uint,
    pub feature_table_legacy: *const ::core::ffi::c_uint,
    pub feature_table_size_legacy: ::core::ffi::c_uint,
    pub validate: Option<unsafe extern "C" fn(*mut virtio_device) -> ::core::ffi::c_int>,
    pub probe: Option<unsafe extern "C" fn(*mut virtio_device) -> ::core::ffi::c_int>,
    pub scan: Option<unsafe extern "C" fn(*mut virtio_device)>,
    pub remove: Option<unsafe extern "C" fn(*mut virtio_device)>,
    pub config_changed: Option<unsafe extern "C" fn(*mut virtio_device)>,
    pub freeze: Option<unsafe extern "C" fn(*mut virtio_device) -> ::core::ffi::c_int>,
    pub restore: Option<unsafe extern "C" fn(*mut virtio_device) -> ::core::ffi::c_int>,
    pub reset_prepare: Option<unsafe extern "C" fn(*mut virtio_device) -> ::core::ffi::c_int>,
    pub reset_done: Option<unsafe extern "C" fn(*mut virtio_device) -> ::core::ffi::c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut virtio_device)>,
}

#[macro_export]
macro_rules! drv_to_virtio { ($drv:expr) => { container_of_const!($drv, virtio_driver, driver) }; }

extern "C" {
    pub fn __register_virtio_driver(drv: *mut virtio_driver, owner: *mut module) -> ::core::ffi::c_int;
    pub fn unregister_virtio_driver(drv: *mut virtio_driver);
    pub fn virtqueue_map_alloc_coherent(vdev: *mut virtio_device, mapping_token: virtio_map,
        size: usize, dma_handle: *mut dma_addr_t, gfp: gfp_t) -> *mut ::core::ffi::c_void;
    pub fn virtqueue_map_free_coherent(vdev: *mut virtio_device, mapping_token: virtio_map,
        size: usize, vaddr: *mut ::core::ffi::c_void, dma_handle: dma_addr_t);
    pub fn virtqueue_map_page_attrs(vq: *const virtqueue, page: *mut page, offset: ::core::ffi::c_ulong,
        size: usize, dir: dma_data_direction, attrs: ::core::ffi::c_ulong) -> dma_addr_t;
    pub fn virtqueue_unmap_page_attrs(vq: *const virtqueue, dma_handle: dma_addr_t, size: usize,
        dir: dma_data_direction, attrs: ::core::ffi::c_ulong);
    pub fn virtqueue_map_single_attrs(vq: *const virtqueue, ptr: *mut ::core::ffi::c_void, size: usize,
        dir: dma_data_direction, attrs: ::core::ffi::c_ulong) -> dma_addr_t;
    pub fn virtqueue_unmap_single_attrs(vq: *const virtqueue, addr: dma_addr_t, size: usize,
        dir: dma_data_direction, attrs: ::core::ffi::c_ulong);
    pub fn virtqueue_map_mapping_error(vq: *const virtqueue, addr: dma_addr_t) -> ::core::ffi::c_int;
    pub fn virtqueue_map_need_sync(vq: *const virtqueue, addr: dma_addr_t) -> bool;
    pub fn virtqueue_map_sync_single_range_for_cpu(vq: *const virtqueue, addr: dma_addr_t,
        offset: ::core::ffi::c_ulong, size: usize, dir: dma_data_direction);
    pub fn virtqueue_map_sync_single_range_for_device(vq: *const virtqueue, addr: dma_addr_t,
        offset: ::core::ffi::c_ulong, size: usize, dir: dma_data_direction);
}

#[cfg(CONFIG_VIRTIO_DEBUG)]
extern "C" {
    pub fn virtio_debug_device_init(dev: *mut virtio_device);
    pub fn virtio_debug_device_exit(dev: *mut virtio_device);
    pub fn virtio_debug_device_filter_features(dev: *mut virtio_device);
    pub fn virtio_debug_init();
    pub fn virtio_debug_exit();
}

/* CONFIG_VIRTIO_DEBUG and CONFIG_PM_SLEEP conditional declarations are preserved as build-time intent. */
#[cfg(not(CONFIG_VIRTIO_DEBUG))]
pub unsafe fn virtio_debug_device_init(_: *mut virtio_device) {}
#[cfg(not(CONFIG_VIRTIO_DEBUG))]
pub unsafe fn virtio_debug_device_exit(_: *mut virtio_device) {}
#[cfg(not(CONFIG_VIRTIO_DEBUG))]
pub unsafe fn virtio_debug_device_filter_features(_: *mut virtio_device) {}
#[cfg(not(CONFIG_VIRTIO_DEBUG))]
pub unsafe fn virtio_debug_init() {}
#[cfg(not(CONFIG_VIRTIO_DEBUG))]
pub unsafe fn virtio_debug_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
