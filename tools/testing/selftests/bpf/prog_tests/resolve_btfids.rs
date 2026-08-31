// SPDX-License-Identifier: GPL-2.0

// C dependencies translated as external declarations:
// <linux/err.h>, <string.h>, <bpf/btf.h>, <bpf/libbpf.h>,
// <linux/btf.h>, <linux/kernel.h>, <linux/btf_ids.h>, "test_progs.h"
// Original C defined CONFIG_DEBUG_INFO_BTF before including <linux/btf_ids.h>.

use core::ffi::{c_char, c_int, c_uint};

const BTF_DATA_FILE: &[u8] = b"resolve_btfids.test.o.BTF\0";

const DECL_TAG_FASTCALL: &[u8] = b"bpf_fastcall\0";
const DECL_TAG_KFUNC: &[u8] = b"bpf_kfunc\0";
const TYPE_ATTR_ARENA: &[u8] = b"address_space(1)\0";

const fn ARENA_ARG(n: u32) -> u32 {
    1_u32 << n
}

const KF_FASTCALL: u32 = 1 << 12;
const KF_ARENA_RET: u32 = 1 << 13;
const KF_ARENA_ARG1: u32 = 1 << 14;
const KF_ARENA_ARG2: u32 = 1 << 15;

type __u32 = u32;
type u32_ = u32;
type s32 = i32;

#[repr(C)]
struct btf {
    _private: [u8; 0],
}

#[repr(C)]
struct btf_type {
    name_off: __u32,
    info: __u32,
    size: __u32,
    type_: __u32,
}

#[repr(C)]
struct btf_param {
    name_off: __u32,
    type_: __u32,
}

#[repr(C)]
struct btf_decl_tag {
    component_idx: s32,
}

#[repr(C)]
struct btf_id_pair {
    id: __u32,
    flags: __u32,
}

#[repr(C)]
struct btf_id_set {
    cnt: __u32,
    ids: [__u32; 0],
}

#[repr(C)]
struct btf_id_set8 {
    cnt: __u32,
    flags: __u32,
    pairs: [btf_id_pair; 0],
}

#[repr(C)]
struct symbol {
    name: *const c_char,
    type_: c_int,
    id: c_int,
}

#[repr(C)]
struct kfunc_symbol {
    name: *const c_char,
    id: s32,
    flags: u32_,
    arena_args: u32_,
    arena_ret: bool,
}

const BTF_KIND_UNKN: c_int = 0;
const BTF_KIND_STRUCT: c_int = 4;
const BTF_KIND_UNION: c_int = 5;
const BTF_KIND_TYPEDEF: c_int = 8;
const BTF_KIND_FUNC: c_int = 12;
const BTF_SET8_KFUNCS: __u32 = 1;

static mut test_symbols: [symbol; 7] = [
    symbol {
        name: b"unused\0".as_ptr() as *const c_char,
        type_: BTF_KIND_UNKN,
        id: 0,
    },
    symbol {
        name: b"S\0".as_ptr() as *const c_char,
        type_: BTF_KIND_TYPEDEF,
        id: -1,
    },
    symbol {
        name: b"T\0".as_ptr() as *const c_char,
        type_: BTF_KIND_TYPEDEF,
        id: -1,
    },
    symbol {
        name: b"U\0".as_ptr() as *const c_char,
        type_: BTF_KIND_TYPEDEF,
        id: -1,
    },
    symbol {
        name: b"S\0".as_ptr() as *const c_char,
        type_: BTF_KIND_STRUCT,
        id: -1,
    },
    symbol {
        name: b"U\0".as_ptr() as *const c_char,
        type_: BTF_KIND_UNION,
        id: -1,
    },
    symbol {
        name: b"func\0".as_ptr() as *const c_char,
        type_: BTF_KIND_FUNC,
        id: -1,
    },
];

static mut kfunc_symbols: [kfunc_symbol; 7] = [
    kfunc_symbol {
        name: b"kfunc_a\0".as_ptr() as *const c_char,
        id: -1,
        flags: 0,
        arena_args: 0,
        arena_ret: false,
    },
    kfunc_symbol {
        name: b"kfunc_b\0".as_ptr() as *const c_char,
        id: -1,
        flags: KF_FASTCALL,
        arena_args: 0,
        arena_ret: false,
    },
    kfunc_symbol {
        name: b"kfunc_c\0".as_ptr() as *const c_char,
        id: -1,
        flags: KF_ARENA_RET | KF_ARENA_ARG1 | KF_ARENA_ARG2,
        arena_args: ARENA_ARG(0) | ARENA_ARG(1),
        arena_ret: true,
    },
    kfunc_symbol {
        name: b"kfunc_d\0".as_ptr() as *const c_char,
        id: -1,
        flags: KF_ARENA_ARG2,
        arena_args: ARENA_ARG(1),
        arena_ret: false,
    },
    kfunc_symbol {
        name: b"kfunc_e\0".as_ptr() as *const c_char,
        id: -1,
        flags: 0,
        arena_args: ARENA_ARG(0) | ARENA_ARG(1) | ARENA_ARG(2) | ARENA_ARG(3) | ARENA_ARG(4),
        arena_ret: false,
    },
    kfunc_symbol {
        name: b"kfunc_f\0".as_ptr() as *const c_char,
        id: -1,
        flags: 0,
        arena_args: ARENA_ARG(1),
        arena_ret: false,
    },
    kfunc_symbol {
        name: b"kfunc_g\0".as_ptr() as *const c_char,
        id: -1,
        flags: KF_ARENA_RET,
        arena_args: ARENA_ARG(0) | ARENA_ARG(1),
        arena_ret: true,
    },
];

