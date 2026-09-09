/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 */

// C dependency: <asm/ptrace.h>

/*
 * Signal context structure - contains all info to do with the state
 * before the signal handler was invoked.
 */
#[repr(C)]
pub struct sigcontext {
    pub regs: user_regs_struct,
    pub v2abi: user_regs_arcv2,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
