// Dependency intent from C: #include <linux/ptrace.h>

/* Export kernel pt_regs structure */
pub type bpf_user_pt_regs_t = pt_regs;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
