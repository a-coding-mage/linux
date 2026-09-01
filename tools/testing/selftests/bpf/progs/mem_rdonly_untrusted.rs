// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C includes:
// <vmlinux.h>, <bpf/bpf_core_read.h>, "bpf_misc.h", "bpf_kfuncs.h",
// "../test_kmods/bpf_testmod_kfunc.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::ffi::c_void;

type u8 = ::core::primitive::u8;
type u16 = ::core::primitive::u16;
type u32 = ::core::primitive::u32;
type u64 = ::core::primitive::u64;

#[repr(C)]
pub struct task_struct {
    pub nameidata: *mut nameidata,
}

#[repr(C)]
pub struct nameidata {
    pub pathname: *mut u8,
    pub flags: u32,
}

#[repr(C)]
pub struct sock {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bpf_dynptr {
    _unused: [u8; 0],
}

const BPF_MAP_TYPE_RINGBUF: u32 = 27;
const BPF_MAP_TYPE_ARRAY: u32 = 2;
const BPF_FEAT_RDONLY_CAST_TO_VOID: i32 = 0;

unsafe extern "C" {
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_core_field_offset_nameidata_pathname() -> u64;
    fn bpf_core_enum_value_exists_bpf_features(value: i32) -> bool;
    fn bpf_rdonly_cast(p: *const c_void, btf_id: u64) -> *mut c_void;
    fn bpf_get_prandom_u32() -> u32;
    fn bpf_core_type_id_kernel_sock() -> u64;
    fn bpf_kfunc_trusted_num_test(p: *mut i32);
    fn bpf_copy_from_user(dst: *mut c_void, size: u32, unsafe_ptr: *const c_void) -> i64;
    fn bpf_ringbuf_reserve(ringbuf: *mut c_void, size: u64, flags: u64) -> *mut c_void;
    fn bpf_ringbuf_discard(data: *mut c_void, flags: u64);
    fn bpf_dynptr_from_mem(data: *mut c_void, size: u32, flags: u64, ptr: *mut bpf_dynptr) -> i32;
    fn bpf_dynptr_slice(ptr: *mut bpf_dynptr, offset: u32, buffer: *mut c_void, buffer__sz: u32) -> *mut c_void;
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
}

// SEC("tp_btf/sys_enter")
// __success
// __log_level(2)
// __msg("r8 = *(u64 *)(r7 +0)          ; R7=ptr_nameidata(imm={{[0-9]+}}) R8=rdonly_untrusted_mem(sz=0)")
// __msg("r9 = *(u8 *)(r8 +0)           ; R8=rdonly_untrusted_mem(sz=0) R9=scalar")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn btf_id_to_ptr_mem(ctx: *mut c_void) -> i32 {
    let task: *mut task_struct;
    let idata: *mut nameidata;
    let ret: u64;
    let off: u64;

    task = unsafe { bpf_get_current_task_btf() };
    idata = unsafe { (*task).nameidata };
    off = unsafe { bpf_core_field_offset_nameidata_pathname() };
    /*
     * asm block to have reliable match target for __msg, equivalent of:
     *   ret = task->nameidata->pathname[0];
     */
    unsafe {
        asm!(
            "r7 = {idata};",
            "r7 += {off};",
            "r8 = *(u64 *)(r7 + 0);",
            "r9 = *(u8 *)(r8 + 0);",
            "{ret} = r9;",
            ret = lateout(reg) ret,
            idata = in(reg) idata,
            off = in(reg) off,
            out("r7") _,
            out("r8") _,
            out("r9") _,
        );
    }
    let _ = ctx;
    ret as i32
}

// SEC("socket")
// __success
// __retval(0)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ldx_is_ok_bad_addr(ctx: *mut c_void) -> i32 {
    let p: *mut i8;

    if !unsafe { bpf_core_enum_value_exists_bpf_features(BPF_FEAT_RDONLY_CAST_TO_VOID) } {
        return 42;
    }

    p = unsafe { bpf_rdonly_cast(core::ptr::null(), 0) as *mut i8 };
    let _ = ctx;
    unsafe { *p.add(0x7fff) as i32 }
}

