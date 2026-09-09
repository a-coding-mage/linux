/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: <linux/string.h> and <uapi/asm/schid.h>.

unsafe extern "C" {
    fn memcmp(
        lhs: *const core::ffi::c_void,
        rhs: *const core::ffi::c_void,
        count: usize,
    ) -> i32;
}

/* Helper function for sane state of pre-allocated subchannel_id. */
#[inline]
pub unsafe fn init_subchannel_id(schid: *mut subchannel_id) {
    core::ptr::write_bytes(
        schid as *mut u8,
        0,
        core::mem::size_of::<subchannel_id>(),
    );
    (*schid).one = 1;
}

#[inline]
pub unsafe fn schid_equal(
    schid1: *mut subchannel_id,
    schid2: *mut subchannel_id,
) -> i32 {
    let equal = memcmp(
        schid1 as *const libc::c_void,
        schid2 as *const libc::c_void,
        core::mem::size_of::<subchannel_id>(),
    ) == 0;
    (!equal) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
