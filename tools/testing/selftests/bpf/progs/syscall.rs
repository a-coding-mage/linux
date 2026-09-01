// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */
// Rust translation of testing/selftests/bpf/progs/syscall.c.
// C include dependencies are expected to be supplied by the surrounding build:
// linux/stddef.h, linux/bpf.h, bpf_helpers.h, bpf_tracing.h, linux/filter.h,
// linux/btf.h, string.h, errno.h, and bpf_misc.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

pub type __u32 = u32;
pub type __u64 = u64;

pub const BPF_MAP_TYPE_ARRAY: __u32 = 2;
pub const BPF_MAP_TYPE_HASH: __u32 = 1;
pub const BPF_MAP_TYPE_ARRAY_OF_MAPS: __u32 = 12;
pub const BPF_PROG_TYPE_XDP: __u32 = 6;
pub const BPF_BTF_LOAD: __u32 = 18;
pub const BPF_MAP_CREATE: __u32 = 0;
pub const BPF_MAP_UPDATE_ELEM: __u32 = 2;
pub const BPF_MAP_DELETE_ELEM: __u32 = 3;
pub const BPF_MAP_GET_FD_BY_ID: __u32 = 14;
pub const BTF_MAGIC: __u16 = 0xeB9F;
pub const BTF_VERSION: __u8 = 1;
pub const BTF_MAX_VLEN: __u32 = 0xffff;
pub const BTF_KIND_INT: __u32 = 1;
pub const BTF_INT_SIGNED: __u32 = 1;
pub const BPF_DW: __u32 = 0x18;
pub const BPF_REG_0: __u32 = 0;
pub const BPF_REG_1: __u32 = 1;
pub const BPF_REG_2: __u32 = 2;
pub const BPF_REG_10: __u32 = 10;
pub const BPF_ADD: __u32 = 0x00;
pub const BPF_JMP: __u32 = 0x05;
pub const BPF_CALL: __u32 = 0x80;
pub const BPF_FUNC_map_lookup_elem: __u32 = 1;

pub type __u8 = u8;
pub type __u16 = u16;

#[repr(C)]
pub struct bpf_map {
    pub id: i32,
}

#[repr(C)]
pub struct args {
    pub log_buf: __u64,
    pub log_size: __u32,
    pub max_entries: i32,
    pub map_fd: i32,
    pub prog_fd: i32,
    pub btf_fd: i32,
}

