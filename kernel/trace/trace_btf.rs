// SPDX-License-Identifier: GPL-2.0
// Translated from trace_btf.c. Linux BTF/kernel/slab declarations are supplied
// by the surrounding build environment.

use core::ffi::c_char;

extern "C" {
    fn bpf_find_btf_id(
        func_name: *const c_char,
        kind: u32,
        btf_p: *mut *mut btf,
    ) -> i32;
    fn btf_type_by_id(btf: *mut btf, id: u32) -> *const btf_type;
    fn btf_type_is_func(t: *const btf_type) -> bool;
    fn btf_type_is_func_proto(t: *const btf_type) -> bool;
    fn btf_put(btf: *mut btf);
    fn btf_type_vlen(t: *const btf_type) -> i32;
    fn btf_type_is_struct(t: *const btf_type) -> bool;
    fn btf_type_skip_modifiers(btf: *mut btf, type_id: u32, tid: *mut u32) -> bool;
    fn btf_name_by_offset(btf: *mut btf, name_off: u32) -> *const c_char;
    fn strcmp(lhs: *const c_char, rhs: *const c_char) -> i32;
}

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_type {
    pub name_off: u32,
    pub info: u32,
    pub type_: u32,
}

#[repr(C)]
pub struct btf_param {
    pub name_off: u32,
    pub type_: u32,
}

#[repr(C)]
pub struct btf_member {
    pub name_off: u32,
    pub type_: u32,
    pub offset: u32,
}

const BTF_KIND_FUNC: u32 = 12;
const BTF_ANON_STACK_MAX: usize = 16;
const EINVAL: isize = 22;
const ENOMEM: isize = 12;

#[repr(C)]
struct btf_anon_stack {
    tid: u32,
    offset: u32,
}

pub unsafe fn btf_find_func_proto(
    func_name: *const c_char,
    btf_p: *mut *mut btf,
) -> *const btf_type {
    let id = bpf_find_btf_id(func_name, BTF_KIND_FUNC, btf_p);
    if id < 0 {
        return core::ptr::null();
    }

    let mut t = btf_type_by_id(*btf_p, id as u32);
    if t.is_null() || !btf_type_is_func(t) {
        btf_put(*btf_p);
        return core::ptr::null();
    }

    t = btf_type_by_id(*btf_p, (*t).type_);
    if t.is_null() || !btf_type_is_func_proto(t) {
        btf_put(*btf_p);
        return core::ptr::null();
    }

    t
}

pub unsafe fn btf_get_func_param(
    func_proto: *const btf_type,
    nr: *mut i32,
) -> *const btf_param {
    if !btf_type_is_func_proto(func_proto) {
        return (-EINVAL) as *const btf_param;
    }

    *nr = btf_type_vlen(func_proto);
    if *nr > 0 {
        func_proto.add(1) as *const btf_param
    } else {
        core::ptr::null()
    }
}

// The C implementation uses kzalloc_objs and for_each_member; these kernel
// helpers are represented by the direct allocation and member iteration below.
pub unsafe fn btf_find_struct_member(
    btf: *mut btf,
    mut type_: *const btf_type,
    member_name: *const c_char,
    anon_offset: *mut u32,
) -> *const btf_member {
    let mut anon_stack: [btf_anon_stack; BTF_ANON_STACK_MAX] =
        core::mem::zeroed();
    let mut top: usize = 0;
    let mut cur_offset: u32 = 0;

    'retry: loop {
        if !btf_type_is_struct(type_) {
            return (-EINVAL) as *const btf_member;
        }

        let member_count = (*type_).info & 0xffff;
        let first = type_.add(1) as *const btf_member;
        for i in 0..member_count {
            let member = &*first.add(i as usize);
            if member.name_off == 0 {
                let mut tid = 0;
                if btf_type_skip_modifiers(btf, member.type_, &mut tid)
                    && top < BTF_ANON_STACK_MAX
                {
                    anon_stack[top].tid = tid;
                    anon_stack[top].offset = cur_offset.wrapping_add(member.offset);
                    top += 1;
                }
            } else {
                let name = btf_name_by_offset(btf, member.name_off);
                if !name.is_null() && strcmp(member_name, name) == 0 {
                    if !anon_offset.is_null() {
                        *anon_offset = cur_offset;
                    }
                    return member;
                }
            }
        }

        if top > 0 {
            top -= 1;
            let entry = anon_stack[top];
            cur_offset = entry.offset;
            type_ = btf_type_by_id(btf, entry.tid);
            continue 'retry;
        }
        return core::ptr::null();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
