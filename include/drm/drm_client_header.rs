/* SPDX-License-Identifier: GPL-2.0 or MIT */

/* Dependencies supplied by the surrounding DRM bindings. */

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type u32 = core::ffi::c_uint;
pub type bool = core::primitive::bool;

#[repr(C)]
pub struct drm_client_dev {
    pub dev: *mut drm_device,
    pub name: *const c_char,
    pub list: list_head,
    pub funcs: *const drm_client_funcs,
    pub file: *mut drm_file,
    pub modeset_mutex: mutex,
    pub modesets: *mut drm_mode_set,
    pub suspended: bool,
    pub hotplug_pending: bool,
    pub hotplug_failed: bool,
}

#[repr(C)]
pub struct drm_client_funcs {
    pub owner: *mut module,
    pub free: Option<unsafe extern "C" fn(client: *mut drm_client_dev)>,
    pub unregister: Option<unsafe extern "C" fn(client: *mut drm_client_dev)>,
    pub restore: Option<unsafe extern "C" fn(client: *mut drm_client_dev, force: bool) -> c_int>,
    pub hotplug: Option<unsafe extern "C" fn(client: *mut drm_client_dev) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(client: *mut drm_client_dev) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(client: *mut drm_client_dev) -> c_int>,
}

#[repr(C)]
pub struct drm_client_buffer {
    pub client: *mut drm_client_dev,
    pub gem: *mut drm_gem_object,
    pub map: iosys_map,
    pub fb: *mut drm_framebuffer,
}

/* Opaque types declared by included DRM and kernel headers. */
#[repr(C)] pub struct drm_device { _private: [u8; 0] }
#[repr(C)] pub struct drm_file { _private: [u8; 0] }
#[repr(C)] pub struct drm_framebuffer { _private: [u8; 0] }
#[repr(C)] pub struct drm_gem_object { _private: [u8; 0] }
#[repr(C)] pub struct drm_minor { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct drm_rect { _private: [u8; 0] }
#[repr(C)] pub struct drm_mode_set { pub crtc: *mut c_void, _private: [u8; 0] }
#[repr(C)] pub struct drm_connector { pub connector_type: c_uint, _private: [u8; 0] }
#[repr(C)] pub struct drm_connector_list_iter { _private: [u8; 0] }
#[repr(C)] pub struct iosys_map { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }

extern "C" {
    pub fn drm_client_init(
        dev: *mut drm_device,
        client: *mut drm_client_dev,
        name: *const c_char,
        funcs: *const drm_client_funcs,
    ) -> c_int;
    pub fn drm_client_release(client: *mut drm_client_dev);
    pub fn drm_client_register(client: *mut drm_client_dev);

    pub fn drm_client_buffer_create(
        client: *mut drm_client_dev,
        width: u32,
        height: u32,
        format: u32,
        handle: u32,
        pitch: u32,
    ) -> *mut drm_client_buffer;
    pub fn drm_client_buffer_create_dumb(
        client: *mut drm_client_dev,
        width: u32,
        height: u32,
        format: u32,
    ) -> *mut drm_client_buffer;
    pub fn drm_client_buffer_delete(buffer: *mut drm_client_buffer);
    pub fn drm_client_buffer_flush(buffer: *mut drm_client_buffer, rect: *mut drm_rect) -> c_int;
    pub fn drm_client_buffer_vmap_local(
        buffer: *mut drm_client_buffer,
        map_copy: *mut iosys_map,
    ) -> c_int;
    pub fn drm_client_buffer_vunmap_local(buffer: *mut drm_client_buffer);
    pub fn drm_client_buffer_vmap(buffer: *mut drm_client_buffer, map: *mut iosys_map) -> c_int;
    pub fn drm_client_buffer_vunmap(buffer: *mut drm_client_buffer);

    pub fn drm_client_modeset_create(client: *mut drm_client_dev) -> c_int;
    pub fn drm_client_modeset_free(client: *mut drm_client_dev);
    pub fn drm_client_modeset_probe(client: *mut drm_client_dev, width: c_uint, height: c_uint) -> c_int;
    pub fn drm_client_rotation(modeset: *mut drm_mode_set, rotation: *mut c_uint) -> bool;
    pub fn drm_client_modeset_check(client: *mut drm_client_dev) -> c_int;
    pub fn drm_client_modeset_commit_locked(client: *mut drm_client_dev) -> c_int;
    pub fn drm_client_modeset_commit(client: *mut drm_client_dev) -> c_int;
    pub fn drm_client_modeset_dpms(client: *mut drm_client_dev, mode: c_int) -> c_int;
    pub fn drm_client_modeset_wait_for_vblank(client: *mut drm_client_dev, crtc_index: c_uint) -> c_int;
}

/*
 * drm_client_for_each_modeset() - Iterate over client modesets
 * @modeset: &drm_mode_set loop cursor
 * @client: DRM client
 *
 * The lockdep assertion is supplied by the surrounding kernel bindings.
 */
#[macro_export]
macro_rules! drm_client_for_each_modeset {
    ($modeset:ident, $client:expr) => {
        for $modeset in unsafe { (*$client).modesets } {
            if (*$modeset).crtc.is_null() { break; }
        }
    };
}

/* Connector iteration excludes writeback connectors; use the surrounding
 * drm_for_each_connector_iter implementation for the actual iterator. */
#[macro_export]
macro_rules! drm_client_for_each_connector_iter {
    ($connector:ident, $iter:ident) => {
        drm_for_each_connector_iter!($connector, $iter);
        if $connector.connector_type != DRM_MODE_CONNECTOR_WRITEBACK
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
