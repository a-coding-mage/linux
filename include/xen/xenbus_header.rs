/* Translated from xenbus.h.  Kernel and Xen types are supplied by dependencies. */

pub const XENBUS_MAX_RING_GRANT_ORDER: u32 = 4;
pub const XENBUS_MAX_RING_GRANTS: u32 = 1u32 << XENBUS_MAX_RING_GRANT_ORDER;

#[repr(C)]
pub struct xenbus_watch {
    pub list: list_head,
    pub node: *const ::core::ffi::c_char,
    pub nr_pending: u32,
    pub will_handle: Option<unsafe extern "C" fn(
        *mut xenbus_watch,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
    ) -> bool>,
    pub callback: Option<unsafe extern "C" fn(
        *mut xenbus_watch,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
    )>,
}

#[repr(C)]
pub struct xenbus_device {
    pub devicetype: *const ::core::ffi::c_char,
    pub nodename: *const ::core::ffi::c_char,
    pub otherend: *const ::core::ffi::c_char,
    pub vanished: bool,
    pub otherend_id: ::core::ffi::c_int,
    pub otherend_watch: xenbus_watch,
    pub dev: device,
    pub state: xenbus_state,
    pub down: completion,
    pub work: work_struct,
    pub reclaim_sem: semaphore,
    pub event_channels: atomic_t,
    pub events: atomic_t,
    pub spurious_events: atomic_t,
    pub jiffies_eoi_delayed: atomic_t,
    pub spurious_threshold: u32,
}

#[macro_export]
macro_rules! to_xenbus_device {
    ($dev:expr) => { container_of_const!($dev, xenbus_device, dev) };
}

#[repr(C)]
pub struct xenbus_device_id {
    pub devicetype: [::core::ffi::c_char; 32],
}

#[repr(C)]
pub struct xenbus_driver {
    pub name: *const ::core::ffi::c_char,
    pub ids: *const xenbus_device_id,
    pub allow_rebind: bool,
    pub not_essential: bool,
    pub probe: Option<unsafe extern "C" fn(*mut xenbus_device, *const xenbus_device_id) -> ::core::ffi::c_int>,
    pub otherend_changed: Option<unsafe extern "C" fn(*mut xenbus_device, xenbus_state)>,
    pub remove: Option<unsafe extern "C" fn(*mut xenbus_device)>,
    pub suspend: Option<unsafe extern "C" fn(*mut xenbus_device) -> ::core::ffi::c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut xenbus_device) -> ::core::ffi::c_int>,
    pub uevent: Option<unsafe extern "C" fn(*const xenbus_device, *mut kobj_uevent_env) -> ::core::ffi::c_int>,
    pub driver: device_driver,
    pub read_otherend_details: Option<unsafe extern "C" fn(*mut xenbus_device) -> ::core::ffi::c_int>,
    pub is_ready: Option<unsafe extern "C" fn(*mut xenbus_device) -> ::core::ffi::c_int>,
    pub reclaim_memory: Option<unsafe extern "C" fn(*mut xenbus_device)>,
}

#[macro_export]
macro_rules! to_xenbus_driver {
    ($drv:expr) => { container_of_const!($drv, xenbus_driver, driver) };
}

#[repr(C)]
pub struct xenbus_transaction { pub id: u32 }

pub const XBT_NIL: xenbus_transaction = xenbus_transaction { id: 0 };

