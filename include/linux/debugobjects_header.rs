/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/debugobjects.h.  The declarations below depend on
// the corresponding list definitions supplied by the surrounding kernel.

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum debug_obj_state {
    ODEBUG_STATE_NONE,
    ODEBUG_STATE_INIT,
    ODEBUG_STATE_INACTIVE,
    ODEBUG_STATE_ACTIVE,
    ODEBUG_STATE_DESTROYED,
    ODEBUG_STATE_NOTAVAILABLE,
    ODEBUG_STATE_MAX,
}

#[repr(C)]
pub struct debug_obj_descr;

/**
 * struct debug_obj - representation of an tracked object
 * @node: hlist node to link the object into the tracker list
 * @state: tracked object state
 * @astate: current active state
 * @object: pointer to the real object
 * @batch_last: pointer to the last hlist node in a batch
 * @descr: pointer to an object type specific debug description structure
 */
#[repr(C)]
pub struct debug_obj {
    pub node: hlist_node,
    pub state: debug_obj_state,
    pub astate: ::core::ffi::c_uint,
    pub object: debug_obj_object,
    pub descr: *const debug_obj_descr,
}

#[repr(C)]
pub union debug_obj_object {
    pub object: *mut ::core::ffi::c_void,
    pub batch_last: *mut hlist_node,
}

/**
 * struct debug_obj_descr - object type specific debug description structure
 *
 * @name: name of the object typee
 * @debug_hint: function returning address, which have associated kernel symbol
 * @is_static_object: return true if the obj is static, otherwise return false
 * @fixup_init: fixup function called when the init check fails
 * @fixup_activate: fixup function called when the activate check fails
 * @fixup_destroy: fixup function called when the destroy check fails
 * @fixup_free: fixup function called when the free check fails
 * @fixup_assert_init: fixup function called when the assert_init check fails
 */
#[repr(C)]
pub struct debug_obj_descr {
    pub name: *const ::core::ffi::c_char,
    pub debug_hint: Option<unsafe extern "C" fn(addr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
    pub is_static_object: Option<unsafe extern "C" fn(addr: *mut ::core::ffi::c_void) -> bool>,
    pub fixup_init: Option<unsafe extern "C" fn(addr: *mut ::core::ffi::c_void, state: debug_obj_state) -> bool>,
    pub fixup_activate: Option<unsafe extern "C" fn(addr: *mut ::core::ffi::c_void, state: debug_obj_state) -> bool>,
    pub fixup_destroy: Option<unsafe extern "C" fn(addr: *mut ::core::ffi::c_void, state: debug_obj_state) -> bool>,
    pub fixup_free: Option<unsafe extern "C" fn(addr: *mut ::core::ffi::c_void, state: debug_obj_state) -> bool>,
    pub fixup_assert_init: Option<unsafe extern "C" fn(addr: *mut ::core::ffi::c_void, state: debug_obj_state) -> bool>,
}

#[repr(C)]
pub struct hlist_node;

#[cfg(CONFIG_DEBUG_OBJECTS)]
extern "C" {
    pub fn debug_object_init(addr: *mut ::core::ffi::c_void, descr: *const debug_obj_descr);
    pub fn debug_object_init_on_stack(addr: *mut ::core::ffi::c_void, descr: *const debug_obj_descr);
    pub fn debug_object_activate(addr: *mut ::core::ffi::c_void, descr: *const debug_obj_descr) -> ::core::ffi::c_int;
    pub fn debug_object_deactivate(addr: *mut ::core::ffi::c_void, descr: *const debug_obj_descr);
    pub fn debug_object_destroy(addr: *mut ::core::ffi::c_void, descr: *const debug_obj_descr);
    pub fn debug_object_free(addr: *mut ::core::ffi::c_void, descr: *const debug_obj_descr);
    pub fn debug_object_assert_init(addr: *mut ::core::ffi::c_void, descr: *const debug_obj_descr);
    pub fn debug_object_active_state(addr: *mut ::core::ffi::c_void, descr: *const debug_obj_descr, expect: ::core::ffi::c_uint, next: ::core::ffi::c_uint);
    pub fn debug_objects_early_init();
    pub fn debug_objects_mem_init();
}

#[cfg(not(CONFIG_DEBUG_OBJECTS))]
pub unsafe fn debug_object_init(_: *mut ::core::ffi::c_void, _: *const debug_obj_descr) {}
#[cfg(not(CONFIG_DEBUG_OBJECTS))]
pub unsafe fn debug_object_init_on_stack(_: *mut ::core::ffi::c_void, _: *const debug_obj_descr) {}
#[cfg(not(CONFIG_DEBUG_OBJECTS))]
pub unsafe fn debug_object_activate(_: *mut ::core::ffi::c_void, _: *const debug_obj_descr) -> ::core::ffi::c_int { 0 }
#[cfg(not(CONFIG_DEBUG_OBJECTS))]
pub unsafe fn debug_object_deactivate(_: *mut ::core::ffi::c_void, _: *const debug_obj_descr) {}
#[cfg(not(CONFIG_DEBUG_OBJECTS))]
pub unsafe fn debug_object_destroy(_: *mut ::core::ffi::c_void, _: *const debug_obj_descr) {}
#[cfg(not(CONFIG_DEBUG_OBJECTS))]
pub unsafe fn debug_object_free(_: *mut ::core::ffi::c_void, _: *const debug_obj_descr) {}
#[cfg(not(CONFIG_DEBUG_OBJECTS))]
pub unsafe fn debug_object_assert_init(_: *mut ::core::ffi::c_void, _: *const debug_obj_descr) {}
#[cfg(not(CONFIG_DEBUG_OBJECTS))]
pub unsafe fn debug_objects_early_init() {}
#[cfg(not(CONFIG_DEBUG_OBJECTS))]
pub unsafe fn debug_objects_mem_init() {}

#[cfg(CONFIG_DEBUG_OBJECTS_FREE)]
extern "C" {
    pub fn debug_check_no_obj_freed(address: *const ::core::ffi::c_void, size: ::core::ffi::c_ulong);
}

#[cfg(not(CONFIG_DEBUG_OBJECTS_FREE))]
pub unsafe fn debug_check_no_obj_freed(_: *const ::core::ffi::c_void, _: ::core::ffi::c_ulong) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
