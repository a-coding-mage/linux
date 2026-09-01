// SPDX-License-Identifier: GPL-2.0
// Dependencies from C includes: string2.h, strfilter.h, errno.h, stdlib.h,
// linux/ctype.h, linux/string.h, linux/zalloc.h.

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

#[repr(C)]
pub struct strfilter_node {
    pub p: *const c_char,
    pub l: *mut strfilter_node,
    pub r: *mut strfilter_node,
}

#[repr(C)]
pub struct strfilter {
    pub root: *mut strfilter_node,
}

unsafe extern "C" {
    fn skip_spaces(s: *const c_char) -> *const c_char;
    fn zfree(ptr: *mut *mut c_char);
    fn free(ptr: *mut c_void);
    fn zalloc(size: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strndup(s: *const c_char, n: usize) -> *mut c_char;
    fn strglobmatch(str_: *const c_char, pat: *const c_char) -> bool;
    fn strlen(s: *const c_char) -> usize;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn malloc(size: usize) -> *mut c_void;
    fn isspace(c: c_int) -> c_int;
}

/* Operators */
static OP_and: &[u8; 2] = b"&\0"; /* Logical AND */
static OP_or: &[u8; 2] = b"|\0"; /* Logical OR */
static OP_not: &[u8; 2] = b"!\0"; /* Logical NOT */

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

#[inline]
unsafe fn is_operator(c: c_char) -> bool {
    c as c_int == b'|' as c_int || c as c_int == b'&' as c_int || c as c_int == b'!' as c_int
}

#[inline]
unsafe fn is_separator(c: c_char) -> bool {
    is_operator(c) || c as c_int == b'(' as c_int || c as c_int == b')' as c_int
}

unsafe fn strfilter_node__delete(node: *mut strfilter_node) {
    if !node.is_null() {
        if !(*node).p.is_null() && !is_operator(*(*node).p) {
            zfree(&mut (*node).p as *mut *const c_char as *mut *mut c_char);
        }
        strfilter_node__delete((*node).l);
        strfilter_node__delete((*node).r);
        free(node as *mut c_void);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strfilter__delete(filter: *mut strfilter) {
    if !filter.is_null() {
        strfilter_node__delete((*filter).root);
        free(filter as *mut c_void);
    }
}

unsafe fn get_token(mut s: *const c_char, e: *mut *const c_char) -> *const c_char {
    let mut p: *const c_char;

    s = skip_spaces(s);

    if *s == 0 {
        p = s;
        *e = p;
        return s;
    }

    p = s.add(1);
    if !is_separator(*s) {
        /* End search */
        loop {
            while *p != 0 && !is_separator(*p) && isspace(*p as c_int) == 0 {
                p = p.add(1);
            }
            /* Escape and special case: '!' is also used in glob pattern */
            if *p.sub(1) as c_int == b'\\' as c_int
                || (*p as c_int == b'!' as c_int && *p.sub(1) as c_int == b'[' as c_int)
            {
                p = p.add(1);
                continue;
            }
            break;
        }
    }
    *e = p;
    s
}

unsafe fn strfilter_node__alloc(
    op: *const c_char,
    l: *mut strfilter_node,
    r: *mut strfilter_node,
) -> *mut strfilter_node {
    let node = zalloc(mem::size_of::<strfilter_node>()) as *mut strfilter_node;

    if !node.is_null() {
        (*node).p = op;
        (*node).l = l;
        (*node).r = r;
    }

    node
}

unsafe fn strfilter_node__new(mut s: *const c_char, ep: *mut *const c_char) -> *mut strfilter_node {
    let mut root: strfilter_node = mem::zeroed();
    let mut cur: *mut strfilter_node;
    let mut last_op: *mut strfilter_node;
    let mut e: *const c_char = ptr::null();

    if s.is_null() {
        return ptr::null_mut();
    }

    memset(
        &mut root as *mut strfilter_node as *mut c_void,
        0,
        mem::size_of::<strfilter_node>(),
    );
    cur = &mut root;
    last_op = cur;

    s = get_token(s, &mut e);
    while *s != 0 && *s as c_int != b')' as c_int {
        match *s as u8 {
            b'&' => {
                /* Exchg last OP->r with AND */
                if (*cur).r.is_null() || (*last_op).r.is_null() {
                    break;
                }
                cur = strfilter_node__alloc(OP_and.as_ptr() as *const c_char, (*last_op).r, ptr::null_mut());
                if cur.is_null() {
                    s = ptr::null();
                    break;
                }
                (*last_op).r = cur;
                last_op = cur;
            }
            b'|' => {
                /* Exchg the root with OR */
                if (*cur).r.is_null() || root.r.is_null() {
                    break;
                }
                cur = strfilter_node__alloc(OP_or.as_ptr() as *const c_char, root.r, ptr::null_mut());
                if cur.is_null() {
                    s = ptr::null();
                    break;
                }
                root.r = cur;
                last_op = cur;
            }
            b'!' => {
                /* Add NOT as a leaf node */
                if !(*cur).r.is_null() {
                    break;
                }
                (*cur).r = strfilter_node__alloc(OP_not.as_ptr() as *const c_char, ptr::null_mut(), ptr::null_mut());
                if (*cur).r.is_null() {
                    s = ptr::null();
                    break;
                }
                cur = (*cur).r;
            }
            b'(' => {
                /* Recursively parses inside the parenthesis */
                if !(*cur).r.is_null() {
                    break;
                }
                (*cur).r = strfilter_node__new(s.add(1), &mut s);
                if s.is_null() {
                    break;
                }
                if (*cur).r.is_null() || *s as c_int != b')' as c_int {
                    break;
                }
                e = s.add(1);
            }
            _ => {
                if !(*cur).r.is_null() {
                    break;
                }
                (*cur).r = strfilter_node__alloc(ptr::null(), ptr::null_mut(), ptr::null_mut());
                if (*cur).r.is_null() {
                    s = ptr::null();
                    break;
                }
                (*(*cur).r).p = strndup(s, e.offset_from(s) as usize);
                if (*(*cur).r).p.is_null() {
                    s = ptr::null();
                    break;
                }
            }
        }
        s = get_token(e, &mut e);
    }
    if !s.is_null() && !(*cur).r.is_null() {
        *ep = s;
        return root.r;
    }

    *ep = s;
    strfilter_node__delete(root.r);
    ptr::null_mut()
}

/*
 * Parse filter rule and return new strfilter.
 * Return NULL if fail, and *ep == NULL if memory allocation failed.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strfilter__new(
    rules: *const c_char,
    err: *mut *const c_char,
) -> *mut strfilter {
    let mut filter = zalloc(mem::size_of::<strfilter>()) as *mut strfilter;
    let mut ep: *const c_char = ptr::null();

    if !filter.is_null() {
        (*filter).root = strfilter_node__new(rules, &mut ep);
    }

    if filter.is_null() || (*filter).root.is_null() || *ep != 0 {
        if !err.is_null() {
            *err = ep;
        }
        strfilter__delete(filter);
        filter = ptr::null_mut();
    }

    filter
}

unsafe fn strfilter__append(
    filter: *mut strfilter,
    _or: bool,
    rules: *const c_char,
    err: *mut *const c_char,
) -> c_int {
    let mut right: *mut strfilter_node;
    let root: *mut strfilter_node;
    let mut ep: *const c_char = ptr::null();

    if filter.is_null() || rules.is_null() {
        return -EINVAL;
    }

    right = strfilter_node__new(rules, &mut ep);
    if right.is_null() || *ep != 0 {
        if !err.is_null() {
            *err = ep;
        }
        strfilter_node__delete(right);
        return if !ep.is_null() { -EINVAL } else { -ENOMEM };
    }
    root = strfilter_node__alloc(
        if _or { OP_or.as_ptr() } else { OP_and.as_ptr() } as *const c_char,
        (*filter).root,
        right,
    );
    if root.is_null() {
        ep = ptr::null();
        strfilter_node__delete(right);
        return if !ep.is_null() { -EINVAL } else { -ENOMEM };
    }

    (*filter).root = root;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strfilter__or(
    filter: *mut strfilter,
    rules: *const c_char,
    err: *mut *const c_char,
) -> c_int {
    strfilter__append(filter, true, rules, err)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strfilter__and(
    filter: *mut strfilter,
    rules: *const c_char,
    err: *mut *const c_char,
) -> c_int {
    strfilter__append(filter, false, rules, err)
}

unsafe fn strfilter_node__compare(node: *mut strfilter_node, str_: *const c_char) -> bool {
    if node.is_null() || (*node).p.is_null() {
        return false;
    }

    match *(*node).p as u8 {
        b'|' => {
            /* OR */
            strfilter_node__compare((*node).l, str_) || strfilter_node__compare((*node).r, str_)
        }
        b'&' => {
            /* AND */
            strfilter_node__compare((*node).l, str_) && strfilter_node__compare((*node).r, str_)
        }
        b'!' => {
            /* NOT */
            !strfilter_node__compare((*node).r, str_)
        }
        _ => strglobmatch(str_, (*node).p),
    }
}

