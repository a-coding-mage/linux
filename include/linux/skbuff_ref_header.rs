/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *	Skb ref helpers.
 */

// Dependency declarations supplied by the Linux skbuff translation.

/// __skb_frag_ref - take an addition reference on a paged fragment.
/// @frag: the paged fragment
///
/// Takes an additional reference on the paged fragment @frag.
#[inline(always)]
pub unsafe fn __skb_frag_ref(frag: *mut skb_frag_t) {
    get_netmem(skb_frag_netmem(frag));
}

/// skb_frag_ref - take an addition reference on a paged fragment of an skb.
/// @skb: the buffer
/// @f: the fragment offset.
///
/// Takes an additional reference on the @f'th paged fragment of @skb.
#[inline(always)]
pub unsafe fn skb_frag_ref(skb: *mut sk_buff, f: i32) {
    __skb_frag_ref(&mut (*skb_shinfo(skb)).frags[f as usize]);
}

unsafe extern "C" {
    pub fn napi_pp_put_page(netmem: netmem_ref) -> bool;
}

#[inline(always)]
pub unsafe fn skb_page_unref(netmem: netmem_ref, recycle: bool) {
    // CONFIG_PAGE_POOL is a build-time condition from the original header.
    #[cfg(feature = "CONFIG_PAGE_POOL")]
    {
        if recycle && napi_pp_put_page(netmem) {
            return;
        }
    }
    put_netmem(netmem);
}

/// __skb_frag_unref - release a reference on a paged fragment.
/// @frag: the paged fragment
/// @recycle: recycle the page if allocated via page_pool
///
/// Releases a reference on the paged fragment @frag
/// or recycles the page via the page_pool API.
#[inline(always)]
pub unsafe fn __skb_frag_unref(frag: *mut skb_frag_t, recycle: bool) {
    skb_page_unref(skb_frag_netmem(frag), recycle);
}

/// skb_frag_unref - release a reference on a paged fragment of an skb.
/// @skb: the buffer
/// @f: the fragment offset
///
/// Releases a reference on the @f'th paged fragment of @skb.
#[inline(always)]
pub unsafe fn skb_frag_unref(skb: *mut sk_buff, f: i32) {
    let shinfo: *mut skb_shared_info = skb_shinfo(skb);

    if !skb_zcopy_managed(skb) {
        __skb_frag_unref(
            &mut (*shinfo).frags[f as usize],
            (*skb).pp_recycle,
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
