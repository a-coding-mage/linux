// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Module version support
 *
 * Copyright (C) 2008 Rusty Russell
 */

// C dependencies supplied by the surrounding kernel translation.

pub unsafe fn check_version(
    info: *const load_info,
    symname: *const ::core::ffi::c_char,
    mod_: *mut module,
    crc: *const u32,
) -> ::core::ffi::c_int {
    let sechdrs: *mut Elf_Shdr = (*info).sechdrs;
    let versindex: ::core::ffi::c_uint = (*info).index.vers;
    let mut i: ::core::ffi::c_uint;
    let num_versions: ::core::ffi::c_uint;
    let versions: *mut modversion_info;
    let mut version_ext: modversion_info_ext = ::core::mem::zeroed();

    /* Exporting module didn't supply crcs?  OK, we're already tainted. */
    if crc.is_null() {
        return 1;
    }

    /* If we have extended version info, rely on it */
    if (*info).index.vers_ext_crc != 0 {
        // Equivalent of for_each_modversion_info_ext(version_ext, info).
        while version_ext.remaining != 0 {
            if strcmp(version_ext.name, symname) != 0 {
                modversion_ext_advance(&mut version_ext);
                continue;
            }
            if *version_ext.crc == *crc {
                return 1;
            }
            pr_debug(c"Found checksum %X vs module %X\n", *crc, *version_ext.crc);
            goto_bad_version(info, symname);
            return 0;
        }
        pr_warn_once(c"%s: no extended symbol version for %s\n", (*info).name, symname);
        return 1;
    }

    /* No versions at all?  modprobe --force does this. */
    if versindex == 0 {
        return (try_to_force_load(mod_, symname) == 0) as ::core::ffi::c_int;
    }

    versions = (*sechdrs.add(versindex as usize)).sh_addr as *mut modversion_info;
    num_versions = ((*sechdrs.add(versindex as usize)).sh_size
        / ::core::mem::size_of::<modversion_info>()) as ::core::ffi::c_uint;

    i = 0;
    while i < num_versions {
        let mut crcval: u32;

        if strcmp((*versions.add(i as usize)).name, symname) != 0 {
            i += 1;
            continue;
        }

        crcval = *crc;
        if (*versions.add(i as usize)).crc == crcval {
            return 1;
        }
        pr_debug(c"Found checksum %X vs module %lX\n",
                 crcval, (*versions.add(i as usize)).crc);
        goto_bad_version(info, symname);
        return 0;
    }

    /* Broken toolchain. Warn once, then let it go.. */
    pr_warn_once(c"%s: no symbol version for %s\n", (*info).name, symname);
    return 1;
}

unsafe fn goto_bad_version(info: *const load_info,
                           symname: *const ::core::ffi::c_char) {
    pr_warn(c"%s: disagrees about version of symbol %s\n", (*info).name, symname);
}

pub unsafe fn check_modstruct_version(info: *const load_info,
                                      mod_: *mut module) -> ::core::ffi::c_int {
    let mut fsa = find_symbol_arg {
        name: c"module_layout".as_ptr(),
        gplok: true,
        ..::core::mem::zeroed()
    };
    let have_symbol: bool;

    /*
     * Since this should be found in kernel (which can't be removed), no
     * locking is necessary. Regardless use a RCU read section to keep
     * lockdep happy.
     */
    // scoped_guard(rcu)
    have_symbol = find_symbol(&mut fsa);
    BUG_ON(!have_symbol);

    check_version(info, c"module_layout".as_ptr(), mod_, fsa.crc)
}

/* First part is kernel version, which we ignore if module has crcs. */
pub unsafe fn same_magic(
    mut amagic: *const ::core::ffi::c_char,
    mut bmagic: *const ::core::ffi::c_char,
    has_crcs: bool,
) -> ::core::ffi::c_int {
    if has_crcs {
        amagic = amagic.add(strcspn(amagic, c" ".as_ptr()));
        bmagic = bmagic.add(strcspn(bmagic, c" ".as_ptr()));
    }
    (strcmp(amagic, bmagic) == 0) as ::core::ffi::c_int
}

pub unsafe fn modversion_ext_start(info: *const load_info,
                                   start: *mut modversion_info_ext) {
    let crc_idx: ::core::ffi::c_uint = (*info).index.vers_ext_crc;
    let name_idx: ::core::ffi::c_uint = (*info).index.vers_ext_name;
    let sechdrs: *mut Elf_Shdr = (*info).sechdrs;

    /*
     * Both of these fields are needed for this to be useful
     * Any future fields should be initialized to NULL if absent.
     */
    if crc_idx == 0 || name_idx == 0 {
        (*start).remaining = 0;
        return;
    }

    (*start).crc = (*sechdrs.add(crc_idx as usize)).sh_addr as *const u32;
    (*start).name = (*sechdrs.add(name_idx as usize)).sh_addr as *const ::core::ffi::c_char;
    (*start).remaining = (*sechdrs.add(crc_idx as usize)).sh_size
        / ::core::mem::size_of::<u32>();
}

pub unsafe fn modversion_ext_advance(vers: *mut modversion_info_ext) {
    (*vers).remaining -= 1;
    (*vers).crc = (*vers).crc.add(1);
    (*vers).name = (*vers).name.add(strlen((*vers).name) + 1);
}

/*
 * Generate the signature for all relevant module structures here.
 * If these change, we don't want to try to parse the module.
 */
pub unsafe fn module_layout(
    _mod_: *mut module,
    _ver: *mut modversion_info,
    _kp: *mut kernel_param,
    _ks: *mut kernel_symbol,
    _tp: *const *mut tracepoint,
) {
}

// EXPORT_SYMBOL(module_layout);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
