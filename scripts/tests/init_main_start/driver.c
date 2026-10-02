/* SPDX-License-Identifier: GPL-2.0-only */
#include "canonical.h"
#include <linux/stdarg.h>
/* Rust-only helper definitions have no public C header. */
void *rust_helper_ERR_PTR(long error);
bool rust_helper_IS_ERR(const void *ptr);
long rust_helper_PTR_ERR(const void *ptr);
#include "err.inc"

extern int printf(const char *, ...);
extern int vsnprintf(char *, size_t, const char *, va_list);
#undef strtoul
extern unsigned long strtoul(const char *, char **, int);
extern void exit(int) __attribute__((noreturn));
extern void fixture_rust_private(char *, char *, const char *, const char *);
extern bool early_boot_irqs_disabled;
extern void (*late_time_init)(void);
extern bool initcall_debug;
extern void start_kernel(void);
void original_start_kernel(void);

void fixture_irq_disable(void);
void fixture_irq_enable(void);
bool fixture_irqs_disabled(void);
bool fixture_warn(bool value, const char *format);
void fixture_setup_boot_config(void);
void fixture_setup_command_line(char *value);
void fixture_early_numa(void);
void fixture_print_kernel_cmdline(const char *value);
void fixture_parse_early_param(void);
void fixture_print_unknown_bootoptions(void);
int fixture_unknown_bootoption(char *parameter, char *value, const char *doing, void *argument);
int fixture_set_init_arg(char *parameter, char *value, const char *doing, void *argument);
void fixture_initcall_debug_enable(void);
void fixture_canary(void);
void fixture_late_time(void);
void fixture_rest_init(void);

#define CHECK(condition) do { if (!(condition)) { printf("FAIL %d\n", __LINE__); exit(90); } } while (0)
static unsigned int scenario;
static bool irq_disabled;
static char command[] = "arch command", saved[] = "saved command";
static char arguments[] = "init arguments", extra[] = "extra arguments";
static char *static_command_line, *extra_init_args;
static const char *panic_later, *panic_param;
static char log[32768];
static unsigned int used;
struct task_struct init_task;
const char linux_banner[] = "original kernel banner\n";
const struct kernel_param __start___param[3] __section(".fixture.params") = { };
const struct kernel_param __stop___param[0] __section(".fixture.params");
#ifdef CONFIG_BLK_DEV_INITRD
unsigned long initrd_start, initrd_end;
int initrd_below_start_ok;
unsigned long min_low_pfn;
static struct page initrd_page;
#endif
#define BOUNDARY(level) initcall_entry_t __initcall##level##_start[0]
BOUNDARY(0); BOUNDARY(1); BOUNDARY(2); BOUNDARY(3);
BOUNDARY(4); BOUNDARY(5); BOUNDARY(6); BOUNDARY(7);
initcall_entry_t __initcall_end[0];

static void event(const char *name)
{
	int size = snprintf(log + used, sizeof(log) - used, "%s:%u:%u:%u\n",
		name, early_boot_irqs_disabled, irq_disabled, static_key_initialized);
	CHECK(size >= 0 && size < sizeof(log) - used);
	used += size;
}
#define VOID_HOOK(name) void name(void) { event(#name); }
#define INT_HOOK(name) int name(void) { event(#name); return -123; }

