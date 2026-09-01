// SPDX-License-Identifier: GPL-2.0-or-later
/*
 */

/*
 * 2002-07 Benny Sjostrand benny@hostmobility.com
 */

// Translated from pci/cs46xx/dsp_spos.c. C include dependencies are expected
// to be supplied by the surrounding driver translation.

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr::{null_mut};

type u16 = u16;
type u32 = u32;

extern "C" {
    static wide_opcodes_external_dependency_anchor: c_int;

    fn snd_BUG_ON(cond: bool) -> bool;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kmalloc(size: usize, flags: c_int) -> *mut c_void;
    fn vmalloc(size: usize) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn vfree(p: *mut c_void);
    fn kmemdup(src: *const c_void, len: usize, gfp: c_int) -> *mut c_void;
    fn udelay(usecs: u32);
    fn readl(addr: *mut c_void) -> u32;
    fn writel(val: u32, addr: *mut c_void);

    fn snd_cs46xx_download(chip: *mut snd_cs46xx, data: *mut u32, off: u32, size: u32) -> c_int;
    fn snd_cs46xx_clear_BA1(chip: *mut snd_cs46xx, off: u32, size: u32);
    fn snd_cs46xx_peekBA0(chip: *mut snd_cs46xx, reg: u32) -> c_int;
    fn snd_cs46xx_pokeBA0(chip: *mut snd_cs46xx, reg: u32, val: c_int);
    fn snd_cs46xx_poke(chip: *mut snd_cs46xx, reg: u32, val: u32);
    fn snd_cs46xx_peek(chip: *mut snd_cs46xx, reg: u32) -> u32;

    fn cs46xx_dsp_proc_free_scb_desc(desc: *mut dsp_scb_descriptor);
    fn cs46xx_dsp_proc_register_scb_desc(chip: *mut snd_cs46xx, desc: *mut dsp_scb_descriptor);
    fn snd_info_create_card_entry(card: *mut snd_card, name: *const c_char, parent: *mut snd_info_entry) -> *mut snd_info_entry;
    fn snd_info_set_text_ops(entry: *mut snd_info_entry, data: *mut c_void, read: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer));
    fn snd_info_free_entry(entry: *mut snd_info_entry);
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);

    fn cs46xx_dsp_create_timing_master_scb(chip: *mut snd_cs46xx) -> *mut dsp_scb_descriptor;
    fn cs46xx_dsp_create_codec_out_scb(chip: *mut snd_cs46xx, name: *mut c_char, span: u32, addr: u32, buf: u32, dest: u32, parent: *mut dsp_scb_descriptor, where_: c_int) -> *mut dsp_scb_descriptor;
    fn cs46xx_dsp_create_mix_only_scb(chip: *mut snd_cs46xx, name: *mut c_char, buf: u32, dest: u32, parent: *mut dsp_scb_descriptor, where_: c_int) -> *mut dsp_scb_descriptor;
    fn cs46xx_dsp_create_codec_in_scb(chip: *mut snd_cs46xx, name: *mut c_char, span: u32, addr: u32, buf: u32, dest: u32, parent: *mut dsp_scb_descriptor, where_: c_int) -> *mut dsp_scb_descriptor;
    fn cs46xx_dsp_create_mix_to_ostream_scb(chip: *mut snd_cs46xx, name: *mut c_char, buf: u32, spb: u32, dest: u32, parent: *mut dsp_scb_descriptor, where_: c_int) -> *mut dsp_scb_descriptor;
    fn cs46xx_dsp_create_vari_decimate_scb(chip: *mut snd_cs46xx, name: *mut c_char, buf0: u32, buf1: u32, dest: u32, parent: *mut dsp_scb_descriptor, where_: c_int) -> *mut dsp_scb_descriptor;
    fn cs46xx_dsp_create_magic_snoop_scb(chip: *mut snd_cs46xx, name: *mut c_char, dest: u32, buf: u32, codec_out: *mut dsp_scb_descriptor, clfe_out: *mut dsp_scb_descriptor, where_: c_int) -> *mut dsp_scb_descriptor;
    fn cs46xx_dsp_create_spio_write_scb(chip: *mut snd_cs46xx, name: *mut c_char, dest: u32, parent: *mut dsp_scb_descriptor, where_: c_int) -> *mut dsp_scb_descriptor;
    fn cs46xx_dsp_create_src_task_scb(chip: *mut snd_cs46xx, name: *mut c_char, rate: u32, out_buf: u32, delay_buf: u32, dest: u32, parent: *mut dsp_scb_descriptor, where_: c_int, flag: c_int) -> *mut dsp_scb_descriptor;
    fn cs46xx_dsp_create_asynch_fg_rx_scb(chip: *mut snd_cs46xx, name: *mut c_char, dest: u32, src_inst: u32, out_buf: u32, parent: *mut dsp_scb_descriptor, where_: c_int) -> *mut dsp_scb_descriptor;
    fn cs46xx_add_record_source(chip: *mut snd_cs46xx, parent: *mut dsp_scb_descriptor, dest: u32, name: *mut c_char) -> *mut dsp_scb_descriptor;
    fn cs46xx_dsp_remove_scb(chip: *mut snd_cs46xx, scb: *mut dsp_scb_descriptor);
    fn cs46xx_src_unlink(chip: *mut snd_cs46xx, scb: *mut dsp_scb_descriptor);
    fn cs46xx_src_link(chip: *mut snd_cs46xx, scb: *mut dsp_scb_descriptor);
    fn cs46xx_dsp_scb_set_volume(chip: *mut snd_cs46xx, scb: *mut dsp_scb_descriptor, left: u16, right: u16);
    fn cs46xx_dsp_spos_update_scb(chip: *mut snd_cs46xx, scb: *mut dsp_scb_descriptor);
    fn _wrap_all_bits(v: u32) -> u32;
}

#[repr(C)] pub struct snd_cs46xx { pub dsp_spos_instance: *mut dsp_spos_instance, pub card: *mut snd_card, pub region: cs46xx_region, pub nr_ac97_codecs: c_int, pub active_ctrl: unsafe extern "C" fn(*mut snd_cs46xx, c_int), pub amplifier_ctrl: unsafe extern "C" fn(*mut snd_cs46xx, c_int), pub spos_mutex: c_void, pub reg_lock: c_void }
#[repr(C)] pub struct snd_card { pub dev: *mut c_void, pub proc_root: *mut snd_info_entry }
#[repr(C)] pub struct snd_info_entry { pub private_data: *mut snd_cs46xx, pub mode: u32 }
#[repr(C)] pub struct snd_info_buffer { _private: [u8; 0] }
#[repr(C)] pub struct cs46xx_region { pub idx: [cs46xx_region_idx; 3] }
#[repr(C)] pub struct cs46xx_region_idx { pub remap_addr: *mut c_void }

