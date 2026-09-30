/* SPDX-License-Identifier: GPL-2.0-only */
#include <linux/stackprotector.h>
#include <linux/topology.h>
#include <linux/string.h>
#include <linux/sched.h>
#include <asm/current.h>

extern void exit(int) __noreturn;
extern int puts(const char *);
extern long syscall(long, ...);
extern unsigned long long strtoull(const char *, char **, int);
extern void original_numa(void), rust_numa(void);
extern void original_canary(void) __noreturn;
extern void rust_canary(void) __noreturn;
extern void fixture_finish_canary(void) __noreturn;
extern struct task_struct *rust_helper_get_current(void);

#define CHECK(condition) do { if (!(condition)) exit(__LINE__); } while (0)
static struct task_struct fixture_task;
struct task_struct *current_task;
#ifdef CONFIG_STACKPROTECTOR
unsigned long __stack_chk_guard;
#endif
static u64 random_value;
static unsigned int random_calls;
u64 get_random_u64(void) { random_calls++; return random_value; }
struct task_struct *rust_helper_get_current(void) { return &fixture_task; }
void fixture_finish_canary(void)
{
#ifdef CONFIG_STACKPROTECTOR
	CHECK(random_calls == 1);
	CHECK(fixture_task.stack_canary == (random_value & CANARY_MASK));
	CHECK(__stack_chk_guard == fixture_task.stack_canary);
#else
	CHECK(!random_calls);
#endif
	puts("INIT_MAIN_CANARY_OK");
	exit(0);
}

#ifdef CONFIG_USE_PERCPU_NUMA_NODE_ID
int numa_node;
#ifdef CONFIG_NUMA
int x86_cpu_to_node_map;
#ifdef CONFIG_SMP
int *x86_cpu_to_node_map_early_ptr;
static int early_map[NR_CPUS];
#endif
#endif
struct cpumask __cpu_possible_mask;
#if NR_CPUS > 1 && !defined(CONFIG_FORCE_NR_CPUS)
unsigned int nr_cpu_ids;
#endif
#ifdef CONFIG_SMP
unsigned long __per_cpu_offset[NR_CPUS];
#endif
static struct { int space[32]; } areas[NR_CPUS], expected[NR_CPUS];

static void reset_nodes(unsigned int pattern, unsigned int limit, bool early)
{
	memset(areas, 0x5a, sizeof(areas));
	memset(&__cpu_possible_mask, 0, sizeof(__cpu_possible_mask));
#if NR_CPUS > 1 && !defined(CONFIG_FORCE_NR_CPUS)
	nr_cpu_ids = limit;
#endif
	for (unsigned int cpu = 0; cpu < NR_CPUS; cpu++) {
#ifdef CONFIG_SMP
		__per_cpu_offset[cpu] = (unsigned long)&areas[cpu].space[16] - (unsigned long)&numa_node;
#endif
		per_cpu(numa_node, cpu) = -1000 - cpu;
#ifdef CONFIG_NUMA
		/* Both original and translated early_per_cpu must use the same real
		 * per-CPU offset for the two separate canonical variables. */
		long delta = (long)&x86_cpu_to_node_map - (long)&numa_node;
		CHECK(delta >= -32 && delta <= 32);
		per_cpu(x86_cpu_to_node_map, cpu) = 17 + 3 * cpu;
#ifdef CONFIG_SMP
		early_map[cpu] = 100 + cpu;
#endif
#endif
		if (pattern == 0 || (pattern == 1 && cpu % 3 == 1) ||
		    (pattern == 2 && (cpu == 0 || cpu == 63 || cpu == 64 || cpu == 129)))
			__cpu_possible_mask.bits[cpu / BITS_PER_LONG] |= 1UL << (cpu % BITS_PER_LONG);
	}
#if defined(CONFIG_NUMA) && defined(CONFIG_SMP)
	x86_cpu_to_node_map_early_ptr = early ? early_map : NULL;
#endif
}
#endif

int main(int argc, char **argv)
{
	CHECK(syscall(158, 0x1001, 0) == 0);
	current_task = &fixture_task;
	if (argc == 3) {
		random_value = strtoull(argv[2], NULL, 0);
		if (!strcmp(argv[1], "original"))
			original_canary();
		rust_canary();
	}
	CHECK(argc == 1);
#ifdef CONFIG_USE_PERCPU_NUMA_NODE_ID
	for (unsigned int pattern = 0; pattern < 4; pattern++) {
		for (unsigned int limit = 1; limit <= NR_CPUS; limit++) {
			for (unsigned int early = 0; early < 2; early++) {
				reset_nodes(pattern, limit, early);
				original_numa();
				memcpy(expected, areas, sizeof(expected));
				reset_nodes(pattern, limit, early);
				rust_numa();
				CHECK(!memcmp(expected, areas, sizeof(expected)));
			}
		}
	}
#else
	original_numa();
	rust_numa();
#endif
	puts("INIT_MAIN_EARLY_NUMA_OK");
	return 0;
}
