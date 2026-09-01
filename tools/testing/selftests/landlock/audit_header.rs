/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Landlock audit helpers
 *
 * Copyright © 2024-2025 Microsoft Corporation
 */

/* C dependencies: _GNU_SOURCE, errno.h, linux/audit.h, linux/limits.h,
 * linux/netlink.h, regex.h, stdbool.h, stdint.h, stdio.h, stdlib.h, string.h,
 * sys/socket.h, sys/time.h, unistd.h, and "kselftest.h".
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

pub const REGEX_LANDLOCK_PREFIX: &[u8] =
    b"^audit([0-9.:]\\+): domain=\\([0-9a-f]\\+\\)\0";

#[repr(C)]
pub struct audit_filter {
    pub record_type: __u32,
    pub exe_len: size_t,
    pub exe: [c_char; PATH_MAX],
}

#[repr(C)]
pub union audit_message_union {
    pub status: audit_status,
    pub features: audit_features,
    pub rule: audit_rule_data,
    pub err: nlmsgerr,
    pub data: [c_char; PATH_MAX + 200],
}

#[repr(C)]
pub struct audit_message {
    pub header: nlmsghdr,
    pub u: audit_message_union,
}

pub static audit_tv_default: timeval = timeval {
    /*
     * Default socket timeout for audit_match_record() callers that expect a
     * record to arrive.  Asynchronous kauditd delivery can exceed 1 usec
     * under heavy debug configs (KASAN, lockdep), where kauditd_thread
     * scheduling between audit_log_end() and netlink_unicast() takes longer
     * than the previous 1 usec timeout. 1 second is a generous ceiling: on
     * the happy path, kauditd delivers within dozens of usec.
     */
    tv_sec: 1,
    tv_usec: 0,
};

pub static audit_tv_fast: timeval = timeval {
    /*
     * Fast timeout for paths that expect no record (audit_init() drain,
     * audit_count_records(), probes).  Causes audit_recv() to return
     * -EAGAIN once the socket buffer is empty, naturally terminating the
     * read loop.
     */
    tv_sec: 0,
    tv_usec: 1,
};

pub unsafe fn audit_send(fd: c_int, msg: *const audit_message) -> c_int {
    let mut addr: sockaddr_nl = zeroed();
    addr.nl_family = AF_NETLINK as _;
    let mut ret: c_int;

    loop {
        ret = sendto(
            fd,
            msg as *const c_void,
            (*msg).header.nlmsg_len as size_t,
            0,
            &addr as *const sockaddr_nl as *const sockaddr,
            size_of::<sockaddr_nl>() as socklen_t,
        ) as c_int;
        if !(ret < 0 && errno() == EINTR) {
            break;
        }
    }

    if ret < 0 {
        return -errno();
    }

    if ret as __u32 != (*msg).header.nlmsg_len {
        return -E2BIG;
    }

    0
}

pub unsafe fn audit_recv(fd: c_int, mut msg: *mut audit_message) -> c_int {
    let mut addr: sockaddr_nl = zeroed();
    let mut addrlen: socklen_t = size_of::<sockaddr_nl>() as socklen_t;
    let mut msg_tmp: audit_message = zeroed();
    let mut err: c_int;

    if msg.is_null() {
        msg = &mut msg_tmp;
    }

    loop {
        err = recvfrom(
            fd,
            msg as *mut c_void,
            size_of::<audit_message>() as size_t,
            0,
            &mut addr as *mut sockaddr_nl as *mut sockaddr,
            &mut addrlen,
        ) as c_int;
        if !(err < 0 && errno() == EINTR) {
            break;
        }
    }

    if err < 0 {
        return -errno();
    }

    if addrlen as usize != size_of::<sockaddr_nl>() || addr.nl_pid != 0 {
        return -EINVAL;
    }

    /* Checks Netlink error or end of messages. */
    if (*msg).header.nlmsg_type == NLMSG_ERROR {
        return (*msg).u.err.error;
    }

    0
}

