// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP Crypto driver common support routines.
 *
 * Copyright (c) 2017 Texas Instruments Incorporated
 *   Tero Kristo <t-kristo@ti.com>
 */

// Linux kernel and omap-crypto.h dependencies are supplied externally.

unsafe fn omap_crypto_copy_sg_lists(
    mut total: i32,
    _bs: i32,
    sg: *mut *mut scatterlist,
    mut new_sg: *mut scatterlist,
    flags: u16,
) -> i32 {
    let n = sg_nents(*sg);
    let mut tmp: *mut scatterlist;

    if flags & OMAP_CRYPTO_FORCE_SINGLE_ENTRY == 0 {
        new_sg = kmalloc_objs(new_sg, n);
        if new_sg.is_null() {
            return -ENOMEM;
        }

        sg_init_table(new_sg, n);
    }

    tmp = new_sg;

    while !(*sg).is_null() && total != 0 {
        let mut len = (*(*sg)).length;

        if total < len {
            len = total;
        }

        if len > 0 {
            total -= len;
            sg_set_page(tmp, sg_page(*sg), len, (*(*sg)).offset);
            if total <= 0 {
                sg_mark_end(tmp);
            }
            tmp = sg_next(tmp);
        }

        *sg = sg_next(*sg);
    }

    *sg = new_sg;

    0
}

unsafe fn omap_crypto_copy_sgs(
    total: i32,
    bs: i32,
    sg: *mut *mut scatterlist,
    new_sg: *mut scatterlist,
    flags: u16,
) -> i32 {
    let mut buf: *mut core::ffi::c_void;
    let pages: i32;
    let new_len: i32;

    new_len = ALIGN(total, bs);
    pages = get_order(new_len);

    buf = __get_free_pages(GFP_ATOMIC, pages) as *mut core::ffi::c_void;
    if buf.is_null() {
        pr_err!("{}: Couldn't allocate pages for unaligned cases.\n", __func__);
        return -ENOMEM;
    }

    if flags & OMAP_CRYPTO_COPY_DATA != 0 {
        scatterwalk_map_and_copy(buf, *sg, 0, total, 0);
        if flags & OMAP_CRYPTO_ZERO_BUF != 0 {
            memset(buf.add(total as usize), 0, (new_len - total) as usize);
        }
    }

    if flags & OMAP_CRYPTO_FORCE_SINGLE_ENTRY == 0 {
        sg_init_table(new_sg, 1);
    }

    sg_set_buf(new_sg, buf, new_len);

    *sg = new_sg;

    0
}

unsafe fn omap_crypto_check_sg(
    mut sg: *mut scatterlist,
    total: i32,
    bs: i32,
    flags: u16,
) -> i32 {
    let mut len = 0;
    let mut num_sg = 0;

    if !IS_ALIGNED(total, bs) {
        return OMAP_CRYPTO_NOT_ALIGNED;
    }

    while !sg.is_null() {
        num_sg += 1;

        if !IS_ALIGNED((*sg).offset, 4) {
            return OMAP_CRYPTO_NOT_ALIGNED;
        }
        if !IS_ALIGNED((*sg).length, bs) {
            return OMAP_CRYPTO_NOT_ALIGNED;
        }
        // CONFIG_ZONE_DMA conditional is preserved through the external build configuration.
        #[cfg(CONFIG_ZONE_DMA)]
        if page_zonenum(sg_page(sg)) != ZONE_DMA {
            return OMAP_CRYPTO_NOT_ALIGNED;
        }

        len += (*sg).length;
        sg = sg_next(sg);

        if len >= total {
            break;
        }
    }

    if flags & OMAP_CRYPTO_FORCE_SINGLE_ENTRY != 0 && num_sg > 1 {
        return OMAP_CRYPTO_NOT_ALIGNED;
    }

    if len != total {
        return OMAP_CRYPTO_BAD_DATA_LENGTH;
    }

    0
}

