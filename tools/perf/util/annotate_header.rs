/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from perf/util/annotate.h. C include dependencies are expected
 * to be supplied by the surrounding Rust translation.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

pub const ANNOTATION__IPC_WIDTH: c_int = 6;
pub const ANNOTATION__CYCLES_WIDTH: c_int = 6;
pub const ANNOTATION__MINMAX_CYCLES_WIDTH: c_int = 19;
pub const ANNOTATION__AVG_IPC_WIDTH: c_int = 36;
pub const ANNOTATION__BR_CNTR_WIDTH: c_int = 30;
pub const ANNOTATION_DUMMY_LEN: c_int = 256;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum perf_disassembler {
    PERF_DISASM_UNKNOWN = 0,
    PERF_DISASM_LLVM,
    PERF_DISASM_CAPSTONE,
    PERF_DISASM_OBJDUMP,
}

pub const MAX_DISASSEMBLERS: usize = perf_disassembler::PERF_DISASM_OBJDUMP as usize + 1;

#[repr(C)]
pub struct annotation_options {
    pub hide_src_code: bool,
    pub hide_src_code_on_title: bool,
    pub use_offset: bool,
    pub jump_arrows: bool,
    pub print_lines: bool,
    pub full_path: bool,
    pub show_linenr: bool,
    pub show_fileloc: bool,
    pub show_nr_jumps: bool,
    pub show_minmax_cycle: bool,
    pub show_asm_raw: bool,
    pub show_br_cntr: bool,
    pub annotate_src: bool,
    pub code_with_type: bool,
    pub full_addr: bool,
    pub offset_level: u8,
    pub disassemblers: [u8; MAX_DISASSEMBLERS],
    pub disassembler_used: u8,
    pub min_pcnt: c_int,
    pub max_lines: c_int,
    pub context: c_int,
    pub objdump_path: *mut c_char,
    pub disassembler_style: *mut c_char,
    pub prefix: *const c_char,
    pub prefix_strip: *const c_char,
    pub percent_type: c_uint,
}

unsafe extern "C" {
    pub static mut annotate_opts: annotation_options;
}

pub const ANNOTATION__OFFSET_JUMP_TARGETS: c_int = 1;
pub const ANNOTATION__OFFSET_CALL: c_int = 2;
pub const ANNOTATION__MAX_OFFSET_LEVEL: c_int = 3;
pub const ANNOTATION__MIN_OFFSET_LEVEL: c_int = ANNOTATION__OFFSET_JUMP_TARGETS;

#[repr(C)]
pub struct annotation {
    pub src: *mut annotated_source,
    pub branch: *mut annotated_branch,
}

#[repr(C)]
pub struct sym_hist_entry {
    pub nr_samples: u64,
    pub period: u64,
}

pub const PERCENT_HITS_LOCAL: c_uint = 0;
pub const PERCENT_HITS_GLOBAL: c_uint = 1;
pub const PERCENT_PERIOD_LOCAL: c_uint = 2;
pub const PERCENT_PERIOD_GLOBAL: c_uint = 3;
pub const PERCENT_MAX: usize = 4;

#[repr(C)]
pub struct annotation_data {
    pub percent: [f64; PERCENT_MAX],
    pub percent_sum: f64,
    pub he: sym_hist_entry,
}

#[repr(C)]
pub struct cycles_info {
    pub ipc: f32,
    pub avg: u64,
    pub max: u64,
    pub min: u64,
}

#[repr(C)]
pub struct annotation_line {
    pub node: list_head,
    pub rb_node: rb_node,
    pub offset: i64,
    pub line: *mut c_char,
    pub line_nr: c_int,
    pub fileloc: *mut c_char,
    pub path: *mut c_char,
    pub cycles: *mut cycles_info,
    pub num_aggr: c_int,
    pub br_cntr_nr: c_int,
    pub br_cntr: *mut u64,
    pub evsel: *mut evsel,
    pub jump_sources: c_int,
    pub idx: u32,
    pub idx_asm: c_int,
    pub data_nr: c_int,
    pub data: [annotation_data; 0],
}

