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

// Dependency supplied by the surrounding translation unit: link_service.h.

extern "C" {
    pub fn get_hpd_line(link: *mut dc_link) -> hpd_source_id;

    /*
     * Function: program_hpd_filter
     *
     * @brief
     *    Programs HPD filter on associated HPD line to default values.
     *
     * @return
     *    true on success, false otherwise
     */
    pub fn program_hpd_filter(link: *const dc_link) -> bool;

    /* Query hot plug status of USB4 DP tunnel.
     * Returns true if HPD high.
     */
    pub fn dpia_query_hpd_status(link: *mut dc_link) -> bool;
    pub fn link_get_hpd_state(link: *mut dc_link) -> bool;
    pub fn link_get_hpd_gpio(
        dcb: *mut dc_bios,
        link_id: graphics_object_id,
        gpio_service: *mut gpio_service,
    ) -> *mut gpio;
    pub fn link_enable_hpd(link: *const dc_link);
    pub fn link_disable_hpd(link: *const dc_link);
    pub fn link_enable_hpd_filter(link: *mut dc_link, enable: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
