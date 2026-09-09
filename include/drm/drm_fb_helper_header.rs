/*
 * Copyright (c) 2006-2009 Red Hat Inc.
 * Copyright (c) 2006-2008 Intel Corporation
 * Copyright (c) 2007 Dave Airlie <airlied@linux.ie>
 *
 * DRM framebuffer helper functions
 *
 * Permission to use, copy, modify, distribute, and sell this software and its
 * documentation for any purpose is hereby granted without fee, provided that
 * the above copyright notice appear in all copies and that both that copyright
 * notice and this permission notice appear in supporting documentation, and
 * that the name of the copyright holders not be used in advertising or
 * publicity pertaining to distribution of the software without specific,
 * written prior permission.  The copyright holders make no representations
 * about the suitability of this software for any purpose.  It is provided "as
 * is" without express or implied warranty.
 *
 * THE COPYRIGHT HOLDERS DISCLAIM ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE FOR ANY SPECIAL, INDIRECT,
 * OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
 * OF THIS SOFTWARE.
 */

// C dependencies: linux/fb.h and drm/drm_client.h.

#[repr(C)]
pub struct drm_fb_helper_surface_size {
    pub fb_width: u32,
    pub fb_height: u32,
    pub surface_width: u32,
    pub surface_height: u32,
    pub surface_bpp: u32,
    pub surface_depth: u32,
}

#[repr(C)]
pub struct drm_fb_helper_funcs {
    pub fb_dirty: Option<unsafe extern "C" fn(*mut drm_fb_helper, *mut drm_clip_rect) -> i32>,
    pub fb_restore: Option<unsafe extern "C" fn(*mut drm_fb_helper)>,
    pub fb_set_suspend: Option<unsafe extern "C" fn(*mut drm_fb_helper, bool)>,
}

#[repr(C)]
pub struct drm_fb_helper {
    pub client: drm_client_dev,
    pub buffer: *mut drm_client_buffer,
    pub fb: *mut drm_framebuffer,
    pub dev: *mut drm_device,
    pub funcs: *const drm_fb_helper_funcs,
    pub info: *mut fb_info,
    pub pseudo_palette: [u32; 17],
    pub damage_clip: drm_clip_rect,
    pub damage_lock: spinlock_t,
    pub damage_work: work_struct,
    pub resume_work: work_struct,
    pub lock: mutex,
    pub delayed_hotplug: bool,
    pub deferred_setup: bool,
    pub preferred_bpp: i32,
    #[cfg(CONFIG_FB_DEFERRED_IO)]
    pub fbdefio: fb_deferred_io,
}

#[inline]
pub unsafe fn drm_fb_helper_from_client(client: *mut drm_client_dev) -> *mut drm_fb_helper {
    container_of!(client, drm_fb_helper, client)
}

// DRM_FB_HELPER_DEFAULT_OPS expands to the default fb_ops initializers:
// fb_check_var = drm_fb_helper_check_var, fb_set_par = drm_fb_helper_set_par,
// fb_setcmap = drm_fb_helper_setcmap, fb_blank = drm_fb_helper_blank,
// fb_pan_display = drm_fb_helper_pan_display, fb_ioctl = drm_fb_helper_ioctl.

#[cfg(CONFIG_DRM_FBDEV_EMULATION)]
extern "C" {
    pub fn drm_fb_helper_prepare(dev: *mut drm_device, helper: *mut drm_fb_helper,
                                  preferred_bpp: c_uint,
                                  funcs: *const drm_fb_helper_funcs);
    pub fn drm_fb_helper_unprepare(fb_helper: *mut drm_fb_helper);
    pub fn drm_fb_helper_init(dev: *mut drm_device, helper: *mut drm_fb_helper) -> i32;
    pub fn drm_fb_helper_fini(helper: *mut drm_fb_helper);
    pub fn drm_fb_helper_blank(blank: i32, info: *mut fb_info) -> i32;
    pub fn drm_fb_helper_pan_display(var: *mut fb_var_screeninfo, info: *mut fb_info) -> i32;
    pub fn drm_fb_helper_set_par(info: *mut fb_info) -> i32;
    pub fn drm_fb_helper_check_var(var: *mut fb_var_screeninfo, info: *mut fb_info) -> i32;
    pub fn drm_fb_helper_restore_fbdev_mode_unlocked(helper: *mut drm_fb_helper, force: bool) -> i32;
    pub fn drm_fb_helper_unregister_info(helper: *mut drm_fb_helper);
    pub fn drm_fb_helper_fill_info(info: *mut fb_info, helper: *mut drm_fb_helper,
                                   sizes: *mut drm_fb_helper_surface_size);
    pub fn drm_fb_helper_damage_range(info: *mut fb_info, off: off_t, len: size_t);
    pub fn drm_fb_helper_damage_area(info: *mut fb_info, x: u32, y: u32, width: u32, height: u32);
    #[cfg(CONFIG_FB_DEFERRED_IO)]
    pub fn drm_fb_helper_deferred_io(info: *mut fb_info, pagereflist: *mut list_head);
    pub fn drm_fb_helper_set_suspend(helper: *mut drm_fb_helper, suspend: bool);
    pub fn drm_fb_helper_set_suspend_unlocked(helper: *mut drm_fb_helper, suspend: bool);
    pub fn drm_fb_helper_setcmap(cmap: *mut fb_cmap, info: *mut fb_info) -> i32;
    pub fn drm_fb_helper_ioctl(info: *mut fb_info, cmd: c_uint, arg: c_ulong) -> i32;
    pub fn drm_fb_helper_hotplug_event(helper: *mut drm_fb_helper) -> i32;
    pub fn drm_fb_helper_initial_config(helper: *mut drm_fb_helper) -> i32;
    pub fn drm_fb_helper_gem_is_fb(helper: *const drm_fb_helper, obj: *const drm_gem_object) -> bool;
}

#[cfg(not(CONFIG_DRM_FBDEV_EMULATION))]
#[inline]
pub unsafe fn drm_fb_helper_gem_is_fb(_helper: *const drm_fb_helper,
                                      _obj: *const drm_gem_object) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
