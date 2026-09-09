/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
struct InvpcidDescriptor {
    d: [u64; 2],
}

/// Invalidate TLB entries according to the supplied PCID, address, and type.
#[inline]
unsafe fn __invpcid(pcid: usize, addr: usize, ty: usize) {
    let desc = InvpcidDescriptor {
        d: [pcid as u64, addr as u64],
    };

    /*
     * The memory clobber is because the whole point is to invalidate
     * stale TLB entries and, especially if we're flushing global
     * mappings, we don't want the compiler to reorder any subsequent
     * memory accesses before the TLB flush.
     */
    core::arch::asm!(
        "invpcid [{desc}], {ty}",
        desc = in(reg) &desc,
        ty = in(reg) ty,
    );
}

const INVPCID_TYPE_INDIV_ADDR: usize = 0;
const INVPCID_TYPE_SINGLE_CTXT: usize = 1;
const INVPCID_TYPE_ALL_INCL_GLOBAL: usize = 2;
const INVPCID_TYPE_ALL_NON_GLOBAL: usize = 3;

/* Flush all mappings for a given pcid and addr, not including globals. */
#[inline]
unsafe fn invpcid_flush_one(pcid: usize, addr: usize) {
    __invpcid(pcid, addr, INVPCID_TYPE_INDIV_ADDR);
}

/* Flush all mappings for a given PCID, not including globals. */
#[inline]
unsafe fn invpcid_flush_single_context(pcid: usize) {
    __invpcid(pcid, 0, INVPCID_TYPE_SINGLE_CTXT);
}

/* Flush all mappings, including globals, for all PCIDs. */
#[inline]
unsafe fn invpcid_flush_all() {
    __invpcid(0, 0, INVPCID_TYPE_ALL_INCL_GLOBAL);
}

/* Flush all mappings for all PCIDs except globals. */
#[inline]
unsafe fn invpcid_flush_all_nonglobals() {
    __invpcid(0, 0, INVPCID_TYPE_ALL_NON_GLOBAL);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