// SEC("socket")
// __success
// __retval(1)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ldx_is_ok_good_addr(ctx: *mut c_void) -> i32 {
    let mut v: i32;
    let p: *mut i32;

    v = 1;
    p = unsafe { bpf_rdonly_cast((&mut v as *mut i32).cast(), 0) as *mut i32 };
    let _ = ctx;
    unsafe { *p }
}

// SEC("socket")
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn offset_not_tracked(ctx: *mut c_void) -> i32 {
    let mut p: *mut i32;
    let mut i: i32;
    let mut s: i32;

    p = unsafe { bpf_rdonly_cast(core::ptr::null(), 0) as *mut i32 };
    s = 0;
    i = 0;
    while i < 1000 * 1000 * 1000 {
        p = unsafe { p.add(1) };
        s = s.wrapping_add(unsafe { *p });
        i += 1;
    }
    let _ = ctx;
    s
}

// SEC("socket")
// __failure
// __msg("cannot write into rdonly_untrusted_mem")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stx_not_ok(ctx: *mut c_void) -> i32 {
    let mut v: i32;
    let p: *mut i32;

    v = 1;
    p = unsafe { bpf_rdonly_cast((&mut v as *mut i32).cast(), 0) as *mut i32 };
    unsafe { *p = 1 };
    let _ = ctx;
    0
}

// SEC("socket")
// __failure
// __msg("cannot write into rdonly_untrusted_mem")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn atomic_not_ok(ctx: *mut c_void) -> i32 {
    let mut v: i32;
    let p: *mut i32;

    v = 1;
    p = unsafe { bpf_rdonly_cast((&mut v as *mut i32).cast(), 0) as *mut i32 };
    unsafe {
        core::intrinsics::atomic_xadd_seqcst(p, 1);
    }
    let _ = ctx;
    0
}

// SEC("socket")
// __failure
// __msg("cannot write into rdonly_untrusted_mem")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn atomic_rmw_not_ok(ctx: *mut c_void) -> i32 {
    let mut v: i64;
    let p: *mut i64;

    v = 1;
    p = unsafe { bpf_rdonly_cast((&mut v as *mut i64).cast(), 0) as *mut i64 };
    let _ = ctx;
    unsafe { core::intrinsics::atomic_cxchg_seqcst_seqcst(p, 0, 42).0 as i32 }
}

// SEC("socket")
// __failure
// __msg("invalid access to memory, mem_size=0 off=0 size=4")
// __msg("R1 min value is outside of the allowed memory range")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_param_not_ok(ctx: *mut c_void) -> i32 {
    let p: *mut i32;

    p = unsafe { bpf_rdonly_cast(core::ptr::null(), 0) as *mut i32 };
    unsafe { bpf_kfunc_trusted_num_test(p) };
    let _ = ctx;
    0
}

// SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
// __failure
// __msg("R1 type=rdonly_untrusted_mem expected=")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn helper_param_not_ok(ctx: *mut c_void) -> i32 {
    let p: *mut i8;

    p = unsafe { bpf_rdonly_cast(core::ptr::null(), 0) as *mut i8 };
    /*
     * Any helper with ARG_MEM_SIZE_OR_ZERO constraint will do,
     * the most permissive constraint
     */
    unsafe { bpf_copy_from_user(p.cast(), 0, 42 as *mut c_void) };
    let _ = ctx;
    0
}

#[inline(never)]
unsafe fn get_some_addr() -> *mut u64 {
    if unsafe { bpf_get_prandom_u32() } != 0 {
        unsafe { bpf_rdonly_cast(core::ptr::null(), bpf_core_type_id_kernel_sock()) as *mut u64 }
    } else {
        unsafe { bpf_rdonly_cast(core::ptr::null(), 0) as *mut u64 }
    }
}

// SEC("socket")
// __success
// __retval(0)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mixed_mem_type(ctx: *mut c_void) -> i32 {
    let p: *mut u64;

    /* Try to avoid compiler hoisting load to if branches by using __noinline func. */
    p = unsafe { get_some_addr() };
    let _ = ctx;
    unsafe { *p as i32 }
}

