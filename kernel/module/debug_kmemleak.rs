// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Module kmemleak support
 *
 * Copyright (C) 2009 Catalin Marinas
 */

// Dependencies supplied by the Linux kernel module and kmemleak interfaces,
// together with the local internal declarations, are provided externally.

/// Scan writable, non-executable module sections for kmemleak references.
pub unsafe fn kmemleak_load_module(
    mod_: *const module,
    info: *const load_info,
) {
    let _ = info;

    // only scan writable, non-executable sections
    for_each_mod_mem_type!(type_, {
        if type_ != MOD_DATA && type_ != MOD_INIT_DATA
            && !(*mod_).mem[type_].is_rox
        {
            kmemleak_no_scan((*mod_).mem[type_].base);
        }
    });
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
