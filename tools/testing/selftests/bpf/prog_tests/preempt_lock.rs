// SPDX-License-Identifier: GPL-2.0
// C dependencies: test_progs.h, network_helpers.h, preempt_lock.skel.h

pub fn test_preempt_lock() {
    RUN_TESTS!(preempt_lock);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
