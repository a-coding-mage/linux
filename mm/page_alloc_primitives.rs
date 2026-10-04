// SPDX-License-Identifier: GPL-2.0-only
// Header algorithms used by the allocator, with pointer writes preserving the
// original READ_ONCE/WRITE_ONCE boundary and list hardening diagnostics.
#[inline]
unsafe fn page_buddy_list(p: *mut page) -> *mut list_head {
    rust_pa_page_buddy_list(p)
}
#[inline]
unsafe fn page_pcp_list(p: *mut page) -> *mut list_head {
    rust_pa_page_pcp_list(p)
}
#[inline]
unsafe fn page_private(p: *const page) -> ULong {
    rust_pa_page_private(p)
}
#[inline]
unsafe fn set_page_private(p: *mut page, v: ULong) {
    rust_pa_set_page_private(p, v);
}
#[inline]
unsafe fn list_empty(h: *const list_head) -> bool {
    rust_pa_read_list_ptr(addr_of!((*h).next)) == h.cast_mut()
}
#[inline]
unsafe fn list_add_valid(new: *mut list_head, prev: *mut list_head, next: *mut list_head) -> bool {
    #[cfg(CONFIG_LIST_HARDENED)]
    {
        #[cfg(not(CONFIG_DEBUG_LIST))]
        {
            if (*next).prev == prev && (*prev).next == next && new != prev && new != next {
                return true;
            }
            rust_pa_list_add_report(new, prev, next);
            return false;
        }
        #[cfg(CONFIG_DEBUG_LIST)]
        {
            return rust_pa_list_add_report(new, prev, next);
        }
    }
    #[cfg(not(CONFIG_LIST_HARDENED))]
    {
        true
    }
}
#[inline]
unsafe fn list_del_valid(p: *mut list_head) -> bool {
    #[cfg(CONFIG_LIST_HARDENED)]
    {
        #[cfg(not(CONFIG_DEBUG_LIST))]
        {
            if (*(*p).prev).next == p && (*(*p).next).prev == p {
                return true;
            }
            rust_pa_list_del_report(p);
            return false;
        }
        #[cfg(CONFIG_DEBUG_LIST)]
        {
            return rust_pa_list_del_report(p);
        }
    }
    #[cfg(not(CONFIG_LIST_HARDENED))]
    {
        true
    }
}
#[inline]
unsafe fn __list_add(new: *mut list_head, prev: *mut list_head, next: *mut list_head) {
    if !list_add_valid(new, prev, next) {
        return;
    }
    (*next).prev = new;
    (*new).next = next;
    (*new).prev = prev;
    rust_pa_write_list_ptr(addr_of_mut!((*prev).next), new);
}
#[inline]
unsafe fn list_add(new: *mut list_head, head: *mut list_head) {
    __list_add(new, head, (*head).next);
}
#[inline]
unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) {
    __list_add(new, (*head).prev, head);
}
#[inline]
unsafe fn __list_del_entry(p: *mut list_head) {
    if !list_del_valid(p) {
        return;
    }
    let prev = (*p).prev;
    let next = (*p).next;
    (*next).prev = prev;
    rust_pa_write_list_ptr(addr_of_mut!((*prev).next), next);
}
#[inline]
unsafe fn list_del(p: *mut list_head) {
    __list_del_entry(p);
    (*p).next = RUST_PA_LIST_POISON1 as usize as *mut list_head;
    (*p).prev = RUST_PA_LIST_POISON2 as usize as *mut list_head;
}
#[inline]
unsafe fn list_del_init(p: *mut list_head) {
    __list_del_entry(p);
    rust_pa_write_list_ptr(addr_of_mut!((*p).next), p);
    rust_pa_write_list_ptr(addr_of_mut!((*p).prev), p);
}
#[inline]
unsafe fn list_move_tail(p: *mut list_head, h: *mut list_head) {
    __list_del_entry(p);
    list_add_tail(p, h);
}
#[inline]
unsafe fn first_zone() -> *mut zone {
    addr_of_mut!((*first_online_pgdat()).node_zones).cast::<zone>()
}
#[inline]
unsafe fn set_pages_refcounted(p: *mut page, mut nr: ULong) {
    let mut pfn = rust_pa_page_to_pfn(p);
    while nr != 0 {
        rust_pa_set_page_refcounted(rust_pa_pfn_to_page(pfn));
        nr -= 1;
        pfn += 1;
    }
}
#[inline]
unsafe fn rust_pa_b_find_buddy(p: *mut page, pfn: ULong, order: u32, out: *mut ULong) -> *mut page {
    let buddy_pfn = pfn ^ (1 << order);
    let buddy = p.wrapping_offset(buddy_pfn.wrapping_sub(pfn) as isize);
    if !out.is_null() {
        *out = buddy_pfn;
    }
    if !rust_pa_b_page_is_guard(buddy) && !rust_pa_page_buddy(buddy) {
        return null_mut();
    }
    if rust_pa_buddy_order(buddy) as u32 != order {
        return null_mut();
    }
    if rust_pa_b_page_zone_id(p) != rust_pa_b_page_zone_id(buddy) {
        return null_mut();
    }
    pa_vm_bug_page!(rust_pa_page_count(buddy) != 0, buddy);
    buddy
}
