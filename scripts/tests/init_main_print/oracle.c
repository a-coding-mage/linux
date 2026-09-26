/* SPDX-License-Identifier: GPL-2.0-only */
#include "canonical.h"
#include "original.inc"

void original_print(const char *line)
{
	print_kernel_cmdline(line);
}

int original_argument_evaluation(int *value)
{
	return pr_notice("%s%s\n", "", ++*value ? "" : "");
}
