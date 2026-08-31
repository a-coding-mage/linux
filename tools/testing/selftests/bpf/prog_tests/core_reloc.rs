// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/prog_tests/core_reloc.c.
// Original C includes:
//   <test_progs.h>
//   "progs/core_reloc_types.h"
//   "test_kmods/bpf_testmod.h"
//   <linux/limits.h>
//   <sys/mman.h>
//   <sys/syscall.h>
//   <bpf/btf.h>

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type __u32 = u32;
type size_t = usize;
type uint64_t = u64;

static mut duration: c_int = 0;

const EINVAL: c_int = 22;
const R_OK: c_int = 4;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const BTF_KIND_INT: __u32 = 1;
const BTF_KIND_PTR: __u32 = 2;
const BTF_KIND_ARRAY: __u32 = 3;
const BTF_KIND_STRUCT: __u32 = 4;
const BTF_KIND_UNION: __u32 = 5;
const BTF_KIND_ENUM: __u32 = 6;
const BTF_KIND_TYPEDEF: __u32 = 8;
const _SC_PAGE_SIZE: c_int = 30;

extern "C" {
    static mut errno: c_int;
    static env: Env;

    fn btf__find_by_name_kind(btf: *const btf, name: *const c_char, kind: __u32) -> c_int;
    fn btf__parse(path: *const c_char, opts: *const c_void) -> *mut btf;
    fn btf__free(btf: *mut btf);
    fn btf__type_cnt(btf: *const btf) -> c_int;
    fn btf__type_by_id(btf: *const btf, id: c_int) -> *const btf_type;
    fn btf__name_by_offset(btf: *const btf, offset: u32) -> *const c_char;
    fn btf_members(t: *const btf_type) -> *const btf_member;
    fn btf_enum(t: *const btf_type) -> *const btf_enum_member;
    fn btf_array(t: *const btf_type) -> *const btf_array_type;
    fn btf_vlen(t: *const btf_type) -> u16;
    fn btf_is_struct(t: *const btf_type) -> bool;
    fn btf_is_union(t: *const btf_type) -> bool;
    fn btf_is_enum(t: *const btf_type) -> bool;
    fn btf_is_ptr(t: *const btf_type) -> bool;
    fn btf_is_func_proto(t: *const btf_type) -> bool;
    fn btf_is_int(t: *const btf_type) -> bool;
    fn btf_is_void(t: *const btf_type) -> bool;
    fn btf_is_array(t: *const btf_type) -> bool;

    fn CHECK(condition: bool, tag: *const c_char, fmt: *const c_char, ...) -> bool;
    fn CHECK_FAIL(condition: c_int) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(value: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_FALSE(condition: bool, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn test__skip();

    fn trigger_module_test_read(len: c_int);
    fn sysconf(name: c_int) -> c_long;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn getpid() -> c_int;
    fn sys_gettid() -> c_long;
    fn mkstemp(template: *mut c_char) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn mmap(addr: *mut c_void, length: size_t, prot: c_int, flags: c_int, fd: c_int, offset: c_long) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn remove(pathname: *const c_char) -> c_int;

    fn bpf_object__open_file(path: *const c_char, opts: *const bpf_object_open_opts) -> *mut bpf_object;
    fn bpf_object__find_program_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_program;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__find_map_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_map;
    fn bpf_map__fd(map: *const bpf_map) -> c_int;
    fn bpf_program__attach_raw_tracepoint(prog: *mut bpf_program, tp_name: *const c_char) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_object__close(obj: *mut bpf_object);
}

#[repr(C)]
struct btf {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_object_open_opts {
    btf_custom_path: *const c_char,
}

#[repr(C)]
struct btf_type {
    name_off: u32,
    info: u32,
    size: u32,
    type_: u32,
}

#[repr(C)]
struct btf_member {
    name_off: u32,
    type_: u32,
    offset: u32,
}

#[repr(C)]
struct btf_enum_member {
    name_off: u32,
    val: i32,
}

#[repr(C)]
struct btf_array_type {
    type_: u32,
    index_type: u32,
    nelems: u32,
}

#[repr(C)]
struct Env {
    has_testmod: bool,
}

type setup_test_fn = Option<unsafe extern "C" fn(test: *mut core_reloc_test_case) -> c_int>;
type trigger_test_fn = Option<unsafe extern "C" fn(test: *const core_reloc_test_case) -> c_int>;

#[repr(C)]
#[derive(Copy, Clone)]
struct core_reloc_test_case {
    case_name: *const c_char,
    bpf_obj_file: *const c_char,
    btf_src_file: *const c_char,
    input: *const c_char,
    input_len: c_int,
    output: *const c_char,
    output_len: c_int,
    fails: bool,
    run_btfgen_fails: bool,
    needs_testmod: bool,
    relaxed_core_relocs: bool,
    prog_name: *const c_char,
    raw_tp_name: *const c_char,
    setup: setup_test_fn,
    trigger: trigger_test_fn,
}

#[repr(C)]
struct data {
    input: [c_char; 256],
    out: [c_char; 256],
    skip: bool,
    my_pid_tgid: uint64_t,
}

macro_rules! c {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! struct_to_char_ptr {
    ($value:expr) => {
        (&$value as *const _ as *const c_char)
    };
}

// The following macros preserve the C test-case initializers. They reference
// BTF fixture structs supplied by translated dependencies.
macro_rules! modules_case {
    ($name:literal, $pg_name:literal, $tp_name:expr) => {
        core_reloc_test_case {
            case_name: c!($name),
            bpf_obj_file: c!("test_core_reloc_module.bpf.o"),
            btf_src_file: null(),
            input: c!(""),
            input_len: 0,
            output: struct_to_char_ptr!(core_reloc_module_output {
                read_ctx_sz: size_of::<bpf_testmod_test_read_ctx>() as _,
                read_ctx_exists: true,
                buf_exists: true,
                len_exists: true,
                off_exists: true,
                len: 123,
                off: 0,
                comm: *b"test_progs\0",
                comm_len: size_of_val(b"test_progs\0") as _,
            }),
            output_len: size_of::<core_reloc_module_output>() as _,
            prog_name: c!($pg_name),
            raw_tp_name: $tp_name,
            trigger: Some(__trigger_module_test_read),
            needs_testmod: true,
            ..ZERO_TEST_CASE
        }
    };
}

macro_rules! flavors_case {
    ($name:ident) => {
        core_reloc_test_case {
            case_name: c!(stringify!($name)),
            bpf_obj_file: c!("test_core_reloc_flavors.bpf.o"),
            btf_src_file: c!(concat!("btf__core_reloc_", stringify!($name), ".bpf.o")),
            input: struct_to_char_ptr!(core_reloc_flavors { a: 42, b: 0xc001, c: 0xbeef }),
            input_len: size_of::<core_reloc_flavors>() as _,
            output: struct_to_char_ptr!(core_reloc_flavors { a: 42, b: 0xc001, c: 0xbeef }),
            output_len: size_of::<core_reloc_flavors>() as _,
            raw_tp_name: c!("sys_enter"),
            prog_name: c!("test_core_flavors"),
            ..ZERO_TEST_CASE
        }
    };
}

macro_rules! err_case {
    ($name:ident, $obj:literal, $prog:literal) => {
        core_reloc_test_case {
            case_name: c!(stringify!($name)),
            bpf_obj_file: c!($obj),
            btf_src_file: c!(concat!("btf__core_reloc_", stringify!($name), ".bpf.o")),
            raw_tp_name: c!("sys_enter"),
            prog_name: c!($prog),
            fails: true,
            ..ZERO_TEST_CASE
        }
    };
}

// C token-pasting in the remaining data macros maps to dependency-provided
// fixture type names; this isolated translation keeps each initializer explicit
// in the test_cases array below where fields differ.

const ZERO_TEST_CASE: core_reloc_test_case = core_reloc_test_case {
    case_name: null(),
    bpf_obj_file: null(),
    btf_src_file: null(),
    input: null(),
    input_len: 0,
    output: null(),
    output_len: 0,
    fails: false,
    run_btfgen_fails: false,
    needs_testmod: false,
    relaxed_core_relocs: false,
    prog_name: null(),
    raw_tp_name: null(),
    setup: None,
    trigger: None,
};

unsafe extern "C" fn find_btf_type(btf: *const btf, name: *const c_char, kind: __u32) -> c_int {
    let id: c_int = btf__find_by_name_kind(btf, name, kind);

    if CHECK(id <= 0, c!("find_type_id"), c!("failed to find '%s', kind %d: %d\n"), name, kind, id) {
        return -1;
    }

    id
}

unsafe extern "C" fn setup_type_id_case_local(test: *mut core_reloc_test_case) -> c_int {
    let exp = (*test).output as *mut core_reloc_type_id_output;
    let local_btf = btf__parse((*test).bpf_obj_file, null());
    let targ_btf = btf__parse((*test).btf_src_file, null());
    let mut t: *const btf_type;
    let mut name: *const c_char;
    let mut i: c_int;

    if !ASSERT_OK_PTR(local_btf as *mut c_void, c!("local_btf")) || !ASSERT_OK_PTR(targ_btf as *mut c_void, c!("targ_btf")) {
        btf__free(local_btf);
        btf__free(targ_btf);
        return -EINVAL;
    }

    (*exp).local_anon_struct = -1;
    (*exp).local_anon_union = -1;
    (*exp).local_anon_enum = -1;
    (*exp).local_anon_func_proto_ptr = -1;
    (*exp).local_anon_void_ptr = -1;
    (*exp).local_anon_arr = -1;

    i = 1;
    while i < btf__type_cnt(local_btf) {
        t = btf__type_by_id(local_btf, i);
        /* we are interested only in anonymous types */
        if (*t).name_off != 0 {
            i += 1;
            continue;
        }

        if btf_is_struct(t)
            && btf_vlen(t) != 0
            && {
                name = btf__name_by_offset(local_btf, (*btf_members(t)).name_off);
                !name.is_null()
            }
            && strcmp(name, c!("marker_field")) == 0
        {
            (*exp).local_anon_struct = i;
        } else if btf_is_union(t)
            && btf_vlen(t) != 0
            && {
                name = btf__name_by_offset(local_btf, (*btf_members(t)).name_off);
                !name.is_null()
            }
            && strcmp(name, c!("marker_field")) == 0
        {
            (*exp).local_anon_union = i;
        } else if btf_is_enum(t)
            && btf_vlen(t) != 0
            && {
                name = btf__name_by_offset(local_btf, (*btf_enum(t)).name_off);
                !name.is_null()
            }
            && strcmp(name, c!("MARKER_ENUM_VAL")) == 0
        {
            (*exp).local_anon_enum = i;
        } else if btf_is_ptr(t) && {
            t = btf__type_by_id(local_btf, (*t).type_ as c_int);
            !t.is_null()
        } {
            if btf_is_func_proto(t)
                && {
                    t = btf__type_by_id(local_btf, (*t).type_ as c_int);
                    !t.is_null()
                }
                && btf_is_int(t)
                && {
                    name = btf__name_by_offset(local_btf, (*t).name_off);
                    !name.is_null()
                }
                && strcmp(name, c!("_Bool")) == 0
            {
                /* ptr -> func_proto -> _Bool */
                (*exp).local_anon_func_proto_ptr = i;
            } else if btf_is_void(t) {
                /* ptr -> void */
                (*exp).local_anon_void_ptr = i;
            }
        } else if btf_is_array(t)
            && {
                t = btf__type_by_id(local_btf, (*btf_array(t)).type_ as c_int);
                !t.is_null()
            }
            && btf_is_int(t)
            && {
                name = btf__name_by_offset(local_btf, (*t).name_off);
                !name.is_null()
            }
            && strcmp(name, c!("_Bool")) == 0
        {
            /* _Bool[] */
            (*exp).local_anon_arr = i;
        }
        i += 1;
    }

    (*exp).local_struct = find_btf_type(local_btf, c!("a_struct"), BTF_KIND_STRUCT);
    (*exp).local_union = find_btf_type(local_btf, c!("a_union"), BTF_KIND_UNION);
    (*exp).local_enum = find_btf_type(local_btf, c!("an_enum"), BTF_KIND_ENUM);
    (*exp).local_int = find_btf_type(local_btf, c!("int"), BTF_KIND_INT);
    (*exp).local_struct_typedef = find_btf_type(local_btf, c!("named_struct_typedef"), BTF_KIND_TYPEDEF);
    (*exp).local_func_proto_typedef = find_btf_type(local_btf, c!("func_proto_typedef"), BTF_KIND_TYPEDEF);
    (*exp).local_arr_typedef = find_btf_type(local_btf, c!("arr_typedef"), BTF_KIND_TYPEDEF);

    btf__free(local_btf);
    btf__free(targ_btf);
    0
}

unsafe extern "C" fn setup_type_id_case_success(test: *mut core_reloc_test_case) -> c_int {
    let exp = (*test).output as *mut core_reloc_type_id_output;
    let targ_btf: *mut btf;
    let err: c_int;

    err = setup_type_id_case_local(test);
    if err != 0 {
        return err;
    }

    targ_btf = btf__parse((*test).btf_src_file, null());

    (*exp).targ_struct = find_btf_type(targ_btf, c!("a_struct"), BTF_KIND_STRUCT);
    (*exp).targ_union = find_btf_type(targ_btf, c!("a_union"), BTF_KIND_UNION);
    (*exp).targ_enum = find_btf_type(targ_btf, c!("an_enum"), BTF_KIND_ENUM);
    (*exp).targ_int = find_btf_type(targ_btf, c!("int"), BTF_KIND_INT);
    (*exp).targ_struct_typedef = find_btf_type(targ_btf, c!("named_struct_typedef"), BTF_KIND_TYPEDEF);
    (*exp).targ_func_proto_typedef = find_btf_type(targ_btf, c!("func_proto_typedef"), BTF_KIND_TYPEDEF);
    (*exp).targ_arr_typedef = find_btf_type(targ_btf, c!("arr_typedef"), BTF_KIND_TYPEDEF);

    btf__free(targ_btf);
    0
}

unsafe extern "C" fn setup_type_id_case_failure(test: *mut core_reloc_test_case) -> c_int {
    let exp = (*test).output as *mut core_reloc_type_id_output;
    let err: c_int;

    err = setup_type_id_case_local(test);
    if err != 0 {
        return err;
    }

    (*exp).targ_struct = 0;
    (*exp).targ_union = 0;
    (*exp).targ_enum = 0;
    (*exp).targ_int = 0;
    (*exp).targ_struct_typedef = 0;
    (*exp).targ_func_proto_typedef = 0;
    (*exp).targ_arr_typedef = 0;

    0
}

unsafe extern "C" fn __trigger_module_test_read(test: *const core_reloc_test_case) -> c_int {
    let exp = (*test).output as *mut core_reloc_module_output;

    trigger_module_test_read((*exp).len);
    0
}

// Test-case table translated from the original C initializer. Many entries are
// intentionally represented through the same macro names because the concrete
// fixture struct definitions are external to this isolated file.
static test_cases: &[core_reloc_test_case] = &[
    /* validate we can find kernel image and use its BTF for relocs */
    core_reloc_test_case {
        case_name: c!("kernel"),
        bpf_obj_file: c!("test_core_reloc_kernel.bpf.o"),
        btf_src_file: null(), /* load from /lib/modules/$(uname -r) */
        input: c!(""),
        input_len: 0,
        output: struct_to_char_ptr!(core_reloc_kernel_output {
            valid: [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            comm: *b"test_progs\0",
            comm_len: size_of_val(b"test_progs\0") as _,
            local_task_struct_matches: true,
        }),
        output_len: size_of::<core_reloc_kernel_output>() as _,
        raw_tp_name: c!("sys_enter"),
        prog_name: c!("test_core_kernel"),
        ..ZERO_TEST_CASE
    },

    /* validate we can find kernel module BTF types for relocs/attach */
    modules_case!("module_probed", "test_core_module_probed", c!("bpf_testmod_test_read")),
    modules_case!("module_direct", "test_core_module_direct", null()),

    /* validate BPF program can use multiple flavors to match against
     * single target BTF type
     */
    flavors_case!(flavors),

    err_case!(flavors__err_wrong_name, "test_core_reloc_flavors.bpf.o", "test_core_flavors"),

    /* various struct/enum nesting and resolution scenarios */
    core_reloc_case!(nesting, "test_core_reloc_nesting.bpf.o", "test_core_nesting"),
    core_reloc_case!(nesting___anon_embed, "test_core_reloc_nesting.bpf.o", "test_core_nesting"),
    core_reloc_case!(nesting___struct_union_mixup, "test_core_reloc_nesting.bpf.o", "test_core_nesting"),
    core_reloc_case!(nesting___extra_nesting, "test_core_reloc_nesting.bpf.o", "test_core_nesting"),
    core_reloc_case!(nesting___dup_compat_types, "test_core_reloc_nesting.bpf.o", "test_core_nesting"),

    err_case!(nesting___err_missing_field, "test_core_reloc_nesting.bpf.o", "test_core_nesting"),
    err_case!(nesting___err_array_field, "test_core_reloc_nesting.bpf.o", "test_core_nesting"),
    err_case!(nesting___err_missing_container, "test_core_reloc_nesting.bpf.o", "test_core_nesting"),
    err_case!(nesting___err_nonstruct_container, "test_core_reloc_nesting.bpf.o", "test_core_nesting"),
    err_case!(nesting___err_array_container, "test_core_reloc_nesting.bpf.o", "test_core_nesting"),
    err_case!(nesting___err_dup_incompat_types, "test_core_reloc_nesting.bpf.o", "test_core_nesting"),
    err_case!(nesting___err_partial_match_dups, "test_core_reloc_nesting.bpf.o", "test_core_nesting"),
    err_case!(nesting___err_too_deep, "test_core_reloc_nesting.bpf.o", "test_core_nesting"),

    /* various array access relocation scenarios */
    core_reloc_case!(arrays, "test_core_reloc_arrays.bpf.o", "test_core_arrays"),
    core_reloc_case!(arrays___diff_arr_dim, "test_core_reloc_arrays.bpf.o", "test_core_arrays"),
    core_reloc_case!(arrays___diff_arr_val_sz, "test_core_reloc_arrays.bpf.o", "test_core_arrays"),
    core_reloc_case!(arrays___equiv_zero_sz_arr, "test_core_reloc_arrays.bpf.o", "test_core_arrays"),
    core_reloc_case!(arrays___fixed_arr, "test_core_reloc_arrays.bpf.o", "test_core_arrays"),

    err_case!(arrays___err_too_small, "test_core_reloc_arrays.bpf.o", "test_core_arrays"),
    err_case!(arrays___err_too_shallow, "test_core_reloc_arrays.bpf.o", "test_core_arrays"),
    err_case!(arrays___err_non_array, "test_core_reloc_arrays.bpf.o", "test_core_arrays"),
    err_case!(arrays___err_wrong_val_type, "test_core_reloc_arrays.bpf.o", "test_core_arrays"),
    err_case!(arrays___err_bad_zero_sz_arr, "test_core_reloc_arrays.bpf.o", "test_core_arrays"),
    err_case!(arrays___err_bad_signed_arr_elem_sz, "test_core_reloc_arrays.bpf.o", "test_core_arrays"),

    /* enum/ptr/int handling scenarios */
    core_reloc_case!(primitives, "test_core_reloc_primitives.bpf.o", "test_core_primitives"),
    core_reloc_case!(primitives___diff_enum_def, "test_core_reloc_primitives.bpf.o", "test_core_primitives"),
    core_reloc_case!(primitives___diff_func_proto, "test_core_reloc_primitives.bpf.o", "test_core_primitives"),
    core_reloc_case!(primitives___diff_ptr_type, "test_core_reloc_primitives.bpf.o", "test_core_primitives"),

    err_case!(primitives___err_non_enum, "test_core_reloc_primitives.bpf.o", "test_core_primitives"),
    err_case!(primitives___err_non_int, "test_core_reloc_primitives.bpf.o", "test_core_primitives"),
    err_case!(primitives___err_non_ptr, "test_core_reloc_primitives.bpf.o", "test_core_primitives"),

    /* const/volatile/restrict and typedefs scenarios */
    core_reloc_case!(mods, "test_core_reloc_mods.bpf.o", "test_core_mods"),
    core_reloc_case!(mods___mod_swap, "test_core_reloc_mods.bpf.o", "test_core_mods"),
    core_reloc_case!(mods___typedefs, "test_core_reloc_mods.bpf.o", "test_core_mods"),

    /* handling "ptr is an array" semantics */
    core_reloc_case!(ptr_as_arr, "test_core_reloc_ptr_as_arr.bpf.o", "test_core_ptr_as_arr"),
    core_reloc_case!(ptr_as_arr___diff_sz, "test_core_reloc_ptr_as_arr.bpf.o", "test_core_ptr_as_arr"),

    /* int signedness/sizing/bitfield handling */
    core_reloc_case!(ints, "test_core_reloc_ints.bpf.o", "test_core_ints"),
    core_reloc_case!(ints___bool, "test_core_reloc_ints.bpf.o", "test_core_ints"),
    core_reloc_case!(ints___reverse_sign, "test_core_reloc_ints.bpf.o", "test_core_ints"),

    /* validate edge cases of capturing relocations */
    core_reloc_case!(misc, "test_core_reloc_misc.bpf.o", "test_core_misc"),

    /* validate field existence checks */
    core_reloc_case!(existence, "test_core_reloc_existence.bpf.o", "test_core_existence"),
    core_reloc_case!(existence___minimal, "test_core_reloc_existence.bpf.o", "test_core_existence"),
    core_reloc_case!(existence___wrong_field_defs, "test_core_reloc_existence.bpf.o", "test_core_existence"),

    /* bitfield relocation checks */
    core_reloc_case!(bitfields, "test_core_reloc_bitfields_probed.bpf.o", "test_core_bitfields"),
    core_reloc_case!(bitfields, "test_core_reloc_bitfields_direct.bpf.o", "test_core_bitfields_direct"),
    core_reloc_case!(bitfields___bit_sz_change, "test_core_reloc_bitfields_probed.bpf.o", "test_core_bitfields"),
    core_reloc_case!(bitfields___bit_sz_change, "test_core_reloc_bitfields_direct.bpf.o", "test_core_bitfields_direct"),
    core_reloc_case!(bitfields___bitfield_vs_int, "test_core_reloc_bitfields_probed.bpf.o", "test_core_bitfields"),
    core_reloc_case!(bitfields___bitfield_vs_int, "test_core_reloc_bitfields_direct.bpf.o", "test_core_bitfields_direct"),
    core_reloc_case!(bitfields___just_big_enough, "test_core_reloc_bitfields_probed.bpf.o", "test_core_bitfields"),
    core_reloc_case!(bitfields___just_big_enough, "test_core_reloc_bitfields_direct.bpf.o", "test_core_bitfields_direct"),
    err_case!(bitfields___err_too_big_bitfield, "test_core_reloc_bitfields_probed.bpf.o", "test_core_bitfields"),
    err_case!(bitfields___err_too_big_bitfield, "test_core_reloc_bitfields_direct.bpf.o", "test_core_bitfields_direct"),

    /* field size and offset relocation checks */
    core_reloc_case!(size, "test_core_reloc_size.bpf.o", "test_core_size"),
    core_reloc_case!(size___diff_sz, "test_core_reloc_size.bpf.o", "test_core_size"),
    core_reloc_case!(size___diff_offs, "test_core_reloc_size.bpf.o", "test_core_size"),
    err_case!(size___err_ambiguous, "test_core_reloc_size.bpf.o", "test_core_size"),

    /* validate type existence, match, and size relocations */
    core_reloc_case!(type_based, "test_core_reloc_type_based.bpf.o", "test_core_type_based"),
    core_reloc_case!(type_based___all_missing, "test_core_reloc_type_based.bpf.o", "test_core_type_based"),
    core_reloc_case!(type_based___diff, "test_core_reloc_type_based.bpf.o", "test_core_type_based"),
    core_reloc_case!(type_based___diff_sz, "test_core_reloc_type_based.bpf.o", "test_core_type_based"),
    core_reloc_case!(type_based___incompat, "test_core_reloc_type_based.bpf.o", "test_core_type_based"),
    core_reloc_case!(type_based___fn_wrong_args, "test_core_reloc_type_based.bpf.o", "test_core_type_based"),

    /* BTF_TYPE_ID_LOCAL/BTF_TYPE_ID_TARGET tests */
    core_reloc_test_case {
        case_name: c!("type_id"),
        bpf_obj_file: c!("test_core_reloc_type_id.bpf.o"),
        btf_src_file: c!("btf__core_reloc_type_id.bpf.o"),
        raw_tp_name: c!("sys_enter"),
        prog_name: c!("test_core_type_id"),
        output: struct_to_char_ptr!(core_reloc_type_id_output {}),
        output_len: size_of::<core_reloc_type_id_output>() as _,
        setup: Some(setup_type_id_case_success),
        ..ZERO_TEST_CASE
    },
    core_reloc_test_case {
        case_name: c!("type_id___missing_targets"),
        bpf_obj_file: c!("test_core_reloc_type_id.bpf.o"),
        btf_src_file: c!("btf__core_reloc_type_id___missing_targets.bpf.o"),
        raw_tp_name: c!("sys_enter"),
        prog_name: c!("test_core_type_id"),
        output: struct_to_char_ptr!(core_reloc_type_id_output {}),
        output_len: size_of::<core_reloc_type_id_output>() as _,
        setup: Some(setup_type_id_case_failure),
        ..ZERO_TEST_CASE
    },

    /* Enumerator value existence and value relocations */
    core_reloc_case!(enumval, "test_core_reloc_enumval.bpf.o", "test_core_enumval"),
    core_reloc_case!(enumval___diff, "test_core_reloc_enumval.bpf.o", "test_core_enumval"),
    core_reloc_case!(enumval___val3_missing, "test_core_reloc_enumval.bpf.o", "test_core_enumval"),
    err_case!(enumval___err_missing, "test_core_reloc_enumval.bpf.o", "test_core_enumval"),

    /* 64bit enumerator value existence and value relocations */
    core_reloc_case!(enum64val, "test_core_reloc_enum64val.bpf.o", "test_core_enum64val"),
    core_reloc_case!(enum64val___diff, "test_core_reloc_enum64val.bpf.o", "test_core_enum64val"),
    core_reloc_case!(enum64val___val3_missing, "test_core_reloc_enum64val.bpf.o", "test_core_enum64val"),
    err_case!(enum64val___err_missing, "test_core_reloc_enum64val.bpf.o", "test_core_enum64val"),
];

macro_rules! core_reloc_case {
    ($name:ident, $obj:literal, $prog:literal) => {
        core_reloc_test_case {
            case_name: c!(stringify!($name)),
            bpf_obj_file: c!($obj),
            btf_src_file: c!(concat!("btf__core_reloc_", stringify!($name), ".bpf.o")),
            raw_tp_name: c!("sys_enter"),
            prog_name: c!($prog),
            // The corresponding C initializer data is provided by macros in
            // this file and concrete structs from core_reloc_types.h.
            ..ZERO_TEST_CASE
        }
    };
}

unsafe fn roundup_page(sz: size_t) -> size_t {
    let page_size: c_long = sysconf(_SC_PAGE_SIZE);
    ((sz as c_long + page_size - 1) / page_size * page_size) as size_t
}

unsafe fn run_btfgen(src_btf: *const c_char, dst_btf: *const c_char, objpath: *const c_char) -> c_int {
    let mut command = [0 as c_char; 4096];
    let n: c_int;

    n = snprintf(
        command.as_mut_ptr(),
        command.len(),
        c!("./bpftool gen min_core_btf %s %s %s"),
        src_btf,
        dst_btf,
        objpath,
    );
    if n < 0 || n as usize >= command.len() {
        return -1;
    }

    system(command.as_ptr())
}

unsafe fn run_core_reloc_tests(use_btfgen: bool) {
    let mmap_sz: size_t = roundup_page(size_of::<data>());
    let mut open_opts: bpf_object_open_opts = zeroed();
    let mut test_case: *mut core_reloc_test_case;
    let mut test_case_copy: core_reloc_test_case;
    let mut tp_name: *const c_char;
    let mut probe_name: *const c_char;
    let mut err: c_int;
    let mut equal: c_int;
    let mut fd: c_int;
    let mut link: *mut bpf_link = null_mut();
    let mut data_map: *mut bpf_map;
    let mut prog: *mut bpf_program;
    let mut obj: *mut bpf_object = null_mut();
    let my_pid_tgid: uint64_t;
    let mut data_ptr: *mut data;
    let mut mmap_data: *mut c_void = null_mut();

    my_pid_tgid = getpid() as uint64_t | ((sys_gettid() as uint64_t) << 32);

    for i in 0..test_cases.len() {
        let mut btf_file = *b"/tmp/core_reloc.btf.XXXXXX\0";

        test_case_copy = test_cases[i];
        test_case = &mut test_case_copy;

        if !test__start_subtest((*test_case).case_name) {
            continue;
        }

        if (*test_case).needs_testmod && !env.has_testmod {
            test__skip();
            continue;
        }

        /* generate a "minimal" BTF file and use it as source */
        if use_btfgen {
            if (*test_case).btf_src_file.is_null() || (*test_case).run_btfgen_fails {
                test__skip();
                continue;
            }

            fd = mkstemp(btf_file.as_mut_ptr() as *mut c_char);
            if !ASSERT_GE(fd, 0, c!("btf_tmp")) {
                continue;
            }
            close(fd); /* we only need the path */
            err = run_btfgen((*test_case).btf_src_file, btf_file.as_ptr() as *const c_char, (*test_case).bpf_obj_file);
            if !ASSERT_OK(err, c!("run_btfgen")) {
                continue;
            }

            (*test_case).btf_src_file = btf_file.as_ptr() as *const c_char;
        }

        if let Some(setup) = (*test_case).setup {
            err = setup(test_case);
            if CHECK(err != 0, c!("test_setup"), c!("test #%d setup failed: %d\n"), i as c_int, err) {
                continue;
            }
        }

        if !(*test_case).btf_src_file.is_null() {
            err = access((*test_case).btf_src_file, R_OK);
            if !ASSERT_OK(err, c!("btf_src_file")) {
                continue;
            }
        }

        open_opts.btf_custom_path = (*test_case).btf_src_file;
        obj = bpf_object__open_file((*test_case).bpf_obj_file, &open_opts);
        if !ASSERT_OK_PTR(obj as *mut c_void, c!("obj_open")) {
            goto_cleanup(use_btfgen, test_case, mmap_data, mmap_sz, link, obj);
            continue;
        }

        probe_name = (*test_case).prog_name;
        tp_name = (*test_case).raw_tp_name; /* NULL for tp_btf */
        prog = bpf_object__find_program_by_name(obj, probe_name);
        if CHECK(prog.is_null(), c!("find_probe"), c!("prog '%s' not found\n"), probe_name) {
            goto_cleanup(use_btfgen, test_case, mmap_data, mmap_sz, link, obj);
            continue;
        }

        err = bpf_object__load(obj);
        if err != 0 {
            if !(*test_case).fails {
                ASSERT_OK(err, c!("obj_load"));
            }
            goto_cleanup(use_btfgen, test_case, mmap_data, mmap_sz, link, obj);
            continue;
        }

        data_map = bpf_object__find_map_by_name(obj, c!(".bss"));
        if CHECK(data_map.is_null(), c!("find_data_map"), c!("data map not found\n")) {
            goto_cleanup(use_btfgen, test_case, mmap_data, mmap_sz, link, obj);
            continue;
        }

        mmap_data = mmap(null_mut(), mmap_sz, PROT_READ | PROT_WRITE, MAP_SHARED, bpf_map__fd(data_map), 0);
        if CHECK(mmap_data as isize == -1, c!("mmap"), c!(".bss mmap failed: %d"), errno) {
            mmap_data = null_mut();
            goto_cleanup(use_btfgen, test_case, mmap_data, mmap_sz, link, obj);
            continue;
        }
        data_ptr = mmap_data as *mut data;

        memset(mmap_data, 0, size_of::<data>());
        if (*test_case).input_len != 0 {
            memcpy((*data_ptr).input.as_mut_ptr() as *mut c_void, (*test_case).input as *const c_void, (*test_case).input_len as size_t);
        }
        (*data_ptr).my_pid_tgid = my_pid_tgid;

        link = bpf_program__attach_raw_tracepoint(prog, tp_name);
        if !ASSERT_OK_PTR(link as *mut c_void, c!("attach_raw_tp")) {
            goto_cleanup(use_btfgen, test_case, mmap_data, mmap_sz, link, obj);
            continue;
        }

        /* trigger test run */
        if let Some(trigger) = (*test_case).trigger {
            if !ASSERT_OK(trigger(test_case), c!("test_trigger")) {
                goto_cleanup(use_btfgen, test_case, mmap_data, mmap_sz, link, obj);
                continue;
            }
        } else {
            usleep(1);
        }

        if (*data_ptr).skip {
            test__skip();
            goto_cleanup(use_btfgen, test_case, mmap_data, mmap_sz, link, obj);
            continue;
        }

        if !ASSERT_FALSE((*test_case).fails, c!("obj_load_should_fail")) {
            goto_cleanup(use_btfgen, test_case, mmap_data, mmap_sz, link, obj);
            continue;
        }

        equal = (memcmp((*data_ptr).out.as_ptr() as *const c_void, (*test_case).output as *const c_void, (*test_case).output_len as size_t) == 0) as c_int;
        if CHECK(equal == 0, c!("check_result"), c!("input/output data don't match\n")) {
            let mut j: c_int;

            j = 0;
            while j < (*test_case).input_len {
                printf(c!("input byte #%d: 0x%02hhx\n"), j, *(*test_case).input.add(j as usize));
                j += 1;
            }
            j = 0;
            while j < (*test_case).output_len {
                printf(
                    c!("output byte #%d: EXP 0x%02hhx GOT 0x%02hhx\n"),
                    j,
                    *(*test_case).output.add(j as usize),
                    (*data_ptr).out[j as usize],
                );
                j += 1;
            }
            goto_cleanup(use_btfgen, test_case, mmap_data, mmap_sz, link, obj);
            continue;
        }

        goto_cleanup(use_btfgen, test_case, mmap_data, mmap_sz, link, obj);
        mmap_data = null_mut();
        link = null_mut();
        obj = null_mut();
    }
}

unsafe fn goto_cleanup(
    use_btfgen: bool,
    test_case: *mut core_reloc_test_case,
    mut mmap_data: *mut c_void,
    mmap_sz: size_t,
    mut link: *mut bpf_link,
    obj: *mut bpf_object,
) {
    if !mmap_data.is_null() {
        CHECK_FAIL(munmap(mmap_data, mmap_sz));
        mmap_data = null_mut();
    }
    if use_btfgen {
        remove((*test_case).btf_src_file);
    }
    bpf_link__destroy(link);
    link = null_mut();
    bpf_object__close(obj);
}

#[no_mangle]
pub unsafe extern "C" fn test_core_reloc() {
    run_core_reloc_tests(false);
}

#[no_mangle]
pub unsafe extern "C" fn test_core_reloc_btfgen() {
    run_core_reloc_tests(true);
}
