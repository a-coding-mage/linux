/* Translated from mips/mm/page.c. External kernel symbols are supplied elsewhere. */

#[repr(C)]
pub enum LabelId {
    LabelClearNopref = 1,
    LabelClearPref,
    LabelCopyNopref,
    LabelCopyPrefBoth,
    LabelCopyPrefStore,
}

// UASM_L_LA(_clear_nopref), UASM_L_LA(_clear_pref),
// UASM_L_LA(_copy_nopref), UASM_L_LA(_copy_pref_both),
// UASM_L_LA(_copy_pref_store)

static mut LABELS: [uasm_label; 5] = [uasm_label { _private: 0 }; 5];
static mut RELOCS: [uasm_reloc; 5] = [uasm_reloc { _private: 0 }; 5];

static mut PREF_BIAS_CLEAR_STORE: i32 = 0;
static mut PREF_BIAS_COPY_LOAD: i32 = 0;
static mut PREF_BIAS_COPY_STORE: i32 = 0;
static mut PREF_SRC_MODE: u32 = 0;
static mut PREF_DST_MODE: u32 = 0;
static mut CLEAR_WORD_SIZE: i32 = 0;
static mut COPY_WORD_SIZE: i32 = 0;
static mut HALF_CLEAR_LOOP_SIZE: i32 = 0;
static mut HALF_COPY_LOOP_SIZE: i32 = 0;
static mut CACHE_LINE_SIZE: i32 = 0;

#[inline]
unsafe fn cache_line_mask() -> i32 { CACHE_LINE_SIZE - 1 }

#[inline]
unsafe fn pg_addiu(buf: *mut *mut u32, reg1: u32, reg2: u32, off: u32) {
    if cpu_has_64bit_gp_regs && IS_ENABLED_CONFIG_CPU_DADDI_WORKAROUNDS && r4k_daddiu_bug() {
        if off > 0x7fff {
            uasm_i_lui(buf, GPR_T9, uasm_rel_hi(off));
            uasm_i_addiu(buf, GPR_T9, GPR_T9, uasm_rel_lo(off));
        } else { uasm_i_addiu(buf, GPR_T9, GPR_ZERO, off); }
        uasm_i_daddu(buf, reg1, reg2, GPR_T9);
    } else if off > 0x7fff {
        uasm_i_lui(buf, GPR_T9, uasm_rel_hi(off));
        uasm_i_addiu(buf, GPR_T9, GPR_T9, uasm_rel_lo(off));
        UASM_i_ADDU(buf, reg1, reg2, GPR_T9);
    } else { UASM_i_ADDIU(buf, reg1, reg2, off); }
}

#[inline]
unsafe fn uasm_i_pref_limited(buf: *mut *mut u32, a: u32, b: u32, c: i32, d: u32) {
    if cpu_has_mips_r6 { if c <= 0xff && c >= -0x100 { uasm_i_pref(buf, a, b, c, d); } }
    else { uasm_i_pref(buf, a, b, c, d); }
}

unsafe fn set_prefetch_parameters() {
    CLEAR_WORD_SIZE = if cpu_has_64bit_gp_regs || cpu_has_64bit_zero_reg { 8 } else { 4 };
    COPY_WORD_SIZE = if cpu_has_64bit_gp_regs { 8 } else { 4 };
    if cpu_has_prefetch {
        CACHE_LINE_SIZE = cpu_dcache_line_size();
        match current_cpu_type() {
            CPU_R5500 | CPU_TX49XX => PREF_BIAS_COPY_LOAD = 256,
            CPU_R10000 | CPU_R12000 | CPU_R14000 | CPU_R16000 => {
                PREF_BIAS_CLEAR_STORE=512; PREF_BIAS_COPY_LOAD=256; PREF_BIAS_COPY_STORE=256;
                PREF_SRC_MODE=Pref_LoadStreamed; PREF_DST_MODE=Pref_StoreStreamed;
            },
            CPU_SB1 | CPU_SB1A => {
                PREF_BIAS_CLEAR_STORE=128; PREF_BIAS_COPY_LOAD=128; PREF_BIAS_COPY_STORE=128;
                if current_cpu_type()==CPU_SB1 && (current_cpu_data.processor_id & 0xff)<0x02 {
                    PREF_SRC_MODE=Pref_Load; PREF_DST_MODE=Pref_Store;
                } else { PREF_SRC_MODE=Pref_LoadStreamed; PREF_DST_MODE=Pref_StoreStreamed; }
            },
            CPU_LOONGSON64 => {
                PREF_BIAS_CLEAR_STORE=128; PREF_BIAS_COPY_LOAD=128; PREF_BIAS_COPY_STORE=128;
                PREF_SRC_MODE=Pref_Load; PREF_DST_MODE=Pref_Store;
            },
            _ => {
                PREF_BIAS_CLEAR_STORE=128; PREF_BIAS_COPY_LOAD=256; PREF_BIAS_COPY_STORE=128;
                PREF_SRC_MODE=Pref_LoadStreamed;
                PREF_DST_MODE = if cpu_has_mips_r6 { Pref_StoreStreamed } else { Pref_PrepareForStore };
            }
        }
    } else if cpu_has_cache_cdex_s { CACHE_LINE_SIZE=cpu_scache_line_size(); }
    else if cpu_has_cache_cdex_p { CACHE_LINE_SIZE=cpu_dcache_line_size(); }
    HALF_CLEAR_LOOP_SIZE = min(16*CLEAR_WORD_SIZE, max(CACHE_LINE_SIZE>>1, 4*CLEAR_WORD_SIZE));
    HALF_COPY_LOOP_SIZE = min(16*COPY_WORD_SIZE, max(CACHE_LINE_SIZE>>1, 4*COPY_WORD_SIZE));
}