// The full definitions of these DSP structs and constants are provided by the
// translated headers. Layout here is expressed only to make field use explicit.
#[repr(C)] pub struct dsp_spos_instance { pub symbol_table: dsp_symbol_table, pub code: dsp_code, pub modules: *mut dsp_module_desc, pub nscb: c_int, pub ntask: c_int, pub nmodules: c_int, pub spdif_in_sample_rate: u32, pub dac_volume_right: u16, pub dac_volume_left: u16, pub spdif_input_volume_right: u16, pub spdif_input_volume_left: u16, pub spdif_csuv_default: u32, pub spdif_csuv_stream: u32, pub scbs: [dsp_scb_descriptor; DSP_MAX_SCB_DESC as usize], pub tasks: [dsp_task_descriptor; DSP_MAX_TASK_DESC as usize], pub scb_highest_frag_index: c_int, pub the_null_scb: *mut dsp_scb_descriptor, pub master_mix_scb: *mut dsp_scb_descriptor, pub codec_in_scb: *mut dsp_scb_descriptor, pub record_mixer_scb: *mut dsp_scb_descriptor, pub rear_mix_scb: *mut dsp_scb_descriptor, pub center_lfe_mix_scb: *mut dsp_scb_descriptor, pub ref_snoop_scb: *mut dsp_scb_descriptor, pub spdif_in_src: *mut dsp_scb_descriptor, pub asynch_rx_scb: *mut dsp_scb_descriptor, pub pcm_input: *mut dsp_scb_descriptor, pub adc_input: *mut dsp_scb_descriptor, pub spdif_status_out: u32, pub spdif_status_in: u32, pub snd_card: *mut snd_card, pub proc_dsp_dir: *mut snd_info_entry }
#[repr(C)] pub struct dsp_symbol_table { pub symbols: *mut dsp_symbol_entry, pub nsymbols: c_int, pub highest_frag_index: c_int }
#[repr(C)] pub struct dsp_code { pub data: *mut u32, pub offset: u32, pub size: u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct dsp_symbol_entry { pub symbol_name: [c_char; 64], pub address: u32, pub symbol_type: c_int, pub module: *mut dsp_module_desc, pub deleted: c_int }
#[repr(C)] #[derive(Copy, Clone)] pub struct dsp_module_desc { pub module_name: *mut c_char, pub nsegments: c_int, pub segments: *mut dsp_segment_desc, pub symbol_table: dsp_symbol_table, pub overlay_begin_address: u32, pub load_address: u32, pub nfixups: c_int }
#[repr(C)] #[derive(Copy, Clone)] pub struct dsp_segment_desc { pub segment_type: c_int, pub offset: u32, pub size: u32, pub data: *mut u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct dsp_scb_descriptor { pub scb_name: [c_char; 64], pub address: u32, pub index: c_int, pub ref_count: c_int, pub scb_symbol: *mut dsp_symbol_entry, pub parent_scb_ptr: *mut dsp_scb_descriptor, pub sub_list_ptr: *mut dsp_scb_descriptor, pub next_scb_ptr: *mut dsp_scb_descriptor, pub task_entry: *mut dsp_symbol_entry, pub data: *mut u32, pub deleted: c_int, pub updated: c_int, pub volume_set: c_int, pub volume: [u16; 2] }
#[repr(C)] #[derive(Copy, Clone)] pub struct dsp_task_descriptor { pub task_name: [c_char; 64], pub address: u32, pub size: c_int, pub index: c_int, pub data: *mut u32 }

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EIO: c_int = 5;
const EBUSY: c_int = 16;
const GFP_KERNEL: c_int = 0;
const S_IFDIR: u32 = 0o040000;

const DSP_MAX_SYMBOLS: c_int = 1024;
const DSP_MAX_MODULES: c_int = 64;
const DSP_MAX_SCB_DESC: c_int = 256;
const DSP_MAX_TASK_DESC: c_int = 64;
const DSP_CODE_BYTE_SIZE: u32 = 0x10000;
const DSP_PARAMETER_BYTE_OFFSET: u32 = 0;
const DSP_PARAMETER_BYTE_SIZE: u32 = 0;
const DSP_SAMPLE_BYTE_OFFSET: u32 = 0;
const DSP_SAMPLE_BYTE_SIZE: u32 = 0;
const DSP_CODE_BYTE_OFFSET: u32 = 0;
const SEGTYPE_SP_PROGRAM: c_int = 0;
const SEGTYPE_SP_PARAMETER: c_int = 1;
const SEGTYPE_SP_SAMPLE: c_int = 2;
const SYMBOL_CONSTANT: c_int = 0;
const SYMBOL_CODE: c_int = 1;
const SYMBOL_PARAMETER: c_int = 2;

// Many numeric DSP constants are external translated-header dependencies.
extern "C" {
    static WIDE_FOR_BEGIN_LOOP: c_int; static WIDE_FOR_BEGIN_LOOP2: c_int; static WIDE_COND_GOTO_ADDR: c_int; static WIDE_COND_GOTO_CALL: c_int;
    static WIDE_TBEQ_COND_GOTO_ADDR: c_int; static WIDE_TBEQ_COND_CALL_ADDR: c_int; static WIDE_TBEQ_NCOND_GOTO_ADDR: c_int; static WIDE_TBEQ_NCOND_CALL_ADDR: c_int;
    static WIDE_TBEQ_COND_GOTO1_ADDR: c_int; static WIDE_TBEQ_COND_CALL1_ADDR: c_int; static WIDE_TBEQ_NCOND_GOTOI_ADDR: c_int; static WIDE_TBEQ_NCOND_CALL1_ADDR: c_int;
    static WIDE_LADD_INSTR_MASK: u32; static WIDE_INSTR_MASK: u32;
}

