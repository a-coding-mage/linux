/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/linux/sunrpc/msg_prot.h
 *
 * Copyright (C) 1996, Olaf Kirch <okir@monad.swb.de>
 */

pub const RPC_VERSION: u32 = 2;

/* spec defines authentication flavor as an unsigned 32 bit integer */
pub type rpc_authflavor_t = u32;

pub const RPC_AUTH_NULL: u32 = 0;
pub const RPC_AUTH_UNIX: u32 = 1;
pub const RPC_AUTH_SHORT: u32 = 2;
pub const RPC_AUTH_DES: u32 = 3;
pub const RPC_AUTH_KRB: u32 = 4;
pub const RPC_AUTH_GSS: u32 = 6;
pub const RPC_AUTH_TLS: u32 = 7;
pub const RPC_AUTH_MAXFLAVOR: u32 = 8;
/* pseudoflavors: */
pub const RPC_AUTH_GSS_KRB5: u32 = 390003;
pub const RPC_AUTH_GSS_KRB5I: u32 = 390004;
pub const RPC_AUTH_GSS_KRB5P: u32 = 390005;
pub const RPC_AUTH_GSS_LKEY: u32 = 390006;
pub const RPC_AUTH_GSS_LKEYI: u32 = 390007;
pub const RPC_AUTH_GSS_LKEYP: u32 = 390008;
pub const RPC_AUTH_GSS_SPKM: u32 = 390009;
pub const RPC_AUTH_GSS_SPKMI: u32 = 390010;
pub const RPC_AUTH_GSS_SPKMP: u32 = 390011;

/* Maximum size (in octets) of the machinename in an AUTH_UNIX
 * credential (per RFC 5531 Appendix A)
 */
pub const RPC_MAX_MACHINENAME: u32 = 255;

/* Maximum size (in bytes) of an rpc credential or verifier */
pub const RPC_MAX_AUTH_SIZE: u32 = 400;

pub const RPC_CALL: u32 = 0;
pub const RPC_REPLY: u32 = 1;

pub const RPC_MSG_ACCEPTED: u32 = 0;
pub const RPC_MSG_DENIED: u32 = 1;

pub const RPC_SUCCESS: u32 = 0;
pub const RPC_PROG_UNAVAIL: u32 = 1;
pub const RPC_PROG_MISMATCH: u32 = 2;
pub const RPC_PROC_UNAVAIL: u32 = 3;
pub const RPC_GARBAGE_ARGS: u32 = 4;
pub const RPC_SYSTEM_ERR: u32 = 5;
/* internal use only */
pub const RPC_DROP_REPLY: u32 = 60000;

pub const RPC_MISMATCH: u32 = 0;
pub const RPC_AUTH_ERROR: u32 = 1;

pub const RPC_AUTH_OK: u32 = 0; /* success */
pub const RPC_AUTH_BADCRED: u32 = 1; /* bad credential (seal broken) */
pub const RPC_AUTH_REJECTEDCRED: u32 = 2; /* client must begin new session */
pub const RPC_AUTH_BADVERF: u32 = 3; /* bad verifier (seal broken) */
pub const RPC_AUTH_REJECTEDVERF: u32 = 4; /* verifier expired or replayed */
pub const RPC_AUTH_TOOWEAK: u32 = 5; /* rejected for security reasons */
pub const RPC_AUTH_INVALIDRESP: u32 = 6; /* bogus response verifier */
pub const RPC_AUTH_FAILED: u32 = 7; /* reason unknown */
/* RPCSEC_GSS errors */
pub const RPCSEC_GSS_CREDPROBLEM: u32 = 13; /* no credentials for user */
pub const RPCSEC_GSS_CTXPROBLEM: u32 = 14; /* problem with context */

pub const RPC_MAXNETNAMELEN: u32 = 256;

/*
 * From RFC 1831: record fragments have a four-byte header; the high bit
 * indicates the last fragment and the low 31 bits contain its length.
 */
pub type rpc_fraghdr = __be32;

pub const RPC_LAST_STREAM_FRAGMENT: u32 = 1u32 << 31;
pub const RPC_FRAGMENT_SIZE_MASK: u32 = !RPC_LAST_STREAM_FRAGMENT;
pub const RPC_MAX_FRAGMENT_SIZE: u32 = (1u32 << 31) - 1;

/* RPC call and reply header size as number of 32bit words (verifier
 * size computed separately, see below)
 */
pub const RPC_CALLHDRSIZE: u32 = 6;
pub const RPC_REPHDRSIZE: u32 = 4;

pub const RPC_MAX_HEADER_WITH_AUTH: u32 =
    RPC_CALLHDRSIZE + 2 * (2 + RPC_MAX_AUTH_SIZE / 4);
pub const RPC_MAX_REPHEADER_WITH_AUTH: u32 =
    RPC_REPHDRSIZE + (2 + RPC_MAX_AUTH_SIZE / 4);

pub const RPCBIND_NETID_UDP: &str = "udp";
pub const RPCBIND_NETID_TCP: &str = "tcp";
pub const RPCBIND_NETID_RDMA: &str = "rdma";
pub const RPCBIND_NETID_SCTP: &str = "sctp";
pub const RPCBIND_NETID_UDP6: &str = "udp6";
pub const RPCBIND_NETID_TCP6: &str = "tcp6";
pub const RPCBIND_NETID_RDMA6: &str = "rdma6";
pub const RPCBIND_NETID_SCTP6: &str = "sctp6";
pub const RPCBIND_NETID_LOCAL: &str = "local";

pub const RPCBIND_MAXNETIDLEN: u32 = 5;

/* Maximum size of the port number part of a universal address */
pub const RPCBIND_MAXUADDRPLEN: usize = ".255.255\0".len();

/* INET_ADDRSTRLEN and INET6_ADDRSTRLEN are supplied by linux/inet.h. */
pub const RPCBIND_MAXUADDR4LEN: usize = INET_ADDRSTRLEN + RPCBIND_MAXUADDRPLEN;
pub const RPCBIND_MAXUADDR6LEN: usize = INET6_ADDRSTRLEN + RPCBIND_MAXUADDRPLEN;

/* Assume INET6_ADDRSTRLEN will always be larger than INET_ADDRSTRLEN... */
pub const RPCBIND_MAXUADDRLEN: usize = RPCBIND_MAXUADDR6LEN;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
