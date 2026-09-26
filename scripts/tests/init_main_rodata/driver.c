/* SPDX-License-Identifier: GPL-2.0-only */
/* Observe the original subsystem calls, not physical page protection. */
#include <linux/init.h>
#include <linux/string.h>
#include <linux/stdarg.h>

extern int printf(const char *, ...);
extern int vsnprintf(char *, size_t, const char *, va_list);
extern void exit(int) __attribute__((noreturn));
extern int original_set(char *), rust_set(char *);
extern void original_mark(void), rust_mark(void);
#if defined(CONFIG_STRICT_KERNEL_RWX) || defined(CONFIG_STRICT_MODULE_RWX)
extern bool rodata_enabled, original_rodata_enabled;
#endif
bool rodata_full, original_rodata_full;
#define BOUNDARY(level) initcall_entry_t __initcall##level##_start[0]
BOUNDARY(0); BOUNDARY(1); BOUNDARY(2); BOUNDARY(3);
BOUNDARY(4); BOUNDARY(5); BOUNDARY(6); BOUNDARY(7);
initcall_entry_t __initcall_end[0];

static char log[1024];
static size_t used;
#define CHECK(condition) do { if (!(condition)) { printf("FAIL %d\n", __LINE__); exit(90); } } while (0)
static void event(char value) { CHECK(used + 1 < sizeof(log)); log[used++] = value; log[used] = 0; }
void flush_module_init_free_work(void) { event('F'); }
void jump_label_init_ro(void) { event('J'); }
void mark_rodata_ro(void) { event('M'); }
bool ptdump_check_wx(void) { event('W'); return true; }
void rodata_test(void) { event('T'); }

__attribute__((force_align_arg_pointer))
int _printk(const char *format, ...)
{
	va_list arguments;
	va_start(arguments, format);
	int length = vsnprintf(log + used, sizeof(log) - used, format, arguments);
	va_end(arguments);
	CHECK(length >= 0 && length < sizeof(log) - used);
	used += length;
	return length;
}

static void compare(char *value, bool enabled, bool full)
{
	char expected[sizeof(log)];
	int (*volatile call)(char *) = original_set;
	void (*volatile mark)(void) = original_mark;
	bool expected_enabled = false;
#if defined(CONFIG_STRICT_KERNEL_RWX) || defined(CONFIG_STRICT_MODULE_RWX)
	rodata_enabled = original_rodata_enabled = enabled;
#endif
	rodata_full = original_rodata_full = full;
	used = 0; log[0] = 0;
	int result = call(value);
	mark();
	memcpy(expected, log, sizeof(expected));
#if defined(CONFIG_STRICT_KERNEL_RWX) || defined(CONFIG_STRICT_MODULE_RWX)
	expected_enabled = original_rodata_enabled;
#endif
	call = rust_set;
	mark = rust_mark;
	used = 0; log[0] = 0;
	CHECK(call(value) == result);
	mark();
	CHECK(!strcmp(log, expected));
#if defined(CONFIG_STRICT_KERNEL_RWX) || defined(CONFIG_STRICT_MODULE_RWX)
	CHECK(rodata_enabled == expected_enabled);
#endif
	CHECK(rodata_full == original_rodata_full);
}

int main(void)
{
	char *values[] = { NULL, "", "on", "off", "noalias", "full", "ON", "OFF",
		" on", "on ", "off\t", "noalias\n", "on=1" };
	unsigned int cases = 0;
	for (unsigned int enabled = 0; enabled < 2; enabled++) {
		for (unsigned int full = 0; full < 2; full++) {
			for (unsigned int i = 0; i < sizeof(values) / sizeof(values[0]); i++) {
				compare(values[i], enabled, full); cases++;
			}
			for (unsigned int byte = 1; byte < 256; byte++) {
				char value[] = { byte, 0 };
				compare(value, enabled, full); cases++;
			}
		}
	}
	printf("INIT_MAIN_RODATA_OK cases=%u\n", cases);
	return 0;
}
