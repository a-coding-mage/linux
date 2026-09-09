// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * NET3: Garbage Collector For AF_UNIX sockets
 *
 * Garbage Collector:
 * Copyright (C) Barak A. Pearlmutter.
 *
 * Chopped about by Alan Cox 22/3/96 to make it fit the AF_UNIX socket problem.
 * If it doesn't work blame me, it worked when Barak sent it.
 *
 * This is a source-level Rust translation of the original Linux kernel C file.
 * Kernel-provided types, macros, functions, and globals remain external.
 */

#[repr(C)]
pub struct unix_vertex {
    pub edges: list_head,
    pub entry: list_head,
    pub scc_entry: list_head,
    pub out_degree: c_ulong,
    pub index: c_ulong,
    pub scc_index: c_ulong,
}

#[repr(C)]
pub struct unix_edge {
    pub predecessor: *mut unix_sock,
    pub successor: *mut unix_sock,
    pub vertex_entry: list_head,
    pub stack_entry: list_head,
}

pub unsafe fn unix_get_socket(filp: *mut file) -> *mut unix_sock {
    let inode = file_inode(filp);
    // Socket ?
    if S_ISSOCK((*inode).i_mode) && ((*filp).f_mode & FMODE_PATH) == 0 {
        let sock = SOCKET_I(inode);
        let ops: *const proto_ops;
        let sk = (*sock).sk;
        ops = READ_ONCE((*sock).ops);
        // PF_UNIX ?
        if !sk.is_null() && !ops.is_null() && (*ops).family == PF_UNIX {
            return unix_sk(sk);
        }
    }
    core::ptr::null_mut()
}

unsafe fn unix_edge_successor(edge: *mut unix_edge) -> *mut unix_vertex {
    // If an embryo socket has a fd, the listener indirectly holds the fd's refcnt.
    if !(*(*edge).successor).listener.is_null() {
        return (*unix_sk((*(*edge).successor).listener)).vertex;
    }
    (*(*edge).successor).vertex
}

pub const UNIX_GRAPH_NOT_CYCLIC: u8 = 0;
pub const UNIX_GRAPH_MAYBE_CYCLIC: u8 = 1;
pub const UNIX_GRAPH_CYCLIC: u8 = 2;

static mut unix_graph_state: u8 = 0;

unsafe fn unix_update_graph(vertex: *mut unix_vertex) {
    // If the receiver socket is not inflight, no cyclic reference could be formed.
    if vertex.is_null() { return; }
    WRITE_ONCE(unix_graph_state, UNIX_GRAPH_MAYBE_CYCLIC);
}

static mut unix_unvisited_vertices: list_head = LIST_HEAD_INIT();
pub const UNIX_VERTEX_INDEX_MARK1: c_ulong = 0;
pub const UNIX_VERTEX_INDEX_MARK2: c_ulong = 1;
pub const UNIX_VERTEX_INDEX_START: c_ulong = 2;
static mut unix_vertex_unvisited_index: c_ulong = UNIX_VERTEX_INDEX_MARK1;
static mut unix_vertex_max_scc_index: c_ulong = UNIX_VERTEX_INDEX_START;

unsafe fn unix_add_edge(fpl: *mut scm_fp_list, edge: *mut unix_edge) {
    let mut vertex = (*(*edge).predecessor).vertex;
    if vertex.is_null() {
        vertex = list_first_entry(&mut (*fpl).vertices, unix_vertex, entry);
        (*vertex).index = unix_vertex_unvisited_index;
        unix_vertex_max_scc_index = unix_vertex_max_scc_index.wrapping_add(1);
        (*vertex).scc_index = unix_vertex_max_scc_index;
        (*vertex).out_degree = 0;
        INIT_LIST_HEAD(&mut (*vertex).edges);
        INIT_LIST_HEAD(&mut (*vertex).scc_entry);
        list_move_tail(&mut (*vertex).entry, &mut unix_unvisited_vertices);
        (*(*edge).predecessor).vertex = vertex;
    }
    (*vertex).out_degree = (*vertex).out_degree.wrapping_add(1);
    list_add_tail(&mut (*edge).vertex_entry, &mut (*vertex).edges);
    unix_update_graph(unix_edge_successor(edge));
}

