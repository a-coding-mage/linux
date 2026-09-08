// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2011-2017 by the PaX Team <pageexec@freemail.hu>
 * Modified by Alexander Popov <alex.popov@linux.com>
 *
 * This is a low-level Rust translation of the GCC plugin implementation.
 * GCC headers, macros, and generated pass declarations remain external.
 */

extern "C" {
    static mut plugin_is_GPL_compatible: ::std::os::raw::c_int;
}

static mut track_frame_size: i32 = -1;
static mut build_for_x86: bool = false;
static track_function: &[u8] = b"__sanitizer_cov_stack_depth\0";
static mut disable: bool = false;
static mut verbose: bool = false;

// GTY root retained for GCC's garbage collector.
static mut track_function_decl: tree = 0 as tree;

#[repr(C)]
struct plugin_info {
    version: *const ::std::os::raw::c_char,
    help: *const ::std::os::raw::c_char,
}

static mut stackleak_plugin_info: plugin_info = plugin_info {
    version: PLUGIN_VERSION,
    help: b"track-min-size=nn\ttrack stack for functions with a stack frame size >= nn bytes\narch=target_arch\tspecify target build arch\ndisable\t\tdo not activate the plugin\nverbose\t\tprint info about the instrumentation\0" as *const u8 as *const _,
};

unsafe fn add_stack_tracking_gcall(gsi: *mut gimple_stmt_iterator, after: bool) {
    let mut stmt: gimple = gimple_build_call(track_function_decl, 0);
    let gimple_call: *mut gcall = as_a_gcall(stmt);
    if after { gsi_insert_after(gsi, gimple_call, GSI_CONTINUE_LINKING); }
    else { gsi_insert_before(gsi, gimple_call, GSI_SAME_STMT); }
    let bb = gimple_bb(gimple_call);
    let node = cgraph_get_create_node(track_function_decl);
    gcc_assert(node != 0 as cgraph_node_ptr);
    cgraph_create_edge(cgraph_get_node(current_function_decl), node, gimple_call,
        (*bb).count, compute_call_stmt_bb_frequency(current_function_decl, bb));
}

unsafe fn is_alloca(stmt: gimple) -> bool {
    if gimple_call_builtin_p(stmt, BUILT_IN_ALLOCA) { return true; }
    if gimple_call_builtin_p(stmt, BUILT_IN_ALLOCA_WITH_ALIGN) { return true; }
    false
}

unsafe fn get_current_stack_pointer_decl() -> tree {
    let mut node: varpool_node_ptr = 0 as varpool_node_ptr;
    // FOR_EACH_VARIABLE(node) { ... }
    for_each_variable!(node, {
        let var = NODE_DECL(node); let name = DECL_NAME(var);
        if DECL_NAME_LENGTH(var) != (b"current_stack_pointer\0".len() - 1) { continue; }
        if strcmp(IDENTIFIER_POINTER(name), b"current_stack_pointer\0".as_ptr() as *const _) != 0 { continue; }
        return var;
    });
    if verbose { fprintf(stderr, b"stackleak: missing current_stack_pointer in %s()\0".as_ptr() as *const _, DECL_NAME_POINTER(current_function_decl)); }
    NULL_TREE
}

unsafe fn add_stack_tracking_gasm(gsi: *mut gimple_stmt_iterator, after: bool) {
    let mut inputs: vec_tree = 0 as vec_tree;
    gcc_assert(build_for_x86);
    let sp_decl = get_current_stack_pointer_decl();
    if sp_decl == NULL_TREE { add_stack_tracking_gcall(gsi, after); return; }
    let input = chainon(NULL_TREE, build_tree_list(build_tree_list(NULL_TREE, build_const_char_string(2, b"r\0".as_ptr() as *const _)), sp_decl));
    vec_safe_push(&mut inputs, input);
    let asm_call = gimple_build_asm_vec(b"call __sanitizer_cov_stack_depth\0".as_ptr() as *const _, inputs, NULL_TREE, NULL_TREE, NULL_TREE);
    gimple_asm_set_volatile(asm_call, true);
    if after { gsi_insert_after(gsi, asm_call, GSI_CONTINUE_LINKING); }
    else { gsi_insert_before(gsi, asm_call, GSI_SAME_STMT); }
    update_stmt(asm_call);
}

unsafe fn add_stack_tracking(gsi: *mut gimple_stmt_iterator, after: bool) {
    if lookup_attribute_spec(get_identifier(b"no_caller_saved_registers\0".as_ptr() as *const _)) != 0 as _ { add_stack_tracking_gasm(gsi, after); }
    else { add_stack_tracking_gcall(gsi, after); }
}

