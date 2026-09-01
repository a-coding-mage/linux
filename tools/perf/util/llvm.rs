// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/util/llvm.c. C include dependencies are represented as
// external Rust declarations; build-time HAVE_LIBLLVM_SUPPORT conditionals are
// preserved with cfg comments/attributes where they affect behavior.

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};
use core::ptr;

pub type u8 = core::ffi::c_uchar;
pub type u64 = c_ulonglong;
pub type size_t = usize;
pub type bool_ = bool;

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct maps {
    pub map: *mut map,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct arch_id {
    pub e_machine: c_int,
}

#[repr(C)]
pub struct arch {
    pub name: *const c_char,
    pub id: arch_id,
}

#[repr(C)]
pub struct annotate_options {
    pub objdump_path: *const c_char,
    pub disassembler_style: *const c_char,
}

#[repr(C)]
pub struct annotate_args {
    pub ms: *mut maps,
    pub options: *mut annotate_options,
    pub arch: *mut arch,
    pub offset: u64,
    pub line: *mut c_char,
    pub line_nr: c_int,
    pub fileloc: *mut c_char,
}

#[repr(C)]
pub struct symbol {
    pub start: u64,
    pub name: *const c_char,
}

#[repr(C)]
pub struct inline_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct llvm_a2l_frame {
    pub filename: *mut c_char,
    pub funcname: *mut c_char,
    pub line: c_uint,
}

#[repr(C)]
pub struct annotation_source {
    pub source: annotation_line,
}

#[repr(C)]
pub struct annotation {
    pub src: *mut annotation_source,
}

#[repr(C)]
pub struct annotation_line {
    _private: [u8; 0],
}

#[repr(C)]
pub struct disasm_line {
    pub al: annotation_line,
}

#[cfg(feature = "libllvm-support")]
pub type LLVMDisasmContextRef = *mut c_void;

#[cfg(feature = "libllvm-support")]
const LLVMDisassembler_ReferenceType_In_Branch: u64 = 1;
#[cfg(feature = "libllvm-support")]
const LLVMDisassembler_ReferenceType_In_PCrel_Load: u64 = 2;
#[cfg(feature = "libllvm-support")]
const LLVMDisassembler_ReferenceType_InOut_None: u64 = 0;
#[cfg(feature = "libllvm-support")]
const LLVMDisassembler_Option_AsmPrinterVariant: c_int = 4;
#[cfg(feature = "libllvm-support")]
const LLVMDisassembler_Option_PrintImmHex: c_int = 16;

const EM_AARCH64: c_int = 183;

unsafe extern "C" {
    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;

    static mut errno: c_int;

    fn symbol__annotation(sym: *mut symbol) -> *mut annotation;
    fn map__dso(map: *mut map) -> *mut dso;
    fn map__rip_2objdump(map: *mut map, addr: u64) -> u64;
    fn dso__read_symbol(
        dso: *mut dso,
        filename: *const c_char,
        map: *mut map,
        sym: *mut symbol,
        code_buf: *mut *mut u8,
        buf_len: *mut u64,
        is_64bit: *mut bool,
    ) -> *const u8;
    fn arch__is_x86(arch: *mut arch) -> bool;
    fn disasm_line__new(args: *mut annotate_args) -> *mut disasm_line;
    fn annotation_line__add(al: *mut annotation_line, source: *mut annotation_line);
    fn expand_tabs(
        line: *const c_char,
        storage: *mut *mut c_char,
        storage_len: *mut size_t,
    ) -> *mut c_char;
    fn pr_debug(fmt: *const c_char, ...);

    #[cfg(feature = "libllvm-support")]
    fn llvm_addr2line(
        dso_name: *const c_char,
        addr: u64,
        file: *mut *mut c_char,
        line: *mut c_uint,
        unwind_inlines: bool,
        inline_frames: *mut *mut llvm_a2l_frame,
    ) -> c_int;
    #[cfg(feature = "libllvm-support")]
    fn new_inline_sym(dso: *mut dso, sym: *mut symbol, funcname: *mut c_char) -> *mut symbol;
    #[cfg(feature = "libllvm-support")]
    fn srcline_from_fileline(filename: *mut c_char, line: c_uint) -> *mut c_char;
    #[cfg(feature = "libllvm-support")]
    fn inline_list__append(
        inline_sym: *mut symbol,
        srcline: *mut c_char,
        node: *mut inline_node,
    ) -> c_int;
    #[cfg(feature = "libllvm-support")]
    fn LLVMInitializeAllTargetInfos();
    #[cfg(feature = "libllvm-support")]
    fn LLVMInitializeAllTargetMCs();
    #[cfg(feature = "libllvm-support")]
    fn LLVMInitializeAllDisassemblers();
    #[cfg(feature = "libllvm-support")]
    fn LLVMCreateDisasm(
        triple_name: *const c_char,
        dis_info: *mut c_void,
        tag_type: c_int,
        get_op_info: *mut c_void,
        symbol_look_up: Option<
            unsafe extern "C" fn(
                *mut c_void,
                u64,
                *mut u64,
                u64,
                *const *const c_char,
            ) -> *const c_char,
        >,
    ) -> LLVMDisasmContextRef;
    #[cfg(feature = "libllvm-support")]
    fn LLVMCreateDisasmCPUFeatures(
        triple: *const c_char,
        cpu: *const c_char,
        features: *const c_char,
        dis_info: *mut c_void,
        tag_type: c_int,
        get_op_info: *mut c_void,
        symbol_look_up: Option<
            unsafe extern "C" fn(
                *mut c_void,
                u64,
                *mut u64,
                u64,
                *const *const c_char,
            ) -> *const c_char,
        >,
    ) -> LLVMDisasmContextRef;
    #[cfg(feature = "libllvm-support")]
    fn LLVMSetDisasmOptions(disasm: LLVMDisasmContextRef, options: c_int) -> c_int;
    #[cfg(feature = "libllvm-support")]
    fn LLVMDisasmInstruction(
        dc: LLVMDisasmContextRef,
        bytes: *mut u8,
        bytes_size: u64,
        pc: u64,
        out_string: *mut c_char,
        out_string_size: size_t,
    ) -> size_t;
    #[cfg(feature = "libllvm-support")]
    fn LLVMDisasmDispose(dc: LLVMDisasmContextRef);
    #[cfg(feature = "libllvm-support")]
    fn llvm_name_for_code(dso: *mut dso, filename: *const c_char, addr: u64) -> *mut c_char;
    #[cfg(feature = "libllvm-support")]
    fn llvm_name_for_data(dso: *mut dso, filename: *const c_char, addr: u64) -> *mut c_char;
}

#[cfg(feature = "libllvm-support")]
unsafe fn zfree_char(ptrp: *mut *mut c_char) {
    if !(*ptrp).is_null() {
        free(*ptrp as *mut c_void);
        *ptrp = ptr::null_mut();
    }
}

#[cfg(feature = "libllvm-support")]
unsafe fn zfree_frames(ptrp: *mut *mut llvm_a2l_frame) {
    if !(*ptrp).is_null() {
        free(*ptrp as *mut c_void);
        *ptrp = ptr::null_mut();
    }
}

#[cfg(feature = "libllvm-support")]
unsafe fn free_llvm_inline_frames(inline_frames: *mut llvm_a2l_frame, num_frames: c_int) {
    if !inline_frames.is_null() {
        let mut i: c_int = 0;
        while i < num_frames {
            zfree_char(&mut (*inline_frames.add(i as usize)).filename);
            zfree_char(&mut (*inline_frames.add(i as usize)).funcname);
            i += 1;
        }
        let mut frames = inline_frames;
        zfree_frames(&mut frames);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn llvm__addr2line(
    dso_name: *const c_char,
    addr: u64,
    file: *mut *mut c_char,
    line: *mut c_uint,
    dso: *mut dso,
    unwind_inlines: bool,
    node: *mut inline_node,
    sym: *mut symbol,
) -> c_int {
    #[cfg(feature = "libllvm-support")]
    {
        let mut inline_frames: *mut llvm_a2l_frame = ptr::null_mut();
        let num_frames = llvm_addr2line(
            dso_name,
            addr,
            file,
            line,
            !node.is_null() && unwind_inlines,
            &mut inline_frames,
        );

        if num_frames == 0 || inline_frames.is_null() {
            /* Error, or we didn't want inlines. */
            return num_frames;
        }

        let mut i: c_int = 0;
        while i < num_frames {
            let inline_sym = new_inline_sym(dso, sym, (*inline_frames.add(i as usize)).funcname);
            let mut srcline: *mut c_char = ptr::null_mut();

            if !(*inline_frames.add(i as usize)).filename.is_null() {
                srcline = srcline_from_fileline(
                    (*inline_frames.add(i as usize)).filename,
                    (*inline_frames.add(i as usize)).line,
                );
            }
            if inline_list__append(inline_sym, srcline, node) != 0 {
                free_llvm_inline_frames(inline_frames, num_frames);
                return 0;
            }
            i += 1;
        }
        free_llvm_inline_frames(inline_frames, num_frames);

        return num_frames;
    }

    #[cfg(not(feature = "libllvm-support"))]
    {
        let _ = (dso_name, addr, file, line, dso, unwind_inlines, node, sym);
        return -1;
    }
}

#[cfg(feature = "libllvm-support")]
unsafe fn init_llvm() {
    static mut INIT: bool = false;

    if !INIT {
        LLVMInitializeAllTargetInfos();
        LLVMInitializeAllTargetMCs();
        LLVMInitializeAllDisassemblers();
        INIT = true;
    }
}

/*
 * Whenever LLVM wants to resolve an address into a symbol, it calls this
 * callback. We don't ever actually _return_ anything (in particular, because
 * it puts quotation marks around what we return), but we use this as a hint
 * that there is a branch or PC-relative address in the expression that we
 * should add some textual annotation for after the instruction. The caller
 * will use this information to add the actual annotation.
 */
#[cfg(feature = "libllvm-support")]
#[repr(C)]
struct symbol_lookup_storage {
    branch_addr: u64,
    pcrel_load_addr: u64,
}

#[cfg(feature = "libllvm-support")]
unsafe extern "C" fn symbol_lookup_callback(
    disinfo: *mut c_void,
    value: u64,
    ref_type: *mut u64,
    _address: u64,
    _ref_: *const *const c_char,
) -> *const c_char {
    let storage = disinfo as *mut symbol_lookup_storage;

    if *ref_type == LLVMDisassembler_ReferenceType_In_Branch {
        (*storage).branch_addr = value;
    } else if *ref_type == LLVMDisassembler_ReferenceType_In_PCrel_Load {
        (*storage).pcrel_load_addr = value;
    }
    *ref_type = LLVMDisassembler_ReferenceType_InOut_None;
    ptr::null()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbol__disassemble_llvm(
    filename: *const c_char,
    sym: *mut symbol,
    args: *mut annotate_args,
) -> c_int {
    #[cfg(feature = "libllvm-support")]
    {
        let notes = symbol__annotation(sym);
        let map = (*(*args).ms).map;
        let dso = map__dso(map);
        let start = map__rip_2objdump(map, (*sym).start);
        /* Malloc-ed buffer containing instructions read from disk. */
        let mut code_buf: *mut u8 = ptr::null_mut();
        /* Pointer to code to be disassembled. */
        let mut buf: *const u8;
        let mut buf_len: u64 = 0;
        let mut pc: u64;
        let mut is_64bit: bool = false;
        let mut disasm_buf = [0 as c_char; 2048];
        let mut disasm_len: size_t;
        let mut dl: *mut disasm_line;
        let mut disasm: LLVMDisasmContextRef = ptr::null_mut();
        let mut storage = symbol_lookup_storage {
            branch_addr: 0,
            pcrel_load_addr: 0,
        };
        let mut line_storage: *mut c_char = ptr::null_mut();
        let mut line_storage_len: size_t = 0;
        let mut ret: c_int = -1;

        if !(*(*args).options).objdump_path.is_null() {
            return -1;
        }

        buf = dso__read_symbol(
            dso,
            filename,
            map,
            sym,
            &mut code_buf,
            &mut buf_len,
            &mut is_64bit,
        );
        if buf.is_null() {
            return errno;
        }

        init_llvm();
        if arch__is_x86((*args).arch) {
            let triplet = if is_64bit {
                b"x86_64-pc-linux\0".as_ptr() as *const c_char
            } else {
                b"i686-pc-linux\0".as_ptr() as *const c_char
            };

            disasm = LLVMCreateDisasm(
                triplet,
                &mut storage as *mut _ as *mut c_void,
                0,
                ptr::null_mut(),
                Some(symbol_lookup_callback),
            );
        } else {
            let mut triplet = [0 as c_char; 64];
            let mut features: *const c_char = ptr::null();

            scnprintf(
                triplet.as_mut_ptr(),
                triplet.len(),
                b"%s-linux-gnu\0".as_ptr() as *const c_char,
                (*(*args).arch).name,
            );
            if (*(*args).arch).id.e_machine == EM_AARCH64 {
                features = b"+all\0".as_ptr() as *const c_char;
            }
            disasm = LLVMCreateDisasmCPUFeatures(
                triplet.as_ptr(),
                b"\0".as_ptr() as *const c_char,
                features,
                &mut storage as *mut _ as *mut c_void,
                0,
                ptr::null_mut(),
                Some(symbol_lookup_callback),
            );
        }

        if disasm.is_null() {
            goto_err(disasm, code_buf, line_storage);
            return ret;
        }

        if !(*(*args).options).disassembler_style.is_null()
            && strcmp(
                (*(*args).options).disassembler_style,
                b"intel\0".as_ptr() as *const c_char,
            ) == 0
        {
            LLVMSetDisasmOptions(disasm, LLVMDisassembler_Option_AsmPrinterVariant);
        }

        /*
         * This needs to be set after AsmPrinterVariant, due to a bug in LLVM;
         * setting AsmPrinterVariant makes a new instruction printer, making it
         * forget about the PrintImmHex flag (which is applied before if both
         * are given to the same call).
         */
        LLVMSetDisasmOptions(disasm, LLVMDisassembler_Option_PrintImmHex);

        /* add the function address and name */
        scnprintf(
            disasm_buf.as_mut_ptr(),
            disasm_buf.len(),
            b"%#llx <%s>:\0".as_ptr() as *const c_char,
            start,
            (*sym).name,
        );

        (*args).offset = !0u64;
        (*args).line = disasm_buf.as_mut_ptr();
        (*args).line_nr = 0;
        (*args).fileloc = ptr::null_mut();
        (*(*args).ms).sym = sym;

        dl = disasm_line__new(args);
        if dl.is_null() {
            LLVMDisasmDispose(disasm);
            free(code_buf as *mut c_void);
            free(line_storage as *mut c_void);
            return ret;
        }

        annotation_line__add(&mut (*dl).al, &mut (*(*notes).src).source);

        pc = start;
        let mut offset: u64 = 0;
        while offset < buf_len {
            let ins_len: c_uint;

            storage.branch_addr = 0;
            storage.pcrel_load_addr = 0;

            /*
             * LLVM's API has the code be disassembled as non-const, cast
             * here as we may be disassembling from mapped read-only memory.
             */
            ins_len = LLVMDisasmInstruction(
                disasm,
                buf.add(offset as usize) as *mut u8,
                buf_len - offset,
                pc,
                disasm_buf.as_mut_ptr(),
                disasm_buf.len(),
            ) as c_uint;
            if ins_len == 0 {
                LLVMDisasmDispose(disasm);
                free(code_buf as *mut c_void);
                free(line_storage as *mut c_void);
                return ret;
            }
            disasm_len = strlen(disasm_buf.as_ptr());

            if storage.branch_addr != 0 {
                let name = llvm_name_for_code(dso, filename, storage.branch_addr);
                if !name.is_null() {
                    disasm_len += scnprintf(
                        disasm_buf.as_mut_ptr().add(disasm_len),
                        disasm_buf.len() - disasm_len,
                        b" <%s>\0".as_ptr() as *const c_char,
                        name,
                    ) as size_t;
                    free(name as *mut c_void);
                }
            }
            if storage.pcrel_load_addr != 0 {
                let name = llvm_name_for_data(dso, filename, storage.pcrel_load_addr);
                disasm_len += scnprintf(
                    disasm_buf.as_mut_ptr().add(disasm_len),
                    disasm_buf.len() - disasm_len,
                    b"  # %#llx\0".as_ptr() as *const c_char,
                    storage.pcrel_load_addr,
                ) as size_t;
                if !name.is_null() {
                    disasm_len += scnprintf(
                        disasm_buf.as_mut_ptr().add(disasm_len),
                        disasm_buf.len() - disasm_len,
                        b" <%s>\0".as_ptr() as *const c_char,
                        name,
                    ) as size_t;
                    free(name as *mut c_void);
                }
            }

            (*args).offset = offset;
            (*args).line = expand_tabs(
                disasm_buf.as_ptr(),
                &mut line_storage,
                &mut line_storage_len,
            );
            (*args).line_nr = 0;
            (*args).fileloc = ptr::null_mut();
            (*(*args).ms).sym = sym;

            llvm_addr2line(
                filename,
                pc,
                &mut (*args).fileloc,
                &mut (*args).line_nr as *mut c_int as *mut c_uint,
                false,
                ptr::null_mut(),
            );

            dl = disasm_line__new(args);
            if dl.is_null() {
                LLVMDisasmDispose(disasm);
                free(code_buf as *mut c_void);
                free(line_storage as *mut c_void);
                return ret;
            }

            annotation_line__add(&mut (*dl).al, &mut (*(*notes).src).source);

            free((*args).fileloc as *mut c_void);
            pc += ins_len as u64;
            offset += ins_len as u64;
        }

        ret = 0;

        LLVMDisasmDispose(disasm);
        free(code_buf as *mut c_void);
        free(line_storage as *mut c_void);
        return ret;
    }

    #[cfg(not(feature = "libllvm-support"))]
    {
        let _ = args;
        pr_debug(
            b"The LLVM disassembler isn't linked in for %s in %s\n\0".as_ptr() as *const c_char,
            (*sym).name,
            filename,
        );
        return -1;
    }
}

#[cfg(feature = "libllvm-support")]
unsafe fn goto_err(disasm: LLVMDisasmContextRef, code_buf: *mut u8, line_storage: *mut c_char) {
    LLVMDisasmDispose(disasm);
    free(code_buf as *mut c_void);
    free(line_storage as *mut c_void);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
