/* SPDX-License-Identifier: GPL-2.0 */

/* C dependency: <linux/wait.h> */

/* for synchronisation between af_netlink and genetlink */
extern "C" {
    pub static mut genl_sk_destructing_cnt: atomic_t;
    pub static mut genl_sk_destructing_waitq: wait_queue_head_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
