// SPDX-License-Identifier: GPL-2.0-only
/*
 * TCP Veno congestion control
 *
 * This is based on the congestion detection/avoidance scheme described in
 *    C. P. Fu, S. C. Liew.
 *    "TCP Veno: TCP Enhancement for Transmission over Wireless Access Networks."
 *    IEEE Journal on Selected Areas in Communication,
 *    Feb. 2003.
 * 	See https://www.ie.cuhk.edu.hk/fileadmin/staff_upload/soung/Journal/J3.pdf
 */

/* Linux kernel dependencies are supplied by the surrounding translation unit. */

/* Default values of the Veno variables, in fixed-point representation
 * with V_PARAM_SHIFT bits to the right of the binary point.
 */
const V_PARAM_SHIFT: u32 = 1;
const beta: i32 = 3 << V_PARAM_SHIFT;

/* Veno variables */
#[repr(C)]
struct veno {
	 doing_veno_now: u8, /* if true, do veno for this rtt */
	 cntrtt: u16,       /* # of rtts measured within last rtt */
	 minrtt: u32,       /* min of rtts measured within last rtt (in usec) */
	 basertt: u32,      /* the min of all Veno rtt measurements seen (in usec) */
	 inc: u32,          /* decide whether to increase cwnd */
	 diff: u32,         /* calculate the diff rate */
}

/* There are several situations when we must "re-start" Veno:
 *
 *  o when a connection is established
 *  o after an RTO
 *  o after fast recovery
 *  o when we send a packet and there is no outstanding
 *    unacknowledged data (restarting an idle connection)
 */
#[inline]
unsafe fn veno_enable(sk: *mut sock) {
	let veno = inet_csk_ca(sk);

	/* turn on Veno */
	(*veno).doing_veno_now = 1;

	(*veno).minrtt = 0x7fffffff;
}

#[inline]
unsafe fn veno_disable(sk: *mut sock) {
	let veno = inet_csk_ca(sk);

	/* turn off Veno */
	(*veno).doing_veno_now = 0;
}

unsafe fn tcp_veno_init(sk: *mut sock) {
	let veno = inet_csk_ca(sk);

	(*veno).basertt = 0x7fffffff;
	(*veno).inc = 1;
	veno_enable(sk);
}

/* Do rtt sampling needed for Veno. */
unsafe fn tcp_veno_pkts_acked(sk: *mut sock, sample: *const ack_sample) {
	let veno = inet_csk_ca(sk);
	let mut vrtt: u32;

	if (*sample).rtt_us < 0 {
		return;
	}

	/* Never allow zero rtt or baseRTT */
	vrtt = ((*sample).rtt_us as u32).wrapping_add(1);

	/* Filter to find propagation delay: */
	if vrtt < (*veno).basertt {
		(*veno).basertt = vrtt;
	}

	/* Find the min rtt during the last rtt to find
	 * the current prop. delay + queuing delay:
	 */
	(*veno).minrtt = core::cmp::min((*veno).minrtt, vrtt);
	(*veno).cntrtt = (*veno).cntrtt.wrapping_add(1);
}

unsafe fn tcp_veno_state(sk: *mut sock, ca_state: u8) {
	if ca_state == TCP_CA_Open {
		veno_enable(sk);
	} else {
		veno_disable(sk);
	}
}

/*
 * If the connection is idle and we are restarting,
 * then we don't want to do any Veno calculations
 * until we get fresh rtt samples.  So when we
 * restart, we reset our Veno state to a clean
 * state. After we get acks for this flight of
 * packets, _then_ we can make Veno calculations
 * again.
 */
unsafe fn tcp_veno_cwnd_event(sk: *mut sock, event: tcp_ca_event) {
	if event == CA_EVENT_CWND_RESTART {
		tcp_veno_init(sk);
	}
}

unsafe fn tcp_veno_cwnd_event_tx_start(sk: *mut sock) {
	tcp_veno_init(sk);
}

