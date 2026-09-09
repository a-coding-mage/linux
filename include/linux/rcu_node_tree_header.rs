/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * RCU node combining tree definitions.  These are used to compute
 * global attributes while avoiding common-case global contention.
 *
 * This seemingly RCU-private file must be available to SRCU users
 * because the size of the TREE SRCU srcu_struct structure depends
 * on these definitions.
 *
 * Copyright IBM Corporation, 2017
 *
 * Author: Paul E. McKenney <paulmck@linux.ibm.com>
 */

/* The following configuration values are supplied by the surrounding build. */

#[cfg(target_pointer_width = "64")]
pub const RCU_FANOUT: usize = 64;
#[cfg(not(target_pointer_width = "64"))]
pub const RCU_FANOUT: usize = 32;

pub const RCU_FANOUT_LEAF: usize = 16;

pub const RCU_FANOUT_1: usize = RCU_FANOUT_LEAF;
pub const RCU_FANOUT_2: usize = RCU_FANOUT_1 * RCU_FANOUT;
pub const RCU_FANOUT_3: usize = RCU_FANOUT_2 * RCU_FANOUT;
pub const RCU_FANOUT_4: usize = RCU_FANOUT_3 * RCU_FANOUT;

#[inline]
pub const fn div_round_up(n: usize, d: usize) -> usize {
    (n + d - 1) / d
}

/*
 * NR_CPUS is a build-time configuration value in the C source.  Select the
 * corresponding branch here by defining the matching Rust configuration
 * value in the surrounding build; the four-level form is the conservative
 * declaration used when no such selection is available.
 */

#[cfg(rcu_num_lvls = "1")]
pub const RCU_NUM_LVLS: usize = 1;
#[cfg(rcu_num_lvls = "2")]
pub const RCU_NUM_LVLS: usize = 2;
#[cfg(rcu_num_lvls = "3")]
pub const RCU_NUM_LVLS: usize = 3;
#[cfg(any(rcu_num_lvls = "4", not(any(rcu_num_lvls = "1", rcu_num_lvls = "2", rcu_num_lvls = "3"))))]
pub const RCU_NUM_LVLS: usize = 4;

pub const NUM_RCU_LVL_0: usize = 1;
pub const NUM_RCU_LVL_1: usize = div_round_up(NR_CPUS, RCU_FANOUT_3);
pub const NUM_RCU_LVL_2: usize = div_round_up(NR_CPUS, RCU_FANOUT_2);
pub const NUM_RCU_LVL_3: usize = div_round_up(NR_CPUS, RCU_FANOUT_1);
pub const NUM_RCU_NODES: usize =
    NUM_RCU_LVL_0 + NUM_RCU_LVL_1 + NUM_RCU_LVL_2 + NUM_RCU_LVL_3;

pub const NUM_RCU_LVL_INIT: [usize; 4] = [
    NUM_RCU_LVL_0,
    NUM_RCU_LVL_1,
    NUM_RCU_LVL_2,
    NUM_RCU_LVL_3,
];

pub const RCU_NODE_NAME_INIT: [&str; 4] = [
    "rcu_node_0",
    "rcu_node_1",
    "rcu_node_2",
    "rcu_node_3",
];

pub const RCU_FQS_NAME_INIT: [&str; 4] = [
    "rcu_node_fqs_0",
    "rcu_node_fqs_1",
    "rcu_node_fqs_2",
    "rcu_node_fqs_3",
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
