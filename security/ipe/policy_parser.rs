// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2024 Microsoft Corporation. All rights reserved.
 */

// Translated from ipe/policy_parser.c. Kernel headers and local headers:
// <linux/err.h>, <linux/slab.h>, <linux/parser.h>, <linux/types.h>,
// <linux/ctype.h>, "policy.h", "policy_parser.h", and "digest.h".

use core::ffi::{c_char, c_int, c_void};
use core::mem::MaybeUninit;
use core::ptr;

type size_t = usize;
type u16 = u16;
type bool_t = bool;

const ENOMEM: c_int = 12;
const EBADMSG: c_int = 74;
const START_COMMENT: c_int = b'#' as c_int;
const IPE_POLICY_DELIM: *const c_char = b" \t\0".as_ptr() as *const c_char;
const IPE_LINE_DELIM: *const c_char = b"\n\r\0".as_ptr() as *const c_char;
const MAX_OPT_ARGS: usize = 3;
const GFP_KERNEL: c_int = 0;

const IPE_ACTION_ALLOW: ipe_action_type = 0;
const IPE_ACTION_DENY: ipe_action_type = 1;
const IPE_ACTION_INVALID: ipe_action_type = 2;

const IPE_OP_EXEC: ipe_op_type = 0;
const IPE_OP_FIRMWARE: ipe_op_type = 1;
const IPE_OP_KERNEL_MODULE: ipe_op_type = 2;
const IPE_OP_KEXEC_IMAGE: ipe_op_type = 3;
const IPE_OP_KEXEC_INITRAMFS: ipe_op_type = 4;
const IPE_OP_POLICY: ipe_op_type = 5;
const IPE_OP_X509: ipe_op_type = 6;
const IPE_OP_INVALID: ipe_op_type = 7;

const IPE_PROP_BOOT_VERIFIED_FALSE: ipe_prop_type = 0;
const IPE_PROP_BOOT_VERIFIED_TRUE: ipe_prop_type = 1;
const IPE_PROP_DMV_ROOTHASH: ipe_prop_type = 2;
const IPE_PROP_DMV_SIG_FALSE: ipe_prop_type = 3;
const IPE_PROP_DMV_SIG_TRUE: ipe_prop_type = 4;
const IPE_PROP_FSV_DIGEST: ipe_prop_type = 5;
const IPE_PROP_FSV_SIG_FALSE: ipe_prop_type = 6;
const IPE_PROP_FSV_SIG_TRUE: ipe_prop_type = 7;
const IPE_PROP_INVALID: ipe_prop_type = 8;

type ipe_action_type = c_int;
type ipe_op_type = c_int;
type ipe_prop_type = c_int;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct ipe_policy_version {
    pub major: u16,
    pub minor: u16,
    pub rev: u16,
}

#[repr(C)]
pub struct ipe_op_table {
    pub default_action: ipe_action_type,
    pub rules: list_head,
}

#[repr(C)]
pub struct ipe_parsed_policy {
    pub name: *mut c_char,
    pub version: ipe_policy_version,
    pub global_default_action: ipe_action_type,
    pub rules: [ipe_op_table; IPE_OP_INVALID as usize],
}

#[repr(C)]
pub struct ipe_rule {
    pub next: list_head,
    pub props: list_head,
    pub op: ipe_op_type,
    pub action: ipe_action_type,
}

#[repr(C)]
pub struct ipe_prop {
    pub next: list_head,
    pub r#type: ipe_prop_type,
    pub value: *mut c_void,
}

#[repr(C)]
pub struct ipe_policy {
    pub text: *const c_char,
    pub textlen: size_t,
    pub parsed: *mut ipe_parsed_policy,
}

#[repr(C)]
pub struct substring_t {
    pub from: *mut c_char,
    pub to: *mut c_char,
}

#[repr(C)]
pub struct match_token_entry {
    pub token: c_int,
    pub pattern: *const c_char,
}

type match_table_t = [match_token_entry; 3];
type operation_match_table_t = [match_token_entry; 8];
type property_match_table_t = [match_token_entry; 9];

