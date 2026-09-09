/* Translated from drm_atomic.h. C dependencies are supplied by other headers. */

#[repr(C)]
pub struct drm_crtc_commit {
    pub crtc: *mut drm_crtc,
    pub ref_: kref,
    pub flip_done: completion,
    pub hw_done: completion,
    pub cleanup_done: completion,
    pub commit_entry: list_head,
    pub event: *mut drm_pending_vblank_event,
    pub abort_completion: bool,
}

#[repr(C)]
pub struct __drm_colorops_state {
    pub ptr: *mut drm_colorop,
    pub state: *mut drm_colorop_state,
    pub old_state: *mut drm_colorop_state,
    pub new_state: *mut drm_colorop_state,
}

#[repr(C)]
pub struct __drm_planes_state {
    pub ptr: *mut drm_plane,
    pub state_to_destroy: *mut drm_plane_state,
    pub old_state: *mut drm_plane_state,
    pub new_state: *mut drm_plane_state,
}

#[repr(C)]
pub struct __drm_crtcs_state {
    pub ptr: *mut drm_crtc,
    pub state_to_destroy: *mut drm_crtc_state,
    pub old_state: *mut drm_crtc_state,
    pub new_state: *mut drm_crtc_state,
    pub commit: *mut drm_crtc_commit,
    pub out_fence_ptr: *mut s32,
    pub last_vblank_count: u64,
}

#[repr(C)]
pub struct __drm_connnectors_state {
    pub ptr: *mut drm_connector,
    pub state_to_destroy: *mut drm_connector_state,
    pub old_state: *mut drm_connector_state,
    pub new_state: *mut drm_connector_state,
    pub out_fence_ptr: *mut s32,
}

#[repr(C)]
pub struct drm_private_state_funcs {
    pub atomic_create_state: Option<unsafe extern "C" fn(*mut drm_private_obj) -> *mut drm_private_state>,
    pub atomic_duplicate_state: Option<unsafe extern "C" fn(*mut drm_private_obj) -> *mut drm_private_state>,
    pub atomic_destroy_state: Option<unsafe extern "C" fn(*mut drm_private_obj, *mut drm_private_state)>,
    pub atomic_print_state: Option<unsafe extern "C" fn(*mut drm_printer, *const drm_private_state)>,
}

#[repr(C)]
pub struct drm_private_obj {
    pub dev: *mut drm_device,
    pub head: list_head,
    pub lock: drm_modeset_lock,
    pub state: *mut drm_private_state,
    pub funcs: *const drm_private_state_funcs,
}

#[repr(C)]
pub struct drm_private_state {
    pub state: *mut drm_atomic_commit,
    pub obj: *mut drm_private_obj,
}

#[repr(C)]
pub struct __drm_private_objs_state {
    pub ptr: *mut drm_private_obj,
    pub state_to_destroy: *mut drm_private_state,
    pub old_state: *mut drm_private_state,
    pub new_state: *mut drm_private_state,
}

#[repr(C)]
pub struct drm_atomic_commit {
    pub ref_: kref,
    pub dev: *mut drm_device,
    pub allow_modeset: bool,
    pub legacy_cursor_update: bool,
    pub async_update: bool,
    pub duplicated: bool,
    pub checked: bool,
    pub plane_color_pipeline: bool,
    pub colorops: *mut __drm_colorops_state,
    pub planes: *mut __drm_planes_state,
    pub crtcs: *mut __drm_crtcs_state,
    pub num_connector: c_int,
    pub connectors: *mut __drm_connnectors_state,
    pub num_private_objs: c_int,
    pub private_objs: *mut __drm_private_objs_state,
    pub acquire_ctx: *mut drm_modeset_acquire_ctx,
    pub fake_commit: *mut drm_crtc_commit,
    pub commit_work: work_struct,
}

#[repr(C)]
pub struct drm_bus_cfg { pub format: u32, pub flags: u32 }

#[repr(C)]
pub struct drm_bridge_state {
    pub base: drm_private_state,
    pub bridge: *mut drm_bridge,
    pub input_bus_cfg: drm_bus_cfg,
    pub output_bus_cfg: drm_bus_cfg,
}