// Align the .BTF_ids section to 4 bytes.
// Original C used inline asm:
// .pushsection BTF_IDS_SECTION ,"a";
// .balign 4, 0;
// .popsection;

/*
 * test_list_local, test_set and test_kfunc_set are .local symbols placed
 * in .BTF_ids by inline asm, and are read here directly by C name. To the
 * compiler they are plain, default-visibility extern objects.
 *
 * When test_progs is linked as a position-independent executable (PIE),
 * taking the address of such an extern is routed through the GOT. The
 * GNU assembler on aarch64 unconditionally converts references to .local
 * symbols into section + addend form (".BTF_ids + <offset>"), but a GOT
 * slot cannot carry an addend (the AArch64 ELF spec mandates zero), so
 * the linker resolves it to the .BTF_ids base.
 *
 * Mark them hidden so the compiler treats them as non-interposable and
 * emits a direct, addend-preserving PC-relative access instead of a GOT
 * load, in both PIE and non-PIE builds. test_list_global is .globl and
 * not affected, so it is left at default visibility.
 */
// Original C emitted these through BTF_ID_LIST/BTF_SET/BTF_KFUNCS macros in
// .BTF_ids with hidden visibility:
// BTF_ID_LIST(test_list_local): unused, typedef S, typedef T, typedef U,
// struct S, union U, func func.
// BTF_SET_START(test_set): typedef S, typedef T, typedef U, struct S,
// union U, func func.
// BTF_KFUNCS_START(test_kfunc_set): kfunc_a, kfunc_b(KF_FASTCALL),
// kfunc_c(KF_ARENA_RET | KF_ARENA_ARG1 | KF_ARENA_ARG2),
// kfunc_d(KF_ARENA_ARG2), kfunc_e, kfunc_f, kfunc_g(KF_ARENA_RET).
// Same kfuncs in reverse declaration order, so resolve_btfids has to
// actually sort at least one of the two sets.
// BTF_KFUNCS_START(test_kfunc_set_rev): kfunc_g(KF_ARENA_RET), kfunc_f,
// kfunc_e, kfunc_d(KF_ARENA_ARG2),
// kfunc_c(KF_ARENA_RET | KF_ARENA_ARG1 | KF_ARENA_ARG2),
// kfunc_b(KF_FASTCALL), kfunc_a.

unsafe extern "C" {
    #[link_name = "test_list_local"]
    static test_list_local: [__u32; 0];
    #[link_name = "test_set"]
    static test_set: btf_id_set;
    #[link_name = "test_kfunc_set"]
    static test_kfunc_set: btf_id_set8;
    #[link_name = "test_kfunc_set_rev"]
    static test_kfunc_set_rev: btf_id_set8;

    static test_list_global: [__u32; 0];

    fn btf__parse_raw(path: *const c_char) -> *mut btf;
    fn btf__free(btf: *mut btf);
    fn btf__type_by_id(btf: *const btf, type_id: __u32) -> *const btf_type;
    fn btf__name_by_offset(btf: *const btf, offset: __u32) -> *const c_char;
    fn btf__type_cnt(btf: *const btf) -> __u32;
    fn btf_is_func(t: *const btf_type) -> bool;
    fn btf_is_decl_tag(t: *const btf_type) -> bool;
    fn btf_is_ptr(t: *const btf_type) -> bool;
    fn btf_is_type_tag(t: *const btf_type) -> bool;
    fn btf_kflag(t: *const btf_type) -> bool;
    fn btf_is_func_proto(t: *const btf_type) -> bool;
    fn btf_decl_tag(t: *const btf_type) -> *const btf_decl_tag;
    fn btf_params(t: *const btf_type) -> *const btf_param;
    fn btf_vlen(t: *const btf_type) -> __u32;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const core::ffi::c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: u64, expected: u64, name: *const c_char) -> bool;
    fn ASSERT_TRUE(actual: bool, name: *const c_char) -> bool;
    fn ASSERT_LE(actual: u64, expected: u64, name: *const c_char) -> bool;
}