unsafe fn stackleak_instrument_execute() -> u32 {
    let (mut bb, entry_bb): (basic_block, basic_block);
    let mut prologue_instrumented = false; let mut is_leaf = true;
    let mut gsi = gimple_stmt_iterator { _private: [0; 1] };
    gcc_assert(single_succ_p(ENTRY_BLOCK_PTR_FOR_FN(cfun)));
    entry_bb = single_succ(ENTRY_BLOCK_PTR_FOR_FN(cfun));
    // FOR_EACH_BB_FN and GIMPLE iterator macros are GCC-provided iteration constructs.
    for_each_bb_fn!(bb, cfun, {
        for (gsi = gsi_start_bb(bb); !gsi_end_p(gsi); gsi_next(&mut gsi)) {
            let stmt = gsi_stmt(gsi);
            if is_gimple_call(stmt) { is_leaf = false; }
            if !is_alloca(stmt) { continue; }
            if verbose { fprintf(stderr, b"stackleak: be careful, alloca() in %s()\n\0".as_ptr() as *const _, DECL_NAME_POINTER(current_function_decl)); }
            add_stack_tracking(&mut gsi, true);
            if bb == entry_bb { prologue_instrumented = true; }
        }
    });
    if prologue_instrumented { return 0; }
    if is_leaf && !TREE_PUBLIC(current_function_decl) && DECL_DECLARED_INLINE_P(current_function_decl) { return 0; }
    if is_leaf && strncmp(IDENTIFIER_POINTER(DECL_NAME(current_function_decl)), b"_paravirt_\0".as_ptr() as *const _, 10) == 0 { return 0; }
    bb = entry_bb;
    if !single_pred_p(bb) {
        split_edge(single_succ_edge(ENTRY_BLOCK_PTR_FOR_FN(cfun)));
        gcc_assert(single_succ_p(ENTRY_BLOCK_PTR_FOR_FN(cfun)));
        bb = single_succ(ENTRY_BLOCK_PTR_FOR_FN(cfun));
    }
    gsi = gsi_after_labels(bb); add_stack_tracking(&mut gsi, false); 0
}

unsafe fn large_stack_frame() -> bool {
    // BUILDING_GCC_VERSION >= 8000 uses maybe_ge; older GCC uses >= directly.
    maybe_ge(get_frame_size(), track_frame_size)
}

unsafe fn remove_stack_tracking_gcall() {
    let mut insn = get_insns(); let mut next;
    while insn != 0 as _ { next = NEXT_INSN(insn); if CALL_P(insn) {
        let mut body = PATTERN(insn); if GET_CODE(body) == PARALLEL { body = XVECEXP(body, 0, 0); }
        if GET_CODE(body) == CALL { body = XEXP(body, 0); if GET_CODE(body) == MEM { body = XEXP(body, 0); if GET_CODE(body) == SYMBOL_REF && SYMBOL_REF_DECL(body) == track_function_decl { delete_insn_and_edges(insn); } } }
    } insn = next; }
}

unsafe fn remove_stack_tracking_gasm() -> bool {
    let mut removed = false; let mut insn = get_insns(); let mut next;
    gcc_assert(build_for_x86);
    while insn != 0 as _ { next = NEXT_INSN(insn); if NONJUMP_INSN_P(insn) { let mut body = PATTERN(insn); if GET_CODE(body) == PARALLEL { body = XVECEXP(body, 0, 0); if GET_CODE(body) == ASM_OPERANDS && strcmp(ASM_OPERANDS_TEMPLATE(body), b"call __sanitizer_cov_stack_depth\0".as_ptr() as *const _) == 0 { delete_insn_and_edges(insn); gcc_assert(!removed); removed = true; } } } insn = next; }
    removed
}

unsafe fn stackleak_cleanup_execute() -> u32 {
    let fn_name = DECL_NAME_POINTER(current_function_decl); let mut removed = false;
    if (*cfun).calls_alloca { if verbose { fprintf(stderr, b"stackleak: instrument %s(): calls_alloca\n\0".as_ptr() as *const _, fn_name); } return 0; }
    if large_stack_frame() { if verbose { fprintf(stderr, b"stackleak: instrument %s()\n\0".as_ptr() as *const _, fn_name); } return 0; }
    if lookup_attribute_spec(get_identifier(b"no_caller_saved_registers\0".as_ptr() as *const _)) != 0 as _ { removed = remove_stack_tracking_gasm(); }
    if !removed { remove_stack_tracking_gcall(); } 0
}

unsafe fn string_equal(node: tree, string: *const ::std::os::raw::c_char, length: i32) -> bool {
    if TREE_STRING_LENGTH(node) < length || TREE_STRING_LENGTH(node) > length + 1 { return false; }
    if TREE_STRING_LENGTH(node) == length + 1 && TREE_STRING_POINTER(node).offset(length as isize).read() != 0 { return false; }
    memcmp(TREE_STRING_POINTER(node), string, length as usize) == 0
}

