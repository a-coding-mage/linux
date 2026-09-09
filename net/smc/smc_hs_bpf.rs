// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Shared Memory Communications over RDMA (SMC-R) and RoCE
 *
 *  Generic hook for SMC handshake flow.
 *
 *  Copyright IBM Corp. 2016
 *  Copyright (c) 2025, Alibaba Inc.
 *
 *  Author: D. Wythe <alibuda@linux.alibaba.com>
 */

// Linux BPF, BTF, RCU-list, and smc_hs_bpf declarations are supplied by the
// surrounding kernel translation unit.

static mut smc_hs_ctrl_list_lock: SpinLock = SpinLock::new();
static mut smc_hs_ctrl_list: ListHead = ListHead::new();

unsafe fn smc_hs_ctrl_reg(ctrl: *mut smc_hs_ctrl) -> c_int {
    let mut ret: c_int = 0;

    spin_lock(&raw mut smc_hs_ctrl_list_lock);
    // already exist or duplicate name
    if !smc_hs_ctrl_find_by_name((*ctrl).name.as_ptr()).is_null() {
        ret = -EEXIST;
    } else {
        list_add_tail_rcu(&raw mut (*ctrl).list, &raw mut smc_hs_ctrl_list);
    }
    spin_unlock(&raw mut smc_hs_ctrl_list_lock);
    ret
}

unsafe fn smc_hs_ctrl_unreg(ctrl: *mut smc_hs_ctrl) {
    spin_lock(&raw mut smc_hs_ctrl_list_lock);
    list_del_rcu(&raw mut (*ctrl).list);
    spin_unlock(&raw mut smc_hs_ctrl_list_lock);

    // Ensure that all readers to complete
    synchronize_rcu();
}

unsafe fn smc_hs_ctrl_find_by_name(name: *const c_char) -> *mut smc_hs_ctrl {
    let mut ctrl: *mut smc_hs_ctrl = core::ptr::null_mut();

    list_for_each_entry_rcu!(ctrl, &raw mut smc_hs_ctrl_list, list, {
        if strcmp((*ctrl).name.as_ptr(), name) == 0 {
            return ctrl;
        }
    });
    core::ptr::null_mut()
}

unsafe extern "C" fn __smc_bpf_stub_set_tcp_option(_tp: *mut tcp_sock) -> c_int { 1 }
unsafe extern "C" fn __smc_bpf_stub_set_tcp_option_cond(
    _tp: *const tcp_sock,
    _ireq: *mut inet_request_sock,
) -> c_int {
    1
}

static mut __smc_bpf_hs_ctrl: smc_hs_ctrl = smc_hs_ctrl {
    syn_option: Some(__smc_bpf_stub_set_tcp_option),
    synack_option: Some(__smc_bpf_stub_set_tcp_option_cond),
    ..smc_hs_ctrl::default()
};

unsafe fn smc_bpf_hs_ctrl_init(_btf: *mut btf) -> c_int { 0 }

unsafe fn smc_bpf_hs_ctrl_reg(kdata: *mut c_void, link: *mut bpf_link) -> c_int {
    if !link.is_null() {
        return -EOPNOTSUPP;
    }
    smc_hs_ctrl_reg(kdata.cast())
}

unsafe fn smc_bpf_hs_ctrl_unreg(kdata: *mut c_void, _link: *mut bpf_link) {
    smc_hs_ctrl_unreg(kdata.cast());
}

unsafe fn smc_bpf_hs_ctrl_init_member(
    t: *const btf_type,
    member: *const btf_member,
    kdata: *mut c_void,
    udata: *const c_void,
) -> c_int {
    let u_ctrl = udata.cast::<smc_hs_ctrl>();
    let k_ctrl = kdata.cast::<smc_hs_ctrl>();
    let moff: u32 = __btf_member_bit_offset(t, member) / 8;

    match moff as usize {
        offset_of!(smc_hs_ctrl, name) => {
            if bpf_obj_name_cpy((*k_ctrl).name.as_mut_ptr(), (*u_ctrl).name.as_ptr(),
                                core::mem::size_of_val(&(*u_ctrl).name)) <= 0 {
                return -EINVAL;
            }
            1
        }
        offset_of!(smc_hs_ctrl, flags) => {
            if (*u_ctrl).flags & !SMC_HS_CTRL_ALL_FLAGS != 0 {
                return -EINVAL;
            }
            (*k_ctrl).flags = (*u_ctrl).flags;
            1
        }
        _ => 0,
    }
}

unsafe fn bpf_smc_hs_func_proto(
    func_id: bpf_func_id,
    prog: *const bpf_prog,
) -> *const bpf_func_proto {
    bpf_base_func_proto(func_id, prog)
}

static smc_bpf_verifier_ops: bpf_verifier_ops = bpf_verifier_ops {
    get_func_proto: Some(bpf_smc_hs_func_proto),
    is_valid_access: Some(bpf_tracing_btf_ctx_access),
    ..bpf_verifier_ops::default()
};

static mut bpf_smc_hs_ctrl_ops: bpf_struct_ops = bpf_struct_ops {
    name: c"smc_hs_ctrl".as_ptr(),
    init: Some(smc_bpf_hs_ctrl_init),
    reg: Some(smc_bpf_hs_ctrl_reg),
    unreg: Some(smc_bpf_hs_ctrl_unreg),
    cfi_stubs: &raw mut __smc_bpf_hs_ctrl,
    verifier_ops: &smc_bpf_verifier_ops,
    init_member: Some(smc_bpf_hs_ctrl_init_member),
    owner: THIS_MODULE,
    ..bpf_struct_ops::default()
};

pub unsafe fn bpf_smc_hs_ctrl_init() -> c_int {
    register_bpf_struct_ops(&raw mut bpf_smc_hs_ctrl_ops, smc_hs_ctrl::TYPE_INFO)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