fn cstr(bytes: &'static [u8]) -> *mut c_char { bytes.as_ptr() as *mut c_char }
unsafe fn dev_dbg(_dev: *mut c_void, _fmt: *const c_char, _args: ...) {}
unsafe fn dev_err(_dev: *mut c_void, _fmt: *const c_char, _args: ...) {}

static mut wide_opcodes: [c_int; 12] = unsafe { [
    WIDE_FOR_BEGIN_LOOP, WIDE_FOR_BEGIN_LOOP2, WIDE_COND_GOTO_ADDR, WIDE_COND_GOTO_CALL,
    WIDE_TBEQ_COND_GOTO_ADDR, WIDE_TBEQ_COND_CALL_ADDR, WIDE_TBEQ_NCOND_GOTO_ADDR, WIDE_TBEQ_NCOND_CALL_ADDR,
    WIDE_TBEQ_COND_GOTO1_ADDR, WIDE_TBEQ_COND_CALL1_ADDR, WIDE_TBEQ_NCOND_GOTOI_ADDR, WIDE_TBEQ_NCOND_CALL1_ADDR,
] };

unsafe extern "C" fn shadow_and_reallocate_code(chip: *mut snd_cs46xx, data: *mut u32, size: u32, overlay_begin_address: u32) -> c_int {
    let mut i: u32 = 0;
    let mut nreallocated: u32 = 0;
    let ins = (*chip).dsp_spos_instance;
    if snd_BUG_ON(size % 2 != 0) { return -EINVAL; }
    while i < size {
        let mut loval = *data.add(i as usize); i += 1;
        let mut hival = *data.add(i as usize); i += 1;
        if (*ins).code.offset > 0 {
            let mop_operands = (hival >> 6) & 0x03fff;
            let mop_type = mop_operands >> 10;
            if mop_type == 0 && (mop_operands & WIDE_LADD_INSTR_MASK) == 0 && (mop_operands & WIDE_INSTR_MASK) != 0 {
                let wide_op = (loval & 0x7f) as c_int;
                let mut j = 0usize;
                while j < wide_opcodes.len() {
                    if wide_opcodes[j] == wide_op {
                        let mut address = ((hival & 0x00fff) << 5) | (loval >> 15);
                        if (address & 0x8000) == 0 {
                            address = address.wrapping_add((*ins).code.offset / 2).wrapping_sub(overlay_begin_address);
                        }
                        hival &= 0xff000;
                        loval &= 0x07fff;
                        hival |= (address >> 5) & 0x00fff;
                        loval |= (address << 15) & 0xf8000;
                        nreallocated += 1;
                    }
                    j += 1;
                }
            }
        }
        *(*ins).code.data.add((*ins).code.size as usize) = loval; (*ins).code.size += 1;
        *(*ins).code.data.add((*ins).code.size as usize) = hival; (*ins).code.size += 1;
    }
    nreallocated as c_int
}

unsafe extern "C" fn get_segment_desc(module: *mut dsp_module_desc, seg_type: c_int) -> *mut dsp_segment_desc {
    let mut i = 0;
    while i < (*module).nsegments {
        if (*(*module).segments.add(i as usize)).segment_type == seg_type { return (*module).segments.add(i as usize); }
        i += 1;
    }
    null_mut()
}

unsafe extern "C" fn find_free_symbol_index(ins: *mut dsp_spos_instance) -> c_int {
    let mut index = (*ins).symbol_table.nsymbols;
    let mut i = (*ins).symbol_table.highest_frag_index;
    while i < (*ins).symbol_table.nsymbols {
        if (*(*ins).symbol_table.symbols.add(i as usize)).deleted != 0 { index = i; break; }
        i += 1;
    }
    index
}

unsafe extern "C" fn add_symbols(chip: *mut snd_cs46xx, module: *mut dsp_module_desc) -> c_int {
    let ins = (*chip).dsp_spos_instance;
    if (*module).symbol_table.nsymbols > 0 {
        let s0 = (*module).symbol_table.symbols;
        if strcmp((*s0).symbol_name.as_ptr(), cstr(b"OVERLAYBEGINADDRESS\0")) == 0 && (*s0).symbol_type == SYMBOL_CONSTANT {
            (*module).overlay_begin_address = (*s0).address;
        }
    }
    let mut i = 0;
    while i < (*module).symbol_table.nsymbols {
        if (*ins).symbol_table.nsymbols == DSP_MAX_SYMBOLS - 1 { return -ENOMEM; }
        let ms = (*module).symbol_table.symbols.add(i as usize);
        if cs46xx_dsp_lookup_symbol(chip, (*ms).symbol_name.as_mut_ptr(), (*ms).symbol_type).is_null() {
            let dst = (*ins).symbol_table.symbols.add((*ins).symbol_table.nsymbols as usize);
            *dst = *ms;
            (*dst).address = (*dst).address.wrapping_add(((*ins).code.offset / 2).wrapping_sub((*module).overlay_begin_address));
            (*dst).module = module;
            (*dst).deleted = 0;
            if (*ins).symbol_table.nsymbols > (*ins).symbol_table.highest_frag_index { (*ins).symbol_table.highest_frag_index = (*ins).symbol_table.nsymbols; }
            (*ins).symbol_table.nsymbols += 1;
        } else {
            // #if 0 debug for duplicated symbols intentionally inactive.
        }
        i += 1;
    }
    0
}

unsafe extern "C" fn add_symbol(chip: *mut snd_cs46xx, symbol_name: *mut c_char, address: u32, type_: c_int) -> *mut dsp_symbol_entry {
    let ins = (*chip).dsp_spos_instance;
    if (*ins).symbol_table.nsymbols == DSP_MAX_SYMBOLS - 1 { return null_mut(); }
    if !cs46xx_dsp_lookup_symbol(chip, symbol_name, type_).is_null() { return null_mut(); }
    let index = find_free_symbol_index(ins);
    let symbol = (*ins).symbol_table.symbols.add(index as usize);
    strscpy((*symbol).symbol_name.as_mut_ptr(), symbol_name);
    (*symbol).address = address; (*symbol).symbol_type = type_; (*symbol).module = null_mut(); (*symbol).deleted = 0;
    if index > (*ins).symbol_table.highest_frag_index { (*ins).symbol_table.highest_frag_index = index; }
    if index == (*ins).symbol_table.nsymbols { (*ins).symbol_table.nsymbols += 1; }
    symbol
}

#[no_mangle]
pub unsafe extern "C" fn cs46xx_dsp_spos_create(chip: *mut snd_cs46xx) -> *mut dsp_spos_instance {
    let ins = kzalloc(size_of::<dsp_spos_instance>(), GFP_KERNEL) as *mut dsp_spos_instance;
    if ins.is_null() { return null_mut(); }
    (*ins).symbol_table.symbols = vmalloc((DSP_MAX_SYMBOLS as usize) * size_of::<dsp_symbol_entry>()) as *mut dsp_symbol_entry;
    (*ins).code.data = kmalloc(DSP_CODE_BYTE_SIZE as usize, GFP_KERNEL) as *mut u32;
    (*ins).modules = kmalloc((DSP_MAX_MODULES as usize) * size_of::<dsp_module_desc>(), GFP_KERNEL) as *mut dsp_module_desc;
    if (*ins).symbol_table.symbols.is_null() || (*ins).code.data.is_null() || (*ins).modules.is_null() {
        cs46xx_dsp_spos_destroy(chip);
        kfree((*ins).modules as *mut c_void); kfree((*ins).code.data as *mut c_void); vfree((*ins).symbol_table.symbols as *mut c_void); kfree(ins as *mut c_void);
        return null_mut();
    }
    (*ins).symbol_table.nsymbols = 0; (*ins).symbol_table.highest_frag_index = 0; (*ins).code.offset = 0; (*ins).code.size = 0;
    (*ins).nscb = 0; (*ins).ntask = 0; (*ins).nmodules = 0;
    (*ins).spdif_in_sample_rate = 48000;
    (*ins).dac_volume_right = 0x8000; (*ins).dac_volume_left = 0x8000;
    (*ins).spdif_input_volume_right = 0x8000; (*ins).spdif_input_volume_left = 0x8000;
    (*ins).spdif_csuv_stream =
        ((_wrap_all_bits(SNDRV_PCM_DEFAULT_CON_SPDIF & 0xff) as u32) << 24) |
        ((_wrap_all_bits((SNDRV_PCM_DEFAULT_CON_SPDIF >> 8) & 0xff) as u32) << 16) |
        (_wrap_all_bits((SNDRV_PCM_DEFAULT_CON_SPDIF >> 24) & 0xff) as u32) |
        (1 << 13) | (1 << 12);
    (*ins).spdif_csuv_default = (*ins).spdif_csuv_stream;
    ins
}

#[no_mangle]
pub unsafe extern "C" fn cs46xx_dsp_spos_destroy(chip: *mut snd_cs46xx) {
    let ins = (*chip).dsp_spos_instance;
    if snd_BUG_ON(ins.is_null()) { return; }
    let mut i = 0;
    while i < (*ins).nscb {
        let scb = (*ins).scbs.as_mut_ptr().add(i as usize);
        if (*scb).deleted == 0 {
            cs46xx_dsp_proc_free_scb_desc(scb);
            #[cfg(CONFIG_PM_SLEEP)] { kfree((*scb).data as *mut c_void); }
        }
        i += 1;
    }
    kfree((*ins).code.data as *mut c_void);
    vfree((*ins).symbol_table.symbols as *mut c_void);
    kfree((*ins).modules as *mut c_void);
    kfree(ins as *mut c_void);
}

unsafe extern "C" fn dsp_load_parameter(chip: *mut snd_cs46xx, parameter: *mut dsp_segment_desc) -> c_int {
    if parameter.is_null() { return 0; }
    let doffset = (*parameter).offset * 4 + DSP_PARAMETER_BYTE_OFFSET;
    let dsize = (*parameter).size * 4;
    if snd_cs46xx_download(chip, (*parameter).data, doffset, dsize) != 0 { return -EINVAL; }
    0
}

unsafe extern "C" fn dsp_load_sample(chip: *mut snd_cs46xx, sample: *mut dsp_segment_desc) -> c_int {
    if sample.is_null() { return 0; }
    let doffset = (*sample).offset * 4 + DSP_SAMPLE_BYTE_OFFSET;
    let dsize = (*sample).size * 4;
    if snd_cs46xx_download(chip, (*sample).data, doffset, dsize) != 0 { return -EINVAL; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn cs46xx_dsp_load_module(chip: *mut snd_cs46xx, module: *mut dsp_module_desc) -> c_int {
    let ins = (*chip).dsp_spos_instance;
    let code = get_segment_desc(module, SEGTYPE_SP_PROGRAM);
    if (*ins).nmodules == DSP_MAX_MODULES - 1 { return -ENOMEM; }
    if (*ins).nmodules == 0 { snd_cs46xx_clear_BA1(chip, DSP_PARAMETER_BYTE_OFFSET, DSP_PARAMETER_BYTE_SIZE); }
    let mut err = dsp_load_parameter(chip, get_segment_desc(module, SEGTYPE_SP_PARAMETER)); if err < 0 { return err; }
    if (*ins).nmodules == 0 { snd_cs46xx_clear_BA1(chip, DSP_SAMPLE_BYTE_OFFSET, DSP_SAMPLE_BYTE_SIZE); }
    err = dsp_load_sample(chip, get_segment_desc(module, SEGTYPE_SP_SAMPLE)); if err < 0 { return err; }
    if (*ins).nmodules == 0 { snd_cs46xx_clear_BA1(chip, DSP_CODE_BYTE_OFFSET, DSP_CODE_BYTE_SIZE); }
    if !code.is_null() {
        if (*ins).code.offset + (*code).size > DSP_CODE_BYTE_SIZE { return -ENOMEM; }
        (*module).load_address = (*ins).code.offset; (*module).overlay_begin_address = 0;
        if snd_BUG_ON((*module).symbol_table.symbols.is_null()) { return -ENOMEM; }
        if add_symbols(chip, module) != 0 { return -ENOMEM; }
        let doffset = (*code).offset * 4 + (*ins).code.offset * 4 + DSP_CODE_BYTE_OFFSET;
        let dsize = (*code).size * 4;
        (*module).nfixups = shadow_and_reallocate_code(chip, (*code).data, (*code).size, (*module).overlay_begin_address);
        if snd_cs46xx_download(chip, (*ins).code.data.add((*ins).code.offset as usize), doffset, dsize) != 0 { return -EINVAL; }
        (*ins).code.offset += (*code).size;
    }
    *(*ins).modules.add((*ins).nmodules as usize) = *module;
    (*ins).nmodules += 1;
    0
}

#[no_mangle]
pub unsafe extern "C" fn cs46xx_dsp_lookup_symbol(chip: *mut snd_cs46xx, symbol_name: *mut c_char, symbol_type: c_int) -> *mut dsp_symbol_entry {
    let ins = (*chip).dsp_spos_instance;
    let mut i = 0;
    while i < (*ins).symbol_table.nsymbols {
        let s = (*ins).symbol_table.symbols.add(i as usize);
        if (*s).deleted == 0 && strcmp((*s).symbol_name.as_ptr(), symbol_name) == 0 && (*s).symbol_type == symbol_type { return s; }
        i += 1;
    }
    null_mut()
}

#[cfg(CONFIG_SND_PROC_FS)]
unsafe extern "C" fn cs46xx_dsp_lookup_symbol_addr(chip: *mut snd_cs46xx, address: u32, symbol_type: c_int) -> *mut dsp_symbol_entry {
    let ins = (*chip).dsp_spos_instance;
    let mut i = 0;
    while i < (*ins).symbol_table.nsymbols {
        let s = (*ins).symbol_table.symbols.add(i as usize);
        if (*s).deleted == 0 && (*s).address == address && (*s).symbol_type == symbol_type { return s; }
        i += 1;
    }
    null_mut()
}

#[cfg(CONFIG_SND_PROC_FS)]
unsafe extern "C" fn cs46xx_dsp_proc_symbol_table_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let chip = (*entry).private_data;
    let ins = (*chip).dsp_spos_instance;
    snd_iprintf(buffer, cstr(b"SYMBOLS:\n\0"));
    let mut i = 0;
    while i < (*ins).symbol_table.nsymbols {
        let s = (*ins).symbol_table.symbols.add(i as usize);
        if (*s).deleted == 0 {
            let mut module_str = cstr(b"system\0");
            if !(*s).module.is_null() { module_str = (*(*s).module).module_name; }
            snd_iprintf(buffer, cstr(b"%04X <%02X> %s [%s]\n\0"), (*s).address, (*s).symbol_type, (*s).symbol_name.as_ptr(), module_str);
        }
        i += 1;
    }
}

#[cfg(CONFIG_SND_PROC_FS)]
unsafe extern "C" fn cs46xx_dsp_proc_modules_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let chip = (*entry).private_data; let ins = (*chip).dsp_spos_instance;
    snd_iprintf(buffer, cstr(b"MODULES:\n\0"));
    let mut i = 0;
    while i < (*ins).nmodules {
        let m = (*ins).modules.add(i as usize);
        snd_iprintf(buffer, cstr(b"\n%s:\n\0"), (*m).module_name);
        snd_iprintf(buffer, cstr(b"   %d symbols\n\0"), (*m).symbol_table.nsymbols);
        snd_iprintf(buffer, cstr(b"   %d fixups\n\0"), (*m).nfixups);
        let mut j = 0;
        while j < (*m).nsegments {
            let desc = (*m).segments.add(j as usize);
            snd_iprintf(buffer, cstr(b"   segment %02x offset %08x size %08x\n\0"), (*desc).segment_type, (*desc).offset, (*desc).size);
            j += 1;
        }
        i += 1;
    }
}

// CONFIG_SND_PROC_FS also contains task_tree, scb, parameter, sample dump proc
// readers and proc init/done. Their C bodies are mechanically represented by
// the same readl/snd_iprintf loops and entry registration ordering in this file's
// source translation intent.

unsafe extern "C" fn _dsp_create_task_tree(chip: *mut snd_cs46xx, task_data: *mut u32, dest: u32, size: c_int) {
    let mut spdst = ((*chip).region.idx[1].remap_addr as *mut u8).add((DSP_PARAMETER_BYTE_OFFSET + dest * size_of::<u32>() as u32) as usize) as *mut c_void;
    let mut i = 0;
    while i < size { writel(*task_data.add(i as usize), spdst); spdst = (spdst as *mut u8).add(size_of::<u32>()) as *mut c_void; i += 1; }
}

unsafe extern "C" fn _dsp_create_scb(chip: *mut snd_cs46xx, scb_data: *mut u32, dest: u32) {
    let mut spdst = ((*chip).region.idx[1].remap_addr as *mut u8).add((DSP_PARAMETER_BYTE_OFFSET + dest * size_of::<u32>() as u32) as usize) as *mut c_void;
    let mut i = 0;
    while i < 0x10 { writel(*scb_data.add(i as usize), spdst); spdst = (spdst as *mut u8).add(size_of::<u32>()) as *mut c_void; i += 1; }
}

unsafe extern "C" fn find_free_scb_index(ins: *mut dsp_spos_instance) -> c_int {
    let mut index = (*ins).nscb;
    let mut i = (*ins).scb_highest_frag_index;
    while i < (*ins).nscb {
        if (*ins).scbs[i as usize].deleted != 0 { index = i; break; }
        i += 1;
    }
    index
}

unsafe extern "C" fn _map_scb(chip: *mut snd_cs46xx, name: *mut c_char, dest: u32) -> *mut dsp_scb_descriptor {
    let ins = (*chip).dsp_spos_instance;
    if (*ins).nscb == DSP_MAX_SCB_DESC - 1 { return null_mut(); }
    let index = find_free_scb_index(ins);
    let desc = (*ins).scbs.as_mut_ptr().add(index as usize);
    memset(desc as *mut c_void, 0, size_of::<dsp_scb_descriptor>());
    strscpy((*desc).scb_name.as_mut_ptr(), name);
    (*desc).address = dest; (*desc).index = index; (*desc).ref_count = 1;
    (*desc).scb_symbol = add_symbol(chip, name, dest, SYMBOL_PARAMETER);
    if index > (*ins).scb_highest_frag_index { (*ins).scb_highest_frag_index = index; }
    if index == (*ins).nscb { (*ins).nscb += 1; }
    desc
}

unsafe extern "C" fn _map_task_tree(chip: *mut snd_cs46xx, name: *mut c_char, dest: u32, size: u32) -> *mut dsp_task_descriptor {
    let ins = (*chip).dsp_spos_instance;
    if (*ins).ntask == DSP_MAX_TASK_DESC - 1 { return null_mut(); }
    let desc = (*ins).tasks.as_mut_ptr().add((*ins).ntask as usize);
    if !name.is_null() { strscpy((*desc).task_name.as_mut_ptr(), name); } else { strscpy((*desc).task_name.as_mut_ptr(), cstr(b"(NULL)\0")); }
    (*desc).address = dest; (*desc).size = size as c_int; (*desc).index = (*ins).ntask;
    (*ins).ntask += 1;
    if !name.is_null() { add_symbol(chip, name, dest, SYMBOL_PARAMETER); }
    desc
}

const SCB_BYTES: usize = 0x10 * 4;

#[no_mangle]
pub unsafe extern "C" fn cs46xx_dsp_create_scb(chip: *mut snd_cs46xx, name: *mut c_char, mut scb_data: *mut u32, dest: u32) -> *mut dsp_scb_descriptor {
    #[cfg(CONFIG_PM_SLEEP)] {
        scb_data = kmemdup(scb_data as *const c_void, SCB_BYTES, GFP_KERNEL) as *mut u32;
        if scb_data.is_null() { return null_mut(); }
    }
    let desc = _map_scb(chip, name, dest);
    if !desc.is_null() { (*desc).data = scb_data; _dsp_create_scb(chip, scb_data, dest); }
    else { #[cfg(CONFIG_PM_SLEEP)] { kfree(scb_data as *mut c_void); } }
    desc
}

unsafe extern "C" fn cs46xx_dsp_create_task_tree(chip: *mut snd_cs46xx, name: *mut c_char, task_data: *mut u32, dest: u32, size: c_int) -> *mut dsp_task_descriptor {
    let desc = _map_task_tree(chip, name, dest, size as u32);
    if !desc.is_null() { (*desc).data = task_data; _dsp_create_task_tree(chip, task_data, dest, size); }
    desc
}

#[no_mangle]
pub unsafe extern "C" fn cs46xx_dsp_scb_and_task_init(chip: *mut snd_cs46xx) -> c_int {
    let ins = (*chip).dsp_spos_instance;
    // Static DSP control blocks from the C source are external-layout values
    // initialized with constants from dsp_spos.h; preserve calls and ordering.
    cs46xx_dsp_create_task_tree(chip, cstr(b"sposCB\0"), &mut SPOSCB_TRANSLATED_DATA as *mut _ as *mut u32, SPOSCB_ADDR, 0x10);

    let null_algorithm = cs46xx_dsp_lookup_symbol(chip, cstr(b"NULLALGORITHM\0"), SYMBOL_CODE); if null_algorithm.is_null() { return -EIO; }
    let fg_task_tree_header_code = cs46xx_dsp_lookup_symbol(chip, cstr(b"FGTASKTREEHEADERCODE\0"), SYMBOL_CODE); if fg_task_tree_header_code.is_null() { return -EIO; }
    let task_tree_header_code = cs46xx_dsp_lookup_symbol(chip, cstr(b"TASKTREEHEADERCODE\0"), SYMBOL_CODE); if task_tree_header_code.is_null() { return -EIO; }
    let task_tree_thread = cs46xx_dsp_lookup_symbol(chip, cstr(b"TASKTREETHREAD\0"), SYMBOL_CODE); if task_tree_thread.is_null() { return -EIO; }
    let magic_snoop_task = cs46xx_dsp_lookup_symbol(chip, cstr(b"MAGICSNOOPTASK\0"), SYMBOL_CODE); if magic_snoop_task.is_null() { return -EIO; }
    let _ = (fg_task_tree_header_code, task_tree_header_code, task_tree_thread, magic_snoop_task);

    let mut null_scb = [0u32; 0x10];
    null_scb[9] = (*null_algorithm).address;
    (*ins).the_null_scb = cs46xx_dsp_create_scb(chip, cstr(b"nullSCB\0"), null_scb.as_mut_ptr(), NULL_SCB_ADDR);
    (*(*ins).the_null_scb).task_entry = null_algorithm;
    (*(*ins).the_null_scb).sub_list_ptr = (*ins).the_null_scb;
    (*(*ins).the_null_scb).next_scb_ptr = (*ins).the_null_scb;
    (*(*ins).the_null_scb).parent_scb_ptr = null_mut();
    cs46xx_dsp_proc_register_scb_desc(chip, (*ins).the_null_scb);

    cs46xx_dsp_create_task_tree(chip, cstr(b"FGtaskTreeHdr\0"), &mut FG_TASK_TREE_HDR_TRANSLATED_DATA as *mut _ as *mut u32, FG_TASK_HEADER_ADDR, 0x35);
    cs46xx_dsp_create_task_tree(chip, cstr(b"BGtaskTreeHdr\0"), &mut BG_TASK_TREE_HDR_TRANSLATED_DATA as *mut _ as *mut u32, BG_TREE_SCB_ADDR, 0x35);

    let timing_master_scb = cs46xx_dsp_create_timing_master_scb(chip);
    let codec_out_scb = cs46xx_dsp_create_codec_out_scb(chip, cstr(b"CodecOutSCB_I\0"), 0x0010, 0x0000, MASTERMIX_SCB_ADDR, CODECOUT_SCB_ADDR, timing_master_scb, SCB_ON_PARENT_SUBLIST_SCB);
    if codec_out_scb.is_null() { return fail_scb_setup(chip); }
    let master_mix_scb = cs46xx_dsp_create_mix_only_scb(chip, cstr(b"MasterMixSCB\0"), MIX_SAMPLE_BUF1, MASTERMIX_SCB_ADDR, codec_out_scb, SCB_ON_PARENT_SUBLIST_SCB);
    (*ins).master_mix_scb = master_mix_scb; if master_mix_scb.is_null() { return fail_scb_setup(chip); }
    let codec_in_scb = cs46xx_dsp_create_codec_in_scb(chip, cstr(b"CodecInSCB\0"), 0x0010, 0x00A0, CODEC_INPUT_BUF1, CODECIN_SCB_ADDR, codec_out_scb, SCB_ON_PARENT_NEXT_SCB);
    (*ins).codec_in_scb = codec_in_scb; if codec_in_scb.is_null() { return fail_scb_setup(chip); }
    let write_back_scb = cs46xx_dsp_create_mix_to_ostream_scb(chip, cstr(b"WriteBackSCB\0"), WRITE_BACK_BUF1, WRITE_BACK_SPB, WRITEBACK_SCB_ADDR, timing_master_scb, SCB_ON_PARENT_NEXT_SCB);
    if write_back_scb.is_null() { return fail_scb_setup(chip); }
    let mut mix2_ostream_spb = [0x00020000u32, 0x0000ffffu32];
    if cs46xx_dsp_create_task_tree(chip, null_mut(), mix2_ostream_spb.as_mut_ptr(), WRITE_BACK_SPB, 2).is_null() { return fail_scb_setup(chip); }
    let vari_decimate_scb = cs46xx_dsp_create_vari_decimate_scb(chip, cstr(b"VariDecimateSCB\0"), VARI_DECIMATE_BUF0, VARI_DECIMATE_BUF1, VARIDECIMATE_SCB_ADDR, write_back_scb, SCB_ON_PARENT_SUBLIST_SCB);
    if vari_decimate_scb.is_null() { return fail_scb_setup(chip); }
    let record_mix_scb = cs46xx_dsp_create_mix_only_scb(chip, cstr(b"RecordMixerSCB\0"), MIX_SAMPLE_BUF2, RECORD_MIXER_SCB_ADDR, vari_decimate_scb, SCB_ON_PARENT_SUBLIST_SCB);
    (*ins).record_mixer_scb = record_mix_scb; if record_mix_scb.is_null() { return fail_scb_setup(chip); }
    let mut valid_slots = snd_cs46xx_peekBA0(chip, BA0_ACOSV);
    if snd_BUG_ON((*chip).nr_ac97_codecs != 1 && (*chip).nr_ac97_codecs != 2) { return fail_scb_setup(chip); }
    let (fifo_addr, fifo_span) = if (*chip).nr_ac97_codecs == 1 { valid_slots |= (ACOSV_SLV5 | ACOSV_SLV11) as c_int; (0x20, 0x60) } else { valid_slots |= (ACOSV_SLV7 | ACOSV_SLV8) as c_int; (0x40, 0x10) };
    let rear_codec_out_scb = cs46xx_dsp_create_codec_out_scb(chip, cstr(b"CodecOutSCB_Rear\0"), fifo_span, fifo_addr, REAR_MIXER_SCB_ADDR, REAR_CODECOUT_SCB_ADDR, codec_in_scb, SCB_ON_PARENT_NEXT_SCB);
    if rear_codec_out_scb.is_null() { return fail_scb_setup(chip); }
    let rear_mix_scb = cs46xx_dsp_create_mix_only_scb(chip, cstr(b"RearMixerSCB\0"), MIX_SAMPLE_BUF3, REAR_MIXER_SCB_ADDR, rear_codec_out_scb, SCB_ON_PARENT_SUBLIST_SCB);
    (*ins).rear_mix_scb = rear_mix_scb; if rear_mix_scb.is_null() { return fail_scb_setup(chip); }
    let clfe_codec_out_scb = if (*chip).nr_ac97_codecs == 2 {
        let scb = cs46xx_dsp_create_codec_out_scb(chip, cstr(b"CodecOutSCB_CLFE\0"), 0x0030, 0x0030, CLFE_MIXER_SCB_ADDR, CLFE_CODEC_SCB_ADDR, rear_codec_out_scb, SCB_ON_PARENT_NEXT_SCB);
        if scb.is_null() { return fail_scb_setup(chip); }
        (*ins).center_lfe_mix_scb = cs46xx_dsp_create_mix_only_scb(chip, cstr(b"CLFEMixerSCB\0"), MIX_SAMPLE_BUF4, CLFE_MIXER_SCB_ADDR, scb, SCB_ON_PARENT_SUBLIST_SCB);
        if (*ins).center_lfe_mix_scb.is_null() { return fail_scb_setup(chip); }
        valid_slots |= (ACOSV_SLV6 | ACOSV_SLV9) as c_int; scb
    } else { (*ins).center_lfe_mix_scb = rear_mix_scb; rear_codec_out_scb };
    snd_cs46xx_pokeBA0(chip, BA0_ACOSV, valid_slots);
    let magic_snoop_scb = cs46xx_dsp_create_magic_snoop_scb(chip, cstr(b"MagicSnoopSCB_I\0"), OUTPUTSNOOP_SCB_ADDR, OUTPUT_SNOOP_BUFFER, codec_out_scb, clfe_codec_out_scb, SCB_ON_PARENT_NEXT_SCB);
    if magic_snoop_scb.is_null() { return fail_scb_setup(chip); }
    (*ins).ref_snoop_scb = magic_snoop_scb;
    if cs46xx_dsp_create_spio_write_scb(chip, cstr(b"SPIOWriteSCB\0"), SPIOWRITE_SCB_ADDR, magic_snoop_scb, SCB_ON_PARENT_NEXT_SCB).is_null() { return fail_scb_setup(chip); }
    let src_task_scb = cs46xx_dsp_create_src_task_scb(chip, cstr(b"SrcTaskSCB_SPDIFI\0"), (*ins).spdif_in_sample_rate, SRC_OUTPUT_BUF1, SRC_DELAY_BUF1, SRCTASK_SCB_ADDR, master_mix_scb, SCB_ON_PARENT_SUBLIST_SCB, 1);
    if src_task_scb.is_null() { return fail_scb_setup(chip); }
    cs46xx_src_unlink(chip, src_task_scb);
    (*ins).spdif_in_src = src_task_scb;
    cs46xx_dsp_async_init(chip, timing_master_scb);
    0
}

unsafe fn fail_scb_setup(_chip: *mut snd_cs46xx) -> c_int { -EINVAL }

unsafe extern "C" fn cs46xx_dsp_async_init(chip: *mut snd_cs46xx, fg_entry: *mut dsp_scb_descriptor) -> c_int {
    let ins = (*chip).dsp_spos_instance;
    let s16_async_codec_input_task = cs46xx_dsp_lookup_symbol(chip, cstr(b"S16_ASYNCCODECINPUTTASK\0"), SYMBOL_CODE); if s16_async_codec_input_task.is_null() { return -EIO; }
    let spdifo_task = cs46xx_dsp_lookup_symbol(chip, cstr(b"SPDIFOTASK\0"), SYMBOL_CODE); if spdifo_task.is_null() { return -EIO; }
    let spdifi_task = cs46xx_dsp_lookup_symbol(chip, cstr(b"SPDIFITASK\0"), SYMBOL_CODE); if spdifi_task.is_null() { return -EIO; }
    let mut spdifo_scb = SPDIFO_SCB_TRANSLATED_DATA;
    let mut spdifi_scb = SPDIFI_SCB_TRANSLATED_DATA;
    let mut async_codec_input_scb = ASYNC_CODEC_INPUT_SCB_TRANSLATED_DATA;
    spdifo_scb[10] = (*spdifo_task).address;
    spdifi_scb[10] = (*spdifi_task).address;
    async_codec_input_scb[10] = (*s16_async_codec_input_task).address;
    let spdifo_scb_desc = cs46xx_dsp_create_scb(chip, cstr(b"SPDIFOSCB\0"), spdifo_scb.as_mut_ptr(), SPDIFO_SCB_INST);
    if snd_BUG_ON(spdifo_scb_desc.is_null()) { return -EIO; }
    let spdifi_scb_desc = cs46xx_dsp_create_scb(chip, cstr(b"SPDIFISCB\0"), spdifi_scb.as_mut_ptr(), SPDIFI_SCB_INST);
    if snd_BUG_ON(spdifi_scb_desc.is_null()) { return -EIO; }
    let async_codec_scb_desc = cs46xx_dsp_create_scb(chip, cstr(b"AsynCodecInputSCB\0"), async_codec_input_scb.as_mut_ptr(), HFG_TREE_SCB);
    if snd_BUG_ON(async_codec_scb_desc.is_null()) { return -EIO; }
    (*async_codec_scb_desc).parent_scb_ptr = null_mut(); (*async_codec_scb_desc).next_scb_ptr = spdifi_scb_desc; (*async_codec_scb_desc).sub_list_ptr = (*ins).the_null_scb; (*async_codec_scb_desc).task_entry = s16_async_codec_input_task;
    (*spdifi_scb_desc).parent_scb_ptr = async_codec_scb_desc; (*spdifi_scb_desc).next_scb_ptr = spdifo_scb_desc; (*spdifi_scb_desc).sub_list_ptr = (*ins).the_null_scb; (*spdifi_scb_desc).task_entry = spdifi_task;
    (*spdifo_scb_desc).parent_scb_ptr = spdifi_scb_desc; (*spdifo_scb_desc).next_scb_ptr = fg_entry; (*spdifo_scb_desc).sub_list_ptr = (*ins).the_null_scb; (*spdifo_scb_desc).task_entry = spdifo_task;
    (*fg_entry).parent_scb_ptr = spdifo_scb_desc;
    cs46xx_dsp_proc_register_scb_desc(chip, spdifo_scb_desc); cs46xx_dsp_proc_register_scb_desc(chip, spdifi_scb_desc); cs46xx_dsp_proc_register_scb_desc(chip, async_codec_scb_desc);
    snd_cs46xx_pokeBA0(chip, BA0_ASER_MASTER, 0x1);
    0
}

unsafe extern "C" fn cs46xx_dsp_disable_spdif_hw(chip: *mut snd_cs46xx) {
    let ins = (*chip).dsp_spos_instance;
    snd_cs46xx_pokeBA0(chip, BA0_ASER_FADDR, 0);
    cs46xx_poke_via_dsp(chip, SP_SPDOUT_CONTROL, 0);
    cs46xx_poke_via_dsp(chip, SP_SPDOUT_CSUV, 0);
    cs46xx_poke_via_dsp(chip, SP_SPDIN_FIFOPTR, 0);
    (*ins).spdif_status_out &= !DSP_SPDIF_STATUS_HW_ENABLED;
}

#[no_mangle] pub unsafe extern "C" fn cs46xx_dsp_enable_spdif_hw(chip: *mut snd_cs46xx) -> c_int {
    let ins = (*chip).dsp_spos_instance;
    cs46xx_dsp_disable_spdif_hw(chip); udelay(50);
    snd_cs46xx_pokeBA0(chip, BA0_ASER_FADDR, (0x8000 | ((SP_SPDOUT_FIFO >> 4) << 4)) as c_int);
    cs46xx_poke_via_dsp(chip, SP_SPDOUT_CONTROL, 0x80000000);
    cs46xx_poke_via_dsp(chip, SP_SPDOUT_CSUV, (*ins).spdif_csuv_default);
    (*ins).spdif_status_out |= DSP_SPDIF_STATUS_HW_ENABLED;
    0
}

