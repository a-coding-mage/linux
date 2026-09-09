/* SPDX-License-Identifier: GPL-2.0-only */

// This header is guarded by __LINUX_COMPILER_VERSION_H in the C source.
// The build system must not include <linux/compiler-version.h> directly.

/*
 * This header exists to force full rebuild when the compiler is upgraded.
 *
 * When fixdep scans this, it will find this string "CONFIG_CC_VERSION_TEXT"
 * and add dependency on include/config/CC_VERSION_TEXT, which is touched
 * by Kconfig when the version string from the compiler changes.
 */

/* Additional tree-wide dependencies start here. */

/*
 * If any of the GCC plugins change, we need to rebuild everything that
 * was built with them, as they may have changed their behavior and those
 * behaviors may need to be synchronized across all translation units.
 */
// C build-time condition: when GCC_PLUGINS is defined, include the generated
// GCC plugin dependency header.

/*
 * If the randstruct seed itself changes (whether for GCC plugins or
 * Clang), the entire tree needs to be rebuilt since the randomization of
 * structures may change between compilation units if not.
 */
// C build-time condition: when RANDSTRUCT is defined, include the generated
// randstruct hash dependency header.

/*
 * If any external changes affect Clang's integer wrapping sanitizer
 * behavior, a full rebuild is needed as the coverage for wrapping types
 * may have changed, which may impact the expected behaviors that should
 * not differ between compilation units.
 */
// C build-time condition: when INTEGER_WRAP is defined, include the generated
// integer-wrap dependency header.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
