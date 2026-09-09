/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * v4l2-event.h
 *
 * V4L2 events.
 *
 * Rust translation of the C header.
 */

/* Dependencies supplied by the surrounding kernel translation. */
extern "C" {
    pub type list_head;
    pub type v4l2_event;
    pub type v4l2_event_subscription;
}

pub struct v4l2_fh;
pub struct v4l2_subdev;
pub struct video_device;

/** Internal kernel event struct. */
#[repr(C)]
pub struct v4l2_kevent {
    pub list: list_head,
    pub sev: *mut v4l2_subscribed_event,
    pub event: v4l2_event,
    pub ts: u64,
}

/** Subscribed event operations. */
#[repr(C)]
pub struct v4l2_subscribed_event_ops {
    pub add: Option<unsafe extern "C" fn(
        sev: *mut v4l2_subscribed_event,
        elems: u32,
    ) -> i32>,
    pub del: Option<unsafe extern "C" fn(sev: *mut v4l2_subscribed_event)>,
    pub replace: Option<unsafe extern "C" fn(
        old: *mut v4l2_event,
        new: *const v4l2_event,
    )>,
    pub merge: Option<unsafe extern "C" fn(
        old: *const v4l2_event,
        new: *mut v4l2_event,
    )>,
}

/** Internal struct representing a subscribed event. */
#[repr(C)]
pub struct v4l2_subscribed_event {
    pub list: list_head,
    pub type_: u32,
    pub id: u32,
    pub flags: u32,
    pub fh: *mut v4l2_fh,
    pub node: list_head,
    pub ops: *const v4l2_subscribed_event_ops,
    pub elems: u32,
    pub first: u32,
    pub in_use: u32,
    /* C flexible array member: __counted_by(elems). */
    pub events: [v4l2_kevent; 0],
}

extern "C" {
    pub fn v4l2_event_dequeue(
        fh: *mut v4l2_fh,
        event: *mut v4l2_event,
        nonblocking: i32,
    ) -> i32;

    pub fn v4l2_event_queue(vdev: *mut video_device, ev: *const v4l2_event);

    pub fn v4l2_event_queue_fh(fh: *mut v4l2_fh, ev: *const v4l2_event);

    pub fn v4l2_event_wake_all(vdev: *mut video_device);

    pub fn v4l2_event_pending(fh: *mut v4l2_fh) -> i32;

    pub fn v4l2_event_subscribe(
        fh: *mut v4l2_fh,
        sub: *const v4l2_event_subscription,
        elems: u32,
        ops: *const v4l2_subscribed_event_ops,
    ) -> i32;

    pub fn v4l2_event_unsubscribe(
        fh: *mut v4l2_fh,
        sub: *const v4l2_event_subscription,
    ) -> i32;

    pub fn v4l2_event_unsubscribe_all(fh: *mut v4l2_fh);

    pub fn v4l2_event_subdev_unsubscribe(
        sd: *mut v4l2_subdev,
        fh: *mut v4l2_fh,
        sub: *mut v4l2_event_subscription,
    ) -> i32;

    pub fn v4l2_src_change_event_subscribe(
        fh: *mut v4l2_fh,
        sub: *const v4l2_event_subscription,
    ) -> i32;

    pub fn v4l2_src_change_event_subdev_subscribe(
        sd: *mut v4l2_subdev,
        fh: *mut v4l2_fh,
        sub: *mut v4l2_event_subscription,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
