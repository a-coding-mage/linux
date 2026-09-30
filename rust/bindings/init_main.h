/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Canonical declarations for the staged Rust init/main implementation.
 * Include this before other kernel headers: normal Rust bindings use MODULE,
 * which otherwise hides the built-in setup records behind init.h's guard.
 * All included declarations use the built-in context before MODULE is restored.
 * This file supplies no C implementation or duplicate boot state.
 */
#ifndef _RUST_BINDINGS_INIT_MAIN_H
#define _RUST_BINDINGS_INIT_MAIN_H

#ifdef _LINUX_INIT_H
#error "init_main.h must precede linux/init.h in the binding input"
#endif

#pragma push_macro("MODULE")
#undef MODULE
#include <linux/init.h>

#include <linux/string.h>
#include <linux/acpi.h>
#include <linux/async.h>
#include <linux/binfmts.h>
#include <linux/bootconfig.h>
#include <linux/buildid.h>
#include <linux/cache.h>
#include <linux/completion.h>
#include <linux/console.h>
#include <linux/context_tracking.h>
#include <linux/cpu.h>
#include <linux/cpuset.h>
#include <linux/cgroup.h>
#include <linux/cred.h>
#include <linux/debug_locks.h>
#include <linux/debugobjects.h>
#include <linux/delay.h>
#include <linux/delayacct.h>
#include <linux/device/driver.h>
#include <linux/dynamic_debug.h>
#include <linux/extable.h>
#include <linux/file.h>
#include <linux/fs.h>
#include <linux/fs_struct.h>
#include <linux/ftrace.h>
#include <linux/init_syscalls.h>
#include <linux/initrd.h>
#include <linux/integrity.h>
#include <linux/interrupt.h>
#include <linux/jump_label.h>
#include <linux/kallsyms.h>
#include <linux/kcsan.h>
#include <linux/kfence.h>
#include <linux/key.h>
#include <linux/kgdb.h>
#include <linux/kprobes.h>
#include <linux/ksysfs.h>
#include <linux/kthread.h>
#include <linux/memblock.h>
#include <linux/maple_tree.h>
#include <linux/memcontrol.h>
#include <linux/mempolicy.h>
#include <linux/mm.h>
#include <linux/moduleparam.h>
#include <linux/moduleloader.h>
#include <linux/nmi.h>
#include <linux/nsfs.h>
#include <linux/padata.h>
#include <linux/perf_event.h>
#include <linux/panic.h>
#include <linux/poison.h>
#include <linux/pid.h>
#include <linux/pidfs.h>
#include <linux/pid_namespace.h>
#include <linux/printk.h>
#include <linux/proc_fs.h>
#include <linux/profile.h>
#include <linux/ptdump.h>
#include <linux/random.h>
#include <linux/randomize_kstack.h>
#include <linux/pti.h>
#include <linux/rcupdate.h>
/* The declaration-only input is also used by unoptimized ownership probes.
 * rmap's unused BUILD_BUG inlines omit a return when __OPTIMIZE__ is absent. */
#ifndef __OPTIMIZE__
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wreturn-type"
#endif
#include <linux/rmap.h>
#ifndef __OPTIMIZE__
#pragma GCC diagnostic pop
#endif
#include <linux/rodata_test.h>
#include <linux/sched/init.h>
#include <linux/sched/clock.h>
#include <linux/sched/isolation.h>
#include <linux/sched/task.h>
#include <linux/sched/task_stack.h>
#include <linux/security.h>
#include <linux/signal.h>
#include <linux/slab.h>
#include <linux/stackprotector.h>
#include <linux/static_call.h>
#include <linux/srcu.h>
#include <linux/sysctl.h>
#include <linux/taskstats_kern.h>
#include <linux/tick.h>
#include <linux/timekeeping.h>
#include <linux/topology.h>
#include <linux/time_namespace.h>
#include <linux/tracepoint.h>
#include <linux/utsname.h>
#include <linux/vdso_datastore.h>
#include <net/net_namespace.h>
#include <asm/setup.h>
#include <asm/sections.h>
#ifdef CONFIG_ARM64
#include <asm/mmu_context.h>
#endif
#include <trace/events/initcall.h>

#ifdef TRACEPOINTS_ENABLED
/* Defined by the staged init/main_tracepoints.c owner, using the original
 * trace_* expansion including its unconditional LOCKDEP RCU check. */
void rust_main_trace_initcall_start(initcall_t function);
void rust_main_trace_initcall_finish(initcall_t function, int result);
void rust_main_trace_initcall_level(const char *level);
#endif
#include <kunit/test.h>