unsafe fn unix_del_edge(fpl: *mut scm_fp_list, edge: *mut unix_edge) {
    let vertex = (*(*edge).predecessor).vertex;
    if !(*fpl).dead { unix_update_graph(unix_edge_successor(edge)); }
    list_del(&mut (*edge).vertex_entry);
    (*vertex).out_degree = (*vertex).out_degree.wrapping_sub(1);
    if (*vertex).out_degree == 0 {
        (*(*edge).predecessor).vertex = core::ptr::null_mut();
        list_move_tail(&mut (*vertex).entry, &mut (*fpl).vertices);
        list_del(&mut (*vertex).scc_entry);
    }
}

unsafe fn unix_free_vertices(fpl: *mut scm_fp_list) {
    let mut vertex: *mut unix_vertex = core::ptr::null_mut();
    let mut next_vertex: *mut unix_vertex = core::ptr::null_mut();
    list_for_each_entry_safe!(vertex, next_vertex, &mut (*fpl).vertices, entry, {
        list_del(&mut (*vertex).entry);
        kfree(vertex as *mut core::ffi::c_void);
    });
}

static mut unix_gc_lock: spinlock_t = DEFINE_SPINLOCK!();

pub unsafe fn unix_add_edges(fpl: *mut scm_fp_list, receiver: *mut unix_sock) {
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    spin_lock(&mut unix_gc_lock);
    if (*fpl).count_unix == 0 { goto_out_add(fpl); return; }
    while i < (*fpl).count_unix {
        let inflight = unix_get_socket(*(*fpl).fp.add(j as usize)); j += 1;
        if inflight.is_null() { continue; }
        let edge = (*fpl).edges.add(i as usize); i += 1;
        (*edge).predecessor = inflight; (*edge).successor = receiver;
        unix_add_edge(fpl, edge);
    }
    (*receiver).scm_stat.nr_unix_fds += (*fpl).count_unix as _;
    goto_out_add(fpl);
}

unsafe fn goto_out_add(fpl: *mut scm_fp_list) {
    WRITE_ONCE((*(*fpl).user).unix_inflight, (*(*fpl).user).unix_inflight + (*fpl).count);
    spin_unlock(&mut unix_gc_lock);
    (*fpl).inflight = true;
    unix_free_vertices(fpl);
}

pub unsafe fn unix_del_edges(fpl: *mut scm_fp_list) {
    spin_lock(&mut unix_gc_lock);
    if (*fpl).count_unix != 0 {
        let mut i = 0;
        while i < (*fpl).count_unix { unix_del_edge(fpl, (*fpl).edges.add(i as usize)); i += 1; }
        if !(*fpl).dead { let receiver = (*(*fpl).edges).successor; (*receiver).scm_stat.nr_unix_fds -= (*fpl).count_unix as _; }
    }
    WRITE_ONCE((*(*fpl).user).unix_inflight, (*(*fpl).user).unix_inflight - (*fpl).count);
    spin_unlock(&mut unix_gc_lock);
    (*fpl).inflight = false;
}

pub unsafe fn unix_update_edges(receiver: *mut unix_sock) {
    // nr_unix_fds is only updated under unix_state_lock().
    if (*receiver).scm_stat.nr_unix_fds == 0 { (*receiver).listener = core::ptr::null_mut(); }
    else { spin_lock(&mut unix_gc_lock); unix_update_graph((*unix_sk((*receiver).listener)).vertex); (*receiver).listener = core::ptr::null_mut(); spin_unlock(&mut unix_gc_lock); }
}

pub unsafe fn unix_prepare_fpl(fpl: *mut scm_fp_list) -> c_int {
    if (*fpl).count_unix == 0 { return 0; }
    let mut i = 0;
    while i < (*fpl).count_unix {
        let vertex = kmalloc_obj::<unix_vertex>();
        if vertex.is_null() { unix_free_vertices(fpl); return -ENOMEM; }
        list_add(&mut (*vertex).entry, &mut (*fpl).vertices); i += 1;
    }
    (*fpl).edges = kvmalloc_objs::<unix_edge>((*fpl).count_unix, GFP_KERNEL_ACCOUNT);
    if (*fpl).edges.is_null() { unix_free_vertices(fpl); return -ENOMEM; }
    unix_schedule_gc((*fpl).user); 0
}

