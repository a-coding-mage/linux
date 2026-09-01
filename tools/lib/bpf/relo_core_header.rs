/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */
/* Copyright (c) 2019 Facebook */

/* Dependency intent from C header: #include <linux/bpf.h> */

pub type __u32 = u32;
pub type __u64 = u64;
pub type size_t = usize;

#[repr(C)]
pub struct btf {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bpf_core_relo {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bpf_insn {
    _unused: [u8; 0],
}

/* enum bpf_core_relo_kind is supplied by <linux/bpf.h>. */
pub type bpf_core_relo_kind = ::std::os::raw::c_uint;

pub const BPF_CORE_SPEC_MAX_LEN: usize = 64;

#[repr(C)]
pub struct bpf_core_cand {
    pub btf: *const btf,
    pub id: __u32,
}

/* dynamically sized list of type IDs and its associated struct btf */
#[repr(C)]
pub struct bpf_core_cand_list {
    pub cands: *mut bpf_core_cand,
    pub len: ::std::os::raw::c_int,
}

/* represents BPF CO-RE field or array element accessor */
#[repr(C)]
pub struct bpf_core_accessor {
    pub type_id: __u32,                    /* struct/union type or array element type */
    pub idx: __u32,                        /* field index or array index */
    pub name: *const ::std::os::raw::c_char, /* field name or NULL for array accessor */
}

#[repr(C)]
pub struct bpf_core_spec {
    pub btf: *const btf,
    /* high-level spec: named fields and array indices only */
    pub spec: [bpf_core_accessor; BPF_CORE_SPEC_MAX_LEN],
    /* original unresolved (no skip_mods_or_typedefs) root type ID */
    pub root_type_id: __u32,
    /* CO-RE relocation kind */
    pub relo_kind: bpf_core_relo_kind,
    /* high-level spec length */
    pub len: ::std::os::raw::c_int,
    /* raw, low-level spec: 1-to-1 with accessor spec string */
    pub raw_spec: [::std::os::raw::c_int; BPF_CORE_SPEC_MAX_LEN],
    /* raw spec length */
    pub raw_len: ::std::os::raw::c_int,
    /* field bit offset represented by spec */
    pub bit_offset: __u32,
}

#[repr(C)]
pub struct bpf_core_relo_res {
    /* expected value in the instruction, unless validate == false */
    pub orig_val: __u64,
    /* new value that needs to be patched up to */
    pub new_val: __u64,
    /* relocation unsuccessful, poison instruction, but don't fail load */
    pub poison: bool,
    /* some relocations can't be validated against orig_val */
    pub validate: bool,
    /* for field byte offset relocations or the forms:
     *     *(T *)(rX + <off>) = rY
     *     rX = *(T *)(rY + <off>),
     * we remember original and resolved field size to adjust direct
     * memory loads of pointers and integers; this is necessary for 32-bit
     * host kernel architectures, but also allows to automatically
     * relocate fields that were resized from, e.g., u32 to u64, etc.
     */
    pub fail_memsz_adjust: bool,
    pub orig_sz: __u32,
    pub orig_type_id: __u32,
    pub new_sz: __u32,
    pub new_type_id: __u32,
}

unsafe extern "C" {
    pub fn __bpf_core_types_are_compat(
        local_btf: *const btf,
        local_id: __u32,
        targ_btf: *const btf,
        targ_id: __u32,
        level: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn bpf_core_types_are_compat(
        local_btf: *const btf,
        local_id: __u32,
        targ_btf: *const btf,
        targ_id: __u32,
    ) -> ::std::os::raw::c_int;

    pub fn __bpf_core_types_match(
        local_btf: *const btf,
        local_id: __u32,
        targ_btf: *const btf,
        targ_id: __u32,
        behind_ptr: bool,
        level: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn bpf_core_types_match(
        local_btf: *const btf,
        local_id: __u32,
        targ_btf: *const btf,
        targ_id: __u32,
    ) -> ::std::os::raw::c_int;

    pub fn bpf_core_essential_name_len(name: *const ::std::os::raw::c_char) -> size_t;

    pub fn bpf_core_calc_relo_insn(
        prog_name: *const ::std::os::raw::c_char,
        relo: *const bpf_core_relo,
        relo_idx: ::std::os::raw::c_int,
        local_btf: *const btf,
        cands: *mut bpf_core_cand_list,
        specs_scratch: *mut bpf_core_spec,
        targ_res: *mut bpf_core_relo_res,
    ) -> ::std::os::raw::c_int;

    pub fn bpf_core_patch_insn(
        prog_name: *const ::std::os::raw::c_char,
        insn: *mut bpf_insn,
        insn_idx: ::std::os::raw::c_int,
        relo: *const bpf_core_relo,
        relo_idx: ::std::os::raw::c_int,
        res: *const bpf_core_relo_res,
    ) -> ::std::os::raw::c_int;

    pub fn bpf_core_parse_spec(
        prog_name: *const ::std::os::raw::c_char,
        btf: *const btf,
        relo: *const bpf_core_relo,
        spec: *mut bpf_core_spec,
    ) -> ::std::os::raw::c_int;

    pub fn bpf_core_format_spec(
        buf: *mut ::std::os::raw::c_char,
        buf_sz: size_t,
        spec: *const bpf_core_spec,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
