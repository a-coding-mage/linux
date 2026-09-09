/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
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

// Dependency declarations are supplied by the surrounding translated sources.

pub const AUX_POWER_UP_WA_DELAY: u32 = 500;
pub const I2C_OVER_AUX_DEFER_WA_DELAY: u32 = 70;
pub const DPVGA_DONGLE_AUX_DEFER_WA_DELAY: u32 = 40;
pub const I2C_OVER_AUX_DEFER_WA_DELAY_1MS: u32 = 1;
pub const LINK_AUX_DEFAULT_LTTPR_TIMEOUT_PERIOD: u32 = 3200; // us
pub const LINK_AUX_DEFAULT_TIMEOUT_PERIOD: u32 = 552; // us

pub const EDID_SEGMENT_SIZE: u32 = 256;

extern "C" {
    pub fn link_create_ddc_service(
        ddc_init_data: *mut ddc_service_init_data,
    ) -> *mut ddc_service;

    pub fn link_destroy_ddc_service(ddc: *mut *mut ddc_service);

    pub fn set_ddc_transaction_type(
        ddc: *mut ddc_service,
        type_: ddc_transaction_type,
    );

    pub fn link_get_ddc_aux_inst(link: *const dc_link) -> u8;

    pub fn link_get_aux_defer_delay(ddc: *mut ddc_service) -> u32;

    pub fn link_is_in_aux_transaction_mode(ddc: *mut ddc_service) -> bool;

    pub fn try_to_configure_aux_timeout(ddc: *mut ddc_service, timeout: u32) -> bool;

    pub fn link_query_ddc_data(
        ddc: *mut ddc_service,
        address: u32,
        write_buf: *mut u8,
        write_size: u32,
        read_buf: *mut u8,
        read_size: u32,
    ) -> bool;

    /* Attempt to submit an aux payload, retrying on timeouts, defers, and busy
     * states as outlined in the DP spec. Returns true if the request was
     * successful.
     *
     * NOTE: The function requires explicit mutex on DM side in order to prevent
     * potential race condition. DC components should call the dpcd read/write
     * function in dm_helpers in order to access dpcd safely.
     */
    pub fn link_aux_transfer_with_retries_no_mutex(
        ddc: *mut ddc_service,
        payload: *mut aux_payload,
    ) -> bool;

    pub fn link_configure_fixed_vs_pe_retimer(
        ddc: *mut ddc_service,
        data: *const u8,
        length: u32,
    ) -> bool;

    pub fn link_query_fixed_vs_pe_retimer(
        ddc: *mut ddc_service,
        data: *mut u8,
        length: u32,
    ) -> bool;

    pub fn link_get_fixed_vs_pe_retimer_read_address(link: *mut dc_link) -> u32;
    pub fn link_get_fixed_vs_pe_retimer_write_address(link: *mut dc_link) -> u32;

    pub fn write_scdc_data(
        ddc_service: *mut ddc_service,
        pix_clk: u32,
        lte_340_scramble: bool,
    );

    pub fn read_scdc_data(ddc_service: *mut ddc_service);

    pub fn write_idcc_data(
        ddc_service: *mut ddc_service,
        idcc_scope: hdmi_idcc_scope,
        write_buf: *mut u8,
        offset: u8,
        write_len: u8,
    );

    pub fn read_idcc_data(
        ddc_service: *mut ddc_service,
        idcc_scope: hdmi_idcc_scope,
        read_buf: *mut u8,
        offset: u8,
        read_len: u8,
    ) -> i32;

    pub fn set_dongle_type(ddc: *mut ddc_service, dongle_type: display_dongle_type);

    pub fn get_ddc_pin(ddc_service: *mut ddc_service) -> *mut ddc;

    pub fn link_aux_transfer_raw(
        ddc: *mut ddc_service,
        payload: *mut aux_payload,
        operation_result: *mut aux_return_code_type,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
