/* SPDX-License-Identifier: GPL-2.0-only */
/* The industrial I/O - event passing to userspace
 *
 * Copyright (c) 2008-2011 Jonathan Cameron
 */

// Dependencies supplied by the original headers:
// linux/iio/types.h
// uapi/linux/iio/events.h

/**
 * _IIO_EVENT_CODE() - create event identifier
 * @chan_type: Type of the channel. Should be one of enum iio_chan_type.
 * @diff: Whether the event is for an differential channel or not.
 * @modifier: Modifier for the channel. Should be one of enum iio_modifier.
 * @direction: Direction of the event. One of enum iio_event_direction.
 * @type: Type of the event. Should be one of enum iio_event_type.
 * @chan: Channel number for non-differential channels.
 * @chan1: First channel number for differential channels.
 * @chan2: Second channel number for differential channels.
 *
 * Drivers should use the specialized macros below instead of using this one
 * directly.
 */
#[inline]
pub const fn _iio_event_code(
    chan_type: u64,
    diff: u64,
    modifier: u64,
    direction: u64,
    event_type: u64,
    chan: u64,
    chan1: u64,
    chan2: u64,
) -> u64 {
    (event_type << 56)
        | (diff << 55)
        | (direction << 48)
        | (modifier << 40)
        | (chan_type << 32)
        | ((chan2 as u16 as u64) << 16)
        | (chan1 as u16 as u64)
        | (chan as u16 as u64)
}

/**
 * IIO_MOD_EVENT_CODE() - create event identifier for modified (non
 * differential) channels
 * @chan_type: Type of the channel. Should be one of enum iio_chan_type.
 * @number: Channel number.
 * @modifier: Modifier for the channel. Should be one of enum iio_modifier.
 * @type: Type of the event. Should be one of enum iio_event_type.
 * @direction: Direction of the event. One of enum iio_event_direction.
 */
#[inline]
pub const fn iio_mod_event_code(
    chan_type: u64,
    number: u64,
    modifier: u64,
    event_type: u64,
    direction: u64,
) -> u64 {
    _iio_event_code(chan_type, 0, modifier, direction, event_type, number, 0, 0)
}

/**
 * IIO_UNMOD_EVENT_CODE() - create event identifier for unmodified (non
 * differential) channels
 * @chan_type: Type of the channel. Should be one of enum iio_chan_type.
 * @number: Channel number.
 * @type: Type of the event. Should be one of enum iio_event_type.
 * @direction: Direction of the event. One of enum iio_event_direction.
 */
#[inline]
pub const fn iio_unmod_event_code(
    chan_type: u64,
    number: u64,
    event_type: u64,
    direction: u64,
) -> u64 {
    _iio_event_code(chan_type, 0, 0, direction, event_type, number, 0, 0)
}

/**
 * IIO_DIFF_EVENT_CODE() - create event identifier for differential channels
 * @chan_type: Type of the channel. Should be one of enum iio_chan_type.
 * @chan1: First channel number for differential channels.
 * @chan2: Second channel number for differential channels.
 * @type: Type of the event. Should be one of enum iio_event_type.
 * @direction: Direction of the event. One of enum iio_event_direction.
 */
#[inline]
pub const fn iio_diff_event_code(
    chan_type: u64,
    chan1: u64,
    chan2: u64,
    event_type: u64,
    direction: u64,
) -> u64 {
    _iio_event_code(chan_type, 1, 0, direction, event_type, 0, chan1, chan2)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