unsafe extern "C" {
    fn strlen(s: *const c_char) -> size_t;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn isspace(c: c_int) -> c_int;

    fn kzalloc(size: size_t, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *const c_void);
    fn kmemdup_nul(src: *const c_void, len: size_t, gfp: c_int) -> *mut c_char;
    fn kstrtou16(s: *const c_char, base: c_uint, res: *mut u16) -> c_int;

    fn match_token(s: *mut c_char, table: *const match_token_entry, args: *mut substring_t) -> c_int;
    fn match_strdup(s: *const substring_t) -> *mut c_char;

    fn ipe_digest_parse(value: *mut c_char) -> *mut c_void;
    fn ipe_digest_free(value: *mut c_void);
}

type c_uint = u32;

unsafe fn err_ptr<T>(err: isize) -> *mut T {
    err as *mut T
}

unsafe fn is_err<T>(ptr: *const T) -> bool {
    (ptr as usize) >= (usize::MAX - 4095)
}

unsafe fn is_err_or_null<T>(ptr: *const T) -> bool {
    ptr.is_null() || is_err(ptr)
}

unsafe fn ptr_err<T>(ptr: *const T) -> c_int {
    ptr as isize as c_int
}

unsafe fn init_list_head(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

unsafe fn list_empty(head: *const list_head) -> bool {
    (*head).next == head as *mut list_head
}

unsafe fn __list_add(new: *mut list_head, prev: *mut list_head, next: *mut list_head) {
    (*next).prev = new;
    (*new).next = next;
    (*new).prev = prev;
    (*prev).next = new;
}

unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) {
    __list_add(new, (*head).prev, head);
}

unsafe fn __list_del(prev: *mut list_head, next: *mut list_head) {
    (*next).prev = prev;
    (*prev).next = next;
}

unsafe fn list_del(entry: *mut list_head) {
    __list_del((*entry).prev, (*entry).next);
    (*entry).next = ptr::null_mut();
    (*entry).prev = ptr::null_mut();
}

unsafe fn container_of_ipe_prop_next(ptr: *mut list_head) -> *mut ipe_prop {
    (ptr as *mut u8).sub(core::mem::offset_of!(ipe_prop, next)) as *mut ipe_prop
}

unsafe fn container_of_ipe_rule_next(ptr: *mut list_head) -> *mut ipe_rule {
    (ptr as *mut u8).sub(core::mem::offset_of!(ipe_rule, next)) as *mut ipe_rule
}

/**
 * new_parsed_policy() - Allocate and initialize a parsed policy.
 *
 * Return:
 * * a pointer to the ipe_parsed_policy structure	- Success
 * * %-ENOMEM						- Out of memory (OOM)
 */
unsafe fn new_parsed_policy() -> *mut ipe_parsed_policy {
    let mut t: *mut ipe_op_table = ptr::null_mut();
    let mut i: size_t = 0;

    let p = kzalloc(core::mem::size_of::<ipe_parsed_policy>(), GFP_KERNEL) as *mut ipe_parsed_policy;
    if p.is_null() {
        return err_ptr(-ENOMEM as isize);
    }

    (*p).global_default_action = IPE_ACTION_INVALID;

    while i < (*p).rules.len() {
        t = &mut (*p).rules[i];

        (*t).default_action = IPE_ACTION_INVALID;
        init_list_head(&mut (*t).rules);
        i += 1;
    }

    p
}

/**
 * remove_comment() - Truncate all chars following START_COMMENT in a string.
 *
 * @line: Supplies a policy line string for preprocessing.
 */
unsafe fn remove_comment(mut line: *mut c_char) {
    line = strchr(line, START_COMMENT);

    if !line.is_null() {
        *line = b'\0' as c_char;
    }
}

/**
 * remove_trailing_spaces() - Truncate all trailing spaces in a string.
 *
 * @line: Supplies a policy line string for preprocessing.
 *
 * Return: The length of truncated string.
 */
unsafe fn remove_trailing_spaces(line: *mut c_char) -> size_t {
    let mut i: size_t = strlen(line);
    while i > 0 && isspace(*line.add(i - 1) as c_int) != 0 {
        i -= 1;
    }

    *line.add(i) = b'\0' as c_char;

    i
}

/**
 * parse_version() - Parse policy version.
 * @ver: Supplies a version string to be parsed.
 * @p: Supplies the partial parsed policy.
 *
 * Return:
 * * %0		- Success
 * * %-EBADMSG	- Version string is invalid
 * * %-ERANGE	- Version number overflow
 * * %-EINVAL	- Parsing error
 */
