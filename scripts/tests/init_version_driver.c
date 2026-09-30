/* SPDX-License-Identifier: GPL-2.0-only */
/* Execute both real hostname callbacks with the original string primitive. */
#include <generated/compile.h>
#include <linux/init.h>
#include <linux/printk.h>
#include <linux/proc_ns.h>
#include <linux/stdarg.h>
#include <linux/string.h>
#include <linux/user_namespace.h>
#include <linux/utsname.h>

extern int printf(const char *, ...);
extern int vsnprintf(char *, size_t, const char *, va_list);
extern void exit(int) __attribute__((noreturn));
extern const struct obs_kernel_param rust_hostname_parameter;
struct user_namespace init_user_ns;
const struct proc_ns_operations utsns_operations;
extern struct uts_namespace original_uts_ns;

#define init_uts_ns original_uts_ns
#define linux_banner original_linux_banner
#define linux_proc_banner original_linux_proc_banner
#include "../../init/version.c"
#undef init_uts_ns
#undef linux_banner
#undef linux_proc_banner

static char log[256];
static unsigned int warnings;
#define CHECK(test) do { if (!(test)) { printf("FAIL %d\n", __LINE__); exit(90); } } while (0)

#ifdef _LINUX_FORTIFY_STRING_H_
/* Valid bounded strings must never reach the original wrapper's panic path. */
void __fortify_panic(const u8 reason, const size_t avail, const size_t size)
{
	printf("unexpected fortify panic %u %zu %zu\n", reason, avail, size);
	exit(91);
}
#endif

#ifdef CONFIG_PRINTK
__attribute__((force_align_arg_pointer))
int _printk(const char *format, ...)
{
	va_list args;
	va_start(args, format);
	int length = vsnprintf(log, sizeof(log), format, args);
	va_end(args);
	warnings++;
	CHECK(length >= 0 && length < sizeof(log));
	return length;
}
#endif

static void compare(char *value)
{
	char expected[sizeof(log)], expected_name[sizeof(init_uts_ns.name.nodename)];
	unsigned int expected_warnings;
	int (*volatile call)(char *) = early_hostname;
	memset(original_uts_ns.name.nodename, 0xa5, sizeof(expected_name));
	memset(init_uts_ns.name.nodename, 0xa5, sizeof(expected_name));
	memset(log, 0, sizeof(log)); warnings = 0;
	int result = call(value);
	memcpy(expected_name, original_uts_ns.name.nodename, sizeof(expected_name));
	memcpy(expected, log, sizeof(log)); expected_warnings = warnings;
	memset(log, 0, sizeof(log)); warnings = 0;
	call = rust_hostname_parameter.setup_func;
	CHECK(call(value) == result);
	CHECK(result == 0);
	CHECK(!memcmp(init_uts_ns.name.nodename, expected_name, sizeof(expected_name)));
	CHECK(!memcmp(log, expected, sizeof(log)));
	CHECK(warnings == expected_warnings);
}

int main(void)
{
	char source[528];
	unsigned int cases = 0;
	CHECK(rust_hostname_parameter.early == 1);
	CHECK(!strcmp(rust_hostname_parameter.str, "hostname"));
	for (unsigned int offset = 0; offset < 16; offset++) {
		for (unsigned int length = 0; length <= 256; length++) {
			memset(source, 0x7e, sizeof(source));
			for (unsigned int i = 0; i < length; i++)
				source[offset + i] = (char)(1 + (i * 61 + offset) % 255);
			source[offset + length] = 0;
			compare(source + offset);
			cases++;
		}
	}
	printf("INIT_VERSION_HOSTNAME_OK cases=%u\n", cases);
	return 0;
}
