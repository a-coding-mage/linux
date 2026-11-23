// SPDX-License-Identifier: GPL-2.0-only
/*
 * Native leaf for the Rust kallsyms_show_value() owner: the configured
 * security_capable() check for CAP_SYSLOG without auditing.
 */
#include <linux/build_bug.h>
#include <linux/capability.h>
#include <linux/cred.h>
#include <linux/security.h>
#include <linux/user_namespace.h>

bool lupos_ksyms_capable_syslog_noaudit(const struct cred *cred);

bool lupos_ksyms_capable_syslog_noaudit(const struct cred *cred)
{
	return security_capable(cred, &init_user_ns, CAP_SYSLOG,
				CAP_OPT_NOAUDIT) == 0;
}