unsafe fn parse_version(mut ver: *mut c_char, p: *mut ipe_parsed_policy) -> c_int {
    let cv: [*mut u16; 3] = [
        &mut (*p).version.major,
        &mut (*p).version.minor,
        &mut (*p).version.rev,
    ];
    let mut sep_count: size_t = 0;
    let mut rc: c_int = 0;

    loop {
        let token = strsep(&mut ver, b".\0".as_ptr() as *const c_char);
        if token.is_null() {
            break;
        }

        /* prevent overflow */
        if sep_count >= cv.len() {
            return -EBADMSG;
        }

        rc = kstrtou16(token, 10, cv[sep_count]);
        if rc != 0 {
            return rc;
        }

        sep_count += 1;
    }

    /* prevent underflow */
    if sep_count != cv.len() {
        return -EBADMSG;
    }

    0
}

const IPE_HEADER_POLICY_NAME: c_int = 0;
const IPE_HEADER_POLICY_VERSION: c_int = 1;
const __IPE_HEADER_MAX: c_int = 2;

static HEADER_TOKENS: match_table_t = [
    match_token_entry { token: IPE_HEADER_POLICY_NAME, pattern: b"policy_name=%s\0".as_ptr() as *const c_char },
    match_token_entry { token: IPE_HEADER_POLICY_VERSION, pattern: b"policy_version=%s\0".as_ptr() as *const c_char },
    match_token_entry { token: __IPE_HEADER_MAX, pattern: ptr::null() },
];

/**
 * parse_header() - Parse policy header information.
 * @line: Supplies header line to be parsed.
 * @p: Supplies the partial parsed policy.
 *
 * Return:
 * * %0		- Success
 * * %-EBADMSG	- Header string is invalid
 * * %-ENOMEM	- Out of memory (OOM)
 * * %-ERANGE	- Version number overflow
 * * %-EINVAL	- Version parsing error
 */
unsafe fn parse_header(mut line: *mut c_char, p: *mut ipe_parsed_policy) -> c_int {
    let mut args: [substring_t; MAX_OPT_ARGS] = MaybeUninit::zeroed().assume_init();
    let mut ver: *mut c_char = ptr::null_mut();
    let mut idx: size_t = 0;
    let mut rc: c_int = 0;

    loop {
        let t = strsep(&mut line, IPE_POLICY_DELIM);
        if t.is_null() {
            break;
        }
        let token: c_int;

        if *t == b'\0' as c_char {
            continue;
        }
        if idx >= __IPE_HEADER_MAX as size_t {
            rc = -EBADMSG;
            break;
        }

        token = match_token(t, HEADER_TOKENS.as_ptr(), args.as_mut_ptr());
        if token != idx as c_int {
            rc = -EBADMSG;
            break;
        }

        match token {
            IPE_HEADER_POLICY_NAME => {
                (*p).name = match_strdup(&args[0]);
                if (*p).name.is_null() {
                    rc = -ENOMEM;
                }
            }
            IPE_HEADER_POLICY_VERSION => {
                ver = match_strdup(&args[0]);
                if ver.is_null() {
                    rc = -ENOMEM;
                } else {
                    rc = parse_version(ver, p);
                }
            }
            _ => {
                rc = -EBADMSG;
            }
        }
        if rc != 0 {
            break;
        }
        idx += 1;
    }

    if rc == 0 && idx != __IPE_HEADER_MAX as size_t {
        rc = -EBADMSG;
    }

    kfree(ver as *const c_void);
    rc
}

/**
 * token_default() - Determine if the given token is "DEFAULT".
 * @token: Supplies the token string to be compared.
 *
 * Return:
 * * %false	- The token is not "DEFAULT"
 * * %true	- The token is "DEFAULT"
 */
unsafe fn token_default(token: *mut c_char) -> bool_t {
    strcmp(token, b"DEFAULT\0".as_ptr() as *const c_char) == 0
}

/**
 * free_rule() - Free the supplied ipe_rule struct.
 * @r: Supplies the ipe_rule struct to be freed.
 *
 * Free a ipe_rule struct @r. Note @r must be removed from any lists before
 * calling this function.
 */
