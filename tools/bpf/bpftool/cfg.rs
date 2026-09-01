// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/* Copyright (C) 2018 Netronome Systems, Inc. */

/* Translated from C implementation source.  The original file included:
 * <linux/list.h>, <stdlib.h>, <string.h>, "cfg.h", "main.h",
 * and "xlated_dumper.h".
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type __u8 = u8;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct bpf_insn {
    pub code: __u8,
    pub dst_src: __u8,
    pub off: i16,
    pub imm: i32,
}

impl bpf_insn {
    unsafe fn src_reg(&self) -> __u8 {
        self.dst_src >> 4
    }
}

#[repr(C)]
pub struct dump_data {
    _private: [u8; 0],
}

#[repr(C)]
struct cfg {
    funcs: list_head,
    func_num: c_int,
}

#[repr(C)]
struct func_node {
    l: list_head,
    bbs: list_head,
    start: *mut bpf_insn,
    end: *mut bpf_insn,
    idx: c_int,
    bb_num: c_int,
}

#[repr(C)]
struct bb_node {
    l: list_head,
    e_prevs: list_head,
    e_succs: list_head,
    head: *mut bpf_insn,
    tail: *mut bpf_insn,
    idx: c_int,
}

const EDGE_FLAG_EMPTY: c_int = 0x0;
const EDGE_FLAG_FALLTHROUGH: c_int = 0x1;
const EDGE_FLAG_JUMP: c_int = 0x2;

#[repr(C)]
struct edge_node {
    l: list_head,
    src: *mut bb_node,
    dst: *mut bb_node,
    flags: c_int,
}

const ENTRY_BLOCK_INDEX: c_int = 0;
const EXIT_BLOCK_INDEX: c_int = 1;
const NUM_FIXED_BLOCKS: c_uint = 2;

const BPF_JMP: __u8 = 0x05;
const BPF_JMP32: __u8 = 0x06;
const BPF_CALL: __u8 = 0x80;
const BPF_EXIT: __u8 = 0x90;
const BPF_JA: __u8 = 0x00;
const BPF_PSEUDO_CALL: __u8 = 1;

unsafe extern "C" {
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn printf(format: *const c_char, ...) -> c_int;
    fn p_err(format: *const c_char, ...);
    fn dump_xlated_for_graph(
        dd: *mut dump_data,
        start: *mut bpf_insn,
        end: *mut bpf_insn,
        start_idx: c_uint,
        opcodes: bool,
        linum: bool,
    );
}

unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    unsafe {
        (*list).next = list;
        (*list).prev = list;
    }
}

unsafe fn __list_add(new: *mut list_head, prev: *mut list_head, next: *mut list_head) {
    unsafe {
        (*next).prev = new;
        (*new).next = next;
        (*new).prev = prev;
        (*prev).next = new;
    }
}

unsafe fn list_add(new: *mut list_head, head: *mut list_head) {
    unsafe {
        __list_add(new, head, (*head).next);
    }
}

unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) {
    unsafe {
        __list_add(new, (*head).prev, head);
    }
}

unsafe fn __list_del(prev: *mut list_head, next: *mut list_head) {
    unsafe {
        (*next).prev = prev;
        (*prev).next = next;
    }
}

unsafe fn list_del(entry: *mut list_head) {
    unsafe {
        __list_del((*entry).prev, (*entry).next);
    }
}

unsafe fn list_empty(head: *const list_head) -> bool {
    unsafe { (*head).next == head as *mut list_head }
}

unsafe fn container_of_func_node(ptr: *mut list_head) -> *mut func_node {
    unsafe { (ptr as *mut u8).sub(offset_of!(func_node, l)) as *mut func_node }
}

unsafe fn container_of_bb_node(ptr: *mut list_head) -> *mut bb_node {
    unsafe { (ptr as *mut u8).sub(offset_of!(bb_node, l)) as *mut bb_node }
}

unsafe fn container_of_edge_node(ptr: *mut list_head) -> *mut edge_node {
    unsafe { (ptr as *mut u8).sub(offset_of!(edge_node, l)) as *mut edge_node }
}

unsafe fn func_prev(func: *mut func_node) -> *mut func_node {
    unsafe { container_of_func_node((*func).l.prev) }
}

unsafe fn func_next(func: *mut func_node) -> *mut func_node {
    unsafe { container_of_func_node((*func).l.next) }
}

unsafe fn bb_prev(bb: *mut bb_node) -> *mut bb_node {
    unsafe { container_of_bb_node((*bb).l.prev) }
}

unsafe fn bb_next(bb: *mut bb_node) -> *mut bb_node {
    unsafe { container_of_bb_node((*bb).l.next) }
}

unsafe fn cfg_first_func(cfg: *mut cfg) -> *mut func_node {
    unsafe { container_of_func_node((*cfg).funcs.next) }
}

unsafe fn cfg_last_func(cfg: *mut cfg) -> *mut func_node {
    unsafe { container_of_func_node((*cfg).funcs.prev) }
}

unsafe fn func_first_bb(func: *mut func_node) -> *mut bb_node {
    unsafe { container_of_bb_node((*func).bbs.next) }
}

unsafe fn func_last_bb(func: *mut func_node) -> *mut bb_node {
    unsafe { container_of_bb_node((*func).bbs.prev) }
}

unsafe fn entry_bb(func: *mut func_node) -> *mut bb_node {
    unsafe { func_first_bb(func) }
}

unsafe fn exit_bb(func: *mut func_node) -> *mut bb_node {
    unsafe { func_last_bb(func) }
}

fn BPF_CLASS(code: __u8) -> __u8 {
    code & 0x07
}

fn BPF_OP(code: __u8) -> __u8 {
    code & 0xf0
}

unsafe fn cfg_append_func(cfg: *mut cfg, insn: *mut bpf_insn) -> *mut func_node {
    unsafe {
        let mut func: *mut func_node = ptr::null_mut();
        let mut pos = (*cfg).funcs.next;

        while pos != &mut (*cfg).funcs {
            func = container_of_func_node(pos);
            if (*func).start == insn {
                return func;
            } else if (*func).start > insn {
                break;
            }
            pos = (*pos).next;
        }

        if pos == &mut (*cfg).funcs {
            func = container_of_func_node(pos);
        }
        func = func_prev(func);
        let new_func = calloc(1, size_of::<func_node>()) as *mut func_node;
        if new_func.is_null() {
            p_err(c"OOM when allocating FUNC node".as_ptr());
            return ptr::null_mut();
        }
        (*new_func).start = insn;
        (*new_func).idx = (*cfg).func_num;
        list_add(&mut (*new_func).l, &mut (*func).l);
        (*cfg).func_num += 1;

        new_func
    }
}

unsafe fn func_append_bb(func: *mut func_node, insn: *mut bpf_insn) -> *mut bb_node {
    unsafe {
        let mut bb: *mut bb_node = ptr::null_mut();
        let mut pos = (*func).bbs.next;

        while pos != &mut (*func).bbs {
            bb = container_of_bb_node(pos);
            if (*bb).head == insn {
                return bb;
            } else if (*bb).head > insn {
                break;
            }
            pos = (*pos).next;
        }

        if pos == &mut (*func).bbs {
            bb = container_of_bb_node(pos);
        }
        bb = bb_prev(bb);
        let new_bb = calloc(1, size_of::<bb_node>()) as *mut bb_node;
        if new_bb.is_null() {
            p_err(c"OOM when allocating BB node".as_ptr());
            return ptr::null_mut();
        }
        (*new_bb).head = insn;
        INIT_LIST_HEAD(&mut (*new_bb).e_prevs);
        INIT_LIST_HEAD(&mut (*new_bb).e_succs);
        list_add(&mut (*new_bb).l, &mut (*bb).l);

        new_bb
    }
}

unsafe fn func_insert_dummy_bb(after: *mut list_head) -> *mut bb_node {
    unsafe {
        let bb = calloc(1, size_of::<bb_node>()) as *mut bb_node;
        if bb.is_null() {
            p_err(c"OOM when allocating BB node".as_ptr());
            return ptr::null_mut();
        }

        INIT_LIST_HEAD(&mut (*bb).e_prevs);
        INIT_LIST_HEAD(&mut (*bb).e_succs);
        list_add(&mut (*bb).l, after);

        bb
    }
}

unsafe fn cfg_partition_funcs(cfg: *mut cfg, mut cur: *mut bpf_insn, end: *mut bpf_insn) -> bool {
    unsafe {
        let mut func = cfg_append_func(cfg, cur);
        if func.is_null() {
            return true;
        }

        while cur < end {
            if (*cur).code != (BPF_JMP | BPF_CALL) {
                cur = cur.add(1);
                continue;
            }
            if (*cur).src_reg() != BPF_PSEUDO_CALL {
                cur = cur.add(1);
                continue;
            }
            func = cfg_append_func(cfg, cur.offset((*cur).off as isize + 1));
            if func.is_null() {
                return true;
            }
            cur = cur.add(1);
        }

        let last_func = cfg_last_func(cfg);
        (*last_func).end = end.sub(1);
        func = cfg_first_func(cfg);
        while &mut (*func).l != &mut (*last_func).l {
            (*func).end = (*func_next(func)).start.sub(1);
            func = func_next(func);
        }

        false
    }
}

fn is_jmp_insn(code: __u8) -> bool {
    BPF_CLASS(code) == BPF_JMP || BPF_CLASS(code) == BPF_JMP32
}

unsafe fn func_partition_bb_head(func: *mut func_node) -> bool {
    unsafe {
        let mut cur = (*func).start;
        let end = (*func).end;
        INIT_LIST_HEAD(&mut (*func).bbs);
        let mut bb = func_append_bb(func, cur);
        if bb.is_null() {
            return true;
        }

        while cur <= end {
            if is_jmp_insn((*cur).code) {
                let opcode: __u8 = BPF_OP((*cur).code);

                if opcode == BPF_EXIT || opcode == BPF_CALL {
                    cur = cur.add(1);
                    continue;
                }

                bb = func_append_bb(func, cur.offset((*cur).off as isize + 1));
                if bb.is_null() {
                    return true;
                }

                if opcode != BPF_JA {
                    bb = func_append_bb(func, cur.add(1));
                    if bb.is_null() {
                        return true;
                    }
                }
            }
            cur = cur.add(1);
        }

        false
    }
}

unsafe fn func_partition_bb_tail(func: *mut func_node) {
    unsafe {
        let mut bb_idx: c_uint = NUM_FIXED_BLOCKS;
        let last = func_last_bb(func);
        (*last).tail = (*func).end;
        let mut bb = func_first_bb(func);
        while &mut (*bb).l != &mut (*last).l {
            (*bb).tail = (*bb_next(bb)).head.sub(1);
            (*bb).idx = bb_idx as c_int;
            bb_idx += 1;
            bb = bb_next(bb);
        }

        (*last).idx = bb_idx as c_int;
        bb_idx += 1;
        (*func).bb_num = bb_idx as c_int;
    }
}

unsafe fn func_add_special_bb(func: *mut func_node) -> bool {
    unsafe {
        let mut bb = func_insert_dummy_bb(&mut (*func).bbs);
        if bb.is_null() {
            return true;
        }
        (*bb).idx = ENTRY_BLOCK_INDEX;

        bb = func_insert_dummy_bb(&mut (*func_last_bb(func)).l);
        if bb.is_null() {
            return true;
        }
        (*bb).idx = EXIT_BLOCK_INDEX;

        false
    }
}

unsafe fn func_partition_bb(func: *mut func_node) -> bool {
    unsafe {
        if func_partition_bb_head(func) {
            return true;
        }

        func_partition_bb_tail(func);

        false
    }
}

unsafe fn func_search_bb_with_head(func: *mut func_node, insn: *mut bpf_insn) -> *mut bb_node {
    unsafe {
        let mut pos = (*func).bbs.next;
        while pos != &mut (*func).bbs {
            let bb = container_of_bb_node(pos);
            if (*bb).head == insn {
                return bb;
            }
            pos = (*pos).next;
        }

        ptr::null_mut()
    }
}

unsafe fn new_edge(src: *mut bb_node, dst: *mut bb_node, flags: c_int) -> *mut edge_node {
    unsafe {
        let e = calloc(1, size_of::<edge_node>()) as *mut edge_node;
        if e.is_null() {
            p_err(c"OOM when allocating edge node".as_ptr());
            return ptr::null_mut();
        }

        if !src.is_null() {
            (*e).src = src;
        }
        if !dst.is_null() {
            (*e).dst = dst;
        }

        (*e).flags |= flags;

        e
    }
}

unsafe fn func_add_bb_edges(func: *mut func_node) -> bool {
    unsafe {
        let mut bb = entry_bb(func);
        let mut e = new_edge(bb, bb_next(bb), EDGE_FLAG_FALLTHROUGH);
        if e.is_null() {
            return true;
        }
        list_add_tail(&mut (*e).l, &mut (*bb).e_succs);

        bb = exit_bb(func);
        e = new_edge(bb_prev(bb), bb, EDGE_FLAG_FALLTHROUGH);
        if e.is_null() {
            return true;
        }
        list_add_tail(&mut (*e).l, &mut (*bb).e_prevs);

        bb = entry_bb(func);
        bb = bb_next(bb);
        while &mut (*bb).l != &mut (*exit_bb(func)).l {
            e = new_edge(bb, ptr::null_mut(), EDGE_FLAG_EMPTY);
            if e.is_null() {
                return true;
            }
            (*e).src = bb;

            let insn = (*bb).tail;
            if !is_jmp_insn((*insn).code)
                || BPF_OP((*insn).code) == BPF_CALL
                || BPF_OP((*insn).code) == BPF_EXIT
            {
                (*e).dst = bb_next(bb);
                (*e).flags |= EDGE_FLAG_FALLTHROUGH;
                list_add_tail(&mut (*e).l, &mut (*bb).e_succs);
                bb = bb_next(bb);
                continue;
            } else if BPF_OP((*insn).code) == BPF_JA {
                (*e).dst = func_search_bb_with_head(func, insn.offset((*insn).off as isize + 1));
                (*e).flags |= EDGE_FLAG_JUMP;
                list_add_tail(&mut (*e).l, &mut (*bb).e_succs);
                bb = bb_next(bb);
                continue;
            }

            (*e).dst = bb_next(bb);
            (*e).flags |= EDGE_FLAG_FALLTHROUGH;
            list_add_tail(&mut (*e).l, &mut (*bb).e_succs);

            e = new_edge(bb, ptr::null_mut(), EDGE_FLAG_JUMP);
            if e.is_null() {
                return true;
            }
            (*e).src = bb;
            (*e).dst = func_search_bb_with_head(func, insn.offset((*insn).off as isize + 1));
            list_add_tail(&mut (*e).l, &mut (*bb).e_succs);
            bb = bb_next(bb);
        }

        false
    }
}

unsafe fn cfg_build(cfg: *mut cfg, insn: *mut bpf_insn, len: c_uint) -> bool {
    unsafe {
        let cnt: c_int = (len as usize / size_of::<bpf_insn>()) as c_int;
        INIT_LIST_HEAD(&mut (*cfg).funcs);

        if cfg_partition_funcs(cfg, insn, insn.add(cnt as usize)) {
            return true;
        }

        let mut pos = (*cfg).funcs.next;
        while pos != &mut (*cfg).funcs {
            let func = container_of_func_node(pos);
            if func_partition_bb(func) || func_add_special_bb(func) {
                return true;
            }

            if func_add_bb_edges(func) {
                return true;
            }
            pos = (*pos).next;
        }

        false
    }
}

unsafe fn cfg_destroy(cfg: *mut cfg) {
    unsafe {
        let mut func_pos = (*cfg).funcs.next;
        while func_pos != &mut (*cfg).funcs {
            let func = container_of_func_node(func_pos);
            let func2 = (*func_pos).next;

            let mut bb_pos = (*func).bbs.next;
            while bb_pos != &mut (*func).bbs {
                let bb = container_of_bb_node(bb_pos);
                let bb2 = (*bb_pos).next;

                let mut e_pos = (*bb).e_prevs.next;
                while e_pos != &mut (*bb).e_prevs {
                    let e = container_of_edge_node(e_pos);
                    let e2 = (*e_pos).next;
                    list_del(&mut (*e).l);
                    free(e as *mut c_void);
                    e_pos = e2;
                }

                e_pos = (*bb).e_succs.next;
                while e_pos != &mut (*bb).e_succs {
                    let e = container_of_edge_node(e_pos);
                    let e2 = (*e_pos).next;
                    list_del(&mut (*e).l);
                    free(e as *mut c_void);
                    e_pos = e2;
                }

                list_del(&mut (*bb).l);
                free(bb as *mut c_void);
                bb_pos = bb2;
            }

            list_del(&mut (*func).l);
            free(func as *mut c_void);
            func_pos = func2;
        }
    }
}

unsafe fn draw_bb_node(
    func: *mut func_node,
    bb: *mut bb_node,
    dd: *mut dump_data,
    opcodes: bool,
    linum: bool,
) {
    unsafe {
        let shape: *const c_char;

        if (*bb).idx == ENTRY_BLOCK_INDEX || (*bb).idx == EXIT_BLOCK_INDEX {
            shape = c"Mdiamond".as_ptr();
        } else {
            shape = c"record".as_ptr();
        }

        printf(
            c"\tfn_%d_bb_%d [shape=%s,style=filled,label=\"".as_ptr(),
            (*func).idx,
            (*bb).idx,
            shape,
        );

        if (*bb).idx == ENTRY_BLOCK_INDEX {
            printf(c"ENTRY".as_ptr());
        } else if (*bb).idx == EXIT_BLOCK_INDEX {
            printf(c"EXIT".as_ptr());
        } else {
            let start_idx: c_uint;
            printf(c"{\\\n".as_ptr());
            start_idx = (*bb).head.offset_from((*func).start) as c_uint;
            dump_xlated_for_graph(dd, (*bb).head, (*bb).tail, start_idx, opcodes, linum);
            printf(c"}".as_ptr());
        }

        printf(c"\"];\n\n".as_ptr());
    }
}

unsafe fn draw_bb_succ_edges(func: *mut func_node, bb: *mut bb_node) {
    unsafe {
        let style: *const c_char = c"\"solid,bold\"".as_ptr();
        let color: *const c_char = c"black".as_ptr();
        let func_idx: c_int = (*func).idx;
        let weight: c_int = 10;

        if list_empty(&(*bb).e_succs) {
            return;
        }

        let mut pos = (*bb).e_succs.next;
        while pos != &mut (*bb).e_succs {
            let e = container_of_edge_node(pos);
            printf(
                c"\tfn_%d_bb_%d:s -> fn_%d_bb_%d:n [style=%s, color=%s, weight=%d, constraint=true".as_ptr(),
                func_idx,
                (*(*e).src).idx,
                func_idx,
                (*(*e).dst).idx,
                style,
                color,
                weight,
            );
            printf(c"];\n".as_ptr());
            pos = (*pos).next;
        }
    }
}

unsafe fn func_output_bb_def(
    func: *mut func_node,
    dd: *mut dump_data,
    opcodes: bool,
    linum: bool,
) {
    unsafe {
        let mut pos = (*func).bbs.next;
        while pos != &mut (*func).bbs {
            let bb = container_of_bb_node(pos);
            draw_bb_node(func, bb, dd, opcodes, linum);
            pos = (*pos).next;
        }
    }
}

unsafe fn func_output_edges(func: *mut func_node) {
    unsafe {
        let func_idx: c_int = (*func).idx;
        let mut pos = (*func).bbs.next;

        while pos != &mut (*func).bbs {
            let bb = container_of_bb_node(pos);
            draw_bb_succ_edges(func, bb);
            pos = (*pos).next;
        }

        /* Add an invisible edge from ENTRY to EXIT, this is to
         * improve the graph layout.
         */
        printf(
            c"\tfn_%d_bb_%d:s -> fn_%d_bb_%d:n [style=\"invis\", constraint=true];\n".as_ptr(),
            func_idx,
            ENTRY_BLOCK_INDEX,
            func_idx,
            EXIT_BLOCK_INDEX,
        );
    }
}

