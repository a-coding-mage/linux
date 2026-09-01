// SPDX-License-Identifier: GPL-2.0
// Translated from linked_list.c. C include dependencies:
// <vmlinux.h>, <bpf/bpf_tracing.h>, <bpf/bpf_helpers.h>,
// <bpf/bpf_core_read.h>, "bpf_experimental.h", "bpf_misc.h",
// and "linked_list.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::c_void;
use core::mem::MaybeUninit;

#[repr(C)]
pub struct head_nested_inner {
    pub lock: bpf_spin_lock,
    pub head: bpf_list_head,
    // C annotation: __contains(foo, node2)
}

#[repr(C)]
pub struct head_nested {
    pub dummy: i32,
    pub inner: head_nested_inner,
}

// External declarations supplied by translated headers / other compilation units.
extern "C" {
    static mut glock: bpf_spin_lock;
    static mut ghead: bpf_list_head;
    static mut array_map: c_void;
    static mut map_of_maps: c_void;

    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
    fn bpf_list_pop_front(head: *mut bpf_list_head) -> *mut bpf_list_node;
    fn bpf_list_pop_back(head: *mut bpf_list_head) -> *mut bpf_list_node;
    fn bpf_list_push_front(head: *mut bpf_list_head, node: *mut bpf_list_node);
    fn bpf_list_push_back(head: *mut bpf_list_head, node: *mut bpf_list_node);
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const i32) -> *mut c_void;
}

extern "C" {
    #[link_name = "bpf_obj_new"]
    fn bpf_obj_new_foo() -> *mut foo;
    #[link_name = "bpf_obj_new"]
    fn bpf_obj_new_bar() -> *mut bar;
    #[link_name = "bpf_obj_drop"]
    fn bpf_obj_drop_foo(obj: *mut foo);
    #[link_name = "bpf_obj_drop"]
    fn bpf_obj_drop_bar(obj: *mut bar);
}

static mut glock_c: bpf_spin_lock = unsafe { MaybeUninit::<bpf_spin_lock>::zeroed().assume_init() };
static mut ghead_array: [bpf_list_head; 2] =
    unsafe { MaybeUninit::<[bpf_list_head; 2]>::zeroed().assume_init() };
// C annotation: __contains(foo, node2)
static mut ghead_array_one: [bpf_list_head; 1] =
    unsafe { MaybeUninit::<[bpf_list_head; 1]>::zeroed().assume_init() };
// C annotation: __contains(foo, node2)

static mut ghead_nested: head_nested = unsafe { MaybeUninit::<head_nested>::zeroed().assume_init() };

unsafe fn container_of_foo_node2(n: *mut bpf_list_node) -> *mut foo {
    (n as *mut u8).sub(core::mem::offset_of!(foo, node2)) as *mut foo
}

unsafe fn container_of_bar_node(n: *mut bpf_list_node) -> *mut bar {
    (n as *mut u8).sub(core::mem::offset_of!(bar, node)) as *mut bar
}

#[inline(always)]
unsafe fn list_push_pop(
    lock: *mut bpf_spin_lock,
    head: *mut bpf_list_head,
    leave_in_map: bool,
) -> i32 {
    let mut n: *mut bpf_list_node;
    let mut f: *mut foo;

    f = bpf_obj_new_foo();
    if f.is_null() {
        return 2;
    }

    bpf_spin_lock(lock);
    n = bpf_list_pop_front(head);
    bpf_spin_unlock(lock);
    if !n.is_null() {
        bpf_obj_drop_foo(container_of_foo_node2(n));
        bpf_obj_drop_foo(f);
        return 3;
    }

    bpf_spin_lock(lock);
    n = bpf_list_pop_back(head);
    bpf_spin_unlock(lock);
    if !n.is_null() {
        bpf_obj_drop_foo(container_of_foo_node2(n));
        bpf_obj_drop_foo(f);
        return 4;
    }

    bpf_spin_lock(lock);
    (*f).data = 42;
    bpf_list_push_front(head, &mut (*f).node2);
    bpf_spin_unlock(lock);
    if leave_in_map {
        return 0;
    }
    bpf_spin_lock(lock);
    n = bpf_list_pop_back(head);
    bpf_spin_unlock(lock);
    if n.is_null() {
        return 5;
    }
    f = container_of_foo_node2(n);
    if (*f).data != 42 {
        bpf_obj_drop_foo(f);
        return 6;
    }

    bpf_spin_lock(lock);
    (*f).data = 13;
    bpf_list_push_front(head, &mut (*f).node2);
    bpf_spin_unlock(lock);
    bpf_spin_lock(lock);
    n = bpf_list_pop_front(head);
    bpf_spin_unlock(lock);
    if n.is_null() {
        return 7;
    }
    f = container_of_foo_node2(n);
    if (*f).data != 13 {
        bpf_obj_drop_foo(f);
        return 8;
    }
    bpf_obj_drop_foo(f);

    bpf_spin_lock(lock);
    n = bpf_list_pop_front(head);
    bpf_spin_unlock(lock);
    if !n.is_null() {
        bpf_obj_drop_foo(container_of_foo_node2(n));
        return 9;
    }

    bpf_spin_lock(lock);
    n = bpf_list_pop_back(head);
    bpf_spin_unlock(lock);
    if !n.is_null() {
        bpf_obj_drop_foo(container_of_foo_node2(n));
        return 10;
    }
    0
}