#[repr(C)]
pub union disasm_line_raw {
    pub bytes: [u8; 4],
    pub raw_insn: u32,
}

#[repr(C)]
pub struct disasm_line {
    pub ins: ins,
    pub ops: ins_operands,
    pub raw: disasm_line_raw,
    /* This needs to be at the end. */
    pub al: annotation_line,
}

unsafe extern "C" {
    pub static perf_disassembler__strs: [*const c_char; MAX_DISASSEMBLERS];

    pub fn annotation_line__add(al: *mut annotation_line, head: *mut list_head);
}

pub unsafe fn annotation_data__percent(data: *mut annotation_data, which: c_uint) -> f64 {
    if (which as usize) < PERCENT_MAX {
        unsafe { (*data).percent[which as usize] }
    } else {
        -1.0
    }
}

pub unsafe fn percent_type_str(type_: c_uint) -> *const c_char {
    static STR_LOCAL_HITS: &[u8] = b"local hits\0";
    static STR_GLOBAL_HITS: &[u8] = b"global hits\0";
    static STR_LOCAL_PERIOD: &[u8] = b"local period\0";
    static STR_GLOBAL_PERIOD: &[u8] = b"global period\0";
    static STR_NA: &[u8] = b"N/A\0";
    static STR: [*const c_char; PERCENT_MAX] = [
        STR_LOCAL_HITS.as_ptr() as *const c_char,
        STR_GLOBAL_HITS.as_ptr() as *const c_char,
        STR_LOCAL_PERIOD.as_ptr() as *const c_char,
        STR_GLOBAL_PERIOD.as_ptr() as *const c_char,
    ];

    if WARN_ON(type_ >= PERCENT_MAX as c_uint) {
        return STR_NA.as_ptr() as *const c_char;
    }

    STR[type_ as usize]
}

pub unsafe fn disasm_line(al: *mut annotation_line) -> *mut disasm_line {
    if !al.is_null() {
        container_of!(al, disasm_line, al)
    } else {
        core::ptr::null_mut()
    }
}

/*
 * Is this offset in the same function as the line it is used?
 * asm functions jump to other functions, for instance.
 */
pub unsafe fn disasm_line__has_local_offset(dl: *const disasm_line) -> bool {
    unsafe { (*dl).ops.target.offset_avail && !(*dl).ops.target.outside }
}

/*
 * Can we draw an arrow from the jump to its target, for instance? I.e.
 * is the jump and its target in the same function?
 */
unsafe extern "C" {
    pub fn disasm_line__is_valid_local_jump(dl: *mut disasm_line, sym: *mut symbol) -> bool;
    pub fn annotation_line__next(
        pos: *mut annotation_line,
        head: *mut list_head,
    ) -> *mut annotation_line;
}

#[repr(C)]
pub struct annotation_write_ops {
    pub first_line: bool,
    pub current_entry: bool,
    pub change_color: bool,
    pub width: c_int,
    pub obj: *mut c_void,
    pub set_color: Option<unsafe extern "C" fn(obj: *mut c_void, color: c_int) -> c_int>,
    pub set_percent_color:
        Option<unsafe extern "C" fn(obj: *mut c_void, percent: f64, current: bool)>,
    pub set_jumps_percent_color:
        Option<unsafe extern "C" fn(obj: *mut c_void, nr: c_int, current: bool) -> c_int>,
    pub printf: Option<unsafe extern "C" fn(obj: *mut c_void, fmt: *const c_char, ...)>,
    pub write_graph: Option<unsafe extern "C" fn(obj: *mut c_void, graph: c_int)>,
}

#[repr(C)]
pub struct annotation_print_data {
    pub he: *mut hist_entry,
    pub evsel: *mut evsel,
    pub arch: *const arch,
    pub dbg: *mut debuginfo,
    /* save data type info keyed by al->offset */
    pub type_hash: *mut hashmap,
    /* It'll be set in hist_entry__annotate_printf() */
    pub addr_fmt_width: c_int,
}

