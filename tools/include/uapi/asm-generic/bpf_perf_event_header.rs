// Dependency intent from C: #include <linux/ptrace.h>

/* Export kernel pt_regs structure */
pub type bpf_user_pt_regs_t = pt_regs;