#[repr(C)]
pub struct btf_header {
    pub magic: __u16,
    pub version: __u8,
    pub flags: __u8,
    pub hdr_len: __u32,
    pub type_off: __u32,
    pub type_len: __u32,
    pub str_off: __u32,
    pub str_len: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_insn {
    pub code: __u8,
    pub dst_src_reg: __u8,
    pub off: i16,
    pub imm: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bpf_attr {
    pub btf_size: __u32,
    pub btf: u64,
    pub map_type: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
    pub btf_key_type_id: __u32,
    pub btf_value_type_id: __u32,
    pub btf_fd: __u32,
    pub map_fd: __u32,
    pub key: __u64,
    pub value: __u64,
    pub prog_type: __u32,
    pub insn_cnt: __u32,
    pub license: __u64,
    pub insns: __u64,
    pub log_buf: __u64,
    pub log_size: __u32,
    pub log_level: __u32,
    pub map_id: __u32,
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SEC(".maps") map-definition objects translated from libbpf map declaration syntax.
#[repr(C)]
pub struct bpf_attr_array_def {
    pub type_: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut bpf_attr_array: bpf_attr_array_def = bpf_attr_array_def {
    type_: BPF_MAP_TYPE_ARRAY,
    key_size: core::mem::size_of::<i32>() as __u32,
    value_size: core::mem::size_of::<bpf_attr>() as __u32,
    max_entries: 1,
};

#[repr(C)]
pub struct inner_map_type {
    pub type_: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut inner_map: inner_map_type = inner_map_type {
    type_: BPF_MAP_TYPE_ARRAY,
    key_size: 4,
    value_size: 4,
    max_entries: 1,
};

#[repr(C)]
pub struct outer_array_map_def {
    pub type_: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
    pub values: [*mut inner_map_type; 1],
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut outer_array_map: outer_array_map_def = outer_array_map_def {
    type_: BPF_MAP_TYPE_ARRAY_OF_MAPS,
    key_size: core::mem::size_of::<i32>() as __u32,
    value_size: core::mem::size_of::<i32>() as __u32,
    max_entries: 1,
    values: [unsafe { &mut inner_map as *mut inner_map_type }],
};

extern "C" {
    pub fn bpf_sys_bpf(cmd: __u32, attr: *mut bpf_attr, size: usize) -> i32;
    pub fn bpf_map_lookup_elem(map: *mut bpf_map, key: *const i32) -> *mut bpf_attr;
    pub fn bpf_sys_close(fd: i32) -> i32;
    pub fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
}

pub const fn BTF_INFO_ENC(kind: __u32, kind_flag: __u32, vlen: __u32) -> __u32 {
    (((kind_flag != 0) as __u32) << 31) | (kind << 24) | (vlen & BTF_MAX_VLEN)
}

pub const fn BTF_TYPE_ENC(name: __u32, info: __u32, size_or_type: __u32) -> (__u32, __u32, __u32) {
    (name, info, size_or_type)
}

pub const fn BTF_INT_ENC(encoding: __u32, bits_offset: __u32, nr_bits: __u32) -> __u32 {
    (encoding << 24) | (bits_offset << 16) | nr_bits
}

pub const fn BTF_TYPE_INT_ENC(
    name: __u32,
    encoding: __u32,
    bits_offset: __u32,
    bits: __u32,
    sz: __u32,
) -> (__u32, __u32, __u32, __u32) {
    let t = BTF_TYPE_ENC(name, BTF_INFO_ENC(BTF_KIND_INT, 0, 0), sz);
    (t.0, t.1, t.2, BTF_INT_ENC(encoding, bits_offset, bits))
}

#[inline]
unsafe fn ptr_to_u64(ptr: *const core::ffi::c_void) -> __u64 {
    ptr as usize as __u64
}

#[repr(C)]
struct btf_blob {
    btf_hdr: btf_header,
    types: [__u32; 8],
    str_: __u32,
}

unsafe fn btf_load() -> i32 {
    let long_ty = BTF_TYPE_INT_ENC(0, BTF_INT_SIGNED, 0, 64, 8);
    let unsigned_long_ty = BTF_TYPE_INT_ENC(0, 0, 0, 64, 8);
    let mut raw_btf = btf_blob {
        btf_hdr: btf_header {
            magic: BTF_MAGIC,
            version: BTF_VERSION,
            flags: 0,
            hdr_len: core::mem::size_of::<btf_header>() as __u32,
            type_off: 0,
            type_len: core::mem::size_of::<[__u32; 8]>() as __u32,
            str_off: (core::mem::offset_of!(btf_blob, str_)
                - core::mem::offset_of!(btf_blob, types)) as __u32,
            str_len: core::mem::size_of::<__u32>() as __u32,
        },
        types: [
            /* long */
            long_ty.0, long_ty.1, long_ty.2, long_ty.3, /* [1] */
            /* unsigned long */
            unsigned_long_ty.0, unsigned_long_ty.1, unsigned_long_ty.2, unsigned_long_ty.3, /* [2] */
        ],
        str_: 0,
    };
    static mut btf_load_attr: bpf_attr = bpf_attr {
        btf_size: core::mem::size_of::<btf_blob>() as __u32,
    };

    btf_load_attr.btf = &mut raw_btf as *mut btf_blob as i64 as u64;
    bpf_sys_bpf(
        BPF_BTF_LOAD,
        &mut btf_load_attr as *mut bpf_attr,
        core::mem::size_of::<bpf_attr>(),
    )
}

const fn bpf_insn(code: __u8, dst_reg: __u32, src_reg: __u32, off: i16, imm: i32) -> bpf_insn {
    bpf_insn {
        code,
        dst_src_reg: ((dst_reg & 0xf) | ((src_reg & 0xf) << 4)) as __u8,
        off,
        imm,
    }
}

const fn BPF_ST_MEM(_size: __u32, dst: __u32, off: i16, imm: i32) -> bpf_insn {
    bpf_insn(0x7a, dst, 0, off, imm)
}

const fn BPF_MOV64_REG(dst: __u32, src: __u32) -> bpf_insn {
    bpf_insn(0xbf, dst, src, 0, 0)
}

const fn BPF_ALU64_IMM(_op: __u32, dst: __u32, imm: i32) -> bpf_insn {
    bpf_insn(0x07, dst, 0, 0, imm)
}

const fn BPF_LD_MAP_FD(dst: __u32, fd: i32) -> bpf_insn {
    bpf_insn(0x18, dst, 1, 0, fd)
}

const fn BPF_RAW_INSN(code: __u32, dst: __u32, src: __u32, off: i16, imm: i32) -> bpf_insn {
    bpf_insn(code as __u8, dst, src, off, imm)
}

const fn BPF_MOV64_IMM(dst: __u32, imm: i32) -> bpf_insn {
    bpf_insn(0xb7, dst, 0, 0, imm)
}

const fn BPF_EXIT_INSN() -> bpf_insn {
    bpf_insn(0x95, 0, 0, 0, 0)
}

#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn load_prog(ctx: *mut args) -> i32 {
    static mut license: [u8; 4] = *b"GPL\0";
    static mut insns: [bpf_insn; 7] = [
        BPF_ST_MEM(BPF_DW, BPF_REG_10, -8, 0),
        BPF_MOV64_REG(BPF_REG_2, BPF_REG_10),
        BPF_ALU64_IMM(BPF_ADD, BPF_REG_2, -8),
        BPF_LD_MAP_FD(BPF_REG_1, 0),
        BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_map_lookup_elem as i32),
        BPF_MOV64_IMM(BPF_REG_0, 0),
        BPF_EXIT_INSN(),
    ];
    static mut map_create_attr: bpf_attr = bpf_attr {
        map_type: BPF_MAP_TYPE_HASH,
    };
    static mut map_update_attr: bpf_attr = bpf_attr { map_fd: 1 };
    static mut key: __u64 = 12;
    static mut value: __u64 = 34;
    static mut prog_load_attr: bpf_attr = bpf_attr {
        prog_type: BPF_PROG_TYPE_XDP,
    };
    let mut ret: i32;

    map_create_attr.key_size = 8;
    map_create_attr.value_size = 8;
    map_create_attr.btf_key_type_id = 1;
    map_create_attr.btf_value_type_id = 2;
    prog_load_attr.insn_cnt = insns.len() as __u32;

    ret = btf_load();
    if ret <= 0 {
        return ret;
    }

    (*ctx).btf_fd = ret;
    map_create_attr.max_entries = (*ctx).max_entries as __u32;
    map_create_attr.btf_fd = ret as __u32;

    prog_load_attr.license = ptr_to_u64(license.as_ptr() as *const core::ffi::c_void);
    prog_load_attr.insns = ptr_to_u64(insns.as_ptr() as *const core::ffi::c_void);
    prog_load_attr.log_buf = (*ctx).log_buf;
    prog_load_attr.log_size = (*ctx).log_size;
    prog_load_attr.log_level = 1;

    ret = bpf_sys_bpf(
        BPF_MAP_CREATE,
        &mut map_create_attr as *mut bpf_attr,
        core::mem::size_of::<bpf_attr>(),
    );
    if ret <= 0 {
        return ret;
    }
    (*ctx).map_fd = ret;
    insns[3].imm = ret;

    map_update_attr.map_fd = ret as __u32;
    map_update_attr.key = ptr_to_u64(&key as *const __u64 as *const core::ffi::c_void);
    map_update_attr.value = ptr_to_u64(&value as *const __u64 as *const core::ffi::c_void);
    ret = bpf_sys_bpf(
        BPF_MAP_UPDATE_ELEM,
        &mut map_update_attr as *mut bpf_attr,
        core::mem::size_of::<bpf_attr>(),
    );
    if ret < 0 {
        return ret;
    }

    ret = bpf_sys_bpf(
        BPF_PROG_LOAD,
        &mut prog_load_attr as *mut bpf_attr,
        core::mem::size_of::<bpf_attr>(),
    );
    if ret <= 0 {
        return ret;
    }
    (*ctx).prog_fd = ret;
    1
}

#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn update_outer_map(_ctx: *mut core::ffi::c_void) -> i32 {
    let mut zero: i32 = 0;
    let mut ret: i32 = 0;
    let mut outer_fd: i32 = -1;
    let mut inner_fd: i32 = -1;
    let mut err: i32;
    let attr_sz: usize = core::mem::size_of::<bpf_attr>();
    let attr: *mut bpf_attr;

    attr = bpf_map_lookup_elem(
        &mut bpf_attr_array as *mut bpf_attr_array_def as *mut bpf_map,
        &mut zero as *mut i32 as *const i32,
    );
    if attr.is_null() {
        goto_out(&mut inner_fd, &mut outer_fd);
        return ret;
    }

    memset(attr as *mut core::ffi::c_void, 0, attr_sz);
    (*attr).map_id = (*( &mut outer_array_map as *mut outer_array_map_def as *mut bpf_map)).id as __u32;
    outer_fd = bpf_sys_bpf(BPF_MAP_GET_FD_BY_ID, attr, attr_sz);
    if outer_fd < 0 {
        goto_out(&mut inner_fd, &mut outer_fd);
        return ret;
    }

    memset(attr as *mut core::ffi::c_void, 0, attr_sz);
    (*attr).map_type = BPF_MAP_TYPE_ARRAY;
    (*attr).key_size = 4;
    (*attr).value_size = 4;
    (*attr).max_entries = 1;
    inner_fd = bpf_sys_bpf(BPF_MAP_CREATE, attr, attr_sz);
    if inner_fd < 0 {
        goto_out(&mut inner_fd, &mut outer_fd);
        return ret;
    }

    memset(attr as *mut core::ffi::c_void, 0, attr_sz);
    (*attr).map_fd = outer_fd as __u32;
    (*attr).key = ptr_to_u64(&zero as *const i32 as *const core::ffi::c_void);
    (*attr).value = ptr_to_u64(&inner_fd as *const i32 as *const core::ffi::c_void);
    err = bpf_sys_bpf(BPF_MAP_UPDATE_ELEM, attr, attr_sz);
    if err != 0 {
        goto_out(&mut inner_fd, &mut outer_fd);
        return ret;
    }

    memset(attr as *mut core::ffi::c_void, 0, attr_sz);
    (*attr).map_fd = outer_fd as __u32;
    (*attr).key = ptr_to_u64(&zero as *const i32 as *const core::ffi::c_void);
    err = bpf_sys_bpf(BPF_MAP_DELETE_ELEM, attr, attr_sz);
    if err != 0 {
        goto_out(&mut inner_fd, &mut outer_fd);
        return ret;
    }
    ret = 1;

    goto_out(&mut inner_fd, &mut outer_fd);
    ret
}

unsafe fn goto_out(inner_fd: &mut i32, outer_fd: &mut i32) {
    if *inner_fd >= 0 {
        bpf_sys_close(*inner_fd);
    }
    if *outer_fd >= 0 {
        bpf_sys_close(*outer_fd);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