#[inline(always)]
unsafe fn list_push_pop_multiple(
    lock: *mut bpf_spin_lock,
    head: *mut bpf_list_head,
    leave_in_map: bool,
) -> i32 {
    let mut n: *mut bpf_list_node;
    let mut f: [*mut foo; 200] = [core::ptr::null_mut(); 200];
    let mut pf: *mut foo;
    let mut i: usize;

    /* Loop following this check adds nodes 2-at-a-time in order to
     * validate multiple release_on_unlock release logic
     */
    if f.len() % 2 != 0 {
        return 10;
    }

    i = 0;
    while i < f.len() {
        f[i] = bpf_obj_new_foo();
        if f[i].is_null() {
            return 2;
        }
        (*f[i]).data = i as i32;

        f[i + 1] = bpf_obj_new_foo();
        if f[i + 1].is_null() {
            bpf_obj_drop_foo(f[i]);
            return 9;
        }
        (*f[i + 1]).data = (i + 1) as i32;

        bpf_spin_lock(lock);
        bpf_list_push_front(head, &mut (*f[i]).node2);
        bpf_list_push_front(head, &mut (*f[i + 1]).node2);
        bpf_spin_unlock(lock);
        i += 2;
    }

    i = 0;
    while i < f.len() {
        bpf_spin_lock(lock);
        n = bpf_list_pop_front(head);
        bpf_spin_unlock(lock);
        if n.is_null() {
            return 3;
        }
        pf = container_of_foo_node2(n);
        if (*pf).data != (f.len() - i - 1) as i32 {
            bpf_obj_drop_foo(pf);
            return 4;
        }
        bpf_spin_lock(lock);
        bpf_list_push_back(head, &mut (*pf).node2);
        bpf_spin_unlock(lock);
        i += 1;
    }

    if leave_in_map {
        return 0;
    }

    i = 0;
    while i < f.len() {
        bpf_spin_lock(lock);
        n = bpf_list_pop_back(head);
        bpf_spin_unlock(lock);
        if n.is_null() {
            return 5;
        }
        pf = container_of_foo_node2(n);
        if (*pf).data != i as i32 {
            bpf_obj_drop_foo(pf);
            return 6;
        }
        bpf_obj_drop_foo(pf);
        i += 1;
    }
    bpf_spin_lock(lock);
    n = bpf_list_pop_back(head);
    bpf_spin_unlock(lock);
    if !n.is_null() {
        bpf_obj_drop_foo(container_of_foo_node2(n));
        return 7;
    }

    bpf_spin_lock(lock);
    n = bpf_list_pop_front(head);
    bpf_spin_unlock(lock);
    if !n.is_null() {
        bpf_obj_drop_foo(container_of_foo_node2(n));
        return 8;
    }
    0
}