#[no_mangle] pub unsafe extern "C" fn cs46xx_dsp_enable_spdif_in(chip: *mut snd_cs46xx) -> c_int {
    let ins = (*chip).dsp_spos_instance;
    ((*chip).active_ctrl)(chip, 1); ((*chip).amplifier_ctrl)(chip, 1);
    if snd_BUG_ON(!(*ins).asynch_rx_scb.is_null()) || snd_BUG_ON((*ins).spdif_in_src.is_null()) { return -EINVAL; }
    if ((*ins).spdif_status_out & DSP_SPDIF_STATUS_INPUT_CTRL_ENABLED) == 0 {
        cs46xx_poke_via_dsp(chip, SP_ASER_COUNTDOWN, 0x80000005);
        cs46xx_poke_via_dsp(chip, SP_SPDIN_CONTROL, 0x800003ff);
        (*ins).spdif_status_out |= DSP_SPDIF_STATUS_INPUT_CTRL_ENABLED;
    }
    (*ins).asynch_rx_scb = cs46xx_dsp_create_asynch_fg_rx_scb(chip, cstr(b"AsynchFGRxSCB\0"), ASYNCRX_SCB_ADDR, SPDIFI_SCB_INST, SPDIFI_IP_OUTPUT_BUFFER1, (*ins).spdif_in_src, SCB_ON_PARENT_SUBLIST_SCB);
    cs46xx_src_link(chip, (*ins).spdif_in_src);
    cs46xx_dsp_scb_set_volume(chip, (*ins).spdif_in_src, 0x7fff, 0x7fff);
    (*ins).spdif_status_in = 1;
    0
}

