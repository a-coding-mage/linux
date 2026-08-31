// SPDX-License-Identifier: GPL-2.0
// C dependencies: test_progs.h, network_helpers.h, preempt_lock.skel.h

pub fn test_preempt_lock() {
    RUN_TESTS!(preempt_lock);
}
