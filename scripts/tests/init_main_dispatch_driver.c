// SPDX-License-Identifier: GPL-2.0-only
/* Run the unchanged shared trace iterator/static-call/SRCU machinery. The
 * host's GS base is verified before using its native per-CPU increments.
 * RCU-watching and warning-report services are observation boundaries. */
#include <linux/init.h>
#include <linux/tracepoint.h>
#include <linux/timekeeping.h>
#include <linux/string.h>
#include <linux/cpumask.h>
#include <trace/events/initcall.h>
#include <asm/prctl.h>
#include <asm/unistd.h>

extern long syscall(long, ...);
extern int puts(const char *);
extern void exit(int) __attribute__((noreturn));
extern void abort(void);
void c_dispatch(initcall_t, int, const char *);
void rust_dispatch(initcall_t, int, const char *);
struct task_struct *rust_helper_get_current(void);

#define CHECK(condition) do { if (!(condition)) { puts(#condition); exit(__LINE__); } } while (0)

static struct srcu_ctr counters;
struct srcu_struct tracepoint_srcu = { .srcu_ctrp = &counters };
DEFINE_PER_CPU_CACHE_HOT(int, cpu_number);
struct cpumask __cpu_online_mask;
struct static_call_key __SCK__WARN_trap;
static struct tracepoint *points[] = { &__tracepoint_initcall_start, &__tracepoint_initcall_finish, &__tracepoint_initcall_level };
struct event { int kind, data; long locks, unlocks; };
static struct event events[32], expected[32];
static int used;
static bool watching;

static void record(int kind, int data)
{
	CHECK(used < ARRAY_SIZE(events));
	events[used++] = (struct event){ kind, data, counters.srcu_locks.counter, counters.srcu_unlocks.counter };
}

__attribute__((force_align_arg_pointer))
bool rcu_is_watching(void)
{
	CHECK(counters.srcu_locks.counter == counters.srcu_unlocks.counter);
	record(10, watching);
	return watching;
}

__attribute__((force_align_arg_pointer))
void __SCT__WARN_trap(struct bug_entry *entry, ...)
{
	const char *format = (const char *)&entry->format_disp + entry->format_disp;
	CHECK(!strcmp(format, "RCU not watching for tracepoint"));
	CHECK(entry->flags == (BUGFLAG_WARNING | BUGFLAG_ARGS | BUGFLAG_ONCE | BUGFLAG_TAINT(TAINT_WARN)));
	record(11, 0);
}

/* Kept references in the Rust .init.text section may include registration;
 * none of these services is part of the dispatch path under observation. */
int tracepoint_probe_register(struct tracepoint *point, void *probe, void *data) { abort(); }
struct task_struct *rust_helper_get_current(void) { abort(); }
ktime_t ktime_get(void) { abort(); }

static int init_function(void) { return 7; }

__attribute__((force_align_arg_pointer))
static void start(void *data, initcall_t function)
{
	CHECK(function == init_function);
	CHECK(counters.srcu_locks.counter == counters.srcu_unlocks.counter + 1);
	record(1, (long)data);
}

__attribute__((force_align_arg_pointer))
static void finish(void *data, initcall_t function, int result)
{
	CHECK(function == init_function && result == -17);
	CHECK(counters.srcu_locks.counter == counters.srcu_unlocks.counter + 1);
	record(2, (long)data);
}

__attribute__((force_align_arg_pointer))
static void level(void *data, const char *name)
{
	CHECK(!strcmp(name, "device"));
	CHECK(counters.srcu_locks.counter == counters.srcu_unlocks.counter + 1);
	record(3, (long)data);
}

int main(void)
{
	struct tracepoint_func probes[3][3] = {
		{ {start, (void *)10}, {start, (void *)11}, {NULL} },
		{ {finish, (void *)20}, {finish, (void *)21}, {NULL} },
		{ {level, (void *)30}, {level, (void *)31}, {NULL} },
	};
	unsigned long gs;
	int cases = 0;
	CHECK(syscall(__NR_arch_prctl, ARCH_GET_GS, &gs) == 0 && gs == 0);
	for (int mode = 0; mode < 64; ++mode)
	for (int rcu = 0; rcu < 2; ++rcu)
	for (int online = 0; online < 2; ++online) {
		int expected_used = 0;
		long expected_locks = 0, expected_unlocks = 0;
		watching = rcu;
		__cpu_online_mask.bits[0] = online;
		for (int owner = 0; owner < 2; ++owner) {
			for (int i = 0; i < 3; ++i) {
				int state = (mode >> (2*i)) & 3;
				points[i]->key.key.enabled.counter = state != 0;
				points[i]->funcs = state == 1 ? NULL : probes[i];
				probes[i][1].func = state == 2 ? NULL : probes[i][0].func;
			}
			used = 0;
			counters.srcu_locks.counter = counters.srcu_unlocks.counter = 0;
			if (owner) rust_dispatch(init_function, -17, "device");
			else c_dispatch(init_function, -17, "device");
			CHECK(counters.srcu_locks.counter == counters.srcu_unlocks.counter);
			int expected_events = 0, enabled = 0;
			if (online) {
				for (int i = 0; i < 3; ++i) {
					int state = (mode >> (2*i)) & 3;
					enabled += state != 0;
					expected_events += state >= 2 ? state - 1 : 0;
				}
#ifdef CONFIG_LOCKDEP
				expected_events += watching ? 3 : 6;
#endif
			}
			CHECK(used == expected_events && counters.srcu_locks.counter == enabled);
			if (!owner) {
				expected_used = used;
				expected_locks = counters.srcu_locks.counter;
				expected_unlocks = counters.srcu_unlocks.counter;
				memcpy(expected, events, sizeof(expected));
			} else {
				CHECK(used == expected_used);
				CHECK(counters.srcu_locks.counter == expected_locks && counters.srcu_unlocks.counter == expected_unlocks);
				for (int i = 0; i < used; ++i) {
					CHECK(events[i].kind == expected[i].kind && events[i].data == expected[i].data);
					CHECK(events[i].locks == expected[i].locks && events[i].unlocks == expected[i].unlocks);
				}
			}
		}
		cases++;
	}
	CHECK(cases == 256);
	puts("INIT_MAIN_DISPATCH_OK cases=256");
	return 0;
}
