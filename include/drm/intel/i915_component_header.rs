/*
 * Copyright © 2014 Intel Corporation
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 */

// Dependency supplied by drm/drm_audio_component.h.

#[repr(C)]
pub enum i915_component_type {
    I915_COMPONENT_AUDIO = 1,
    I915_COMPONENT_HDCP,
    I915_COMPONENT_PXP,
    I915_COMPONENT_GSC_PROXY,
    INTEL_COMPONENT_LB,
}

/* MAX_PORT is the number of port
 * It must be sync with I915_MAX_PORTS defined i915_drv.h
 */
pub const MAX_PORTS: usize = 9;

/**
 * struct i915_audio_component - Used for direct communication between i915 and hda drivers
 */
#[repr(C)]
pub struct i915_audio_component {
    /**
     * @base: the drm_audio_component base class
     */
    pub base: drm_audio_component,

    /**
     * @aud_sample_rate: the array of audio sample rate per port
     */
    pub aud_sample_rate: [::core::ffi::c_int; MAX_PORTS],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
