// SPDX-License-Identifier: GPL-2.0-only
//! Original initcall blacklist allocation, registration and symbol matching.

use super::{bindings, main_initcall_types::InitcallFn};
#[cfg(not(CONFIG_KALLSYMS))]
use super::main_printk::main_printk;
#[cfg(CONFIG_KALLSYMS)]
use super::{main_debug::main_debug, main_globals};
use kernel::ffi::{c_char, c_int};

#[cfg(CONFIG_KALLSYMS)]
#[repr(C)]
struct BlacklistEntry {
    next: bindings::list_head,
    buffer: *mut c_char,
}

/// Insert an allocated entry using the original inline list_add operations.
#[cfg(CONFIG_KALLSYMS)]
unsafe fn add_entry(new: *mut bindings::list_head, head: *mut bindings::list_head) {
    // SAFETY: boot owns the list and new entry; validation follows list.h before
    // any link writes. No references span the corruption reporter boundary.
    unsafe {
        let next = (*head).next;
        #[cfg(CONFIG_LIST_HARDENED)]
        {
            #[cfg(not(CONFIG_DEBUG_LIST))]
            let valid = (*next).prev == head && (*head).next == next && new != head && new != next;
            #[cfg(CONFIG_DEBUG_LIST)]
            let valid = false;
            if !valid {
                let reported = bindings::rust_helper___list_add_valid_or_report(new, head, next);
                // With hardening alone, the original ret starts false on the
                // slow path, so ret &= reporter remains false even if it says
                // true. DEBUG_LIST takes the reporter's result directly.
                if !cfg!(CONFIG_DEBUG_LIST) || !reported {
                    return;
                }
            }
        }
        (*next).prev = new;
        (*new).next = next;
        (*new).prev = head;
        core::ptr::addr_of_mut!((*head).next).write_volatile(new);
    }
}

/// Parse and retain every comma-separated name, including empty names.
///
/// # Safety
///
/// Early boot owns the nullable writable terminated string and serializes the
/// blacklist, debug and memblock services. Allocations retain kernel lifetime.
#[cfg(CONFIG_KALLSYMS)]
#[link_section = ".init.text"]
pub(super) unsafe extern "C" fn initcall_blacklist(mut value: *mut c_char) -> c_int {
    // SAFETY: strsep mutates caller-owned storage; memblock allocations are
    // infallible here and no Rust references cross the debug/allocator calls.
    unsafe {
        loop {
            let name = bindings::strsep(&mut value, c",".as_ptr().cast());
            if name.is_null() {
                break;
            }
            main_debug!("initcall_blacklist", "blacklisting initcall %s\n", name);
            let alignment = bindings::RUST_INIT_MAIN_SMP_CACHE_BYTES as bindings::phys_addr_t;
            let function = c"initcall_blacklist".as_ptr().cast();
            let entry = bindings::__memblock_alloc_or_panic(
                core::mem::size_of::<BlacklistEntry>() as bindings::phys_addr_t,
                alignment, function,
            ).cast::<BlacklistEntry>();
            let buffer = bindings::__memblock_alloc_or_panic(
                bindings::strlen(name).wrapping_add(1) as bindings::phys_addr_t,
                alignment, function,
            ).cast::<c_char>();
            (*entry).buffer = buffer;
            bindings::strcpy(buffer, name);
            add_entry(core::ptr::addr_of_mut!((*entry).next),
                      core::ptr::addr_of_mut!(main_globals::blacklisted_initcalls));
        }
    }
    1
}

#[cfg(not(CONFIG_KALLSYMS))]
#[link_section = ".init.text"]
pub(super) unsafe extern "C" fn initcall_blacklist(_value: *mut c_char) -> c_int {
    // SAFETY: this original constant diagnostic has no variadic arguments.
    #[cfg_attr(not(CONFIG_PRINTK), allow(unused_unsafe))]
    unsafe { main_printk!("initcall_blacklist", b"\x014initcall_blacklist requires CONFIG_KALLSYMS\n\0"); }
    0
}

super::main_setup::setup_param!("initcall_blacklist=", initcall_blacklist_record, Some(initcall_blacklist), 0);

/// Match the original symbol name, stripping its optional module suffix.
///
/// # Safety
///
/// The caller serializes list access and supplies a valid initcall address for
/// the architecture's symbol/descriptor interfaces. Blacklist allocations and
/// symbol lookup/debug services are live, including module-time calls.
#[cfg_attr(not(CONFIG_MODULES), link_section = ".init.text")]
pub(super) unsafe fn initcall_blacklisted(function: InitcallFn) -> bool {
    #[cfg(not(CONFIG_KALLSYMS))]
    {
        let _ = function;
        false
    }
    #[cfg(CONFIG_KALLSYMS)]
    // SAFETY: boot/module init supplies stable list and symbol storage. Raw
    // pointer traversal follows the C list with no references across callbacks.
    unsafe {
        let head = core::ptr::addr_of_mut!(main_globals::blacklisted_initcalls);
        if core::ptr::addr_of!((*head).next).read_volatile() == head {
            return false;
        }
        let address = function as usize;
        #[cfg(CONFIG_HAVE_FUNCTION_DESCRIPTORS)]
        let address = bindings::dereference_function_descriptor(address as *mut kernel::ffi::c_void) as usize;
        let mut name = [0; bindings::RUST_INIT_MAIN_KSYM_SYMBOL_LEN as usize];
        bindings::sprint_symbol_no_offset(name.as_mut_ptr(), address as kernel::ffi::c_ulong);
        bindings::strreplace(name.as_mut_ptr(), b' ', 0);
        let mut node = (*head).next;
        while node != head {
            // next is the first repr(C) field, so container_of has zero offset.
            let entry = node.cast::<BlacklistEntry>();
            if bindings::strcmp(name.as_ptr(), (*entry).buffer) == 0 {
                main_debug!("initcall_blacklisted", "initcall %s blacklisted\n", name.as_ptr());
                return true;
            }
            node = (*node).next;
        }
        false
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