unsafe fn free_rule(r: *mut ipe_rule) {
    if is_err_or_null(r) {
        return;
    }

    let head = &mut (*r).props as *mut list_head;
    let mut pos = (*head).next;
    while pos != head {
        let next = (*pos).next;
        let p = container_of_ipe_prop_next(pos);
        list_del(&mut (*p).next);
        ipe_digest_free((*p).value);
        kfree(p as *const c_void);
        pos = next;
    }

    kfree(r as *const c_void);
}

static OPERATION_TOKENS: operation_match_table_t = [
    match_token_entry { token: IPE_OP_EXEC, pattern: b"op=EXECUTE\0".as_ptr() as *const c_char },
    match_token_entry { token: IPE_OP_FIRMWARE, pattern: b"op=FIRMWARE\0".as_ptr() as *const c_char },
    match_token_entry { token: IPE_OP_KERNEL_MODULE, pattern: b"op=KMODULE\0".as_ptr() as *const c_char },
    match_token_entry { token: IPE_OP_KEXEC_IMAGE, pattern: b"op=KEXEC_IMAGE\0".as_ptr() as *const c_char },
    match_token_entry { token: IPE_OP_KEXEC_INITRAMFS, pattern: b"op=KEXEC_INITRAMFS\0".as_ptr() as *const c_char },
    match_token_entry { token: IPE_OP_POLICY, pattern: b"op=POLICY\0".as_ptr() as *const c_char },
    match_token_entry { token: IPE_OP_X509, pattern: b"op=X509_CERT\0".as_ptr() as *const c_char },
    match_token_entry { token: IPE_OP_INVALID, pattern: ptr::null() },
];

/**
 * parse_operation() - Parse the operation type given a token string.
 * @t: Supplies the token string to be parsed.
 *
 * Return: The parsed operation type.
 */
unsafe fn parse_operation(t: *mut c_char) -> ipe_op_type {
    let mut args: [substring_t; MAX_OPT_ARGS] = MaybeUninit::zeroed().assume_init();

    match_token(t, OPERATION_TOKENS.as_ptr(), args.as_mut_ptr())
}

static ACTION_TOKENS: match_table_t = [
    match_token_entry { token: IPE_ACTION_ALLOW, pattern: b"action=ALLOW\0".as_ptr() as *const c_char },
    match_token_entry { token: IPE_ACTION_DENY, pattern: b"action=DENY\0".as_ptr() as *const c_char },
    match_token_entry { token: IPE_ACTION_INVALID, pattern: ptr::null() },
];

/**
 * parse_action() - Parse the action type given a token string.
 * @t: Supplies the token string to be parsed.
 *
 * Return: The parsed action type.
 */
unsafe fn parse_action(t: *mut c_char) -> ipe_action_type {
    let mut args: [substring_t; MAX_OPT_ARGS] = MaybeUninit::zeroed().assume_init();

    match_token(t, ACTION_TOKENS.as_ptr(), args.as_mut_ptr())
}

static PROPERTY_TOKENS: property_match_table_t = [
    match_token_entry { token: IPE_PROP_BOOT_VERIFIED_FALSE, pattern: b"boot_verified=FALSE\0".as_ptr() as *const c_char },
    match_token_entry { token: IPE_PROP_BOOT_VERIFIED_TRUE, pattern: b"boot_verified=TRUE\0".as_ptr() as *const c_char },
    match_token_entry { token: IPE_PROP_DMV_ROOTHASH, pattern: b"dmverity_roothash=%s\0".as_ptr() as *const c_char },
    match_token_entry { token: IPE_PROP_DMV_SIG_FALSE, pattern: b"dmverity_signature=FALSE\0".as_ptr() as *const c_char },
    match_token_entry { token: IPE_PROP_DMV_SIG_TRUE, pattern: b"dmverity_signature=TRUE\0".as_ptr() as *const c_char },
    match_token_entry { token: IPE_PROP_FSV_DIGEST, pattern: b"fsverity_digest=%s\0".as_ptr() as *const c_char },
    match_token_entry { token: IPE_PROP_FSV_SIG_FALSE, pattern: b"fsverity_signature=FALSE\0".as_ptr() as *const c_char },
    match_token_entry { token: IPE_PROP_FSV_SIG_TRUE, pattern: b"fsverity_signature=TRUE\0".as_ptr() as *const c_char },
    match_token_entry { token: IPE_PROP_INVALID, pattern: ptr::null() },
];