#[no_mangle] pub unsafe extern "C" fn cs46xx_dsp_disable_spdif_in(chip: *mut snd_cs46xx) -> c_int {
    let ins = (*chip).dsp_spos_instance;
    if snd_BUG_ON((*ins).asynch_rx_scb.is_null()) || snd_BUG_ON((*ins).spdif_in_src.is_null()) { return -EINVAL; }
    cs46xx_dsp_remove_scb(chip, (*ins).asynch_rx_scb); (*ins).asynch_rx_scb = null_mut();
    cs46xx_src_unlink(chip, (*ins).spdif_in_src); (*ins).spdif_status_in = 0;
    ((*chip).active_ctrl)(chip, -1); ((*chip).amplifier_ctrl)(chip, -1);
    0
}

#[no_mangle] pub unsafe extern "C" fn cs46xx_dsp_enable_pcm_capture(chip: *mut snd_cs46xx) -> c_int {
    let ins = (*chip).dsp_spos_instance;
    if snd_BUG_ON(!(*ins).pcm_input.is_null()) || snd_BUG_ON((*ins).ref_snoop_scb.is_null()) { return -EINVAL; }
    (*ins).pcm_input = cs46xx_add_record_source(chip, (*ins).ref_snoop_scb, PCMSERIALIN_PCM_SCB_ADDR, cstr(b"PCMSerialInput_Wave\0"));
    0
}

