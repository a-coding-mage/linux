// SPDX-License-Identifier: GPL-2.0
// C dependencies and kernel/BPF macros are supplied by the surrounding build.

use core::ffi::c_void;

const QDISC_OPS_KF_COMMON: u32 = 0;
const QDISC_OPS_KF_ENQUEUE: u32 = 1 << 0;
const QDISC_OPS_KF_DEQUEUE: u32 = 1 << 1;

#[repr(C)]
pub struct bpf_sched_data {
    pub watchdog: qdisc_watchdog,
}

#[repr(C)]
pub struct bpf_sk_buff_ptr {
    pub skb: *mut sk_buff,
}

extern "C" {
    static mut bpf_Qdisc_ops: bpf_struct_ops;
    static bpf_qdisc_ids: [u32; 1];
    static bpf_sk_buff_ids: [u32; 1];
    static bpf_sk_buff_ptr_ids: [u32; 1];
    static bpf_qdisc_init_prologue_ids: [u32; 1];
    static bpf_qdisc_reset_destroy_epilogue_ids: [u32; 1];
    static bpf_sk_buff_dtor_ids: [u32; 1];

    fn btf_ctx_arg_idx(btf: *mut btf, proto: *mut btf_func_proto, off: i32) -> u32;
    fn bpf_tracing_btf_ctx_access(off: i32, size: i32, ty: bpf_access_type,
                                  prog: *const bpf_prog, info: *mut bpf_insn_access_aux) -> bool;
    fn btf_type_by_id(btf: *mut btf, id: u32) -> *const btf_type;
    fn btf_name_by_offset(btf: *mut btf, off: u32) -> *const i8;
    fn bpf_log(log: *mut bpf_verifier_log, fmt: *const i8, ...);
    fn bpf_base_func_proto() -> *const bpf_func_proto;
    fn __btf_member_bit_offset(t: *const btf_type, member: *const btf_member) -> u32;
    fn bpf_obj_name_cpy(dst: *mut i8, src: *const i8, len: usize) -> i32;
    fn register_qdisc(ops: *mut Qdisc_ops) -> i32;
    fn unregister_qdisc(ops: *mut Qdisc_ops);
    fn qdisc_peek_dequeued(sch: *mut Qdisc) -> *mut sk_buff;
    fn qdisc_watchdog_init(watchdog: *mut qdisc_watchdog, sch: *mut Qdisc);
    fn qdisc_watchdog_cancel(watchdog: *mut qdisc_watchdog);
    fn qdisc_watchdog_schedule_range_ns(watchdog: *mut qdisc_watchdog, expire: u64, delta_ns: u64);
    fn qdisc_priv(sch: *mut Qdisc) -> *mut c_void;
    fn qdisc_dev(sch: *mut Qdisc) -> *mut net_device;
    fn qdisc_lookup(dev: *mut net_device, handle: u32) -> *mut Qdisc;
    fn skb_get_hash(skb: *mut sk_buff) -> u32;
    fn kfree_skb(skb: *mut sk_buff);
    fn bstats_update(stats: *mut c_void, skb: *const sk_buff);
    fn register_btf_kfunc_id_set(prog_type: u32, set: *const btf_kfunc_id_set) -> i32;
    fn register_btf_id_dtor_kfuncs(dtors: *const btf_id_dtor_kfunc, count: usize, owner: *mut c_void) -> i32;
    fn register_bpf_struct_ops(ops: *mut bpf_struct_ops, type_: *mut Qdisc_ops) -> i32;
    fn NL_SET_ERR_MSG(extack: *mut netlink_ext_ack, msg: *const i8);
    fn __qdisc_drop(skb: *mut sk_buff, to_free: *mut *mut sk_buff);
}

unsafe fn bpf_qdisc_init(_btf: *mut btf) -> i32 { 0 }

