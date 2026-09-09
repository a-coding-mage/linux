/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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
 * Authors: AMD
 */

/* FILE POLICY AND INTENDED USAGE:
 *
 * This file implements functions that manage basic HPD components such as gpio.
 * It also provides wrapper functions to execute HPD related programming. This
 * file only manages basic HPD functionality. It doesn't manage detection or
 * feature or signal specific HPD behaviors.
 */

pub unsafe fn link_get_hpd_state(link: *mut dc_link) -> bool {
    if !(*link).link_enc.is_null() {
        (*(*(*link).link_enc).funcs).get_hpd_state((*link).link_enc)
    } else {
        false
    }
}

pub unsafe fn link_enable_hpd(link: *const dc_link) {
    if !(*link).link_enc.is_null() {
        (*(*(*link).link_enc).funcs).enable_hpd((*link).link_enc);
    }
}

pub unsafe fn link_disable_hpd(link: *const dc_link) {
    if !(*link).link_enc.is_null() {
        (*(*(*link).link_enc).funcs).disable_hpd((*link).link_enc);
    }
}

pub unsafe fn link_enable_hpd_filter(link: *mut dc_link, enable: bool) {
    if enable {
        (*link).is_hpd_filter_disabled = false;
        program_hpd_filter(link);
    } else {
        (*link).is_hpd_filter_disabled = true;
        if !(*link).link_enc.is_null() {
            (*(*(*link).link_enc).funcs).program_hpd_filter((*link).link_enc, 0, 0);
        }
    }
}

pub unsafe fn program_hpd_filter(link: *const dc_link) -> bool {
    let mut delay_on_connect_in_ms: i32 = 0;
    let mut delay_on_disconnect_in_ms: i32 = 0;

    if (*link).is_hpd_filter_disabled || (*link).link_enc.is_null() {
        ASSERT(!(*link).link_enc.is_null());
        return false;
    }

    /* Verify feature is supported */
    match (*link).connector_signal {
        SIGNAL_TYPE_DVI_SINGLE_LINK | SIGNAL_TYPE_DVI_DUAL_LINK |
        SIGNAL_TYPE_HDMI_TYPE_A | SIGNAL_TYPE_HDMI_FRL => {
            /* Program hpd filter */
            delay_on_connect_in_ms = 500;
            delay_on_disconnect_in_ms = 100;
        }
        SIGNAL_TYPE_DISPLAY_PORT | SIGNAL_TYPE_DISPLAY_PORT_MST => {
            /* Program hpd filter to allow DP signal to settle */
            /* 500: not able to detect MST <-> SST switch as HPD is low for
             * only 100ms on DELL U2413
             * 0: some passive dongle still show aux mode instead of i2c
             * 20-50: not enough to hide bouncing HPD with passive dongle.
             * also see intermittent i2c read issues.
             */
            delay_on_connect_in_ms = 80;
            delay_on_disconnect_in_ms = 0;
        }
        SIGNAL_TYPE_LVDS | SIGNAL_TYPE_EDP => return false,
        _ => return false,
    }

    (*(*(*link).link_enc).funcs).program_hpd_filter(
        (*link).link_enc,
        delay_on_connect_in_ms,
        delay_on_disconnect_in_ms,
    )
}

pub unsafe fn link_get_hpd_gpio(
    dcb: *mut dc_bios,
    link_id: graphics_object_id,
    gpio_service: *mut gpio_service,
) -> *mut gpio {
    let mut hpd_info: graphics_object_hpd_info = core::mem::zeroed();
    let mut pin_info: gpio_pin_info = core::mem::zeroed();

    if (*(*dcb).funcs).get_hpd_info(dcb, link_id, &mut hpd_info) != BP_RESULT_OK {
        return core::ptr::null_mut();
    }

    let bp_result = (*(*dcb).funcs).get_gpio_pin_info(
        dcb,
        hpd_info.hpd_int_gpio_uid,
        &mut pin_info,
    );
    if bp_result != BP_RESULT_OK {
        return core::ptr::null_mut();
    }

    dal_gpio_service_create_irq(gpio_service, pin_info.offset, pin_info.mask)
}

pub unsafe fn get_hpd_line(link: *mut dc_link) -> hpd_source_id {
    let hpd: *mut gpio;
    let mut hpd_id = HPD_SOURCEID_UNKNOWN;

    /* Use GPIO path where supported, otherwise use hardware encoder path */
    if !(*link).ctx.is_null() && (*(*link).ctx).dce_version <= DCN_VERSION_4_01 {
        hpd = link_get_hpd_gpio(
            (*(*link).ctx).dc_bios,
            (*link).link_id,
            (*(*link).ctx).gpio_service,
        );
    } else {
        hpd = core::ptr::null_mut();
    }

    if !hpd.is_null() {
        match dal_irq_get_source(hpd) {
            DC_IRQ_SOURCE_HPD1 => hpd_id = HPD_SOURCEID1,
            DC_IRQ_SOURCE_HPD2 => hpd_id = HPD_SOURCEID2,
            DC_IRQ_SOURCE_HPD3 => hpd_id = HPD_SOURCEID3,
            DC_IRQ_SOURCE_HPD4 => hpd_id = HPD_SOURCEID4,
            DC_IRQ_SOURCE_HPD5 => hpd_id = HPD_SOURCEID5,
            DC_IRQ_SOURCE_HPD6 => hpd_id = HPD_SOURCEID6,
            _ => BREAK_TO_DEBUGGER(),
        }

        dal_gpio_destroy_irq(&hpd);
    }

    hpd_id
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