void set_task_stack_end_magic(struct task_struct *task)
{
	CHECK(task == &init_task);
	event("stack_magic");
}
VOID_HOOK(smp_setup_processor_id)
#ifdef CONFIG_DEBUG_OBJECTS
VOID_HOOK(debug_objects_early_init)
#endif
#if defined(CONFIG_STACKTRACE_BUILD_ID) || defined(CONFIG_VMCORE_INFO)
VOID_HOOK(init_vmlinux_build_id)
#endif
#ifdef CONFIG_CGROUPS
INT_HOOK(cgroup_init_early)
INT_HOOK(cgroup_init)
#endif
void fixture_irq_disable(void) { event("irq_disable"); irq_disabled = true; }
void fixture_irq_enable(void) { event("irq_enable"); irq_disabled = false; }
bool fixture_irqs_disabled(void) { event("irqs_disabled"); return irq_disabled; }
bool fixture_warn(bool value, const char *format)
{
	event(value ? format : "no_warn");
	return value;
}
VOID_HOOK(boot_cpu_init)
VOID_HOOK(rust_helper_page_address_init)
void setup_arch(char **value) { event("setup_arch"); *value = command; }
VOID_HOOK(mm_core_init_early)
#ifdef CONFIG_JUMP_LABEL
void jump_label_init(void) { event("jump_label_init"); static_key_initialized = true; }
#endif
#ifdef CONFIG_HAVE_STATIC_CALL_INLINE
INT_HOOK(static_call_init)
#endif
#ifdef CONFIG_SECURITY
INT_HOOK(early_security_init)
INT_HOOK(security_init)
#endif
VOID_HOOK(fixture_setup_boot_config)
void fixture_setup_command_line(char *value) { CHECK(value == command); event("setup_command_line"); }
#ifdef CONFIG_SMP
VOID_HOOK(setup_nr_cpu_ids)
VOID_HOOK(call_function_init)
#endif
VOID_HOOK(setup_per_cpu_areas)
VOID_HOOK(smp_prepare_boot_cpu)
VOID_HOOK(fixture_early_numa)
VOID_HOOK(boot_cpu_hotplug_init)
void fixture_print_kernel_cmdline(const char *value) { CHECK(value == saved); event("print_cmdline"); }
VOID_HOOK(fixture_parse_early_param)
VOID_HOOK(fixture_print_unknown_bootoptions)
int fixture_unknown_bootoption(char *parameter, char *value, const char *doing, void *argument)
{
	CHECK(parameter == command && value == arguments && !strcmp(doing, "Booting kernel") && !argument);
	event("unknown_bootoption"); return 37;
}
int fixture_set_init_arg(char *parameter, char *value, const char *doing, void *argument)
{
	CHECK(parameter == command && value == arguments && !argument);
	CHECK(!strcmp(doing, "Setting init args") || !strcmp(doing, "Setting extra init args"));
	event("set_init_arg"); return -37;
}
char *parse_args(const char *doing, char *args, const struct kernel_param *params,
		unsigned int count, s16 min, s16 max, void *argument,
		int (*unknown)(char *, char *, const char *, void *))
{
	CHECK(min == -1 && max == -1 && !argument);
	event(doing);
	if (!strcmp(doing, "Booting kernel")) {
		CHECK(args == command && params == __start___param && count == 3);
		CHECK(unknown(command, arguments, doing, argument) == 37);
		return scenario % 3 == 0 ? NULL : scenario % 3 == 1 ? ERR_PTR(-22) : arguments;
	}
	CHECK(!params && !count);
	CHECK(args == (!strcmp(doing, "Setting init args") ? arguments : extra));
	CHECK(unknown(command, arguments, doing, argument) == -37);
	return ERR_PTR(-123);
}
void random_init_early(const char *value) { CHECK(value == command); event("random_init_early"); }
#ifdef CONFIG_PRINTK
void setup_log_buf(int early) { CHECK(early == 0); event("setup_log_buf"); }
__attribute__((force_align_arg_pointer))
int _printk(const char *format, ...)
{
	va_list arguments;
	va_start(arguments, format);
	int size = vsnprintf(log + used, sizeof(log) - used, format, arguments);
	va_end(arguments);
	CHECK(size >= 0 && size < sizeof(log) - used);
	used += size;
	return size;
}
#endif
VOID_HOOK(vfs_caches_init_early)
VOID_HOOK(sort_main_extable)
VOID_HOOK(trap_init)
VOID_HOOK(mm_core_init)
VOID_HOOK(maple_tree_init)
VOID_HOOK(poking_init)
#ifdef CONFIG_DYNAMIC_FTRACE
VOID_HOOK(ftrace_init)
#endif
#ifdef CONFIG_TRACING
VOID_HOOK(early_trace_init)
VOID_HOOK(trace_init)
#endif
void sched_init(void) { event("sched_init"); if (scenario & 1) irq_disabled = false; }
VOID_HOOK(radix_tree_init)
#ifdef CONFIG_CPU_ISOLATION
VOID_HOOK(housekeeping_init)
#endif
VOID_HOOK(workqueue_init_early)
VOID_HOOK(rcu_init)
VOID_HOOK(kvfree_rcu_init)
VOID_HOOK(fixture_initcall_debug_enable)
#ifdef CONFIG_CONTEXT_TRACKING_USER_FORCE
VOID_HOOK(context_tracking_init)
#endif
INT_HOOK(early_irq_init)
VOID_HOOK(init_IRQ)
#ifdef CONFIG_GENERIC_CLOCKEVENTS
VOID_HOOK(tick_init)
#endif
#ifdef CONFIG_RCU_NOCB_CPU
VOID_HOOK(rcu_init_nohz)
#endif
VOID_HOOK(timers_init)
VOID_HOOK(srcu_init)
VOID_HOOK(hrtimers_init)
VOID_HOOK(softirq_init)
#ifdef CONFIG_VDSO_DATASTORE
VOID_HOOK(vdso_setup_data_pages)
#endif
VOID_HOOK(timekeeping_init)
VOID_HOOK(time_init)
void random_init(void) { event("random_init"); if (scenario & 2) irq_disabled = false; }
#ifdef CONFIG_KFENCE
VOID_HOOK(kfence_init)
#endif
VOID_HOOK(fixture_canary)
#ifdef CONFIG_PERF_EVENTS
VOID_HOOK(perf_event_init)
#endif
#ifdef CONFIG_PROFILING
INT_HOOK(profile_init)
#endif
VOID_HOOK(kmem_cache_init_late)
VOID_HOOK(console_init)

