// SPDX-License-Identifier: GPL-2.0
/* Itemized compiler macros only; address/PFN/RCU-sched algorithms are Rust. */
#include "physaddr_bindings.h"

bool rust_physaddr_cpu_has_la57(void)
{
	return cpu_feature_enabled(X86_FEATURE_LA57);
}

void rust_physaddr_preempt_disable(void) { preempt_disable(); }
void rust_physaddr_preempt_enable(void) { preempt_enable(); }

#ifdef CONFIG_DEBUG_VIRTUAL
/* Separate sites preserve distinct fatal diagnostics, with this new source
 * as the accurate macro attribution. No Rust debug_assert or panic replaces BUG.
 */
void __noreturn rust_physaddr_bug_image(void) { BUG(); unreachable(); }
void __noreturn rust_physaddr_bug_direct_map(void) { BUG(); unreachable(); }
#endif

#ifdef CONFIG_DEBUG_LOCK_ALLOC
/* _THIS_IP_ is a C compiler label-address primitive. The RCU operation ordering
 * and conditional dispatch stay in Rust; no rcu_read_* inline body is wrapped.
 */
void rust_physaddr_rcu_lock_acquire(void)
{
	lock_acquire(&rcu_sched_lock_map, 0, 0, 2, 0, NULL, _THIS_IP_);
}
void rust_physaddr_rcu_lock_release(void)
{
	lock_release(&rcu_sched_lock_map, _THIS_IP_);
}
#endif

#ifdef CONFIG_PROVE_RCU
/* One native once-only metadata record for each original macro expansion. */
void rust_physaddr_rcu_lock_warn(void)
{
	RCU_LOCKDEP_WARN(!rcu_is_watching(),
			 "rcu_read_lock_sched() used illegally while idle");
}
void rust_physaddr_rcu_unlock_warn(void)
{
	RCU_LOCKDEP_WARN(!rcu_is_watching(),
			 "rcu_read_unlock_sched() used illegally while idle");
}
#endif

#ifdef CONFIG_SPARSEMEM_VMEMMAP
struct mem_section_usage *rust_physaddr_read_usage(struct mem_section_usage * const *ptr)
{
	return READ_ONCE(*ptr);
}
bool rust_physaddr_test_bit(unsigned long nr, const unsigned long *addr)
{
	return test_bit(nr, addr);
}
#endif
