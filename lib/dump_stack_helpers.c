// SPDX-License-Identifier: GPL-2.0
/*
 * Native leaves for the Rust dump_stack() owner: the CPU number, the real
 * credential's euid, the configured kexec/stop-machine/sched_ext
 * reporters, the init utsname strings and the IRQ-state primitives of
 * printk_cpu_sync_get_irqsave(). The report sequence lives in
 * lib/dump_stack.rs.
 */
#include <linux/cred.h>
#include <linux/irqflags.h>
#include <linux/kexec.h>
#include <linux/printk.h>
#include <linux/sched.h>
#include <linux/sched/ext.h>
#include <linux/smp.h>
#include <linux/stop_machine.h>
#include <linux/utsname.h>

int lupos_dump_stack_cpu(void);
unsigned int lupos_dump_stack_current_euid(void);
bool lupos_dump_stack_kexec_crash_loaded(void);
const char *lupos_dump_stack_uts_release(void);
const char *lupos_dump_stack_uts_version(void);
void lupos_dump_stack_print_stop_info(const char *log_lvl, struct task_struct *task);
void lupos_dump_stack_print_scx_info(const char *log_lvl, struct task_struct *task);
unsigned long lupos_dump_stack_irq_save(void);
void lupos_dump_stack_irq_restore(unsigned long flags);

int lupos_dump_stack_cpu(void)
{
	return raw_smp_processor_id();
}

unsigned int lupos_dump_stack_current_euid(void)
{
	return __kuid_val(current_real_cred()->euid);
}

bool lupos_dump_stack_kexec_crash_loaded(void)
{
	return kexec_crash_loaded();
}

const char *lupos_dump_stack_uts_release(void)
{
	return init_utsname()->release;
}

const char *lupos_dump_stack_uts_version(void)
{
	return init_utsname()->version;
}

void lupos_dump_stack_print_stop_info(const char *log_lvl, struct task_struct *task)
{
	print_stop_info(log_lvl, task);
}

void lupos_dump_stack_print_scx_info(const char *log_lvl, struct task_struct *task)
{
	print_scx_info(log_lvl, task);
}

unsigned long lupos_dump_stack_irq_save(void)
{
	unsigned long flags;

	local_irq_save(flags);
	return flags;
}

void lupos_dump_stack_irq_restore(unsigned long flags)
{
	local_irq_restore(flags);
}
