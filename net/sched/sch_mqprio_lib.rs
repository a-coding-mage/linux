// SPDX-License-Identifier: GPL-2.0-only
//
// Dependencies are supplied by the surrounding kernel translation unit.

use core::cmp::{max, min};

/* Returns true if the intervals [a, b) and [c, d) overlap. */
unsafe fn intervals_overlap(a: i32, b: i32, c: i32, d: i32) -> bool {
    let left = max(a, c);
    let right = min(b, d);

    left < right
}

unsafe fn mqprio_validate_queue_counts(
    dev: *mut net_device,
    qopt: *const tc_mqprio_qopt,
    allow_overlapping_txqs: bool,
    extack: *mut netlink_ext_ack,
) -> i32 {
    let mut i: i32 = 0;

    while i < (*qopt).num_tc as i32 {
        let last: u32 = (*qopt).offset[i as usize] + (*qopt).count[i as usize];

        if (*qopt).count[i as usize] == 0 {
            nl_set_err_msg_fmt_mod(extack, "No queues for TC %d", i);
            return -22; // -EINVAL
        }

        /* Verify the queue count is in tx range being equal to the
         * real_num_tx_queues indicates the last queue is in use.
         */
        if (*qopt).offset[i as usize] >= (*dev).real_num_tx_queues
            || last > (*dev).real_num_tx_queues
        {
            nl_set_err_msg_fmt_mod(
                extack,
                "Queues %d:%d for TC %d exceed the %d TX queues available",
                (*qopt).count[i as usize],
                (*qopt).offset[i as usize],
                i,
                (*dev).real_num_tx_queues,
            );
            return -22; // -EINVAL
        }

        if allow_overlapping_txqs {
            i += 1;
            continue;
        }

        /* Verify that the offset and counts do not overlap */
        let mut j = i + 1;
        while j < (*qopt).num_tc as i32 {
            if intervals_overlap(
                (*qopt).offset[i as usize] as i32,
                last as i32,
                (*qopt).offset[j as usize] as i32,
                ((*qopt).offset[j as usize] + (*qopt).count[j as usize]) as i32,
            ) {
                nl_set_err_msg_fmt_mod(
                    extack,
                    "TC %d queues %d@%d overlap with TC %d queues %d@%d",
                    i,
                    (*qopt).count[i as usize],
                    (*qopt).offset[i as usize],
                    j,
                    (*qopt).count[j as usize],
                    (*qopt).offset[j as usize],
                );
                return -22; // -EINVAL
            }
            j += 1;
        }
        i += 1;
    }

    0
}

pub unsafe fn mqprio_validate_qopt(
    dev: *mut net_device,
    qopt: *mut tc_mqprio_qopt,
    validate_queue_counts: bool,
    allow_overlapping_txqs: bool,
    extack: *mut netlink_ext_ack,
) -> i32 {
    /* Verify num_tc is not out of max range */
    if (*qopt).num_tc > TC_MAX_QUEUE {
        nl_set_err_msg(extack, "Number of traffic classes is outside valid range");
        return -22; // -EINVAL
    }

    /* Verify priority mapping uses valid tcs */
    let mut i = 0;
    while i <= TC_BITMASK {
        if (*qopt).prio_tc_map[i as usize] >= (*qopt).num_tc {
            nl_set_err_msg(
                extack,
                "Invalid traffic class in priority to traffic class mapping",
            );
            return -22; // -EINVAL
        }
        i += 1;
    }

    if validate_queue_counts {
        let err = mqprio_validate_queue_counts(dev, qopt, allow_overlapping_txqs, extack);
        if err != 0 {
            return err;
        }
    }

    0
}

pub unsafe fn mqprio_qopt_reconstruct(dev: *mut net_device, qopt: *mut tc_mqprio_qopt) {
    let num_tc = netdev_get_num_tc(dev);

    (*qopt).num_tc = num_tc;
    let mut tc = 0;
    while tc <= TC_BITMASK {
        (*qopt).prio_tc_map[tc as usize] = netdev_get_prio_tc_map(dev, tc);
        tc += 1;
    }

    tc = 0;
    while tc < num_tc {
        let res = (*dev).tc_to_txq[tc as usize].combined;
        (*qopt).count[tc as usize] = res.count;
        (*qopt).offset[tc as usize] = res.offset;
        tc += 1;
    }
}

pub unsafe fn mqprio_fp_to_offload(
    fp: *const u32,
    mqprio: *mut tc_mqprio_qopt_offload,
) {
    let mut preemptible_tcs: usize = 0;
    let mut tc = 0;

    while tc < TC_QOPT_MAX_QUEUE {
        if *fp.add(tc as usize) == TC_FP_PREEMPTIBLE {
            preemptible_tcs |= 1usize << tc;
        }
        tc += 1;
    }

    (*mqprio).preemptible_tcs = preemptible_tcs;
}

// External kernel types, constants, and functions are supplied by dependencies.
extern "C" {
    fn nl_set_err_msg_fmt_mod(extack: *mut netlink_ext_ack, fmt: &str, ...);
    fn nl_set_err_msg(extack: *mut netlink_ext_ack, msg: &str);
    fn netdev_get_num_tc(dev: *mut net_device) -> i32;
    fn netdev_get_prio_tc_map(dev: *mut net_device, tc: i32) -> u32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
