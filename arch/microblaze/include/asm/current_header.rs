/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2008-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2008-2009 PetaLogix
 * Copyright (C) 2006 Atmark Techno, Inc.
 */

/*
 * Register used to hold the current task pointer while in the kernel.
 * Any `call clobbered' register without a special meaning should be OK,
 * but check asm/microblaze/kernel/entry.S to be sure.
 */
pub const CURRENT_TASK: &str = "r31";

/*
 * Dedicate r31 to keeping the current task pointer.
 *
 * The C declaration uses the MicroBlaze r31 register. Rust has no
 * corresponding register-global declaration, so the external mutable
 * pointer represents that ABI-provided global.
 */
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

extern "C" {
    pub static mut current: *mut task_struct;
}

#[inline]
pub unsafe fn get_current() -> *mut task_struct {
    current
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
