/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

// Dependency supplied by another translated file.
pub enum pt_regs {}

extern "C" {
    pub fn execute_protection_fault(regs: *mut pt_regs);
    pub fn write_protection_fault(regs: *mut pt_regs);
    pub fn read_protection_fault(regs: *mut pt_regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
