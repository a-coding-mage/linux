/* SPDX-License-Identifier: GPL-2.0 */

/* Config Specific Defines */
pub const CVMX_LLM_NUM_PORTS: i32 = 1;
pub const CVMX_NULL_POINTER_PROTECT: i32 = 1;
pub const CVMX_ENABLE_DEBUG_PRINTS: i32 = 1;
/* PKO queues per port for interface 0 (ports 0-15) */
pub const CVMX_PKO_QUEUES_PER_PORT_INTERFACE0: i32 = 1;
/* PKO queues per port for interface 1 (ports 16-31) */
pub const CVMX_PKO_QUEUES_PER_PORT_INTERFACE1: i32 = 1;
/* Limit on the number of PKO ports enabled for interface 0 */
pub const CVMX_PKO_MAX_PORTS_INTERFACE0: i32 = CVMX_HELPER_PKO_MAX_PORTS_INTERFACE0;
/* Limit on the number of PKO ports enabled for interface 1 */
pub const CVMX_PKO_MAX_PORTS_INTERFACE1: i32 = CVMX_HELPER_PKO_MAX_PORTS_INTERFACE1;
/* PKO queues per port for PCI (ports 32-35) */
pub const CVMX_PKO_QUEUES_PER_PORT_PCI: i32 = 1;
/* PKO queues per port for Loop devices (ports 36-39) */
pub const CVMX_PKO_QUEUES_PER_PORT_LOOP: i32 = 1;

/* FPA allocation: pool sizes in bytes, must be multiple of a cache line */
pub const CVMX_FPA_POOL_0_SIZE: usize = 16 * CVMX_CACHE_LINE_SIZE;
pub const CVMX_FPA_POOL_1_SIZE: usize = 1 * CVMX_CACHE_LINE_SIZE;
pub const CVMX_FPA_POOL_2_SIZE: usize = 8 * CVMX_CACHE_LINE_SIZE;
pub const CVMX_FPA_POOL_3_SIZE: usize = 0 * CVMX_CACHE_LINE_SIZE;
pub const CVMX_FPA_POOL_4_SIZE: usize = 0 * CVMX_CACHE_LINE_SIZE;
pub const CVMX_FPA_POOL_5_SIZE: usize = 0 * CVMX_CACHE_LINE_SIZE;
pub const CVMX_FPA_POOL_6_SIZE: usize = 0 * CVMX_CACHE_LINE_SIZE;
pub const CVMX_FPA_POOL_7_SIZE: usize = 0 * CVMX_CACHE_LINE_SIZE;

/* Pools in use */
/* Packet buffers */
pub const CVMX_FPA_PACKET_POOL: i32 = 0;
pub const CVMX_FPA_PACKET_POOL_SIZE: usize = CVMX_FPA_POOL_0_SIZE;
/* Work queue entries */
pub const CVMX_FPA_WQE_POOL: i32 = 1;
pub const CVMX_FPA_WQE_POOL_SIZE: usize = CVMX_FPA_POOL_1_SIZE;
/* PKO queue command buffers */
pub const CVMX_FPA_OUTPUT_BUFFER_POOL: i32 = 2;
pub const CVMX_FPA_OUTPUT_BUFFER_POOL_SIZE: usize = CVMX_FPA_POOL_2_SIZE;

/*
 * The fetch and add registers are allocated here. They are arranged in
 * order of descending size so that all alignment constraints are
 * automatically met. The values entered always increase by 1. FAU
 * registers are accessed with byte addresses.
 */
macro_rules! CVMX_FAU_REG_64_ADDR {
    ($x:expr) => (($x << 3) + CVMX_FAU_REG_64_START);
}
pub type cvmx_fau_reg_64_t = i32;
pub const CVMX_FAU_REG_64_START: i32 = 0;
pub const CVMX_FAU_REG_64_END: i32 = CVMX_FAU_REG_64_ADDR!(0);