unsafe fn bpf_qdisc_is_valid_access(off: i32, size: i32, ty: bpf_access_type,
                                    prog: *const bpf_prog, info: *mut bpf_insn_access_aux) -> bool {
    let btf = (*(*prog).aux).attach_btf;
    let arg = btf_ctx_arg_idx(btf, (*(*prog).aux).attach_func_proto, off);
    if (*(*prog).aux).attach_st_ops_member_off == offsetof!(Qdisc_ops, enqueue) && arg == 2 && ty == BPF_READ {
        (*info).reg_type = PTR_TO_BTF_ID | PTR_TRUSTED;
        (*info).btf = btf;
        (*info).btf_id = bpf_sk_buff_ptr_ids[0];
        return true;
    }
    bpf_tracing_btf_ctx_access(off, size, ty, prog, info)
}

unsafe fn bpf_qdisc_qdisc_access(_log: *mut bpf_verifier_log, _reg: *const bpf_reg_state,
                                 off: i32, end: *mut usize) -> i32 {
    if off == offsetof!(Qdisc, limit) { *end = offsetofend!(Qdisc, limit); }
    else if off == offsetof!(Qdisc, q) + offsetof!(qdisc_skb_head, qlen) { *end = offsetof!(Qdisc, q) + offsetofend!(qdisc_skb_head, qlen); }
    else if off >= offsetof!(Qdisc, qstats) && off < offsetofend!(Qdisc, qstats) { *end = offsetofend!(Qdisc, qstats); }
    else { return -EACCES; }
    0
}

unsafe fn bpf_qdisc_sk_buff_access(_log: *mut bpf_verifier_log, _reg: *const bpf_reg_state,
                                   off: i32, end: *mut usize) -> i32 {
    if off == offsetof!(sk_buff, tstamp) { *end = offsetofend!(sk_buff, tstamp); }
    else if off >= offsetof!(sk_buff, cb) + offsetof!(qdisc_skb_cb, data[0]) && off <= offsetof!(sk_buff, cb) + offsetof!(qdisc_skb_cb, data[QDISC_CB_PRIV_LEN - 1]) { *end = offsetof!(sk_buff, cb) + offsetofend!(qdisc_skb_cb, data[QDISC_CB_PRIV_LEN - 1]); }
    else { return -EACCES; }
    0
}

unsafe fn bpf_qdisc_btf_struct_access(log: *mut bpf_verifier_log, reg: *const bpf_reg_state, off: i32, size: i32) -> i32 {
    let skbt = btf_type_by_id((*reg).btf, bpf_sk_buff_ids[0]);
    let qdisct = btf_type_by_id((*reg).btf, bpf_qdisc_ids[0]);
    let t = btf_type_by_id((*reg).btf, (*reg).btf_id);
    let mut end = 0usize;
    let err = if t == skbt { bpf_qdisc_sk_buff_access(log, reg, off, &mut end) } else if t == qdisct { bpf_qdisc_qdisc_access(log, reg, off, &mut end) } else { bpf_log(log, c"only read is supported\n".as_ptr(),); return -EACCES; };
    if err != 0 || (off as usize).saturating_add(size as usize) > end { return -EACCES; }
    0
}

unsafe fn bpf_qdisc_gen_prologue(insn_buf: *mut bpf_insn, _direct_write: bool, prog: *const bpf_prog) -> i32 {
    if (*(*prog).aux).attach_st_ops_member_off != offsetof!(Qdisc_ops, init) { return 0; }
    let mut insn = insn_buf;
    *insn = BPF_MOV64_REG(BPF_REG_6, BPF_REG_1); insn = insn.add(1);
    *insn = BPF_LDX_MEM(BPF_DW, BPF_REG_2, BPF_REG_1, 16); insn = insn.add(1);
    *insn = BPF_LDX_MEM(BPF_DW, BPF_REG_1, BPF_REG_1, 0); insn = insn.add(1);
    *insn = BPF_CALL_KFUNC(0, bpf_qdisc_init_prologue_ids[0]); insn = insn.add(1);
    *insn = BPF_JMP_IMM(BPF_JEQ, BPF_REG_0, 0, 1); insn = insn.add(1);
    *insn = BPF_EXIT_INSN(); insn = insn.add(1);
    *insn = BPF_MOV64_REG(BPF_REG_1, BPF_REG_6); insn = insn.add(1);
    *insn = (*prog).insnsi[0];
    insn.offset_from(insn_buf) as i32 + 1
}