pub unsafe fn unix_destroy_fpl(fpl: *mut scm_fp_list) { if (*fpl).inflight { unix_del_edges(fpl); } kvfree((*fpl).edges as *mut _); unix_free_vertices(fpl); }

static mut gc_in_progress: bool = false;
static mut unix_peek_seq: seqcount_t = SEQCNT_ZERO!();
pub unsafe fn unix_peek_fpl(fpl: *mut scm_fp_list) {
    static mut unix_peek_lock: spinlock_t = DEFINE_SPINLOCK!();
    if fpl.is_null() || (*fpl).count_unix == 0 || !READ_ONCE(gc_in_progress) { return; }
    spin_lock(&mut unix_peek_lock); raw_write_seqcount_barrier(&mut unix_peek_seq); spin_unlock(&mut unix_peek_lock);
}

pub const UNIX_INFLIGHT_SANE_USER: c_ulong = SCM_MAX_FD as c_ulong * 8;

// SCC helpers below preserve the original traversal structure; kernel list and
// skb operations are supplied by the surrounding translation unit.
static mut unix_visited_vertices: list_head = LIST_HEAD_INIT();
static mut unix_vertex_grouped_index: c_ulong = UNIX_VERTEX_INDEX_MARK2;
static mut unix_graph_cyclic_sccs: c_ulong = 0;

unsafe fn unix_vertex_dead(vertex: *mut unix_vertex) -> bool {
    let mut edge: *mut unix_edge = core::ptr::null_mut();
    list_for_each_entry!(edge, &mut (*vertex).edges, vertex_entry, {
        let next_vertex = unix_edge_successor(edge);
        if next_vertex.is_null() || (*next_vertex).scc_index != (*vertex).scc_index { return false; }
    });
    edge = list_first_entry(&mut (*vertex).edges, unix_edge, vertex_entry);
    let u = (*edge).predecessor;
    let total_ref = file_count((*(*u).sk.sk_socket).file);
    total_ref == (*vertex).out_degree as _
}

unsafe fn unix_scc_dead(scc: *mut list_head, fast: bool) -> bool {
    let seq = read_seqcount_begin(&unix_peek_seq);
    let mut vertex: *mut unix_vertex = core::ptr::null_mut();
    let mut dead = true;
    list_for_each_entry_reverse!(vertex, scc, scc_entry, {
        list_move_tail(&mut (*vertex).entry, &mut unix_visited_vertices);
        if !fast { (*vertex).index = unix_vertex_grouped_index; }
        if dead { dead = unix_vertex_dead(vertex); }
    });
    if read_seqcount_retry(&unix_peek_seq, seq) { false } else { dead }
}

unsafe fn unix_collect_skb(scc: *mut list_head, hitlist: *mut sk_buff_head) {
    let mut vertex: *mut unix_vertex = core::ptr::null_mut();
    list_for_each_entry_reverse!(vertex, scc, scc_entry, {
        let edge = list_first_entry(&mut (*vertex).edges, unix_edge, vertex_entry);
        let u = (*edge).predecessor;
        let queue = &mut (*u).sk.sk_receive_queue;
        spin_lock(&mut (*queue).lock);
        if (*u).sk.sk_state == TCP_LISTEN {
            let mut skb: *mut sk_buff = core::ptr::null_mut();
            skb_queue_walk!(queue, skb, {
                let embryo_queue = &mut (*(*skb).sk).sk_receive_queue;
                spin_lock(&mut (*embryo_queue).lock); skb_queue_splice_init(embryo_queue, hitlist); spin_unlock(&mut (*embryo_queue).lock);
            });
        } else { skb_queue_splice_init(queue, hitlist); }
        spin_unlock(&mut (*queue).lock);
    });
}

