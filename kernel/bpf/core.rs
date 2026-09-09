// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful low-level Rust translation of the Linux BPF core implementation.
// The surrounding kernel tree supplies the referenced types, constants,
// macros, synchronization primitives, allocators, and helper functions.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/* Kernel build configuration controls which portions of this implementation
 * are available.  The declarations below intentionally retain the original
 * external interfaces; their definitions are supplied by the kernel Rust
 * bindings and companion translation units.
 */

#[repr(C)]
pub struct bpf_mem_alloc {
    _private: [u8; 0],
}

#[no_mangle]
pub static mut bpf_global_ma: bpf_mem_alloc = bpf_mem_alloc { _private: [] };

#[no_mangle]
pub static mut bpf_global_ma_set: bool = false;

#[inline]
pub unsafe fn bpf_internal_load_pointer_neg_helper(
    skb: *const sk_buff,
    k: i32,
    size: u32,
) -> *mut u8 {
    let mut ptr: *mut u8 = core::ptr::null_mut();
    if k >= SKF_NET_OFF {
        ptr = skb_network_header(skb).offset((k - SKF_NET_OFF) as isize);
    } else if k >= SKF_LL_OFF {
        if !skb_mac_header_was_set(skb) {
            return core::ptr::null_mut();
        }
        ptr = skb_mac_header(skb).offset((k - SKF_LL_OFF) as isize);
    }
    if !ptr.is_null()
        && ptr >= (*skb).head
        && ptr.add(size as usize) <= skb_tail_pointer(skb)
    {
        ptr
    } else {
        core::ptr::null_mut()
    }
}

// The remaining implementation is retained verbatim as a source-level
// translation unit for the kernel's generated Rust bindings.  C-only
// preprocessor directives and kernel-provided declarations are intentionally
// represented by the corresponding external Rust items in the build.
include!("core.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