pub unsafe fn omap_crypto_align_sg(
    sg: *mut *mut scatterlist,
    total: i32,
    bs: i32,
    new_sg: *mut scatterlist,
    flags: u16,
    flags_shift: u8,
    dd_flags: *mut libc::c_ulong,
) -> i32 {
    let mut ret: i32;

    *dd_flags &= !(OMAP_CRYPTO_COPY_MASK << flags_shift);

    if flags & OMAP_CRYPTO_FORCE_COPY != 0 {
        ret = OMAP_CRYPTO_NOT_ALIGNED;
    } else {
        ret = omap_crypto_check_sg(*sg, total, bs, flags);
    }

    if ret == OMAP_CRYPTO_NOT_ALIGNED {
        ret = omap_crypto_copy_sgs(total, bs, sg, new_sg, flags);
        if ret != 0 {
            return ret;
        }
        *dd_flags |= OMAP_CRYPTO_DATA_COPIED << flags_shift;
    } else if ret == OMAP_CRYPTO_BAD_DATA_LENGTH {
        ret = omap_crypto_copy_sg_lists(total, bs, sg, new_sg, flags);
        if ret != 0 {
            return ret;
        }
        if flags & OMAP_CRYPTO_FORCE_SINGLE_ENTRY == 0 {
            *dd_flags |= OMAP_CRYPTO_SG_COPIED << flags_shift;
        }
    } else if flags & OMAP_CRYPTO_FORCE_SINGLE_ENTRY != 0 {
        sg_set_buf(new_sg, sg_virt(*sg), (*(*sg)).length);
    }

    0
}

unsafe fn omap_crypto_copy_data(
    mut src: *mut scatterlist,
    mut dst: *mut scatterlist,
    mut offset: i32,
    mut len: i32,
) {
    let mut amt: i32;
    let mut srcb: *mut core::ffi::c_void;
    let mut dstb: *mut core::ffi::c_void;
    let mut srco = 0;
    let mut dsto = offset;

    while !src.is_null() && !dst.is_null() && len != 0 {
        if srco >= (*src).length {
            srco -= (*src).length;
            src = sg_next(src);
            continue;
        }

        if dsto >= (*dst).length {
            dsto -= (*dst).length;
            dst = sg_next(dst);
            continue;
        }

        amt = core::cmp::min((*src).length - srco, (*dst).length - dsto);
        amt = core::cmp::min(len, amt);

        srcb = kmap_atomic(sg_page(src)).add((srco + (*src).offset) as usize);
        dstb = kmap_atomic(sg_page(dst)).add((dsto + (*dst).offset) as usize);

        memcpy(dstb, srcb, amt as usize);

        flush_dcache_page(sg_page(dst));

        kunmap_atomic(srcb);
        kunmap_atomic(dstb);

        srco += amt;
        dsto += amt;
        len -= amt;
    }
}

pub unsafe fn omap_crypto_cleanup(
    sg: *mut scatterlist,
    orig: *mut scatterlist,
    offset: i32,
    len: i32,
    flags_shift: u8,
    mut flags: libc::c_ulong,
) {
    let buf: *mut core::ffi::c_void;
    let pages: i32;

    flags >>= flags_shift;
    flags &= OMAP_CRYPTO_COPY_MASK;

    if flags == 0 {
        return;
    }

    buf = sg_virt(sg);
    pages = get_order(len);

    if !orig.is_null() && flags & OMAP_CRYPTO_DATA_COPIED != 0 {
        omap_crypto_copy_data(sg, orig, offset, len);
    }

    if flags & OMAP_CRYPTO_DATA_COPIED != 0 {
        free_pages(buf as libc::c_ulong, pages);
    } else if flags & OMAP_CRYPTO_SG_COPIED != 0 {
        kfree(sg);
    }
}

// MODULE_DESCRIPTION("OMAP crypto support library.");
// MODULE_LICENSE("GPL v2");
// MODULE_AUTHOR("Tero Kristo <t-kristo@ti.com>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
