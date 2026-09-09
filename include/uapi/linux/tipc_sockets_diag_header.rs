/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* AF_TIPC sock_diag interface for querying open sockets */

// Dependency intent: equivalent to <linux/types.h> and <linux/sock_diag.h>.

/* Request */
#[repr(C)]
pub struct tipc_sock_diag_req {
    pub sdiag_family: __u8,   /* must be AF_TIPC */
    pub sdiag_protocol: __u8, /* must be 0 */
    pub pad: __u16,           /* must be 0 */
    pub tidiag_states: __u32, /* query*/
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
