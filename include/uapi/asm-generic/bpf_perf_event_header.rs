// Dependency equivalent of: #include <linux/ptrace.h>

/* Export kernel pt_regs structure */
pub type bpf_user_pt_regs_t = pt_regs;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