unsafe extern "C" {
    pub fn annotation_line__write(
        al: *mut annotation_line,
        notes: *mut annotation,
        ops: *const annotation_write_ops,
        apd: *mut annotation_print_data,
    );
    pub fn __annotation__scnprintf_samples_period(
        notes: *mut annotation,
        bf: *mut c_char,
        size: usize,
        evsel: *mut evsel,
        show_freq: bool,
    ) -> c_int;
    pub fn disasm__fprintf(head: *mut list_head, fp: *mut FILE) -> usize;
    pub fn symbol__calc_percent(sym: *mut symbol, evsel: *mut evsel);
}

/**
 * struct sym_hist - symbol histogram information for an event
 *
 * @nr_samples: Total number of samples.
 * @period: Sum of sample periods.
 */
#[repr(C)]
pub struct sym_hist {
    pub nr_samples: u64,
    pub period: u64,
}

/**
 * struct cyc_hist - (CPU) cycle histogram for a basic block
 *
 * @start: Start address of current block (if known).
 * @cycles: Sum of cycles for the longest basic block.
 * @cycles_aggr: Total cycles for this address.
 * @cycles_max: Max cycles for this address.
 * @cycles_min: Min cycles for this address.
 * @cycles_spark: History of cycles for the longest basic block.
 * @num: Number of samples for the longest basic block.
 * @num_aggr: Total number of samples for this address.
 * @have_start: Whether the current branch info has a start address.
 * @reset: Number of resets due to a different start address.
 *
 * If sample has branch_stack and cycles info, it can construct basic blocks
 * between two adjacent branches.  It'd have start and end addresses but
 * sometimes the start address may not be available.  So the cycles are
 * accounted at the end address.  If multiple basic blocks end at the same
 * address, it will take the longest one.
 *
 * The @start, @cycles, @cycles_spark and @num fields are used for the longest
 * block only.  Other fields are used for all cases.
 *
 * See __symbol__account_cycles().
 */
#[repr(C)]
pub struct cyc_hist {
    pub start: u64,
    pub cycles: u64,
    pub cycles_aggr: u64,
    pub cycles_max: u64,
    pub cycles_min: u64,
    pub cycles_spark: [i64; NUM_SPARKS],
    pub num: u32,
    pub num_aggr: u32,
    pub have_start: u8,
    /* 1 byte padding */
    pub reset: u16,
}

/**
 * struct annotated_source - symbols with hits have this attached as in annotation
 *
 * @source: List head for annotated_line (embeded in disasm_line).
 * @histograms: Array of symbol histograms per event to maintain the total number
 *              of samples and period.
 * @nr_histograms: This may not be the same as evsel->evlist->core.nr_entries if
 *                 we have more than a group in a evlist, where we will want
 *                 to see each group separately, that is why symbol__annotate2()
 *                 sets src->nr_histograms to evsel->nr_members.
 * @samples: Hash map of sym_hist_entry.  Keyed by event index and offset in symbol.
 * @nr_events: Number of events in the current output.
 * @nr_entries: Number of annotated_line in the source list.
 * @nr_asm_entries: Number of annotated_line with actual asm instruction in the
 *                  source list.
 * @max_jump_sources: Maximum number of jump instructions targeting to the same
 *                    instruction.
 * @widths: Precalculated width of each column in the TUI output.
 *
 * disasm_lines are allocated, percentages calculated and all sorted by percentage
 * when the annotation is about to be presented, so the percentages are for
 * one of the entries in the histogram array, i.e. for the event/counter being
 * presented. It is deallocated right after symbol__{tui,tty,etc}_annotate
 * returns.
 */
#[repr(C)]
pub struct annotated_source {
    pub source: list_head,
    pub histograms: *mut sym_hist,
    pub samples: *mut hashmap,
    pub nr_histograms: c_int,
    pub nr_events: c_int,
    pub nr_entries: c_int,
    pub nr_asm_entries: c_int,
    pub max_jump_sources: c_int,
    pub tried_source: bool,
    pub start: u64,
    pub widths: annotated_source_widths,
}

#[repr(C)]
pub struct annotated_source_widths {
    pub addr: u8,
    pub jumps: u8,
    pub target: u8,
    pub min_addr: u8,
    pub max_addr: u8,
    pub max_ins_name: u8,
    pub max_line_len: u16,
}