unsafe fn tcp_veno_cong_avoid(sk: *mut sock, ack: u32, mut acked: u32) {
	let tp = tcp_sk(sk);
	let veno = inet_csk_ca(sk);

	if (*veno).doing_veno_now == 0 {
		tcp_reno_cong_avoid(sk, ack, acked);
		return;
	}

	/* limited by applications */
	if !tcp_is_cwnd_limited(sk) {
		return;
	}

	/* We do the Veno calculations only if we got enough rtt samples */
	if (*veno).cntrtt <= 2 {
		/* We don't have enough rtt samples to do the Veno
		 * calculation, so we'll behave like Reno.
		 */
		tcp_reno_cong_avoid(sk, ack, acked);
	} else {
		let mut target_cwnd: u64;
		let rtt: u32;

		/* We have enough rtt samples, so, using the Veno
		 * algorithm, we determine the state of the network.
		 */
		rtt = (*veno).minrtt;

		target_cwnd = (tcp_snd_cwnd(tp) as u64) * (*veno).basertt as u64;
		target_cwnd <<= V_PARAM_SHIFT;
		target_cwnd /= rtt as u64;

		(*veno).diff = (tcp_snd_cwnd(tp) << V_PARAM_SHIFT) - target_cwnd as u32;

		if tcp_in_slow_start(tp) {
			/* Slow start. */
			acked = tcp_slow_start(tp, acked);
			if acked == 0 {
				goto done;
			}
		}

		/* Congestion avoidance. */
		if (*veno).diff < beta as u32 {
			/* In the "non-congestive state", increase cwnd
			 * every rtt.
			 */
			tcp_cong_avoid_ai(tp, tcp_snd_cwnd(tp), acked);
		} else {
			/* In the "congestive state", increase cwnd
			 * every other rtt.
			 */
			if (*tp).snd_cwnd_cnt >= tcp_snd_cwnd(tp) {
				if (*veno).inc != 0 && tcp_snd_cwnd(tp) < (*tp).snd_cwnd_clamp {
					tcp_snd_cwnd_set(tp, tcp_snd_cwnd(tp) + 1);
					(*veno).inc = 0;
				} else {
					(*veno).inc = 1;
				}
				(*tp).snd_cwnd_cnt = 0;
			} else {
				(*tp).snd_cwnd_cnt += acked;
			}
		}
done:
		if tcp_snd_cwnd(tp) < 2 {
			tcp_snd_cwnd_set(tp, 2);
		} else if tcp_snd_cwnd(tp) > (*tp).snd_cwnd_clamp {
			tcp_snd_cwnd_set(tp, (*tp).snd_cwnd_clamp);
		}
	}
	/* Wipe the slate clean for the next rtt. */
	/* veno->cntrtt = 0; */
	(*veno).minrtt = 0x7fffffff;
}

/* Veno MD phase */
unsafe fn tcp_veno_ssthresh(sk: *mut sock) -> u32 {
	let tp = tcp_sk(sk);
	let veno = inet_csk_ca(sk);

	if (*veno).diff < beta as u32 {
		/* in "non-congestive state", cut cwnd by 1/5 */
		core::cmp::max(tcp_snd_cwnd(tp) * 4 / 5, 2_u32)
	} else {
		/* in "congestive state", cut cwnd by 1/2 */
		core::cmp::max(tcp_snd_cwnd(tp) >> 1_u32, 2_u32)
	}
}

static mut tcp_veno: tcp_congestion_ops = tcp_congestion_ops {
	.init = Some(tcp_veno_init),
	.ssthresh = Some(tcp_veno_ssthresh),
	.undo_cwnd = Some(tcp_reno_undo_cwnd),
	.cong_avoid = Some(tcp_veno_cong_avoid),
	.pkts_acked = Some(tcp_veno_pkts_acked),
	.set_state = Some(tcp_veno_state),
	.cwnd_event = Some(tcp_veno_cwnd_event),
	.cwnd_event_tx_start = Some(tcp_veno_cwnd_event_tx_start),
	.owner = THIS_MODULE,
	.name = "veno",
};

unsafe fn tcp_veno_register() -> i32 {
	/* BUILD_BUG_ON(sizeof(struct veno) > ICSK_CA_PRIV_SIZE); */
	tcp_register_congestion_control(&mut tcp_veno);
	0
}

unsafe fn tcp_veno_unregister() {
	tcp_unregister_congestion_control(&mut tcp_veno);
}

/* module_init(tcp_veno_register); */
/* module_exit(tcp_veno_unregister); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
