/*
 * Copyright 2007-8 Advanced Micro Devices, Inc.
 * Copyright 2008 Red Hat Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: Dave Airlie
 *          Alex Deucher
 */

// Dependencies supplied by the surrounding kernel translation.

pub unsafe fn amdgpu_link_encoder_connector(dev: *mut drm_device) {
    let adev = drm_to_adev(dev);
    let mut connector: *mut drm_connector;
    let mut iter: drm_connector_list_iter;
    let mut amdgpu_connector: *mut amdgpu_connector;
    let mut encoder: *mut drm_encoder;
    let mut amdgpu_encoder: *mut amdgpu_encoder;

    drm_connector_list_iter_begin(dev, &mut iter);
    /* walk the list and link encoders to connectors */
    drm_for_each_connector_iter!(connector, &mut iter, {
        amdgpu_connector = to_amdgpu_connector(connector);
        list_for_each_entry!(encoder, (*dev).mode_config.encoder_list, head, {
            amdgpu_encoder = to_amdgpu_encoder(encoder);
            if (*amdgpu_encoder).devices & (*amdgpu_connector).devices != 0 {
                drm_connector_attach_encoder(connector, encoder);
                if (*amdgpu_encoder).devices & ATOM_DEVICE_LCD_SUPPORT != 0 {
                    amdgpu_atombios_encoder_init_backlight(amdgpu_encoder, connector);
                    (*adev).mode_info.bl_encoder = amdgpu_encoder;
                }
            }
        });
    });
    drm_connector_list_iter_end(&mut iter);
}

pub unsafe fn amdgpu_encoder_set_active_device(encoder: *mut drm_encoder) {
    let dev = (*encoder).dev;
    let amdgpu_encoder = to_amdgpu_encoder(encoder);
    let mut connector: *mut drm_connector;
    let mut iter: drm_connector_list_iter;

    drm_connector_list_iter_begin(dev, &mut iter);
    drm_for_each_connector_iter!(connector, &mut iter, {
        if (*connector).encoder == encoder {
            let amdgpu_connector = to_amdgpu_connector(connector);
            (*amdgpu_encoder).active_device = (*amdgpu_encoder).devices & (*amdgpu_connector).devices;
            DRM_DEBUG_KMS!("setting active device to %08x from %08x %08x for encoder %d\n",
                (*amdgpu_encoder).active_device, (*amdgpu_encoder).devices,
                (*amdgpu_connector).devices, (*encoder).encoder_type);
        }
    });
    drm_connector_list_iter_end(&mut iter);
}

pub unsafe fn amdgpu_get_connector_for_encoder(encoder: *mut drm_encoder) -> *mut drm_connector {
    let dev = (*encoder).dev;
    let amdgpu_encoder = to_amdgpu_encoder(encoder);
    let mut connector: *mut drm_connector;
    let mut found: *mut drm_connector = core::ptr::null_mut();
    let mut iter: drm_connector_list_iter;
    let mut amdgpu_connector: *mut amdgpu_connector;

    drm_connector_list_iter_begin(dev, &mut iter);
    drm_for_each_connector_iter!(connector, &mut iter, {
        amdgpu_connector = to_amdgpu_connector(connector);
        if (*amdgpu_encoder).active_device & (*amdgpu_connector).devices != 0 {
            found = connector;
            break;
        }
    });
    drm_connector_list_iter_end(&mut iter);
    found
}

pub unsafe fn amdgpu_get_connector_for_encoder_init(encoder: *mut drm_encoder) -> *mut drm_connector {
    let dev = (*encoder).dev;
    let amdgpu_encoder = to_amdgpu_encoder(encoder);
    let mut connector: *mut drm_connector;
    let mut found: *mut drm_connector = core::ptr::null_mut();
    let mut iter: drm_connector_list_iter;
    let mut amdgpu_connector: *mut amdgpu_connector;

    drm_connector_list_iter_begin(dev, &mut iter);
    drm_for_each_connector_iter!(connector, &mut iter, {
        amdgpu_connector = to_amdgpu_connector(connector);
        if (*amdgpu_encoder).devices & (*amdgpu_connector).devices != 0 {
            found = connector;
            break;
        }
    });
    drm_connector_list_iter_end(&mut iter);
    found
}

pub unsafe fn amdgpu_get_external_encoder(encoder: *mut drm_encoder) -> *mut drm_encoder {
    let dev = (*encoder).dev;
    let amdgpu_encoder = to_amdgpu_encoder(encoder);
    let mut other_encoder: *mut drm_encoder;
    let mut other_amdgpu_encoder: *mut amdgpu_encoder;

    if (*amdgpu_encoder).is_ext_encoder {
        return core::ptr::null_mut();
    }

    list_for_each_entry!(other_encoder, (*dev).mode_config.encoder_list, head, {
        if other_encoder == encoder {
            continue;
        }
        other_amdgpu_encoder = to_amdgpu_encoder(other_encoder);
        if (*other_amdgpu_encoder).is_ext_encoder
            && (*amdgpu_encoder).devices & (*other_amdgpu_encoder).devices != 0
        {
            return other_encoder;
        }
    });
    core::ptr::null_mut()
}

