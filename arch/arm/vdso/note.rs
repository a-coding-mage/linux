// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2012-2018 ARM Limited
 *
 * This supplies .note.* sections to go into the PT_NOTE inside the vDSO text.
 * Here we can supply some information useful to userland.
 */

// C dependencies: <linux/uts.h>, <linux/version.h>, <linux/elfnote.h>,
// and <linux/build-salt.h>.

// ELFNOTE32("Linux", 0, LINUX_VERSION_CODE);
// The ELFNOTE32 macro is supplied by linux/elfnote.h and emits the Linux
// version ELF note using LINUX_VERSION_CODE from linux/version.h.

// BUILD_SALT;
// The BUILD_SALT macro is supplied by linux/build-salt.h and emits the build
// salt ELF note.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
