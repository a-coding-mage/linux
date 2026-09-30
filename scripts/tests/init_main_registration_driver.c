// SPDX-License-Identifier: GPL-2.0-only
/* Observe the registry service boundary; native callback prototypes, original
 * register_trace_* inlines and formatted WARN metadata remain unchanged. */
#include <linux/init.h>
#include <linux/tracepoint.h>
#include <linux/timekeeping.h>
#include <linux/string.h>
#include <trace/events/initcall.h>

extern int puts(const char *);
extern void abort(void);
void c_register(void);
void rust_register(void);
extern void (*c_start)(void *, initcall_t);
extern void (*c_finish)(void *, initcall_t, int);
extern void (*c_level)(void *, const char *);
extern void (*rust_start)(void *, initcall_t);
extern void (*rust_finish)(void *, initcall_t, int);
extern void (*rust_level)(void *, const char *);
struct task_struct *fixture_current(void);
struct task_struct *rust_helper_get_current(void);

struct tracepoint __tracepoint_initcall_start;
struct tracepoint __tracepoint_initcall_finish;
struct tracepoint __tracepoint_initcall_level;
struct static_call_key __SCK__WARN_trap;

static int results[3], calls, warnings, is_rust;
static void *first_data;

struct task_struct *fixture_current(void) { abort(); }
struct task_struct *rust_helper_get_current(void) { abort(); }
ktime_t ktime_get(void) { abort(); }

int tracepoint_probe_register(struct tracepoint *tp, void *probe, void *data)
{
	if (calls == 0) {
		if (tp != &__tracepoint_initcall_start ||
		    probe != (void *)(is_rust ? rust_start : c_start) || !data)
			abort();
		first_data = data;
	} else if (calls == 1) {
		if (tp != &__tracepoint_initcall_finish ||
		    probe != (void *)(is_rust ? rust_finish : c_finish) || data != first_data)
			abort();
	} else if (calls == 2) {
		if (tp != &__tracepoint_initcall_level ||
		    probe != (void *)(is_rust ? rust_level : c_level) || data)
			abort();
	} else {
		abort();
	}
	return results[calls++];
}

#ifdef CONFIG_BUG
void __SCT__WARN_trap(struct bug_entry *entry, ...)
{
	const char *format = (const char *)&entry->format_disp + entry->format_disp;

	if (strcmp(format, "Failed to register initcall tracepoints\n") ||
	    entry->flags != (BUGFLAG_WARNING | BUGFLAG_ARGS | BUGFLAG_TAINT(TAINT_WARN)))
		abort();
	warnings++;
}
#endif

int main(void)
{
	const int errors[] = { 0, 1, -1, -5, (-2147483647 - 1) };

	for (int a = 0; a < 5; ++a) {
		for (int b = 0; b < 5; ++b) {
			for (int c = 0; c < 5; ++c) {
				results[0] = errors[a]; results[1] = errors[b]; results[2] = errors[c];
				calls = warnings = is_rust = 0;
				c_register();
				if (calls != 3)
					abort();
				int expected = warnings;
				calls = warnings = 0; is_rust = 1;
				rust_register();
				if (calls != 3 || warnings != expected)
					abort();
			}
		}
	}
	puts("INIT_MAIN_REGISTER_OK cases=125");
	return 0;
}
