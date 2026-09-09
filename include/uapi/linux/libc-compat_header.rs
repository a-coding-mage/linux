/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Compatibility interface for userspace libc header coordination.
 *
 * The original header uses the C preprocessor to coordinate UAPI definitions
 * with libc headers. Rust has no direct equivalent for those include guards;
 * the constants below represent the Linux-header-first/default configuration.
 * Consumers selecting a libc-specific configuration should provide equivalent
 * conditional overrides.
 */

/* Definitions for if.h.  The __GLIBC__/_NET_IF_H/__USE_MISC branches in the
 * C header select zero for definitions already supplied by glibc. */
pub const __UAPI_DEF_IF_IFCONF: i32 = 1;
pub const __UAPI_DEF_IF_IFMAP: i32 = 1;
pub const __UAPI_DEF_IF_IFNAMSIZ: i32 = 1;
pub const __UAPI_DEF_IF_IFREQ: i32 = 1;
/* Everything up to IFF_DYNAMIC, matches net/if.h until glibc 2.23. */
pub const __UAPI_DEF_IF_NET_DEVICE_FLAGS: i32 = 1;
/* For the future if glibc adds IFF_LOWER_UP, IFF_DORMANT and IFF_ECHO. */
pub const __UAPI_DEF_IF_NET_DEVICE_FLAGS_LOWER_UP_DORMANT_ECHO: i32 = 1;

/* Definitions for in.h.  When _NETINET_IN_H is present, the C header uses
 * zero for definitions supplied by glibc. */
pub const __UAPI_DEF_IN_ADDR: i32 = 1;
pub const __UAPI_DEF_IN_IPPROTO: i32 = 1;
pub const __UAPI_DEF_IN_PKTINFO: i32 = 1;
pub const __UAPI_DEF_IP_MREQ: i32 = 1;
pub const __UAPI_DEF_SOCKADDR_IN: i32 = 1;
pub const __UAPI_DEF_IN_CLASS: i32 = 1;

pub const __UAPI_DEF_IN6_ADDR: i32 = 1;
/* The C header sets this to zero when __USE_MISC or __USE_GNU is present,
 * otherwise one; the in6_addr compatibility macros must be coordinated. */
pub const __UAPI_DEF_IN6_ADDR_ALT: i32 = 1;
pub const __UAPI_DEF_SOCKADDR_IN6: i32 = 1;
pub const __UAPI_DEF_IPV6_MREQ: i32 = 1;
pub const __UAPI_DEF_IPPROTO_V6: i32 = 1;
pub const __UAPI_DEF_IPV6_OPTIONS: i32 = 1;
pub const __UAPI_DEF_IN6_PKTINFO: i32 = 1;
pub const __UAPI_DEF_IP6_MTUINFO: i32 = 1;

/* Definitions for xattr.h.  The C header sets this to zero when
 * _SYS_XATTR_H is already defined. */
pub const __UAPI_DEF_XATTR: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
