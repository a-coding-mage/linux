/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * H_WATCHDOG Input
 *
 * R4: "flags":
 *
 *         Bits 48-55: "operation"
 */
pub const PSERIES_WDTF_OP_START: usize = 0x100; /* start timer */
pub const PSERIES_WDTF_OP_STOP: usize = 0x200; /* stop timer */
pub const PSERIES_WDTF_OP_QUERY: usize = 0x300; /* query timer capabilities */

/*
 *         Bits 56-63: "timeoutAction" (for "Start Watchdog" only)
 */
pub const PSERIES_WDTF_ACTION_HARD_POWEROFF: usize = 0x1; /* poweroff */
pub const PSERIES_WDTF_ACTION_HARD_RESTART: usize = 0x2; /* restart */
pub const PSERIES_WDTF_ACTION_DUMP_RESTART: usize = 0x3; /* dump + restart */

/*
 * R5: "watchdogNumber":
 *       PAPR says use -1 (all ones) to stop all watchdogs.
 */
pub const PSERIES_WDT_NUM_ALL: usize = usize::MAX;

/*
 * H_WATCHDOG Output
 *
 * R3: Return code
 *
 *     H_SUCCESS    The operation completed.
 *
 *     H_BUSY       The hypervisor is too busy; retry the operation.
 *
 *     H_PARAMETER  The given "flags" are somehow invalid.  Either the
 *                  "operation" or "timeoutAction" is invalid, or a
 *                  reserved bit is set.
 *
 *     H_P2         The given "watchdogNumber" is zero or exceeds the
 *                  supported maximum value.
 *
 *     H_P3         The given "timeoutInMs" is below the supported
 *                  minimum value.
 *
 *     H_NOOP       The given "watchdogNumber" is already stopped.
 *
 *     H_HARDWARE   The operation failed for ineffable reasons.
 *
 *     H_FUNCTION   The H_WATCHDOG hypercall is not supported by this
 *                  hypervisor.
 *
 * R4:
 *
 * - For the "Query Watchdog Capabilities" operation, a 64-bit
 *   structure:
 */
#[inline]
pub const fn PSERIES_WDTQ_MIN_TIMEOUT(cap: usize) -> usize {
    (cap >> 48) & 0xffff
}

#[inline]
pub const fn PSERIES_WDTQ_MAX_NUMBER(cap: usize) -> usize {
    (cap >> 32) & 0xffff
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