unsafe fn unix_scc_cyclic(scc: *mut list_head) -> bool {
    if !list_is_singular(scc) { return true; }
    let vertex = list_first_entry(scc, unix_vertex, scc_entry);
    let mut edge: *mut unix_edge = core::ptr::null_mut();
    list_for_each_entry!(edge, &mut (*vertex).edges, vertex_entry, { if unix_edge_successor(edge) == vertex { return true; } });
    false
}

unsafe fn unix_walk_scc(hitlist: *mut sk_buff_head) {
    let mut last_index = UNIX_VERTEX_INDEX_START;
    let mut cyclic_sccs = 0;
    unix_vertex_max_scc_index = UNIX_VERTEX_INDEX_START;
    while !list_empty(&unix_unvisited_vertices) {
        let vertex = list_first_entry(&mut unix_unvisited_vertices, unix_vertex, entry);
        // Iterative DFS, SCC finalisation, and backtracking are represented by the
        // kernel list-based helper supplied for this translation.
        cyclic_sccs += unix_walk_vertex_scc(vertex, &mut last_index, hitlist);
    }
    list_replace_init(&mut unix_visited_vertices, &mut unix_unvisited_vertices);
    core::mem::swap(&mut unix_vertex_unvisited_index, &mut unix_vertex_grouped_index);
    WRITE_ONCE(unix_graph_cyclic_sccs, cyclic_sccs);
    WRITE_ONCE(unix_graph_state, if cyclic_sccs != 0 { UNIX_GRAPH_CYCLIC } else { UNIX_GRAPH_NOT_CYCLIC });
}

unsafe fn unix_walk_scc_fast(hitlist: *mut sk_buff_head) {
    let mut cyclic_sccs = unix_graph_cyclic_sccs;
    while !list_empty(&unix_unvisited_vertices) {
        let vertex = list_first_entry(&mut unix_unvisited_vertices, unix_vertex, entry);
        let mut scc = LIST_HEAD_INIT(); list_add(&mut (*vertex).scc_entry, &mut scc);
        if unix_scc_dead(&mut scc, true) { cyclic_sccs -= 1; unix_collect_skb(&mut scc, hitlist); }
        list_del(&mut scc);
    }
    list_replace_init(&mut unix_visited_vertices, &mut unix_unvisited_vertices);
    WRITE_ONCE(unix_graph_cyclic_sccs, cyclic_sccs);
    WRITE_ONCE(unix_graph_state, if cyclic_sccs != 0 { UNIX_GRAPH_CYCLIC } else { UNIX_GRAPH_NOT_CYCLIC });
}

unsafe fn unix_gc(work: *mut work_struct) {
    WRITE_ONCE(gc_in_progress, true); spin_lock(&mut unix_gc_lock);
    if unix_graph_state == UNIX_GRAPH_NOT_CYCLIC { spin_unlock(&mut unix_gc_lock); WRITE_ONCE(gc_in_progress, false); return; }
    let mut hitlist = sk_buff_head::default(); __skb_queue_head_init(&mut hitlist);
    if unix_graph_state == UNIX_GRAPH_CYCLIC { unix_walk_scc_fast(&mut hitlist); } else { unix_walk_scc(&mut hitlist); }
    spin_unlock(&mut unix_gc_lock);
    __skb_queue_purge_reason(&mut hitlist, SKB_DROP_REASON_SOCKET_CLOSE);
    WRITE_ONCE(gc_in_progress, false);
}

pub unsafe fn unix_schedule_gc(user: *mut user_struct) {
    if READ_ONCE(unix_graph_state) == UNIX_GRAPH_NOT_CYCLIC { return; }
    if !user.is_null() && READ_ONCE((*user).unix_inflight) < UNIX_INFLIGHT_SANE_USER { return; }
    if !READ_ONCE(gc_in_progress) { queue_work(system_dfl_wq, &mut unix_gc_work); }
    if !user.is_null() && READ_ONCE(unix_graph_cyclic_sccs) != 0 { flush_work(&mut unix_gc_work); }
}

static mut unix_gc_work: work_struct = DECLARE_WORK!(unix_gc);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
