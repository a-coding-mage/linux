/* SPDX-License-Identifier: GPL-2.0-only */
#include <linux/init.h>
#include <linux/completion.h>
#include <linux/cpu.h>
#include <linux/kthread.h>
#include <linux/mempolicy.h>
#include <linux/pid_namespace.h>
#include <linux/sched/task.h>
#include <linux/string.h>
#include <linux/ctype.h>

extern void exit(int) __noreturn;
extern int puts(const char *);
extern int printf(const char *, ...);
extern long syscall(long, ...);
extern unsigned long strtoul(const char *, char **, int);
extern void rust_rest(void) __noreturn;
extern void original_rest(void) __noreturn;
extern unsigned int rust_boot_cpu(void);
extern const struct cpumask *rust_boot_mask(unsigned int);
extern struct completion *rust_completion(void);
extern struct completion *original_completion(void);
extern int fixture_kernel_init(void *);
extern void rust_helper_rcu_read_lock(void);
extern void rust_helper_rcu_read_unlock(void);

enum system_states system_state;
struct pid_namespace init_pid_ns;
struct task_struct *kthreadd_task;
static struct task_struct init_task_fixture, kthreadd_fixture;
static unsigned int events[32], count, read_depth, selected_cpu, initial_flags;
static int lookup_count, pin_result, init_pid_result, thread_pid_result;
static struct completion *expected_completion;
int cpu_number;
#include "bitmap.inc"

#define CHECK(condition) do { if (!(condition)) exit(__LINE__); } while (0)
static void record(unsigned int event)
{
	CHECK(count < ARRAY_SIZE(events));
	CHECK(system_state == (event >= 12 ? SYSTEM_SCHEDULING : SYSTEM_BOOTING));
	events[count++] = event;
}
void rcu_scheduler_starting(void) { record(1); }
int fixture_kernel_init(void *argument) { CHECK(!argument); return 73; }
pid_t kernel_clone(struct kernel_clone_args *arguments)
{
	record(2);
	CHECK(arguments->flags == (CLONE_VM | CLONE_UNTRACED));
	CHECK(!arguments->pidfd && !arguments->child_tid && !arguments->parent_tid);
	CHECK(!arguments->name && !arguments->exit_signal);
	CHECK(!arguments->kthread && !arguments->io_thread && !arguments->user_worker);
	CHECK(!arguments->no_files && !arguments->umh);
	CHECK(!arguments->stack && !arguments->stack_size && !arguments->tls);
	CHECK(!arguments->set_tid && !arguments->set_tid_size && !arguments->cgroup);
	CHECK(!arguments->idle && !arguments->fn_arg && !arguments->cgrp);
	CHECK(!arguments->cset && !arguments->kill_seq);
	CHECK(arguments->fn && arguments->fn(NULL) == 73);
	return init_pid_result;
}
void rust_helper_rcu_read_lock(void)
{
	CHECK(!read_depth);
	read_depth++;
	record(lookup_count ? 9 : 3);
}
void rust_helper_rcu_read_unlock(void)
{
	CHECK(read_depth == 1);
	read_depth--;
	record(lookup_count == 1 ? 6 : 11);
}
struct task_struct *find_task_by_pid_ns(pid_t pid, struct pid_namespace *namespace)
{
	CHECK(read_depth == 1 && namespace == &init_pid_ns);
	CHECK(pid == (lookup_count ? thread_pid_result : init_pid_result));
	record(lookup_count ? 10 : 4);
	return lookup_count++ ? &kthreadd_fixture : &init_task_fixture;
}
int set_cpus_allowed_ptr(struct task_struct *task, const struct cpumask *mask)
{
	record(5);
	CHECK(read_depth == 1 && task == &init_task_fixture);
	CHECK(task->flags == (initial_flags | PF_NO_SETAFFINITY));
	CHECK(mask == cpumask_of(selected_cpu));
	for (unsigned int cpu = 0; cpu < NR_CPUS; cpu++)
		CHECK(!!(mask->bits[cpu / BITS_PER_LONG] & (1UL << (cpu % BITS_PER_LONG))) ==
		      (cpu == selected_cpu));
	return pin_result;
}
#ifdef CONFIG_NUMA
void numa_default_policy(void) { CHECK(!read_depth); record(7); }
#endif
int kthreadd(void *argument) { CHECK(!argument); return 91; }
pid_t kernel_thread(int (*function)(void *), void *argument, const char *name, unsigned long flags)
{
	record(8);
	CHECK(!read_depth && !argument && !name && flags == (CLONE_FS | CLONE_FILES));
	CHECK(function == kthreadd && function(argument) == 91);
	CHECK(lookup_count == 1);
	return thread_pid_result;
}
unsigned int debug_smp_processor_id(void);
unsigned int debug_smp_processor_id(void)
{
	CHECK(read_depth == 1 || count == 0);
	return selected_cpu;
}
void complete(struct completion *completion)
{
	record(12);
	CHECK(!read_depth && lookup_count == 2 && kthreadd_task == &kthreadd_fixture);
	CHECK(completion == expected_completion);
}
void schedule_preempt_disabled(void) { CHECK(!read_depth); record(13); }
void cpu_startup_entry(enum cpuhp_state state)
{
	record(14);
	CHECK(state == CPUHP_ONLINE && !read_depth);
	CHECK(count == (IS_ENABLED(CONFIG_NUMA) ? 14 : 13));
	for (unsigned int i = 0, expected = 1; i < count; i++, expected++) {
		if (!IS_ENABLED(CONFIG_NUMA) && expected == 7)
			expected++;
		CHECK(events[i] == expected);
	}
	puts("INIT_MAIN_REST_OK");
	exit(0);
}

int main(int argc, char **argv)
{
	CHECK(argc == 7);
	selected_cpu = strtoul(argv[2], NULL, 0);
	CHECK(selected_cpu < NR_CPUS);
	initial_flags = strtoul(argv[3], NULL, 0);
	pin_result = (int)strtoul(argv[4], NULL, 0);
	init_pid_result = (int)strtoul(argv[5], NULL, 0);
	thread_pid_result = (int)strtoul(argv[6], NULL, 0);
	init_task_fixture.flags = initial_flags;
	cpu_number = selected_cpu;
	/* Userspace x86 uses FS for TLS; make the isolated test's GS base explicit.
	 * The real Rust and original C stable per-CPU loads then read cpu_number. */
	CHECK(syscall(158, 0x1001, 0) == 0);
	CHECK(rust_boot_cpu() == selected_cpu);
	for (unsigned int cpu = 0; cpu < NR_CPUS; cpu++)
		CHECK(rust_boot_mask(cpu) == cpumask_of(cpu));
	if (!strcmp(argv[1], "original")) {
		expected_completion = original_completion();
		original_rest();
	}
	expected_completion = rust_completion();
	rust_rest();
}