/**
 * parse_property() - Parse a rule property given a token string.
 * @t: Supplies the token string to be parsed.
 * @r: Supplies the ipe_rule the parsed property will be associated with.
 *
 * This function parses and associates a property with an IPE rule based
 * on a token string.
 *
 * Return:
 * * %0		- Success
 * * %-ENOMEM	- Out of memory (OOM)
 * * %-EBADMSG	- The supplied token cannot be parsed
 */
unsafe fn parse_property(t: *mut c_char, r: *mut ipe_rule) -> c_int {
    let mut args: [substring_t; MAX_OPT_ARGS] = MaybeUninit::zeroed().assume_init();
    let mut rc: c_int = 0;
    let token: c_int;
    let mut dup: *mut c_char = ptr::null_mut();

    let p = kzalloc(core::mem::size_of::<ipe_prop>(), GFP_KERNEL) as *mut ipe_prop;
    if p.is_null() {
        return -ENOMEM;
    }

    token = match_token(t, PROPERTY_TOKENS.as_ptr(), args.as_mut_ptr());

    match token {
        IPE_PROP_DMV_ROOTHASH | IPE_PROP_FSV_DIGEST => {
            dup = match_strdup(&args[0]);
            if dup.is_null() {
                rc = -ENOMEM;
            } else {
                (*p).value = ipe_digest_parse(dup);
                if is_err((*p).value) {
                    rc = ptr_err((*p).value);
                } else {
                    (*p).r#type = token;
                }
            }
        }
        IPE_PROP_BOOT_VERIFIED_FALSE
        | IPE_PROP_BOOT_VERIFIED_TRUE
        | IPE_PROP_DMV_SIG_FALSE
        | IPE_PROP_DMV_SIG_TRUE
        | IPE_PROP_FSV_SIG_FALSE
        | IPE_PROP_FSV_SIG_TRUE => {
            (*p).r#type = token;
        }
        _ => {
            rc = -EBADMSG;
        }
    }

    if rc != 0 {
        kfree(p as *const c_void);
    } else {
        list_add_tail(&mut (*p).next, &mut (*r).props);
    }

    kfree(dup as *const c_void);
    rc
}

/**
 * parse_rule() - parse a policy rule line.
 * @line: Supplies rule line to be parsed.
 * @p: Supplies the partial parsed policy.
 *
 * Return:
 * * 0		- Success
 * * %-ENOMEM	- Out of memory (OOM)
 * * %-EBADMSG	- Policy syntax error
 */
unsafe fn parse_rule(mut line: *mut c_char, p: *mut ipe_parsed_policy) -> c_int {
    let mut action: ipe_action_type = IPE_ACTION_INVALID;
    let mut op: ipe_op_type = IPE_OP_INVALID;
    let mut is_default_rule: bool_t = false;
    let mut first_token: bool_t = true;
    let mut op_parsed: bool_t = false;
    let mut rc: c_int = 0;
    let mut t: *mut c_char = ptr::null_mut();

    if is_err_or_null(line) {
        return -EBADMSG;
    }

    let r = kzalloc(core::mem::size_of::<ipe_rule>(), GFP_KERNEL) as *mut ipe_rule;
    if r.is_null() {
        return -ENOMEM;
    }

    init_list_head(&mut (*r).next);
    init_list_head(&mut (*r).props);

    loop {
        t = strsep(&mut line, IPE_POLICY_DELIM);
        if line.is_null() {
            break;
        }
        if *t == b'\0' as c_char {
            continue;
        }
        if first_token && token_default(t) {
            is_default_rule = true;
        } else if !op_parsed {
            op = parse_operation(t);
            if op == IPE_OP_INVALID {
                rc = -EBADMSG;
            } else {
                op_parsed = true;
            }
        } else {
            rc = parse_property(t, r);
        }

        if rc != 0 {
            free_rule(r);
            return rc;
        }
        first_token = false;
    }

    action = parse_action(t);
    if action == IPE_ACTION_INVALID {
        rc = -EBADMSG;
        free_rule(r);
        return rc;
    }

    if is_default_rule {
        if !list_empty(&(*r).props) {
            rc = -EBADMSG;
        } else if op == IPE_OP_INVALID {
            if (*p).global_default_action != IPE_ACTION_INVALID {
                rc = -EBADMSG;
            } else {
                (*p).global_default_action = action;
            }
        } else if (*p).rules[op as usize].default_action != IPE_ACTION_INVALID {
            rc = -EBADMSG;
        } else {
            (*p).rules[op as usize].default_action = action;
        }
    } else if op != IPE_OP_INVALID && action != IPE_ACTION_INVALID {
        (*r).op = op;
        (*r).action = action;
    } else {
        rc = -EBADMSG;
    }

    if rc != 0 {
        free_rule(r);
        return rc;
    }
    if !is_default_rule {
        list_add_tail(&mut (*r).next, &mut (*p).rules[op as usize].rules);
    } else {
        free_rule(r);
    }

    rc
}

