/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) International Business Machines Corp., 2000-2001
 *   Portions Copyright (C) Christoph Hellwig, 2001-2002
 */

/*
 * The C header guard and Linux kernel includes are intentionally omitted from
 * executable Rust; the referenced kernel symbols are supplied externally.
 */

/*
 *	jfs_lock.h
 */

/*
 * Conditional sleep where condition is protected by spinlock
 *
 * lock_cmd and unlock_cmd take and release the spinlock
 */
#[macro_export]
macro_rules! __SLEEP_COND {
    ($wq:expr, $cond:expr, $($lock_cmd:tt)*, $($unlock_cmd:tt)*) => {{
        DECLARE_WAITQUEUE!(__wait, current);

        add_wait_queue!(&$wq, &__wait);
        loop {
            set_current_state!(TASK_UNINTERRUPTIBLE);
            if $cond {
                break;
            }
            $($unlock_cmd)*;
            io_schedule!();
            $($lock_cmd)*;
        }
        __set_current_state!(TASK_RUNNING);
        remove_wait_queue!(&$wq, &__wait);
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
