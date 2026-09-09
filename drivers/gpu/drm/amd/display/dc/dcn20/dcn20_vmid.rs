/*
 * Copyright 2018 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding driver translation.

macro_rules! REG { ($vmid:expr, $reg:ident) => { $vmid.regs.$reg }; }
macro_rules! CTX { ($vmid:expr) => { $vmid.ctx }; }
macro_rules! FN {
    ($vmid:expr, $reg_name:ident, $field_name:ident) => {
        ($vmid.shifts.$field_name, $vmid.masks.$field_name)
    };
}
macro_rules! DC_LOGGER { ($vmid:expr) => { CTX!($vmid).logger }; }

unsafe fn dcn20_wait_for_vmid_ready(vmid: *mut dcn20_vmid) {
    /* According the hardware spec, we need to poll for the lowest
     * bit of PAGE_TABLE_BASE_ADDR_LO32 = 1 any time a GPUVM
     * context is updated. We can't use REG_WAIT here since we
     * don't have a seperate field to wait on.
     *
     * TODO: Confirm timeout / poll interval with hardware team
     */

    let max_times: i32 = 10000;
    let delay_us: i32 = 5;
    let mut i: i32 = 0;

    while i < max_times {
        let mut entry_lo32: u32 = 0;

        REG_GET!(
            vmid,
            PAGE_TABLE_BASE_ADDR_LO32,
            VM_CONTEXT0_PAGE_DIRECTORY_ENTRY_LO32,
            &mut entry_lo32
        );

        if entry_lo32 & 0x1 != 0 {
            return;
        }

        udelay(delay_us);
        i += 1;
    }

    /* VM setup timed out */
    DC_LOG_WARNING!("Timeout while waiting for GPUVM context update\n");
    ASSERT!(0);
}

pub unsafe fn dcn20_vmid_setup(
    vmid: *mut dcn20_vmid,
    config: *const dcn_vmid_page_table_config,
) {
    REG_SET!(
        vmid,
        PAGE_TABLE_START_ADDR_HI32,
        0,
        VM_CONTEXT0_START_LOGICAL_PAGE_NUMBER_HI4,
        ((*config).page_table_start_addr >> 32) & 0xF
    );
    REG_SET!(
        vmid,
        PAGE_TABLE_START_ADDR_LO32,
        0,
        VM_CONTEXT0_START_LOGICAL_PAGE_NUMBER_LO32,
        (*config).page_table_start_addr & 0xFFFFFFFF
    );

    REG_SET!(
        vmid,
        PAGE_TABLE_END_ADDR_HI32,
        0,
        VM_CONTEXT0_END_LOGICAL_PAGE_NUMBER_HI4,
        ((*config).page_table_end_addr >> 32) & 0xF
    );
    REG_SET!(
        vmid,
        PAGE_TABLE_END_ADDR_LO32,
        0,
        VM_CONTEXT0_END_LOGICAL_PAGE_NUMBER_LO32,
        (*config).page_table_end_addr & 0xFFFFFFFF
    );

    REG_SET_2!(
        vmid,
        CNTL,
        0,
        VM_CONTEXT0_PAGE_TABLE_DEPTH,
        (*config).depth,
        VM_CONTEXT0_PAGE_TABLE_BLOCK_SIZE,
        (*config).block_size
    );

    REG_SET!(
        vmid,
        PAGE_TABLE_BASE_ADDR_HI32,
        0,
        VM_CONTEXT0_PAGE_DIRECTORY_ENTRY_HI32,
        ((*config).page_table_base_addr >> 32) & 0xFFFFFFFF
    );
    /* Note: per hardware spec PAGE_TABLE_BASE_ADDR_LO32 must be programmed last in sequence */
    REG_SET!(
        vmid,
        PAGE_TABLE_BASE_ADDR_LO32,
        0,
        VM_CONTEXT0_PAGE_DIRECTORY_ENTRY_LO32,
        (*config).page_table_base_addr & 0xFFFFFFFF
    );

    dcn20_wait_for_vmid_ready(vmid);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