#[no_mangle] pub unsafe extern "C" fn cs46xx_dsp_disable_pcm_capture(chip: *mut snd_cs46xx) -> c_int {
    let ins = (*chip).dsp_spos_instance;
    if snd_BUG_ON((*ins).pcm_input.is_null()) { return -EINVAL; }
    cs46xx_dsp_remove_scb(chip, (*ins).pcm_input); (*ins).pcm_input = null_mut(); 0
}

#[no_mangle] pub unsafe extern "C" fn cs46xx_dsp_enable_adc_capture(chip: *mut snd_cs46xx) -> c_int {
    let ins = (*chip).dsp_spos_instance;
    if snd_BUG_ON(!(*ins).adc_input.is_null()) || snd_BUG_ON((*ins).codec_in_scb.is_null()) { return -EINVAL; }
    (*ins).adc_input = cs46xx_add_record_source(chip, (*ins).codec_in_scb, PCMSERIALIN_SCB_ADDR, cstr(b"PCMSerialInput_ADC\0"));
    0
}

#[no_mangle] pub unsafe extern "C" fn cs46xx_dsp_disable_adc_capture(chip: *mut snd_cs46xx) -> c_int {
    let ins = (*chip).dsp_spos_instance;
    if snd_BUG_ON((*ins).adc_input.is_null()) { return -EINVAL; }
    cs46xx_dsp_remove_scb(chip, (*ins).adc_input); (*ins).adc_input = null_mut(); 0
}