unsafe fn bpf_qdisc_gen_epilogue(insn_buf: *mut bpf_insn, prog: *const bpf_prog, ctx_stack_off: i16) -> i32 {
    let member = (*(*prog).aux).attach_st_ops_member_off;
    if member != offsetof!(Qdisc_ops, reset) && member != offsetof!(Qdisc_ops, destroy) { return 0; }
    let mut insn = insn_buf;
    *insn = BPF_LDX_MEM(BPF_DW, BPF_REG_1, BPF_REG_FP, ctx_stack_off); insn = insn.add(1);
    *insn = BPF_LDX_MEM(BPF_DW, BPF_REG_1, BPF_REG_1, 0); insn = insn.add(1);
    *insn = BPF_CALL_KFUNC(0, bpf_qdisc_reset_destroy_epilogue_ids[0]); insn = insn.add(1);
    *insn = BPF_EXIT_INSN();
    insn.offset_from(insn_buf) as i32 + 1
}

pub unsafe extern "C" fn bpf_skb_get_hash(skb: *mut sk_buff) -> u32 { skb_get_hash(skb) }
pub unsafe extern "C" fn bpf_kfree_skb(skb: *mut sk_buff) { kfree_skb(skb); }
pub unsafe extern "C" fn bpf_kfree_skb_dtor(skb: *mut c_void) { bpf_kfree_skb(skb as *mut sk_buff); }
pub unsafe extern "C" fn bpf_qdisc_skb_drop(skb: *mut sk_buff, to_free_list: *mut bpf_sk_buff_ptr) { __qdisc_drop(skb, to_free_list as *mut *mut sk_buff); }

pub unsafe extern "C" fn bpf_qdisc_watchdog_schedule(sch: *mut Qdisc, expire: u64, delta_ns: u64) {
    let q = qdisc_priv(sch) as *mut bpf_sched_data;
    qdisc_watchdog_schedule_range_ns(&mut (*q).watchdog, expire, delta_ns);
}

pub unsafe extern "C" fn bpf_qdisc_init_prologue(sch: *mut Qdisc, extack: *mut netlink_ext_ack) -> i32 {
    let q = qdisc_priv(sch) as *mut bpf_sched_data;
    let dev = qdisc_dev(sch);
    qdisc_watchdog_init(&mut (*q).watchdog, sch);
    if (*sch).parent != TC_H_ROOT {
        let p = qdisc_lookup(dev, TC_H_MAJ((*sch).parent));
        if !p.is_null() && ((*p).flags & TCQ_F_MQROOT) == 0 { NL_SET_ERR_MSG(extack, c"BPF qdisc only supported on root or mq\0".as_ptr()); return -EINVAL; }
    }
    0
}

pub unsafe extern "C" fn bpf_qdisc_reset_destroy_epilogue(sch: *mut Qdisc) {
    let q = qdisc_priv(sch) as *mut bpf_sched_data;
    qdisc_watchdog_cancel(&mut (*q).watchdog);
}
pub unsafe extern "C" fn bpf_qdisc_bstats_update(sch: *mut Qdisc, skb: *const sk_buff) { bstats_update(&mut (*sch).bstats as *mut _ as *mut c_void, skb); }

unsafe fn bpf_qdisc_reg(kdata: *mut c_void, _link: *mut bpf_link) -> i32 { register_qdisc(kdata as *mut Qdisc_ops) }
unsafe fn bpf_qdisc_unreg(kdata: *mut c_void, _link: *mut bpf_link) { unregister_qdisc(kdata as *mut Qdisc_ops); }
unsafe fn bpf_qdisc_validate(kdata: *mut c_void) -> i32 {
    let ops = kdata as *mut Qdisc_ops;
    if (*ops).enqueue.is_none() || (*ops).dequeue.is_none() || (*ops).init.is_none() || (*ops).reset.is_none() || (*ops).destroy.is_none() { return -EINVAL; }
    0
}