macro_rules! CVMX_FAU_REG_32_ADDR {
    ($x:expr) => (($x << 2) + CVMX_FAU_REG_32_START);
}
pub type cvmx_fau_reg_32_t = i32;
pub const CVMX_FAU_REG_32_START: i32 = CVMX_FAU_REG_64_END;
pub const CVMX_FAU_REG_32_END: i32 = CVMX_FAU_REG_32_ADDR!(0);

macro_rules! CVMX_FAU_REG_16_ADDR {
    ($x:expr) => (($x << 1) + CVMX_FAU_REG_16_START);
}
pub type cvmx_fau_reg_16_t = i32;
pub const CVMX_FAU_REG_16_START: i32 = CVMX_FAU_REG_32_END;
pub const CVMX_FAU_REG_16_END: i32 = CVMX_FAU_REG_16_ADDR!(0);

macro_rules! CVMX_FAU_REG_8_ADDR {
    ($x:expr) => (($x) + CVMX_FAU_REG_8_START);
}
pub type cvmx_fau_reg_8_t = i32;
pub const CVMX_FAU_REG_8_START: i32 = CVMX_FAU_REG_16_END;
pub const CVMX_FAU_REG_8_END: i32 = CVMX_FAU_REG_8_ADDR!(0);

/* First available unallocated FAU address; this is 64-bit aligned. */
pub const CVMX_FAU_REG_AVAIL_BASE: i32 = (CVMX_FAU_REG_8_END + 0x7) & (!0x7_i32);
pub const CVMX_FAU_REG_END: i32 = 2048;

/* Scratchpad memory allocation. These are byte memory addresses. */
/* Generic scratch iobdma area */
pub const CVMX_SCR_SCRATCH: i32 = 0;
/* First location available after cvmx-config.h allocated region. */
pub const CVMX_SCR_REG_AVAIL_BASE: i32 = 8;

/* Bytes to reserve before the beginning of the packet. */
pub const CVMX_HELPER_FIRST_MBUFF_SKIP: i32 = 184;
/* Bytes to reserve in each chained packet element. */
pub const CVMX_HELPER_NOT_FIRST_MBUFF_SKIP: i32 = 0;
/* Enable back pressure for all input ports. */
pub const CVMX_HELPER_ENABLE_BACK_PRESSURE: i32 = 1;
/* Enable IPD in the helper function. */
pub const CVMX_HELPER_ENABLE_IPD: i32 = 0;
/* Type of tag that IPD assigns to incoming packets. */
pub const CVMX_HELPER_INPUT_TAG_TYPE: i32 = CVMX_POW_TAG_TYPE_ORDERED;

pub const CVMX_ENABLE_PARAMETER_CHECKING: i32 = 0;

/* Fields used by PIP to generate the tag on INPUT. */
pub const CVMX_HELPER_INPUT_TAG_IPV6_SRC_IP: i32 = 0;
pub const CVMX_HELPER_INPUT_TAG_IPV6_DST_IP: i32 = 0;
pub const CVMX_HELPER_INPUT_TAG_IPV6_SRC_PORT: i32 = 0;
pub const CVMX_HELPER_INPUT_TAG_IPV6_DST_PORT: i32 = 0;
pub const CVMX_HELPER_INPUT_TAG_IPV6_NEXT_HEADER: i32 = 0;
pub const CVMX_HELPER_INPUT_TAG_IPV4_SRC_IP: i32 = 0;
pub const CVMX_HELPER_INPUT_TAG_IPV4_DST_IP: i32 = 0;
pub const CVMX_HELPER_INPUT_TAG_IPV4_SRC_PORT: i32 = 0;
pub const CVMX_HELPER_INPUT_TAG_IPV4_DST_PORT: i32 = 0;
pub const CVMX_HELPER_INPUT_TAG_IPV4_PROTOCOL: i32 = 0;
pub const CVMX_HELPER_INPUT_TAG_INPUT_PORT: i32 = 1;

/* Select skip mode for input ports. */
pub const CVMX_HELPER_INPUT_PORT_SKIP_MODE: i32 = CVMX_PIP_PORT_CFG_MODE_SKIPL2;

/* Force backpressure to be disabled. */
pub const CVMX_HELPER_DISABLE_RGMII_BACKPRESSURE: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
