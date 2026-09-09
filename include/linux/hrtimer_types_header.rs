/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding Linux translation:
// linux/types.h and linux/timerqueue_types.h

pub struct hrtimer_clock_base;

/*
 * Return values for the callback function
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hrtimer_restart {
    HRTIMER_NORESTART, /* Timer is not restarted */
    HRTIMER_RESTART,   /* Timer must be restarted */
}

/**
 * struct hrtimer - the basic hrtimer structure
 * @node: Linked timerqueue node, which also manages node.expires,
 *        the absolute expiry time in the hrtimers internal
 *        representation. The time is related to the clock on
 *        which the timer is based. Is setup by adding
 *        slack to the _softexpires value. For non range timers
 *        identical to _softexpires.
 * @_softexpires: the absolute earliest expiry time of the hrtimer.
 *        The time which was given as expiry time when the timer
 *        was armed.
 * @function: timer expiry callback function
 * @base: pointer to the timer base (per cpu and per clock)
 * @is_queued: Indicates whether a timer is enqueued or not
 * @is_rel: Set if the timer was armed relative
 * @is_soft: Set if hrtimer will be expired in soft interrupt context.
 * @is_hard: Set if hrtimer will be expired in hard interrupt context
 *        even on RT.
 * @is_lazy: Set if the timer is frequently rearmed to avoid updates
 *        of the clock event device
 *
 * The hrtimer structure must be initialized by hrtimer_setup()
 */
#[repr(C)]
pub struct hrtimer {
    pub node: timerqueue_linked_node,
    pub base: *mut hrtimer_clock_base,
    pub is_queued: bool,
    pub is_rel: bool,
    pub is_soft: bool,
    pub is_hard: bool,
    pub is_lazy: bool,
    pub _softexpires: ktime_t,
    pub function: Option<unsafe extern "C" fn(*mut hrtimer) -> hrtimer_restart>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