extern "C" {
    pub fn __xenbus_register_frontend(drv: *mut xenbus_driver, owner: *mut module, mod_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn __xenbus_register_backend(drv: *mut xenbus_driver, owner: *mut module, mod_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn xenbus_unregister_driver(drv: *mut xenbus_driver);
    pub fn xenbus_directory(t: xenbus_transaction, dir: *const ::core::ffi::c_char, node: *const ::core::ffi::c_char, num: *mut u32) -> *mut *mut ::core::ffi::c_char;
    pub fn xenbus_read(t: xenbus_transaction, dir: *const ::core::ffi::c_char, node: *const ::core::ffi::c_char, len: *mut u32) -> *mut ::core::ffi::c_void;
    pub fn xenbus_write(t: xenbus_transaction, dir: *const ::core::ffi::c_char, node: *const ::core::ffi::c_char, string: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn xenbus_exists(t: xenbus_transaction, dir: *const ::core::ffi::c_char, node: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn xenbus_rm(t: xenbus_transaction, dir: *const ::core::ffi::c_char, node: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn xenbus_transaction_start(t: *mut xenbus_transaction) -> ::core::ffi::c_int;
    pub fn xenbus_transaction_end(t: xenbus_transaction, abort: bool) -> ::core::ffi::c_int;
    pub fn xenbus_scanf(t: xenbus_transaction, dir: *const ::core::ffi::c_char, node: *const ::core::ffi::c_char, fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    pub fn xenbus_read_unsigned(dir: *const ::core::ffi::c_char, node: *const ::core::ffi::c_char, default_val: u32) -> u32;
    pub fn xenbus_printf(t: xenbus_transaction, dir: *const ::core::ffi::c_char, node: *const ::core::ffi::c_char, fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    pub fn xenbus_gather(t: xenbus_transaction, dir: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    pub static mut xenstored_ready: ::core::ffi::c_int;
    pub fn register_xenstore_notifier(nb: *mut notifier_block) -> ::core::ffi::c_int;
    pub fn unregister_xenstore_notifier(nb: *mut notifier_block);
    pub fn register_xenbus_watch(watch: *mut xenbus_watch) -> ::core::ffi::c_int;
    pub fn unregister_xenbus_watch(watch: *mut xenbus_watch);
    pub fn xs_suspend();
    pub fn xs_resume();
    pub fn xs_suspend_cancel();
    pub fn xenbus_watch_path(dev: *mut xenbus_device, path: *const ::core::ffi::c_char, watch: *mut xenbus_watch, will_handle: Option<unsafe extern "C" fn(*mut xenbus_watch, *const ::core::ffi::c_char, *const ::core::ffi::c_char) -> bool>, callback: Option<unsafe extern "C" fn(*mut xenbus_watch, *const ::core::ffi::c_char, *const ::core::ffi::c_char)>) -> ::core::ffi::c_int;
    pub fn xenbus_watch_pathfmt(dev: *mut xenbus_device, watch: *mut xenbus_watch, will_handle: Option<unsafe extern "C" fn(*mut xenbus_watch, *const ::core::ffi::c_char, *const ::core::ffi::c_char) -> bool>, callback: Option<unsafe extern "C" fn(*mut xenbus_watch, *const ::core::ffi::c_char, *const ::core::ffi::c_char)>, pathfmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    pub fn xenbus_switch_state(dev: *mut xenbus_device, new_state: xenbus_state) -> ::core::ffi::c_int;
    pub fn xenbus_setup_ring(dev: *mut xenbus_device, gfp: gfp_t, vaddr: *mut *mut ::core::ffi::c_void, nr_pages: u32, grefs: *mut grant_ref_t) -> ::core::ffi::c_int;
    pub fn xenbus_teardown_ring(vaddr: *mut *mut ::core::ffi::c_void, nr_pages: u32, grefs: *mut grant_ref_t);
    pub fn xenbus_map_ring_valloc(dev: *mut xenbus_device, gnt_refs: *mut grant_ref_t, nr_grefs: u32, vaddr: *mut *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn xenbus_unmap_ring_vfree(dev: *mut xenbus_device, vaddr: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn xenbus_alloc_evtchn(dev: *mut xenbus_device, port: *mut evtchn_port_t) -> ::core::ffi::c_int;
    pub fn xenbus_free_evtchn(dev: *mut xenbus_device, port: evtchn_port_t) -> ::core::ffi::c_int;
    pub fn xenbus_read_driver_state(dev: *const xenbus_device, path: *const ::core::ffi::c_char) -> xenbus_state;
    pub fn xenbus_dev_error(dev: *mut xenbus_device, err: ::core::ffi::c_int, fmt: *const ::core::ffi::c_char, ...);
    pub fn xenbus_dev_fatal(dev: *mut xenbus_device, err: ::core::ffi::c_int, fmt: *const ::core::ffi::c_char, ...);
    pub fn xenbus_strstate(state: xenbus_state) -> *const ::core::ffi::c_char;
    pub fn xenbus_dev_is_online(dev: *mut xenbus_device) -> ::core::ffi::c_int;
    pub fn xenbus_frontend_closed(dev: *mut xenbus_device) -> ::core::ffi::c_int;
    pub static xen_xenbus_fops: file_operations;
    pub static mut xen_store_interface: *mut xenstore_domain_interface;
    pub static mut xen_store_evtchn: ::core::ffi::c_int;
}

#[macro_export]
macro_rules! xenbus_register_frontend { ($drv:expr) => { __xenbus_register_frontend($drv, THIS_MODULE, KBUILD_MODNAME) }; }
#[macro_export]
macro_rules! xenbus_register_backend { ($drv:expr) => { __xenbus_register_backend($drv, THIS_MODULE, KBUILD_MODNAME) }; }
#[macro_export]
macro_rules! XENBUS_EXIST_ERR { ($err:expr) => { ($err) == -ENOENT || ($err) == -ERANGE }; }

/* C macro preserved as a low-level Rust macro; IS_ERR, strlen, kfree and
 * ERR_PTR are supplied by the kernel dependency. */
#[macro_export]
macro_rules! XENBUS_IS_ERR_READ {
    ($str:ident) => {{
        if !IS_ERR!($str) && strlen($str) == 0 {
            kfree($str);
            $str = ERR_PTR!(-ERANGE);
        }
        IS_ERR!($str)
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
