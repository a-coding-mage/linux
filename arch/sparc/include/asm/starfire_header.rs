/* SPDX-License-Identifier: GPL-2.0 */
/*
 * starfire.h: Group all starfire specific code together.
 *
 * Copyright (C) 2000 Anton Blanchard (anton@samba.org)
 */

// Header guard: _SPARC64_STARFIRE_H
// These declarations are excluded when assembling (__ASSEMBLER__).

unsafe extern "C" {
    pub static mut this_is_starfire: i32;

    pub fn check_if_starfire();
    pub fn starfire_hookup(_: i32);
    pub fn starfire_translate(imap: u64, upaid: u32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
