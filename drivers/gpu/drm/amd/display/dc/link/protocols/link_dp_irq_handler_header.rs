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
 *
 */

// Dependency: declarations supplied by "link_service.h".

extern "C" {
    pub fn dp_parse_link_loss_status(
        link: *mut dc_link,
        hpd_irq_dpcd_data: *mut hpd_irq_data,
    ) -> bool;

    pub fn dp_should_allow_hpd_rx_irq(link: *const dc_link) -> bool;

    pub fn dp_handle_link_loss(link: *mut dc_link);

    pub fn dp_read_hpd_rx_irq_data(
        link: *mut dc_link,
        irq_data: *mut hpd_irq_data,
    ) -> dc_status;

    pub fn dp_handle_hpd_rx_irq(
        link: *mut dc_link,
        out_hpd_irq_dpcd_data: *mut hpd_irq_data,
        out_link_loss: *mut bool,
        defer_handling: bool,
        has_left_work: *mut bool,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
