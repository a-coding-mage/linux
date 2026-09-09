/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Return hooking with list-based shadow stack.
 *
 * Rust translation of the C header. Types supplied by the surrounding kernel
 * environment are intentionally referenced but not defined here.
 */

use core::ffi::c_void;

pub type rethook_handler_t = unsafe extern "C" fn(
    node: *mut rethook_node,
    data: *mut c_void,
    ret_addr: usize,
    regs: *mut pt_regs,
);

/**
 * struct rethook - The rethook management data structure.
 * @data: The user-defined data storage.
 * @handler: The user-defined return hook handler.
 * @pool: The pool of struct rethook_node.
 * @ref: The reference counter.
 * @rcu: The rcu_head for deferred freeing.
 *
 * Don't embed to another data structure, because this is a self-destructive
 * data structure when all rethook_node are freed.
 */
#[repr(C)]
pub struct rethook {
    pub data: *mut c_void,
    /*
     * To avoid sparse warnings, this uses a raw function pointer with
     * __rcu, instead of rethook_handler_t. But this must be same as
     * rethook_handler_t.
     */
    pub handler: Option<rethook_handler_t>,
    pub pool: objpool_head,
    pub rcu: rcu_head,
}

/**
 * struct rethook_node - The rethook shadow-stack entry node.
 * @rcu: The rcu_head for deferred freeing.
 * @llist: The llist, linked to a struct task_struct::rethooks.
 * @rethook: The pointer to the struct rethook.
 * @ret_addr: The storage for the real return address.
 * @frame: The storage for the frame pointer.
 *
 * You can embed this to your extended data structure to store any data
 * on each entry of the shadow stack.
 */
#[repr(C)]
pub struct rethook_node {
    pub rcu: rcu_head,
    pub llist: llist_node,
    pub rethook: *mut rethook,
    pub ret_addr: usize,
    pub frame: usize,
}

extern "C" {
    pub fn rethook_alloc(
        data: *mut c_void,
        handler: rethook_handler_t,
        size: i32,
        num: i32,
    ) -> *mut rethook;
    pub fn rethook_stop(rh: *mut rethook);
    pub fn rethook_free(rh: *mut rethook);
    pub fn rethook_try_get(rh: *mut rethook) -> *mut rethook_node;
    pub fn rethook_recycle(node: *mut rethook_node);
    pub fn rethook_hook(node: *mut rethook_node, regs: *mut pt_regs, mcount: bool);
    pub fn rethook_find_ret_addr(
        tsk: *mut task_struct,
        frame: usize,
        cur: *mut *mut llist_node,
    ) -> usize;

    /* Arch dependent code must implement arch_* and trampoline code */
    pub fn arch_rethook_prepare(node: *mut rethook_node, regs: *mut pt_regs, mcount: bool);
    pub fn arch_rethook_trampoline();

    /* If the architecture needs to fixup the return address, implement it. */
    pub fn arch_rethook_fixup_return(regs: *mut pt_regs, correct_ret_addr: usize);

    /* Generic trampoline handler, arch code must prepare asm stub */
    pub fn rethook_trampoline_handler(regs: *mut pt_regs, frame: usize) -> usize;
}

/**
 * is_rethook_trampoline() - Check whether the address is rethook trampoline
 * @addr: The address to be checked
 *
 * Return true if the @addr is the rethook trampoline address.
 */
pub unsafe fn is_rethook_trampoline(addr: usize) -> bool {
    addr == dereference_symbol_descriptor(arch_rethook_trampoline as usize)
}

#[cfg(feature = "CONFIG_RETHOOK")]
extern "C" {
    pub fn rethook_flush_task(tk: *mut task_struct);
}

#[cfg(not(feature = "CONFIG_RETHOOK"))]
#[inline]
pub unsafe fn rethook_flush_task(_tsk: *mut task_struct) {
    // C equivalent: #define rethook_flush_task(tsk) do { } while (0)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
