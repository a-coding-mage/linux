/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * The retained fdtget.c and fdtput.c predate util.h's length-output argument
 * and shared usage() macro. They were not originally Kbuild targets. Adapt
 * only those stale interfaces for the selectable C builds, without changing
 * the original applets, their option handling, or the shared utility library.
 */
#ifndef FDT_LEGACY_COMPAT_H
#define FDT_LEGACY_COMPAT_H

#include <stdio.h>
#include "util.h"
#undef usage
#define utilfdt_read(filename) utilfdt_read(filename, NULL)

#endif
