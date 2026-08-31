// SPDX-License-Identifier: GPL-2.0
//
// Source-level Rust translation of verifier_bpf_fastcall.c.
// C include dependencies intentionally remain external:
// linux/bpf.h, bpf_helpers.h, bpf_core_read.h, filter.h, bpf_misc.h, bpf_kfuncs.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::arch::asm;

const MAX_BPF_STACK: i32 = 512;
const BPF_JMP: u32 = 0x05;
const BPF_JCOND: u32 = 0xe0;

#[repr(C)]
pub union bpf_attr {
    _bindgen_union_align: u64,
}

unsafe extern "C" {
    fn bpf_get_smp_processor_id() -> u64;
    fn bpf_probe_read_kernel(dst: *mut core::ffi::c_void, size: u32, unsafe_ptr: *const core::ffi::c_void) -> i64;
    fn bpf_get_prandom_u32() -> u32;
    fn bpf_loop(nr_loops: u32, callback_fn: *const core::ffi::c_void, callback_ctx: *mut core::ffi::c_void, flags: u64) -> i64;
    fn bpf_cast_to_kern_ctx(ctx: u64) -> u64;
    fn bpf_rdonly_cast(obj: u64, btf_id: u64) -> u64;
    fn bpf_core_type_id_kernel<T>() -> u64;
}

// SEC("raw_tp")
// __arch_x86_64
// __log_level(4)
// __msg("subprog 0 (simple) main insns_self {{[0-9]+}} insns_total {{[0-9]+}} stack 8")
// __xlated("4: r5 = 5")
// __xlated("5: r0 = ")
// __xlated("6: r0 = &(void __percpu *)(r0)")
// __xlated("7: r0 = *(u32 *)(r0 +0)")
// __xlated("8: exit")
// __success
#[link_section = "raw_tp"]
pub unsafe extern "C" fn simple() {
    asm!(
        "r1 = 1;",
        "r2 = 2;",
        "r3 = 3;",
        "r4 = 4;",
        "r5 = 5;",
        "*(u64 *)(r10 - 16) = r1;",
        "*(u64 *)(r10 - 24) = r2;",
        "*(u64 *)(r10 - 32) = r3;",
        "*(u64 *)(r10 - 40) = r4;",
        "*(u64 *)(r10 - 48) = r5;",
        "call {bpf_get_smp_processor_id};",
        "r5 = *(u64 *)(r10 - 48);",
        "r4 = *(u64 *)(r10 - 40);",
        "r3 = *(u64 *)(r10 - 32);",
        "r2 = *(u64 *)(r10 - 24);",
        "r1 = *(u64 *)(r10 - 16);",
        "exit;",
        bpf_get_smp_processor_id = sym bpf_get_smp_processor_id,
    );
}

/* The logic for detecting and verifying bpf_fastcall pattern is the same for
 * any arch, however x86 differs from arm64 or riscv64 in a way
 * bpf_get_smp_processor_id is rewritten:
 * - on x86 it is done by verifier
 * - on arm64 and riscv64 it is done by jit
 *
 * Which leads to different xlated patterns for different archs:
 * - on x86 the call is expanded as 3 instructions
 * - on arm64 and riscv64 the call remains as is
 *   (but spills/fills are still removed)
 *
 * It is really desirable to check instruction indexes in the xlated
 * patterns, so add this canary test to check that function rewrite by
 * jit is correctly processed by bpf_fastcall logic, keep the rest of the
 * tests as x86.
 */
// SEC("raw_tp")
// __arch_arm64
// __arch_riscv64
// __xlated("0: r1 = 1")
// __xlated("1: call bpf_get_smp_processor_id")
// __xlated("2: exit")
// __success
#[link_section = "raw_tp"]
pub unsafe extern "C" fn canary_arm64_riscv64() {
    asm!(
        "r1 = 1;",
        "*(u64 *)(r10 - 16) = r1;",
        "call {bpf_get_smp_processor_id};",
        "r1 = *(u64 *)(r10 - 16);",
        "exit;",
        bpf_get_smp_processor_id = sym bpf_get_smp_processor_id,
    );
}

// SEC("raw_tp")
// __arch_x86_64
// __xlated("1: r0 = &(void __percpu *)(r0)")
// __xlated("...")
// __xlated("3: exit")
// __success
#[link_section = "raw_tp"]
pub unsafe extern "C" fn canary_zero_spills() {
    asm!(
        "call {bpf_get_smp_processor_id};",
        "exit;",
        bpf_get_smp_processor_id = sym bpf_get_smp_processor_id,
    );
}

