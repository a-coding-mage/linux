// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/tests/vmlinux-kallsyms.c.
// Dependencies from the original C includes are represented as external Rust
// declarations; macro-provided iteration and suite registration are preserved
// in comments where this isolated file cannot map them directly.

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

type bool_ = bool;
type u64 = u64;
type s64 = i64;

const TEST_FAIL: c_int = -1;
const TEST_SKIP: c_int = 2;
const HOST_KERNEL_ID: c_int = 0;

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_root {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct maps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    pub kmaps: *mut maps,
    pub vmlinux_map: *mut map,
}

#[repr(C)]
pub struct symbol {
    pub rb_node: rb_node,
    pub start: u64,
    pub end: u64,
    pub name: *const c_char,
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
struct test__vmlinux_matches_kallsyms_cb_args {
    kallsyms: machine,
    vmlinux_map: *mut map,
    header_printed: bool_,
}

unsafe extern "C" {
    static mut page_size: usize;
    static mut verbose: c_int;
    static mut stderr: *mut c_void;

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn toupper(c: c_int) -> c_int;
    fn llabs(j: i64) -> i64;

    fn pr_info(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);

    fn map__unmap_ip(map: *mut map, ip: u64) -> u64;
    fn map__dso(map: *mut map) -> *mut dso;
    fn map__set_priv(map: *mut map);
    fn map__priv(map: *mut map) -> bool_;
    fn map__put(map: *mut map);
    fn map__fprintf(map: *mut map, fp: *mut c_void);
    fn map__start(map: *mut map) -> u64;
    fn map__end(map: *mut map) -> u64;
    fn map__pgoff(map: *mut map) -> u64;

    fn maps__find_by_name(maps: *mut maps, name: *const c_char) -> *mut map;
    fn maps__find(maps: *mut maps, ip: u64) -> *mut map;
    fn maps__for_each_map(
        maps: *mut maps,
        cb: unsafe extern "C" fn(*mut map, *mut c_void) -> c_int,
        data: *mut c_void,
    );

    fn dso__kernel(dso: *mut dso) -> bool_;
    fn dso__short_name(dso: *mut dso) -> *const c_char;
    fn dso__name(dso: *mut dso) -> *const c_char;

    fn machine__init(machine: *mut machine, root_dir: *const c_char, pid: c_int) -> c_int;
    fn machine__exit(machine: *mut machine);
    fn machine__kernel_maps(machine: *mut machine) -> *mut maps;
    fn machine__create_kernel_maps(machine: *mut machine) -> c_int;
    fn machine__load_kallsyms(machine: *mut machine, filename: *const c_char) -> c_int;
    fn machine__kernel_map(machine: *mut machine) -> *mut map;
    fn machine__load_vmlinux_path(machine: *mut machine) -> c_int;
    fn machine__find_kernel_symbol(
        machine: *mut machine,
        addr: u64,
        mapp: *mut *mut map,
    ) -> *mut symbol;
    fn machine__find_kernel_symbol_by_name(
        machine: *mut machine,
        name: *const c_char,
        mapp: *mut *mut map,
    ) -> *mut symbol;

    fn arch__compare_symbol_names(namea: *const c_char, nameb: *const c_char) -> c_int;
    fn symbol__type(sym: *mut symbol) -> c_char;
}

unsafe fn is_ignored_symbol(name: *const c_char, type_: c_char) -> bool_ {
    /* Symbol names that exactly match to the following are ignored.*/
    static IGNORED_SYMBOLS: [*const c_char; 9] = [
        /*
         * Symbols which vary between passes. Passes 1 and 2 must have
         * identical symbol lists. The kallsyms_* symbols below are
         * only added after pass 1, they would be included in pass 2
         * when --all-symbols is specified so exclude them to get a
         * stable symbol list.
         */
        b"kallsyms_offsets\0".as_ptr() as *const c_char,
        b"kallsyms_num_syms\0".as_ptr() as *const c_char,
        b"kallsyms_names\0".as_ptr() as *const c_char,
        b"kallsyms_markers\0".as_ptr() as *const c_char,
        b"kallsyms_token_table\0".as_ptr() as *const c_char,
        b"kallsyms_token_index\0".as_ptr() as *const c_char,
        /* Exclude linker generated symbols which vary between passes */
        b"_SDA_BASE_\0".as_ptr() as *const c_char,  /* ppc */
        b"_SDA2_BASE_\0".as_ptr() as *const c_char, /* ppc */
        ptr::null(),
    ];

    /* Symbol names that begin with the following are ignored.*/
    static IGNORED_PREFIXES: [*const c_char; 14] = [
        b"$\0".as_ptr() as *const c_char,              /* local symbols for ARM, MIPS, etc. */
        b".L\0".as_ptr() as *const c_char,             /* local labels, .LBB,.Ltmpxxx,.L__unnamed_xx,.LASANPC, etc. */
        b"__crc_\0".as_ptr() as *const c_char,         /* modversions */
        b"__efistub_\0".as_ptr() as *const c_char,     /* arm64 EFI stub namespace */
        b"__kvm_nvhe_$\0".as_ptr() as *const c_char,   /* arm64 local symbols in non-VHE KVM namespace */
        b"__kvm_nvhe_.L\0".as_ptr() as *const c_char,  /* arm64 local symbols in non-VHE KVM namespace */
        b"__AArch64ADRPThunk_\0".as_ptr() as *const c_char, /* arm64 lld */
        b"__ARMV5PILongThunk_\0".as_ptr() as *const c_char, /* arm lld */
        b"__ARMV7PILongThunk_\0".as_ptr() as *const c_char,
        b"__ThumbV7PILongThunk_\0".as_ptr() as *const c_char,
        b"__LA25Thunk_\0".as_ptr() as *const c_char,   /* mips lld */
        b"__microLA25Thunk_\0".as_ptr() as *const c_char,
        ptr::null(),
        ptr::null(),
    ];

    /* Symbol names that end with the following are ignored.*/
    static IGNORED_SUFFIXES: [*const c_char; 4] = [
        b"_from_arm\0".as_ptr() as *const c_char,   /* arm */
        b"_from_thumb\0".as_ptr() as *const c_char, /* arm */
        b"_veneer\0".as_ptr() as *const c_char,     /* arm */
        ptr::null(),
    ];

    /* Symbol names that contain the following are ignored.*/
    static IGNORED_MATCHES: [*const c_char; 4] = [
        b".long_branch.\0".as_ptr() as *const c_char, /* ppc stub */
        b".plt_branch.\0".as_ptr() as *const c_char,  /* ppc stub */
        ptr::null(),
        ptr::null(),
    ];

    let mut i = 0usize;
    while !IGNORED_SYMBOLS[i].is_null() {
        if strcmp(name, IGNORED_SYMBOLS[i]) == 0 {
            return true;
        }
        i += 1;
    }

    i = 0;
    while !IGNORED_PREFIXES[i].is_null() {
        if strncmp(name, IGNORED_PREFIXES[i], strlen(IGNORED_PREFIXES[i])) == 0 {
            return true;
        }
        i += 1;
    }

    i = 0;
    while !IGNORED_SUFFIXES[i].is_null() {
        let l = strlen(name) as isize - strlen(IGNORED_SUFFIXES[i]) as isize;

        if l >= 0 && strcmp(name.offset(l), IGNORED_SUFFIXES[i]) == 0 {
            return true;
        }
        i += 1;
    }

    i = 0;
    while !IGNORED_MATCHES[i].is_null() {
        if !strstr(name, IGNORED_MATCHES[i]).is_null() {
            return true;
        }
        i += 1;
    }

    if type_ == b'U' as c_char || type_ == b'u' as c_char {
        return true;
    }
    /* exclude debugging symbols */
    if type_ == b'N' as c_char || type_ == b'n' as c_char {
        return true;
    }

    if toupper(type_ as c_int) == b'A' as c_int {
        /* Keep these useful absolute symbols */
        if strcmp(name, b"__kernel_syscall_via_break\0".as_ptr() as *const c_char) != 0
            && strcmp(name, b"__kernel_syscall_via_epc\0".as_ptr() as *const c_char) != 0
            && strcmp(name, b"__kernel_sigtramp\0".as_ptr() as *const c_char) != 0
            && strcmp(name, b"__gp\0".as_ptr() as *const c_char) != 0
        {
            return true;
        }
    }

    false
}

unsafe extern "C" fn test__vmlinux_matches_kallsyms_cb1(
    map_: *mut map,
    data: *mut c_void,
) -> c_int {
    let args = data as *mut test__vmlinux_matches_kallsyms_cb_args;
    let dso = map__dso(map_);
    /*
     * If it is the kernel, kallsyms is always "[kernel.kallsyms]", while
     * the kernel will have the path for the vmlinux file being used, so use
     * the short name, less descriptive but the same ("[kernel]" in both
     * cases.
     */
    let pair = maps__find_by_name(
        (*args).kallsyms.kmaps,
        if dso__kernel(dso) {
            dso__short_name(dso)
        } else {
            dso__name(dso)
        },
    );

    if !pair.is_null() {
        map__set_priv(pair);
        map__put(pair);
    } else {
        if !(*args).header_printed {
            pr_info(b"WARN: Maps only in vmlinux:\n\0".as_ptr() as *const c_char);
            (*args).header_printed = true;
        }
        map__fprintf(map_, stderr);
    }
    0
}

unsafe extern "C" fn test__vmlinux_matches_kallsyms_cb2(
    map_: *mut map,
    data: *mut c_void,
) -> c_int {
    let args = data as *mut test__vmlinux_matches_kallsyms_cb_args;
    let mem_start = map__unmap_ip((*args).vmlinux_map, map__start(map_));
    let mem_end = map__unmap_ip((*args).vmlinux_map, map__end(map_));

    let pair = maps__find((*args).kallsyms.kmaps, mem_start);

    if !pair.is_null() && !map__priv(pair) && map__start(pair) == mem_start {
        let dso = map__dso(map_);

        if !(*args).header_printed {
            pr_info(
                b"WARN: Maps in vmlinux with a different name in kallsyms:\n\0".as_ptr()
                    as *const c_char,
            );
            (*args).header_printed = true;
        }

        pr_info(
            b"WARN: %lx-%lx %lx %s in kallsyms as\0".as_ptr() as *const c_char,
            map__start(map_),
            map__end(map_),
            map__pgoff(map_),
            dso__name(dso),
        );
        if mem_end != map__end(pair) {
            pr_info(
                b":\nWARN: *%lx-%lx %lx\0".as_ptr() as *const c_char,
                map__start(pair),
                map__end(pair),
                map__pgoff(pair),
            );
        }
        pr_info(b" %s\n\0".as_ptr() as *const c_char, dso__name(dso));
        map__set_priv(pair);
    }
    map__put(pair);
    0
}

unsafe extern "C" fn test__vmlinux_matches_kallsyms_cb3(
    map_: *mut map,
    data: *mut c_void,
) -> c_int {
    let args = data as *mut test__vmlinux_matches_kallsyms_cb_args;

    if !map__priv(map_) {
        if !(*args).header_printed {
            pr_info(b"WARN: Maps only in kallsyms:\n\0".as_ptr() as *const c_char);
            (*args).header_printed = true;
        }
        map__fprintf(map_, stderr);
    }
    0
}

unsafe extern "C" fn test__vmlinux_matches_kallsyms(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut err: c_int = TEST_FAIL;
    let mut nd: *mut rb_node = ptr::null_mut();
    let mut sym: *mut symbol = ptr::null_mut();
    let mut kallsyms_map: *mut map;
    let mut vmlinux: machine = core::mem::zeroed();
    let mut maps: *mut maps;
    let mut mem_start: u64;
    let mut mem_end: u64;
    let mut args: test__vmlinux_matches_kallsyms_cb_args = core::mem::zeroed();

    /*
     * Step 1:
     *
     * Init the machines that will hold kernel, modules obtained from
     * both vmlinux + .ko files and from /proc/kallsyms split by modules.
     */
    if machine__init(
        &mut args.kallsyms,
        b"\0".as_ptr() as *const c_char,
        HOST_KERNEL_ID,
    ) != 0
    {
        goto_out(&mut args, &mut vmlinux, err)
    } else if machine__init(&mut vmlinux, b"\0".as_ptr() as *const c_char, HOST_KERNEL_ID) != 0 {
        goto_out(&mut args, &mut vmlinux, err)
    } else {
        maps = machine__kernel_maps(&mut vmlinux);

        /*
         * Step 2:
         *
         * Create the kernel maps for kallsyms and the DSO where we will then
         * load /proc/kallsyms. Also create the modules maps from /proc/modules
         * and find the .ko files that match them in /lib/modules/`uname -r`/.
         */
        if machine__create_kernel_maps(&mut args.kallsyms) < 0 {
            pr_debug(b"machine__create_kernel_maps failed\0".as_ptr() as *const c_char);
            err = TEST_SKIP;
            goto_out(&mut args, &mut vmlinux, err)
        } else {
            /*
             * Step 3:
             *
             * Load and split /proc/kallsyms into multiple maps, one per module.
             * Do not use kcore, as this test was designed before kcore support
             * and has parts that only make sense if using the non-kcore code.
             * XXX: extend it to stress the kcorre code as well, hint: the list
             * of modules extracted from /proc/kcore, in its current form, can't
             * be compacted against the list of modules found in the "vmlinux"
             * code and with the one got from /proc/modules from the "kallsyms" code.
             */
            if machine__load_kallsyms(
                &mut args.kallsyms,
                b"/proc/kallsyms\0".as_ptr() as *const c_char,
            ) <= 0
            {
                pr_debug(b"machine__load_kallsyms failed\0".as_ptr() as *const c_char);
                err = TEST_SKIP;
                goto_out(&mut args, &mut vmlinux, err)
            } else {
                /*
                 * Step 4:
                 *
                 * kallsyms will be internally on demand sorted by name so that we can
                 * find the reference relocation * symbol, i.e. the symbol we will use
                 * to see if the running kernel was relocated by checking if it has the
                 * same value in the vmlinux file we load.
                 */
                kallsyms_map = machine__kernel_map(&mut args.kallsyms);

                /*
                 * Step 5:
                 *
                 * Now repeat step 2, this time for the vmlinux file we'll auto-locate.
                 */
                if machine__create_kernel_maps(&mut vmlinux) < 0 {
                    pr_info(b"machine__create_kernel_maps failed\0".as_ptr() as *const c_char);
                    goto_out(&mut args, &mut vmlinux, err)
                } else {
                    args.vmlinux_map = machine__kernel_map(&mut vmlinux);

                    /*
                     * Step 6:
                     *
                     * Locate a vmlinux file in the vmlinux path that has a buildid that
                     * matches the one of the running kernel.
                     *
                     * While doing that look if we find the ref reloc symbol, if we find it
                     * we'll have its ref_reloc_symbol.unrelocated_addr and then
                     * maps__reloc_vmlinux will notice and set proper ->[un]map_ip routines
                     * to fixup the symbols.
                     */
                    if machine__load_vmlinux_path(&mut vmlinux) <= 0 {
                        pr_info(b"Couldn't find a vmlinux that matches the kernel running on this machine, skipping test\n\0".as_ptr() as *const c_char);
                        err = TEST_SKIP;
                        goto_out(&mut args, &mut vmlinux, err)
                    } else {
                        err = 0;
                        /*
                         * Step 7:
                         *
                         * Now look at the symbols in the vmlinux DSO and check if we find all of them
                         * in the kallsyms dso. For the ones that are in both, check its names and
                         * end addresses too.
                         */
                        /*
                         * Original C:
                         * map__for_each_symbol(args.vmlinux_map, sym, nd) {
                         *     sym = rb_entry(nd, struct symbol, rb_node);
                         *     ...
                         * }
                         *
                         * This isolated Rust translation preserves the body below as the
                         * literal per-symbol operation. The rb_entry and map__for_each_symbol
                         * macro expansion is supplied by external perf infrastructure.
                         */
                        while !nd.is_null() {
                            let mut pair: *mut symbol;
                            let first_pair: *mut symbol;

                            sym = rb_entry_symbol(nd);

                            if (*sym).start == (*sym).end {
                                nd = map__for_each_symbol_next(args.vmlinux_map, nd);
                                continue;
                            }

                            mem_start = map__unmap_ip(args.vmlinux_map, (*sym).start);
                            mem_end = map__unmap_ip(args.vmlinux_map, (*sym).end);

                            first_pair =
                                machine__find_kernel_symbol(&mut args.kallsyms, mem_start, ptr::null_mut());
                            pair = first_pair;

                            if !pair.is_null() && map__unmap_ip(kallsyms_map, (*pair).start) == mem_start {
                                loop {
                                    if arch__compare_symbol_names((*sym).name, (*pair).name) == 0 {
                                        /*
                                         * kallsyms don't have the symbol end, so we
                                         * set that by using the next symbol start - 1,
                                         * in some cases we get this up to a page
                                         * wrong, trace_kmalloc when I was developing
                                         * this code was one such example, 2106 bytes
                                         * off the real size. More than that and we
                                         * _really_ have a problem.
                                         */
                                        let skew: s64 =
                                            mem_end as s64 - map__unmap_ip(kallsyms_map, (*pair).end) as s64;
                                        if llabs(skew) as usize >= page_size {
                                            pr_debug(
                                                b"WARN: %#lx: diff end addr for %s v: %#lx k: %#lx\n\0"
                                                    .as_ptr() as *const c_char,
                                                mem_start,
                                                (*sym).name,
                                                mem_end,
                                                map__unmap_ip(kallsyms_map, (*pair).end),
                                            );
                                        }

                                        /*
                                         * Do not count this as a failure, because we
                                         * could really find a case where it's not
                                         * possible to get proper function end from
                                         * kallsyms.
                                         */
                                        break;
                                    } else {
                                        pair = machine__find_kernel_symbol_by_name(
                                            &mut args.kallsyms,
                                            (*sym).name,
                                            ptr::null_mut(),
                                        );
                                        if !pair.is_null() {
                                            if map__unmap_ip(kallsyms_map, (*pair).start) == mem_start {
                                                continue;
                                            }

                                            pr_debug(
                                                b"WARN: %#lx: diff name v: %s k: %s\n\0".as_ptr()
                                                    as *const c_char,
                                                mem_start,
                                                (*sym).name,
                                                (*pair).name,
                                            );
                                        } else {
                                            pr_debug(
                                                b"WARN: %#lx: diff name v: %s k: %s\n\0".as_ptr()
                                                    as *const c_char,
                                                mem_start,
                                                (*sym).name,
                                                (*first_pair).name,
                                            );
                                        }

                                        break;
                                    }
                                }
                            } else if mem_start == map__end(args.kallsyms.vmlinux_map) {
                                /*
                                 * Ignore aliases to _etext, i.e. to the end of the kernel text area,
                                 * such as __indirect_thunk_end.
                                 */
                            } else if is_ignored_symbol((*sym).name, symbol__type(sym)) {
                                /*
                                 * Ignore hidden symbols, see scripts/kallsyms.c for the details
                                 */
                            } else {
                                pr_debug(
                                    b"ERR : %#lx: %s not on kallsyms\n\0".as_ptr() as *const c_char,
                                    mem_start,
                                    (*sym).name,
                                );
                                err = -1;
                            }

                            nd = map__for_each_symbol_next(args.vmlinux_map, nd);
                        }

                        if verbose <= 0 {
                            goto_out(&mut args, &mut vmlinux, err)
                        } else {
                            args.header_printed = false;
                            maps__for_each_map(
                                maps,
                                test__vmlinux_matches_kallsyms_cb1,
                                &mut args as *mut _ as *mut c_void,
                            );

                            args.header_printed = false;
                            maps__for_each_map(
                                maps,
                                test__vmlinux_matches_kallsyms_cb2,
                                &mut args as *mut _ as *mut c_void,
                            );

                            args.header_printed = false;
                            maps = machine__kernel_maps(&mut args.kallsyms);
                            maps__for_each_map(
                                maps,
                                test__vmlinux_matches_kallsyms_cb3,
                                &mut args as *mut _ as *mut c_void,
                            );

                            goto_out(&mut args, &mut vmlinux, err)
                        }
                    }
                }
            }
        }
    }
}

unsafe fn goto_out(
    args: *mut test__vmlinux_matches_kallsyms_cb_args,
    vmlinux: *mut machine,
    err: c_int,
) -> c_int {
    machine__exit(&mut (*args).kallsyms);
    machine__exit(vmlinux);
    err
}

unsafe extern "C" {
    fn rb_entry_symbol(nd: *mut rb_node) -> *mut symbol;
    fn map__for_each_symbol_next(map: *mut map, nd: *mut rb_node) -> *mut rb_node;
}

/*
 * Original C:
 * DEFINE_SUITE("vmlinux symtab matches kallsyms", vmlinux_matches_kallsyms);
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