pub unsafe fn amdgpu_encoder_get_dp_bridge_encoder_id(encoder: *mut drm_encoder) -> u16 {
    let other_encoder = amdgpu_get_external_encoder(encoder);

    if !other_encoder.is_null() {
        let amdgpu_encoder = to_amdgpu_encoder(other_encoder);
        match (*amdgpu_encoder).encoder_id {
            ENCODER_OBJECT_ID_TRAVIS | ENCODER_OBJECT_ID_NUTMEG => (*amdgpu_encoder).encoder_id,
            _ => ENCODER_OBJECT_ID_NONE,
        }
    } else {
        ENCODER_OBJECT_ID_NONE
    }
}

pub unsafe fn amdgpu_panel_mode_fixup(
    encoder: *mut drm_encoder,
    adjusted_mode: *mut drm_display_mode,
) {
    let amdgpu_encoder = to_amdgpu_encoder(encoder);
    let native_mode = &(*amdgpu_encoder).native_mode;
    let hblank = native_mode.htotal - native_mode.hdisplay;
    let vblank = native_mode.vtotal - native_mode.vdisplay;
    let hover = native_mode.hsync_start - native_mode.hdisplay;
    let vover = native_mode.vsync_start - native_mode.vdisplay;
    let hsync_width = native_mode.hsync_end - native_mode.hsync_start;
    let vsync_width = native_mode.vsync_end - native_mode.vsync_start;

    (*adjusted_mode).clock = native_mode.clock;
    (*adjusted_mode).flags = native_mode.flags;
    (*adjusted_mode).hdisplay = native_mode.hdisplay;
    (*adjusted_mode).vdisplay = native_mode.vdisplay;
    (*adjusted_mode).htotal = native_mode.hdisplay + hblank;
    (*adjusted_mode).hsync_start = native_mode.hdisplay + hover;
    (*adjusted_mode).hsync_end = (*adjusted_mode).hsync_start + hsync_width;
    (*adjusted_mode).vtotal = native_mode.vdisplay + vblank;
    (*adjusted_mode).vsync_start = native_mode.vdisplay + vover;
    (*adjusted_mode).vsync_end = (*adjusted_mode).vsync_start + vsync_width;

    drm_mode_set_crtcinfo(adjusted_mode, CRTC_INTERLACE_HALVE_V);

    (*adjusted_mode).crtc_hdisplay = native_mode.hdisplay;
    (*adjusted_mode).crtc_vdisplay = native_mode.vdisplay;
    (*adjusted_mode).crtc_htotal = (*adjusted_mode).crtc_hdisplay + hblank;
    (*adjusted_mode).crtc_hsync_start = (*adjusted_mode).crtc_hdisplay + hover;
    (*adjusted_mode).crtc_hsync_end = (*adjusted_mode).crtc_hsync_start + hsync_width;
    (*adjusted_mode).crtc_vtotal = (*adjusted_mode).crtc_vdisplay + vblank;
    (*adjusted_mode).crtc_vsync_start = (*adjusted_mode).crtc_vdisplay + vover;
    (*adjusted_mode).crtc_vsync_end = (*adjusted_mode).crtc_vsync_start + vsync_width;
}

pub unsafe fn amdgpu_dig_monitor_is_duallink(
    encoder: *mut drm_encoder,
    pixel_clock: u32,
) -> bool {
    let mut connector: *mut drm_connector;
    let mut amdgpu_connector: *mut amdgpu_connector;
    let mut dig_connector: *mut amdgpu_connector_atom_dig;

    connector = amdgpu_get_connector_for_encoder(encoder);
    /* if we don't have an active device yet, just use one of
     * the connectors tied to the encoder.
     */
    if connector.is_null() {
        connector = amdgpu_get_connector_for_encoder_init(encoder);
    }
    amdgpu_connector = to_amdgpu_connector(connector);

    match (*connector).connector_type {
        DRM_MODE_CONNECTOR_DVII | DRM_MODE_CONNECTOR_HDMIB => {
            if (*amdgpu_connector).use_digital {
                /* HDMI 1.3 supports up to 340 Mhz over single link */
                if (*connector).display_info.is_hdmi {
                    pixel_clock > 340000
                } else {
                    pixel_clock > 165000
                }
            } else {
                false
            }
        }
        DRM_MODE_CONNECTOR_DVID | DRM_MODE_CONNECTOR_HDMIA | DRM_MODE_CONNECTOR_DisplayPort => {
            dig_connector = (*amdgpu_connector).con_priv;
            if (*dig_connector).dp_sink_type == CONNECTOR_OBJECT_ID_DISPLAYPORT
                || (*dig_connector).dp_sink_type == CONNECTOR_OBJECT_ID_eDP
            {
                false
            } else if (*connector).display_info.is_hdmi {
                pixel_clock > 340000
            } else {
                pixel_clock > 165000
            }
        }
        _ => false,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
