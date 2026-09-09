/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/linux/sunrpc/types.h
 *
 * Generic types and misc stuff for RPC.
 *
 * Copyright (C) 1996, Olaf Kirch <okir@monad.swb.de>
 */

// Dependencies supplied by the corresponding Linux headers:
// <linux/timer.h>
// <linux/sched/signal.h>
// <linux/workqueue.h>
// <linux/sunrpc/debug.h>
// <linux/list.h>

/*
 * Shorthands
 */
macro_rules! signalled {
    () => {
        signal_pending(current)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