macro_rules! fastcall_prog {
    ($name:ident, [$($meta:expr),* $(,)?], $($asm:expr),+ $(,)?) => {
        $(#[doc = $meta])*
        #[link_section = "raw_tp"]
        pub unsafe extern "C" fn $name() {
            asm!(
                $($asm,)+
                bpf_get_smp_processor_id = sym bpf_get_smp_processor_id,
                bpf_probe_read_kernel = sym bpf_probe_read_kernel,
                bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
                bpf_loop = sym bpf_loop,
            );
        }
    };
}

fastcall_prog!(wrong_reg_in_pattern1, [
    r#"SEC("raw_tp") __arch_x86_64 __log_level(4) __msg("subprog 0 (wrong_reg_in_pattern1) main {{.*}} stack 16")"#,
    r#"__xlated("1: *(u64 *)(r10 -16) = r1") __xlated("...") __xlated("3: r0 = &(void __percpu *)(r0)") __xlated("...") __xlated("5: r2 = *(u64 *)(r10 -16)") __success"#,
],
    "r1 = 1;", "*(u64 *)(r10 - 16) = r1;", "call {bpf_get_smp_processor_id};", "r2 = *(u64 *)(r10 - 16);", "exit;"
);

fastcall_prog!(wrong_reg_in_pattern2, [r#"SEC("raw_tp") __arch_x86_64 __xlated("1: *(u64 *)(r10 -16) = r6") __xlated("...") __xlated("3: r0 = &(void __percpu *)(r0)") __xlated("...") __xlated("5: r6 = *(u64 *)(r10 -16)") __success"#],
    "r6 = 1;", "*(u64 *)(r10 - 16) = r6;", "call {bpf_get_smp_processor_id};", "r6 = *(u64 *)(r10 - 16);", "exit;"
);

fastcall_prog!(wrong_reg_in_pattern3, [r#"SEC("raw_tp") __arch_x86_64 __xlated("1: *(u64 *)(r10 -16) = r0") __xlated("...") __xlated("3: r0 = &(void __percpu *)(r0)") __xlated("...") __xlated("5: r0 = *(u64 *)(r10 -16)") __success"#],
    "r0 = 1;", "*(u64 *)(r10 - 16) = r0;", "call {bpf_get_smp_processor_id};", "r0 = *(u64 *)(r10 - 16);", "exit;"
);

fastcall_prog!(wrong_base_in_pattern, [r#"SEC("raw_tp") __arch_x86_64 __xlated("2: *(u64 *)(r2 -16) = r1") __xlated("...") __xlated("4: r0 = &(void __percpu *)(r0)") __xlated("...") __xlated("6: r1 = *(u64 *)(r10 -16)") __success"#],
    "r1 = 1;", "r2 = r10;", "*(u64 *)(r2 - 16) = r1;", "call {bpf_get_smp_processor_id};", "r1 = *(u64 *)(r10 - 16);", "exit;"
);

fastcall_prog!(wrong_insn_in_pattern, [r#"SEC("raw_tp") __arch_x86_64 __xlated("1: *(u64 *)(r10 -16) = r1") __xlated("...") __xlated("3: r0 = &(void __percpu *)(r0)") __xlated("...") __xlated("5: r2 = 1") __success"#],
    "r1 = 1;", "*(u64 *)(r10 - 16) = r1;", "call {bpf_get_smp_processor_id};", "r2 = 1;", "r1 = *(u64 *)(r10 - 16);", "exit;"
);

fastcall_prog!(wrong_off_in_pattern1, [r#"SEC("raw_tp") __arch_x86_64 __xlated("2: *(u64 *)(r10 -16) = r1") __xlated("...") __xlated("4: r0 = &(void __percpu *)(r0)") __xlated("...") __xlated("6: r1 = *(u64 *)(r10 -8)") __success"#],
    "r1 = 1;", "*(u64 *)(r10 - 8) = r1;", "*(u64 *)(r10 - 16) = r1;", "call {bpf_get_smp_processor_id};", "r1 = *(u64 *)(r10 - 8);", "exit;"
);

fastcall_prog!(wrong_off_in_pattern2, [r#"SEC("raw_tp") __arch_x86_64 __xlated("1: *(u32 *)(r10 -4) = r1") __xlated("...") __xlated("3: r0 = &(void __percpu *)(r0)") __xlated("...") __xlated("5: r1 = *(u32 *)(r10 -4)") __success"#],
    "r1 = 1;", "*(u32 *)(r10 - 4) = r1;", "call {bpf_get_smp_processor_id};", "r1 = *(u32 *)(r10 - 4);", "exit;"
);

fastcall_prog!(wrong_size_in_pattern, [r#"SEC("raw_tp") __arch_x86_64 __xlated("1: *(u32 *)(r10 -16) = r1") __xlated("...") __xlated("3: r0 = &(void __percpu *)(r0)") __xlated("...") __xlated("5: r1 = *(u32 *)(r10 -16)") __success"#],
    "r1 = 1;", "*(u32 *)(r10 - 16) = r1;", "call {bpf_get_smp_processor_id};", "r1 = *(u32 *)(r10 - 16);", "exit;"
);

fastcall_prog!(partial_pattern, [r#"SEC("raw_tp") __arch_x86_64 __xlated("2: *(u32 *)(r10 -8) = r1") __xlated("...") __xlated("4: r0 = &(void __percpu *)(r0)") __xlated("...") __xlated("6: r1 = *(u32 *)(r10 -8)") __success"#],
    "r1 = 1;", "r2 = 2;", "*(u32 *)(r10 - 8) = r1;", "*(u64 *)(r10 - 16) = r2;", "call {bpf_get_smp_processor_id};", "r2 = *(u64 *)(r10 - 16);", "r1 = *(u32 *)(r10 - 8);", "exit;"
);

fastcall_prog!(min_stack_offset, [r#"SEC("raw_tp") __arch_x86_64 min_stack_offset xlated patterns; not patched for -8/-16, patched for -24/-32; __success"#],
    "r1 = 1;", "r2 = 2;", "*(u64 *)(r10 - 8) = r1;", "*(u64 *)(r10 - 16) = r2;", "call {bpf_get_smp_processor_id};", "r2 = *(u64 *)(r10 - 16);", "r1 = *(u64 *)(r10 - 8);", "*(u64 *)(r10 - 24) = r1;", "*(u64 *)(r10 - 32) = r2;", "call {bpf_get_smp_processor_id};", "r2 = *(u64 *)(r10 - 32);", "r1 = *(u64 *)(r10 - 24);", "exit;"
);

fastcall_prog!(bad_fixed_read, [r#"SEC("raw_tp") __arch_x86_64 bad_fixed_read __success"#],
    "r1 = 1;", "*(u64 *)(r10 - 8) = r1;", "call {bpf_get_smp_processor_id};", "r1 = *(u64 *)(r10 - 8);", "r1 = r10;", "r1 += -8;", "r1 = *(u64 *)(r1 - 0);", "exit;"
);

fastcall_prog!(bad_fixed_write, [r#"SEC("raw_tp") __arch_x86_64 bad_fixed_write __success"#],
    "r1 = 1;", "*(u64 *)(r10 - 8) = r1;", "call {bpf_get_smp_processor_id};", "r1 = *(u64 *)(r10 - 8);", "r1 = r10;", "r1 += -8;", "*(u64 *)(r1 - 0) = r1;", "exit;"
);

fastcall_prog!(bad_varying_read, [r#"SEC("raw_tp") __arch_x86_64 bad_varying_read __success"#],
    "r6 = *(u64 *)(r1 + 0);", "r6 &= 0x7;", "r6 += 0x2;", "r7 = 0;", "r7 -= r6;", "r1 = 1;", "*(u64 *)(r10 - 16) = r1;", "call {bpf_get_smp_processor_id};", "r1 = *(u64 *)(r10 - 16);", "r1 = r10;", "r1 += r7;", "r1 = *(u8 *)(r1 - 0);", "exit;"
);

fastcall_prog!(bad_varying_write, [r#"SEC("raw_tp") __arch_x86_64 bad_varying_write __success"#],
    "r6 = *(u64 *)(r1 + 0);", "r6 &= 0x7;", "r6 += 0x2;", "r7 = 0;", "r7 -= r6;", "r1 = 1;", "*(u64 *)(r10 - 16) = r1;", "call {bpf_get_smp_processor_id};", "r1 = *(u64 *)(r10 - 16);", "r1 = r10;", "r1 += r7;", "*(u8 *)(r1 - 0) = r7;", "exit;"
);

fastcall_prog!(bad_write_in_subprog, [r#"SEC("raw_tp") __arch_x86_64 bad_write_in_subprog __success"#],
    "r1 = 1;", "*(u64 *)(r10 - 8) = r1;", "call {bpf_get_smp_processor_id};", "r1 = *(u64 *)(r10 - 8);", "r1 = r10;", "r1 += -8;", "call bad_write_in_subprog_aux;", "exit;"
);

unsafe extern "C" fn bad_write_in_subprog_aux() {
    asm!("r0 = 1;", "*(u64 *)(r1 - 0) = r0;", "exit;");
}

fastcall_prog!(bad_helper_write, [r#"SEC("raw_tp") __arch_x86_64 bad_helper_write __success; read dst is fp[-8], thus bpf_fastcall rewrite not applied"#],
    "r1 = 1;", "*(u64 *)(r10 - 8) = r1;", "call {bpf_get_smp_processor_id};", "r1 = *(u64 *)(r10 - 8);", "r1 = r10;", "r1 += -8;", "r2 = 1;", "r3 = 42;", "call {bpf_probe_read_kernel};", "exit;"
);

fastcall_prog!(invalidate_one_subprog, [r#"SEC("raw_tp") __arch_x86_64 main not patched; subprogram patched; __success"#],
    "r1 = 1;", "*(u64 *)(r10 - 8) = r1;", "call {bpf_get_smp_processor_id};", "r1 = *(u64 *)(r10 - 8);", "r1 = r10;", "r1 += -8;", "r1 = *(u64 *)(r1 - 0);", "call invalidate_one_subprog_aux;", "exit;"
);

unsafe extern "C" fn invalidate_one_subprog_aux() {
    asm!("r1 = 1;", "*(u64 *)(r10 - 8) = r1;", "call {bpf_get_smp_processor_id};", "r1 = *(u64 *)(r10 - 8);", "exit;", bpf_get_smp_processor_id = sym bpf_get_smp_processor_id);
}

fastcall_prog!(subprogs_use_independent_offsets, [r#"SEC("raw_tp") __arch_x86_64 main and subprogram use independent offsets; __success"#],
    "r1 = 1;", "*(u64 *)(r10 - 16) = r1;", "call {bpf_get_smp_processor_id};", "r1 = *(u64 *)(r10 - 16);", "call subprogs_use_independent_offsets_aux;", "exit;"
);

unsafe extern "C" fn subprogs_use_independent_offsets_aux() {
    asm!("r1 = 1;", "*(u64 *)(r10 - 24) = r1;", "call {bpf_get_smp_processor_id};", "r1 = *(u64 *)(r10 - 24);", "*(u64 *)(r10 - 16) = r1;", "exit;", bpf_get_smp_processor_id = sym bpf_get_smp_processor_id);
}

fastcall_prog!(helper_call_does_not_prevent_bpf_fastcall, [r#"SEC("raw_tp") __arch_x86_64 __log_level(4) __msg("subprog 0 (helper_call_does_not_prevent_bpf_fastcall) main {{.*}} stack 8") __xlated("2: r0 = &(void __percpu *)(r0)") __success"#],
    "r1 = 1;", "*(u64 *)(r10 - 8) = r1;", "call {bpf_get_smp_processor_id};", "r1 = *(u64 *)(r10 - 8);", "*(u64 *)(r10 - 8) = r1;", "call {bpf_get_prandom_u32};", "r1 = *(u64 *)(r10 - 8);", "exit;"
);

fastcall_prog!(may_goto_interaction_x86_64, [r#"SEC("raw_tp") __arch_x86_64 __log_level(4) __msg("subprog 0 (may_goto_interaction_x86_64) main {{.*}} stack 24"); may_goto counter at -24, timestamp at -16; __success"#],
    "r1 = 1;", "*(u64 *)(r10 - 16) = r1;", "call {bpf_get_smp_processor_id};", "r1 = *(u64 *)(r10 - 16);", ".8byte 0x1000000e5;", "*(u64 *)(r10 - 8) = r1;", "exit;"
);

fastcall_prog!(may_goto_interaction, [r#"SEC("raw_tp") __arch_arm64 __arch_riscv64 __arch_loongarch __log_level(4) __msg("subprog 0 (may_goto_interaction) main {{.*}} stack 24"); may_goto expansion checked; __success"#],
    "r1 = 1;", "*(u64 *)(r10 - 16) = r1;", "call {bpf_get_smp_processor_id};", "r1 = *(u64 *)(r10 - 16);", ".8byte 0x1000000e5;", "*(u64 *)(r10 - 8) = r1;", "exit;"
);

unsafe extern "C" fn dummy_loop_callback() {
    asm!("r0 = 0;", "exit;");
}

#[link_section = "raw_tp"]
pub unsafe extern "C" fn bpf_loop_interaction1() -> i32 {
    asm!(
        "r1 = 1;",
        "*(u64 *)(r10 - 16) = r1;",
        "call {bpf_get_smp_processor_id};",
        "r1 = *(u64 *)(r10 - 16);",
        "r2 = {dummy_loop_callback};",
        "r3 = 0;",
        "r4 = 0;",
        "call {bpf_loop};",
        "r0 = 0;",
        "exit;",
        dummy_loop_callback = sym dummy_loop_callback,
        bpf_get_smp_processor_id = sym bpf_get_smp_processor_id,
        bpf_loop = sym bpf_loop,
    );
    0
}

#[link_section = "raw_tp"]
pub unsafe extern "C" fn bpf_loop_interaction2() -> i32 {
    asm!(
        "r1 = 42;",
        "*(u64 *)(r10 - 16) = r1;",
        "call {bpf_get_smp_processor_id};",
        "r1 = *(u64 *)(r10 - 16);",
        "*(u64 *)(r10 - 16) = r1;",
        "call {bpf_get_prandom_u32};",
        "r1 = *(u64 *)(r10 - 16);",
        "r2 = {dummy_loop_callback};",
        "r3 = 0;",
        "r4 = 0;",
        "call {bpf_loop};",
        "r0 = 0;",
        "exit;",
        dummy_loop_callback = sym dummy_loop_callback,
        bpf_get_smp_processor_id = sym bpf_get_smp_processor_id,
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
        bpf_loop = sym bpf_loop,
    );
    0
}

#[link_section = "raw_tp"]
pub unsafe extern "C" fn cumulative_stack_depth() -> i32 {
    asm!(
        "r1 = 42;",
        "*(u64 *)(r10 - {max_bpf_stack}) = r1;",
        "call cumulative_stack_depth_subprog;",
        "exit;",
        max_bpf_stack = const MAX_BPF_STACK,
    );
    0
}

unsafe extern "C" fn cumulative_stack_depth_subprog() {
    asm!(
        "*(u64 *)(r10 - 8) = r1;",
        "call {bpf_get_smp_processor_id};",
        "r1 = *(u64 *)(r10 - 8);",
        "exit;",
        bpf_get_smp_processor_id = sym bpf_get_smp_processor_id,
    );
}

#[link_section = "cgroup/getsockname_unix"]
pub unsafe extern "C" fn kfunc_bpf_cast_to_kern_ctx() {
    asm!(
        "r2 = 1;",
        "*(u64 *)(r10 - 32) = r2;",
        "call {bpf_cast_to_kern_ctx};",
        "r2 = *(u64 *)(r10 - 32);",
        "r0 = r2;",
        "exit;",
        bpf_cast_to_kern_ctx = sym bpf_cast_to_kern_ctx,
    );
}

#[link_section = "raw_tp"]
pub unsafe extern "C" fn kfunc_bpf_rdonly_cast() {
    let btf_id = bpf_core_type_id_kernel::<bpf_attr>();
    asm!(
        "r2 = {btf_id};",
        "r3 = 1;",
        "*(u64 *)(r10 - 32) = r3;",
        "call {bpf_rdonly_cast};",
        "r3 = *(u64 *)(r10 - 32);",
        "r0 = r3;",
        btf_id = in(reg) btf_id,
        bpf_rdonly_cast = sym bpf_rdonly_cast,
    );
}

/* BTF FUNC records are not generated for kfuncs referenced
 * from inline assembly. These records are necessary for
 * libbpf to link the program. The function below is a hack
 * to ensure that BTF FUNC records are generated.
 */
pub unsafe extern "C" fn kfunc_root() {
    bpf_cast_to_kern_ctx(0);
    bpf_rdonly_cast(0, 0);
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
