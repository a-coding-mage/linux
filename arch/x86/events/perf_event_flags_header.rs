/*
 * struct hw_perf_event.flags flags
 */
pub const PEBS_LDLAT: u32 = 0x0000001; /* ld+ldlat data address sampling */
pub const PEBS_ST: u32 = 0x0000002; /* st data address sampling */
pub const PEBS_ST_HSW: u32 = 0x0000004; /* haswell style datala, store */
pub const PEBS_LD_HSW: u32 = 0x0000008; /* haswell style datala, load */
pub const PEBS_NA_HSW: u32 = 0x0000010; /* haswell style datala, unknown */
pub const EXCL: u32 = 0x0000020; /* HT exclusivity on counter */
pub const DYNAMIC: u32 = 0x0000040; /* dynamic alloc'd constraint */
pub const PEBS_CNTR: u32 = 0x0000080; /* PEBS counters snapshot */
pub const EXCL_ACCT: u32 = 0x0000100; /* accounted EXCL event */
pub const AUTO_RELOAD: u32 = 0x0000200; /* use PEBS auto-reload */
pub const LARGE_PEBS: u32 = 0x0000400; /* use large PEBS */
pub const PEBS_VIA_PT: u32 = 0x0000800; /* use PT buffer for PEBS */
pub const PAIR: u32 = 0x0001000; /* Large Increment per Cycle */
pub const LBR_SELECT: u32 = 0x0002000; /* Save/Restore MSR_LBR_SELECT */
pub const TOPDOWN: u32 = 0x0004000; /* Count Topdown slots/metrics events */
pub const PEBS_STLAT: u32 = 0x0008000; /* st+stlat data address sampling */
pub const AMD_BRS: u32 = 0x0010000; /* AMD Branch Sampling */
pub const PEBS_LAT_HYBRID: u32 = 0x0020000; /* ld and st lat for hybrid */
pub const NEEDS_BRANCH_STACK: u32 = 0x0040000; /* require branch stack setup */
pub const BRANCH_COUNTERS: u32 = 0x0080000; /* logs the counters in the extra space of each branch */
pub const ACR: u32 = 0x0100000; /* Auto counter reload */
pub const UNPRIVILEGED: u32 = 0x0200000; /* Unprivileged event (wrt perf_allow_kernel()) */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
