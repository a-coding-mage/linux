/* SPDX-License-Identifier: GPL-2.0-only */
/* The included declarations and four bodies are extracted unchanged from main.c. */
#define boot_command_line original_boot_command_line
#define saved_command_line original_saved_command_line
#define saved_command_line_len original_saved_command_line_len
#define cmdline_has_extra_options original_cmdline_has_extra_options
#include "canonical.h"
#include <linux/ctype.h>
#include "original.inc"

void original_prepare(char *boot, char *extra, char *args, size_t offset)
{
	strcpy(boot_command_line, boot);
	extra_command_line = extra;
	extra_init_args = args;
#ifdef CONFIG_BOOT_CONFIG
	initargs_offs = offset;
#endif
}

void original_setup(char *arch)
{
	setup_command_line(arch);
}

char *original_saved(void) { return saved_command_line; }
char *original_static(void) { return static_command_line; }
char *original_extra(void) { return extra_init_args; }
unsigned int original_length(void) { return saved_command_line_len; }

int original_init(char *value, bool ramdisk)
{
	unsigned int i;

	for (i = 0; i < ARRAY_SIZE(argv_init); i++)
		argv_init[i] = value;
	execute_command = NULL;
	ramdisk_execute_command = "/init";
	ramdisk_execute_command_set = false;
	return ramdisk ? rdinit_setup(value) : init_setup(value);
}

const char *original_arg(unsigned int index) { return argv_init[index]; }
char *original_execute(void) { return execute_command; }
char *original_rdinit(void) { return ramdisk_execute_command; }
bool original_rdinit_set(void) { return ramdisk_execute_command_set; }