pub unsafe fn audit_request(
    fd: c_int,
    request: *const audit_message,
    mut reply: *mut audit_message,
) -> c_int {
    let mut msg_tmp: audit_message = zeroed();
    let mut first_reply = true;
    let mut err: c_int;

    err = audit_send(fd, request);
    if err != 0 {
        return err;
    }

    if reply.is_null() {
        reply = &mut msg_tmp;
    }

    loop {
        if first_reply {
            first_reply = false;
        } else {
            reply = &mut msg_tmp;
        }

        err = audit_recv(fd, reply);
        if err != 0 {
            return err;
        }

        if !((*reply).header.nlmsg_type != NLMSG_ERROR
            && (*reply).u.err.msg.nlmsg_type != (*request).header.nlmsg_type)
        {
            break;
        }
    }

    (*reply).u.err.error
}

pub unsafe fn audit_filter_exe(
    audit_fd: c_int,
    filter: *const audit_filter,
    type_: __u16,
) -> c_int {
    let mut msg: audit_message = zeroed();
    msg.header.nlmsg_len = NLMSG_SPACE(size_of::<audit_rule_data>() as c_uint)
        + NLMSG_ALIGN((*filter).exe_len as c_uint);
    msg.header.nlmsg_type = type_;
    msg.header.nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK;
    msg.u.rule.flags = AUDIT_FILTER_EXCLUDE;
    msg.u.rule.action = AUDIT_NEVER;
    msg.u.rule.field_count = 1;
    msg.u.rule.fields[0] = (*filter).record_type;
    msg.u.rule.fieldflags[0] = AUDIT_NOT_EQUAL;
    msg.u.rule.values[0] = (*filter).exe_len as _;
    msg.u.rule.buflen = (*filter).exe_len as _;

    if (*filter).record_type != AUDIT_EXE {
        return -EINVAL;
    }

    memcpy(
        msg.u.rule.buf.as_mut_ptr() as *mut c_void,
        (*filter).exe.as_ptr() as *const c_void,
        (*filter).exe_len,
    );
    audit_request(audit_fd, &msg, ptr::null_mut())
}

pub unsafe fn audit_filter_drop(audit_fd: c_int, type_: __u16) -> c_int {
    let mut msg: audit_message = zeroed();
    msg.header.nlmsg_len = NLMSG_SPACE(size_of::<audit_rule_data>() as c_uint);
    msg.header.nlmsg_type = type_;
    msg.header.nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK;
    msg.u.rule.flags = AUDIT_FILTER_EXCLUDE;
    msg.u.rule.action = AUDIT_NEVER;
    msg.u.rule.field_count = 1;
    msg.u.rule.fields[0] = AUDIT_MSGTYPE;
    msg.u.rule.fieldflags[0] = AUDIT_NOT_EQUAL;
    msg.u.rule.values[0] = AUDIT_LANDLOCK_DOMAIN;

    audit_request(audit_fd, &msg, ptr::null_mut())
}

pub unsafe fn audit_set_status(fd: c_int, key: __u32, val: __u32) -> c_int {
    let mut msg: audit_message = zeroed();
    msg.header.nlmsg_len = NLMSG_SPACE(size_of::<audit_status>() as c_uint);
    msg.header.nlmsg_type = AUDIT_SET;
    msg.header.nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK;
    msg.u.status.mask = key;
    msg.u.status.enabled = if key == AUDIT_STATUS_ENABLED { val } else { 0 };
    msg.u.status.pid = if key == AUDIT_STATUS_PID { val } else { 0 };

    audit_request(fd, &msg, ptr::null_mut())
}

/*
 * @domain_id: The domain ID extracted from the audit message (if the first part
 * of @pattern is REGEX_LANDLOCK_PREFIX).  It is set to 0 if the domain ID is
 * not found.
 */
