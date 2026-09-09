// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2008 Red Hat, Inc., Eric Paris <eparis@redhat.com>
 */

/*
 * Basic idea behind the notification queue: An fsnotify group (like inotify)
 * sends the userspace notification about events asynchronously some time after
 * the event happened.  When inotify gets an event it will need to add that
 * event to the group notify queue.  Since a single event might need to be on
 * multiple group's notification queues we can't add the event directly to each
 * queue and instead add a small "event_holder" to each queue.  This event_holder
 * has a pointer back to the original event.  Since the majority of events are
 * going to end up on one, and only one, notification queue we embed one
 * event_holder into each event.  This means we have a single allocation instead
 * of always needing two.  If the embedded event_holder is already in use by
 * another group a new event_holder (from fsnotify_event_holder_cachep) will be
 * allocated and used.
 */

// Linux kernel headers and "fsnotify.h" provide the types and functions used below.

static mut fsnotify_sync_cookie: atomic_t = ATOMIC_INIT(0);

/**
 * fsnotify_get_cookie - return a unique cookie for use in synchronizing events.
 * Called from fsnotify_move, which is inlined into filesystem modules.
 */
pub unsafe fn fsnotify_get_cookie() -> u32 {

	atomic_inc_return(&raw mut fsnotify_sync_cookie)
}

pub unsafe fn fsnotify_destroy_event(
	group: *mut fsnotify_group,
	event: *mut fsnotify_event,
) {
	/* Overflow events are per-group and we don't want to free them */
	if event.is_null() || event == (*group).overflow_event {
		return;
	}
	/*
	 * If the event is still queued, we have a problem... Do an unreliable
	 * lockless check first to avoid locking in the common case. The
	 * locking may be necessary for permission events which got removed
	 * from the list by a different CPU than the one freeing the event.
	 */
	if !list_empty(&raw mut (*event).list) {
		spin_lock(&raw mut (*group).notification_lock);
		WARN_ON(!list_empty(&raw mut (*event).list));
		spin_unlock(&raw mut (*group).notification_lock);
	}
	((*(*group).ops).free_event)(group, event);
}

/*
 * Try to add an event to the notification queue.
 * The group can later pull this event off the queue to deal with.
 * The group can use the @merge hook to merge the event with a queued event.
 * The group can use the @insert hook to insert the event into hash table.
 * The function returns:
 * 0 if the event was added to a queue
 * 1 if the event was merged with some other queued event
 * 2 if the event was not queued - either the queue of events has overflown
 *   or the group is shutting down.
 */
pub unsafe fn fsnotify_insert_event(
	group: *mut fsnotify_group,
	event: *mut fsnotify_event,
	merge: Option<unsafe extern "C" fn(*mut fsnotify_group, *mut fsnotify_event) -> i32>,
	insert: Option<unsafe extern "C" fn(*mut fsnotify_group, *mut fsnotify_event)>,
) -> i32 {
	let mut ret: i32 = 0;
	let list: *mut list_head = &raw mut (*group).notification_list;

	pr_debug!("%s: group=%p event=%p\\n", __func__, group, event);

	spin_lock(&raw mut (*group).notification_lock);

	if (*group).shutdown {
		spin_unlock(&raw mut (*group).notification_lock);
		return 2;
	}

	let queue_overflow = event == (*group).overflow_event || (*group).q_len >= (*group).max_events;
	if queue_overflow {
		ret = 2;
		/* Queue overflow event only if it isn't already queued */
		if !list_empty(&raw mut (*(*group).overflow_event).list) {
			spin_unlock(&raw mut (*group).notification_lock);
			return ret;
		}
		event = (*group).overflow_event;
	}

	if !queue_overflow && !list_empty(list) {
		if let Some(merge_fn) = merge {
			ret = merge_fn(group, event);
			if ret != 0 {
				spin_unlock(&raw mut (*group).notification_lock);
				return ret;
			}
		}
	}

	(*group).q_len += 1;
	list_add_tail(&raw mut (*event).list, list);
	if let Some(insert_fn) = insert {
		insert_fn(group, event);
	}
	spin_unlock(&raw mut (*group).notification_lock);

	wake_up(&raw mut (*group).notification_waitq);
	kill_fasync(&raw mut (*group).fsn_fa, SIGIO, POLL_IN);
	ret
}

pub unsafe fn fsnotify_remove_queued_event(
	group: *mut fsnotify_group,
	event: *mut fsnotify_event,
) {
	assert_spin_locked(&raw mut (*group).notification_lock);
	/*
	 * We need to init list head for the case of overflow event so that
	 * check in fsnotify_add_event() works
	 */
	list_del_init(&raw mut (*event).list);
	(*group).q_len -= 1;
}

/*
 * Return the first event on the notification list without removing it.
 * Returns NULL if the list is empty.
 */
pub unsafe fn fsnotify_peek_first_event(
	group: *mut fsnotify_group,
) -> *mut fsnotify_event {
	assert_spin_locked(&raw mut (*group).notification_lock);

	if fsnotify_notify_queue_is_empty(group) {
		return core::ptr::null_mut();
	}

	list_first_entry(&(*group).notification_list, fsnotify_event, list)
}

/*
 * Remove and return the first event from the notification list.  It is the
 * responsibility of the caller to destroy the obtained event
 */
pub unsafe fn fsnotify_remove_first_event(
	group: *mut fsnotify_group,
) -> *mut fsnotify_event {
	let event: *mut fsnotify_event = fsnotify_peek_first_event(group);

	if event.is_null() {
		return core::ptr::null_mut();
	}

	pr_debug!("%s: group=%p event=%p\\n", __func__, group, event);

	fsnotify_remove_queued_event(group, event);

	event
}

/*
 * Called when a group is being torn down to clean up any outstanding
 * event notifications.
 */
pub unsafe fn fsnotify_flush_notify(group: *mut fsnotify_group) {
	let mut event: *mut fsnotify_event;

	spin_lock(&raw mut (*group).notification_lock);
	while !fsnotify_notify_queue_is_empty(group) {
		event = fsnotify_remove_first_event(group);
		spin_unlock(&raw mut (*group).notification_lock);
		fsnotify_destroy_event(group, event);
		spin_lock(&raw mut (*group).notification_lock);
	}
	spin_unlock(&raw mut (*group).notification_lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