#[repr(C)]
pub struct ringbuf_map {
    pub type_: u32,
    pub max_entries: u32,
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut ringbuf: ringbuf_map = ringbuf_map {
    type_: BPF_MAP_TYPE_RINGBUF,
    max_entries: 4096,
};

#[repr(C)]
pub struct array_map {
    pub type_: u32,
    pub max_entries: u32,
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut array: array_map = array_map {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
};

#[unsafe(no_mangle)]
pub static mut dynptr_data: [i8; 8] = [0; 8];

#[unsafe(no_mangle)]
pub static mut zero: i32 = 0;

// SEC("socket")
// __success
// __log_level(2)
// __msg("r8 = *(u64 *)(r7 +0){{.*}}R7=untrusted_ptr_sock")
// __msg("r8 = *(u64 *)(r7 +0){{.*}}R7=ringbuf_mem")
// __retval(0)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mixed_mem_untrusted_btf_id_type(ctx: *mut c_void) -> i32 {
    let p: *mut u64;
    let q: *mut u64;
    let v: u64;

    p = unsafe { bpf_ringbuf_reserve((&raw mut ringbuf).cast(), core::mem::size_of::<u64>() as u64, 0) as *mut u64 };
    if p.is_null() {
        return 1;
    }
    unsafe { *p = 42 };
    q = unsafe { bpf_rdonly_cast(core::ptr::null(), bpf_core_type_id_kernel_sock()) as *mut u64 };
    /*
     * The load below is reached with PTR_TO_MEM | MEM_RINGBUF on one
     * path and with PTR_TO_BTF_ID | PTR_UNTRUSTED on the other. The
     * merged type has to keep the BPF_PROBE_MEM rewrite, otherwise
     * the NULL deref taken at runtime panics the kernel instead of
     * returning 0.
     */
    unsafe {
        asm!(
            "r7 = {p};",
            "if {zero} != 0 goto +1;",
            "r7 = {q};",
            "r8 = *(u64 *)(r7 + 0);",
            "{v} = r8;",
            v = lateout(reg) v,
            p = in(reg) p,
            q = in(reg) q,
            zero = in(reg) zero,
            out("r7") _,
            out("r8") _,
        );
        bpf_ringbuf_discard(p.cast(), 0);
    }
    let _ = ctx;
    v as i32
}

// SEC("socket")
// __success
// __log_level(2)
// __msg("r8 = *(u32 *)(r7 +0){{.*}}R7=ptr_nameidata")
// __msg("r8 = *(u32 *)(r7 +0){{.*}}R7=ringbuf_mem")
// __retval(0)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mixed_mem_btf_id_type(ctx: *mut c_void) -> i32 {
    let task: *mut task_struct;
    let p: *mut u32;
    let q: *mut u32;
    let v: u64;

    p = unsafe { bpf_ringbuf_reserve((&raw mut ringbuf).cast(), core::mem::size_of::<u32>() as u64, 0) as *mut u32 };
    if p.is_null() {
        return 1;
    }
    unsafe { *p = 42 };
    task = unsafe { bpf_get_current_task_btf() };
    /*
     * A plain BTF pointer walk yields a bare PTR_TO_BTF_ID, and
     * task->nameidata is NULL unless the task currently is in the
     * middle of a path lookup.
     */
    q = unsafe { (&raw mut (*(*task).nameidata).flags).cast() };
    /*
     * Same as above, except that the other path yields a bare
     * PTR_TO_BTF_ID. Merging it with PTR_TO_MEM used to drop the
     * BPF_PROBE_MEM rewrite the bare PTR_TO_BTF_ID would have
     * gotten on its own.
     */
    unsafe {
        asm!(
            "r7 = {p};",
            "if {zero} != 0 goto +1;",
            "r7 = {q};",
            "r8 = *(u32 *)(r7 + 0);",
            "{v} = r8;",
            v = lateout(reg) v,
            p = in(reg) p,
            q = in(reg) q,
            zero = in(reg) zero,
            out("r7") _,
            out("r8") _,
        );
        bpf_ringbuf_discard(p.cast(), 0);
    }
    let _ = ctx;
    v as i32
}