unsafe fn cfg_dump(cfg: *mut cfg, dd: *mut dump_data, opcodes: bool, linum: bool) {
    unsafe {
        printf(c"digraph \"DOT graph for eBPF program\" {\n".as_ptr());
        let mut pos = (*cfg).funcs.next;
        while pos != &mut (*cfg).funcs {
            let func = container_of_func_node(pos);
            printf(
                c"subgraph \"cluster_%d\" {\n\tstyle=\"dashed\";\n\tcolor=\"black\";\n\tlabel=\"func_%d ()\";\n".as_ptr(),
                (*func).idx,
                (*func).idx,
            );
            func_output_bb_def(func, dd, opcodes, linum);
            func_output_edges(func);
            printf(c"}\n".as_ptr());
            pos = (*pos).next;
        }
        printf(c"}\n".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dump_xlated_cfg(
    dd: *mut dump_data,
    buf: *mut c_void,
    len: c_uint,
    opcodes: bool,
    linum: bool,
) {
    unsafe {
        let insn = buf as *mut bpf_insn;
        let mut cfg: cfg = core::mem::zeroed();

        memset(
            &mut cfg as *mut cfg as *mut c_void,
            0,
            size_of::<cfg>(),
        );
        if cfg_build(&mut cfg, insn, len) {
            return;
        }

        cfg_dump(&mut cfg, dd, opcodes, linum);

        cfg_destroy(&mut cfg);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
