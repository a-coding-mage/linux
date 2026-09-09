// SPDX-License-Identifier: GPL-2.0-only
/* identifiers for device / performance-differentiated memory regions */
// Dependencies supplied by the surrounding kernel bindings:
// linux/idr.h, linux/types.h, and linux/memregion.h.

// Equivalent to the C DEFINE_IDA(memregion_ids) declaration.  The `ida` type
// and the allocator functions are supplied by the surrounding bindings.
static mut memregion_ids: ida = unsafe { core::mem::zeroed() };

extern "C" {
    fn ida_alloc(ida: *mut ida, gfp: gfp_t) -> i32;
    fn ida_free(ida: *mut ida, id: i32);
}

#[no_mangle]
pub unsafe extern "C" fn memregion_alloc(gfp: gfp_t) -> i32 {
    ida_alloc(&raw mut memregion_ids, gfp)
}

#[no_mangle]
pub unsafe extern "C" fn memregion_free(id: i32) {
    ida_free(&raw mut memregion_ids, id);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
