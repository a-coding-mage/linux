/* SPDX-License-Identifier: MIT */
/* Copyright © 2025 Intel Corporation x*/

// Translated from display_parent_interface.h. Dependencies are supplied by the
// surrounding kernel bindings.

#[allow(non_camel_case_types)]
pub enum vlv_iosf_sb_unit {}

macro_rules! opaque_structs { ($($name:ident),* $(,)?) => { $(pub enum $name {})* }; }
opaque_structs!(dma_fence, drm_device, drm_file, drm_framebuffer, drm_gem_object,
    drm_mode_fb_cmd2, drm_plane_state, drm_scanout_buffer, fb_info, i915_gtt_view,
    i915_vma, intel_dpt, intel_dsb_buffer, intel_frontbuffer,
    intel_hdcp_gsc_context, intel_initial_plane_config, intel_panic,
    intel_stolen_node, iosys_map, ref_tracker, seq_file, vm_area_struct);

#[repr(C)]
pub struct intel_fb_pin_params {
    pub view: *const i915_gtt_view,
    pub alignment: core::ffi::c_uint,
    pub phys_alignment: core::ffi::c_uint,
    pub vtd_guard: core::ffi::c_uint,
    pub needs_cpu_lmem_access: bool,
    pub needs_low_address: bool,
    pub needs_physical: bool,
    pub needs_fence: bool,
}

type CFn<A, R> = Option<unsafe extern "C" fn(A) -> R>;

#[repr(C)]
pub struct intel_display_bo_interface {
    pub is_tiled: CFn<(*mut drm_gem_object,), bool>,
    pub is_userptr: CFn<(*mut drm_gem_object,), bool>,
    pub is_shmem: CFn<(*mut drm_gem_object,), bool>,
    pub is_protected: CFn<(*mut drm_gem_object,), bool>,
    pub key_check: CFn<(*mut drm_gem_object,), core::ffi::c_int>,
    pub fb_mmap: CFn<(*mut drm_gem_object, *mut vm_area_struct), core::ffi::c_int>,
    pub read_from_page: CFn<(*mut drm_gem_object, u64, *mut core::ffi::c_void, core::ffi::c_int), core::ffi::c_int>,
    pub describe: CFn<(*mut seq_file, *mut drm_gem_object), ()>,
    pub framebuffer_init: CFn<(*mut drm_gem_object, *mut drm_mode_fb_cmd2), core::ffi::c_int>,
    pub framebuffer_fini: CFn<(*mut drm_gem_object,), ()>,
    pub framebuffer_lookup: CFn<(*mut drm_device, *mut drm_file, *const drm_mode_fb_cmd2), *mut drm_gem_object>,
    #[cfg(feature = "CONFIG_DRM_FBDEV_EMULATION")]
    pub fbdev_create: CFn<(*mut drm_device, core::ffi::c_int), *mut drm_gem_object>,
    #[cfg(feature = "CONFIG_DRM_FBDEV_EMULATION")]
    pub fbdev_destroy: CFn<(*mut drm_gem_object,), ()>,
    #[cfg(feature = "CONFIG_DRM_FBDEV_EMULATION")]
    pub fbdev_fill_info: CFn<(*mut drm_gem_object, *mut fb_info, *mut i915_vma), core::ffi::c_int>,
    #[cfg(feature = "CONFIG_DRM_FBDEV_EMULATION")]
    pub fbdev_pitch_align: CFn<(u32,), u32>,
}

#[repr(C)] pub struct intel_display_dpt_interface { pub create: CFn<(*mut drm_gem_object, usize), *mut intel_dpt>, pub destroy: CFn<(*mut intel_dpt,), ()>, pub suspend: CFn<(*mut intel_dpt,), ()>, pub resume: CFn<(*mut intel_dpt,), ()> }
#[repr(C)] pub struct intel_display_dsb_interface { pub ggtt_offset: CFn<(*mut intel_dsb_buffer,), u32>, pub write: CFn<(*mut intel_dsb_buffer, u32, u32), ()>, pub read: CFn<(*mut intel_dsb_buffer, u32), u32>, pub fill: CFn<(*mut intel_dsb_buffer, u32, u32, usize), ()>, pub create: CFn<(*mut drm_device, usize), *mut intel_dsb_buffer>, pub cleanup: CFn<(*mut intel_dsb_buffer,), ()>, pub flush_map: CFn<(*mut intel_dsb_buffer,), ()> }

