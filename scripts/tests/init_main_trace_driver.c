/* SPDX-License-Identifier: GPL-2.0-only */
/* Shared observations of time, current task and printk; no trace dispatcher. */
#include <linux/init.h>
#include <linux/sched.h>
#include <linux/ktime.h>
#include <linux/printk.h>
#include <linux/stdarg.h>

extern void exit(int) __attribute__((noreturn));
extern int puts(const char *);

extern void (*c_start)(void *, initcall_t), (*rust_start)(void *, initcall_t);
extern void (*c_finish)(void *, initcall_t, int), (*rust_finish)(void *, initcall_t, int);
extern void (*c_level)(void *, const char *), (*rust_level)(void *, const char *);
extern void c_fallback(bool, initcall_t, int, const char *, ktime_t *);
extern void rust_fallback(bool, initcall_t, int, const char *, ktime_t *);
struct task_struct *fixture_current(void);
struct task_struct *rust_helper_get_current(void);

static struct task_struct task;
static ktime_t tick, replacement;
static ktime_t *observed;
static bool mutate_on_clock;
static unsigned int used, cases;
struct event { unsigned int kind; const void *function; long long value, result; };
static struct event events[12], expected[12];

#define CHECK(condition) do { if (!(condition)) { puts(#condition); exit(__LINE__); } } while (0)

struct task_struct *fixture_current(void) { return &task; }
struct task_struct *rust_helper_get_current(void) { return &task; }

__attribute__((force_align_arg_pointer))
ktime_t ktime_get(void)
{
	CHECK(used < ARRAY_SIZE(events));
	events[used++] = (struct event){.kind = 1, .value = *observed};
	if (mutate_on_clock) *observed = replacement;
	return tick;
}

#ifdef CONFIG_PRINTK
__attribute__((force_align_arg_pointer))
int _printk(const char *format, ...)
{
	struct event event = {0};
	va_list args;
	va_start(args, format);
	if (!strcmp(format, "\0017calling  %pS @ %i\n")) {
		event.kind = 2;
		event.function = va_arg(args, void *);
		event.result = va_arg(args, int);
		event.value = *observed;
	} else if (!strcmp(format, "\0017initcall %pS returned %d after %lld usecs\n")) {
		event.kind = 3;
		event.function = va_arg(args, void *);
		event.result = va_arg(args, int);
		event.value = (long long)va_arg(args, unsigned long long);
	} else {
		CHECK(!strcmp(format, "\0017entering initcall level: %s\n"));
		CHECK(!strcmp(va_arg(args, const char *), "device-level"));
		event.kind = 4;
	}
	va_end(args);
	CHECK(used < ARRAY_SIZE(events));
	events[used++] = event;
	return -31;
}
#endif

static int init_function(void) { return 123; }

int main(void)
{
	const ktime_t times[] = {0, 1, -1, 999, -999, 1000, -1000, 1001, -1001,
		LLONG_MAX, LLONG_MIN, LLONG_MAX - 1000, LLONG_MIN + 1000};
	const int results[] = {0, 1, -1, INT_MIN, INT_MAX};
	for (unsigned int before = 0; before < ARRAY_SIZE(times); before++)
	for (unsigned int after = 0; after < ARRAY_SIZE(times); after++)
	for (unsigned int result = 0; result < ARRAY_SIZE(results); result++)
	for (unsigned int mode = 0; mode < 4; mode++) {
		unsigned int count = 0;
		ktime_t expected_time = 0;
		for (unsigned int owner = 0; owner < 2; owner++) {
			ktime_t calltime = times[before];
			void (*start)(void *, initcall_t) = owner ? rust_start : c_start;
			void (*finish)(void *, initcall_t, int) = owner ? rust_finish : c_finish;
			void (*level)(void *, const char *) = owner ? rust_level : c_level;
			void (*fallback)(bool, initcall_t, int, const char *, ktime_t *) = owner ? rust_fallback : c_fallback;
			/* The real do_one_initcall always invokes a nonnull function. */
			initcall_t function = init_function;
			task.pid = results[result];
			observed = &calltime;
			tick = times[after];
			replacement = times[before];
			mutate_on_clock = mode == 1;
			used = 0;
			memset(events, 0, sizeof(events));
			if (mode < 2) {
				start(&calltime, function);
				finish(&calltime, function, results[result]);
				level(NULL, "device-level");
			} else {
				fallback(mode == 3, function, results[result], "device-level", &calltime);
			}
			if (!owner) {
				count = used;
				expected_time = calltime;
				memcpy(expected, events, sizeof(events));
			} else {
				CHECK(count == used && expected_time == calltime);
				/* Padding bytes are not observations of kernel behavior. */
				for (unsigned int i = 0; i < used; i++) {
					CHECK(expected[i].kind == events[i].kind);
					CHECK(expected[i].function == events[i].function);
					CHECK(expected[i].value == events[i].value);
					CHECK(expected[i].result == events[i].result);
				}
			}
		}
		cases++;
	}
	CHECK(cases == 3380);
	puts("INIT_MAIN_TRACE_OK cases=3380");
	return 0;
}
