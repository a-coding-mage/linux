/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from: */
/* Documentation/netlink/specs/dev-energymodel.yaml */
/* YNL-GEN uapi header */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

pub const DEV_ENERGYMODEL_FAMILY_NAME: &str = "dev-energymodel";
pub const DEV_ENERGYMODEL_FAMILY_VERSION: u32 = 1;

/**
 * enum dev_energymodel_perf_state_flags
 * @DEV_ENERGYMODEL_PERF_STATE_FLAGS_PERF_STATE_INEFFICIENT: The performance
 *   state is inefficient. There is in this perf-domain, another performance
 *   state with a higher frequency but a lower or equal power cost.
 */
pub const DEV_ENERGYMODEL_PERF_STATE_FLAGS_PERF_STATE_INEFFICIENT: u32 = 1;

/**
 * enum dev_energymodel_perf_domain_flags
 * @DEV_ENERGYMODEL_PERF_DOMAIN_FLAGS_PERF_DOMAIN_MICROWATTS: The power values
 *   are in micro-Watts or some other scale.
 * @DEV_ENERGYMODEL_PERF_DOMAIN_FLAGS_PERF_DOMAIN_SKIP_INEFFICIENCIES: Skip
 *   inefficient states when estimating energy consumption.
 * @DEV_ENERGYMODEL_PERF_DOMAIN_FLAGS_PERF_DOMAIN_ARTIFICIAL: The power values
 *   are artificial and might be created by platform missing real power
 *   information.
 */
pub const DEV_ENERGYMODEL_PERF_DOMAIN_FLAGS_PERF_DOMAIN_MICROWATTS: u32 = 1;
pub const DEV_ENERGYMODEL_PERF_DOMAIN_FLAGS_PERF_DOMAIN_SKIP_INEFFICIENCIES: u32 = 2;
pub const DEV_ENERGYMODEL_PERF_DOMAIN_FLAGS_PERF_DOMAIN_ARTIFICIAL: u32 = 4;

pub const DEV_ENERGYMODEL_A_PERF_DOMAIN_PAD: u32 = 1;
pub const DEV_ENERGYMODEL_A_PERF_DOMAIN_PERF_DOMAIN_ID: u32 = 2;
pub const DEV_ENERGYMODEL_A_PERF_DOMAIN_FLAGS: u32 = 3;
pub const DEV_ENERGYMODEL_A_PERF_DOMAIN_CPUS: u32 = 4;
pub const __DEV_ENERGYMODEL_A_PERF_DOMAIN_MAX: u32 = 5;
pub const DEV_ENERGYMODEL_A_PERF_DOMAIN_MAX: u32 = __DEV_ENERGYMODEL_A_PERF_DOMAIN_MAX - 1;

pub const DEV_ENERGYMODEL_A_PERF_TABLE_PERF_DOMAIN_ID: u32 = 1;
pub const DEV_ENERGYMODEL_A_PERF_TABLE_PERF_STATE: u32 = 2;
pub const __DEV_ENERGYMODEL_A_PERF_TABLE_MAX: u32 = 3;
pub const DEV_ENERGYMODEL_A_PERF_TABLE_MAX: u32 = __DEV_ENERGYMODEL_A_PERF_TABLE_MAX - 1;

pub const DEV_ENERGYMODEL_A_PERF_STATE_PAD: u32 = 1;
pub const DEV_ENERGYMODEL_A_PERF_STATE_PERFORMANCE: u32 = 2;
pub const DEV_ENERGYMODEL_A_PERF_STATE_FREQUENCY: u32 = 3;
pub const DEV_ENERGYMODEL_A_PERF_STATE_POWER: u32 = 4;
pub const DEV_ENERGYMODEL_A_PERF_STATE_COST: u32 = 5;
pub const DEV_ENERGYMODEL_A_PERF_STATE_FLAGS: u32 = 6;
pub const __DEV_ENERGYMODEL_A_PERF_STATE_MAX: u32 = 7;
pub const DEV_ENERGYMODEL_A_PERF_STATE_MAX: u32 = __DEV_ENERGYMODEL_A_PERF_STATE_MAX - 1;

pub const DEV_ENERGYMODEL_CMD_GET_PERF_DOMAINS: u32 = 1;
pub const DEV_ENERGYMODEL_CMD_GET_PERF_TABLE: u32 = 2;
pub const DEV_ENERGYMODEL_CMD_PERF_DOMAIN_CREATED: u32 = 3;
pub const DEV_ENERGYMODEL_CMD_PERF_DOMAIN_UPDATED: u32 = 4;
pub const DEV_ENERGYMODEL_CMD_PERF_DOMAIN_DELETED: u32 = 5;
pub const __DEV_ENERGYMODEL_CMD_MAX: u32 = 6;
pub const DEV_ENERGYMODEL_CMD_MAX: u32 = __DEV_ENERGYMODEL_CMD_MAX - 1;

pub const DEV_ENERGYMODEL_MCGRP_EVENT: &str = "event";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
