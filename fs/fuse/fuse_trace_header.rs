/* SPDX-License-Identifier: GPL-2.0 */
// Translation of fuse_trace.h.  TRACE_SYSTEM is `fuse`.

// The C header obtains these values from the FUSE opcode declarations supplied
// by the kernel headers.  Keep the symbolic table available to trace printers.
pub const OPCODES: &[(&str, &str)] = &[
    ("FUSE_LOOKUP", "FUSE_LOOKUP"),
    ("FUSE_FORGET", "FUSE_FORGET"),
    ("FUSE_GETATTR", "FUSE_GETATTR"),
    ("FUSE_SETATTR", "FUSE_SETATTR"),
    ("FUSE_READLINK", "FUSE_READLINK"),
    ("FUSE_SYMLINK", "FUSE_SYMLINK"),
    ("FUSE_MKNOD", "FUSE_MKNOD"),
    ("FUSE_MKDIR", "FUSE_MKDIR"),
    ("FUSE_UNLINK", "FUSE_UNLINK"),
    ("FUSE_RMDIR", "FUSE_RMDIR"),
    ("FUSE_RENAME", "FUSE_RENAME"),
    ("FUSE_LINK", "FUSE_LINK"),
    ("FUSE_OPEN", "FUSE_OPEN"),
    ("FUSE_READ", "FUSE_READ"),
    ("FUSE_WRITE", "FUSE_WRITE"),
    ("FUSE_STATFS", "FUSE_STATFS"),
    ("FUSE_RELEASE", "FUSE_RELEASE"),
    ("FUSE_FSYNC", "FUSE_FSYNC"),
    ("FUSE_SETXATTR", "FUSE_SETXATTR"),
    ("FUSE_GETXATTR", "FUSE_GETXATTR"),
    ("FUSE_LISTXATTR", "FUSE_LISTXATTR"),
    ("FUSE_REMOVEXATTR", "FUSE_REMOVEXATTR"),
    ("FUSE_FLUSH", "FUSE_FLUSH"),
    ("FUSE_INIT", "FUSE_INIT"),
    ("FUSE_OPENDIR", "FUSE_OPENDIR"),
    ("FUSE_READDIR", "FUSE_READDIR"),
    ("FUSE_RELEASEDIR", "FUSE_RELEASEDIR"),
    ("FUSE_FSYNCDIR", "FUSE_FSYNCDIR"),
    ("FUSE_GETLK", "FUSE_GETLK"),
    ("FUSE_SETLK", "FUSE_SETLK"),
    ("FUSE_SETLKW", "FUSE_SETLKW"),
    ("FUSE_ACCESS", "FUSE_ACCESS"),
    ("FUSE_CREATE", "FUSE_CREATE"),
    ("FUSE_INTERRUPT", "FUSE_INTERRUPT"),
    ("FUSE_BMAP", "FUSE_BMAP"),
    ("FUSE_DESTROY", "FUSE_DESTROY"),
    ("FUSE_IOCTL", "FUSE_IOCTL"),
    ("FUSE_POLL", "FUSE_POLL"),
    ("FUSE_NOTIFY_REPLY", "FUSE_NOTIFY_REPLY"),
    ("FUSE_BATCH_FORGET", "FUSE_BATCH_FORGET"),
    ("FUSE_FALLOCATE", "FUSE_FALLOCATE"),
    ("FUSE_READDIRPLUS", "FUSE_READDIRPLUS"),
    ("FUSE_RENAME2", "FUSE_RENAME2"),
    ("FUSE_LSEEK", "FUSE_LSEEK"),
    ("FUSE_COPY_FILE_RANGE", "FUSE_COPY_FILE_RANGE"),
    ("FUSE_SETUPMAPPING", "FUSE_SETUPMAPPING"),
    ("FUSE_REMOVEMAPPING", "FUSE_REMOVEMAPPING"),
    ("FUSE_SYNCFS", "FUSE_SYNCFS"),
    ("FUSE_TMPFILE", "FUSE_TMPFILE"),
    ("FUSE_STATX", "FUSE_STATX"),
    ("CUSE_INIT", "CUSE_INIT"),
];

// TRACE_EVENT declarations from the C tracepoint API.  The field layout and
// assignments are retained here because the actual tracepoint registration is
// supplied by the kernel tracepoint dependency.
#[repr(C)]
pub struct FuseRequestSendEntry {
    pub connection: usize, // dev_t
    pub unique: u64,
    pub opcode: u32, // enum fuse_opcode
    pub len: u32,
}

#[repr(C)]
pub struct FuseRequestSentEntry {
    pub connection: usize, // dev_t
    pub unique: u64,
    pub opcode: u32, // enum fuse_opcode
}

#[repr(C)]
pub struct FuseRequestEndEntry {
    pub connection: usize, // dev_t
    pub unique: u64,
    pub len: u32,
    pub error: i32,
}

// For each event, C assigns from req->chan->conn->dev and req->in/out.h:
// fuse_request_send: connection, unique, opcode, len (input header length).
// fuse_request_sent: connection, unique, opcode.
// fuse_request_end: connection, unique, len (output header length), error.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
