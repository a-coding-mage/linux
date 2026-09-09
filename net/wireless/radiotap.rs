// SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause
/*
 * Radiotap parser
 *
 * Copyright 2007        Andy Green <andy@warmcat.com>
 * Copyright 2009        Johannes Berg <johannes@sipsolutions.net>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/kernel.h, linux/export.h, net/cfg80211.h,
// net/ieee80211_radiotap.h, and linux/unaligned.h.

static rtap_namespace_sizes: [struct radiotap_align_size; 22] = [
    struct radiotap_align_size { align: 8, size: 8 },
    struct radiotap_align_size { align: 1, size: 1 },
    struct radiotap_align_size { align: 1, size: 1 },
    struct radiotap_align_size { align: 2, size: 4 },
    struct radiotap_align_size { align: 2, size: 2 },
    struct radiotap_align_size { align: 1, size: 1 },
    struct radiotap_align_size { align: 1, size: 1 },
    struct radiotap_align_size { align: 2, size: 2 },
    struct radiotap_align_size { align: 2, size: 2 },
    struct radiotap_align_size { align: 2, size: 2 },
    struct radiotap_align_size { align: 1, size: 1 },
    struct radiotap_align_size { align: 1, size: 1 },
    struct radiotap_align_size { align: 1, size: 1 },
    struct radiotap_align_size { align: 1, size: 1 },
    struct radiotap_align_size { align: 2, size: 2 },
    struct radiotap_align_size { align: 2, size: 2 },
    struct radiotap_align_size { align: 1, size: 1 },
    struct radiotap_align_size { align: 1, size: 1 },
    struct radiotap_align_size { align: 1, size: 3 },
    struct radiotap_align_size { align: 4, size: 8 },
    struct radiotap_align_size { align: 2, size: 12 },
    struct radiotap_align_size { align: 0, size: 0 },
];

static radiotap_ns: struct ieee80211_radiotap_namespace = struct ieee80211_radiotap_namespace {
    n_bits: rtap_namespace_sizes.len(),
    align_size: rtap_namespace_sizes.as_ptr(),
};

pub unsafe fn ieee80211_radiotap_iterator_init(
    iterator: *mut struct ieee80211_radiotap_iterator,
    radiotap_header: *mut struct ieee80211_radiotap_header,
    max_length: i32,
    vns: *const struct ieee80211_radiotap_vendor_namespaces,
) -> i32 {
    if max_length < core::mem::size_of::<struct ieee80211_radiotap_header>() as i32 { return -EINVAL; }
    if (*radiotap_header).it_version != 0 { return -EINVAL; }
    if max_length < get_unaligned_le16(&(*radiotap_header).it_len) as i32 { return -EINVAL; }

    (*iterator)._rtheader = radiotap_header;
    (*iterator)._max_length = get_unaligned_le16(&(*radiotap_header).it_len) as usize;
    (*iterator)._arg_index = 0;
    (*iterator)._bitmap_shifter = get_unaligned_le32(&(*radiotap_header).it_present);
    (*iterator)._arg = (*radiotap_header).it_optional.as_mut_ptr();
    (*iterator)._reset_on_ext = 0;
    (*iterator)._next_bitmap = (*radiotap_header).it_optional.as_mut_ptr();
    (*iterator)._vns = vns;
    (*iterator).current_namespace = &radiotap_ns;
    (*iterator).is_radiotap_ns = 1;

    if (*iterator)._bitmap_shifter & BIT(IEEE80211_RADIOTAP_EXT) != 0 {
        if (*iterator)._arg.offset_from((*iterator)._rtheader as *mut u8) as usize
            + core::mem::size_of::<u32>() > (*iterator)._max_length { return -EINVAL; }
        while get_unaligned_le32((*iterator)._arg) & BIT(IEEE80211_RADIOTAP_EXT) != 0 {
            (*iterator)._arg = (*iterator)._arg.add(core::mem::size_of::<u32>());
            if (*iterator)._arg.offset_from((*iterator)._rtheader as *mut u8) as usize
                + core::mem::size_of::<u32>() > (*iterator)._max_length { return -EINVAL; }
        }
        (*iterator)._arg = (*iterator)._arg.add(core::mem::size_of::<u32>());
    }
    (*iterator).this_arg = (*iterator)._arg;
    0
}

unsafe fn find_ns(iterator: *mut struct ieee80211_radiotap_iterator, oui: u32, subns: u8) {
    (*iterator).current_namespace = core::ptr::null();
    if (*iterator)._vns.is_null() { return; }
    for i in 0..(*(*iterator)._vns).n_ns {
        let ns = (*(*iterator)._vns).ns.add(i as usize);
        if (*ns).oui != oui || (*ns).subns != subns { continue; }
        (*iterator).current_namespace = ns;
        break;
    }
}

pub unsafe fn ieee80211_radiotap_iterator_next(
    iterator: *mut struct ieee80211_radiotap_iterator,
) -> i32 {
    loop {
        let mut hit = 0;
        let (mut pad, mut align, mut size, mut subns): (usize, usize, usize, i32);
        let mut oui: u32;
        if (*iterator)._arg_index % 32 == IEEE80211_RADIOTAP_EXT && (*iterator)._bitmap_shifter & 1 == 0 { return -ENOENT; }
        if (*iterator)._bitmap_shifter & 1 == 0 { goto_next_entry(iterator); continue; }
        match (*iterator)._arg_index % 32 {
            IEEE80211_RADIOTAP_RADIOTAP_NAMESPACE | IEEE80211_RADIOTAP_EXT => { align = 1; size = 0; }
            IEEE80211_RADIOTAP_VENDOR_NAMESPACE => { align = 2; size = 6; }
            _ => {
                if (*iterator).current_namespace.is_null() || (*iterator)._arg_index >= (*(*iterator).current_namespace).n_bits { align = 0; size = 0; }
                else { align = (*(*iterator).current_namespace).align_size.add((*iterator)._arg_index as usize).read().align as usize; size = (*(*iterator).current_namespace).align_size.add((*iterator)._arg_index as usize).read().size as usize; }
                if align == 0 {
                    if (*iterator).current_namespace == &radiotap_ns { return -ENOENT; }
                    (*iterator)._arg = (*iterator)._next_ns_data; (*iterator).current_namespace = core::ptr::null(); goto_next_entry(iterator); continue;
                }
            }
        }
        pad = ((*iterator)._arg.offset_from((*iterator)._rtheader as *mut u8) as usize) & (align - 1);
        if pad != 0 { (*iterator)._arg = (*iterator)._arg.add(align - pad); }
        if (*iterator)._arg_index % 32 == IEEE80211_RADIOTAP_VENDOR_NAMESPACE {
            if (*iterator)._arg.offset_from((*iterator)._rtheader as *mut u8) as usize + size > (*iterator)._max_length { return -EINVAL; }
            oui = ((*(*iterator)._arg as u32) << 16) | ((*(*iterator)._arg.add(1) as u32) << 8) | *(*iterator)._arg.add(2) as u32;
            subns = *(*iterator)._arg.add(3); find_ns(iterator, oui, subns as u8);
            let vnslen = get_unaligned_le16((*iterator)._arg.add(4));
            (*iterator)._next_ns_data = (*iterator)._arg.add(size + vnslen as usize);
            if (*iterator).current_namespace.is_null() { size += vnslen as usize; }
        }
        (*iterator).this_arg_index = (*iterator)._arg_index; (*iterator).this_arg = (*iterator)._arg; (*iterator).this_arg_size = size; (*iterator)._arg = (*iterator)._arg.add(size);
        if (*iterator)._arg.offset_from((*iterator)._rtheader as *mut u8) as usize > (*iterator)._max_length { return -EINVAL; }
        match (*iterator)._arg_index % 32 {
            IEEE80211_RADIOTAP_VENDOR_NAMESPACE => { (*iterator)._reset_on_ext = 1; (*iterator).is_radiotap_ns = 0; (*iterator).this_arg_index = IEEE80211_RADIOTAP_VENDOR_NAMESPACE; if (*iterator).current_namespace.is_null() { hit = 1; } goto_next_entry(iterator); }
            IEEE80211_RADIOTAP_RADIOTAP_NAMESPACE => { (*iterator)._reset_on_ext = 1; (*iterator).current_namespace = &radiotap_ns; (*iterator).is_radiotap_ns = 1; goto_next_entry(iterator); }
            IEEE80211_RADIOTAP_EXT => { (*iterator)._bitmap_shifter = get_unaligned_le32((*iterator)._next_bitmap); (*iterator)._next_bitmap = (*iterator)._next_bitmap.add(1); if (*iterator)._reset_on_ext != 0 { (*iterator)._arg_index = 0; } else { (*iterator)._arg_index += 1; } (*iterator)._reset_on_ext = 0; }
            _ => { hit = 1; goto_next_entry(iterator); }
        }
        if hit != 0 { return 0; }
    }
}

#[inline(always)] unsafe fn goto_next_entry(iterator: *mut struct ieee80211_radiotap_iterator) { (*iterator)._bitmap_shifter >>= 1; (*iterator)._arg_index += 1; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
