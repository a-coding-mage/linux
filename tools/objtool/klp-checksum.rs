// SPDX-License-Identifier: GPL-2.0-or-later

// Translated from objtool/klp-checksum.c.  C header dependencies are expected
// to provide the referenced objtool types, globals, constants, macros/helpers,
// and ELF/checksum routines in Rust form.

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_ulong, c_void};
use core::mem::{size_of, size_of_val};
use core::ptr;

type s64 = i64;

static mut IN_ALT: bool = false;

unsafe fn checksum_debug_init(file: *mut objtool_file) -> c_int {
    let mut dup: *mut c_char;
    let mut s: *mut c_char;

    if opts.debug_checksum.is_null() {
        return 0;
    }

    dup = strdup(opts.debug_checksum);
    if dup.is_null() {
        ERROR_GLIBC(c"strdup".as_ptr());
        return -1;
    }

    s = dup;
    while *s != 0 {
        let mut found: bool = false;
        let mut sym: *mut symbol;
        let comma: *mut c_char;

        comma = strchr(s, ',' as c_int);
        if !comma.is_null() {
            *comma = 0;
        }

        for_each_sym_by_name((*file).elf, s, sym, {
            if !is_func_sym(sym) && !is_object_sym(sym) {
                continue;
            }
            (*sym).debug_checksum = 1;
            found = true;
        });

        if !found {
            WARN(c"--debug-checksum: can't find '%s'".as_ptr(), s);
        }

        if comma.is_null() {
            break;
        }

        s = comma.add(1);
    }

    free(dup as *mut c_void);
    0
}

unsafe fn checksum_update_insn(
    file: *mut objtool_file,
    func: *mut symbol,
    insn: *mut instruction,
) {
    let reloc: *mut reloc = insn_reloc(file, insn);
    let mut alt: *mut alternative;
    let mut offset: c_ulong;
    let mut sym: *mut symbol;
    if (*insn).fake {
        return;
    }

    if reloc.is_null() {
        let call_dest: *mut symbol = insn_call_dest(insn);
        let jump_dest: *mut instruction = (*insn).jump_dest;

        /*
         * For a jump/call non-relocated dest offset embedded in the
         * instruction, the offset may vary due to changes in
         * surrounding code.  Just hash the opcode and a
         * position-independent representation of the destination.
         */

        if !call_dest.is_null() || !jump_dest.is_null() {
            let mut buf: [c_uchar; 16] = [0; 16];
            let len: usize;

            len = arch_jump_opcode_bytes(file, insn, buf.as_mut_ptr());
            __checksum_update_insn(func, insn, buf.as_ptr() as *const c_void, len);

            if !call_dest.is_null() {
                __checksum_update_insn(
                    func,
                    insn,
                    (*call_dest).demangled_name as *const c_void,
                    strlen((*call_dest).demangled_name),
                );
            } else if !jump_dest.is_null() {
                let dest_sym: *mut symbol;
                let mut jump_offset: c_ulong;

                /*
                 * use insn->_sym instead of insn_sym() here.
                 * For alternative replacements, the latter
                 * would give the function of the code being
                 * replaced.
                 */
                dest_sym = (*jump_dest)._sym;
                if dest_sym.is_null() {
                    goto_alts(file, func, insn);
                    return;
                }

                __checksum_update_insn(
                    func,
                    insn,
                    (*dest_sym).demangled_name as *const c_void,
                    strlen((*dest_sym).demangled_name),
                );

                jump_offset = (*jump_dest).offset.wrapping_sub((*dest_sym).offset);
                __checksum_update_insn(
                    func,
                    insn,
                    &jump_offset as *const c_ulong as *const c_void,
                    size_of::<c_ulong>(),
                );
            }

            goto_alts(file, func, insn);
            return;
        }
    }

    __checksum_update_insn(
        func,
        insn,
        ((*(*(*insn).sec).data).d_buf as *mut u8).add((*insn).offset as usize) as *const c_void,
        (*insn).len,
    );

    if reloc.is_null() {
        goto_alts(file, func, insn);
        return;
    }

    sym = (*reloc).sym;
    offset = arch_insn_adjusted_addend(insn, reloc) as c_ulong;

    if is_string_sec((*sym).sec) {
        let str_: *mut c_char;

        str_ = ((*(*(*sym).sec).data).d_buf as *mut c_char)
            .add((*sym).offset.wrapping_add(offset) as usize);
        __checksum_update_insn(func, insn, str_ as *const c_void, strlen(str_));
        goto_alts(file, func, insn);
        return;
    }

    if is_sec_sym(sym) {
        sym = find_symbol_containing((*(*reloc).sym).sec, offset);
        if sym.is_null() {
            goto_alts(file, func, insn);
            return;
        }

        offset = offset.wrapping_sub((*sym).offset);
    }

    __checksum_update_insn(
        func,
        insn,
        (*sym).demangled_name as *const c_void,
        strlen((*sym).demangled_name),
    );
    __checksum_update_insn(
        func,
        insn,
        &offset as *const c_ulong as *const c_void,
        size_of::<c_ulong>(),
    );

    goto_alts(file, func, insn);
}