#[repr(C)]
pub struct intel_display_fb_pin_interface {
    pub ggtt_pin: CFn<(*mut drm_gem_object, *const intel_fb_pin_params, *mut *mut i915_vma, *mut u32, *mut core::ffi::c_int), core::ffi::c_int>,
    pub ggtt_unpin: CFn<(*mut i915_vma, core::ffi::c_int), ()>,
    pub dpt_pin: CFn<(*mut drm_gem_object, *mut intel_dpt, *const intel_fb_pin_params, *mut *mut i915_vma, *mut *mut i915_vma, *mut u32), core::ffi::c_int>,
    pub dpt_unpin: CFn<(*mut intel_dpt, *mut i915_vma, *mut i915_vma), ()>,
    pub reuse_vma: CFn<(*mut i915_vma, *mut drm_gem_object, *const i915_gtt_view, *mut drm_gem_object, *const i915_gtt_view, *mut u32), *mut i915_vma>,
    pub get_map: CFn<(*mut i915_vma, *mut iosys_map), ()>,
}

#[repr(C)] pub struct intel_display_frontbuffer_interface { pub get: CFn<(*mut drm_gem_object,), *mut intel_frontbuffer>, pub ref_: CFn<(*mut intel_frontbuffer,), ()>, pub put: CFn<(*mut intel_frontbuffer,), ()>, pub flush_for_display: CFn<(*mut intel_frontbuffer,), ()> }
#[repr(C)] pub struct intel_display_hdcp_interface { pub gsc_msg_send: CFn<(*mut intel_hdcp_gsc_context, *mut core::ffi::c_void, usize, *mut core::ffi::c_void, usize), isize>, pub gsc_check_status: CFn<(*mut drm_device,), bool>, pub gsc_context_alloc: CFn<(*mut drm_device,), *mut intel_hdcp_gsc_context>, pub gsc_context_free: CFn<(*mut intel_hdcp_gsc_context,), ()> }
#[repr(C)] pub struct intel_display_initial_plane_interface { pub alloc_obj: CFn<(*mut drm_device, *mut intel_initial_plane_config), *mut drm_gem_object>, pub setup: CFn<(*mut drm_plane_state, *mut intel_initial_plane_config, *mut drm_framebuffer, *mut i915_vma), core::ffi::c_int>, pub config_fini: CFn<(*mut intel_initial_plane_config,), ()> }
#[repr(C)] pub struct intel_display_irq_interface { pub enabled: CFn<(*mut drm_device,), bool>, pub synchronize: CFn<(*mut drm_device,), ()> }
#[repr(C)] pub struct intel_display_pc8_interface { pub block: CFn<(*mut drm_device,), ()>, pub unblock: CFn<(*mut drm_device,), ()> }
#[repr(C)] pub struct intel_display_pcode_interface { pub read: CFn<(*mut drm_device, u32, *mut u32, *mut u32), core::ffi::c_int>, pub write: CFn<(*mut drm_device, u32, u32, core::ffi::c_int), core::ffi::c_int>, pub request: CFn<(*mut drm_device, u32, u32, u32, u32, core::ffi::c_int), core::ffi::c_int> }
#[repr(C)] pub struct intel_display_rps_interface { pub boost_if_not_started: CFn<(*mut dma_fence,), ()>, pub mark_interactive: CFn<(*mut drm_device, bool), ()>, pub ilk_irq_handler: CFn<(*mut drm_device,), ()> }
#[repr(C)] pub struct intel_display_vlv_iosf_interface { pub get: CFn<(*mut drm_device, core::ffi::c_ulong), ()>, pub put: CFn<(*mut drm_device, core::ffi::c_ulong), ()>, pub read: CFn<(*mut drm_device, vlv_iosf_sb_unit, u32), u32>, pub write: CFn<(*mut drm_device, vlv_iosf_sb_unit, u32, u32), core::ffi::c_int> }

