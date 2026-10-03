/* SPDX-License-Identifier: GPL-2.0 */
#include <generated/compile.h>
#include <generated/utsversion.h>
#include <generated/utsrelease.h>
// Cut here.
pub const LUPOS_BOOT_BUILD_STR: &[u8] = concat!(UTS_RELEASE, " (", LINUX_COMPILE_BY, "@", LINUX_COMPILE_HOST, ") (", LINUX_COMPILER, ") ", UTS_VERSION, "\0").as_bytes();
