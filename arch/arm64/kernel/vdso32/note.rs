// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2012-2018 ARM Limited
 *
 * This supplies .note.* sections to go into the PT_NOTE inside the vDSO text.
 * Here we can supply some information useful to userland.
 */

// Dependencies supplied by the Linux headers in the original source:
// linux/uts.h, linux/version.h, linux/elfnote.h, linux/build-salt.h

ELFNOTE32!("Linux", 0, LINUX_VERSION_CODE);
BUILD_SALT!();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
