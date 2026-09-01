// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
//
// C dependencies: <bpf/bpf.h>, <bpf/libbpf.h>, <test_progs.h>

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type __u32 = u32;
type __u64 = u64;
type size_t = usize;
type uint8_t = u8;
type uint64_t = u64;
type FILE = c_void;
type LLVMDisasmContextRef = *mut c_void;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EOPNOTSUPP: c_int = 95;
const VERBOSE_VERY: c_int = 2;

const LLVMDisassembler_ReferenceType_InOut_None: uint64_t = 0;
const LLVMDisassembler_ReferenceType_In_Branch: uint64_t = 1;
const LLVMDisassembler_Option_PrintImmHex: uint64_t = 1;

#[repr(C)]
struct bpf_prog_info {
    _unused: [u8; 0],
    jited_prog_len: __u32,
    nr_jited_func_lens: __u32,
    jited_prog_insns: __u64,
    jited_func_lens: __u64,
}

#[repr(C)]
struct test_env {
    verbosity: c_int,
}

unsafe extern "C" {
    static mut env: test_env;
    static mut errno: c_int;

    fn LLVMGetDefaultTargetTriple() -> *mut c_char;
    fn LLVMGetHostCPUName() -> *mut c_char;
    fn LLVMGetHostCPUFeatures() -> *mut c_char;
    fn LLVMCreateDisasmCPUFeatures(
        triple: *const c_char,
        cpu: *const c_char,
        features: *const c_char,
        dis_info: *mut c_void,
        tag_type: c_int,
        get_op_info: *mut c_void,
        symbol_lookup: Option<
            unsafe extern "C" fn(
                data: *mut c_void,
                ref_value: uint64_t,
                ref_type: *mut uint64_t,
                ref_pc: uint64_t,
                ref_name: *mut *const c_char,
            ) -> *const c_char,
        >,
    ) -> LLVMDisasmContextRef;
    fn LLVMDisposeMessage(message: *mut c_char);
    fn LLVMSetDisasmOptions(dc: LLVMDisasmContextRef, options: uint64_t) -> c_int;
    fn LLVMDisasmInstruction(
        dc: LLVMDisasmContextRef,
        bytes: *mut uint8_t,
        bytes_size: __u64,
        pc: uint64_t,
        out_string: *mut c_char,
        out_string_size: size_t,
    ) -> size_t;
    fn LLVMDisasmDispose(dc: LLVMDisasmContextRef);
    fn LLVMInitializeAllTargetInfos();
    fn LLVMInitializeAllTargetMCs();
    fn LLVMInitializeAllDisassemblers();

    fn bpf_prog_get_info_by_fd(fd: c_int, info: *mut bpf_prog_info, info_len: *__u32) -> c_int;
    fn fmemopen(buf: *mut c_void, size: size_t, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn qsort(
        base: *mut c_void,
        nmemb: size_t,
        size: size_t,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    );
    fn bsearch(
        key: *const c_void,
        base: *const c_void,
        nmemb: size_t,
        size: size_t,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    ) -> *mut c_void;
    fn snprintf(str: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fputc(c: c_int, stream: *mut FILE) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn PRINT_FAIL(format: *const c_char, ...);
}

// #ifdef HAVE_LLVM_SUPPORT

/* The intent is to use get_jited_program_text() for small test
 * programs written in BPF assembly, thus assume that 32 local labels
 * would be sufficient.
 */
const MAX_LOCAL_LABELS: usize = 32;

/* Local labels are encoded as 'L42', this requires 4 bytes of storage:
 * 3 characters + zero byte
 */
const LOCAL_LABEL_LEN: usize = 4;

static mut llvm_initialized: bool = false;

#[repr(C)]
struct local_labels {
    print_phase: bool,
    prog_len: __u32,
    cnt: __u32,
    pcs: [__u32; MAX_LOCAL_LABELS],
    names: [[c_char; LOCAL_LABEL_LEN]; MAX_LOCAL_LABELS],
}

unsafe extern "C" fn lookup_symbol(
    data: *mut c_void,
    ref_value: uint64_t,
    ref_type: *mut uint64_t,
    _ref_pc: uint64_t,
    ref_name: *mut *const c_char,
) -> *const c_char {
    let labels: *mut local_labels = data as *mut local_labels;
    let type_: uint64_t = unsafe { *ref_type };
    let mut i: c_int;

    unsafe {
        *ref_type = LLVMDisassembler_ReferenceType_InOut_None;
        *ref_name = core::ptr::null();
    }
    if type_ != LLVMDisassembler_ReferenceType_In_Branch {
        return core::ptr::null();
    }
    /* Depending on labels->print_phase either discover local labels or
     * return a name assigned with local jump target:
     * - if print_phase is true and ref_value is in labels->pcs,
     *   return corresponding labels->name.
     * - if print_phase is false, save program-local jump targets
     *   in labels->pcs;
     */
    unsafe {
        if (*labels).print_phase {
            i = 0;
            while i < (*labels).cnt as c_int {
                if (*labels).pcs[i as usize] == ref_value as __u32 {
                    return (*labels).names[i as usize].as_ptr();
                }
                i += 1;
            }
        } else if (*labels).cnt < MAX_LOCAL_LABELS as __u32 && ref_value < (*labels).prog_len as uint64_t {
            let cnt = (*labels).cnt as usize;
            (*labels).pcs[cnt] = ref_value as __u32;
            (*labels).cnt += 1;
        }
    }
    core::ptr::null()
}

unsafe fn disasm_insn(
    ctx: LLVMDisasmContextRef,
    image: *mut uint8_t,
    len: __u32,
    pc: __u32,
    buf: *mut c_char,
    buf_sz: __u32,
) -> c_int {
    let mut i: c_int;
    let cnt: c_int;

    cnt = unsafe {
        LLVMDisasmInstruction(
            ctx,
            image.add(pc as usize),
            (len - pc) as __u64,
            pc as uint64_t,
            buf,
            buf_sz as size_t,
        ) as c_int
    };
    if cnt > 0 {
        return cnt;
    }
    unsafe {
        PRINT_FAIL(c"Can't disasm instruction at offset %d:".as_ptr(), pc as c_int);
    }
    i = 0;
    while i < 16 && pc + i as __u32 < len {
        unsafe {
            printf(c" %02x".as_ptr(), *image.add((pc + i as __u32) as usize) as c_int);
        }
        i += 1;
    }
    unsafe {
        printf(c"\n".as_ptr());
    }
    -EINVAL
}

unsafe extern "C" fn cmp_u32(_a: *const c_void, _b: *const c_void) -> c_int {
    let a: __u32 = unsafe { *(_a as *mut __u32) };
    let b: __u32 = unsafe { *(_b as *mut __u32) };

    if a < b {
        return -1;
    }
    if a > b {
        return 1;
    }
    0
}

unsafe fn disasm_one_func(text_out: *mut FILE, image: *mut uint8_t, len: __u32) -> c_int {
    let mut label: *const c_char;
    let mut colon: *const c_char;
    let mut triple: *mut c_char = core::ptr::null_mut();
    let mut ctx: LLVMDisasmContextRef = core::ptr::null_mut();
    let mut labels: local_labels = unsafe { core::mem::zeroed() };
    let mut label_pc: *mut __u32;
    let mut pc: __u32;
    let mut i: c_int;
    let mut cnt: c_int;
    let mut err: c_int = 0;
    let mut buf: [c_char; 64] = [0; 64];
    let cpu: *mut c_char;
    let features: *mut c_char;

    unsafe {
        triple = LLVMGetDefaultTargetTriple();

        cpu = LLVMGetHostCPUName();
        features = LLVMGetHostCPUFeatures();

        ctx = LLVMCreateDisasmCPUFeatures(
            triple,
            cpu,
            features,
            &mut labels as *mut local_labels as *mut c_void,
            0,
            core::ptr::null_mut(),
            Some(lookup_symbol),
        );

        LLVMDisposeMessage(cpu);
        LLVMDisposeMessage(features);

        if !ASSERT_OK_PTR(ctx as *const c_void, c"LLVMCreateDisasmCPUFeatures".as_ptr()) {
            err = -EINVAL;
            goto_out(&mut triple, &mut ctx);
            return err;
        }

        cnt = LLVMSetDisasmOptions(ctx, LLVMDisassembler_Option_PrintImmHex);
        if !ASSERT_EQ(cnt, 1, c"LLVMSetDisasmOptions".as_ptr()) {
            err = -EINVAL;
            goto_out(&mut triple, &mut ctx);
            return err;
        }
    }

    /* discover labels */
    labels.prog_len = len;
    pc = 0;
    while pc < len {
        cnt = unsafe { disasm_insn(ctx, image, len, pc, buf.as_mut_ptr(), 1) };
        if cnt < 0 {
            err = cnt;
            unsafe {
                goto_out(&mut triple, &mut ctx);
            }
            return err;
        }
        pc += cnt as __u32;
    }
    unsafe {
        qsort(
            labels.pcs.as_mut_ptr() as *mut c_void,
            labels.cnt as size_t,
            core::mem::size_of_val(&labels.pcs[0]),
            Some(cmp_u32),
        );
    }
    /* gcc is unable to infer upper bound for labels.cnt and
     * assumes it to be U32_MAX. U32_MAX takes 10 decimal digits.
     * snprintf below prints into labels.names[*], which has space
     * only for two digits and a letter.  To avoid truncation
     * warning use (i < MAX_LOCAL_LABELS), which informs gcc about
     * printed value upper bound.
     */
    i = 0;
    while i < labels.cnt as c_int && i < MAX_LOCAL_LABELS as c_int {
        unsafe {
            snprintf(
                labels.names[i as usize].as_mut_ptr(),
                core::mem::size_of_val(&labels.names[i as usize]),
                c"L%d".as_ptr(),
                i,
            );
        }
        i += 1;
    }

    /* now print with labels */
    labels.print_phase = true;
    pc = 0;
    while pc < len {
        cnt = unsafe { disasm_insn(ctx, image, len, pc, buf.as_mut_ptr(), core::mem::size_of_val(&buf) as __u32) };
        if cnt < 0 {
            err = cnt;
            unsafe {
                goto_out(&mut triple, &mut ctx);
            }
            return err;
        }
        unsafe {
            label_pc = bsearch(
                &pc as *const __u32 as *const c_void,
                labels.pcs.as_ptr() as *const c_void,
                labels.cnt as size_t,
                core::mem::size_of_val(&labels.pcs[0]),
                Some(cmp_u32),
            ) as *mut __u32;
        }
        label = c"".as_ptr();
        colon = c"".as_ptr();
        if !label_pc.is_null() {
            unsafe {
                label = labels
                    .names
                    .as_ptr()
                    .add(label_pc.offset_from(labels.pcs.as_ptr()) as usize)
                    .cast::<c_char>();
            }
            colon = c":".as_ptr();
        }
        unsafe {
            fprintf(text_out, c"%x:\t".as_ptr(), pc as c_uint);
        }
        i = 0;
        while i < cnt {
            unsafe {
                fprintf(text_out, c"%02x ".as_ptr(), *image.add((pc + i as __u32) as usize) as c_int);
            }
            i += 1;
        }
        i = cnt * 3;
        while i < 12 * 3 {
            unsafe {
                fputc(' ' as c_int, text_out);
            }
            i += 1;
        }
        unsafe {
            fprintf(text_out, c"%s%s%s\n".as_ptr(), label, colon, buf.as_ptr());
        }
        pc += cnt as __u32;
    }

    unsafe {
        goto_out(&mut triple, &mut ctx);
    }
    err
}

unsafe fn goto_out(triple: *mut *mut c_char, ctx: *mut LLVMDisasmContextRef) {
    unsafe {
        if !(*triple).is_null() {
            LLVMDisposeMessage(*triple);
        }
        if !(*ctx).is_null() {
            LLVMDisasmDispose(*ctx);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_jited_program_text(
    fd: c_int,
    text: *mut c_char,
    text_sz: size_t,
) -> c_int {
    let mut info: bpf_prog_info = unsafe { core::mem::zeroed() };
    let mut info_len: __u32 = core::mem::size_of_val(&info) as __u32;
    let jited_funcs: __u32;
    let len: __u32;
    let mut pc: __u32;
    let mut func_lens: *mut __u32 = core::ptr::null_mut();
    let mut text_out: *mut FILE = core::ptr::null_mut();
    let mut image: *mut uint8_t = core::ptr::null_mut();
    let mut i: c_int;
    let mut err: c_int = 0;

    unsafe {
        if !llvm_initialized {
            LLVMInitializeAllTargetInfos();
            LLVMInitializeAllTargetMCs();
            LLVMInitializeAllDisassemblers();
            llvm_initialized = true;
        }

        text_out = fmemopen(text as *mut c_void, text_sz, c"w".as_ptr());
        if !ASSERT_OK_PTR(text_out as *const c_void, c"open_memstream".as_ptr()) {
            err = -errno;
            goto_text_out(text_out, image, func_lens as *mut c_void);
            return err;
        }

        /* first call is to find out jited program len */
        err = bpf_prog_get_info_by_fd(fd, &mut info, &mut info_len);
        if !ASSERT_OK(err, c"bpf_prog_get_info_by_fd #1".as_ptr()) {
            goto_text_out(text_out, image, func_lens as *mut c_void);
            return err;
        }

        len = info.jited_prog_len;
        image = malloc(len as size_t) as *mut uint8_t;
        if !ASSERT_OK_PTR(image as *const c_void, c"malloc(info.jited_prog_len)".as_ptr()) {
            err = -ENOMEM;
            goto_text_out(text_out, image, func_lens as *mut c_void);
            return err;
        }

        jited_funcs = info.nr_jited_func_lens;
        func_lens = malloc((jited_funcs as size_t) * core::mem::size_of::<__u32>()) as *mut __u32;
        if !ASSERT_OK_PTR(func_lens as *const c_void, c"malloc(info.nr_jited_func_lens)".as_ptr()) {
            err = -ENOMEM;
            goto_text_out(text_out, image, func_lens as *mut c_void);
            return err;
        }

        memset(
            &mut info as *mut bpf_prog_info as *mut c_void,
            0,
            core::mem::size_of_val(&info),
        );
        info.jited_prog_insns = image as __u64;
        info.jited_prog_len = len;
        info.jited_func_lens = func_lens as __u64;
        info.nr_jited_func_lens = jited_funcs;
        err = bpf_prog_get_info_by_fd(fd, &mut info, &mut info_len);
        if !ASSERT_OK(err, c"bpf_prog_get_info_by_fd #2".as_ptr()) {
            goto_text_out(text_out, image, func_lens as *mut c_void);
            return err;
        }

        pc = 0;
        i = 0;
        while i < jited_funcs as c_int {
            fprintf(text_out, c"func #%d:\n".as_ptr(), i);
            disasm_one_func(text_out, image.add(pc as usize), *func_lens.add(i as usize));
            fprintf(text_out, c"\n".as_ptr());
            pc += *func_lens.add(i as usize);
            i += 1;
        }

        goto_text_out(text_out, image, func_lens as *mut c_void);
    }
    err
}

unsafe fn goto_text_out(text_out: *mut FILE, image: *mut uint8_t, func_lens: *mut c_void) {
    unsafe {
        if !text_out.is_null() {
            fclose(text_out);
        }
        if !image.is_null() {
            free(image as *mut c_void);
        }
        if !func_lens.is_null() {
            free(func_lens);
        }
    }
}

// #else /* HAVE_LLVM_SUPPORT */
//
// int get_jited_program_text(int fd, char *text, size_t text_sz)
// {
//     if (env.verbosity >= VERBOSE_VERY)
//         printf("compiled w/o llvm development libraries, can't dis-assembly binary code");
//     return -EOPNOTSUPP;
// }
//
// #endif /* HAVE_LLVM_SUPPORT */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
