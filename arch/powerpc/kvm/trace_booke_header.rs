/* SPDX-License-Identifier: GPL-2.0 */

/* Translation of trace_booke.h.  The original tracepoint macros and kernel
 * types are supplied by the surrounding build environment. */

#[derive(Copy, Clone)]
pub struct TraceSymbol {
    pub value: u32,
    pub name: &'static str,
}

pub const KVM_TRACE_SYMBOL_EXIT: &[TraceSymbol] = &[
    TraceSymbol { value: 0, name: "CRITICAL" },
    TraceSymbol { value: 1, name: "MACHINE_CHECK" },
    TraceSymbol { value: 2, name: "DATA_STORAGE" },
    TraceSymbol { value: 3, name: "INST_STORAGE" },
    TraceSymbol { value: 4, name: "EXTERNAL" },
    TraceSymbol { value: 5, name: "ALIGNMENT" },
    TraceSymbol { value: 6, name: "PROGRAM" },
    TraceSymbol { value: 7, name: "FP_UNAVAIL" },
    TraceSymbol { value: 8, name: "SYSCALL" },
    TraceSymbol { value: 9, name: "AP_UNAVAIL" },
    TraceSymbol { value: 10, name: "DECREMENTER" },
    TraceSymbol { value: 11, name: "FIT" },
    TraceSymbol { value: 12, name: "WATCHDOG" },
    TraceSymbol { value: 13, name: "DTLB_MISS" },
    TraceSymbol { value: 14, name: "ITLB_MISS" },
    TraceSymbol { value: 15, name: "DEBUG" },
    TraceSymbol { value: 32, name: "SPE_UNAVAIL" },
    TraceSymbol { value: 33, name: "SPE_FP_DATA" },
    TraceSymbol { value: 34, name: "SPE_FP_ROUND" },
    TraceSymbol { value: 35, name: "PERFORMANCE_MONITOR" },
    TraceSymbol { value: 36, name: "DOORBELL" },
    TraceSymbol { value: 37, name: "DOORBELL_CRITICAL" },
    TraceSymbol { value: 38, name: "GUEST_DBELL" },
    TraceSymbol { value: 39, name: "GUEST_DBELL_CRIT" },
    TraceSymbol { value: 40, name: "HV_SYSCALL" },
    TraceSymbol { value: 41, name: "HV_PRIV" },
];

#[repr(C)]
pub struct KvmExitEntry {
    pub exit_nr: u32,
    pub pc: usize,
    pub msr: usize,
    pub dar: usize,
    pub last_inst: usize,
}

#[repr(C)]
pub struct KvmBooke206StlbWriteEntry {
    pub mas0: u32,
    pub mas8: u32,
    pub mas1: u32,
    pub mas2: u64,
    pub mas7_3: u64,
}

#[repr(C)]
pub struct KvmBooke206GtlbWriteEntry {
    pub mas0: u32,
    pub mas1: u32,
    pub mas2: u64,
    pub mas7_3: u64,
}

#[repr(C)]
pub struct KvmBooke206RefReleaseEntry {
    pub pfn: u64,
    pub flags: u32,
}

#[cfg(feature = "CONFIG_SPE_POSSIBLE")]
pub const KVM_TRACE_SYMBOL_IRQPRIO_SPE: &[TraceSymbol] = &[
    TraceSymbol { value: BOOKE_IRQPRIO_SPE_UNAVAIL, name: "SPE_UNAVAIL" },
    TraceSymbol { value: BOOKE_IRQPRIO_SPE_FP_DATA, name: "SPE_FP_DATA" },
    TraceSymbol { value: BOOKE_IRQPRIO_SPE_FP_ROUND, name: "SPE_FP_ROUND" },
];

#[cfg(not(feature = "CONFIG_SPE_POSSIBLE"))]
pub const KVM_TRACE_SYMBOL_IRQPRIO_SPE: &[TraceSymbol] = &[];

#[cfg(feature = "CONFIG_PPC_E500MC")]
pub const KVM_TRACE_SYMBOL_IRQPRIO_E500MC: &[TraceSymbol] = &[
    TraceSymbol { value: BOOKE_IRQPRIO_ALTIVEC_UNAVAIL, name: "ALTIVEC_UNAVAIL" },
    TraceSymbol { value: BOOKE_IRQPRIO_ALTIVEC_ASSIST, name: "ALTIVEC_ASSIST" },
];

#[cfg(not(feature = "CONFIG_PPC_E500MC"))]
pub const KVM_TRACE_SYMBOL_IRQPRIO_E500MC: &[TraceSymbol] = &[];

/* BOOKE_IRQPRIO_* constants are provided by the PowerPC KVM dependencies. */
pub const KVM_TRACE_SYMBOL_IRQPRIO: &[TraceSymbol] = &[
    TraceSymbol { value: BOOKE_IRQPRIO_DATA_STORAGE, name: "DATA_STORAGE" },
    TraceSymbol { value: BOOKE_IRQPRIO_INST_STORAGE, name: "INST_STORAGE" },
    TraceSymbol { value: BOOKE_IRQPRIO_ALIGNMENT, name: "ALIGNMENT" },
    TraceSymbol { value: BOOKE_IRQPRIO_PROGRAM, name: "PROGRAM" },
    TraceSymbol { value: BOOKE_IRQPRIO_FP_UNAVAIL, name: "FP_UNAVAIL" },
    TraceSymbol { value: BOOKE_IRQPRIO_SYSCALL, name: "SYSCALL" },
    TraceSymbol { value: BOOKE_IRQPRIO_AP_UNAVAIL, name: "AP_UNAVAIL" },
    TraceSymbol { value: BOOKE_IRQPRIO_DTLB_MISS, name: "DTLB_MISS" },
    TraceSymbol { value: BOOKE_IRQPRIO_ITLB_MISS, name: "ITLB_MISS" },
    TraceSymbol { value: BOOKE_IRQPRIO_MACHINE_CHECK, name: "MACHINE_CHECK" },
    TraceSymbol { value: BOOKE_IRQPRIO_DEBUG, name: "DEBUG" },
    TraceSymbol { value: BOOKE_IRQPRIO_CRITICAL, name: "CRITICAL" },
    TraceSymbol { value: BOOKE_IRQPRIO_WATCHDOG, name: "WATCHDOG" },
    TraceSymbol { value: BOOKE_IRQPRIO_EXTERNAL, name: "EXTERNAL" },
    TraceSymbol { value: BOOKE_IRQPRIO_FIT, name: "FIT" },
    TraceSymbol { value: BOOKE_IRQPRIO_DECREMENTER, name: "DECREMENTER" },
    TraceSymbol { value: BOOKE_IRQPRIO_PERFORMANCE_MONITOR, name: "PERFORMANCE_MONITOR" },
    TraceSymbol { value: BOOKE_IRQPRIO_EXTERNAL_LEVEL, name: "EXTERNAL_LEVEL" },
    TraceSymbol { value: BOOKE_IRQPRIO_DBELL, name: "DBELL" },
    TraceSymbol { value: BOOKE_IRQPRIO_DBELL_CRIT, name: "DBELL_CRIT" },
];

#[repr(C)]
pub struct KvmBookeQueueIrqprioEntry {
    pub cpu_nr: u32,
    pub priority: u32,
    pub pending: usize,
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
