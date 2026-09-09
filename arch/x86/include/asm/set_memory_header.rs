/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// asm/page.h, asm-generic/set_memory.h, and asm/pgtable.h.

// #define set_memory_rox set_memory_rox

/*
 * The set_memory_* API can be used to change various attributes of a virtual
 * address range. The attributes include:
 * Cacheability  : UnCached, WriteCombining, WriteThrough, WriteBack
 * Executability : eXecutable, NoteXecutable
 * Read/Write    : ReadOnly, ReadWrite
 * Presence      : NotPresent
 * Encryption    : Encrypted, Decrypted
 *
 * Within a category, the attributes are mutually exclusive.
 *
 * The implementation of this API will take care of various aspects that
 * are associated with changing such attributes, such as:
 * - Flushing TLBs
 * - Flushing CPU caches
 * - Making sure aliases of the memory behind the mapping don't violate
 *   coherency rules as defined by the CPU in the system.
 *
 * What this API does not do:
 * - Provide exclusion between various callers - including callers that
 *   operation on other mappings of the same physical page
 * - Restore default attributes when a page is freed
 * - Guarantee that mappings other than the requested one are
 *   in any state, other than that these do not violate rules for
 *   the CPU you have. Do not depend on any effects on other mappings,
 *   CPUs other than the one you have may have more relaxed rules.
 * The caller is required to take care of these.
 */

// Opaque declaration supplied by the page subsystem.
#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

extern "C" {
    pub fn set_memory_rox(addr: usize, numpages: i32) -> i32;

    pub fn _set_memory_uc(addr: usize, numpages: i32) -> i32;
    pub fn _set_memory_wc(addr: usize, numpages: i32) -> i32;
    pub fn _set_memory_wt(addr: usize, numpages: i32) -> i32;
    pub fn _set_memory_wb(addr: usize, numpages: i32) -> i32;
    pub fn set_memory_uc(addr: usize, numpages: i32) -> i32;
    pub fn set_memory_wc(addr: usize, numpages: i32) -> i32;
    pub fn set_memory_wb(addr: usize, numpages: i32) -> i32;
    pub fn set_memory_np(addr: usize, numpages: i32) -> i32;
    pub fn set_memory_p(addr: usize, numpages: i32) -> i32;
    pub fn set_memory_4k(addr: usize, numpages: i32) -> i32;

    pub fn set_memory_enc_stop_conversion() -> bool;
    pub fn set_memory_encrypted(addr: usize, numpages: i32) -> i32;
    pub fn set_memory_decrypted(addr: usize, numpages: i32) -> i32;

    pub fn set_memory_np_noalias(addr: usize, numpages: i32) -> i32;
    pub fn set_memory_nonglobal(addr: usize, numpages: i32) -> i32;
    pub fn set_memory_global(addr: usize, numpages: i32) -> i32;

    pub fn set_pages_array_uc(pages: *mut *mut page, addrinarray: i32) -> i32;
    pub fn set_pages_array_wc(pages: *mut *mut page, addrinarray: i32) -> i32;
    pub fn set_pages_array_wb(pages: *mut *mut page, addrinarray: i32) -> i32;

    /*
     * For legacy compatibility with the old APIs, a few functions
     * are provided that work on a "struct page".
     * These functions operate ONLY on the 1:1 kernel mapping of the
     * memory that the struct page represents, and internally just
     * call the set_memory_* function. See the description of the
     * set_memory_* function for more details on conventions.
     *
     * These APIs should be considered *deprecated* and are likely going to
     * be removed in the future.
     * The reason for this is the implicit operation on the 1:1 mapping only,
     * making this not a generally useful API.
     *
     * Specifically, many users of the old APIs had a virtual address,
     * called virt_to_page() or vmalloc_to_page() on that address to
     * get a struct page* that the old API required.
     * To convert these cases, use set_memory_*() on the original
     * virtual address, do not use these functions.
     */
    pub fn set_pages_uc(page: *mut page, numpages: i32) -> i32;
    pub fn set_pages_wb(page: *mut page, numpages: i32) -> i32;
    pub fn set_pages_ro(page: *mut page, numpages: i32) -> i32;
    pub fn set_pages_rw(page: *mut page, numpages: i32) -> i32;

    pub fn set_direct_map_invalid_noflush(page: *mut page) -> i32;
    pub fn set_direct_map_default_noflush(page: *mut page) -> i32;
    pub fn set_direct_map_valid_noflush(page: *mut page, nr: u32, valid: bool) -> i32;
    pub fn kernel_page_present(page: *mut page) -> bool;

    pub static mut kernel_set_to_readonly: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