#[inline(always)]
unsafe fn list_in_list(
    lock: *mut bpf_spin_lock,
    head: *mut bpf_list_head,
    leave_in_map: bool,
) -> i32 {
    let mut n: *mut bpf_list_node;
    let mut ba: [*mut bar; 8] = [core::ptr::null_mut(); 8];
    let mut b: *mut bar;
    let mut f: *mut foo;
    let mut i: usize;

    f = bpf_obj_new_foo();
    if f.is_null() {
        return 2;
    }
    i = 0;
    while i < ba.len() {
        b = bpf_obj_new_bar();
        if b.is_null() {
            bpf_obj_drop_foo(f);
            return 3;
        }
        ba[i] = b;
        (*b).data = i as i32;
        bpf_spin_lock(&mut (*f).lock);
        bpf_list_push_back(&mut (*f).head, &mut (*b).node);
        bpf_spin_unlock(&mut (*f).lock);
        i += 1;
    }

    bpf_spin_lock(lock);
    (*f).data = 42;
    bpf_list_push_front(head, &mut (*f).node2);
    bpf_spin_unlock(lock);

    if leave_in_map {
        return 0;
    }

    bpf_spin_lock(lock);
    n = bpf_list_pop_front(head);
    bpf_spin_unlock(lock);
    if n.is_null() {
        return 4;
    }
    f = container_of_foo_node2(n);
    if (*f).data != 42 {
        bpf_obj_drop_foo(f);
        return 5;
    }

    i = 0;
    while i < ba.len() {
        bpf_spin_lock(&mut (*f).lock);
        n = bpf_list_pop_front(&mut (*f).head);
        bpf_spin_unlock(&mut (*f).lock);
        if n.is_null() {
            bpf_obj_drop_foo(f);
            return 6;
        }
        b = container_of_bar_node(n);
        if (*b).data != i as i32 {
            bpf_obj_drop_foo(f);
            bpf_obj_drop_bar(b);
            return 7;
        }
        bpf_obj_drop_bar(b);
        i += 1;
    }
    bpf_spin_lock(&mut (*f).lock);
    n = bpf_list_pop_front(&mut (*f).head);
    bpf_spin_unlock(&mut (*f).lock);
    if !n.is_null() {
        bpf_obj_drop_foo(f);
        bpf_obj_drop_bar(container_of_bar_node(n));
        return 8;
    }
    bpf_obj_drop_foo(f);
    0
}

#[inline(always)]
unsafe fn test_list_push_pop(lock: *mut bpf_spin_lock, head: *mut bpf_list_head) -> i32 {
    let mut ret: i32;

    ret = list_push_pop(lock, head, false);
    if ret != 0 {
        return ret;
    }
    list_push_pop(lock, head, true)
}

#[inline(always)]
unsafe fn test_list_push_pop_multiple(lock: *mut bpf_spin_lock, head: *mut bpf_list_head) -> i32 {
    let mut ret: i32;

    ret = list_push_pop_multiple(lock, head, false);
    if ret != 0 {
        return ret;
    }
    list_push_pop_multiple(lock, head, true)
}

#[inline(always)]
unsafe fn test_list_in_list(lock: *mut bpf_spin_lock, head: *mut bpf_list_head) -> i32 {
    let mut ret: i32;

    ret = list_in_list(lock, head, false);
    if ret != 0 {
        return ret;
    }
    list_in_list(lock, head, true)
}

const MAX_LIST_CLEAR_NODES: i32 = 256;