/* Return true if STR matches the filter rules */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strfilter__compare(filter: *mut strfilter, str_: *const c_char) -> bool {
    if filter.is_null() {
        return false;
    }
    strfilter_node__compare((*filter).root, str_)
}

unsafe fn strfilter_node__sprint(node: *mut strfilter_node, buf: *mut c_char) -> c_int {
    let mut len: c_int = 0;
    let rlen: c_int;

    if node.is_null() || (*node).p.is_null() {
        return -EINVAL;
    }

    match *(*node).p as u8 {
        b'|' | b'&' => {
            len = strfilter_node__sprint_pt((*node).l, buf);
            if len < 0 {
                return len;
            }
            if !buf.is_null() {
                *buf.add(len as usize) = *(*node).p;
                len += 1;
                let buf = buf.add(len as usize);
                rlen = strfilter_node__sprint_pt((*node).r, buf);
            } else {
                len += 1;
                rlen = strfilter_node__sprint_pt((*node).r, buf);
            }
            if rlen < 0 {
                return rlen;
            }
            len += rlen;
        }
        b'!' => {
            if !buf.is_null() {
                *buf.add(len as usize) = *(*node).p;
                len += 1;
                let buf = buf.add(len as usize);
                rlen = strfilter_node__sprint_pt((*node).r, buf);
            } else {
                len += 1;
                rlen = strfilter_node__sprint_pt((*node).r, buf);
            }
            if rlen < 0 {
                return rlen;
            }
            len += rlen;
        }
        _ => {
            len = strlen((*node).p) as c_int;
            if !buf.is_null() {
                strcpy(buf, (*node).p);
            }
        }
    }

    len
}

/* sprint node in parenthesis if needed */
unsafe fn strfilter_node__sprint_pt(node: *mut strfilter_node, mut buf: *mut c_char) -> c_int {
    let len: c_int;
    let pt: c_int = if !(*node).r.is_null() { 2 } else { 0 }; /* don't need to check node->l */

    if !buf.is_null() && pt != 0 {
        *buf = b'(' as c_char;
        buf = buf.add(1);
    }
    len = strfilter_node__sprint(node, buf);
    if len < 0 {
        return len;
    }
    if !buf.is_null() && pt != 0 {
        *buf.add(len as usize) = b')' as c_char;
    }
    len + pt
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strfilter__string(filter: *mut strfilter) -> *mut c_char {
    let len: c_int;
    let mut ret: *mut c_char = ptr::null_mut();

    len = strfilter_node__sprint((*filter).root, ptr::null_mut());
    if len < 0 {
        return ptr::null_mut();
    }

    ret = malloc((len + 1) as usize) as *mut c_char;
    if !ret.is_null() {
        strfilter_node__sprint((*filter).root, ret);
    }

    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
