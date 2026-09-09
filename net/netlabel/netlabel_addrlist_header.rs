/* SPDX-License-Identifier: GPL-2.0-or-later */
/* NetLabel Network Address Lists -- Rust translation of netlabel_addrlist.h */

/* Linux dependencies are supplied by the surrounding translation unit. */

#[repr(C)]
pub struct netlbl_af4list {
    pub addr: __be32,
    pub mask: __be32,
    pub valid: u32,
    pub list: list_head,
}

#[repr(C)]
pub struct netlbl_af6list {
    pub addr: in6_addr,
    pub mask: in6_addr,
    pub valid: u32,
    pub list: list_head,
}

/* These types and list/RCU primitives are provided by the Linux bindings. */
pub type __be32 = u32;
pub type c_int = i32;
pub type c_char = i8;

extern "C" {
    pub fn netlbl_af4list_add(entry: *mut netlbl_af4list, head: *mut list_head) -> c_int;
    pub fn netlbl_af4list_remove(addr: __be32, mask: __be32, head: *mut list_head)
        -> *mut netlbl_af4list;
    pub fn netlbl_af4list_remove_entry(entry: *mut netlbl_af4list);
    pub fn netlbl_af4list_search(addr: __be32, head: *mut list_head) -> *mut netlbl_af4list;
    pub fn netlbl_af4list_search_exact(
        addr: __be32,
        mask: __be32,
        head: *mut list_head,
    ) -> *mut netlbl_af4list;

    pub fn netlbl_af6list_add(entry: *mut netlbl_af6list, head: *mut list_head) -> c_int;
    pub fn netlbl_af6list_remove(
        addr: *const in6_addr,
        mask: *const in6_addr,
        head: *mut list_head,
    ) -> *mut netlbl_af6list;
    pub fn netlbl_af6list_remove_entry(entry: *mut netlbl_af6list);
    pub fn netlbl_af6list_search(addr: *const in6_addr, head: *mut list_head)
        -> *mut netlbl_af6list;
    pub fn netlbl_af6list_search_exact(
        addr: *const in6_addr,
        mask: *const in6_addr,
        head: *mut list_head,
    ) -> *mut netlbl_af6list;
}

/* The C container_of operation is represented by the corresponding field-offset cast. */
#[inline]
pub unsafe fn __af4list_entry(ptr: *mut list_head) -> *mut netlbl_af4list {
    ptr as *mut netlbl_af4list
}

#[inline]
pub unsafe fn __af4list_valid(
    s: *mut list_head,
    h: *mut list_head,
) -> *mut netlbl_af4list {
    let mut i = s;
    let mut n = __af4list_entry(s);
    while i != h && (*n).valid == 0 {
        i = (*i).next;
        n = __af4list_entry(i);
    }
    n
}

#[inline]
pub unsafe fn __af4list_valid_rcu(
    s: *mut list_head,
    h: *mut list_head,
) -> *mut netlbl_af4list {
    let mut i = s;
    let mut n = __af4list_entry(s);
    while i != h && (*n).valid == 0 {
        i = (*i).next;
        n = __af4list_entry(i);
    }
    n
}

#[inline]
pub unsafe fn netlbl_af4list_audit_addr(
    _audit_buf: *mut audit_buffer,
    _src: c_int,
    _dev: *const c_char,
    _addr: __be32,
    _mask: __be32,
) {
    /* CONFIG_AUDIT-disabled inline implementation. */
}

#[cfg(feature = "CONFIG_AUDIT")]
extern "C" {
    pub fn netlbl_af4list_audit_addr_external(
        audit_buf: *mut audit_buffer,
        src: c_int,
        dev: *const c_char,
        addr: __be32,
        mask: __be32,
    );
}

#[inline]
pub unsafe fn __af6list_entry(ptr: *mut list_head) -> *mut netlbl_af6list {
    ptr as *mut netlbl_af6list
}

#[inline]
pub unsafe fn __af6list_valid(
    s: *mut list_head,
    h: *mut list_head,
) -> *mut netlbl_af6list {
    let mut i = s;
    let mut n = __af6list_entry(s);
    while i != h && (*n).valid == 0 {
        i = (*i).next;
        n = __af6list_entry(i);
    }
    n
}

#[inline]
pub unsafe fn __af6list_valid_rcu(
    s: *mut list_head,
    h: *mut list_head,
) -> *mut netlbl_af6list {
    let mut i = s;
    let mut n = __af6list_entry(s);
    while i != h && (*n).valid == 0 {
        i = (*i).next;
        n = __af6list_entry(i);
    }
    n
}

#[inline]
pub unsafe fn netlbl_af6list_audit_addr(
    _audit_buf: *mut audit_buffer,
    _src: c_int,
    _dev: *const c_char,
    _addr: *const in6_addr,
    _mask: *const in6_addr,
) {
    /* CONFIG_AUDIT-disabled inline implementation. */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
