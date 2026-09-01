/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

// Dependencies from <sys/socket.h> and <bpf/bpf_helpers.h>:
// SOL_SOCKET, SO_PRIORITY, bpf_getsockopt, and bpf_setsockopt.
unsafe extern "C" {
    fn bpf_getsockopt(
        ctx: *mut c_void,
        level: i32,
        optname: i32,
        optval: *mut c_void,
        optlen: i32,
    ) -> i64;
    fn bpf_setsockopt(
        ctx: *mut c_void,
        level: i32,
        optname: i32,
        optval: *mut c_void,
        optlen: i32,
    ) -> i64;
}

unsafe extern "C" {
    static SOL_SOCKET: i32;
    static SO_PRIORITY: i32;
}

#[no_mangle]
pub unsafe extern "C" fn get_set_sk_priority(ctx: *mut c_void) -> i32 {
    let mut prio: i32 = 0;

    /* Verify that context allows calling bpf_getsockopt and
     * bpf_setsockopt by reading and writing back socket
     * priority.
     */

    if unsafe {
        bpf_getsockopt(
            ctx,
            SOL_SOCKET,
            SO_PRIORITY,
            (&mut prio as *mut i32).cast::<c_void>(),
            core::mem::size_of_val(&prio) as i32,
        )
    } != 0
    {
        return 0;
    }
    if unsafe {
        bpf_setsockopt(
            ctx,
            SOL_SOCKET,
            SO_PRIORITY,
            (&mut prio as *mut i32).cast::<c_void>(),
            core::mem::size_of_val(&prio) as i32,
        )
    } != 0
    {
        return 0;
    }

    1
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
