// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/util/lock-contention.c.
// C include dependencies intentionally remain external to this translation:
// debug.h, env.h, lock-contention.h, machine.h, symbol.h, linux/hash.h,
// linux/zalloc.h, limits.h, and string.h.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type u64 = u64;
type bool_ = bool;
type uint16_t = u16;

const ULLONG_MAX: u64 = u64::MAX;

const LOCKHASH_BITS: c_uint = 12;
const EM_PPC: uint16_t = 20;
const EM_PPC64: uint16_t = 21;

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct hlist_node {
    pub next: *mut hlist_node,
    pub pprev: *mut *mut hlist_node,
}

#[repr(C)]
pub struct hlist_head {
    pub first: *mut hlist_node,
}

#[repr(C)]
pub struct callstack_filter {
    pub list: list_head,
    pub name: [c_char; 0],
}

#[repr(C)]
pub struct perf_env {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    pub env: *mut perf_env,
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    pub name: *const c_char,
}

#[repr(C)]
pub struct lock_stat {
    pub hash_entry: hlist_node,
    pub addr: u64,
    pub name: *mut c_char,
    pub flags: c_int,
    pub wait_time_min: u64,
}

unsafe extern "C" {
    fn hash_long(val: c_ulong, bits: c_uint) -> c_ulong;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtok_r(s: *mut c_char, delim: *const c_char, saveptr: *mut *mut c_char) -> *mut c_char;
    fn malloc(size: usize) -> *mut c_void;
    fn zalloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> usize;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn pr_err(fmt: *const c_char, ...);
    fn perf_env__e_machine(env: *mut perf_env, e_flags: *mut c_void) -> uint16_t;
    fn machine__find_kernel_symbol(
        machine: *mut machine,
        addr: u64,
        mapp: *mut *mut map,
    ) -> *mut symbol;
}

#[inline]
unsafe fn __lockhashfn(key: u64) -> c_ulong {
    unsafe { hash_long(key as c_ulong, LOCKHASH_BITS) }
}

#[inline]
unsafe fn lockhashentry(key: u64) -> *mut hlist_head {
    unsafe { lockhash_table.add(__lockhashfn(key) as usize) }
}

#[inline]
unsafe fn list_empty(head: *const list_head) -> bool {
    unsafe { (*head).next == head as *mut list_head }
}

#[inline]
unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) {
    unsafe {
        let prev = (*head).prev;
        (*new).next = head;
        (*new).prev = prev;
        (*prev).next = new;
        (*head).prev = new;
    }
}

#[inline]
unsafe fn hlist_add_head(new: *mut hlist_node, h: *mut hlist_head) {
    unsafe {
        let first = (*h).first;
        (*new).next = first;
        if !first.is_null() {
            (*first).pprev = &mut (*new).next;
        }
        (*h).first = new;
        (*new).pprev = &mut (*h).first;
    }
}

#[inline]
unsafe fn lock_stat_from_hash_entry(ptr: *mut hlist_node) -> *mut lock_stat {
    unsafe { (ptr as *mut u8).sub(offset_of!(lock_stat, hash_entry)) as *mut lock_stat }
}

#[inline]
unsafe fn callstack_filter_from_list(ptr: *mut list_head) -> *mut callstack_filter {
    unsafe { (ptr as *mut u8).sub(offset_of!(callstack_filter, list)) as *mut callstack_filter }
}

#[inline]
unsafe fn callstack_filter_name(filter: *mut callstack_filter) -> *mut c_char {
    unsafe { ptr::addr_of_mut!((*filter).name) as *mut c_char }
}

#[unsafe(no_mangle)]
pub static mut callstack_filters: list_head = list_head {
    next: unsafe { &raw mut callstack_filters },
    prev: unsafe { &raw mut callstack_filters },
};

