/* SPDX-License-Identifier: MIT */
/*
 * Copyright © 2025 Intel Corporation
 */

// C dependencies supplied by other files:
// linux/list.h, linux/mutex.h

use core::ffi::c_void;

#[repr(C)]
pub struct drm_device {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct drm_pagemap {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct drm_pagemap_cache {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct drm_pagemap_owner {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct drm_pagemap_shrinker {
    _opaque: [u8; 0],
}

// Supplied by linux/list.h and linux/mutex.h.
#[repr(C)]
pub struct list_head {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _opaque: [u8; 0],
}

/**
 * struct drm_pagemap_peer - Structure representing a fast interconnect peer
 * @list: Pointer to a &struct drm_pagemap_owner_list used to keep track of peers
 * @link: List link for @list's list of peers.
 * @owner: Pointer to a &struct drm_pagemap_owner, common for a set of peers having
 * fast interconnects.
 * @private: Pointer private to the struct embedding this struct.
 */
#[repr(C)]
pub struct drm_pagemap_peer {
    pub list: *mut drm_pagemap_owner_list,
    pub link: list_head,
    pub owner: *mut drm_pagemap_owner,
    pub private: *mut c_void,
}

/**
 * struct drm_pagemap_owner_list - Keeping track of peers and owners
 * @peer: List of peers.
 *
 * The owner list defines the scope where we identify peers having fast interconnects
 * and a common owner. Typically a driver has a single global owner list to
 * keep track of common owners for the driver's pagemaps.
 */
#[repr(C)]
pub struct drm_pagemap_owner_list {
    /** @lock: Mutex protecting the @peers list. */
    pub lock: mutex,
    /** @peers: List of peers. */
    pub peers: list_head,
}

/*
 * Convenience macro to define an owner list.
 * Typically the owner list statically declared
 * driver-wide.
 *
 * The mutex and list initializers are supplied by the Linux compatibility layer.
 */
#[macro_export]
macro_rules! DRM_PAGEMAP_OWNER_LIST_DEFINE {
    ($name:ident) => {
        static mut $name: drm_pagemap_owner_list = drm_pagemap_owner_list {
            lock: __MUTEX_INITIALIZER!($name.lock),
            peers: LIST_HEAD_INIT!($name.peers),
        };
    };
}

extern "C" {
    pub fn drm_pagemap_shrinker_add(dpagemap: *mut drm_pagemap);

    pub fn drm_pagemap_cache_lock_lookup(cache: *mut drm_pagemap_cache) -> i32;

    pub fn drm_pagemap_cache_unlock_lookup(cache: *mut drm_pagemap_cache);

    pub fn drm_pagemap_shrinker_create_devm(
        drm: *mut drm_device,
    ) -> *mut drm_pagemap_shrinker;

    pub fn drm_pagemap_cache_create_devm(
        shrinker: *mut drm_pagemap_shrinker,
    ) -> *mut drm_pagemap_cache;

    pub fn drm_pagemap_get_from_cache(cache: *mut drm_pagemap_cache) -> *mut drm_pagemap;

    pub fn drm_pagemap_cache_set_pagemap(
        cache: *mut drm_pagemap_cache,
        dpagemap: *mut drm_pagemap,
    );

    pub fn drm_pagemap_get_from_cache_if_active(
        cache: *mut drm_pagemap_cache,
    ) -> *mut drm_pagemap;

    #[cfg(feature = "CONFIG_PROVE_LOCKING")]
    pub fn drm_pagemap_shrinker_might_lock(dpagemap: *mut drm_pagemap);

    pub fn drm_pagemap_release_owner(peer: *mut drm_pagemap_peer);

    pub fn drm_pagemap_acquire_owner(
        peer: *mut drm_pagemap_peer,
        owner_list: *mut drm_pagemap_owner_list,
        has_interconnect: Option<unsafe extern "C" fn(
            peer1: *mut drm_pagemap_peer,
            peer2: *mut drm_pagemap_peer,
        ) -> bool>,
    ) -> i32;
}

#[cfg(not(feature = "CONFIG_PROVE_LOCKING"))]
#[inline]
pub unsafe fn drm_pagemap_shrinker_might_lock(_dpagemap: *mut drm_pagemap) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