// SEC("socket")
// __success
// __log_level(2)
// __msg("r8 = *(u32 *)(r7 +0){{.*}}R7=ptr_nameidata")
// __msg("r8 = *(u32 *)(r7 +0){{.*}}R7=rdonly_mem")
// __retval(0)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mixed_rdonly_mem_btf_id_type(ctx: *mut c_void) -> i32 {
    let task: *mut task_struct;
    let mut dptr: bpf_dynptr;
    let mut buf: [i8; core::mem::size_of::<u32>()];
    let p: *mut u32;
    let q: *mut u32;
    let v: u64;

    dptr = core::mem::zeroed();
    buf = [0; core::mem::size_of::<u32>()];
    if unsafe { bpf_dynptr_from_mem((&raw mut dynptr_data).cast(), core::mem::size_of_val(&dynptr_data) as u32, 0, &mut dptr) } != 0 {
        return 1;
    }
    p = unsafe { bpf_dynptr_slice(&mut dptr, 0, buf.as_mut_ptr().cast(), core::mem::size_of_val(&buf) as u32) as *mut u32 };
    if p.is_null() {
        return 1;
    }
    task = unsafe { bpf_get_current_task_btf() };
    q = unsafe { (&raw mut (*(*task).nameidata).flags).cast() };
    /*
     * Same as above, except that the PTR_TO_MEM side already carries
     * MEM_RDONLY. Merging it with a bare PTR_TO_BTF_ID used to yield
     * PTR_TO_MEM | MEM_RDONLY, which is not rewritten either since
     * only its PTR_UNTRUSTED variant is.
     */
    unsafe {
        asm!(
            "r7 = {p};",
            "if {zero} != 0 goto +1;",
            "r7 = {q};",
            "r8 = *(u32 *)(r7 + 0);",
            "{v} = r8;",
            v = lateout(reg) v,
            p = in(reg) p,
            q = in(reg) q,
            zero = in(reg) zero,
            out("r7") _,
            out("r8") _,
        );
    }
    let _ = ctx;
    v as i32
}

// SEC("socket")
// __success
// __log_level(2)
// __msg("r8 = *(u64 *)(r7 +0){{.*}}R7=ringbuf_mem")
// __msg("r8 = *(u64 *)(r7 +0){{.*}}R7=rdonly_untrusted_mem")
// __retval(0)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mixed_mem_mem_type(ctx: *mut c_void) -> i32 {
    let p: *mut u64;
    let q: *mut u64;
    let v: u64;

    p = unsafe { bpf_ringbuf_reserve((&raw mut ringbuf).cast(), core::mem::size_of::<u64>() as u64, 0) as *mut u64 };
    if p.is_null() {
        return 1;
    }
    unsafe { *p = 42 };
    q = unsafe { bpf_rdonly_cast(core::ptr::null(), 0) as *mut u64 };
    /*
     * Both paths are PTR_TO_MEM based, so they used to not trip the
     * type mismatch check and skipped the merge altogether, leaving
     * the insn with the PTR_TO_MEM | MEM_RINGBUF recorded first and
     * hence without the BPF_PROBE_MEM rewrite the other path needs.
     */
    unsafe {
        asm!(
            "r7 = {q};",
            "if {zero} == 0 goto +1;",
            "r7 = {p};",
            "r8 = *(u64 *)(r7 + 0);",
            "{v} = r8;",
            v = lateout(reg) v,
            p = in(reg) p,
            q = in(reg) q,
            zero = in(reg) zero,
            out("r7") _,
            out("r8") _,
        );
        bpf_ringbuf_discard(p.cast(), 0);
    }
    let _ = ctx;
    v as i32
}

// SEC("socket")
// __failure
// __msg("same insn cannot be used with different pointers")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mixed_map_value_mem_type(ctx: *mut c_void) -> i32 {
    let p: *mut u64;
    let q: *mut u64;
    let v: u64;
    let key: u32 = 0;

    p = unsafe { bpf_map_lookup_elem((&raw mut array).cast(), (&key as *const u32).cast()) as *mut u64 };
    if p.is_null() {
        return 1;
    }
    q = unsafe { bpf_rdonly_cast(core::ptr::null(), 0) as *mut u64 };
    /*
     * PTR_TO_MAP_VALUE is neither PTR_TO_MEM nor PTR_TO_BTF_ID based,
     * so it cannot be merged into a type which keeps the BPF_PROBE_MEM
     * rewrite the PTR_TO_MEM | MEM_RDONLY | PTR_UNTRUSTED of the other
     * path needs. Both bases were mismatch ok, hence the load used to be
     * accepted with the PTR_TO_MAP_VALUE recorded and the NULL deref on
     * the second path panicked the kernel.
     */
    unsafe {
        asm!(
            "r7 = {q};",
            "if {zero} == 0 goto +1;",
            "r7 = {p};",
            "r8 = *(u64 *)(r7 + 0);",
            "{v} = r8;",
            v = lateout(reg) v,
            p = in(reg) p,
            q = in(reg) q,
            zero = in(reg) zero,
            out("r7") _,
            out("r8") _,
        );
    }
    let _ = ctx;
    v as i32
}