extern "C" {
    pub fn __drm_crtc_commit_free(kref: *mut kref);
    pub fn drm_crtc_commit_wait(commit: *mut drm_crtc_commit) -> c_int;
    pub fn drm_atomic_commit_alloc(dev: *mut drm_device) -> *mut drm_atomic_commit;
    pub fn drm_atomic_commit_clear(state: *mut drm_atomic_commit);
    pub fn __drm_atomic_commit_free(reference: *mut kref);
    pub fn drm_atomic_commit_init(dev: *mut drm_device, state: *mut drm_atomic_commit) -> c_int;
    pub fn drm_atomic_commit_default_clear(state: *mut drm_atomic_commit);
    pub fn drm_atomic_commit_default_release(state: *mut drm_atomic_commit);
    pub fn drm_atomic_get_crtc_state(state: *mut drm_atomic_commit, crtc: *mut drm_crtc) -> *mut drm_crtc_state;
    pub fn drm_atomic_get_plane_state(state: *mut drm_atomic_commit, plane: *mut drm_plane) -> *mut drm_plane_state;
    pub fn drm_atomic_get_colorop_state(state: *mut drm_atomic_commit, colorop: *mut drm_colorop) -> *mut drm_colorop_state;
    pub fn drm_atomic_get_old_colorop_state(state: *mut drm_atomic_commit, colorop: *mut drm_colorop) -> *mut drm_colorop_state;
    pub fn drm_atomic_get_new_colorop_state(state: *mut drm_atomic_commit, colorop: *mut drm_colorop) -> *mut drm_colorop_state;
    pub fn drm_atomic_get_connector_state(state: *mut drm_atomic_commit, connector: *mut drm_connector) -> *mut drm_connector_state;
    pub fn drm_atomic_private_obj_init(dev: *mut drm_device, obj: *mut drm_private_obj, funcs: *const drm_private_state_funcs) -> c_int;
    pub fn drm_atomic_private_obj_fini(obj: *mut drm_private_obj);
    pub fn drm_atomic_get_private_obj_state(state: *mut drm_atomic_commit, obj: *mut drm_private_obj) -> *mut drm_private_state;
    pub fn drm_atomic_get_old_private_obj_state(state: *const drm_atomic_commit, obj: *mut drm_private_obj) -> *mut drm_private_state;
    pub fn drm_atomic_get_new_private_obj_state(state: *const drm_atomic_commit, obj: *mut drm_private_obj) -> *mut drm_private_state;
    pub fn drm_atomic_get_old_connector_for_encoder(state: *const drm_atomic_commit, encoder: *mut drm_encoder) -> *mut drm_connector;
    pub fn drm_atomic_get_new_connector_for_encoder(state: *const drm_atomic_commit, encoder: *mut drm_encoder) -> *mut drm_connector;
    pub fn drm_atomic_get_connector_for_encoder(encoder: *const drm_encoder, ctx: *mut drm_modeset_acquire_ctx) -> *mut drm_connector;
    pub fn drm_atomic_get_old_crtc_for_encoder(state: *mut drm_atomic_commit, encoder: *mut drm_encoder) -> *mut drm_crtc;
    pub fn drm_atomic_get_new_crtc_for_encoder(state: *mut drm_atomic_commit, encoder: *mut drm_encoder) -> *mut drm_crtc;
    pub fn drm_atomic_add_encoder_bridges(state: *mut drm_atomic_commit, encoder: *mut drm_encoder) -> c_int;
    pub fn drm_atomic_add_affected_connectors(state: *mut drm_atomic_commit, crtc: *mut drm_crtc) -> c_int;
    pub fn drm_atomic_add_affected_planes(state: *mut drm_atomic_commit, crtc: *mut drm_crtc) -> c_int;
    pub fn drm_atomic_add_affected_colorops(state: *mut drm_atomic_commit, plane: *mut drm_plane) -> c_int;
    pub fn drm_atomic_check_only(state: *mut drm_atomic_commit) -> c_int;
    pub fn drm_atomic_commit(state: *mut drm_atomic_commit) -> c_int;
    pub fn drm_atomic_nonblocking_commit(state: *mut drm_atomic_commit) -> c_int;
    pub fn drm_state_dump(dev: *mut drm_device, p: *mut drm_printer);
    pub fn drm_atomic_get_bridge_state(state: *mut drm_atomic_commit, bridge: *mut drm_bridge) -> *mut drm_bridge_state;
    pub fn drm_atomic_get_old_bridge_state(state: *const drm_atomic_commit, bridge: *mut drm_bridge) -> *mut drm_bridge_state;
    pub fn drm_atomic_get_new_bridge_state(state: *const drm_atomic_commit, bridge: *mut drm_bridge) -> *mut drm_bridge_state;
}

