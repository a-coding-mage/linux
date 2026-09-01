// SPDX-License-Identifier: GPL-2.0-only

/*
 * Translated from verifier.c.
 *
 * C dependencies removed from executable Rust code:
 * test_progs.h, cap_helpers.h, and the generated *.skel.h headers listed in
 * the source provide the external types, macros, and symbols declared below.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};
use core::mem::MaybeUninit;
use core::ptr;

const MAX_ENTRIES: usize = 11;
const CAP_SYS_ADMIN: c_int = 21;
const EINVAL: c_int = 22;

#[repr(C)]
pub struct test_val {
    pub index: c_uint,
    pub foo: [c_int; MAX_ENTRIES],
}

#[repr(C)]
pub struct test_loader {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

type __u64 = u64;
type skel_elf_bytes_fn = Option<unsafe extern "C" fn(*mut usize) -> *const c_void>;
type pre_execution_cb = Option<unsafe extern "C" fn(*mut bpf_object) -> c_int>;

unsafe extern "C" {
    fn cap_disable_effective(mask: c_ulonglong, old_caps: *mut __u64) -> c_int;
    fn cap_enable_effective(old_caps: __u64, new_caps: *mut c_void) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn test_loader__set_pre_execution_cb(tester: *mut test_loader, cb: pre_execution_cb);
    fn test_loader__run_subtests(
        tester: *mut test_loader,
        skel_name: *const c_char,
        elf_bytes_factory: skel_elf_bytes_fn,
    );
    fn test_loader_fini(tester: *mut test_loader);
    fn bpf_object__find_map_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_map;
    fn bpf_map__fd(map: *const bpf_map) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;

    fn PRINT_FAIL(fmt: *const c_char, ...);
    fn RUN_TESTS(skel_name: *const c_char);

    fn arena_kfunc__elf_bytes(sz: *mut usize) -> *const c_void;
    fn arena_kfunc_jit__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_align__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_and__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_arena__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_arena_large__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_arena_globals1__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_arena_globals2__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_array_access__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_async_cb_context__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_basic_stack__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_bitfield_write__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_bounds__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_bounds_deduction__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_bounds_deduction_non_const__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_bounds_mix_sign_unsign__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_bpf_get_stack__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_bpf_trap__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_bswap__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_btf_ctx_access__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_btf_unreliable_prog__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_call_large_imm__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_cfg__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_cgroup_inv_retcode__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_cgroup_skb__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_cgroup_storage__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_const__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_const_or__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_ctx__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_ctx_sk_msg__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_d_path__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_default_trusted_ptr__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_direct_packet_access__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_direct_stack_access_wraparound__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_div0__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_div_mod_bounds__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_div_overflow__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_flow_keys__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_global_subprogs__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_global_ptr_args__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_gotol__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_gotox__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_helper_access_var_len__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_helper_packet_access__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_helper_restricted__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_helper_value_access__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_int_ptr__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_iterating_callbacks__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_jeq_infer_not_null__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_jit_convergence__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_ld_ind__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_ldsx__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_leak_ptr__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_linked_scalars__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_live_stack__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_liveness_exp__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_load_acquire__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_loops1__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_lwt__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_map_in_map__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_map_lookup_refine__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_map_ptr__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_map_ptr_mixing__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_map_ret_val__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_masking__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_may_goto_1__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_may_goto_2__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_mem_size_reg__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_meta_access__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_movsx__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_mtu__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_mul__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_netfilter_ctx__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_netfilter_retcode__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_bpf_fastcall__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_or_jmp32_k__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_percpu_addr__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_precision__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_prevent_map_lookup__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_private_stack__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_ptr_to_buf__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_raw_stack__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_raw_tp_writable__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_reg_equal__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_ref_tracking__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_regalloc__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_ringbuf__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_runtime_jit__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_scalar_ids__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_sdiv__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_search_pruning__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_sock__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_sock_addr__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_sockmap_mutate__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_spill_fill__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_spin_lock__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_stack_arg__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_stack_arg_order__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_stack_ptr__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_store_release__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_subprog_insn_stats__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_subprog_precision__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_subprog_topo__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_subreg__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_tailcall__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_tailcall_jit__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_typedef__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_uninit__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_unpriv__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_unpriv_perf__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_value_adj_spill__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_value__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_value_illegal_alu__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_value_or_null__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_value_ptr_arith__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_var_off__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_vfs_accept__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_vfs_reject__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_xadd__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_xdp__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_xdp_direct_packet_access__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_bits_iter__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_set_retval__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_lsm__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_jit_inline__elf_bytes(sz: *mut usize) -> *const c_void;
    fn irq__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_ctx_ptr_param__elf_bytes(sz: *mut usize) -> *const c_void;
    fn verifier_zext__elf_bytes(sz: *mut usize) -> *const c_void;
}

unsafe fn run_tests_aux(
    skel_name: *const c_char,
    elf_bytes_factory: skel_elf_bytes_fn,
    pre_execution_cb_arg: pre_execution_cb,
) {
    let mut tester = MaybeUninit::<test_loader>::zeroed().assume_init();
    let mut old_caps: __u64 = 0;
    let mut err: c_int;

    /* test_verifier tests are executed w/o CAP_SYS_ADMIN, do the same here */
    err = cap_disable_effective(1_u64 << CAP_SYS_ADMIN, &mut old_caps);
    if err != 0 {
        PRINT_FAIL(
            b"failed to drop CAP_SYS_ADMIN: %i, %s\n\0".as_ptr() as *const c_char,
            err,
            strerror(-err),
        );
        return;
    }

    test_loader__set_pre_execution_cb(&mut tester, pre_execution_cb_arg);
    test_loader__run_subtests(&mut tester, skel_name, elf_bytes_factory);
    test_loader_fini(&mut tester);

    err = cap_enable_effective(old_caps, ptr::null_mut());
    if err != 0 {
        PRINT_FAIL(
            b"failed to restore CAP_SYS_ADMIN: %i, %s\n\0".as_ptr() as *const c_char,
            err,
            strerror(-err),
        );
    }
}

