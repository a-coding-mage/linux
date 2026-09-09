/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * Authors: Waiman Long <longman@redhat.com>
 */

// The original header is an X-macro event list.  This enum is the direct
// Rust representation of the generated event identifiers.
#[repr(usize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LockEvent {
    // Locking events for PV qspinlock.
    // CONFIG_QUEUED_SPINLOCKS and CONFIG_PARAVIRT_SPINLOCKS conditional.
    PvHashHops,
    PvKickUnlock,
    PvKickWake,
    PvLatencyKick,
    PvLatencyWake,
    PvLockStealing,
    PvSpuriousWakeup,
    PvWaitAgain,
    PvWaitEarly,
    PvWaitHead,
    PvWaitNode,

    // Locking events for qspinlock.
    // Subtracting lock_use_node[234] from lock_slowpath will give you
    // lock_use_node1.
    LockPending,
    LockSlowpath,
    LockUseNode2,
    LockUseNode3,
    LockUseNode4,
    LockNoNode,

    // Locking events for Resilient Queued Spin Lock.
    RqspinlockLockTimeout,

    // Locking events for rwsem.
    RwsemSleepReader,
    RwsemSleepWriter,
    RwsemWakeReader,
    RwsemWakeWriter,
    RwsemOptLock,
    RwsemOptFail,
    RwsemOptNospin,
    RwsemRlock,
    RwsemRlockSteal,
    RwsemRlockFast,
    RwsemRlockFail,
    RwsemRlockHandoff,
    RwsemWlock,
    RwsemWlockFail,
    RwsemWlockHandoff,

    // Locking events for rtlock_slowlock().
    RtlockSlowlock,
    RtlockSlowAcq1,
    RtlockSlowAcq2,
    RtlockSlowSleep,
    RtlockSlowWake,

    // Locking events for rt_mutex_slowlock().
    RtmutexSlowlock,
    RtmutexSlowBlock,
    RtmutexSlowAcq1,
    RtmutexSlowAcq2,
    RtmutexSlowAcq3,
    RtmutexSlowSleep,
    RtmutexSlowWake,
    RtmutexDeadlock,

    // Locking events for lockdep.
    LockdepAcquire,
    LockdepLock,
    LockdepNocheck,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