pub unsafe fn audit_match_record(
    audit_fd: c_int,
    type_: __u16,
    pattern: *const c_char,
    domain_id: *mut __u64,
) -> c_int {
    let mut msg: audit_message = zeroed();
    let mut last_mismatch: audit_message = zeroed();
    let mut err: c_int = 0;
    let mut num_type_match: c_int = 0;
    let mut matches: [regmatch_t; 2] = zeroed();
    let mut regex: regex_t = zeroed();

    let mut ret = regcomp(&mut regex, pattern, 0);
    if ret != 0 {
        return -EINVAL;
    }

    /*
     * Reads records until one matches both the expected type and the
     * pattern.  Type-matching records with non-matching content are
     * silently consumed, which handles stale domain deallocation records
     * from a previous test emitted asynchronously by kworker threads.
     */
    loop {
        memset(
            &mut msg as *mut audit_message as *mut c_void,
            0,
            size_of::<audit_message>(),
        );
        err = audit_recv(audit_fd, &mut msg);
        if err != 0 {
            if num_type_match != 0 {
                printf(b"DATA: %s\n\0".as_ptr() as *const c_char, msg.u.data.as_ptr());
                printf(
                    b"ERROR: %d record(s) matched type %u but not pattern: %s\n\0"
                        .as_ptr() as *const c_char,
                    num_type_match,
                    type_ as c_uint,
                    pattern,
                );
            }
            break;
        }

        if type_ != 0 && msg.header.nlmsg_type != type_ {
            continue;
        }

        ret = regexec(
            &regex,
            msg.u.data.as_ptr(),
            matches.len(),
            matches.as_mut_ptr(),
            0,
        );
        if ret == 0 {
            break;
        }

        num_type_match += 1;
        last_mismatch = msg;
    }

    if err == 0 && !domain_id.is_null() {
        *domain_id = 0;
        if matches[1].rm_so != -1 {
            let match_len: c_int = matches[1].rm_eo - matches[1].rm_so;
            /* The maximal characters of a 2^64 hexadecimal number is 17. */
            let mut dom_id: [c_char; 18] = [0; 18];

            if match_len > 0 && (match_len as usize) < size_of::<[c_char; 18]>() {
                memcpy(
                    dom_id.as_mut_ptr() as *mut c_void,
                    msg.u.data.as_ptr().add(matches[1].rm_so as usize) as *const c_void,
                    match_len as size_t,
                );
                dom_id[match_len as usize] = 0;
                if !domain_id.is_null() {
                    *domain_id = strtoull(dom_id.as_ptr(), ptr::null_mut(), 16) as __u64;
                }
            }
        }
    }

    regfree(&mut regex);
    err
}

pub unsafe fn matches_log_domain_allocated(
    audit_fd: c_int,
    pid: pid_t,
    domain_id: *mut __u64,
) -> c_int {
    static LOG_TEMPLATE: &[u8] = b"^audit([0-9.:]\\+): domain=\\([0-9a-f]\\+\\) status=allocated mode=enforcing pid=%d uid=[0-9]\\+ exe=\"[^\"]\\+\" comm=\".*_test\"$\0";
    let mut log_match: [c_char; LOG_TEMPLATE.len() + 10] = [0; LOG_TEMPLATE.len() + 10];

    let log_match_len = snprintf(
        log_match.as_mut_ptr(),
        log_match.len(),
        LOG_TEMPLATE.as_ptr() as *const c_char,
        pid,
    );
    if log_match_len as usize >= log_match.len() {
        return -E2BIG;
    }

    audit_match_record(
        audit_fd,
        AUDIT_LANDLOCK_DOMAIN,
        log_match.as_ptr(),
        domain_id,
    )
}

/*
 * Matches a domain deallocation record.  When expected_domain_id is non-zero,
 * the pattern includes the specific domain ID so that stale deallocation
 * records from a previous test (with a different domain ID) are skipped by
 * audit_match_record(), waiting for the asynchronous kworker deallocation with
 * the default patient timeout.
 *
 * When expected_domain_id is zero, the caller is probing for any dealloc record
 * that may or may not arrive.  Temporarily lowers the socket timeout to
 * audit_tv_fast for this probe so it returns promptly when no record is
 * pending; restores audit_tv_default after.
 */