// SEC("socket")
// __failure
// __msg("same insn cannot be used with different pointers")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mixed_stack_mem_type(ctx: *mut c_void) -> i32 {
    let p: *mut u64 = unsafe { bpf_rdonly_cast(core::ptr::null(), 0) as *mut u64 };
    let s: u64 = 42;
    let v: u64;

    /*
     * Same as above, but for a PTR_TO_STACK on the other path.
     */
    unsafe {
        asm!(
            "r7 = {p};",
            "if {zero} == 0 goto +1;",
            "r7 = {s};",
            "r8 = *(u64 *)(r7 + 0);",
            "{v} = r8;",
            v = lateout(reg) v,
            p = in(reg) p,
            s = in(reg) &s,
            zero = in(reg) zero,
            out("r7") _,
            out("r8") _,
        );
    }
    let _ = ctx;
    v as i32
}

#[repr(align(8))]
pub struct aligned_global(pub [u8; 9]);

#[unsafe(no_mangle)]
pub static mut global: aligned_global = aligned_global([
    0x11, 0x22, 0x33, 0x44,
    0x55, 0x66, 0x77, 0x88,
    0x99,
]);

#[inline(always)]
unsafe fn combine(p: *mut c_void) -> u64 {
    let mut acc: u64;

    acc = 0;
    #[cfg(target_endian = "little")]
    {
        acc |= ((unsafe { *(p as *mut u64) } >> 56) << 24) as u64;
        acc |= ((unsafe { *(p as *mut u32) } >> 24) << 16) as u64;
        acc |= ((unsafe { *(p as *mut u16) } >> 8) << 8) as u64;
        acc |= unsafe { *(p as *mut u8) } as u64;
    }
    #[cfg(not(target_endian = "little"))]
    {
        acc |= ((unsafe { *(p as *mut u64) } & 0xff) << 24) as u64;
        acc |= ((unsafe { *(p as *mut u32) } & 0xff) << 16) as u64;
        acc |= ((unsafe { *(p as *mut u16) } & 0xff) << 8) as u64;
        acc |= unsafe { *(p as *mut u8) } as u64;
    }
    acc
}

// SEC("socket")
// __retval(0x88442211)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn diff_size_access(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    unsafe { combine(bpf_rdonly_cast((&raw mut global).cast(), 0)) as i32 }
}

// SEC("socket")
// __retval(0x99553322)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn misaligned_access(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    unsafe { combine((bpf_rdonly_cast((&raw mut global).cast(), 0) as *mut u8).add(1).cast()) as i32 }
}

#[unsafe(no_mangle)]
pub extern "C" fn return_one() -> i32 {
    1
}

// SEC("socket")
// __success
// __retval(1)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn null_check(ctx: *mut c_void) -> i32 {
    let p: *mut i32;

    p = unsafe { bpf_rdonly_cast(core::ptr::null(), 0) as *mut i32 };
    if p.is_null() {
        /* make this a function call to avoid compiler
         * moving r0 assignment before check.
         */
        return return_one();
    }
    let _ = ctx;
    0
}

// SEC("socket")
// __success
// __retval(1)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ldx_is_ok_commuted_addr(ctx: *mut c_void) -> i32 {
    let mut v: i32;
    let p: *mut i32;
    let derived: *mut i32;

    v = 1;
    p = unsafe { bpf_rdonly_cast((&mut v as *mut i32).cast(), 0) as *mut i32 };
    unsafe {
        asm!(
            "{dst} = 0;",
            "{dst} += {src};",
            dst = lateout(reg) derived,
            src = in(reg) p,
            options(nostack),
        );
    }
    let _ = ctx;
    unsafe { *derived }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
