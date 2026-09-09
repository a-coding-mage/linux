/* SPDX-License-Identifier: GPL-2.0 */

/* To avoid some locking problems, we hard allocate certain PILs
 * for SMP cross call messages that must do a etrap/rtrap.
 *
 * A local_irq_disable() does not block the cross call delivery, so
 * when SMP locking is an issue we reschedule the event into a PIL
 * interrupt which is blocked by local_irq_disable().
 *
 * In fact any XCALL which has to etrap/rtrap has a problem because
 * it is difficult to prevent rtrap from running BH's, and that would
 * need to be done if the XCALL arrived while %pil==PIL_NORMAL_MAX.
 *
 * Finally, in order to handle profiling events even when a
 * local_irq_disable() is in progress, we only disable up to level 14
 * interrupts.  Profile counter overflow interrupts arrive at level
 * 15.
 */
pub const PIL_SMP_CALL_FUNC: i32 = 1;
pub const PIL_SMP_RECEIVE_SIGNAL: i32 = 2;
pub const PIL_SMP_CAPTURE: i32 = 3;
pub const PIL_DEVICE_IRQ: i32 = 5;
pub const PIL_SMP_CALL_FUNC_SNGL: i32 = 6;
pub const PIL_DEFERRED_PCR_WORK: i32 = 7;
pub const PIL_KGDB_CAPTURE: i32 = 8;
pub const PIL_NORMAL_MAX: i32 = 14;
pub const PIL_NMI: i32 = 15;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