/**
 * ipe_free_parsed_policy() - free a parsed policy structure.
 * @p: Supplies the parsed policy.
 */
#[no_mangle]
pub unsafe extern "C" fn ipe_free_parsed_policy(p: *mut ipe_parsed_policy) {
    let mut i: size_t = 0;

    if is_err_or_null(p) {
        return;
    }

    while i < (*p).rules.len() {
        let head = &mut (*p).rules[i].rules as *mut list_head;
        let mut pos = (*head).next;
        while pos != head {
            let next = (*pos).next;
            let pp = container_of_ipe_rule_next(pos);
            list_del(&mut (*pp).next);
            free_rule(pp);
            pos = next;
        }
        i += 1;
    }

    kfree((*p).name as *const c_void);
    kfree(p as *const c_void);
}

/**
 * validate_policy() - validate a parsed policy.
 * @p: Supplies the fully parsed policy.
 *
 * Given a policy structure that was just parsed, validate that all
 * operations have their default rules or a global default rule is set.
 *
 * Return:
 * * %0		- Success
 * * %-EBADMSG	- Policy is invalid
 */
unsafe fn validate_policy(p: *const ipe_parsed_policy) -> c_int {
    let mut i: size_t = 0;

    if (*p).global_default_action != IPE_ACTION_INVALID {
        return 0;
    }

    while i < (*p).rules.len() {
        if (*p).rules[i].default_action == IPE_ACTION_INVALID {
            return -EBADMSG;
        }
        i += 1;
    }

    0
}

/**
 * ipe_parse_policy() - Given a string, parse the string into an IPE policy.
 * @p: partially filled ipe_policy structure to populate with the result.
 *     it must have text and textlen set.
 *
 * Return:
 * * %0		- Success
 * * %-EBADMSG	- Policy is invalid
 * * %-ENOMEM	- Out of Memory
 * * %-ERANGE	- Policy version number overflow
 * * %-EINVAL	- Policy version parsing error
 */
#[no_mangle]
pub unsafe extern "C" fn ipe_parse_policy(p: *mut ipe_policy) -> c_int {
    let mut policy: *mut c_char;
    let dup: *mut c_char;
    let mut header_parsed: bool_t = false;
    let mut line: *mut c_char;
    let mut len: size_t;
    let mut rc: c_int = 0;

    if (*p).textlen == 0 {
        return -EBADMSG;
    }

    policy = kmemdup_nul((*p).text as *const c_void, (*p).textlen, GFP_KERNEL);
    if policy.is_null() {
        return -ENOMEM;
    }
    dup = policy;

    let pp = new_parsed_policy();
    if is_err(pp) {
        rc = ptr_err(pp);
        kfree(dup as *const c_void);
        return rc;
    }

    loop {
        line = strsep(&mut policy, IPE_LINE_DELIM);
        if line.is_null() {
            break;
        }
        remove_comment(line);
        len = remove_trailing_spaces(line);
        if len == 0 {
            continue;
        }

        if !header_parsed {
            rc = parse_header(line, pp);
            if rc != 0 {
                ipe_free_parsed_policy(pp);
                kfree(dup as *const c_void);
                return rc;
            }
            header_parsed = true;
        } else {
            rc = parse_rule(line, pp);
            if rc != 0 {
                ipe_free_parsed_policy(pp);
                kfree(dup as *const c_void);
                return rc;
            }
        }
    }

    if !header_parsed || validate_policy(pp) != 0 {
        rc = -EBADMSG;
        ipe_free_parsed_policy(pp);
        kfree(dup as *const c_void);
        return rc;
    }

    (*p).parsed = pp;

    kfree(dup as *const c_void);
    rc
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
