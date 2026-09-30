/* SPDX-License-Identifier: GPL-2.0-only */
/* Canonical version data, generated separately for each UTS_VERSION stage. */
#ifndef _RUST_BINDINGS_INIT_VERSION_H
#define _RUST_BINDINGS_INIT_VERSION_H

#ifdef _LINUX_INIT_H
#error "init_version.h must precede linux/init.h"
#endif
#pragma push_macro("MODULE")
#undef MODULE
#include <generated/compile.h>
#include <generated/utsrelease.h>
/*
 * libclang describes the MS anonymous tagged member in ns_common, but
 * bindgen 0.71 drops it. Give the very same union arm a second, named view;
 * retain the original anonymous arm and all promoted canonical fields.
 * The completed tag is included first so its definition is not rewritten.
 * Native tests compare the entire object and every relocation with C.
 */
#include <linux/ns/nstree_types.h>
#define ns_tree ns_tree; struct ns_tree rust_ns_tree
#include <linux/ns/ns_common_types.h>
#undef ns_tree
#include <linux/init.h>
#include <linux/printk.h>
#include <linux/proc_ns.h>
#include <linux/string.h>
#include <linux/uts.h>
#include <linux/utsname.h>

#define RUST_VERSION_SYSNAME UTS_SYSNAME
#define RUST_VERSION_NODENAME UTS_NODENAME
#define RUST_VERSION_RELEASE UTS_RELEASE
#define RUST_VERSION_VERSION UTS_VERSION
#define RUST_VERSION_MACHINE UTS_MACHINE
#define RUST_VERSION_DOMAINNAME UTS_DOMAINNAME
#define RUST_VERSION_BUILD_SALT CONFIG_BUILD_SALT
#define RUST_VERSION_PROC_BANNER "%s version %s (" LINUX_COMPILE_BY "@" \
	LINUX_COMPILE_HOST ") (" LINUX_COMPILER ") %s\n"
#define RUST_VERSION_BANNER "Linux version " UTS_RELEASE " (" LINUX_COMPILE_BY \
	"@" LINUX_COMPILE_HOST ") (" LINUX_COMPILER ") " UTS_VERSION "\n"
/* Record whether linux/string.h actually selected its fortified wrappers. */
#ifdef _LINUX_FORTIFY_STRING_H_
#define RUST_VERSION_FORTIFY 1
#else
#define RUST_VERSION_FORTIFY 0
#endif
enum {
	RUST_VERSION_NODENAME_SIZE = sizeof(init_uts_ns.name.nodename),
	RUST_VERSION_NS_TYPE = ns_common_type(&init_uts_ns),
	RUST_VERSION_NS_ID = ns_init_id(&init_uts_ns),
	RUST_VERSION_NS_INUM = ns_init_inum(&init_uts_ns),
};
#pragma pop_macro("MODULE")
#endif
