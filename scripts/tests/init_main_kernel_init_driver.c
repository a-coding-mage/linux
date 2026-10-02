/* SPDX-License-Identifier: GPL-2.0-only */
/* Observe boot lifecycle boundaries; do not free memory or change host mappings. */
#include <linux/async.h>
#include <linux/completion.h>
#include <linux/fs_struct.h>
#include <linux/ftrace.h>
#include <linux/kgdb.h>
#include <linux/kprobes.h>
#include <linux/mempolicy.h>
#include <linux/mm.h>
#include <linux/pti.h>
#include <linux/rcupdate.h>
#include <linux/string.h>
#include "init_main_fortify_observer.h"
#include <linux/sysctl.h>

extern void exit(int) __attribute__((noreturn));
extern int puts(const char *);
extern int original_kernel_init(void *);
extern int rust_kernel_init(void *);
extern void fixture_freeable(void);
extern void fixture_exit_boot_config(void);
extern void fixture_mark_readonly(void);
extern int fixture_execute(void);

enum system_states system_state;
DECLARE_COMPLETION(kthreadd_done);
static unsigned int events[32], count;

#define CHECK(condition) do { if (!(condition)) exit(__LINE__); } while (0)

static void record(unsigned int event)
{
	CHECK(count < ARRAY_SIZE(events));
	events[count++] = event * 16 + system_state;
}

void init_userspace_fs(void) { record(1); }
void wait_for_completion(struct completion *completion)
{
	CHECK(completion == &kthreadd_done);
	record(2);
}
void fixture_freeable(void) { record(3); }
void async_synchronize_full(void) { record(4); }
#ifdef CONFIG_KPROBES
void kprobe_free_init_mem(void) { record(5); }
#endif
#if defined(CONFIG_FUNCTION_TRACER) && defined(CONFIG_DYNAMIC_FTRACE)
void ftrace_free_init_mem(void) { record(6); }
#endif
#ifdef CONFIG_TRACER_SNAPSHOT
void ftrace_boot_snapshot(void) { record(7); }
#endif
#ifdef CONFIG_KGDB
void kgdb_free_init_mem(void) { record(8); }
#endif
void fixture_exit_boot_config(void) { record(9); }
void free_initmem(void) { record(10); }
void fixture_mark_readonly(void) { record(11); }
#ifdef CONFIG_MITIGATION_PAGE_TABLE_ISOLATION
void pti_finalize(void) { record(12); }
#endif
#ifdef CONFIG_NUMA
void numa_default_policy(void) { record(13); }
#endif
#ifdef CONFIG_TREE_RCU
void rcu_end_inkernel_boot(void) { record(14); }
#endif
#ifdef CONFIG_SYSCTL
void do_sysctl_args(void) { record(15); }
#endif
int fixture_execute(void) { record(16); return 0; }

int main(void)
{
	int (*callbacks[])(void *) = { original_kernel_init, rust_kernel_init };
	unsigned int expected[32], expected_count = 0;
	for (unsigned int owner = 0; owner < ARRAY_SIZE(callbacks); owner++) {
		count = 0;
		system_state = SYSTEM_SCHEDULING;
		CHECK(callbacks[owner]((void *)0x123) == 0);
		CHECK(system_state == SYSTEM_RUNNING);
		CHECK(events[0] == 16 + SYSTEM_SCHEDULING);
		CHECK(events[count - 1] == 16 * 16 + SYSTEM_RUNNING);
		for (unsigned int i = 1; i < count; i++) {
			unsigned int event = events[i] / 16;
			CHECK(events[i] % 16 == (event <= 4 ? SYSTEM_SCHEDULING :
				(event <= 12 ? SYSTEM_FREEING_INITMEM : SYSTEM_RUNNING)));
			CHECK(event > events[i - 1] / 16);
		}
		if (!owner) {
			expected_count = count;
			memcpy(expected, events, count * sizeof(events[0]));
		} else {
			CHECK(count == expected_count);
			CHECK(!memcmp(expected, events, count * sizeof(events[0])));
		}
	}
	puts("INIT_MAIN_KERNEL_INIT_OK");
	return 0;
}
