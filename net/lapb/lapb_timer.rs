// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	LAPB release 002
 *
 *	This code REQUIRES 2.1.15 or higher/ NET3.038
 *
 *	History
 *	LAPB 001	Jonathan Naylor	Started Coding
 *	LAPB 002	Jonathan Naylor	New timer architecture.
 */

// Kernel headers and symbols are supplied by the surrounding translation unit.

unsafe fn lapb_t1timer_expiry(t: *mut timer_list);
unsafe fn lapb_t2timer_expiry(t: *mut timer_list);

pub unsafe fn lapb_start_t1timer(lapb: *mut lapb_cb) {
	unsafe {
		timer_delete(&mut (*lapb).t1timer);

		(*lapb).t1timer.function = Some(lapb_t1timer_expiry);
		(*lapb).t1timer.expires = jiffies + (*lapb).t1;

		(*lapb).t1timer_running = true;
		add_timer(&mut (*lapb).t1timer);
	}
}

pub unsafe fn lapb_start_t2timer(lapb: *mut lapb_cb) {
	unsafe {
		timer_delete(&mut (*lapb).t2timer);

		(*lapb).t2timer.function = Some(lapb_t2timer_expiry);
		(*lapb).t2timer.expires = jiffies + (*lapb).t2;

		(*lapb).t2timer_running = true;
		add_timer(&mut (*lapb).t2timer);
	}
}

pub unsafe fn lapb_stop_t1timer(lapb: *mut lapb_cb) {
	unsafe {
		(*lapb).t1timer_running = false;
		timer_delete(&mut (*lapb).t1timer);
	}
}

pub unsafe fn lapb_stop_t2timer(lapb: *mut lapb_cb) {
	unsafe {
		(*lapb).t2timer_running = false;
		timer_delete(&mut (*lapb).t2timer);
	}
}

pub unsafe fn lapb_t1timer_running(lapb: *mut lapb_cb) -> i32 {
	unsafe { (*lapb).t1timer_running as i32 }
}

unsafe fn lapb_t2timer_expiry(t: *mut timer_list) {
	unsafe {
		let lapb = timer_container_of!(t, lapb_cb, t2timer);

		spin_lock_bh(&mut (*lapb).lock);
		if timer_pending(&(*lapb).t2timer) {
			goto_out!();
		}
		if !(*lapb).t2timer_running {
			goto_out!();
		}

		if (*lapb).condition & LAPB_ACK_PENDING_CONDITION != 0 {
			(*lapb).condition &= !LAPB_ACK_PENDING_CONDITION;
			lapb_timeout_response(lapb);
		}
		(*lapb).t2timer_running = false;

		goto_out! {
			spin_unlock_bh(&mut (*lapb).lock);
		}
	}
}

unsafe fn lapb_t1timer_expiry(t: *mut timer_list) {
	unsafe {
		let lapb = timer_container_of!(t, lapb_cb, t1timer);

		spin_lock_bh(&mut (*lapb).lock);
		if timer_pending(&(*lapb).t1timer) {
			goto_out!();
		}
		if !(*lapb).t1timer_running {
			goto_out!();
		}

		match (*lapb).state {
			LAPB_STATE_0 => {
				if (*lapb).mode & LAPB_DCE != 0 && (*lapb).n2count != (*lapb).n2 {
					(*lapb).n2count += 1;
					lapb_send_control(lapb, LAPB_DM, LAPB_POLLOFF, LAPB_RESPONSE);
				} else {
					(*lapb).state = LAPB_STATE_1;
					lapb_establish_data_link(lapb);
				}
			}
			LAPB_STATE_1 => {
				if (*lapb).n2count == (*lapb).n2 {
					lapb_clear_queues(lapb);
					(*lapb).state = LAPB_STATE_0;
					lapb_disconnect_indication(lapb, LAPB_TIMEDOUT);
					lapb_dbg(0, "(%p) S1 -> S0\n", (*lapb).dev);
					(*lapb).t1timer_running = false;
					goto_out!();
				} else {
					(*lapb).n2count += 1;
					if (*lapb).mode & LAPB_EXTENDED != 0 {
						lapb_dbg(1, "(%p) S1 TX SABME(1)\n", (*lapb).dev);
						lapb_send_control(lapb, LAPB_SABME, LAPB_POLLON, LAPB_COMMAND);
					} else {
						lapb_dbg(1, "(%p) S1 TX SABM(1)\n", (*lapb).dev);
						lapb_send_control(lapb, LAPB_SABM, LAPB_POLLON, LAPB_COMMAND);
					}
				}
			}
			LAPB_STATE_2 => {
				if (*lapb).n2count == (*lapb).n2 {
					lapb_clear_queues(lapb); (*lapb).state = LAPB_STATE_0;
					lapb_disconnect_confirmation(lapb, LAPB_TIMEDOUT);
					lapb_dbg(0, "(%p) S2 -> S0\n", (*lapb).dev);
					(*lapb).t1timer_running = false; goto_out!();
				} else { (*lapb).n2count += 1; lapb_dbg(1, "(%p) S2 TX DISC(1)\n", (*lapb).dev); lapb_send_control(lapb, LAPB_DISC, LAPB_POLLON, LAPB_COMMAND); }
			}
			LAPB_STATE_3 => {
				if (*lapb).n2count == (*lapb).n2 {
					lapb_clear_queues(lapb); (*lapb).state = LAPB_STATE_0; lapb_stop_t2timer(lapb);
					lapb_disconnect_indication(lapb, LAPB_TIMEDOUT); lapb_dbg(0, "(%p) S3 -> S0\n", (*lapb).dev);
					(*lapb).t1timer_running = false; goto_out!();
				} else { (*lapb).n2count += 1; lapb_requeue_frames(lapb); lapb_kick(lapb); }
			}
			LAPB_STATE_4 => {
				if (*lapb).n2count == (*lapb).n2 {
					lapb_clear_queues(lapb); (*lapb).state = LAPB_STATE_0;
					lapb_disconnect_indication(lapb, LAPB_TIMEDOUT); lapb_dbg(0, "(%p) S4 -> S0\n", (*lapb).dev);
					(*lapb).t1timer_running = false; goto_out!();
				} else { (*lapb).n2count += 1; lapb_transmit_frmr(lapb); }
			}
			_ => {}
		}

		lapb_start_t1timer(lapb);

		goto_out! { spin_unlock_bh(&mut (*lapb).lock); }
	}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