const fn BTF_INFO_KIND(info: __u32) -> c_int {
    ((info >> 24) & 0x1f) as c_int
}

unsafe fn __resolve_symbol(btf: *mut btf, type_id: c_int) -> c_int {
    let type_: *const btf_type;
    let str_: *const c_char;
    let mut i: c_uint;

    type_ = btf__type_by_id(btf, type_id as __u32);
    if !ASSERT_OK_PTR(type_ as *const core::ffi::c_void, b"btf__type_by_id\0".as_ptr() as *const c_char) {
        return -1;
    }

    str_ = btf__name_by_offset(btf, (*type_).name_off);

    i = 0;
    while (i as usize) < test_symbols.len() {
        if test_symbols[i as usize].id >= 0 {
            i += 1;
            continue;
        }

        if BTF_INFO_KIND((*type_).info) != test_symbols[i as usize].type_ {
            i += 1;
            continue;
        }

        if strcmp(str_, test_symbols[i as usize].name) == 0 {
            test_symbols[i as usize].id = type_id;
        }
        i += 1;
    }

    if !btf_is_func(type_) {
        return 0;
    }

    i = 0;
    while (i as usize) < kfunc_symbols.len() {
        if kfunc_symbols[i as usize].id >= 0 {
            i += 1;
            continue;
        }
        if strcmp(str_, kfunc_symbols[i as usize].name) == 0 {
            kfunc_symbols[i as usize].id = type_id;
        }
        i += 1;
    }

    0
}

unsafe fn resolve_symbols(btf: *mut btf) -> c_int {
    let nr: __u32 = btf__type_cnt(btf);
    let mut type_id: c_int;

    type_id = 1;
    while type_id < nr as c_int {
        if __resolve_symbol(btf, type_id) != 0 {
            return -1;
        }
        type_id += 1;
    }
    0
}

unsafe fn btf_has_decl_tag(btf: *mut btf, tag_name: *const c_char, target_id: s32) -> bool {
    let mut t: *const btf_type;
    let mut name: *const c_char;
    let nr: c_int;
    let mut id: c_int;

    nr = btf__type_cnt(btf) as c_int;
    id = 1;
    while id < nr {
        t = btf__type_by_id(btf, id as __u32);
        if !btf_is_decl_tag(t) {
            id += 1;
            continue;
        }
        if (*t).type_ != target_id as __u32 {
            id += 1;
            continue;
        }
        if (*btf_decl_tag(t)).component_idx != -1 {
            id += 1;
            continue;
        }
        name = btf__name_by_offset(btf, (*t).name_off);
        if strcmp(name, tag_name) == 0 {
            return true;
        }
        id += 1;
    }
    false
}

unsafe fn check_kfunc_set(set: *mut btf_id_set8) {
    let mut i: c_uint;
    let mut j: c_uint;

    ASSERT_EQ((*set).flags as u64, BTF_SET8_KFUNCS as u64, b"kfunc_set_flags\0".as_ptr() as *const c_char);
    ASSERT_EQ((*set).cnt as u64, kfunc_symbols.len() as u64, b"kfunc_set_cnt\0".as_ptr() as *const c_char);

    i = 0;
    while i < (*set).cnt {
        j = 0;
        while (j as usize) < kfunc_symbols.len() {
            let pair = (*set).pairs.as_ptr().add(i as usize);
            if kfunc_symbols[j as usize].id == (*pair).id as s32 {
                ASSERT_EQ(
                    (*pair).flags as u64,
                    kfunc_symbols[j as usize].flags as u64,
                    b"kfunc_flags_check\0".as_ptr() as *const c_char,
                );
                break;
            }
            j += 1;
        }

        ASSERT_TRUE((j as usize) < kfunc_symbols.len(), b"kfunc_id_found\0".as_ptr() as *const c_char);

        if i > 0 {
            let prev = (*set).pairs.as_ptr().add((i - 1) as usize);
            let cur = (*set).pairs.as_ptr().add(i as usize);
            ASSERT_LE((*prev).id as u64, (*cur).id as u64, b"kfunc_sort_check\0".as_ptr() as *const c_char);
        }
        i += 1;
    }
}

/* True if @id is PTR -> TYPE_TAG(kflag=1, "address_space(1)") -> pointee */
unsafe fn is_arena_tagged_ptr(btf: *mut btf, id: __u32) -> bool {
    let ptr: *const btf_type;
    let tag: *const btf_type;
    let name: *const c_char;

    ptr = btf__type_by_id(btf, id);
    if !btf_is_ptr(ptr) {
        return false;
    }
    tag = btf__type_by_id(btf, (*ptr).type_);
    if !btf_is_type_tag(tag) || !btf_kflag(tag) {
        return false;
    }
    name = btf__name_by_offset(btf, (*tag).name_off);
    strcmp(name, TYPE_ATTR_ARENA.as_ptr() as *const c_char) == 0
}