#[inline(always)]
unsafe fn clear_list(lock: *mut bpf_spin_lock, head: *mut bpf_list_head) -> i32 {
    let mut n: *mut bpf_list_node;
    let mut i: i32;

    i = 0;
    while i < MAX_LIST_CLEAR_NODES {
        bpf_spin_lock(lock);
        n = bpf_list_pop_front(head);
        bpf_spin_unlock(lock);
        if n.is_null() {
            return 0;
        }
        bpf_obj_drop_foo(container_of_foo_node2(n));
        i += 1;
    }
    1
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn clear_map_list(_ctx: *mut c_void) -> i32 {
    let mut v: *mut map_value;

    v = bpf_map_lookup_elem(&mut array_map, &0) as *mut map_value;
    if v.is_null() {
        return 1;
    }
    clear_list(&mut (*v).lock, &mut (*v).head)
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn clear_inner_map_list(_ctx: *mut c_void) -> i32 {
    let mut v: *mut map_value;
    let mut map: *mut c_void;

    map = bpf_map_lookup_elem(&mut map_of_maps, &0);
    if map.is_null() {
        return 1;
    }
    v = bpf_map_lookup_elem(map, &0) as *mut map_value;
    if v.is_null() {
        return 1;
    }
    clear_list(&mut (*v).lock, &mut (*v).head)
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn clear_global_list(_ctx: *mut c_void) -> i32 {
    clear_list(&mut glock, &mut ghead)
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn clear_global_nested_list(_ctx: *mut c_void) -> i32 {
    clear_list(
        &mut ghead_nested.inner.lock,
        &mut ghead_nested.inner.head,
    )
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn clear_global_array_list(_ctx: *mut c_void) -> i32 {
    let mut ret: i32;

    ret = clear_list(&mut glock_c, &mut ghead_array[0]);
    if ret != 0 {
        return ret;
    }
    ret = clear_list(&mut glock_c, &mut ghead_array[1]);
    if ret != 0 {
        return ret;
    }
    clear_list(&mut glock_c, &mut ghead_array_one[0])
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn map_list_push_pop(_ctx: *mut c_void) -> i32 {
    let mut v: *mut map_value;

    v = bpf_map_lookup_elem(&mut array_map, &0) as *mut map_value;
    if v.is_null() {
        return 1;
    }
    test_list_push_pop(&mut (*v).lock, &mut (*v).head)
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn inner_map_list_push_pop(_ctx: *mut c_void) -> i32 {
    let mut v: *mut map_value;
    let mut map: *mut c_void;

    map = bpf_map_lookup_elem(&mut map_of_maps, &0);
    if map.is_null() {
        return 1;
    }
    v = bpf_map_lookup_elem(map, &0) as *mut map_value;
    if v.is_null() {
        return 1;
    }
    test_list_push_pop(&mut (*v).lock, &mut (*v).head)
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn global_list_push_pop(_ctx: *mut c_void) -> i32 {
    test_list_push_pop(&mut glock, &mut ghead)
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn global_list_push_pop_nested(_ctx: *mut c_void) -> i32 {
    test_list_push_pop(
        &mut ghead_nested.inner.lock,
        &mut ghead_nested.inner.head,
    )
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn global_list_array_push_pop(_ctx: *mut c_void) -> i32 {
    let mut r: i32;

    r = test_list_push_pop(&mut glock_c, &mut ghead_array[0]);
    if r != 0 {
        return r;
    }

    r = test_list_push_pop(&mut glock_c, &mut ghead_array[1]);
    if r != 0 {
        return r;
    }

    /* Arrays with only one element is a special case, being treated
     * just like a bpf_list_head variable by the verifier, not an
     * array.
     */
    test_list_push_pop(&mut glock_c, &mut ghead_array_one[0])
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn map_list_push_pop_multiple(_ctx: *mut c_void) -> i32 {
    let mut v: *mut map_value;

    v = bpf_map_lookup_elem(&mut array_map, &0) as *mut map_value;
    if v.is_null() {
        return 1;
    }
    test_list_push_pop_multiple(&mut (*v).lock, &mut (*v).head)
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn inner_map_list_push_pop_multiple(_ctx: *mut c_void) -> i32 {
    let mut v: *mut map_value;
    let mut map: *mut c_void;

    map = bpf_map_lookup_elem(&mut map_of_maps, &0);
    if map.is_null() {
        return 1;
    }
    v = bpf_map_lookup_elem(map, &0) as *mut map_value;
    if v.is_null() {
        return 1;
    }
    test_list_push_pop_multiple(&mut (*v).lock, &mut (*v).head)
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn global_list_push_pop_multiple(_ctx: *mut c_void) -> i32 {
    let mut ret: i32;

    ret = list_push_pop_multiple(&mut glock, &mut ghead, false);
    if ret != 0 {
        return ret;
    }
    list_push_pop_multiple(&mut glock, &mut ghead, true)
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn map_list_in_list(_ctx: *mut c_void) -> i32 {
    let mut v: *mut map_value;

    v = bpf_map_lookup_elem(&mut array_map, &0) as *mut map_value;
    if v.is_null() {
        return 1;
    }
    test_list_in_list(&mut (*v).lock, &mut (*v).head)
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn inner_map_list_in_list(_ctx: *mut c_void) -> i32 {
    let mut v: *mut map_value;
    let mut map: *mut c_void;

    map = bpf_map_lookup_elem(&mut map_of_maps, &0);
    if map.is_null() {
        return 1;
    }
    v = bpf_map_lookup_elem(map, &0) as *mut map_value;
    if v.is_null() {
        return 1;
    }
    test_list_in_list(&mut (*v).lock, &mut (*v).head)
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn global_list_in_list(_ctx: *mut c_void) -> i32 {
    test_list_in_list(&mut glock, &mut ghead)
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
