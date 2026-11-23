// SPDX-License-Identifier: GPL-2.0
/*
 * Native records for the Rust personality owner: the SYSCALL_DEFINE1()
 * entry wrapper with its configured ABI and metadata, and the module_init()
 * record of the /proc/execdomains initializer. Both bodies live in
 * kernel/exec_domain.rs.
 */
#include <linux/init.h>
#include <linux/module.h>
#include <linux/personality.h>
#include <linux/syscalls.h>

unsigned int lupos_sys_personality(unsigned int personality);

#ifdef CONFIG_PROC_FS
int __init proc_execdomains_init(void);
module_init(proc_execdomains_init);
#endif

SYSCALL_DEFINE1(personality, unsigned int, personality)
{
	return lupos_sys_personality(personality);
}