unsafe fn stackleak_gate() -> bool {
    let mut section = lookup_attribute(b"section\0".as_ptr() as *const _, DECL_ATTRIBUTES(current_function_decl));
    if section != NULL_TREE && TREE_VALUE(section) != NULL_TREE { section = TREE_VALUE(TREE_VALUE(section)); for s in [b".init.text\0", b".devinit.text\0", b".cpuinit.text\0", b".meminit.text\0", b".noinstr.text\0", b".entry.text\0", b".head.text\0"] { if string_equal(section, s.as_ptr() as *const _, s.len() as i32 - 1) { return false; } } }
    track_frame_size >= 0
}

unsafe fn stackleak_instrument_gate() -> bool { stackleak_gate() }
unsafe fn stackleak_cleanup_gate() -> bool { stackleak_gate() }

unsafe fn stackleak_start_unit(_gcc_data: *mut ::std::ffi::c_void, _user_data: *mut ::std::ffi::c_void) {
    let fntype = build_function_type_list(void_type_node, NULL_TREE);
    track_function_decl = build_fn_decl(track_function.as_ptr() as *const _, fntype);
    DECL_ASSEMBLER_NAME(track_function_decl); TREE_PUBLIC(track_function_decl) = 1; TREE_USED(track_function_decl) = 1; DECL_EXTERNAL(track_function_decl) = 1; DECL_ARTIFICIAL(track_function_decl) = 1; DECL_PRESERVE_P(track_function_decl) = 1;
}

// Generated by gcc-generate-gimple-pass.h / gcc-generate-rtl-pass.h in C.
const PASS_NAME_STACKLEAK_INSTRUMENT: &str = "stackleak_instrument";
const PASS_NAME_STACKLEAK_CLEANUP: &str = "stackleak_cleanup";

#[no_mangle]
pub unsafe extern "C" fn plugin_init(plugin_info: *mut plugin_name_args, version: *mut plugin_gcc_version) -> i32 {
    let plugin_name = (*plugin_info).base_name; let argc = (*plugin_info).argc; let argv = (*plugin_info).argv;
    if !plugin_default_version_check(version, &gcc_version) { error(G_(b"incompatible gcc/plugin versions\0".as_ptr() as *const _)); return 1; }
    for i in 0..argc { let arg = &*argv.add(i as usize); if strcmp(arg.key, b"track-min-size\0".as_ptr() as *const _) == 0 { if arg.value.is_null() { error(G_(b"no value supplied for option '-fplugin-arg-%s-%s'\0".as_ptr() as *const _), plugin_name, arg.key); return 1; } track_frame_size = atoi(arg.value); if track_frame_size < 0 { error(G_(b"invalid option argument '-fplugin-arg-%s-%s=%s'\0".as_ptr() as *const _), plugin_name, arg.key, arg.value); return 1; } } else if strcmp(arg.key, b"arch\0".as_ptr() as *const _) == 0 { if arg.value.is_null() { error(G_(b"no value supplied for option '-fplugin-arg-%s-%s'\0".as_ptr() as *const _), plugin_name, arg.key); return 1; } if strcmp(arg.value, b"x86\0".as_ptr() as *const _) == 0 { build_for_x86 = true; } } else if strcmp(arg.key, b"disable\0".as_ptr() as *const _) == 0 { disable = true; } else if strcmp(arg.key, b"verbose\0".as_ptr() as *const _) == 0 { verbose = true; } else { error(G_(b"unknown option '-fplugin-arg-%s-%s'\0".as_ptr() as *const _), plugin_name, arg.key); return 1; } }
    if disable { if verbose { fprintf(stderr, b"stackleak: disabled for this translation unit\n\0".as_ptr() as *const _); } return 0; }
    register_callback(plugin_name, PLUGIN_INFO, NULL_TREE, &stackleak_plugin_info as *const _ as *mut _);
    register_callback(plugin_name, PLUGIN_START_UNIT, stackleak_start_unit as *const _ as *mut _, NULL_TREE);
    register_callback(plugin_name, PLUGIN_REGISTER_GGC_ROOTS, NULL_TREE, &gt_ggc_r_gt_stackleak as *const _ as *mut _);
    register_callback(plugin_name, PLUGIN_PASS_MANAGER_SETUP, NULL_TREE, &stackleak_instrument_pass_info as *const _ as *mut _);
    register_callback(plugin_name, PLUGIN_PASS_MANAGER_SETUP, NULL_TREE, &stackleak_cleanup_pass_info as *const _ as *mut _); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
