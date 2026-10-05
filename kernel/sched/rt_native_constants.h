/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_RT_NATIVE_CONSTANTS_H
#define LUPOS_RT_NATIVE_CONSTANTS_H
/* Values are evaluated by the configured native headers, never copied numbers. */
static const int RUST_RT_RR_TIMESLICE = RR_TIMESLICE;
static const u64 RUST_RT_MAX_BW = MAX_BW;
static const int RUST_RT_MAX_RT_PRIO = MAX_RT_PRIO;
static const int RUST_RT_CPUPRI_INVALID = CPUPRI_INVALID;
static const unsigned int RUST_RT_DEQUEUE_SAVE = DEQUEUE_SAVE;
static const unsigned int RUST_RT_DEQUEUE_MOVE = DEQUEUE_MOVE;
static const unsigned int RUST_RT_DEQUEUE_SLEEP = DEQUEUE_SLEEP;
static const unsigned int RUST_RT_ENQUEUE_HEAD = ENQUEUE_HEAD;
static const unsigned int RUST_RT_ENQUEUE_WAKEUP = ENQUEUE_WAKEUP;
static const int RUST_RT_WF_TTWU = WF_TTWU;
static const int RUST_RT_WF_FORK = WF_FORK;
static const unsigned int RUST_RT_SCHED_RR = SCHED_RR;
static const int RUST_RT_SD_WAKE_AFFINE = SD_WAKE_AFFINE;
static const unsigned int RUST_RT_TASK_INTERRUPTIBLE = TASK_INTERRUPTIBLE;
static const unsigned int RUST_RT_TASK_UNINTERRUPTIBLE = TASK_UNINTERRUPTIBLE;
static const u64 RUST_RT_RUNTIME_INF = RUNTIME_INF;
static const u64 RUST_RT_U64_MAX = U64_MAX;
static const long RUST_RT_HZ = HZ;
static const long RUST_RT_MSEC_PER_SEC = MSEC_PER_SEC;
static const long RUST_RT_USEC_PER_SEC = USEC_PER_SEC;
static const long RUST_RT_NSEC_PER_USEC = NSEC_PER_USEC;
static const int RUST_RT_CLOCK_MONOTONIC = CLOCK_MONOTONIC;
static const unsigned int RUST_RT_RLIMIT_RTTIME = RLIMIT_RTTIME;
static const unsigned long RUST_RT_RLIM_INFINITY = RLIM_INFINITY;
static const int RUST_RT_EINVAL = EINVAL;
static const int RUST_RT_EBUSY = EBUSY;
#endif