unsafe fn bpf_qdisc_init_member(t: *const btf_type, member: *const btf_member, kdata: *mut c_void, udata: *const c_void) -> i32 {
    let uqdisc_ops = udata as *const Qdisc_ops;
    let qdisc_ops = kdata as *mut Qdisc_ops;
    let moff = __btf_member_bit_offset(t, member) / 8;
    if moff == offsetof!(Qdisc_ops, priv_size) {
        if (*uqdisc_ops).priv_size != 0 { return -EINVAL; }
        (*qdisc_ops).priv_size = core::mem::size_of::<bpf_sched_data>();
        return 1;
    }
    if moff == offsetof!(Qdisc_ops, peek) { (*qdisc_ops).peek = Some(qdisc_peek_dequeued); return 0; }
    if moff == offsetof!(Qdisc_ops, id) {
        if bpf_obj_name_cpy((*qdisc_ops).id.as_mut_ptr(), (*uqdisc_ops).id.as_ptr(), (*qdisc_ops).id.len()) <= 0 { return -EINVAL; }
        return 1;
    }
    0
}

unsafe fn bpf_qdisc_kfunc_filter(prog: *const bpf_prog, kfunc_id: u32) -> i32 {
    // The BTF ID-set membership and per-Qdisc operation flags are supplied by bindings.
    if !btf_id_set8_contains(&qdisc_kfunc_ids, kfunc_id) { return 0; }
    if (*(*prog).aux).st_ops != &raw mut bpf_Qdisc_ops { return -EACCES; }
    let moff = (*(*prog).aux).attach_st_ops_member_off;
    let flags = qdisc_ops_context_flags[QDISC_MOFF_IDX(moff)];
    if (flags & QDISC_OPS_KF_ENQUEUE) != 0 && btf_id_set_contains(&qdisc_enqueue_kfunc_set, kfunc_id) { return 0; }
    if (flags & QDISC_OPS_KF_DEQUEUE) != 0 && btf_id_set_contains(&qdisc_dequeue_kfunc_set, kfunc_id) { return 0; }
    if btf_id_set_contains(&qdisc_common_kfunc_set, kfunc_id) { return 0; }
    -EACCES
}

unsafe fn bpf_qdisc_kfunc_init() -> i32 {
    let skb_kfunc_dtors = [btf_id_dtor_kfunc { btf_id: bpf_sk_buff_ids[0], kfunc_btf_id: bpf_sk_buff_dtor_ids[0] }];
    let mut ret = register_btf_kfunc_id_set(BPF_PROG_TYPE_STRUCT_OPS, &bpf_qdisc_kfunc_set);
    if ret == 0 { ret = register_btf_id_dtor_kfuncs(skb_kfunc_dtors.as_ptr(), skb_kfunc_dtors.len(), THIS_MODULE); }
    if ret == 0 { ret = register_bpf_struct_ops(&raw mut bpf_Qdisc_ops, &raw mut Qdisc_ops); }
    ret
}

static qdisc_ops_context_flags: [u32; 5] = [QDISC_OPS_KF_COMMON; 5];
static qdisc_kfunc_ids: btf_id_set8 = btf_id_set8 {};
static qdisc_common_kfunc_set: btf_id_set = btf_id_set {};
static qdisc_enqueue_kfunc_set: btf_id_set = btf_id_set {};
static qdisc_dequeue_kfunc_set: btf_id_set = btf_id_set {};
static bpf_qdisc_kfunc_set: btf_kfunc_id_set = btf_kfunc_id_set { owner: THIS_MODULE, set: &qdisc_kfunc_ids, filter: Some(bpf_qdisc_kfunc_filter) };

unsafe extern "C" fn Qdisc_ops__enqueue(_skb: *mut sk_buff, _sch: *mut Qdisc, _to_free: *mut *mut sk_buff) -> i32 { 0 }
unsafe extern "C" fn Qdisc_ops__dequeue(_sch: *mut Qdisc) -> *mut sk_buff { core::ptr::null_mut() }
unsafe extern "C" fn Qdisc_ops__init(_sch: *mut Qdisc, _arg: *mut nlattr, _extack: *mut netlink_ext_ack) -> i32 { 0 }
unsafe extern "C" fn Qdisc_ops__reset(_sch: *mut Qdisc) {}
unsafe extern "C" fn Qdisc_ops__destroy(_sch: *mut Qdisc) {}

// BTF_ID_LIST, BTF_KFUNCS, BTF_SET, struct-ops tables, and late_initcall registrations
// are emitted by the kernel's surrounding Rust bindings/build macros. The local C
// declarations above preserve each implementation and externally visible callback.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
