// SPDX-License-Identifier: GPL-2.0

// Dependencies from <linux/bpf.h> and <bpf/bpf_helpers.h> are expected to be
// supplied by the surrounding BPF build environment.

pub type __u32 = u32;

pub const BPF_MAP_TYPE_ARRAY: u32 = 2;

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct S {
    pub v: i32,
}

#[no_mangle]
pub static mut global_variable: S = S { v: 0 };

#[repr(C)]
pub struct values {
    pub type_: u32,
    pub max_entries: u32,
    pub key: __u32,
    pub value: i32,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut values: values = values {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 7,
    key: 0,
    value: 0,
};

extern "C" {
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
}

unsafe fn save_value(index: __u32, value: i32) {
    let index = index;
    let value = value;

    unsafe {
        bpf_map_update_elem(
            core::ptr::addr_of_mut!(values) as *mut core::ffi::c_void,
            core::ptr::addr_of!(index) as *const core::ffi::c_void,
            core::ptr::addr_of!(value) as *const core::ffi::c_void,
            0,
        );
    }
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn foo(index: __u32, s: *mut S) -> i32 {
    if !s.is_null() {
        unsafe {
            save_value(index, (*s).v);
            (*s).v = (*s).v.wrapping_add(1);
            return (*s).v;
        }
    }

    unsafe {
        save_value(index, 0);
    }

    1
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn bar(index: __u32, s: *mut S) -> i32 {
    if !s.is_null() {
        unsafe {
            save_value(index, core::ptr::addr_of!((*s).v).read_volatile());
            let v = core::ptr::addr_of!((*s).v)
                .read_volatile()
                .wrapping_add(1);
            core::ptr::addr_of_mut!((*s).v).write_volatile(v);
            return core::ptr::addr_of!((*s).v).read_volatile();
        }
    }

    unsafe {
        save_value(index, 0);
    }

    1
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn baz(s: *mut *mut S) -> i32 {
    if !s.is_null() {
        unsafe {
            *s = core::ptr::null_mut();
        }
    }

    0
}

#[no_mangle]
#[link_section = "cgroup_skb/ingress"]
pub unsafe extern "C" fn test_cls(_skb: *mut __sk_buff) -> i32 {
    let mut index: __u32 = 0;

    {
        let v: i32 = unsafe {
            let old = index;
            index = index.wrapping_add(1);
            foo(old, core::ptr::null_mut())
        };

        unsafe {
            let old = index;
            index = index.wrapping_add(1);
            save_value(old, v);
        }
    }

    {
        let mut s = S { v: 100 };

        unsafe {
            let old = index;
            index = index.wrapping_add(1);
            foo(old, core::ptr::addr_of_mut!(s));
        }
        unsafe {
            let old = index;
            index = index.wrapping_add(1);
            save_value(old, s.v);
        }
    }

    {
        unsafe {
            global_variable.v = 42;
        }
        unsafe {
            let old = index;
            index = index.wrapping_add(1);
            bar(old, core::ptr::addr_of_mut!(global_variable));
        }
        unsafe {
            let old = index;
            index = index.wrapping_add(1);
            save_value(old, global_variable.v);
        }
    }

    {
        let mut v = S { v: 0 };
        let mut p: *mut S = core::ptr::addr_of_mut!(v);

        unsafe {
            baz(core::ptr::addr_of_mut!(p));
        }
        unsafe {
            let old = index;
            index = index.wrapping_add(1);
            save_value(old, (p.is_null()) as i32);
        }
    }

    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
