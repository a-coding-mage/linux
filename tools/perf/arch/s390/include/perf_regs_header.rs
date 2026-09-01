// Translated from perf/arch/s390/include/perf_regs.h.
// C includes removed; this translation expects the included Linux/uapi symbols
// such as `PERF_REG_S390_MAX` and `PERF_SAMPLE_REGS_ABI_64` to be provided by
// surrounding bindings.

unsafe extern "C" {
    pub fn perf_regs_load(regs: *mut u64);
}

pub const PERF_REGS_MASK: u64 = (1u64 << (PERF_REG_S390_MAX as u32)) - 1;
pub const PERF_REGS_MAX: u64 = PERF_REG_S390_MAX as u64;
pub const PERF_SAMPLE_REGS_ABI: u64 = PERF_SAMPLE_REGS_ABI_64 as u64;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