unsafe extern "C" {
    pub fn annotated_source__get_line(
        src: *mut annotated_source,
        offset: i64,
    ) -> *mut annotation_line;
}

/* A branch counter once saturated */
pub const ANNOTATION__BR_CNTR_SATURATED_FLAG: u64 = 1u64 << 63;

/**
 * struct annotated_branch - basic block and IPC information for a symbol.
 *
 * @hit_cycles: Total executed cycles.
 * @hit_insn: Total number of instructions executed.
 * @total_insn: Number of instructions in the function.
 * @cover_insn: Number of distinct, actually executed instructions.
 * @cycles_hist: Array of cyc_hist for each instruction.
 * @max_coverage: Maximum number of covered basic block (used for block-range).
 * @br_cntr: Array of the occurrences of events (branch counters) during a block.
 *
 * This struct is used by two different codes when the sample has branch stack
 * and cycles information.  annotation__compute_ipc() calculates average IPC
 * using @hit_insn / @hit_cycles.  The actual coverage can be calculated using
 * @cover_insn / @total_insn.  The @cycles_hist can give IPC for each (longest)
 * basic block ends at the given address.
 * process_basic_block() calculates coverage of instructions (or basic blocks)
 * in the function.
 */
#[repr(C)]
pub struct annotated_branch {
    pub hit_cycles: u64,
    pub hit_insn: u64,
    pub total_insn: c_uint,
    pub cover_insn: c_uint,
    pub cycles_hist: *mut cyc_hist,
    pub max_coverage: u64,
    pub br_cntr: *mut u64,
}

/* C used `struct LOCKABLE annotation`; lock annotation attributes are preserved
 * by comments on the declarations below.
 */
pub unsafe fn annotation__init(_notes: *mut annotation) {}

unsafe extern "C" {
    pub fn annotation__exit(notes: *mut annotation);
    /* EXCLUSIVE_LOCK_FUNCTION(*notes) */
    pub fn annotation__lock(notes: *mut annotation);
    /* UNLOCK_FUNCTION(*notes) */
    pub fn annotation__unlock(notes: *mut annotation);
    /* EXCLUSIVE_TRYLOCK_FUNCTION(true, *notes) */
    pub fn annotation__trylock(notes: *mut annotation) -> bool;
}

pub unsafe fn annotation__cycles_width(notes: *mut annotation) -> c_int {
    unsafe {
        if !(*notes).branch.is_null() && annotate_opts.show_minmax_cycle {
            return ANNOTATION__IPC_WIDTH + ANNOTATION__MINMAX_CYCLES_WIDTH;
        }

        if !(*notes).branch.is_null() {
            ANNOTATION__IPC_WIDTH + ANNOTATION__CYCLES_WIDTH
        } else {
            0
        }
    }
}

pub unsafe fn annotation__pcnt_width(notes: *mut annotation) -> c_int {
    unsafe {
        (if symbol_conf.show_total_period { 12 } else { 8 }) * (*(*notes).src).nr_events
    }
}

pub unsafe fn annotation_line__filter(al: *mut annotation_line) -> bool {
    unsafe { annotate_opts.hide_src_code && (*al).offset == -1 }
}

pub unsafe fn annotation__br_cntr_width() -> u8 {
    unsafe {
        if annotate_opts.show_br_cntr {
            ANNOTATION__BR_CNTR_WIDTH as u8
        } else {
            0
        }
    }
}

unsafe extern "C" {
    pub fn annotation__update_column_widths(notes: *mut annotation);
    pub fn annotation__toggle_full_addr(notes: *mut annotation, ms: *mut map_symbol);
}

pub unsafe fn annotated_source__histogram(
    src: *mut annotated_source,
    evsel: *const evsel,
) -> *mut sym_hist {
    unsafe { (*src).histograms.add((*evsel).core.idx as usize) }
}

pub unsafe fn annotation__histogram(
    notes: *mut annotation,
    evsel: *const evsel,
) -> *mut sym_hist {
    unsafe { annotated_source__histogram((*notes).src, evsel) }
}