macro_rules! RUN {
    ($skel:ident, $name:expr) => {
        run_tests_aux(
            $name.as_ptr() as *const c_char,
            Some($skel),
            None,
        )
    };
}

#[no_mangle]
pub unsafe extern "C" fn test_arena_kfunc() { RUN_TESTS(b"arena_kfunc\0".as_ptr() as *const c_char); }

#[no_mangle]
pub unsafe extern "C" fn test_arena_kfunc_jit() { RUN_TESTS(b"arena_kfunc_jit\0".as_ptr() as *const c_char); }

#[no_mangle]
pub unsafe extern "C" fn test_verifier_align() { RUN!(verifier_align__elf_bytes, b"verifier_align\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_and() { RUN!(verifier_and__elf_bytes, b"verifier_and\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_arena() { RUN!(verifier_arena__elf_bytes, b"verifier_arena\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_arena_large() { RUN!(verifier_arena_large__elf_bytes, b"verifier_arena_large\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_arena_globals1() { RUN!(verifier_arena_globals1__elf_bytes, b"verifier_arena_globals1\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_arena_globals2() { RUN!(verifier_arena_globals2__elf_bytes, b"verifier_arena_globals2\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_basic_stack() { RUN!(verifier_basic_stack__elf_bytes, b"verifier_basic_stack\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_bitfield_write() { RUN!(verifier_bitfield_write__elf_bytes, b"verifier_bitfield_write\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_bounds() { RUN!(verifier_bounds__elf_bytes, b"verifier_bounds\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_bounds_deduction() { RUN!(verifier_bounds_deduction__elf_bytes, b"verifier_bounds_deduction\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_bounds_deduction_non_const() { RUN!(verifier_bounds_deduction_non_const__elf_bytes, b"verifier_bounds_deduction_non_const\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_bounds_mix_sign_unsign() { RUN!(verifier_bounds_mix_sign_unsign__elf_bytes, b"verifier_bounds_mix_sign_unsign\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_bpf_get_stack() { RUN!(verifier_bpf_get_stack__elf_bytes, b"verifier_bpf_get_stack\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_bpf_trap() { RUN!(verifier_bpf_trap__elf_bytes, b"verifier_bpf_trap\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_bswap() { RUN!(verifier_bswap__elf_bytes, b"verifier_bswap\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_btf_ctx_access() { RUN!(verifier_btf_ctx_access__elf_bytes, b"verifier_btf_ctx_access\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_btf_unreliable_prog() { RUN!(verifier_btf_unreliable_prog__elf_bytes, b"verifier_btf_unreliable_prog\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_call_large_imm() { RUN!(verifier_call_large_imm__elf_bytes, b"verifier_call_large_imm\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_cfg() { RUN!(verifier_cfg__elf_bytes, b"verifier_cfg\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_cgroup_inv_retcode() { RUN!(verifier_cgroup_inv_retcode__elf_bytes, b"verifier_cgroup_inv_retcode\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_cgroup_skb() { RUN!(verifier_cgroup_skb__elf_bytes, b"verifier_cgroup_skb\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_cgroup_storage() { RUN!(verifier_cgroup_storage__elf_bytes, b"verifier_cgroup_storage\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_const() { RUN!(verifier_const__elf_bytes, b"verifier_const\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_const_or() { RUN!(verifier_const_or__elf_bytes, b"verifier_const_or\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_ctx() { RUN_TESTS(b"verifier_ctx\0".as_ptr() as *const c_char); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_ctx_sk_msg() { RUN!(verifier_ctx_sk_msg__elf_bytes, b"verifier_ctx_sk_msg\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_d_path() { RUN!(verifier_d_path__elf_bytes, b"verifier_d_path\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_default_trusted_ptr() { RUN_TESTS(b"verifier_default_trusted_ptr\0".as_ptr() as *const c_char); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_direct_packet_access() { RUN!(verifier_direct_packet_access__elf_bytes, b"verifier_direct_packet_access\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_direct_stack_access_wraparound() { RUN!(verifier_direct_stack_access_wraparound__elf_bytes, b"verifier_direct_stack_access_wraparound\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_div0() { RUN!(verifier_div0__elf_bytes, b"verifier_div0\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_div_mod_bounds() { RUN!(verifier_div_mod_bounds__elf_bytes, b"verifier_div_mod_bounds\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_div_overflow() { RUN!(verifier_div_overflow__elf_bytes, b"verifier_div_overflow\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_flow_keys() { RUN!(verifier_flow_keys__elf_bytes, b"verifier_flow_keys\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_global_subprogs() { RUN!(verifier_global_subprogs__elf_bytes, b"verifier_global_subprogs\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_global_ptr_args() { RUN!(verifier_global_ptr_args__elf_bytes, b"verifier_global_ptr_args\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_gotol() { RUN!(verifier_gotol__elf_bytes, b"verifier_gotol\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_gotox() { RUN!(verifier_gotox__elf_bytes, b"verifier_gotox\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_helper_access_var_len() { RUN!(verifier_helper_access_var_len__elf_bytes, b"verifier_helper_access_var_len\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_helper_packet_access() { RUN!(verifier_helper_packet_access__elf_bytes, b"verifier_helper_packet_access\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_helper_restricted() { RUN!(verifier_helper_restricted__elf_bytes, b"verifier_helper_restricted\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_helper_value_access() { RUN!(verifier_helper_value_access__elf_bytes, b"verifier_helper_value_access\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_int_ptr() { RUN!(verifier_int_ptr__elf_bytes, b"verifier_int_ptr\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_iterating_callbacks() { RUN!(verifier_iterating_callbacks__elf_bytes, b"verifier_iterating_callbacks\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_jeq_infer_not_null() { RUN!(verifier_jeq_infer_not_null__elf_bytes, b"verifier_jeq_infer_not_null\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_jit_convergence() { RUN!(verifier_jit_convergence__elf_bytes, b"verifier_jit_convergence\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_load_acquire() { RUN!(verifier_load_acquire__elf_bytes, b"verifier_load_acquire\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_ld_ind() { RUN!(verifier_ld_ind__elf_bytes, b"verifier_ld_ind\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_ldsx() { RUN!(verifier_ldsx__elf_bytes, b"verifier_ldsx\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_leak_ptr() { RUN!(verifier_leak_ptr__elf_bytes, b"verifier_leak_ptr\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_linked_scalars() { RUN!(verifier_linked_scalars__elf_bytes, b"verifier_linked_scalars\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_live_stack() { RUN!(verifier_live_stack__elf_bytes, b"verifier_live_stack\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_liveness_exp() { RUN!(verifier_liveness_exp__elf_bytes, b"verifier_liveness_exp\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_loops1() { RUN!(verifier_loops1__elf_bytes, b"verifier_loops1\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_lwt() { RUN!(verifier_lwt__elf_bytes, b"verifier_lwt\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_map_in_map() { RUN!(verifier_map_in_map__elf_bytes, b"verifier_map_in_map\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_map_lookup_refine() { RUN!(verifier_map_lookup_refine__elf_bytes, b"verifier_map_lookup_refine\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_map_ptr() { RUN!(verifier_map_ptr__elf_bytes, b"verifier_map_ptr\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_map_ptr_mixing() { RUN!(verifier_map_ptr_mixing__elf_bytes, b"verifier_map_ptr_mixing\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_map_ret_val() { RUN!(verifier_map_ret_val__elf_bytes, b"verifier_map_ret_val\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_masking() { RUN!(verifier_masking__elf_bytes, b"verifier_masking\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_may_goto_1() { RUN!(verifier_may_goto_1__elf_bytes, b"verifier_may_goto_1\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_may_goto_2() { RUN!(verifier_may_goto_2__elf_bytes, b"verifier_may_goto_2\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_mem_size_reg() { RUN!(verifier_mem_size_reg__elf_bytes, b"verifier_mem_size_reg\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_meta_access() { RUN!(verifier_meta_access__elf_bytes, b"verifier_meta_access\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_movsx() { RUN!(verifier_movsx__elf_bytes, b"verifier_movsx\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_mul() { RUN!(verifier_mul__elf_bytes, b"verifier_mul\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_netfilter_ctx() { RUN!(verifier_netfilter_ctx__elf_bytes, b"verifier_netfilter_ctx\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_netfilter_retcode() { RUN!(verifier_netfilter_retcode__elf_bytes, b"verifier_netfilter_retcode\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_bpf_fastcall() { RUN!(verifier_bpf_fastcall__elf_bytes, b"verifier_bpf_fastcall\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_or_jmp32_k() { RUN!(verifier_or_jmp32_k__elf_bytes, b"verifier_or_jmp32_k\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_percpu_addr() { RUN!(verifier_percpu_addr__elf_bytes, b"verifier_percpu_addr\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_precision() { RUN!(verifier_precision__elf_bytes, b"verifier_precision\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_prevent_map_lookup() { RUN!(verifier_prevent_map_lookup__elf_bytes, b"verifier_prevent_map_lookup\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_private_stack() { RUN!(verifier_private_stack__elf_bytes, b"verifier_private_stack\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_ptr_to_buf() { RUN!(verifier_ptr_to_buf__elf_bytes, b"verifier_ptr_to_buf\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_raw_stack() { RUN!(verifier_raw_stack__elf_bytes, b"verifier_raw_stack\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_raw_tp_writable() { RUN!(verifier_raw_tp_writable__elf_bytes, b"verifier_raw_tp_writable\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_reg_equal() { RUN!(verifier_reg_equal__elf_bytes, b"verifier_reg_equal\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_ref_tracking() { RUN!(verifier_ref_tracking__elf_bytes, b"verifier_ref_tracking\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_regalloc() { RUN!(verifier_regalloc__elf_bytes, b"verifier_regalloc\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_ringbuf() { RUN!(verifier_ringbuf__elf_bytes, b"verifier_ringbuf\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_runtime_jit() { RUN!(verifier_runtime_jit__elf_bytes, b"verifier_runtime_jit\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_scalar_ids() { RUN!(verifier_scalar_ids__elf_bytes, b"verifier_scalar_ids\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_sdiv() { RUN!(verifier_sdiv__elf_bytes, b"verifier_sdiv\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_search_pruning() { RUN!(verifier_search_pruning__elf_bytes, b"verifier_search_pruning\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_sock() { RUN!(verifier_sock__elf_bytes, b"verifier_sock\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_sock_addr() { RUN!(verifier_sock_addr__elf_bytes, b"verifier_sock_addr\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_sockmap_mutate() { RUN!(verifier_sockmap_mutate__elf_bytes, b"verifier_sockmap_mutate\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_spill_fill() { RUN!(verifier_spill_fill__elf_bytes, b"verifier_spill_fill\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_spin_lock() { RUN!(verifier_spin_lock__elf_bytes, b"verifier_spin_lock\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_stack_arg() { RUN!(verifier_stack_arg__elf_bytes, b"verifier_stack_arg\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_stack_arg_order() { RUN!(verifier_stack_arg_order__elf_bytes, b"verifier_stack_arg_order\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_stack_ptr() { RUN!(verifier_stack_ptr__elf_bytes, b"verifier_stack_ptr\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_store_release() { RUN!(verifier_store_release__elf_bytes, b"verifier_store_release\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_subprog_insn_stats() { RUN!(verifier_subprog_insn_stats__elf_bytes, b"verifier_subprog_insn_stats\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_subprog_precision() { RUN!(verifier_subprog_precision__elf_bytes, b"verifier_subprog_precision\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_subprog_topo() { RUN!(verifier_subprog_topo__elf_bytes, b"verifier_subprog_topo\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_subreg() { RUN!(verifier_subreg__elf_bytes, b"verifier_subreg\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_tailcall() { RUN!(verifier_tailcall__elf_bytes, b"verifier_tailcall\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_tailcall_jit() { RUN!(verifier_tailcall_jit__elf_bytes, b"verifier_tailcall_jit\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_typedef() { RUN!(verifier_typedef__elf_bytes, b"verifier_typedef\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_uninit() { RUN!(verifier_uninit__elf_bytes, b"verifier_uninit\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_unpriv() { RUN!(verifier_unpriv__elf_bytes, b"verifier_unpriv\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_unpriv_perf() { RUN!(verifier_unpriv_perf__elf_bytes, b"verifier_unpriv_perf\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_value_adj_spill() { RUN!(verifier_value_adj_spill__elf_bytes, b"verifier_value_adj_spill\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_value() { RUN!(verifier_value__elf_bytes, b"verifier_value\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_value_illegal_alu() { RUN!(verifier_value_illegal_alu__elf_bytes, b"verifier_value_illegal_alu\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_value_or_null() { RUN!(verifier_value_or_null__elf_bytes, b"verifier_value_or_null\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_var_off() { RUN!(verifier_var_off__elf_bytes, b"verifier_var_off\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_vfs_accept() { RUN!(verifier_vfs_accept__elf_bytes, b"verifier_vfs_accept\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_vfs_reject() { RUN!(verifier_vfs_reject__elf_bytes, b"verifier_vfs_reject\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_xadd() { RUN!(verifier_xadd__elf_bytes, b"verifier_xadd\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_xdp() { RUN!(verifier_xdp__elf_bytes, b"verifier_xdp\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_xdp_direct_packet_access() { RUN!(verifier_xdp_direct_packet_access__elf_bytes, b"verifier_xdp_direct_packet_access\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_bits_iter() { RUN!(verifier_bits_iter__elf_bytes, b"verifier_bits_iter\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_set_retval() { RUN!(verifier_set_retval__elf_bytes, b"verifier_set_retval\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_lsm() { RUN!(verifier_lsm__elf_bytes, b"verifier_lsm\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_irq() { RUN!(irq__elf_bytes, b"irq\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_mtu() { RUN!(verifier_mtu__elf_bytes, b"verifier_mtu\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_jit_inline() { RUN!(verifier_jit_inline__elf_bytes, b"verifier_jit_inline\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_ctx_ptr_param() { RUN!(verifier_ctx_ptr_param__elf_bytes, b"verifier_ctx_ptr_param\0"); }
#[no_mangle]
pub unsafe extern "C" fn test_verifier_zext() { RUN_TESTS(b"verifier_zext\0".as_ptr() as *const c_char); }

unsafe extern "C" fn init_test_val_map(obj: *mut bpf_object, map_name: *mut c_char) -> c_int {
    let mut value = test_val {
        index: ((6 + 1) * core::mem::size_of::<c_int>()) as c_uint,
        foo: [0; MAX_ENTRIES],
    };
    value.foo[6] = 0xabcdef12_u32 as c_int;
    let mut key: c_int = 0;

    let map = bpf_object__find_map_by_name(obj, map_name);
    if map.is_null() {
        PRINT_FAIL(
            b"Can't find map '%s'\n\0".as_ptr() as *const c_char,
            map_name,
        );
        return -EINVAL;
    }

    let err = bpf_map_update_elem(
        bpf_map__fd(map),
        &mut key as *mut c_int as *const c_void,
        &mut value as *mut test_val as *const c_void,
        0,
    );
    if err != 0 {
        PRINT_FAIL(
            b"Error while updating map '%s': %d\n\0".as_ptr() as *const c_char,
            map_name,
            err,
        );
        return err;
    }

    0
}

unsafe extern "C" fn init_array_access_maps(obj: *mut bpf_object) -> c_int {
    init_test_val_map(obj, b"map_array_ro\0".as_ptr() as *mut c_char)
}

#[no_mangle]
pub unsafe extern "C" fn test_verifier_array_access() {
    run_tests_aux(
        b"verifier_array_access\0".as_ptr() as *const c_char,
        Some(verifier_array_access__elf_bytes),
        Some(init_array_access_maps),
    );
}

#[no_mangle]
pub unsafe extern "C" fn test_verifier_async_cb_context() { RUN!(verifier_async_cb_context__elf_bytes, b"verifier_async_cb_context\0"); }

unsafe extern "C" fn init_value_ptr_arith_maps(obj: *mut bpf_object) -> c_int {
    init_test_val_map(obj, b"map_array_48b\0".as_ptr() as *mut c_char)
}

#[no_mangle]
pub unsafe extern "C" fn test_verifier_value_ptr_arith() {
    run_tests_aux(
        b"verifier_value_ptr_arith\0".as_ptr() as *const c_char,
        Some(verifier_value_ptr_arith__elf_bytes),
        Some(init_value_ptr_arith_maps),
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
