/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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
 */

/* This file implements basic DPCD read/write functionality and range checks. */

/* Symbols supplied by the surrounding display driver are intentionally left as dependencies. */

#[repr(C)]
pub struct dc_link {
    pub ctx: *mut core::ffi::c_void,
    pub aux_access_disabled: bool,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum dc_status {
    DC_ERROR_UNEXPECTED,
    DC_OK,
}

extern "C" {
    fn dm_helpers_dp_read_dpcd(ctx: *mut core::ffi::c_void, link: *mut dc_link,
        address: u32, data: *mut u8, size: u32) -> bool;
    fn dm_helpers_dp_write_dpcd(ctx: *mut core::ffi::c_void, link: *mut dc_link,
        address: u32, data: *const u8, size: u32) -> bool;
    fn kcalloc(size: usize, size_of_element: usize, flags: u32) -> *mut u8;
    fn kfree(ptr: *mut u8);
}

const GFP_KERNEL: u32 = 0;

#[repr(C)]
#[derive(Clone, Copy)]
struct dpcd_address_range { start: u32, end: u32 }

/* Dependency-provided DPCD constants and function-like macros are preserved below. */
extern "C" {
    static DP_LT_TUNABLE_PHY_REPEATER_FIELD_DATA_STRUCTURE_REV: u32;
    static DP_PHY_REPEATER_128B132B_RATES: u32;
    static DP_LTTPR_MAX_ADD: u32;
    static DP_DPCD_MAX_ADD: u32;
}

unsafe fn internal_link_read_dpcd(link: *mut dc_link, address: u32, data: *mut u8, size: u32) -> dc_status {
    if !(*link).aux_access_disabled && !dm_helpers_dp_read_dpcd((*link).ctx, link, address, data, size) {
        return dc_status::DC_ERROR_UNEXPECTED;
    }
    dc_status::DC_OK
}

unsafe fn internal_link_write_dpcd(link: *mut dc_link, address: u32, data: *const u8, size: u32) -> dc_status {
    if !(*link).aux_access_disabled && !dm_helpers_dp_write_dpcd((*link).ctx, link, address, data, size) {
        return dc_status::DC_ERROR_UNEXPECTED;
    }
    dc_status::DC_OK
}

/* Partition the entire DPCD address space. */
static mandatory_dpcd_partitions: &[dpcd_address_range] = &[
    dpcd_address_range { start: 0, end: 0 },
    dpcd_address_range { start: 0, end: 0 }, dpcd_address_range { start: 0, end: 0 },
    dpcd_address_range { start: 0, end: 0 }, dpcd_address_range { start: 0, end: 0 },
    dpcd_address_range { start: 0, end: 0 }, dpcd_address_range { start: 0, end: 0 },
    dpcd_address_range { start: 0, end: 0 }, dpcd_address_range { start: 0, end: 0 },
    dpcd_address_range { start: 0, end: 0 }, dpcd_address_range { start: 0, end: 0 },
    dpcd_address_range { start: 0, end: 0 }, dpcd_address_range { start: 0, end: 0 },
    dpcd_address_range { start: 0, end: 0 }, dpcd_address_range { start: 0, end: 0 },
    dpcd_address_range { start: 0, end: 0 }, dpcd_address_range { start: 0, end: 0 },
    dpcd_address_range { start: 0, end: 0 },
];

#[inline]
unsafe fn do_addresses_intersect_with_range(range: *const dpcd_address_range, start_address: u32, end_address: u32) -> bool {
    start_address <= (*range).end && end_address >= (*range).start
}

unsafe fn dpcd_get_next_partition_size(address: u32, size: u32) -> u32 {
    let end_address = address.wrapping_add(size).wrapping_sub(1);
    let mut partition_iterator = 0usize;
    while !do_addresses_intersect_with_range(&mandatory_dpcd_partitions[partition_iterator], address, end_address) {
        partition_iterator += 1;
    }
    if end_address < mandatory_dpcd_partitions[partition_iterator].end { size }
    else { mandatory_dpcd_partitions[partition_iterator].end.wrapping_sub(address).wrapping_add(1) }
}

static mandatory_dpcd_blocks: [dpcd_address_range; 1] = [dpcd_address_range { start: 0, end: 0 }];

unsafe fn dpcd_extend_address_range(in_address: u32, in_data: *mut u8, in_size: u32,
    out_address: *mut u32, out_data: *mut *mut u8, out_size: *mut u32) {
    let end_address = in_address.wrapping_add(in_size).wrapping_sub(1);
    let mut new_addr_range = dpcd_address_range { start: in_address, end: end_address };
    for addr_range in mandatory_dpcd_blocks.iter() {
        if addr_range.start <= in_address && addr_range.end >= in_address { new_addr_range.start = addr_range.start; }
        if addr_range.start <= end_address && addr_range.end >= end_address { new_addr_range.end = addr_range.end; }
    }
    *out_address = in_address; *out_size = in_size; *out_data = in_data;
    if new_addr_range.start != in_address || new_addr_range.end != end_address {
        *out_address = new_addr_range.start;
        *out_size = new_addr_range.end.wrapping_sub(new_addr_range.start).wrapping_add(1);
        *out_data = kcalloc(*out_size as usize, core::mem::size_of::<u8>(), GFP_KERNEL);
        assert!(!(*out_data).is_null());
    }
}

unsafe fn dpcd_reduce_address_range(extended_address: u32, extended_data: *mut u8, _extended_size: u32,
    reduced_address: u32, reduced_data: *mut u8, reduced_size: u32) {
    let offset = reduced_address.wrapping_sub(extended_address);
    if extended_data == reduced_data { return; }
    core::ptr::copy_nonoverlapping(reduced_data, extended_data.add(offset as usize), reduced_size as usize);
    kfree(extended_data);
}

pub unsafe fn core_link_read_dpcd(link: *mut dc_link, address: u32, data: *mut u8, size: u32) -> dc_status {
    let mut extended_address = 0; let mut extended_data = core::ptr::null_mut(); let mut extended_size = 0;
    dpcd_extend_address_range(address, data, size, &mut extended_address, &mut extended_data, &mut extended_size);
    let mut partitioned_address = extended_address; let mut size_left_to_read = extended_size;
    let mut status = dc_status::DC_ERROR_UNEXPECTED; let mut data_index = 0;
    while size_left_to_read != 0 {
        let partition_size = dpcd_get_next_partition_size(partitioned_address, size_left_to_read);
        status = internal_link_read_dpcd(link, partitioned_address, extended_data.add(data_index as usize), partition_size);
        if status != dc_status::DC_OK { break; }
        partitioned_address += partition_size; data_index += partition_size; size_left_to_read -= partition_size;
    }
    dpcd_reduce_address_range(extended_address, extended_data, extended_size, address, data, size); status
}

pub unsafe fn core_link_write_dpcd(link: *mut dc_link, mut address: u32, data: *const u8, mut size: u32) -> dc_status {
    let mut data_index = 0; let mut status = dc_status::DC_ERROR_UNEXPECTED;
    while size != 0 {
        let partition_size = dpcd_get_next_partition_size(address, size);
        status = internal_link_write_dpcd(link, address, data.add(data_index as usize), partition_size);
        if status != dc_status::DC_OK { break; }
        address += partition_size; data_index += partition_size; size -= partition_size;
    }
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
