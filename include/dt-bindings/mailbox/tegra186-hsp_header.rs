/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides constants for binding nvidia,tegra186-hsp.
 */

/*
 * These define the type of mailbox that is to be used (doorbell, shared
 * mailbox, shared semaphore or arbitrated semaphore).
 */
pub const TEGRA_HSP_MBOX_TYPE_DB: u32 = 0x0;
pub const TEGRA_HSP_MBOX_TYPE_SM: u32 = 0x1;
pub const TEGRA_HSP_MBOX_TYPE_SS: u32 = 0x2;
pub const TEGRA_HSP_MBOX_TYPE_AS: u32 = 0x3;

/*
 * These define the types of shared mailbox supported based on data size.
 */
pub const TEGRA_HSP_MBOX_TYPE_SM_128BIT: u32 = 1 << 8;

/*
 * These defines represent the bit associated with the given master ID in the
 * doorbell registers.
 */
pub const TEGRA_HSP_DB_MASTER_CCPLEX: u32 = 17;
pub const TEGRA_HSP_DB_MASTER_BPMP: u32 = 19;

/*
 * Shared mailboxes are unidirectional, so the direction needs to be specified
 * in the device tree.
 */
pub const TEGRA_HSP_SM_MASK: u32 = 0x00ffffff;
pub const TEGRA_HSP_SM_FLAG_RX: u32 = 0 << 31;
pub const TEGRA_HSP_SM_FLAG_TX: u32 = 1 << 31;

#[macro_export]
macro_rules! TEGRA_HSP_SM_RX {
    ($x:expr) => {
        $crate::TEGRA_HSP_SM_FLAG_RX | (($x) & $crate::TEGRA_HSP_SM_MASK)
    };
}

#[macro_export]
macro_rules! TEGRA_HSP_SM_TX {
    ($x:expr) => {
        $crate::TEGRA_HSP_SM_FLAG_TX | (($x) & $crate::TEGRA_HSP_SM_MASK)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