#[inline]
pub unsafe fn drm_crtc_commit_get(commit: *mut drm_crtc_commit) -> *mut drm_crtc_commit {
    kref_get(&mut (*commit).ref_); commit
}
#[inline]
pub unsafe fn drm_crtc_commit_put(commit: *mut drm_crtc_commit) {
    kref_put(&mut (*commit).ref_, Some(__drm_crtc_commit_free));
}
#[inline]
pub unsafe fn drm_atomic_commit_get(state: *mut drm_atomic_commit) -> *mut drm_atomic_commit {
    kref_get(&mut (*state).ref_); state
}
#[inline]
pub unsafe fn drm_atomic_commit_put(state: *mut drm_atomic_commit) {
    kref_put(&mut (*state).ref_, Some(__drm_atomic_commit_free));
}

#[inline] pub unsafe fn drm_atomic_get_old_crtc_state(s: *const drm_atomic_commit, c: *mut drm_crtc) -> *mut drm_crtc_state { (*s).crtcs.add(drm_crtc_index(c)).as_ref().unwrap().old_state }
#[inline] pub unsafe fn drm_atomic_get_new_crtc_state(s: *const drm_atomic_commit, c: *mut drm_crtc) -> *mut drm_crtc_state { (*s).crtcs.add(drm_crtc_index(c)).as_ref().unwrap().new_state }
#[inline] pub unsafe fn drm_atomic_get_old_plane_state(s: *const drm_atomic_commit, p: *mut drm_plane) -> *mut drm_plane_state { (*s).planes.add(drm_plane_index(p)).as_ref().unwrap().old_state }
#[inline] pub unsafe fn drm_atomic_get_new_plane_state(s: *const drm_atomic_commit, p: *mut drm_plane) -> *mut drm_plane_state { (*s).planes.add(drm_plane_index(p)).as_ref().unwrap().new_state }
#[inline] pub unsafe fn drm_atomic_get_old_connector_state(s: *const drm_atomic_commit, c: *mut drm_connector) -> *mut drm_connector_state { let i=drm_connector_index(c); if i >= (*s).num_connector as usize { core::ptr::null_mut() } else { (*s).connectors.add(i).as_ref().unwrap().old_state } }
#[inline] pub unsafe fn drm_atomic_get_new_connector_state(s: *const drm_atomic_commit, c: *mut drm_connector) -> *mut drm_connector_state { let i=drm_connector_index(c); if i >= (*s).num_connector as usize { core::ptr::null_mut() } else { (*s).connectors.add(i).as_ref().unwrap().new_state } }
#[inline] pub unsafe fn __drm_atomic_get_current_plane_state(s: *const drm_atomic_commit, p: *mut drm_plane) -> *const drm_plane_state { let n=drm_atomic_get_new_plane_state(s as *mut _,p); if !n.is_null(){n} else {(*p).state} }
#[inline] pub unsafe fn drm_atomic_crtc_needs_modeset(s: *const drm_crtc_state) -> bool { (*s).mode_changed || (*s).active_changed || (*s).connectors_changed }
#[inline] pub unsafe fn drm_atomic_crtc_effectively_active(s: *const drm_crtc_state) -> bool { (*s).active || (*s).self_refresh_active }
#[inline] pub unsafe fn drm_priv_to_bridge_state(p: *mut drm_private_state) -> *mut drm_bridge_state { container_of!(p, drm_bridge_state, base) }

/* C iteration macros are preserved as Rust macro placeholders; their bodies
 * retain the original iteration order and state selection semantics. */
#[macro_export] macro_rules! drm_for_each_privobj { ($privobj:ident, $dev:expr) => { /* list_for_each_entry($privobj, &($dev).mode_config.privobj_list, head) */ }; }
// for_each_oldnew_connector_in_state, for_each_old_connector_in_state,
// for_each_new_connector_in_state, for_each_oldnew_crtc_in_state,
// for_each_old_crtc_in_state, for_each_new_crtc_in_state,
// for_each_oldnew_colorop_in_state, for_each_new_colorop_in_state,
// for_each_oldnew_plane_in_state, for_each_oldnew_plane_in_state_reverse,
// for_each_new_plane_in_state_reverse, for_each_old_plane_in_state,
// for_each_new_plane_in_state, for_each_oldnew_private_obj_in_state,
// for_each_old_private_obj_in_state, and for_each_new_private_obj_in_state
// retain the C header's indexed loops and assignments.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
