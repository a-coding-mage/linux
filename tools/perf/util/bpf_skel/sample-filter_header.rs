pub const MAX_FILTERS: usize = 64;
pub const MAX_IDX_HASH: usize = 16 * 1024;
pub const MAX_EVT_HASH: usize = 1024 * 1024;

/* supported filter operations */
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum perf_bpf_filter_op {
    PBF_OP_EQ = 0,
    PBF_OP_NEQ = 1,
    PBF_OP_GT = 2,
    PBF_OP_GE = 3,
    PBF_OP_LT = 4,
    PBF_OP_LE = 5,
    PBF_OP_AND = 6,
    PBF_OP_GROUP_BEGIN = 7,
    PBF_OP_GROUP_END = 8,
    PBF_OP_DONE = 9,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum perf_bpf_filter_term {
    /* No term is in use. */
    PBF_TERM_NONE = 0,
    /* Terms that correspond to PERF_SAMPLE_xx values. */
    PBF_TERM_SAMPLE_START = 1,
    PBF_TERM_IP = 1,              /* SAMPLE_IP = 1U << 0 */
    PBF_TERM_TID = 2,             /* SAMPLE_TID = 1U << 1 */
    PBF_TERM_TIME = 3,            /* SAMPLE_TIME = 1U << 2 */
    PBF_TERM_ADDR = 4,            /* SAMPLE_ADDR = 1U << 3 */
    __PBF_UNUSED_TERM4 = 5,       /* SAMPLE_READ = 1U << 4 */
    __PBF_UNUSED_TERM5 = 6,       /* SAMPLE_CALLCHAIN = 1U << 5 */
    PBF_TERM_ID = 7,              /* SAMPLE_ID = 1U << 6 */
    PBF_TERM_CPU = 8,             /* SAMPLE_CPU = 1U << 7 */
    PBF_TERM_PERIOD = 9,          /* SAMPLE_PERIOD = 1U << 8 */
    __PBF_UNUSED_TERM9 = 10,      /* SAMPLE_STREAM_ID = 1U << 9 */
    __PBF_UNUSED_TERM10 = 11,     /* SAMPLE_RAW = 1U << 10 */
    __PBF_UNUSED_TERM11 = 12,     /* SAMPLE_BRANCH_STACK = 1U << 11 */
    __PBF_UNUSED_TERM12 = 13,     /* SAMPLE_REGS_USER = 1U << 12 */
    __PBF_UNUSED_TERM13 = 14,     /* SAMPLE_STACK_USER = 1U << 13 */
    PBF_TERM_WEIGHT = 15,         /* SAMPLE_WEIGHT = 1U << 14 */
    PBF_TERM_DATA_SRC = 16,       /* SAMPLE_DATA_SRC = 1U << 15 */
    __PBF_UNUSED_TERM16 = 17,     /* SAMPLE_IDENTIFIER = 1U << 16 */
    PBF_TERM_TRANSACTION = 18,    /* SAMPLE_TRANSACTION = 1U << 17 */
    __PBF_UNUSED_TERM18 = 19,     /* SAMPLE_REGS_INTR = 1U << 18 */
    PBF_TERM_PHYS_ADDR = 20,      /* SAMPLE_PHYS_ADDR = 1U << 19 */
    __PBF_UNUSED_TERM20 = 21,     /* SAMPLE_AUX = 1U << 20 */
    PBF_TERM_CGROUP = 22,         /* SAMPLE_CGROUP = 1U << 21 */
    PBF_TERM_DATA_PAGE_SIZE = 23, /* SAMPLE_DATA_PAGE_SIZE = 1U << 22 */
    PBF_TERM_CODE_PAGE_SIZE = 24, /* SAMPLE_CODE_PAGE_SIZE = 1U << 23 */
    PBF_TERM_WEIGHT_STRUCT = 25,  /* SAMPLE_WEIGHT_STRUCT = 1U << 24 */
    PBF_TERM_SAMPLE_END = 25,
    /* Terms computed from BPF helpers. */
    PBF_TERM_UID = 26,
    PBF_TERM_GID = 27,
}

/* BPF map entry for filtering */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct perf_bpf_filter_entry {
    pub op: perf_bpf_filter_op,
    pub part: u32, /* sub-sample type info when it has multiple values */
    pub term: perf_bpf_filter_term,
    pub value: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct idx_hash_key {
    pub evt_id: u64,
    pub tgid: u32,
    pub reserved: u32,
}
