/*
 * Copyright © 2016 Intel Corporation
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

// Dependency declarations corresponding to the original Linux includes are
// supplied by the surrounding translation unit.

pub struct platform_device;

pub const HDMI_MAX_ELD_BYTES: usize = 128;

#[repr(C)]
pub struct intel_hdmi_lpe_audio_port_pdata {
    pub eld: [u8; HDMI_MAX_ELD_BYTES],
    pub port: i32,
    pub pipe: i32,
    pub ls_clock: i32,
    pub dp_output: bool,
}

#[repr(C)]
pub struct intel_hdmi_lpe_audio_pdata {
    /// For ports B, C, D.
    pub port: [intel_hdmi_lpe_audio_port_pdata; 3],
    pub num_ports: i32,
    pub num_pipes: i32,

    /// Port: 0 == B, 1 == C, 2 == D.
    pub notify_audio_lpe:
        Option<unsafe extern "C" fn(pdev: *mut platform_device, port: i32)>,
    pub lpe_audio_slock: spinlock_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
