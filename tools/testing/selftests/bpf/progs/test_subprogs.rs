// Translated from test_subprogs.c.
// Original includes: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_core_read.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

type __u32 = u32;
type __u64 = u64;
type uintptr_t = usize;

const BPF_MAP_TYPE_ARRAY: u32 = 2;

#[repr(C)]
pub struct task_struct {
    pub pid: i32,
    pub tgid: i32,
}

#[repr(C)]
pub struct array_map_def {
    // C declaration uses BPF map-definition macros:
    // __uint(type, BPF_MAP_TYPE_ARRAY);
    // __uint(max_entries, 1);
    // __type(key, __u32);
    // __type(value, __u64);
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

unsafe extern "C" {
    fn bpf_map_lookup_elem(map: *const array_map_def, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_get_current_task() -> u64;
    fn bpf_loop(
        nr_loops: __u32,
        callback_fn: Option<unsafe extern "C" fn(index: __u32, data: *mut core::ffi::c_void) -> i32>,
        callback_ctx: *mut core::ffi::c_void,
        flags: u64,
    ) -> i64;
}

// const char LICENSE[] SEC("license") = "GPL";
#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static LICENSE: [u8; 4] = *b"GPL\0";

// } array SEC(".maps");
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static array: array_map_def = array_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as u32,
    value_size: core::mem::size_of::<__u64>() as u32,
};

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sub1(x: i32) -> i32 {
    let key: i32 = 0;

    unsafe {
        bpf_map_lookup_elem(
            &array as *const array_map_def,
            &key as *const i32 as *const core::ffi::c_void,
        );
    }
    x + 1
}

#[inline(never)]
unsafe extern "C" fn sub5(v: i32) -> i32 {
    unsafe { sub1(v) - 1 }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sub2(y: i32) -> i32 {
    unsafe { sub5(y + 2) }
}

#[inline(never)]
unsafe extern "C" fn sub3(z: i32) -> i32 {
    unsafe { z + 3 + sub1(4) }
}

#[inline(never)]
unsafe extern "C" fn sub4(w: i32) -> i32 {
    let key: i32 = 0;

    unsafe {
        bpf_map_lookup_elem(
            &array as *const array_map_def,
            &key as *const i32 as *const core::ffi::c_void,
        );
    }
    unsafe { w + sub3(5) + sub1(6) }
}

/* sub5() is an identitify function, just to test weirder functions layout and
 * call patterns
 */

/* unfortunately verifier rejects `struct task_struct *t` as an unknown pointer
 * type, so we need to accept pointer as integer and then cast it inside the
 * function
 */
#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_task_tgid(t: uintptr_t) -> i32 {
    /* this ensures that CO-RE relocs work in multi-subprogs .text */
    unsafe { (*(t as *mut core::ffi::c_void as *mut task_struct)).tgid }
}

#[unsafe(no_mangle)]
pub static mut res1: i32 = 0;
#[unsafe(no_mangle)]
pub static mut res2: i32 = 0;
#[unsafe(no_mangle)]
pub static mut res3: i32 = 0;
#[unsafe(no_mangle)]
pub static mut res4: i32 = 0;

#[unsafe(link_section = "raw_tp/sys_enter")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prog1(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    /* perform some CO-RE relocations to ensure they work with multi-prog
     * sections correctly
     */
    let t: *mut task_struct = unsafe { bpf_get_current_task() as *mut core::ffi::c_void as *mut task_struct };

    if unsafe { (*t).pid } == 0 || unsafe { get_task_tgid(t as uintptr_t) } == 0 {
        return 1;
    }

    unsafe {
        res1 = sub1(1) + sub3(2);
    } /* (1 + 1) + (2 + 3 + (4 + 1)) = 12 */
    0
}

#[unsafe(link_section = "raw_tp/sys_exit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prog2(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    let t: *mut task_struct = unsafe { bpf_get_current_task() as *mut core::ffi::c_void as *mut task_struct };

    if unsafe { (*t).pid } == 0 || unsafe { get_task_tgid(t as uintptr_t) } == 0 {
        return 1;
    }

    unsafe {
        res2 = sub2(3) + sub3(4);
    } /* (3 + 2) + (4 + 3 + (4 + 1)) = 17 */
    0
}

unsafe extern "C" fn empty_callback(index: __u32, data: *mut core::ffi::c_void) -> i32 {
    let _ = index;
    let _ = data;
    0
}

/* prog3 has the same section name as prog1 */
#[unsafe(link_section = "raw_tp/sys_enter")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prog3(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    let t: *mut task_struct = unsafe { bpf_get_current_task() as *mut core::ffi::c_void as *mut task_struct };

    if unsafe { (*t).pid } == 0 || unsafe { get_task_tgid(t as uintptr_t) } == 0 {
        return 1;
    }

    /* test that ld_imm64 with BPF_PSEUDO_FUNC doesn't get blinded */
    unsafe {
        bpf_loop(1, Some(empty_callback), core::ptr::null_mut(), 0);
    }

    unsafe {
        res3 = sub3(5) + 6;
    } /* (5 + 3 + (4 + 1)) + 6 = 19 */
    0
}

/* prog4 has the same section name as prog2 */
#[unsafe(link_section = "raw_tp/sys_exit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prog4(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    let t: *mut task_struct = unsafe { bpf_get_current_task() as *mut core::ffi::c_void as *mut task_struct };

    if unsafe { (*t).pid } == 0 || unsafe { get_task_tgid(t as uintptr_t) } == 0 {
        return 1;
    }

    unsafe {
        res4 = sub4(7) + sub1(8);
    } /* (7 + (5 + 3 + (4 + 1)) + (6 + 1)) + (8 + 1) = 36 */
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