unsafe fn goto_alts(file: *mut objtool_file, func: *mut symbol, insn: *mut instruction) {
    let mut alt: *mut alternative;

    alt = (*insn).alts;
    while !alt.is_null() {
        let alt_group: *mut alt_group = (*(*alt).insn).alt_group;

        /* Prevent __ex_table recursion, e.g. LOAD_SEGMENT() */
        if IN_ALT {
            break;
        }
        IN_ALT = true;

        __checksum_update_insn(
            func,
            insn,
            &(*alt).r#type as *const _ as *const c_void,
            size_of_val(&(*alt).r#type),
        );

        if !alt_group.is_null() && !(*alt_group).orig_group.is_null() {
            let mut alt_insn: *mut instruction;

            __checksum_update_insn(
                func,
                insn,
                &(*alt_group).feature as *const _ as *const c_void,
                size_of_val(&(*alt_group).feature),
            );

            alt_insn = (*alt).insn;
            while !alt_insn.is_null() {
                checksum_update_insn(file, func, alt_insn);
                if (*alt_group).last_insn.is_null() || alt_insn == (*alt_group).last_insn {
                    break;
                }
                alt_insn = next_insn_same_sec(file, alt_insn);
            }
        } else {
            checksum_update_insn(file, func, (*alt).insn);
        }

        IN_ALT = false;
        alt = (*alt).next;
    }
}

unsafe fn checksum_update_object(file: *mut objtool_file, sym: *mut symbol) {
    let mut reloc: *mut reloc;

    __checksum_update_object(
        sym,
        0,
        c"len".as_ptr(),
        &(*sym).len as *const _ as *const c_void,
        size_of_val(&(*sym).len),
    );

    if !(*(*(*sym).sec).data).d_buf.is_null() {
        __checksum_update_object(
            sym,
            0,
            c"data".as_ptr(),
            ((*(*(*sym).sec).data).d_buf as *mut u8).add((*sym).offset as usize) as *const c_void,
            (*sym).len,
        );
    }

    sym_for_each_reloc((*file).elf, sym, reloc, {
        let sym_offset: c_ulong = reloc_offset(reloc).wrapping_sub((*sym).offset);
        let mut target: *mut symbol = (*reloc).sym;
        let mut offset: s64;

        offset = reloc_addend(reloc);

        if is_string_sec((*target).sec) {
            let str_: *mut c_char;

            str_ = ((*(*(*target).sec).data).d_buf as *mut c_char)
                .add((*target).offset.wrapping_add(offset as c_ulong) as usize);
            __checksum_update_object(
                sym,
                sym_offset,
                c"reloc string".as_ptr(),
                str_ as *const c_void,
                strlen(str_),
            );
            continue;
        }

        if is_sec_sym(target) {
            target = find_symbol_containing((*(*reloc).sym).sec, offset as c_ulong);
            if target.is_null() {
                continue;
            }

            offset -= (*target).offset as s64;
        }

        __checksum_update_object(
            sym,
            sym_offset,
            c"reloc name".as_ptr(),
            (*target).demangled_name as *const c_void,
            strlen((*target).demangled_name),
        );
        __checksum_update_object(
            sym,
            sym_offset,
            c"reloc addend".as_ptr(),
            &offset as *const s64 as *const c_void,
            size_of::<s64>(),
        );
    });
}

#[no_mangle]
pub unsafe extern "C" fn calculate_checksums(file: *mut objtool_file) -> c_int {
    let mut insn: *mut instruction;
    let mut sym: *mut symbol;

    if checksum_debug_init(file) != 0 {
        return -1;
    }

    for_each_sym((*file).elf, sym, {
        /*
         * Skip cold subfunctions and aliases: they share the
         * parent's checksum via func_for_each_insn() which
         * follows func->cfunc into the cold subfunction.
         */
        if is_cold_func(sym)
            || is_alias_sym(sym)
            || (*sym).len == 0
            || (*sym).sec.is_null()
            || (*(*sym).sec).data.is_null()
        {
            continue;
        }

        if is_func_sym(sym) {
            checksum_init(sym);
            func_for_each_insn(file, sym, insn, {
                checksum_update_insn(file, sym, insn);
            });
            checksum_finish(sym);
        } else if is_object_sym(sym) {
            checksum_init(sym);
            checksum_update_object(file, sym);
            checksum_finish(sym);
        }
    });

    0
}

#[no_mangle]
pub unsafe extern "C" fn create_sym_checksum_section(file: *mut objtool_file) -> c_int {
    let mut sec: *mut section;
    let mut sym: *mut symbol;
    let mut idx: c_uint = 0;
    let mut checksum: *mut sym_checksum;
    let entsize: usize = size_of::<sym_checksum>();

    sec = find_section_by_name((*file).elf, c".discard.sym_checksum".as_ptr());
    if !sec.is_null() {
        if !opts.dryrun {
            WARN(c"file already has .discard.sym_checksum section, skipping".as_ptr());
        }

        return 0;
    }

    for_each_sym((*file).elf, sym, {
        if (*sym).csum.checksum != 0 {
            idx = idx.wrapping_add(1);
        }
    });

    sec = elf_create_section_pair(
        (*file).elf,
        c".discard.sym_checksum".as_ptr(),
        entsize,
        idx,
        idx,
    );
    if sec.is_null() {
        return -1;
    }

    idx = 0;
    for_each_sym((*file).elf, sym, {
        if (*sym).csum.checksum == 0 {
            continue;
        }

        if !elf_init_reloc(
            (*file).elf,
            (*sec).rsec,
            idx,
            (idx as usize).wrapping_mul(entsize),
            sym,
            0,
            R_TEXT64,
        ) {
            return -1;
        }

        checksum = ((*(*sec).data).d_buf as *mut sym_checksum).add(idx as usize);
        (*checksum).addr = 0; /* reloc */
        (*checksum).checksum = (*sym).csum.checksum;

        mark_sec_changed((*file).elf, sec, true);

        idx = idx.wrapping_add(1);
    });

    0
}

static mut KLP_CHECKSUM_USAGE: [*const c_char; 2] = [
    c"objtool klp checksum [<options>] file.o".as_ptr(),
    ptr::null(),
];

#[no_mangle]
pub unsafe extern "C" fn cmd_klp_checksum(argc: c_int, argv: *mut *const c_char) -> c_int {
    let mut file: *mut objtool_file;
    let mut ret: c_int;

    let options = [
        OPT_STRING(
            0,
            c"debug-checksum".as_ptr(),
            &mut opts.debug_checksum,
            c"syms".as_ptr(),
            c"enable checksum debug output".as_ptr(),
        ),
        OPT_BOOLEAN(
            0,
            c"dry-run".as_ptr(),
            &mut opts.dryrun,
            c"don't write modifications".as_ptr(),
        ),
        OPT_END(),
    ];

    let argc = parse_options(argc, argv, options.as_ptr(), KLP_CHECKSUM_USAGE.as_ptr(), 0);
    if argc != 1 {
        usage_with_options(KLP_CHECKSUM_USAGE.as_ptr(), options.as_ptr());
    }

    opts.checksum = true;

    objname = *argv;

    file = objtool_open_read(objname);
    if file.is_null() {
        return 1;
    }

    ret = decode_file(file);
    if ret != 0 {
        free_insns(file);
        return ret;
    }

    ret = calculate_checksums(file);
    if ret != 0 {
        free_insns(file);
        return ret;
    }

    ret = create_sym_checksum_section(file);

    free_insns(file);

    if ret != 0 {
        return ret;
    }

    if !opts.dryrun && (*(*file).elf).changed && elf_write((*file).elf) != 0 {
        return 1;
    }

    elf_close((*file).elf)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
