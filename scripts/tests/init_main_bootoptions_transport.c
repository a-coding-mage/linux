/* SPDX-License-Identifier: GPL-2.0-only */
/* Only external alias, diagnostic and fatal services are instrumented. */
#include <linux/init.h>
#include <linux/moduleparam.h>
#include <linux/printk.h>
#include <linux/string.h>
#include <linux/sysctl.h>

extern void bootoption_event(unsigned int kind, const char *value);
extern __noreturn void bootoption_abort(void);

__noreturn void rust_helper_BUG(void) { bootoption_abort(); }

int _printk(const char *format, ...)
{
	__builtin_va_list arguments;

	__builtin_va_start(arguments, format);
	bootoption_event(10, format);
	bootoption_event(11, __builtin_va_arg(arguments, const char *));
	__builtin_va_end(arguments);
	return 0;
}

#ifdef CONFIG_SYSCTL
bool sysctl_is_alias(char *parameter)
{
	bootoption_event(20, parameter);
	return !strncmp(parameter, "alias", 5);
}
#endif

static int __init callback_zero(char *value)
{
	bootoption_event(30, value);
	return 0;
}

static int __init callback_one(char *value)
{
	bootoption_event(31, value);
	return 1;
}

__setup_param("early", early_record, callback_one, 1);
__setup_param("early", early_continuation, callback_zero, 0);
__setup_param("chain=", chain_first, callback_zero, 0);
__setup_param("chain=", chain_second, callback_one, 0);
__setup_param("obsolete", obsolete_record, NULL, 0);
__setup_param("dash-key=", dash_record, callback_one, 0);
__setup_param("plain=", plain_record, callback_zero, 0);