#[no_mangle]
pub unsafe extern "C" fn test_resolve_btfids() {
    let mut test_list: *const __u32;
    let test_lists: [*const __u32; 2] = [test_list_local.as_ptr(), test_list_global.as_ptr()];
    let mut i: c_uint;
    let mut j: c_uint;
    let btf: *mut btf;

    btf = btf__parse_raw(BTF_DATA_FILE.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(btf as *const core::ffi::c_void, b"btf_parse\0".as_ptr() as *const c_char) {
        return;
    }

    if resolve_symbols(btf) != 0 {
        btf__free(btf);
        return;
    }

    /*
     * Check BTF_ID_LIST(test_list_local) and
     * BTF_ID_LIST_GLOBAL(test_list_global) IDs
     */
    j = 0;
    while (j as usize) < test_lists.len() {
        test_list = test_lists[j as usize];
        i = 0;
        while (i as usize) < test_symbols.len() {
            ASSERT_EQ(
                *test_list.add(i as usize) as u64,
                test_symbols[i as usize].id as u64,
                test_symbols[i as usize].name,
            );
            i += 1;
        }
        j += 1;
    }

    /* Check BTF_SET_START(test_set) IDs */
    i = 0;
    while i < test_set.cnt {
        let mut found: bool = false;

        j = 0;
        while (j as usize) < test_symbols.len() {
            let ids = test_set.ids.as_ptr();
            if test_symbols[j as usize].id != *ids.add(i as usize) as c_int {
                j += 1;
                continue;
            }
            found = true;
            break;
        }

        if !ASSERT_TRUE(found, b"id_in_test_symbols\0".as_ptr() as *const c_char) {
            break;
        }

        if i > 0 {
            let ids = test_set.ids.as_ptr();
            ASSERT_LE(
                *ids.add((i - 1) as usize) as u64,
                *ids.add(i as usize) as u64,
                b"sort_check\0".as_ptr() as *const c_char,
            );
        }
        i += 1;
    }

    check_kfunc_set(&test_kfunc_set as *const btf_id_set8 as *mut btf_id_set8);
    check_kfunc_set(&test_kfunc_set_rev as *const btf_id_set8 as *mut btf_id_set8);

    /* Check resolve_btfids emitted a bpf_kfunc decl_tag for each kfunc */
    i = 0;
    while (i as usize) < kfunc_symbols.len() {
        ASSERT_TRUE(
            btf_has_decl_tag(
                btf,
                DECL_TAG_KFUNC.as_ptr() as *const c_char,
                kfunc_symbols[i as usize].id,
            ),
            kfunc_symbols[i as usize].name,
        );
        i += 1;
    }

    /* Check resolve_btfids emitted bpf_fastcall for KF_FASTCALL kfuncs */
    i = 0;
    while (i as usize) < kfunc_symbols.len() {
        if (kfunc_symbols[i as usize].flags & KF_FASTCALL) != 0 {
            ASSERT_TRUE(
                btf_has_decl_tag(
                    btf,
                    DECL_TAG_FASTCALL.as_ptr() as *const c_char,
                    kfunc_symbols[i as usize].id,
                ),
                kfunc_symbols[i as usize].name,
            );
        }
        i += 1;
    }

    /*
     * Check resolve_btfids wrapped exactly the arena-flagged or suffixed
     * return/args with the address_space(1) type attribute, and left other
     * pointers/returns untouched.
     */
    i = 0;
    while (i as usize) < kfunc_symbols.len() {
        let fn_: *const btf_type;
        let proto: *const btf_type;
        let params: *const btf_param;
        let name: *const c_char = kfunc_symbols[i as usize].name;
        let arena_args: u32_ = kfunc_symbols[i as usize].arena_args;
        let nr: __u32;

        fn_ = btf__type_by_id(btf, kfunc_symbols[i as usize].id as __u32);
        if !ASSERT_TRUE(btf_is_func(fn_), name) {
            i += 1;
            continue;
        }
        proto = btf__type_by_id(btf, (*fn_).type_);
        if !ASSERT_TRUE(btf_is_func_proto(proto), name) {
            i += 1;
            continue;
        }
        params = btf_params(proto);
        nr = btf_vlen(proto);

        ASSERT_EQ(
            is_arena_tagged_ptr(btf, (*proto).type_) as u64,
            kfunc_symbols[i as usize].arena_ret as u64,
            name,
        );
        j = 0;
        while j < nr {
            ASSERT_EQ(
                is_arena_tagged_ptr(btf, (*params.add(j as usize)).type_) as u64,
                ((arena_args & ARENA_ARG(j)) != 0) as u64,
                name,
            );
            j += 1;
        }
        i += 1;
    }

    btf__free(btf);
}