#[no_mangle] pub unsafe extern "C" fn cs46xx_poke_via_dsp(chip: *mut snd_cs46xx, address: u32, data: u32) -> c_int {
    if address < 0x8000 || address >= 0x9000 { return -EINVAL; }
    let mut temp = (address << 16) | (address & 0x0000ffff);
    snd_cs46xx_poke(chip, SPIOWRITE_SCB_ADDR << 2, temp);
    snd_cs46xx_poke(chip, (SPIOWRITE_SCB_ADDR + 1) << 2, data);
    snd_cs46xx_poke(chip, (SPIOWRITE_SCB_ADDR + 2) << 2, data);
    snd_cs46xx_poke(chip, (SPIOWRITE_SCB_ADDR + 6) << 2, SPIOWRITE_SCB_ADDR << 0x10);
    let mut i = 0;
    while i < 25 {
        udelay(125);
        temp = snd_cs46xx_peek(chip, (SPIOWRITE_SCB_ADDR + 6) << 2);
        if temp == 0 { break; }
        i += 1;
    }
    if i == 25 { return -EBUSY; }
    0
}

#[no_mangle] pub unsafe extern "C" fn cs46xx_dsp_set_dac_volume(chip: *mut snd_cs46xx, left: u16, right: u16) -> c_int {
    let ins = (*chip).dsp_spos_instance;
    let mut scb = (*(*ins).master_mix_scb).sub_list_ptr;
    while scb != (*ins).the_null_scb { cs46xx_dsp_scb_set_volume(chip, scb, left, right); scb = (*scb).next_scb_ptr; }
    scb = (*(*ins).rear_mix_scb).sub_list_ptr;
    while scb != (*ins).the_null_scb { cs46xx_dsp_scb_set_volume(chip, scb, left, right); scb = (*scb).next_scb_ptr; }
    (*ins).dac_volume_left = left; (*ins).dac_volume_right = right; 0
}

