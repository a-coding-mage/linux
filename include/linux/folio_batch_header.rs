/* SPDX-License-Identifier: GPL-2.0 */
/*
 * include/linux/folio_batch.h
 *
 * In many places it is efficient to batch an operation up against multiple
 * folios.  A folio_batch is a container which is used for that.
 */

// Dependency provided by the Linux types translation.

/* 31 pointers + header align the folio_batch structure to a power of two */
pub const FOLIO_BATCH_SIZE: usize = 31;

pub enum folio {}

/**
 * struct folio_batch - A collection of folios.
 *
 * The folio_batch is used to amortise the cost of retrieving and
 * operating on a set of folios.  The order of folios in the batch may be
 * significant (eg delete_from_page_cache_batch()).  Some users of the
 * folio_batch store "exceptional" entries in it which can be removed
 * by calling folio_batch_remove_exceptionals().
 */
#[repr(C)]
pub struct folio_batch {
    pub nr: u8,
    pub i: u8,
    pub percpu_pvec_drained: bool,
    pub folios: [*mut folio; FOLIO_BATCH_SIZE],
}

/**
 * folio_batch_init() - Initialise a batch of folios
 * @fbatch: The folio batch.
 *
 * A freshly initialised folio_batch contains zero folios.
 */
#[inline]
pub unsafe fn folio_batch_init(fbatch: *mut folio_batch) {
    (*fbatch).nr = 0;
    (*fbatch).i = 0;
    (*fbatch).percpu_pvec_drained = false;
}

#[inline]
pub unsafe fn folio_batch_reinit(fbatch: *mut folio_batch) {
    (*fbatch).nr = 0;
    (*fbatch).i = 0;
}

#[inline]
pub unsafe fn folio_batch_count(fbatch: *const folio_batch) -> u32 {
    (*fbatch).nr as u32
}

#[inline]
pub unsafe fn folio_batch_space(fbatch: *const folio_batch) -> u32 {
    FOLIO_BATCH_SIZE as u32 - (*fbatch).nr as u32
}

/**
 * folio_batch_add() - Add a folio to a batch.
 * @fbatch: The folio batch.
 * @folio: The folio to add.
 *
 * The folio is added to the end of the batch.
 * The batch must have previously been initialised using folio_batch_init().
 *
 * Return: The number of slots still available.
 */
#[inline]
pub unsafe fn folio_batch_add(fbatch: *mut folio_batch, folio: *mut folio) -> u32 {
    let nr = (*fbatch).nr;
    (*fbatch).folios[nr as usize] = folio;
    (*fbatch).nr = nr.wrapping_add(1);
    folio_batch_space(fbatch)
}

/**
 * folio_batch_next - Return the next folio to process.
 * @fbatch: The folio batch being processed.
 *
 * Use this function to implement a queue of folios.
 *
 * Return: The next folio in the queue, or NULL if the queue is empty.
 */
#[inline]
pub unsafe fn folio_batch_next(fbatch: *mut folio_batch) -> *mut folio {
    if (*fbatch).i == (*fbatch).nr {
        return core::ptr::null_mut();
    }
    let i = (*fbatch).i;
    (*fbatch).i = i.wrapping_add(1);
    (*fbatch).folios[i as usize]
}

unsafe extern "C" {
    pub fn __folio_batch_release(fbatch: *mut folio_batch);
    pub fn folio_batch_remove_exceptionals(fbatch: *mut folio_batch);
}

#[inline]
pub unsafe fn folio_batch_release(fbatch: *mut folio_batch) {
    if folio_batch_count(fbatch) != 0 {
        __folio_batch_release(fbatch);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
