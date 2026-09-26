/* SPDX-License-Identifier: GPL-2.0-only */
#include "canonical.h"

initcall_entry_t __initcall0_start[0], __initcall1_start[0], __initcall2_start[0];
initcall_entry_t __initcall3_start[0], __initcall4_start[0], __initcall5_start[0];
initcall_entry_t __initcall6_start[0], __initcall7_start[0], __initcall_end[0];

/* Capture only the formatter boundary; the parameter operations and parser
 * below are the original C bodies and execute through each real descriptor. */
int sprintf(char *buffer, const char *format, ...)
{
	va_list args;
	if (strcmp(format, "%c\n"))
		__builtin_trap();
	va_start(args, format);
	buffer[0] = va_arg(args, int);
	va_end(args);
	buffer[1] = '\n';
	buffer[2] = 0;
	return 2;
}
#include "bool.inc"

static bool original_debug;
core_param(initcall_debug, original_debug, bool, 0644);
const struct kernel_param *original_parameter(void) { return &__param_initcall_debug; }
void original_reset(bool value) { original_debug = value; }
bool original_value(void) { return original_debug; }
