/* SPDX-License-Identifier: GPL-2.0-only */
#define boot_command_line original_boot_command_line
#define saved_command_line original_saved_command_line
#define saved_command_line_len original_saved_command_line_len
#include "canonical.h"
#include <linux/unaligned.h>
#include "original.inc"

void original_setup(const char *command)
{
	strcpy(boot_command_line, command);
	setup_boot_config();
}
char *original_extra(void) { return extra_command_line; }
char *original_args(void) { return extra_init_args; }
size_t original_offset(void) { return initargs_offs; }
bool original_found(void) { return bootconfig_found; }
void *original_trailer(size_t *size) { return get_boot_config_from_initrd(size); }
void original_exit(void) { exit_boot_config(); }
int original_warn(void) { return warn_bootconfig(NULL); }
