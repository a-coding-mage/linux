// SPDX-License-Identifier: GPL-2.0
/* Native PSI storage, registration and header leaves. No psi.c inclusion and
 * no original C algorithm fallback. Rust owns pressure accounting/control.
 */
#include "psi_native_bindings.h"
#error "Lupos PSI source proposal is not build-admitted; qualification is incomplete"

int rust_psi_bug __read_mostly;
u64 rust_psi_period __read_mostly;
DEFINE_STATIC_KEY_FALSE(psi_disabled);
static DEFINE_STATIC_KEY_TRUE(rust_psi_cgroups_key);
#ifdef CONFIG_PSI_DEFAULT_DISABLED
bool rust_psi_enable;
#else
bool rust_psi_enable = true;
#endif

static DEFINE_PER_CPU(struct psi_group_cpu, rust_psi_system_group_pcpu);
struct psi_group psi_system = {
	.pcpu = &rust_psi_system_group_pcpu,
};
static DEFINE_PER_CPU(seqcount_t, rust_psi_seq) = SEQCNT_ZERO(rust_psi_seq);

__setup("psi=", rust_psi_setup);
EXPORT_SYMBOL_GPL(psi_memstall_enter);
EXPORT_SYMBOL_GPL(psi_memstall_leave);

#define PSI_LEAF(ret, name, args, body) ret rust_psi_##name args body
#include "psi_native_primitives.inc"
#undef PSI_LEAF

#ifdef CONFIG_PROC_FS
#define PSI_PROC_OPS(resource) \
	static const struct proc_ops rust_psi_##resource##_proc_ops = { \
		.proc_open = rust_psi_##resource##_open, \
		.proc_read = seq_read, \
		.proc_lseek = seq_lseek, \
		.proc_write = rust_psi_##resource##_write, \
		.proc_poll = rust_psi_fop_poll, \
		.proc_release = rust_psi_fop_release, \
	}; \
	void rust_psi_proc_create_##resource(void) \
	{ \
		proc_create("pressure/" #resource, 0666, NULL, \
			    &rust_psi_##resource##_proc_ops); \
	}
PSI_PROC_OPS(io)
PSI_PROC_OPS(memory)
PSI_PROC_OPS(cpu)
#ifdef CONFIG_IRQ_TIME_ACCOUNTING
PSI_PROC_OPS(irq)
#endif
#undef PSI_PROC_OPS

void rust_psi_proc_mkdir(void)
{
	proc_mkdir("pressure", NULL);
}
module_init(rust_psi_proc_init);
#endif