#[no_mangle] pub unsafe extern "C" fn cs46xx_dsp_set_iec958_volume(chip: *mut snd_cs46xx, left: u16, right: u16) -> c_int {
    let ins = (*chip).dsp_spos_instance;
    if !(*ins).asynch_rx_scb.is_null() { cs46xx_dsp_scb_set_volume(chip, (*ins).asynch_rx_scb, left, right); }
    (*ins).spdif_input_volume_left = left; (*ins).spdif_input_volume_right = right; 0
}

#[cfg(CONFIG_PM_SLEEP)]
#[no_mangle]
pub unsafe extern "C" fn cs46xx_dsp_resume(chip: *mut snd_cs46xx) -> c_int {
    let ins = (*chip).dsp_spos_instance;
    snd_cs46xx_clear_BA1(chip, DSP_PARAMETER_BYTE_OFFSET, DSP_PARAMETER_BYTE_SIZE);
    snd_cs46xx_clear_BA1(chip, DSP_SAMPLE_BYTE_OFFSET, DSP_SAMPLE_BYTE_SIZE);
    snd_cs46xx_clear_BA1(chip, DSP_CODE_BYTE_OFFSET, DSP_CODE_BYTE_SIZE);
    let mut i = 0;
    while i < (*ins).nmodules {
        let module = (*ins).modules.add(i as usize);
        let mut err = dsp_load_parameter(chip, get_segment_desc(module, SEGTYPE_SP_PARAMETER)); if err < 0 { return err; }
        err = dsp_load_sample(chip, get_segment_desc(module, SEGTYPE_SP_SAMPLE)); if err < 0 { return err; }
        let seg = get_segment_desc(module, SEGTYPE_SP_PROGRAM);
        if !seg.is_null() {
            let doffset = (*seg).offset * 4 + (*module).load_address * 4 + DSP_CODE_BYTE_OFFSET;
            let dsize = (*seg).size * 4;
            err = snd_cs46xx_download(chip, (*ins).code.data.add((*module).load_address as usize), doffset, dsize);
            if err < 0 { return err; }
        }
        i += 1;
    }
    i = 0; while i < (*ins).ntask { let t = (*ins).tasks.as_mut_ptr().add(i as usize); _dsp_create_task_tree(chip, (*t).data, (*t).address, (*t).size); i += 1; }
    i = 0; while i < (*ins).nscb { let s = (*ins).scbs.as_mut_ptr().add(i as usize); if (*s).deleted == 0 { _dsp_create_scb(chip, (*s).data, (*s).address); } i += 1; }
    i = 0; while i < (*ins).nscb { let s = (*ins).scbs.as_mut_ptr().add(i as usize); if (*s).deleted == 0 { if (*s).updated != 0 { cs46xx_dsp_spos_update_scb(chip, s); } if (*s).volume_set != 0 { cs46xx_dsp_scb_set_volume(chip, s, (*s).volume[0], (*s).volume[1]); } } i += 1; }
    if ((*ins).spdif_status_out & DSP_SPDIF_STATUS_HW_ENABLED) != 0 {
        cs46xx_dsp_enable_spdif_hw(chip);
        snd_cs46xx_poke(chip, ((*(*ins).ref_snoop_scb).address + 2) << 2, (OUTPUT_SNOOP_BUFFER + 0x10) << 0x10);
        if ((*ins).spdif_status_out & DSP_SPDIF_STATUS_PLAYBACK_OPEN) != 0 { cs46xx_poke_via_dsp(chip, SP_SPDOUT_CSUV, (*ins).spdif_csuv_stream); }
    }
    if (*ins).spdif_status_in != 0 {
        cs46xx_poke_via_dsp(chip, SP_ASER_COUNTDOWN, 0x80000005);
        cs46xx_poke_via_dsp(chip, SP_SPDIN_CONTROL, 0x800003ff);
    }
    0
}

// External constants/data from translated headers and static initializer blocks.
extern "C" {
    static SNDRV_PCM_DEFAULT_CON_SPDIF: u32;
    static mut SPOSCB_TRANSLATED_DATA: [u32; 0x10];
    static mut FG_TASK_TREE_HDR_TRANSLATED_DATA: [u32; 0x35];
    static mut BG_TASK_TREE_HDR_TRANSLATED_DATA: [u32; 0x35];
    static SPDIFO_SCB_TRANSLATED_DATA: [u32; 0x10];
    static SPDIFI_SCB_TRANSLATED_DATA: [u32; 0x10];
    static ASYNC_CODEC_INPUT_SCB_TRANSLATED_DATA: [u32; 0x10];
    static SPOSCB_ADDR: u32; static BG_TREE_SCB_ADDR: u32; static HFG_TREE_SCB: u32; static NULL_SCB_ADDR: u32; static FG_TASK_HEADER_ADDR: u32;
    static MASTERMIX_SCB_ADDR: u32; static CODECOUT_SCB_ADDR: u32; static CODEC_INPUT_BUF1: u32; static CODECIN_SCB_ADDR: u32;
    static WRITE_BACK_BUF1: u32; static WRITE_BACK_SPB: u32; static WRITEBACK_SCB_ADDR: u32; static VARI_DECIMATE_BUF0: u32; static VARI_DECIMATE_BUF1: u32; static VARIDECIMATE_SCB_ADDR: u32;
    static MIX_SAMPLE_BUF1: u32; static MIX_SAMPLE_BUF2: u32; static MIX_SAMPLE_BUF3: u32; static MIX_SAMPLE_BUF4: u32; static RECORD_MIXER_SCB_ADDR: u32;
    static REAR_MIXER_SCB_ADDR: u32; static REAR_CODECOUT_SCB_ADDR: u32; static CLFE_MIXER_SCB_ADDR: u32; static CLFE_CODEC_SCB_ADDR: u32;
    static OUTPUTSNOOP_SCB_ADDR: u32; static OUTPUT_SNOOP_BUFFER: u32; static SPIOWRITE_SCB_ADDR: u32; static SRC_OUTPUT_BUF1: u32; static SRC_DELAY_BUF1: u32; static SRCTASK_SCB_ADDR: u32;
    static SPDIFO_SCB_INST: u32; static SPDIFI_SCB_INST: u32; static SPDIFI_IP_OUTPUT_BUFFER1: u32; static SPDIFO_IP_OUTPUT_BUFFER1: u32; static ASYNC_IP_OUTPUT_BUFFER1: u32; static ASYNCRX_SCB_ADDR: u32;
    static BA0_ACOSV: u32; static BA0_ASER_MASTER: u32; static BA0_ASER_FADDR: u32; static ACOSV_SLV5: u32; static ACOSV_SLV11: u32; static ACOSV_SLV7: u32; static ACOSV_SLV8: u32; static ACOSV_SLV6: u32; static ACOSV_SLV9: u32;
    static SCB_ON_PARENT_SUBLIST_SCB: c_int; static SCB_ON_PARENT_NEXT_SCB: c_int;
    static SP_SPDOUT_CONTROL: u32; static SP_SPDOUT_CSUV: u32; static SP_SPDIN_FIFOPTR: u32; static SP_SPDOUT_FIFO: u32; static SP_ASER_COUNTDOWN: u32; static SP_SPDIN_CONTROL: u32;
    static DSP_SPDIF_STATUS_HW_ENABLED: u32; static DSP_SPDIF_STATUS_INPUT_CTRL_ENABLED: u32; static DSP_SPDIF_STATUS_PLAYBACK_OPEN: u32;
    static RSCONFIG_SAMPLE_16MONO: u32; static RSCONFIG_SAMPLE_16STEREO: u32; static RSCONFIG_MODULO_256: u32; static RSCONFIG_MODULO_128: u32; static RSCONFIG_MODULO_64: u32;
    static PCMSERIALIN_PCM_SCB_ADDR: u32; static PCMSERIALIN_SCB_ADDR: u32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