enum {
	RUST_INIT_MAIN_COMMAND_LINE_SIZE = COMMAND_LINE_SIZE,
	RUST_INIT_MAIN_MAX_INIT_ARGS = CONFIG_INIT_ENV_ARG_LIMIT,
	RUST_INIT_MAIN_MAX_INIT_ENVS = CONFIG_INIT_ENV_ARG_LIMIT,
	RUST_INIT_MAIN_SMP_CACHE_BYTES = SMP_CACHE_BYTES,
	RUST_INIT_MAIN_CMDLINE_LOG_WRAP_IDEAL_LEN = CONFIG_CMDLINE_LOG_WRAP_IDEAL_LEN,
	RUST_INIT_MAIN_DPRINTK_CLASS_BITS = CLS_BITS,
	RUST_INIT_MAIN_DPRINTK_CLASS_DEFAULT = _DPRINTK_CLASS_DFLT,
	RUST_INIT_MAIN_DPRINTK_FLAGS_PRINT = _DPRINTK_FLAGS_PRINT,
	RUST_INIT_MAIN_DPRINTK_FLAGS_STACK = _DPRINTK_FLAGS_INCL_STACK,
	RUST_INIT_MAIN_THREAD_SIZE = THREAD_SIZE,
	RUST_INIT_MAIN_PAGE_SIZE = PAGE_SIZE,
	RUST_INIT_MAIN_KSYM_SYMBOL_LEN = KSYM_SYMBOL_LEN,
	RUST_INIT_MAIN_GFP_BITS_MASK = __GFP_BITS_MASK,
};

#ifdef CONFIG_LIST_HARDENED
/* The actual forward-only ABI boundary is defined in rust/helpers/list.c.
 * The original reporter can use __preserve_most, unsupported by bindgen/Rust. */
bool rust_helper___list_add_valid_or_report(struct list_head *new,
					  struct list_head *prev,
					  struct list_head *next);
#endif

#ifdef CONFIG_CPUSETS
/* Ordinary C ABI boundaries for the original IRQ and _Generic seqcount
 * primitives. The init-task state transition remains in Rust. */
unsigned long rust_helper_local_irq_save(void);
void rust_helper_local_irq_restore(unsigned long flags);
void rust_helper_write_seqcount_spinlock_begin(seqcount_spinlock_t *sequence);
void rust_helper_write_seqcount_spinlock_end(seqcount_spinlock_t *sequence);
#endif

#ifdef CONFIG_BLK_DEV_INITRD
struct page *rust_helper_virt_to_page(const void *address);
unsigned long rust_helper_page_to_pfn(const struct page *page);
#endif
void rust_helper_page_address_init(void);

#define RUST_INIT_MAIN_DEFAULT_INIT CONFIG_DEFAULT_INIT
#define RUST_INIT_MAIN_CANARY_MASK CANARY_MASK

#ifndef CONFIG_JUMP_LABEL
void rust_helper_static_key_enable(struct static_key *key);
void rust_helper_static_key_disable(struct static_key *key);
#endif

#if defined(CONFIG_USE_PERCPU_NUMA_NODE_ID) && !defined(cpu_to_node)
enum { RUST_INIT_MAIN_INITIALIZE_NUMA_NODES = 1 };
#else
enum { RUST_INIT_MAIN_INITIALIZE_NUMA_NODES = 0 };
#endif

#ifdef CONFIG_ARM64_PTR_AUTH
bool rust_helper_system_supports_address_auth(void);
enum { RUST_INIT_MAIN_PTRAUTH_ENABLE_BITS =
	SCTLR_ELx_ENIA | SCTLR_ELx_ENIB | SCTLR_ELx_ENDA | SCTLR_ELx_ENDB };
#endif

/* Compiler-metadata allocation boundary in staged init/main_alloc.c. */
char *rust_init_main_kzalloc_command_line(size_t length);

const phys_addr_t RUST_INIT_MAIN_MEMBLOCK_LOW_LIMIT = MEMBLOCK_LOW_LIMIT;
const phys_addr_t RUST_INIT_MAIN_MEMBLOCK_ALLOC_ACCESSIBLE = MEMBLOCK_ALLOC_ACCESSIBLE;
#if defined(CONFIG_X86_64) || defined(CONFIG_ARM64)
enum { RUST_INIT_MAIN_PREEMPT_NEED_RESCHED = PREEMPT_NEED_RESCHED };
#endif
#ifdef CONFIG_X86_64
enum { RUST_INIT_MAIN_IRQ_MASK = X86_EFLAGS_IF };
#elif defined(CONFIG_ARM64)
enum { RUST_INIT_MAIN_IRQ_MASK = PSR_I_BIT };
#endif

#pragma pop_macro("MODULE")

/* SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783 */
#endif