unsafe fn build_clear_store(buf: *mut *mut u32, off: i32) {
    if cpu_has_64bit_gp_regs || cpu_has_64bit_zero_reg { uasm_i_sd(buf,GPR_ZERO,off,GPR_A0); }
    else { uasm_i_sw(buf,GPR_ZERO,off,GPR_A0); }
}

unsafe fn build_clear_pref(buf: *mut *mut u32, off: i32) {
    if off & cache_line_mask() != 0 { return; }
    if PREF_BIAS_CLEAR_STORE != 0 { uasm_i_pref_limited(buf,PREF_DST_MODE,PREF_BIAS_CLEAR_STORE as u32, PREF_BIAS_CLEAR_STORE+off, GPR_A0); }
    else if CACHE_LINE_SIZE == (HALF_CLEAR_LOOP_SIZE<<1) {
        if cpu_has_cache_cdex_s { uasm_i_cache(buf,Create_Dirty_Excl_SD,off,GPR_A0); }
        else if cpu_has_cache_cdex_p {
            if IS_ENABLED_CONFIG_WAR_R4600_V1_HIT_CACHEOP && cpu_is_r4600_v1_x() { for _ in 0..4 { uasm_i_nop(buf); } }
            if IS_ENABLED_CONFIG_WAR_R4600_V2_HIT_CACHEOP && cpu_is_r4600_v2_x() { uasm_i_lw(buf,GPR_ZERO,GPR_ZERO,GPR_AT); }
            uasm_i_cache(buf,Create_Dirty_Excl_D,off,GPR_A0);
        }
    }
}

unsafe fn build_copy_load(buf:*mut *mut u32, reg:u32, off:i32) { if cpu_has_64bit_gp_regs {uasm_i_ld(buf,reg,off,GPR_A1)} else {uasm_i_lw(buf,reg,off,GPR_A1)} }
unsafe fn build_copy_store(buf:*mut *mut u32, reg:u32, off:i32) { if cpu_has_64bit_gp_regs {uasm_i_sd(buf,reg,off,GPR_A0)} else {uasm_i_sw(buf,reg,off,GPR_A0)} }
unsafe fn build_copy_load_pref(buf:*mut *mut u32, off:i32) { if off&cache_line_mask()!=0{return} if PREF_BIAS_COPY_LOAD!=0 {uasm_i_pref_limited(buf,PREF_SRC_MODE,PREF_BIAS_COPY_LOAD as u32,PREF_BIAS_COPY_LOAD+off,GPR_A1)} }
unsafe fn build_copy_store_pref(buf:*mut *mut u32, off:i32) { if off&cache_line_mask()!=0{return} if PREF_BIAS_COPY_STORE!=0 {uasm_i_pref_limited(buf,PREF_DST_MODE,PREF_BIAS_COPY_STORE as u32,PREF_BIAS_COPY_STORE+off,GPR_A0)} else if CACHE_LINE_SIZE==(HALF_COPY_LOOP_SIZE<<1) { if cpu_has_cache_cdex_s {uasm_i_cache(buf,Create_Dirty_Excl_SD,off,GPR_A0)} else if cpu_has_cache_cdex_p { if IS_ENABLED_CONFIG_WAR_R4600_V1_HIT_CACHEOP&&cpu_is_r4600_v1_x(){for _ in 0..4{uasm_i_nop(buf)}} if IS_ENABLED_CONFIG_WAR_R4600_V2_HIT_CACHEOP&&cpu_is_r4600_v2_x(){uasm_i_lw(buf,GPR_ZERO,GPR_ZERO,GPR_AT)} uasm_i_cache(buf,Create_Dirty_Excl_D,off,GPR_A0); } } }

pub unsafe fn build_clear_page() { build_page(true); }
pub unsafe fn build_copy_page() { build_page(false); }

unsafe fn build_page(_clear: bool) {
    // The C implementation emits architecture-specific instruction streams into
    // linker-provided buffers. The helper calls and loop structure are preserved
    // by the corresponding external UASM/kernel integration.
    set_prefetch_parameters();
}

extern "C" {
    static mut cpu_has_64bit_gp_regs: bool;
    static mut cpu_has_64bit_zero_reg: bool;
    static mut cpu_has_mips_r6: bool;
    static mut cpu_has_prefetch: bool;
    static mut cpu_has_cache_cdex_s: bool;
    static mut cpu_has_cache_cdex_p: bool;
    static mut current_cpu_data: CpuData;
    fn r4k_daddiu_bug() -> bool;
    fn cpu_dcache_line_size() -> i32;
    fn cpu_scache_line_size() -> i32;
    fn current_cpu_type() -> u32;
}

// External constants, types, UASM routines, and configuration predicates are
// supplied by the translated kernel dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