pub unsafe fn matches_log_domain_deallocated(
    audit_fd: c_int,
    num_denials: c_uint,
    expected_domain_id: __u64,
    domain_id: *mut __u64,
) -> c_int {
    static LOG_TEMPLATE: &[u8] =
        b"^audit([0-9.:]\\+): domain=\\([0-9a-f]\\+\\) status=deallocated denials=%u$\0";
    static LOG_TEMPLATE_WITH_ID: &[u8] =
        b"^audit([0-9.:]\\+): domain=\\(%llx\\) status=deallocated denials=%u$\0";
    let mut log_match: [c_char; LOG_TEMPLATE_WITH_ID.len() + 32] =
        [0; LOG_TEMPLATE_WITH_ID.len() + 32];

    let log_match_len = if expected_domain_id != 0 {
        snprintf(
            log_match.as_mut_ptr(),
            log_match.len(),
            LOG_TEMPLATE_WITH_ID.as_ptr() as *const c_char,
            expected_domain_id as c_ulonglong,
            num_denials,
        )
    } else {
        snprintf(
            log_match.as_mut_ptr(),
            log_match.len(),
            LOG_TEMPLATE.as_ptr() as *const c_char,
            num_denials,
        )
    };

    if log_match_len as usize >= log_match.len() {
        return -E2BIG;
    }

    if expected_domain_id == 0 {
        if setsockopt(
            audit_fd,
            SOL_SOCKET,
            SO_RCVTIMEO,
            &audit_tv_fast as *const timeval as *const c_void,
            size_of::<timeval>() as socklen_t,
        ) != 0
        {
            return -errno();
        }
    }

    let mut err = audit_match_record(
        audit_fd,
        AUDIT_LANDLOCK_DOMAIN,
        log_match.as_ptr(),
        domain_id,
    );

    if expected_domain_id == 0 {
        if setsockopt(
            audit_fd,
            SOL_SOCKET,
            SO_RCVTIMEO,
            &audit_tv_default as *const timeval as *const c_void,
            size_of::<timeval>() as socklen_t,
        ) != 0
            && err == 0
        {
            err = -errno();
        }
    }

    err
}

#[repr(C)]
pub struct audit_records {
    pub access: size_t,
    pub domain: size_t,
}

/*
 * Counts remaining audit records by type, skipping domain deallocation records.
 * Deallocation records are emitted asynchronously from kworker threads after a
 * previous test's child has exited, so they can arrive after the drain in
 * audit_init() and after the preceding audit_match_record() call.  Allocation
 * records are emitted synchronously during landlock_log_denial() in the current
 * test's syscall context, so only those are counted in records->domain.
 *
 * Temporarily lowers SO_RCVTIMEO to audit_tv_fast for the read loop: this is a
 * "no record expected" path that should terminate on the first -EAGAIN.  The
 * default patient timeout is restored on exit for subsequent
 * audit_match_record() callers.
 */
pub unsafe fn audit_count_records(audit_fd: c_int, records: *mut audit_records) -> c_int {
    static DEALLOC_PATTERN: &[u8] =
        b"^audit([0-9.:]\\+): domain=\\([0-9a-f]\\+\\) status=deallocated \0";
    let mut msg: audit_message = zeroed();
    let mut dealloc_re: regex_t = zeroed();
    let mut err: c_int = 0;

    let mut ret = regcomp(&mut dealloc_re, DEALLOC_PATTERN.as_ptr() as *const c_char, 0);
    if ret != 0 {
        return -ENOMEM;
    }

    (*records).access = 0;
    (*records).domain = 0;

    if setsockopt(
        audit_fd,
        SOL_SOCKET,
        SO_RCVTIMEO,
        &audit_tv_fast as *const timeval as *const c_void,
        size_of::<timeval>() as socklen_t,
    ) != 0
    {
        err = -errno();
    } else {
        loop {
            memset(
                &mut msg as *mut audit_message as *mut c_void,
                0,
                size_of::<audit_message>(),
            );
            err = audit_recv(audit_fd, &mut msg);
            if err != 0 {
                if err == -EAGAIN {
                    err = 0;
                }
                break;
            }

            match msg.header.nlmsg_type {
                AUDIT_LANDLOCK_ACCESS => {
                    (*records).access += 1;
                }
                AUDIT_LANDLOCK_DOMAIN => {
                    ret = regexec(&dealloc_re, msg.u.data.as_ptr(), 0, ptr::null_mut(), 0);
                    if ret == REG_NOMATCH {
                        (*records).domain += 1;
                    } else if ret != 0 {
                        err = -EIO;
                        break;
                    }
                }
                _ => {}
            }
        }
    }

    if setsockopt(
        audit_fd,
        SOL_SOCKET,
        SO_RCVTIMEO,
        &audit_tv_default as *const timeval as *const c_void,
        size_of::<timeval>() as socklen_t,
    ) != 0
        && err == 0
    {
        err = -errno();
    }
    regfree(&mut dealloc_re);
    err
}