#[repr(C)] pub struct intel_display_panic_interface { pub alloc: CFn<(), *mut intel_panic>, pub setup: CFn<(*mut intel_panic, *mut drm_scanout_buffer, *mut drm_gem_object, Option<unsafe extern "C" fn(u32, u32, u32, u32) -> core::ffi::c_uint>), core::ffi::c_int>, pub finish: CFn<(*mut intel_panic,), ()> }
#[repr(C)] pub struct intel_display_overlay_interface {
    pub is_active: CFn<(*mut drm_device,), bool>,
    pub overlay_on: CFn<(*mut drm_device, u32), core::ffi::c_int>,
    pub overlay_continue: CFn<(*mut drm_device, *mut i915_vma, bool), core::ffi::c_int>,
    pub overlay_off: CFn<(*mut drm_device,), core::ffi::c_int>,
    pub recover_from_interrupt: CFn<(*mut drm_device,), core::ffi::c_int>,
    pub release_old_vid: CFn<(*mut drm_device,), core::ffi::c_int>,
    pub reset: CFn<(*mut drm_device,), ()>,
    pub pin_fb: CFn<(*mut drm_device, *mut drm_gem_object, *mut u32), *mut i915_vma>,
    pub unpin_fb: CFn<(*mut drm_device, *mut i915_vma), ()>,
    pub obj_lookup: CFn<(*mut drm_device, *mut drm_file, u32), *mut drm_gem_object>,
    pub setup: CFn<(*mut drm_device, bool), *mut core::ffi::c_void>,
    pub cleanup: CFn<(*mut drm_device,), ()>,
}
#[repr(C)] pub struct intel_display_stolen_interface { pub insert_node_in_range: CFn<(*mut intel_stolen_node, u64, core::ffi::c_uint, u64, u64), core::ffi::c_int>, pub insert_node: CFn<(*mut intel_stolen_node, u64, core::ffi::c_uint), core::ffi::c_int>, pub remove_node: CFn<(*mut intel_stolen_node,), ()>, pub initialized: CFn<(*mut drm_device,), bool>, pub node_allocated: CFn<(*const intel_stolen_node,), bool>, pub node_offset: CFn<(*const intel_stolen_node,), u64>, pub area_address: CFn<(*mut drm_device,), u64>, pub area_size: CFn<(*mut drm_device,), u64>, pub node_address: CFn<(*const intel_stolen_node,), u64>, pub node_size: CFn<(*const intel_stolen_node,), u64>, pub node_alloc: CFn<(*mut drm_device,), *mut intel_stolen_node>, pub node_free: CFn<(*const intel_stolen_node,), ()> }

#[repr(C)] pub struct intel_display_parent_interface {
    pub bo: *const intel_display_bo_interface,
    pub dpt: *const intel_display_dpt_interface,
    pub dsb: *const intel_display_dsb_interface,
    pub fb_pin: *const intel_display_fb_pin_interface,
    pub frontbuffer: *const intel_display_frontbuffer_interface,
    pub hdcp: *const intel_display_hdcp_interface,
    pub initial_plane: *const intel_display_initial_plane_interface,
    pub irq: *const intel_display_irq_interface,
    pub panic: *const intel_display_panic_interface,
    pub overlay: *const intel_display_overlay_interface,
    pub pc8: *const intel_display_pc8_interface,
    pub pcode: *const intel_display_pcode_interface,
    pub rpm: *const intel_display_rpm_interface,
    pub rps: *const intel_display_rps_interface,
    pub stolen: *const intel_display_stolen_interface,
    pub vlv_iosf: *const intel_display_vlv_iosf_interface,
    pub generic: intel_display_generic_interface,
}

#[repr(C)] pub struct intel_display_rpm_interface { pub get: CFn<(*const drm_device,), *mut ref_tracker>, pub get_raw: CFn<(*const drm_device,), *mut ref_tracker>, pub get_if_in_use: CFn<(*const drm_device,), *mut ref_tracker>, pub get_noresume: CFn<(*const drm_device,), *mut ref_tracker>, pub put: CFn<(*const drm_device, *mut ref_tracker), ()>, pub put_raw: CFn<(*const drm_device, *mut ref_tracker), ()>, pub put_unchecked: CFn<(*const drm_device,), ()>, pub suspended: CFn<(*const drm_device,), bool>, pub assert_held: CFn<(*const drm_device,), ()>, pub assert_block: CFn<(*const drm_device,), ()>, pub assert_unblock: CFn<(*const drm_device,), ()> }
#[repr(C)] pub struct intel_display_generic_interface { pub fence_priority_display: CFn<(*mut dma_fence,), ()>, pub has_auxccs: CFn<(*mut drm_device,), bool>, pub has_fenced_regions: CFn<(*mut drm_device,), bool>, pub vgpu_active: CFn<(*mut drm_device,), bool> }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
