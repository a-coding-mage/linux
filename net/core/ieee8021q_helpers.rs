// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2024 Pengutronix, Oleksij Rempel <kernel@pengutronix.de>

/* Dependencies supplied by the surrounding kernel translation. */

/* verify that table covers all 8 traffic types */

/* The following arrays map Traffic Types (TT) to traffic classes (TC) for
 * different number of queues as shown in the example provided by
 * IEEE 802.1Q-2022 in Annex I "I.3 Traffic type to traffic class mapping" and
 * Table I-1 "Traffic type to traffic class mapping".
 */
static ieee8021q_8queue_tt_tc_map: [u8; IEEE8021Q_TT_MAX as usize] = [
    0, 1, 2, 3, 4, 5, 6, 7,
];

static ieee8021q_7queue_tt_tc_map: [u8; IEEE8021Q_TT_MAX as usize] = [
    0, 1, 2, 3, 4, 4, 5, 6,
];

static ieee8021q_6queue_tt_tc_map: [u8; IEEE8021Q_TT_MAX as usize] = [
    0, 1, 2, 2, 3, 3, 4, 5,
];

static ieee8021q_5queue_tt_tc_map: [u8; IEEE8021Q_TT_MAX as usize] = [
    0, 0, 1, 1, 2, 2, 3, 4,
];

static ieee8021q_4queue_tt_tc_map: [u8; IEEE8021Q_TT_MAX as usize] = [
    0, 0, 1, 1, 2, 2, 3, 3,
];

static ieee8021q_3queue_tt_tc_map: [u8; IEEE8021Q_TT_MAX as usize] = [
    0, 0, 0, 0, 1, 1, 2, 2,
];

static ieee8021q_2queue_tt_tc_map: [u8; IEEE8021Q_TT_MAX as usize] = [
    0, 0, 0, 0, 1, 1, 1, 1,
];

static ieee8021q_1queue_tt_tc_map: [u8; IEEE8021Q_TT_MAX as usize] = [
    0, 0, 0, 0, 0, 0, 0, 0,
];

/**
 * ieee8021q_tt_to_tc - Map IEEE 802.1Q Traffic Type to Traffic Class
 * @tt: IEEE 802.1Q Traffic Type
 * @num_queues: Number of queues
 *
 * This function maps an IEEE 802.1Q Traffic Type to a Traffic Class (TC) based
 * on the number of queues configured on the NIC. The mapping is based on the
 * example provided by IEEE 802.1Q-2022 in Annex I "I.3 Traffic type to traffic
 * class mapping" and Table I-1 "Traffic type to traffic class mapping".
 *
 * Return: Traffic Class corresponding to the given Traffic Type or negative
 * value in case of error.
 */
pub unsafe fn ieee8021q_tt_to_tc(
    tt: ieee8021q_traffic_type,
    num_queues: u32,
) -> i32 {
    let tt_value = tt as i32;
    if tt_value < 0 || tt_value >= IEEE8021Q_TT_MAX as i32 {
        pr_err!("Requested Traffic Type ({}) is out of range ({})\n", tt_value, IEEE8021Q_TT_MAX);
        return -EINVAL;
    }

    match num_queues {
        8 => ieee8021q_8queue_tt_tc_map[tt_value as usize] as i32,
        7 => ieee8021q_7queue_tt_tc_map[tt_value as usize] as i32,
        6 => ieee8021q_6queue_tt_tc_map[tt_value as usize] as i32,
        5 => ieee8021q_5queue_tt_tc_map[tt_value as usize] as i32,
        4 => ieee8021q_4queue_tt_tc_map[tt_value as usize] as i32,
        3 => ieee8021q_3queue_tt_tc_map[tt_value as usize] as i32,
        2 => ieee8021q_2queue_tt_tc_map[tt_value as usize] as i32,
        1 => ieee8021q_1queue_tt_tc_map[tt_value as usize] as i32,
        _ => {
            pr_err!("Invalid number of queues {}\n", num_queues);
            return -EINVAL;
        }
    }
}

/**
 * ietf_dscp_to_ieee8021q_tt - Map IETF DSCP to IEEE 802.1Q Traffic Type
 * @dscp: IETF DSCP value
 *
 * This function maps an IETF DSCP value to an IEEE 802.1Q Traffic Type (TT).
 * Since there is no corresponding mapping between DSCP and IEEE 802.1Q Traffic
 * Type, this function is inspired by the RFC8325 documentation which describe
 * the mapping between DSCP and 802.11 User Priority (UP) values.
 *
 * Return: IEEE 802.1Q Traffic Type corresponding to the given DSCP value
 */
pub fn ietf_dscp_to_ieee8021q_tt(dscp: u8) -> i32 {
    /* Comment from RFC8325:
     * [RFC4594] recommends High-Throughput Data be marked AF1x. There is no
     * corresponding fit in the constrained 4 Access Category model, so it is
     * generally recommended to map it to UP 0 / Best Effort. The mapping
     * between UP and IEEE 802.1Q Traffic Type is not defined in the RFC, but
     * AC_BK and AC_BE are closely related to Traffic Types BK and BE.
     */
    match dscp {
        DSCP_CS0 | DSCP_AF11 | DSCP_AF12 | DSCP_AF13 => IEEE8021Q_TT_BE,
        /* Comment from RFC8325: RFC3662 and RFC4594 recommend Low-Priority
         * Data be marked with DSCP CS1; it loosely corresponds to the
         * Background Access Category.
         */
        DSCP_CS1 => IEEE8021Q_TT_BK,
        DSCP_CS2 | DSCP_AF21 | DSCP_AF22 | DSCP_AF23 => IEEE8021Q_TT_EE,
        DSCP_CS3 | DSCP_AF31 | DSCP_AF32 | DSCP_AF33 => IEEE8021Q_TT_CA,
        DSCP_CS4 | DSCP_AF41 | DSCP_AF42 | DSCP_AF43 => IEEE8021Q_TT_VI,
        DSCP_CS5 | DSCP_EF | DSCP_VOICE_ADMIT => IEEE8021Q_TT_VO,
        DSCP_CS6 => IEEE8021Q_TT_IC,
        DSCP_CS7 => IEEE8021Q_TT_NC,
        _ => SIMPLE_IETF_DSCP_TO_IEEE8021Q_TT(dscp),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
