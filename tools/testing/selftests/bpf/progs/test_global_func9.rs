// SPDX-License-Identifier: GPL-2.0-only
// C includes translated as external dependencies:
// <stddef.h>, <linux/bpf.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"

#[repr(C)]
pub struct S {
    pub x: i32,
}

#[repr(C)]
pub struct C {
    pub x: i32,
    pub y: i32,
}

// Original BPF map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_ARRAY);
//     __uint(max_entries, 1);
//     __type(key, __u32);
//     __type(value, struct S);
// } map SEC(".maps");
#[repr(C)]
pub struct Map {
    _private: [u8; 0],
}

unsafe extern "C" {
    #[link_name = "map"]
    pub static mut map: Map;
}

#[repr(C)]
pub enum E {
    E_ITEM = 0,
}

static mut global_data_x: i32 = 100;
static mut global_data_y: i32 = 500;

#[repr(C)]
pub struct __sk_buff {
    pub len: u32,
    pub family: u32,
}

unsafe extern "C" {
    pub fn bpf_get_prandom_u32() -> u32;
    pub fn bpf_map_lookup_elem(map: *mut Map, key: *const u32) -> *mut core::ffi::c_void;
}

#[inline(never)]
pub unsafe extern "C" fn foo(s: *const S) -> i32 {
    if !s.is_null() {
        return (unsafe { bpf_get_prandom_u32() } < unsafe { (*s).x } as u32) as i32;
    }

    0
}

#[inline(never)]
pub unsafe extern "C" fn bar(x: *mut i32) -> i32 {
    if !x.is_null() {
        unsafe {
            *x &= bpf_get_prandom_u32() as i32;
        }
    }

    0
}

#[inline(never)]
pub unsafe extern "C" fn baz(x: *mut i32) -> i32 {
    if !x.is_null() {
        unsafe {
            let value = core::ptr::read_volatile(x);
            core::ptr::write_volatile(x, value & bpf_get_prandom_u32() as i32);
        }
    }

    0
}

#[inline(never)]
pub unsafe extern "C" fn qux(e: *mut E) -> i32 {
    if !e.is_null() {
        return unsafe { *(e as *mut i32) };
    }

    0
}

#[inline(never)]
pub unsafe extern "C" fn quux(arr: *mut [i32; 10]) -> i32 {
    if !arr.is_null() {
        return unsafe { (*arr)[9] };
    }

    0
}

#[inline(never)]
pub unsafe extern "C" fn quuz(p: *mut *mut i32) -> i32 {
    if !p.is_null() {
        unsafe {
            *p = core::ptr::null_mut();
        }
    }

    0
}

// Original section/verification annotations:
// SEC("cgroup_skb/ingress")
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn global_func9(skb: *mut __sk_buff) -> i32 {
    let mut result: i32 = 0;

    {
        let s = S {
            x: unsafe { (*skb).len } as i32,
        };

        result |= unsafe { foo(&s) };
    }

    {
        let key: u32 = 1;
        let s = unsafe { bpf_map_lookup_elem(&raw mut map, &key) as *const S };

        result |= unsafe { foo(s) };
    }

    {
        let c = C {
            x: unsafe { (*skb).len } as i32,
            y: unsafe { (*skb).family } as i32,
        };

        result |= unsafe { foo((&c as *const C).cast::<S>()) };
    }

    {
        result |= unsafe { foo(core::ptr::null()) };
    }

    {
        unsafe {
            bar(&mut result);
            bar(&raw mut global_data_x);
        }
    }

    {
        result |= unsafe { baz(&raw mut global_data_y) };
    }

    {
        let mut e = E::E_ITEM;

        result |= unsafe { qux(&mut e) };
    }

    {
        let mut array: [i32; 10] = [0; 10];

        result |= unsafe { quux(&mut array) };
    }

    {
        let mut p: *mut i32;

        result |= unsafe { quuz(&mut p) };
    }

    if result != 0 { 1 } else { 0 }
}