pub unsafe fn annotated_source__hist_entry(
    src: *mut annotated_source,
    evsel: *const evsel,
    offset: u64,
) -> *mut sym_hist_entry {
    let mut entry: *mut sym_hist_entry = core::ptr::null_mut();
    let key: isize = ((offset << 16) | unsafe { (*evsel).core.idx as u64 }) as isize;

    if unsafe { !hashmap__find((*src).samples, key, &mut entry) } {
        return core::ptr::null_mut();
    }
    entry
}

pub unsafe fn symbol__annotation(sym: *mut symbol) -> *mut annotation {
    unsafe { (sym as *mut u8).sub(symbol_conf.priv_size as usize) as *mut annotation }
}

unsafe extern "C" {
    pub fn addr_map_symbol__inc_samples(
        ams: *mut addr_map_symbol,
        sample: *mut perf_sample,
    ) -> c_int;
    pub fn annotation__get_branch(notes: *mut annotation) -> *mut annotated_branch;
    pub fn addr_map_symbol__account_cycles(
        ams: *mut addr_map_symbol,
        start: *mut addr_map_symbol,
        cycles: c_uint,
        evsel: *mut evsel,
        br_cntr: u64,
    ) -> c_int;
    pub fn hist_entry__inc_addr_samples(
        he: *mut hist_entry,
        sample: *mut perf_sample,
        addr: u64,
    ) -> c_int;
    pub fn symbol__hists(sym: *mut symbol, nr_hists: c_int) -> *mut annotated_source;
    pub fn symbol__annotate_zero_histograms(sym: *mut symbol);
    pub fn symbol__annotate(
        ms: *mut map_symbol,
        evsel: *mut evsel,
        parch: *mut *const arch,
    ) -> c_int;
    pub fn symbol__annotate2(
        ms: *mut map_symbol,
        evsel: *mut evsel,
        parch: *mut *const arch,
    ) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum symbol_disassemble_errno {
    SYMBOL_ANNOTATE_ERRNO__SUCCESS = 0,

    /*
     * Choose an arbitrary negative big number not to clash with standard
     * errno since SUS requires the errno has distinct positive values.
     * See 'Issue 6' in the link below.
     *
     * http://pubs.opengroup.org/onlinepubs/9699919799/basedefs/errno.h.html
     */
    __SYMBOL_ANNOTATE_ERRNO__START = -10000,

    SYMBOL_ANNOTATE_ERRNO__NO_VMLINUX = -10000,
    SYMBOL_ANNOTATE_ERRNO__NO_LIBOPCODES_FOR_BPF,
    SYMBOL_ANNOTATE_ERRNO__ARCH_INIT_CPUID_PARSING,
    SYMBOL_ANNOTATE_ERRNO__ARCH_INIT_REGEXP,
    SYMBOL_ANNOTATE_ERRNO__BPF_INVALID_FILE,
    SYMBOL_ANNOTATE_ERRNO__BPF_MISSING_BTF,
    SYMBOL_ANNOTATE_ERRNO__COULDNT_DETERMINE_FILE_TYPE,

    __SYMBOL_ANNOTATE_ERRNO__END,
}

unsafe extern "C" {
    pub fn symbol__strerror_disassemble(
        ms: *mut map_symbol,
        errnum: c_int,
        buf: *mut c_char,
        buflen: usize,
    ) -> c_int;
    pub fn symbol__annotate_zero_histogram(sym: *mut symbol, evsel: *mut evsel);
    pub fn symbol__annotate_decay_histogram(sym: *mut symbol, evsel: *mut evsel);
    pub fn annotated_source__purge(as_: *mut annotated_source);
    pub fn map_symbol__annotation_dump(
        ms: *mut map_symbol,
        evsel: *mut evsel,
        he: *mut hist_entry,
    ) -> c_int;
    pub fn ui__has_annotation() -> bool;
    pub fn hist_entry__annotate_printf(he: *mut hist_entry, evsel: *mut evsel) -> c_int;
    pub fn hist_entry__tty_annotate(he: *mut hist_entry, evsel: *mut evsel) -> c_int;
    pub fn hist_entry__tty_annotate2(he: *mut hist_entry, evsel: *mut evsel) -> c_int;
    pub fn annotation_options__init();
    pub fn annotation_options__exit();
    pub fn annotation_config__init();
    pub fn annotate_parse_percent_type(
        opt: *const option,
        _str: *const c_char,
        unset: c_int,
    ) -> c_int;
    pub fn annotate_check_args() -> c_int;
}

/**
 * struct annotated_op_loc - Location info of instruction operand
 * @reg1: First register in the operand
 * @reg2: Second register in the operand
 * @offset: Memory access offset in the operand
 * @segment: Segment selector register
 * @mem_ref: Whether the operand accesses memory
 * @multi_regs: Whether the second register is used
 * @imm: Whether the operand is an immediate value (in offset)
 */
#[repr(C)]
pub struct annotated_op_loc {
    pub reg1: c_int,
    pub reg2: c_int,
    pub offset: c_int,
    pub segment: u8,
    pub mem_ref: bool,
    pub multi_regs: bool,
    pub imm: bool,
}

pub const INSN_OP_SOURCE: c_uint = 0;
pub const INSN_OP_TARGET: c_uint = 1;
pub const INSN_OP_MAX: usize = 2;

pub const INSN_SEG_NONE: c_uint = 0;
pub const INSN_SEG_X86_CS: c_uint = 1;
pub const INSN_SEG_X86_DS: c_uint = 2;
pub const INSN_SEG_X86_ES: c_uint = 3;
pub const INSN_SEG_X86_FS: c_uint = 4;
pub const INSN_SEG_X86_GS: c_uint = 5;
pub const INSN_SEG_X86_SS: c_uint = 6;

/**
 * struct annotated_insn_loc - Location info of instruction
 * @ops: Array of location info for source and target operands
 */
#[repr(C)]
pub struct annotated_insn_loc {
    pub ops: [annotated_op_loc; INSN_OP_MAX],
}

/* for_each_insn_op_loc(insn_loc, i, op_loc):
 * for (i = INSN_OP_SOURCE, op_loc = &(insn_loc)->ops[i];
 *      i < INSN_OP_MAX;
 *      i++, op_loc++)
 */

/* Get detailed location info in the instruction */
unsafe extern "C" {
    pub fn annotate_get_insn_location(
        arch: *const arch,
        dl: *mut disasm_line,
        loc: *mut annotated_insn_loc,
    ) -> c_int;

    /* Returns a data type from the sample instruction (if any) */
    pub fn hist_entry__get_data_type(he: *mut hist_entry) -> *mut annotated_data_type;
}

#[repr(C)]
pub struct annotated_item_stat {
    pub list: list_head,
    pub name: *mut c_char,
    pub good: c_int,
    pub bad: c_int,
}

unsafe extern "C" {
    pub static mut ann_insn_stat: list_head;

    /* Calculate PC-relative address */
    pub fn annotate_calc_pcrel(
        ms: *mut map_symbol,
        ip: u64,
        offset: c_int,
        dl: *mut disasm_line,
    ) -> u64;
}

/**
 * struct annotated_basic_block - Basic block of instructions
 * @list: List node
 * @begin: start instruction in the block
 * @end: end instruction in the block
 */
#[repr(C)]
pub struct annotated_basic_block {
    pub list: list_head,
    pub begin: *mut disasm_line,
    pub end: *mut disasm_line,
}

unsafe extern "C" {
    /* Get a list of basic blocks from src to dst addresses */
    pub fn annotate_get_basic_blocks(
        sym: *mut symbol,
        src: i64,
        dst: i64,
        head: *mut list_head,
    ) -> c_int;

    pub fn debuginfo_cache__delete();

    pub fn annotation_br_cntr_entry(
        str_: *mut *mut c_char,
        br_cntr_nr: c_int,
        br_cntr: *mut u64,
        num_aggr: c_int,
        evsel: *mut evsel,
    ) -> c_int;
    pub fn annotation_br_cntr_abbr_list(
        str_: *mut *mut c_char,
        evsel: *mut evsel,
        header: bool,
    ) -> c_int;

    pub fn map_symbol__get_arch(ms: *mut map_symbol, parch: *mut *const arch) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
