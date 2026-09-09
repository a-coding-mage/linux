// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Module kdb support
 *
 * Copyright (C) 2010 Jason Wessel
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/module.h, linux/kdb.h, and internal.h.

/*
 * kdb_lsmod - This function implements the 'lsmod' command.  Lists
 *	currently loaded kernel modules.
 *	Mostly taken from userland lsmod.
 */
pub unsafe fn kdb_lsmod(argc: ::core::ffi::c_int, _argv: *const *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut mod_: *mut module;

    if argc != 0 {
        return KDB_ARGCOUNT;
    }

    kdb_printf(c"Module                  Size  modstruct     Used by\n".as_ptr());
    // Direct translation of list_for_each_entry(mod, &modules, list).
    list_for_each_entry!(mod_, modules, list, {
        if (*mod_).state == MODULE_STATE_UNFORMED {
            continue;
        }

        kdb_printf(c"%-20s%8u".as_ptr(), (*mod_).name, (*mod_).mem[MOD_TEXT].size);
        kdb_printf(c"/%8u".as_ptr(), (*mod_).mem[MOD_RODATA].size);
        kdb_printf(c"/%8u".as_ptr(), (*mod_).mem[MOD_RO_AFTER_INIT].size);
        kdb_printf(c"/%8u".as_ptr(), (*mod_).mem[MOD_DATA].size);

        kdb_printf(c"  0x%px ".as_ptr(), mod_ as *mut ::core::ffi::c_void);
        // CONFIG_MODULE_UNLOAD is a build-time condition from the C source.
        #[cfg(CONFIG_MODULE_UNLOAD)]
        kdb_printf(c"%4d ".as_ptr(), module_refcount(mod_));

        if (*mod_).state == MODULE_STATE_GOING {
            kdb_printf(c" (Unloading)".as_ptr());
        } else if (*mod_).state == MODULE_STATE_COMING {
            kdb_printf(c" (Loading)".as_ptr());
        } else {
            kdb_printf(c" (Live)".as_ptr());
        }
        kdb_printf(c" 0x%px".as_ptr(), (*mod_).mem[MOD_TEXT].base);
        kdb_printf(c"/0x%px".as_ptr(), (*mod_).mem[MOD_RODATA].base);
        kdb_printf(c"/0x%px".as_ptr(), (*mod_).mem[MOD_RO_AFTER_INIT].base);
        kdb_printf(c"/0x%px".as_ptr(), (*mod_).mem[MOD_DATA].base);

        // CONFIG_MODULE_UNLOAD is a build-time condition from the C source.
        #[cfg(CONFIG_MODULE_UNLOAD)]
        {
            let mut use_: *mut module_use;

            kdb_printf(c" [ ".as_ptr());
            // Direct translation of list_for_each_entry(use, &mod->source_list,
            // source_list).
            list_for_each_entry!(use_, (*mod_).source_list, source_list, {
                kdb_printf(c"%s ".as_ptr(), (*(*use_).target).name);
            });
            kdb_printf(c"]\n".as_ptr());
        }
    });

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