pub unsafe fn audit_init() -> c_int {
    let fd = socket(PF_NETLINK, SOCK_RAW, NETLINK_AUDIT);
    if fd < 0 {
        return -errno();
    }

    let mut err = audit_set_status(fd, AUDIT_STATUS_ENABLED, 1);
    if err != 0 {
        close(fd);
        return err;
    }

    err = audit_set_status(fd, AUDIT_STATUS_PID, getpid() as __u32);
    if err != 0 {
        close(fd);
        return err;
    }

    /* Uses the fast timeout to drain stale records below. */
    err = setsockopt(
        fd,
        SOL_SOCKET,
        SO_RCVTIMEO,
        &audit_tv_fast as *const timeval as *const c_void,
        size_of::<timeval>() as socklen_t,
    );
    if err != 0 {
        err = -errno();
        close(fd);
        return err;
    }

    /*
     * Drains stale audit records that accumulated in the kernel backlog
     * while no audit daemon socket was open.  This happens when non-audit
     * Landlock tests generate records while audit_enabled is non-zero (e.g.
     * from boot configuration), or when domain deallocation records arrive
     * asynchronously after a previous test's socket was closed.
     */
    while audit_recv(fd, ptr::null_mut()) == 0 {}

    /*
     * Restores the default timeout for audit_match_record() callers that
     * expect a record to arrive.  Paths that expect no record restore the
     * fast timeout locally (audit_count_records(), the expected_domain_id
     * == 0 probe in matches_log_domain_deallocated()).
     */
    err = setsockopt(
        fd,
        SOL_SOCKET,
        SO_RCVTIMEO,
        &audit_tv_default as *const timeval as *const c_void,
        size_of::<timeval>() as socklen_t,
    );
    if err != 0 {
        err = -errno();
        close(fd);
        return err;
    }

    fd
}

pub unsafe fn audit_init_filter_exe(filter: *mut audit_filter, path: *const c_char) -> c_int {
    let mut absolute_path: *mut c_char = ptr::null_mut();

    /* It is assume that there is not already filtering rules. */
    (*filter).record_type = AUDIT_EXE;
    if path.is_null() {
        let ret = readlink(
            b"/proc/self/exe\0".as_ptr() as *const c_char,
            (*filter).exe.as_mut_ptr(),
            size_of::<[c_char; PATH_MAX]>() - 1,
        );
        if ret < 0 {
            return -errno();
        }

        (*filter).exe_len = ret as size_t;
        return 0;
    }

    absolute_path = realpath(path, ptr::null_mut());
    if absolute_path.is_null() {
        return -errno();
    }

    /* No need for the terminating NULL byte. */
    (*filter).exe_len = strlen(absolute_path);
    if (*filter).exe_len > size_of::<[c_char; PATH_MAX]>() {
        free(absolute_path as *mut c_void);
        return -E2BIG;
    }

    memcpy(
        (*filter).exe.as_mut_ptr() as *mut c_void,
        absolute_path as *const c_void,
        (*filter).exe_len,
    );
    free(absolute_path as *mut c_void);
    0
}

pub unsafe fn audit_cleanup(audit_fd: c_int, filter: *mut audit_filter) -> c_int {
    let mut audit_fd = audit_fd;
    let mut filter = filter;
    let mut new_filter: audit_filter = zeroed();
    let mut err: c_int = 0;

    if audit_fd < 0 || filter.is_null() {
        /*
         * Simulates audit_init_with_exe_filter() when called from
         * FIXTURE_TEARDOWN_PARENT().
         */
        audit_fd = audit_init();
        if audit_fd < 0 {
            return audit_fd;
        }

        filter = &mut new_filter;
        err = audit_init_filter_exe(filter, ptr::null());
        if err != 0 {
            close(audit_fd);
            return err;
        }
    }

    /* Filters might not be in place. */
    audit_filter_exe(audit_fd, filter, AUDIT_DEL_RULE);
    audit_filter_drop(audit_fd, AUDIT_DEL_RULE);

    err = audit_set_status(audit_fd, AUDIT_STATUS_ENABLED, 0);

    close(audit_fd);
    err
}

pub unsafe fn audit_init_with_exe_filter(filter: *mut audit_filter) -> c_int {
    let fd = audit_init();
    if fd < 0 {
        return fd;
    }

    let mut err = audit_init_filter_exe(filter, ptr::null());
    if err != 0 {
        close(fd);
        return err;
    }

    err = audit_filter_exe(fd, filter, AUDIT_ADD_RULE);
    if err != 0 {
        close(fd);
        return err;
    }

    fd
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