static void finish(const char *state)
{
	CHECK(!early_boot_irqs_disabled && !irq_disabled);
	printf("%sINIT_MAIN_START_%s_OK irq=%u key=%u", log, state, irq_disabled, static_key_initialized);
#ifdef CONFIG_BLK_DEV_INITRD
	printf(" initrd=%lu", initrd_start);
#endif
	printf("\n");
	exit(0);
}
__attribute__((force_align_arg_pointer))
void panic(const char *format, ...)
{
	va_list arguments;
	va_start(arguments, format);
	int size = vsnprintf(log + used, sizeof(log) - used, format, arguments);
	va_end(arguments);
	CHECK(size >= 0 && size < sizeof(log) - used);
	used += size;
	finish("PANIC");
	__builtin_unreachable();
}
#ifdef CONFIG_LOCKDEP
VOID_HOOK(lockdep_init)
#endif
#ifdef CONFIG_DEBUG_LOCKING_API_SELFTESTS
VOID_HOOK(locking_selftest)
#endif
#ifdef CONFIG_BLK_DEV_INITRD
struct page *rust_helper_virt_to_page(const void *address)
{
	CHECK(address == (void *)initrd_start); event("virt_to_page"); return &initrd_page;
}
unsigned long rust_helper_page_to_pfn(const struct page *page)
{
	CHECK(page == &initrd_page); event("page_to_pfn"); return (scenario / 3) % 3;
}
#endif
VOID_HOOK(setup_per_cpu_pageset)
#ifdef CONFIG_NUMA
VOID_HOOK(numa_policy_init)
#endif
#ifdef CONFIG_ACPI
VOID_HOOK(acpi_early_init)
VOID_HOOK(acpi_subsystem_init)
#endif
VOID_HOOK(fixture_late_time)
VOID_HOOK(sched_clock_init)
VOID_HOOK(calibrate_delay)
#ifdef CONFIG_ARCH_HAS_CPU_FINALIZE_INIT
VOID_HOOK(arch_cpu_finalize_init)
#endif
VOID_HOOK(pid_idr_init)
#ifdef CONFIG_MMU
VOID_HOOK(anon_vma_init)
#endif
VOID_HOOK(thread_stack_cache_init)
VOID_HOOK(cred_init)
VOID_HOOK(fork_init)
VOID_HOOK(proc_caches_init)
#ifdef CONFIG_UTS_NS
VOID_HOOK(uts_ns_init)
#endif
#ifdef CONFIG_TIME_NS
VOID_HOOK(time_ns_init)
#endif
#ifdef CONFIG_KEYS
VOID_HOOK(key_init)
#endif
#ifdef CONFIG_KGDB
VOID_HOOK(dbg_late_init)
#endif
#ifdef CONFIG_NET
VOID_HOOK(net_ns_init)
#endif
VOID_HOOK(vfs_caches_init)
VOID_HOOK(pagecache_init)
VOID_HOOK(signals_init)
VOID_HOOK(seq_file_init)
#ifdef CONFIG_PROC_FS
VOID_HOOK(proc_root_init)
#endif
VOID_HOOK(nsfs_init)
VOID_HOOK(pidfs_init)
#ifdef CONFIG_CPUSETS
INT_HOOK(cpuset_init)
#endif
#ifdef CONFIG_MEMCG
INT_HOOK(mem_cgroup_init)
#endif
#ifdef CONFIG_TASKSTATS
VOID_HOOK(taskstats_init_early)
#endif
#ifdef CONFIG_TASK_DELAY_ACCT
VOID_HOOK(delayacct_init)
#endif
VOID_HOOK(arch_post_acpi_subsys_init)
#ifdef CONFIG_KCSAN
VOID_HOOK(kcsan_init)
#endif
void fixture_rest_init(void) { event("rest_init"); finish("REST"); __builtin_unreachable(); }