#[unsafe(no_mangle)]
pub static mut lockhash_table: *mut hlist_head = ptr::null_mut();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_call_stack(
    _opt: *const option,
    str_: *const c_char,
    _unset: c_int,
) -> c_int {
    let mut s: *mut c_char;
    let mut tmp: *mut c_char = ptr::null_mut();
    let mut tok: *mut c_char;
    let ret: c_int = 0;

    unsafe {
        s = strdup(str_);
        if s.is_null() {
            return -1;
        }

        tok = strtok_r(s, c", ".as_ptr(), &mut tmp);
        while !tok.is_null() {
            let entry: *mut callstack_filter;

            entry = malloc(size_of::<callstack_filter>() + strlen(tok) + 1) as *mut callstack_filter;
            if entry.is_null() {
                pr_err(c"Memory allocation failure\n".as_ptr());
                free(s as *mut c_void);
                return -1;
            }

            strcpy(callstack_filter_name(entry), tok);
            list_add_tail(ptr::addr_of_mut!((*entry).list), ptr::addr_of_mut!(callstack_filters));

            tok = strtok_r(ptr::null_mut(), c", ".as_ptr(), &mut tmp);
        }

        free(s as *mut c_void);
    }
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn needs_callstack() -> bool_ {
    unsafe { !list_empty(ptr::addr_of!(callstack_filters)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lock_stat_find(addr: u64) -> *mut lock_stat {
    let entry: *mut hlist_head;
    let mut node: *mut hlist_node;

    unsafe {
        entry = lockhashentry(addr);
        node = (*entry).first;
        while !node.is_null() {
            let ret = lock_stat_from_hash_entry(node);
            if (*ret).addr == addr {
                return ret;
            }
            node = (*node).next;
        }
    }
    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lock_stat_findnew(
    addr: u64,
    name: *const c_char,
    flags: c_int,
) -> *mut lock_stat {
    let entry: *mut hlist_head;
    let mut node: *mut hlist_node;
    let new: *mut lock_stat;

    unsafe {
        entry = lockhashentry(addr);
        node = (*entry).first;
        while !node.is_null() {
            let ret = lock_stat_from_hash_entry(node);
            if (*ret).addr == addr {
                return ret;
            }
            node = (*node).next;
        }

        new = zalloc(size_of::<lock_stat>()) as *mut lock_stat;
        if new.is_null() {
            pr_err(c"memory allocation failed\n".as_ptr());
            return ptr::null_mut();
        }

        (*new).addr = addr;
        (*new).name = strdup(name);
        if (*new).name.is_null() {
            free(new as *mut c_void);
            pr_err(c"memory allocation failed\n".as_ptr());
            return ptr::null_mut();
        }

        (*new).flags = flags;
        (*new).wait_time_min = ULLONG_MAX;

        hlist_add_head(ptr::addr_of_mut!((*new).hash_entry), entry);
        new
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn match_callstack_filter(
    machine: *mut machine,
    callstack: *mut u64,
    max_stack_depth: c_int,
) -> bool_ {
    let mut kmap: *mut map = ptr::null_mut();
    let mut sym: *mut symbol;
    let mut ip: u64;
    let e_machine: uint16_t;
    let is_powerpc: bool;

    unsafe {
        e_machine = perf_env__e_machine((*machine).env, ptr::null_mut());
        is_powerpc = e_machine == EM_PPC64 || e_machine == EM_PPC;

        if list_empty(ptr::addr_of!(callstack_filters)) {
            return true;
        }

        let mut i: c_int = 0;
        while i < max_stack_depth {
            /*
             * In powerpc, the callchain saved by kernel always includes
             * first three entries as the NIP (next instruction pointer),
             * LR (link register), and the contents of LR save area in the
             * second stack frame. In certain scenarios its possible to have
             * invalid kernel instruction addresses in either LR or the second
             * stack frame's LR. In that case, kernel will store that address as
             * zero.
             *
             * The below check will continue to look into callstack,
             * incase first or second callstack index entry has 0
             * address for powerpc.
             */
            if callstack.is_null()
                || (*callstack.add(i as usize) == 0 && (!is_powerpc || (i != 1 && i != 2)))
            {
                break;
            }

            ip = *callstack.add(i as usize);
            sym = machine__find_kernel_symbol(machine, ip, &mut kmap);
            if sym.is_null() {
                i += 1;
                continue;
            }

            let mut pos = callstack_filters.next;
            while pos != ptr::addr_of_mut!(callstack_filters) {
                let filter = callstack_filter_from_list(pos);
                if !strstr((*sym).name, callstack_filter_name(filter)).is_null() {
                    return true;
                }
                pos = (*pos).next;
            }

            i += 1;
        }
    }
    false
}
