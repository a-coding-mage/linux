/* SPDX-License-Identifier: GPL-2.0 */
/* Header guard and Linux include directives omitted in Rust translation.
 * Depends on Linux kernel symbols from <linux/kernel.h> and <linux/bug.h>.
 */

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

unsafe extern "C" {
    fn BUG_ON(condition: bool);
    fn unlikely(condition: bool) -> bool;
    fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void;
    fn page_to_phys(page: *mut page) -> dma_addr_t;
    fn virt_to_page(addr: *const c_void) -> *mut page;
    fn offset_in_page(addr: *const c_void) -> u32;
}

/* External dependency types supplied by the translated Linux environment. */
use crate::{dma_addr_t, page};

#[repr(C)]
pub struct scatterlist {
    pub page_link: libc::c_ulong,
    pub offset: u32,
    pub length: u32,
    pub dma_address: dma_addr_t,
}

/* Scatterlist helpers, stolen from linux/scatterlist.h */
#[inline]
pub unsafe fn sg_is_chain(sg: *mut scatterlist) -> libc::c_ulong {
    unsafe { (*sg).page_link & 0x01 }
}

#[inline]
pub unsafe fn sg_is_last(sg: *mut scatterlist) -> libc::c_ulong {
    unsafe { (*sg).page_link & 0x02 }
}

#[inline]
pub unsafe fn sg_chain_ptr(sg: *mut scatterlist) -> *mut scatterlist {
    unsafe { ((*sg).page_link & !0x03) as *mut scatterlist }
}

/**
 * sg_assign_page - Assign a given page to an SG entry
 * @sg:             SG entry
 * @page:           The page
 *
 * Description:
 *   Assign page to sg entry. Also see sg_set_page(), the most commonly used
 *   variant.
 *
 **/
#[inline]
pub unsafe fn sg_assign_page(sg: *mut scatterlist, page: *mut page) {
    let page_link: libc::c_ulong = unsafe { (*sg).page_link & 0x3 };

    /*
     * In order for the low bit stealing approach to work, pages
     * must be aligned at a 32-bit boundary as a minimum.
     */
    unsafe { BUG_ON(((page as libc::c_ulong) & 0x03) != 0) };
    /* CONFIG_DEBUG_SG: BUG_ON(sg_is_chain(sg)); */
    unsafe {
        (*sg).page_link = page_link | page as libc::c_ulong;
    }
}

/**
 * sg_set_page - Set sg entry to point at given page
 * @sg:          SG entry
 * @page:        The page
 * @len:         Length of data
 * @offset:      Offset into page
 *
 * Description:
 *   Use this function to set an sg entry pointing at a page, never assign
 *   the page directly. We encode sg table information in the lower bits
 *   of the page pointer. See sg_page() for looking up the page belonging
 *   to an sg entry.
 *
 **/
#[inline]
pub unsafe fn sg_set_page(
    sg: *mut scatterlist,
    page: *mut page,
    len: u32,
    offset: u32,
) {
    unsafe {
        sg_assign_page(sg, page);
        (*sg).offset = offset;
        (*sg).length = len;
    }
}

#[inline]
pub unsafe fn sg_page(sg: *mut scatterlist) -> *mut page {
    /* CONFIG_DEBUG_SG: BUG_ON(sg_is_chain(sg)); */
    unsafe { ((*sg).page_link & !0x3) as *mut page }
}

/*
 * Loop over each sg element, following the pointer to a new list if necessary
 *
 * C macro:
 * for (__i = 0, sg = (sglist); __i < (nr); __i++, sg = sg_next(sg))
 */
#[macro_export]
macro_rules! for_each_sg {
    ($sglist:expr, $sg:ident, $nr:expr, $__i:ident, $body:block) => {{
        $__i = 0;
        $sg = $sglist;
        while $__i < $nr {
            $body
            $__i += 1;
            $sg = sg_next($sg);
        }
    }};
}

/**
 * sg_chain - Chain two sglists together
 * @prv:         First scatterlist
 * @prv_nents:   Number of entries in prv
 * @sgl:         Second scatterlist
 *
 * Description:
 *   Links @prv@ and @sgl@ together, to form a longer scatterlist.
 *
 **/
#[inline]
pub unsafe fn sg_chain(prv: *mut scatterlist, prv_nents: u32, sgl: *mut scatterlist) {
    /*
     * offset and length are unused for chain entry.  Clear them.
     */
    unsafe {
        (*prv.add((prv_nents - 1) as usize)).offset = 0;
        (*prv.add((prv_nents - 1) as usize)).length = 0;

        /*
         * Set lowest bit to indicate a link pointer, and make sure to clear
         * the termination bit if it happens to be set.
         */
        (*prv.add((prv_nents - 1) as usize)).page_link =
            ((sgl as libc::c_ulong) | 0x01) & !0x02;
    }
}

/**
 * sg_mark_end - Mark the end of the scatterlist
 * @sg:          SG entryScatterlist
 *
 * Description:
 *   Marks the passed in sg entry as the termination point for the sg
 *   table. A call to sg_next() on this entry will return NULL.
 *
 **/
#[inline]
pub unsafe fn sg_mark_end(sg: *mut scatterlist) {
    /*
     * Set termination bit, clear potential chain bit
     */
    unsafe {
        (*sg).page_link |= 0x02;
        (*sg).page_link &= !0x01;
    }
}

/**
 * sg_unmark_end - Undo setting the end of the scatterlist
 * @sg:          SG entryScatterlist
 *
 * Description:
 *   Removes the termination marker from the given entry of the scatterlist.
 *
 **/
#[inline]
pub unsafe fn sg_unmark_end(sg: *mut scatterlist) {
    unsafe {
        (*sg).page_link &= !0x02;
    }
}

#[inline]
pub unsafe fn sg_next(mut sg: *mut scatterlist) -> *mut scatterlist {
    unsafe {
        if sg_is_last(sg) != 0 {
            return ptr::null_mut();
        }

        sg = sg.add(1);
        if unlikely(sg_is_chain(sg) != 0) {
            sg = sg_chain_ptr(sg);
        }

        sg
    }
}

#[inline]
pub unsafe fn sg_init_table(sgl: *mut scatterlist, nents: u32) {
    unsafe {
        memset(
            sgl as *mut c_void,
            0,
            size_of::<scatterlist>().wrapping_mul(nents as usize),
        );
        sg_mark_end(sgl.add((nents - 1) as usize));
    }
}

#[inline]
pub unsafe fn sg_phys(sg: *mut scatterlist) -> dma_addr_t {
    unsafe { page_to_phys(sg_page(sg)) + (*sg).offset as dma_addr_t }
}

#[inline]
pub unsafe fn sg_set_buf(sg: *mut scatterlist, buf: *const c_void, buflen: u32) {
    unsafe {
        sg_set_page(sg, virt_to_page(buf), buflen, offset_in_page(buf));
    }
}

#[inline]
pub unsafe fn sg_init_one(sg: *mut scatterlist, buf: *const c_void, buflen: u32) {
    unsafe {
        sg_init_table(sg, 1);
        sg_set_buf(sg, buf, buflen);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