/* Observe only independently tested sibling owners and CPU/page primitives.
 * Configuration-specific subsystem stubs above remain the actual headers'. */
#undef local_irq_disable
#undef local_irq_enable
#undef irqs_disabled
#define local_irq_disable fixture_irq_disable
#define local_irq_enable fixture_irq_enable
#define irqs_disabled fixture_irqs_disabled
#undef WARN
#define WARN(value, format) fixture_warn(value, format)
#undef page_address_init
#define page_address_init rust_helper_page_address_init
#undef virt_to_page
#undef page_to_pfn
#define virt_to_page rust_helper_virt_to_page
#define page_to_pfn rust_helper_page_to_pfn
#define early_numa_node_init fixture_early_numa
#define boot_init_stack_canary fixture_canary
#define setup_boot_config fixture_setup_boot_config
#define setup_command_line fixture_setup_command_line
#define print_kernel_cmdline fixture_print_kernel_cmdline
#define parse_early_param fixture_parse_early_param
#define print_unknown_bootoptions fixture_print_unknown_bootoptions
#define unknown_bootoption fixture_unknown_bootoption
#define set_init_arg fixture_set_init_arg
#define initcall_debug_enable fixture_initcall_debug_enable
#define rest_init fixture_rest_init
#define start_kernel original_start_kernel
#ifndef CONFIG_SMP
static inline void setup_nr_cpu_ids(void) { }
#endif
#include "original.inc"
#undef start_kernel

int main(int argc, char **argv)
{
	CHECK(argc == 3);
	scenario = strtoul(argv[2], NULL, 10);
	irq_disabled = (scenario & 4) != 0;
	initcall_debug = (scenario & 8) != 0;
	extra_init_args = (scenario & 16) ? extra : NULL;
	late_time_init = (scenario & 32) ? fixture_late_time : NULL;
	panic_later = (scenario & 64) ? "env" : NULL;
	panic_param = "panic argument";
	static_command_line = command;
	saved_command_line = saved;
	early_boot_irqs_disabled = false;
	static_key_initialized = false;
#ifdef CONFIG_BLK_DEV_INITRD
	initrd_start = scenario % 5 ? 0x5000 : 0;
	initrd_below_start_ok = scenario % 7 == 0;
	min_low_pfn = 1;
#endif
	if (argv[1][0] == 'c') original_start_kernel();
	else {
		fixture_rust_private(command, extra_init_args, panic_later, panic_param);
		start_kernel();
	}
	return 91;
}
