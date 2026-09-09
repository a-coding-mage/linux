// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/sched/act_meta_prio.c IFE skb->priority metadata module
 *
 * copyright Jamal Hadi Salim (2015)
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn skbprio_check(skb: *mut sk_buff, e: *mut tcf_meta_info) -> i32 {
    unsafe { ife_check_meta_u32((*skb).priority, e) }
}

unsafe fn skbprio_encode(
    skb: *mut sk_buff,
    skbdata: *mut core::ffi::c_void,
    e: *mut tcf_meta_info,
) -> i32 {
    let ifeprio: u32 = unsafe { (*skb).priority };

    unsafe { ife_encode_meta_u32(ifeprio, skbdata, e) }
}

unsafe fn skbprio_decode(skb: *mut sk_buff, data: *mut core::ffi::c_void, _len: u16) -> i32 {
    let ifeprio: u32 = unsafe { *(data as *const u32) };

    unsafe {
        (*skb).priority = u32::from_be(ifeprio);
    }
    0
}

static mut ife_prio_ops: tcf_meta_ops = tcf_meta_ops {
    metaid: IFE_META_PRIO,
    metatype: NLA_U32,
    name: "skbprio",
    synopsis: "skb prio metadata",
    check_presence: Some(skbprio_check),
    encode: Some(skbprio_encode),
    decode: Some(skbprio_decode),
    get: Some(ife_get_meta_u32),
    alloc: Some(ife_alloc_meta_u32),
    owner: THIS_MODULE,
};

unsafe fn ifeprio_init_module() -> i32 {
    unsafe { register_ife_op(&raw mut ife_prio_ops) }
}

unsafe fn ifeprio_cleanup_module() {
    unsafe { unregister_ife_op(&raw mut ife_prio_ops) };
}

// module_init(ifeprio_init_module);
// module_exit(ifeprio_cleanup_module);

// MODULE_AUTHOR("Jamal Hadi Salim(2015)");
// MODULE_DESCRIPTION("Inter-FE skb prio metadata action");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS_IFE_META("skbprio");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
