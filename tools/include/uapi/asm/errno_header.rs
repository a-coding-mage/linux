/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C header dependency selection:
 *
 * If defined(__i386__) || defined(__x86_64__):
 *   include "../../../arch/x86/include/uapi/asm/errno.h"
 * Else if defined(__powerpc__):
 *   include "../../../arch/powerpc/include/uapi/asm/errno.h"
 * Else if defined(__sparc__):
 *   include "../../../arch/sparc/include/uapi/asm/errno.h"
 * Else if defined(__alpha__):
 *   include "../../../arch/alpha/include/uapi/asm/errno.h"
 * Else if defined(__mips__):
 *   include "../../../arch/mips/include/uapi/asm/errno.h"
 * Else if defined(__hppa__):
 *   include "../../../arch/parisc/include/uapi/asm/errno.h"
 * Else:
 *   include <asm-generic/errno.h>
 */
