// SPDX-License-Identifier: GPL-2.0-or-later
/* Xenbus code for blkif backend. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const RINGREF_NAME_LEN: usize = 20;

#[repr(C)] pub struct xenbus_device { pub dev: device, pub nodename: *const c_char, pub otherend: *const c_char, pub otherend_id: domid_t, pub state: xenbus_state }
#[repr(C)] pub struct device;
#[repr(C)] pub struct xenbus_watch { pub node: *mut c_char }
#[repr(C)] pub struct xen_blkif { pub be: *mut backend_info, pub domid: domid_t, pub rings: *mut xen_blkif_ring, pub nr_rings: c_uint, pub nr_ring_pages: c_uint, pub blk_protocol: c_int, pub multi_ref: bool, pub vbd: xen_vbd, pub buffer_squeeze_end: c_ulong, pub free_work: work_struct, pub refcnt: atomic_t, pub drain_complete: completion }
#[repr(C)] pub struct xen_blkif_ring;
#[repr(C)] pub struct xen_vbd { pub bdev_file: *mut file, pub handle: blkif_vdev_t, pub readonly: c_int, pub type_: c_uint, pub pdevice: c_uint, pub size: u64, pub flush_support: bool, pub discard_secure: bool, pub feature_gnt_persistent_parm: bool, pub feature_gnt_persistent: bool, pub overflow_max_grants: c_uint }
#[repr(C)] pub struct backend_info { pub dev: *mut xenbus_device, pub blkif: *mut xen_blkif, pub backend_watch: xenbus_watch, pub major: c_uint, pub minor: c_uint, pub mode: *mut c_char }
#[repr(C)] pub struct work_struct; #[repr(C)] pub struct atomic_t; #[repr(C)] pub struct completion; #[repr(C)] pub struct file; #[repr(C)] pub struct block_device; #[repr(C)] pub struct xenbus_transaction; #[repr(C)] pub struct xenbus_device_id; #[repr(C)] pub struct xenbus_driver; #[repr(C)] pub struct device_attribute; #[repr(C)] pub struct attribute; #[repr(C)] pub struct attribute_group; #[repr(C)] pub struct kthread;
type domid_t = u16; type grant_ref_t = u32; type blkif_vdev_t = u32; type ssize_t = isize;
#[repr(C)] enum xenbus_state { XenbusStateUnknown, XenbusStateInitialising, XenbusStateInitWait, XenbusStateInitialised, XenbusStateConnected, XenbusStateClosing, XenbusStateClosed }

extern "C" {
    static mut xen_blkif_cachep: *mut c_void; static mut feature_persistent: bool; static mut buffer_squeeze_duration_ms: c_uint;
    static xenblk_max_queues: c_uint; static xen_blkif_max_ring_order: c_uint;
    fn xen_blkif_free(_: *mut xen_blkif); fn xen_vbd_free(_: *mut xen_vbd); fn connect(_: *mut backend_info); fn connect_ring(_: *mut backend_info) -> c_int;
    fn xenbus_read(_: c_int, _: *const c_char, _: *const c_char, _: *mut c_void) -> *mut c_char; fn kfree(_: *mut c_void); fn xenbus_dev_error(_: *mut xenbus_device, _: c_int, _: *const c_char, ...); fn xenbus_dev_fatal(_: *mut xenbus_device, _: c_int, _: *const c_char, ...);
    fn xen_blkif_schedule(_: *mut xen_blkif_ring) -> c_int; fn kthread_run(_: unsafe extern "C" fn(*mut xen_blkif_ring)->c_int, _: *mut xen_blkif_ring, _: *const c_char, ...)->*mut kthread; fn kthread_stop(_: *mut kthread)->c_int;
    fn sync_blockdev(_: *mut block_device)->c_int; fn invalidate_inode_pages2(_: *mut c_void)->c_int; fn file_bdev(_: *mut file)->*mut block_device; fn PTR_ERR(_: *const c_void)->c_int; fn IS_ERR(_: *const c_void)->bool;
    fn xenbus_map_ring_valloc(_: *mut xenbus_device, _: *mut grant_ref_t, _: c_uint, _: *mut *mut c_void)->c_int; fn xenbus_unmap_ring_vfree(_: *mut xenbus_device, _: *mut c_void); fn bind_interdomain_evtchn_to_irqhandler_lateeoi(_: *mut xenbus_device, _: c_uint, _: *const c_void, _: c_int, _: *const c_char, _: *mut xen_blkif_ring)->c_int; fn unbind_from_irqhandler(_: c_int, _: *mut xen_blkif_ring);
    fn xen_blkbk_unmap_purged_grants(_: *mut work_struct); fn xen_blkbk_free_caches(_: *mut xen_blkif_ring); fn xen_blkif_put(_: *mut xen_blkif); fn fput(_: *mut file); fn vbd_sz(_: *mut xen_vbd)->u64; fn bdev_file_open_by_dev(_: c_uint, _: c_uint, _: *mut c_void, _: *mut c_void)->*mut file; fn disk_to_cdi(_: *mut c_void)->*mut c_void; fn bdev_write_cache(_: *mut block_device)->bool; fn bdev_max_secure_erase_sectors(_: *mut block_device)->u64;
    fn xenbus_printf(_: *mut xenbus_transaction, _: *const c_char, _: *const c_char, _: *const c_char, ...)->c_int; fn xenbus_read_unsigned(_: *const c_char, _: *const c_char, _: c_uint)->c_uint; fn bdev_max_discard_sectors(_: *mut block_device)->u64; fn bdev_discard_granularity(_: *mut block_device)->c_uint; fn bdev_discard_alignment(_: *mut block_device)->c_uint; fn xenbus_switch_state(_: *mut xenbus_device, _: xenbus_state)->c_int; fn xenbus_watch_pathfmt(_: *mut xenbus_device, _: *mut xenbus_watch, _: *const c_char, _: *const c_void, _: *const c_char, ...)->c_int; fn xenbus_scanf(_: *mut xenbus_transaction, _: *const c_char, _: *const c_char, _: *const c_char, ...)->c_int; fn xenbus_transaction_start(_: *mut xenbus_transaction)->c_int; fn xenbus_transaction_end(_: *mut xenbus_transaction, _: c_int)->c_int; fn xenbus_dev_is_online(_: *mut xenbus_device)->bool; fn device_unregister(_: *mut device); fn xenbus_strstate(_: xenbus_state)->*const c_char; fn xenbus_register_backend(_: *mut xenbus_driver)->c_int; fn xenbus_unregister_driver(_: *mut xenbus_driver);
}

#[no_mangle] pub unsafe extern "C" fn xen_blkbk_xenbus(be: *mut backend_info) -> *mut xenbus_device { (*be).dev }
unsafe fn xen_blkif_deferred_free(work: *mut work_struct) { xen_blkif_free(work as *mut xen_blkif); }
unsafe fn xen_update_blkif_status(blkif: *mut xen_blkif) { if (*blkif).rings.is_null() { return; } if (*(*blkif).be).dev.is_null() { return; } if (*(*(*blkif).be).dev).state == xenbus_state::XenbusStateConnected { return; } connect((*blkif).be); }
unsafe fn xen_blkif_disconnect(blkif: *mut xen_blkif) -> c_int { if !(*blkif).rings.is_null() { kfree((*blkif).rings as *mut c_void); (*blkif).rings = core::ptr::null_mut(); } (*blkif).nr_rings=0; (*blkif).nr_ring_pages=0; 0 }
unsafe fn xen_blkif_alloc(domid: domid_t) -> *mut xen_blkif { let p=libc::calloc(1, core::mem::size_of::<xen_blkif>()) as *mut xen_blkif; if !p.is_null(){(*p).domid=domid;} p }
unsafe fn xen_blkif_alloc_rings(_: *mut xen_blkif)->c_int { 0 }
unsafe fn xen_vbd_create(_: *mut xen_blkif, _: blkif_vdev_t, _: c_uint, _: c_uint, _: c_int, _: c_int)->c_int { 0 }
unsafe fn xen_blkbk_remove(dev:*mut xenbus_device){ let be=(*dev).dev as *mut backend_info; if !be.is_null(){ if !(*be).blkif.is_null(){xen_blkif_disconnect((*be).blkif); xen_blkif_free((*be).blkif);} kfree(be as *mut c_void); } }
unsafe fn xen_blkbk_probe(dev:*mut xenbus_device, _: *const xenbus_device_id)->c_int { let be=libc::calloc(1,core::mem::size_of::<backend_info>()) as *mut backend_info; if be.is_null(){return -12;} (*be).dev=dev; (*dev).dev=core::mem::zeroed(); (*be).blkif=xen_blkif_alloc((*dev).otherend_id); if (*be).blkif.is_null(){xen_blkbk_remove(dev);return -12;} (*(*be).blkif).be=be; xenbus_switch_state(dev,xenbus_state::XenbusStateInitWait) }
unsafe fn backend_changed(_: *mut xenbus_watch, _: *const c_char, _: *const c_char) {}
unsafe fn frontend_changed(_: *mut xenbus_device, _: xenbus_state) {}
unsafe fn reclaim_memory(_: *mut xenbus_device) {}
unsafe fn connect(_: *mut backend_info) {}
unsafe fn read_per_ring_refs(_: *mut xen_blkif_ring, _: *const c_char)->c_int { 0 }
unsafe fn connect_ring(_: *mut backend_info)->c_int { 0 }

#[no_mangle] pub unsafe extern "C" fn xen_blkif_interface_init()->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn xen_blkif_interface_fini() {}
#[no_mangle] pub unsafe extern "C" fn xen_blkbk_flush_diskcache(_: *mut xenbus_transaction, _: *mut backend_info, _: c_int)->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn xen_blkbk_barrier(_: *mut xenbus_transaction, _: *mut backend_info, _: c_int)->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn xen_blkif_xenbus_init()->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn xen_blkif_xenbus_fini() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
