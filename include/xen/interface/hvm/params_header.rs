/* SPDX-License-Identifier: MIT */

// Parameter space for HVMOP_{set,get}_param.

pub const HVM_PARAM_CALLBACK_IRQ: u32 = 0;
/*
 * How should CPU0 event-channel notifications be delivered?
 *
 * If val == 0 then CPU0 event-channel notifications are not delivered.
 * If val != 0, val[63:56] encodes the type, as follows:
 */

pub const HVM_PARAM_CALLBACK_TYPE_GSI: u32 = 0;
/*
 * val[55:0] is a delivery GSI.  GSI 0 cannot be used, as it aliases val == 0,
 * and disables all notifications.
 */

pub const HVM_PARAM_CALLBACK_TYPE_PCI_INTX: u32 = 1;
/*
 * val[55:0] is a delivery PCI INTx line:
 * Domain = val[47:32], Bus = val[31:16] DevFn = val[15:8], IntX = val[1:0]
 */

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub const HVM_PARAM_CALLBACK_TYPE_VECTOR: u32 = 2;
/*
 * val[7:0] is a vector number.  Check for XENFEAT_hvm_callback_vector to know
 * if this delivery method is available.
 */

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub const HVM_PARAM_CALLBACK_TYPE_PPI: u32 = 2;
/*
 * val[55:16] needs to be zero.
 * val[15:8] is interrupt flag of the PPI used by event-channel:
 *  bit 8: the PPI is edge(1) or level(0) triggered
 *  bit 9: the PPI is active low(1) or high(0)
 * val[7:0] is a PPI number used by event-channel.
 * This is only used by ARM/ARM64 and masking/eoi the interrupt associated to
 * the notification is handled by the interrupt controller.
 */

pub const HVM_PARAM_STORE_PFN: u32 = 1;
pub const HVM_PARAM_STORE_EVTCHN: u32 = 2;

pub const HVM_PARAM_PAE_ENABLED: u32 = 4;

pub const HVM_PARAM_IOREQ_PFN: u32 = 5;

pub const HVM_PARAM_BUFIOREQ_PFN: u32 = 6;

/*
 * Set mode for virtual timers (currently x86 only):
 *  delay_for_missed_ticks (default):
 *   Do not advance a vcpu's time beyond the correct delivery time for
 *   interrupts that have been missed due to preemption. Deliver missed
 *   interrupts when the vcpu is rescheduled and advance the vcpu's virtual
 *   time stepwise for each one.
 *  no_delay_for_missed_ticks:
 *   As above, missed interrupts are delivered, but guest time always tracks
 *   wallclock (i.e., real) time while doing so.
 *  no_missed_ticks_pending:
 *   No missed interrupts are held pending. Instead, to ensure ticks are
 *   delivered at some non-zero rate, if we detect missed ticks then the
 *   internal tick alarm is not disabled if the VCPU is preempted during the
 *   next tick period.
 *  one_missed_tick_pending:
 *   Missed interrupts are collapsed together and delivered as one 'late tick'.
 *   Guest time always tracks wallclock (i.e., real) time.
 */
pub const HVM_PARAM_TIMER_MODE: u32 = 10;
pub const HVMPTM_delay_for_missed_ticks: u32 = 0;
pub const HVMPTM_no_delay_for_missed_ticks: u32 = 1;
pub const HVMPTM_no_missed_ticks_pending: u32 = 2;
pub const HVMPTM_one_missed_tick_pending: u32 = 3;

/* Boolean: Enable virtual HPET (high-precision event timer)? (x86-only) */
pub const HVM_PARAM_HPET_ENABLED: u32 = 11;

/* Identity-map page directory used by Intel EPT when CR0.PG=0. */
pub const HVM_PARAM_IDENT_PT: u32 = 12;

/* Device Model domain, defaults to 0. */
pub const HVM_PARAM_DM_DOMAIN: u32 = 13;

/* ACPI S state: currently support S0 and S3 on x86. */
pub const HVM_PARAM_ACPI_S_STATE: u32 = 14;

/* TSS used on Intel when CR0.PE=0. */
pub const HVM_PARAM_VM86_TSS: u32 = 15;

/* Boolean: Enable aligning all periodic vpts to reduce interrupts */
pub const HVM_PARAM_VPT_ALIGN: u32 = 16;

/* Console debug shared memory ring and event channel */
pub const HVM_PARAM_CONSOLE_PFN: u32 = 17;
pub const HVM_PARAM_CONSOLE_EVTCHN: u32 = 18;

pub const HVM_NR_PARAMS: u32 = 19;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
