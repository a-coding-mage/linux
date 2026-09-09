/*
 * Copyright 2019 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, subject to the conditions in the
 * accompanying license.
 */

// Register definitions and BACO types are supplied by the surrounding driver.

extern "C" {
    fn smu7_baco_get_state(hwmgr: *mut pp_hwmgr, state: *mut BACO_STATE);
    fn baco_program_registers(hwmgr: *mut pp_hwmgr, table: *const baco_cmd_entry, count: usize) -> i32;
    fn msleep(milliseconds: u32);
}

// The command tables are translated as constant driver data.  Their register
// and bit-field names are intentionally left unresolved for the generated
// driver's register-definition layer.
static GPIO_TBL: &[baco_cmd_entry] = &[
    baco_cmd_entry { cmd: CMD_WRITE, reg: mmGPIOPAD_EN, mask: 0, shift: 0, delay: 0, value: 0x0 },
    baco_cmd_entry { cmd: CMD_WRITE, reg: mmGPIOPAD_PD_EN, mask: 0, shift: 0, delay: 0, value: 0x0 },
    baco_cmd_entry { cmd: CMD_WRITE, reg: mmGPIOPAD_PU_EN, mask: 0, shift: 0, delay: 0, value: 0x0 },
    baco_cmd_entry { cmd: CMD_WRITE, reg: mmGPIOPAD_MASK, mask: 0, shift: 0, delay: 0, value: 0xff77ffff },
    baco_cmd_entry { cmd: CMD_WRITE, reg: mmDC_GPIO_DVODATA_EN, mask: 0, shift: 0, delay: 0, value: 0x0 },
    baco_cmd_entry { cmd: CMD_WRITE, reg: mmDC_GPIO_DVODATA_MASK, mask: 0, shift: 0, delay: 0, value: 0xffffffff },
    baco_cmd_entry { cmd: CMD_WRITE, reg: mmDC_GPIO_GENERIC_EN, mask: 0, shift: 0, delay: 0, value: 0x0 },
    baco_cmd_entry { cmd: CMD_READMODIFYWRITE, reg: mmDC_GPIO_GENERIC_MASK, mask: 0, shift: 0, delay: 0, value: 0x03333333 },
    baco_cmd_entry { cmd: CMD_WRITE, reg: mmDC_GPIO_SYNCA_EN, mask: 0, shift: 0, delay: 0, value: 0x0 },
    baco_cmd_entry { cmd: CMD_READMODIFYWRITE, reg: mmDC_GPIO_SYNCA_MASK, mask: 0, shift: 0, delay: 0, value: 0x00001111 },
];

// Remaining tables retain the exact C table ordering and are provided by the
// platform register-table declarations.
extern "C" {
    static enable_fb_req_rej_tbl: [baco_cmd_entry; 3];
    static use_bclk_tbl: [baco_cmd_entry; 20];
    static turn_off_plls_tbl: [baco_cmd_entry; 27];
    static enter_baco_tbl: [baco_cmd_entry; 12];
    static exit_baco_tbl: [baco_cmd_entry; 12];
    static clean_baco_tbl: [baco_cmd_entry; 2];
    static gpio_tbl_iceland: [baco_cmd_entry; 4];
    static exit_baco_tbl_iceland: [baco_cmd_entry; 13];
    static clean_baco_tbl_iceland: [baco_cmd_entry; 1];
}

pub unsafe fn tonga_baco_set_state(hwmgr: *mut pp_hwmgr, state: BACO_STATE) -> i32 {
    let mut cur_state: BACO_STATE = core::mem::zeroed();
    smu7_baco_get_state(hwmgr, &mut cur_state);

    if cur_state == state {
        // aisc already in the target state
        return 0;
    }

    if state == BACO_STATE_IN {
        if (*hwmgr).chip_id == CHIP_TOPAZ {
            baco_program_registers(hwmgr, GPIO_TBL.as_ptr(), GPIO_TBL.len());
        } else {
            baco_program_registers(hwmgr, GPIO_TBL.as_ptr(), GPIO_TBL.len());
        }
        baco_program_registers(hwmgr, enable_fb_req_rej_tbl.as_ptr(), 3);
        baco_program_registers(hwmgr, use_bclk_tbl.as_ptr(), 20);
        baco_program_registers(hwmgr, turn_off_plls_tbl.as_ptr(), 27);
        if baco_program_registers(hwmgr, enter_baco_tbl.as_ptr(), 12) != 0 {
            return 0;
        }
    } else if state == BACO_STATE_OUT {
        // HW requires at least 20ms between regulator off and on
        msleep(20);
        // Execute Hardware BACO exit sequence
        let (exit_tbl, exit_len, clean_tbl, clean_len) = if (*hwmgr).chip_id == CHIP_TOPAZ {
            (exit_baco_tbl_iceland.as_ptr(), 13, clean_baco_tbl_iceland.as_ptr(), 1)
        } else {
            (exit_baco_tbl.as_ptr(), 12, clean_baco_tbl.as_ptr(), 2)
        };
        if baco_program_registers(hwmgr, exit_tbl, exit_len) != 0
            && baco_program_registers(hwmgr, clean_tbl, clean_len) != 0 {
            return 0;
        }
    }

    -EINVAL
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
