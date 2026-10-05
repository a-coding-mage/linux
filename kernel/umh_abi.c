// SPDX-License-Identifier: GPL-2.0-only
#error "SOURCE ONLY HOLD: usermode-helper callback ABI is unqualified"
/*
 * Inactive signature-only entry adapter. The public native callback signature
 * remains exactly include/linux/umh.h; all setup policy remains in umh.rs.
 */
#include "umh_native_bindings.h"

struct subprocess_info *call_usermodehelper_setup(const char *path, char **argv,
		char **envp, gfp_t gfp_mask,
		int (*init)(struct subprocess_info *info, struct cred *new),
		void (*cleanup)(struct subprocess_info *info), void *data)
{
	const struct lupos_umh_callbacks callbacks = {
		.init = init,
		.cleanup = cleanup,
	};

	return lupos_umh_setup_body(path, argv, envp, gfp_mask, &callbacks, data);
}
