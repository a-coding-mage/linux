// SPDX-License-Identifier: GPL-2.0-or-later
/*
 */

/*
 * 2002-07 Benny Sjostrand benny@hostmobility.com
 */

use crate::*;

#[repr(C)]
pub struct proc_scb_info {
    pub scb_desc: *mut dsp_scb_descriptor,
    pub chip: *mut snd_cs46xx,
}

unsafe fn remove_symbol(chip: *mut snd_cs46xx, symbol: *mut dsp_symbol_entry) {
    unsafe {
        let ins = (*chip).dsp_spos_instance;
        let symbol_index = symbol.offset_from((*(*ins).symbol_table.symbols).as_mut_ptr()) as i32;

        if snd_BUG_ON((*ins).symbol_table.nsymbols <= 0) {
            return;
        }
        if snd_BUG_ON(
            symbol_index < 0 || symbol_index >= (*ins).symbol_table.nsymbols,
        ) {
            return;
        }

        (*(*ins).symbol_table.symbols.offset(symbol_index as isize)).deleted = 1;

        if symbol_index < (*ins).symbol_table.highest_frag_index {
            (*ins).symbol_table.highest_frag_index = symbol_index;
        }

        if symbol_index == (*ins).symbol_table.nsymbols - 1 {
            (*ins).symbol_table.nsymbols -= 1;
        }

        if (*ins).symbol_table.highest_frag_index > (*ins).symbol_table.nsymbols {
            (*ins).symbol_table.highest_frag_index = (*ins).symbol_table.nsymbols;
        }
    }
}

/* CONFIG_SND_PROC_FS: procfs SCB info reader translated from the C conditional block. */
#[cfg(CONFIG_SND_PROC_FS)]
unsafe fn cs46xx_dsp_proc_scb_info_read(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    unsafe {
        let scb_info = (*entry).private_data as *mut proc_scb_info;
        let scb = (*scb_info).scb_desc;
        let chip = (*scb_info).chip;
        let mut col: i32;
        let mut j: i32;
        let mut dst = (*chip).region.idx[1].remap_addr.add(DSP_PARAMETER_BYTE_OFFSET as usize);

        guard_mutex(&mut (*chip).spos_mutex);
        snd_iprintf(buffer, c"%04x %s:\n".as_ptr(), (*scb).address, (*scb).scb_name);

        col = 0;
        j = 0;
        while j < 0x10 {
            if col == 4 {
                snd_iprintf(buffer, c"\n".as_ptr());
                col = 0;
            }
            snd_iprintf(
                buffer,
                c"%08x ".as_ptr(),
                readl(dst.add((((*scb).address + j as u32) as usize) * core::mem::size_of::<u32>())),
            );
            j += 1;
            col += 1;
        }

        snd_iprintf(buffer, c"\n".as_ptr());

        if !(*scb).parent_scb_ptr.is_null() {
            snd_iprintf(
                buffer,
                c"parent [%s:%04x] ".as_ptr(),
                (*(*scb).parent_scb_ptr).scb_name,
                (*(*scb).parent_scb_ptr).address,
            );
        } else {
            snd_iprintf(buffer, c"parent [none] ".as_ptr());
        }

        snd_iprintf(
            buffer,
            c"sub_list_ptr [%s:%04x]\nnext_scb_ptr [%s:%04x]  task_entry [%s:%04x]\n".as_ptr(),
            (*(*scb).sub_list_ptr).scb_name,
            (*(*scb).sub_list_ptr).address,
            (*(*scb).next_scb_ptr).scb_name,
            (*(*scb).next_scb_ptr).address,
            (*(*scb).task_entry).symbol_name,
            (*(*scb).task_entry).address,
        );

        snd_iprintf(
            buffer,
            c"index [%d] ref_count [%d]\n".as_ptr(),
            (*scb).index,
            (*scb).ref_count,
        );
    }
}

unsafe fn _dsp_unlink_scb(chip: *mut snd_cs46xx, scb: *mut dsp_scb_descriptor) {
    unsafe {
        let ins = (*chip).dsp_spos_instance;

        if !(*scb).parent_scb_ptr.is_null() {
            /* unlink parent SCB */
            if snd_BUG_ON(
                (*(*scb).parent_scb_ptr).sub_list_ptr != scb
                    && (*(*scb).parent_scb_ptr).next_scb_ptr != scb,
            ) {
                return;
            }

            if (*(*scb).parent_scb_ptr).sub_list_ptr == scb {
                if (*scb).next_scb_ptr == (*ins).the_null_scb {
                    /* last and only node in parent sublist */
                    (*(*scb).parent_scb_ptr).sub_list_ptr = (*scb).sub_list_ptr;

                    if (*scb).sub_list_ptr != (*ins).the_null_scb {
                        (*(*scb).sub_list_ptr).parent_scb_ptr = (*scb).parent_scb_ptr;
                    }
                    (*scb).sub_list_ptr = (*ins).the_null_scb;
                } else {
                    /* first node in parent sublist */
                    (*(*scb).parent_scb_ptr).sub_list_ptr = (*scb).next_scb_ptr;

                    if (*scb).next_scb_ptr != (*ins).the_null_scb {
                        /* update next node parent ptr. */
                        (*(*scb).next_scb_ptr).parent_scb_ptr = (*scb).parent_scb_ptr;
                    }
                    (*scb).next_scb_ptr = (*ins).the_null_scb;
                }
            } else {
                (*(*scb).parent_scb_ptr).next_scb_ptr = (*scb).next_scb_ptr;

                if (*scb).next_scb_ptr != (*ins).the_null_scb {
                    /* update next node parent ptr. */
                    (*(*scb).next_scb_ptr).parent_scb_ptr = (*scb).parent_scb_ptr;
                }
                (*scb).next_scb_ptr = (*ins).the_null_scb;
            }

            /* update parent first entry in DSP RAM */
            cs46xx_dsp_spos_update_scb(chip, (*scb).parent_scb_ptr);

            /* then update entry in DSP RAM */
            cs46xx_dsp_spos_update_scb(chip, scb);

            (*scb).parent_scb_ptr = core::ptr::null_mut();
        }
    }
}

unsafe fn _dsp_clear_sample_buffer(
    chip: *mut snd_cs46xx,
    sample_buffer_addr: u32,
    dword_count: i32,
) {
    unsafe {
        let mut dst = (*chip).region.idx[2].remap_addr.add(sample_buffer_addr as usize);
        let mut i = 0;

        while i < dword_count {
            writel(0, dst);
            dst = dst.add(4);
            i += 1;
        }
    }
}

pub unsafe fn cs46xx_dsp_remove_scb(
    chip: *mut snd_cs46xx,
    scb: *mut dsp_scb_descriptor,
) {
    unsafe {
        let ins = (*chip).dsp_spos_instance;

        /* check integrety */
        if snd_BUG_ON(
            (*scb).index < 0
                || (*scb).index >= (*ins).nscb
                || (*ins).scbs.add((*scb).index as usize) != scb,
        ) {
            return;
        }

        /*
         * Disabled in C: cannot remove an SCB with children before removing
         * children first.
         */

        scoped_spinlock_irqsave(&mut (*chip).reg_lock, || {
            unsafe { _dsp_unlink_scb(chip, scb) }
        });

        cs46xx_dsp_proc_free_scb_desc(scb);
        if snd_BUG_ON((*scb).scb_symbol.is_null()) {
            return;
        }
        remove_symbol(chip, (*scb).scb_symbol);

        (*(*ins).scbs.add((*scb).index as usize)).deleted = 1;
        #[cfg(CONFIG_PM_SLEEP)]
        {
            kfree((*(*ins).scbs.add((*scb).index as usize)).data);
            (*(*ins).scbs.add((*scb).index as usize)).data = core::ptr::null_mut();
        }

        if (*scb).index < (*ins).scb_highest_frag_index {
            (*ins).scb_highest_frag_index = (*scb).index;
        }

        if (*scb).index == (*ins).nscb - 1 {
            (*ins).nscb -= 1;
        }

        if (*ins).scb_highest_frag_index > (*ins).nscb {
            (*ins).scb_highest_frag_index = (*ins).nscb;
        }
    }
}

#[cfg(CONFIG_SND_PROC_FS)]
pub unsafe fn cs46xx_dsp_proc_free_scb_desc(scb: *mut dsp_scb_descriptor) {
    unsafe {
        if !(*scb).proc_info.is_null() {
            let scb_info = (*(*scb).proc_info).private_data as *mut proc_scb_info;
            let chip = (*scb_info).chip;

            dev_dbg(
                (*(*chip).card).dev,
                c"cs46xx_dsp_proc_free_scb_desc: freeing %s\n".as_ptr(),
                (*scb).scb_name,
            );

            snd_info_free_entry((*scb).proc_info);
            (*scb).proc_info = core::ptr::null_mut();

            kfree(scb_info as *mut core::ffi::c_void);
        }
    }
}

#[cfg(not(CONFIG_SND_PROC_FS))]
pub unsafe fn cs46xx_dsp_proc_free_scb_desc(_scb: *mut dsp_scb_descriptor) {}

#[cfg(CONFIG_SND_PROC_FS)]
pub unsafe fn cs46xx_dsp_proc_register_scb_desc(
    chip: *mut snd_cs46xx,
    scb: *mut dsp_scb_descriptor,
) {
    unsafe {
        let ins = (*chip).dsp_spos_instance;
        let mut entry: *mut snd_info_entry = core::ptr::null_mut();
        let scb_info: *mut proc_scb_info;

        /* register to proc */
        if !(*ins).snd_card.is_null()
            && !(*ins).proc_dsp_dir.is_null()
            && (*scb).proc_info.is_null()
        {
            entry = snd_info_create_card_entry((*ins).snd_card, (*scb).scb_name, (*ins).proc_dsp_dir);
            if !entry.is_null() {
                scb_info = kmalloc_obj::<proc_scb_info>();
                if scb_info.is_null() {
                    snd_info_free_entry(entry);
                    entry = core::ptr::null_mut();
                } else {
                    (*scb_info).chip = chip;
                    (*scb_info).scb_desc = scb;
                    snd_info_set_text_ops(entry, scb_info as *mut core::ffi::c_void, cs46xx_dsp_proc_scb_info_read);
                }
            }
            (*scb).proc_info = entry;
        }
    }
}

#[cfg(not(CONFIG_SND_PROC_FS))]
pub unsafe fn cs46xx_dsp_proc_register_scb_desc(
    _chip: *mut snd_cs46xx,
    _scb: *mut dsp_scb_descriptor,
) {
}

unsafe fn _dsp_create_generic_scb(
    chip: *mut snd_cs46xx,
    name: *mut i8,
    scb_data: *mut u32,
    dest: u32,
    task_entry: *mut dsp_symbol_entry,
    parent_scb: *mut dsp_scb_descriptor,
    scb_child_type: i32,
) -> *mut dsp_scb_descriptor {
    unsafe {
        let ins = (*chip).dsp_spos_instance;
        let scb: *mut dsp_scb_descriptor;

        if snd_BUG_ON((*ins).the_null_scb.is_null()) {
            return core::ptr::null_mut();
        }

        /* fill the data that will be wroten to DSP */
        *scb_data.add(SCBsubListPtr as usize) =
            ((*(*ins).the_null_scb).address << 0x10) | (*(*ins).the_null_scb).address;

        *scb_data.add(SCBfuncEntryPtr as usize) &= 0xFFFF0000;
        *scb_data.add(SCBfuncEntryPtr as usize) |= (*task_entry).address;

        dev_dbg((*(*chip).card).dev, c"dsp_spos: creating SCB <%s>\n".as_ptr(), name);

        scb = cs46xx_dsp_create_scb(chip, name, scb_data, dest);

        (*scb).sub_list_ptr = (*ins).the_null_scb;
        (*scb).next_scb_ptr = (*ins).the_null_scb;
        (*scb).parent_scb_ptr = parent_scb;
        (*scb).task_entry = task_entry;

        /* update parent SCB */
        if !(*scb).parent_scb_ptr.is_null() {
            /* C debug block under #if 0 omitted, intent preserved. */
            /* link to parent SCB */
            if scb_child_type == SCB_ON_PARENT_NEXT_SCB {
                if snd_BUG_ON((*(*scb).parent_scb_ptr).next_scb_ptr != (*ins).the_null_scb) {
                    return core::ptr::null_mut();
                }

                (*(*scb).parent_scb_ptr).next_scb_ptr = scb;
            } else if scb_child_type == SCB_ON_PARENT_SUBLIST_SCB {
                if snd_BUG_ON((*(*scb).parent_scb_ptr).sub_list_ptr != (*ins).the_null_scb) {
                    return core::ptr::null_mut();
                }

                (*(*scb).parent_scb_ptr).sub_list_ptr = scb;
            } else {
                snd_BUG();
            }

            scoped_spinlock_irqsave(&mut (*chip).reg_lock, || {
                unsafe {
                    /* update entry in DSP RAM */
                    cs46xx_dsp_spos_update_scb(chip, (*scb).parent_scb_ptr);
                }
            });
        }

        cs46xx_dsp_proc_register_scb_desc(chip, scb);

        scb
    }
}

unsafe fn cs46xx_dsp_create_generic_scb(
    chip: *mut snd_cs46xx,
    name: *mut i8,
    scb_data: *mut u32,
    dest: u32,
    task_entry_name: *mut i8,
    parent_scb: *mut dsp_scb_descriptor,
    scb_child_type: i32,
) -> *mut dsp_scb_descriptor {
    unsafe {
        let task_entry = cs46xx_dsp_lookup_symbol(chip, task_entry_name, SYMBOL_CODE);

        if task_entry.is_null() {
            dev_err(
                (*(*chip).card).dev,
                c"dsp_spos: symbol %s not found\n".as_ptr(),
                task_entry_name,
            );
            return core::ptr::null_mut();
        }

        _dsp_create_generic_scb(
            chip,
            name,
            scb_data,
            dest,
            task_entry,
            parent_scb,
            scb_child_type,
        )
    }
}

pub unsafe fn cs46xx_dsp_create_timing_master_scb(
    chip: *mut snd_cs46xx,
) -> *mut dsp_scb_descriptor {
    unsafe {
        let mut timing_master_scb = dsp_timing_master_scb {
            basic_req: [0, 0, 0, 0],
            sg_req: [0, 0, 0, 0, 0],
            next_scb_ptr: 0,
            sub_list_ptr: 0,
            entry_point: 0,
            spb_ptr: NULL_SCB_ADDR,
            extra_sample_accum: 0,
            tm_reserved: 0,
            codec_fifo_ptr: 0,
            codec_fifo_syncd: 0,
            frac_samp_accum_qm1: 0x0001,
            tm_frms_left_in_group: 0x8000,
            frac_samp_correction_qm1: 0x0001,
            tm_frm_group_length: 0x0000,
            n_samp_per_frm_q15: 0x00060000,
        };

        cs46xx_dsp_create_generic_scb(
            chip,
            c"TimingMasterSCBInst".as_ptr() as *mut i8,
            &mut timing_master_scb as *mut _ as *mut u32,
            TIMINGMASTER_SCB_ADDR,
            c"TIMINGMASTER".as_ptr() as *mut i8,
            core::ptr::null_mut(),
            SCB_NO_PARENT,
        )
    }
}

pub unsafe fn cs46xx_dsp_create_codec_out_scb(
    chip: *mut snd_cs46xx,
    codec_name: *mut i8,
    channel_disp: u16,
    fifo_addr: u16,
    child_scb_addr: u16,
    dest: u32,
    parent_scb: *mut dsp_scb_descriptor,
    scb_child_type: i32,
) -> *mut dsp_scb_descriptor {
    unsafe {
        let mut codec_out_scb = dsp_codec_output_scb {
            basic_req: [0, 0, 0, 0],
            sg_req: [0, 0, 0, 0, 0],
            next_scb_ptr: 0,
            sub_list_ptr: 0,
            entry_point: 0,
            spb_ptr: NULL_SCB_ADDR,
            co_strm_rs_config: 0,
            co_strm_buf_ptr: 0,
            left_chan_base_io_addr: channel_disp,
            right_chan_io_disp: fifo_addr,
            co_exp_vol_change_rate: 0x0000,
            co_scale_shift_count: 0x0080,
            co_reserved: 0,
            co_child_scb: child_scb_addr,
        };

        cs46xx_dsp_create_generic_scb(
            chip,
            codec_name,
            &mut codec_out_scb as *mut _ as *mut u32,
            dest,
            c"S16_CODECOUTPUTTASK".as_ptr() as *mut i8,
            parent_scb,
            scb_child_type,
        )
    }
}

pub unsafe fn cs46xx_dsp_create_codec_in_scb(
    chip: *mut snd_cs46xx,
    codec_name: *mut i8,
    channel_disp: u16,
    fifo_addr: u16,
    sample_buffer_addr: u16,
    dest: u32,
    parent_scb: *mut dsp_scb_descriptor,
    scb_child_type: i32,
) -> *mut dsp_scb_descriptor {
    unsafe {
        let mut codec_input_scb = dsp_codec_input_scb {
            basic_req: [0, 0, 0, 0],
            sg_req: [0, 0, 0, 0, 0],
            next_scb_ptr: 0,
            sub_list_ptr: 0,
            entry_point: 0,
            spb_ptr: 0,
            strm_rs_config: RSCONFIG_SAMPLE_16STEREO + RSCONFIG_MODULO_64,
            strm_buf_ptr: (sample_buffer_addr as u32) << 0x10,
            left_chan_base_in_addr: channel_disp,
            right_chan_in_disp: fifo_addr,
            exp_vol_change_rate: 0x0000,
            scale_shift_count: 0x0000,
            reserved: 0x80008000,
        };

        cs46xx_dsp_create_generic_scb(
            chip,
            codec_name,
            &mut codec_input_scb as *mut _ as *mut u32,
            dest,
            c"S16_CODECINPUTTASK".as_ptr() as *mut i8,
            parent_scb,
            scb_child_type,
        )
    }
}

unsafe fn cs46xx_dsp_create_pcm_reader_scb(
    chip: *mut snd_cs46xx,
    scb_name: *mut i8,
    sample_buffer_addr: u16,
    dest: u32,
    virtual_channel: i32,
    playback_hw_addr: u32,
    parent_scb: *mut dsp_scb_descriptor,
    scb_child_type: i32,
) -> *mut dsp_scb_descriptor {
    unsafe {
        let ins = (*chip).dsp_spos_instance;
        let mut pcm_reader_scb = dsp_generic_scb {
            basic_req: [
                DMA_RQ_C1_SOURCE_ON_HOST
                    + DMA_RQ_C1_SOURCE_MOD1024
                    + DMA_RQ_C1_DEST_MOD32
                    + DMA_RQ_C1_WRITEBACK_SRC_FLAG
                    + DMA_RQ_C1_WRITEBACK_DEST_FLAG
                    + 15,
                DMA_RQ_C2_AC_NONE + DMA_RQ_C2_SIGNAL_SOURCE_PINGPONG + virtual_channel as u32,
                playback_hw_addr,
                DMA_RQ_SD_SP_SAMPLE_ADDR + sample_buffer_addr as u32,
            ],
            sg_req: [0, 0, 0, 0, 0],
            next_scb_ptr: NULL_SCB_ADDR,
            sub_list_ptr: NULL_SCB_ADDR,
            entry_point: 0,
            spb_ptr: NULL_SCB_ADDR,
            strm_rs_config: RSCONFIG_DMA_ENABLE
                + (19 << RSCONFIG_MAX_DMA_SIZE_SHIFT)
                + ((dest >> 4) << RSCONFIG_STREAM_NUM_SHIFT)
                + RSCONFIG_SAMPLE_16STEREO
                + RSCONFIG_MODULO_32,
            strm_buf_ptr: (sample_buffer_addr as u32) << 0x10,
            frac_inc: 0,
            volume: [0xffff, 0xffff, 0xffff, 0xffff],
        };

        if (*ins).null_algorithm.is_null() {
            (*ins).null_algorithm =
                cs46xx_dsp_lookup_symbol(chip, c"NULLALGORITHM".as_ptr() as *mut i8, SYMBOL_CODE);

            if (*ins).null_algorithm.is_null() {
                dev_err((*(*chip).card).dev, c"dsp_spos: symbol NULLALGORITHM not found\n".as_ptr());
                return core::ptr::null_mut();
            }
        }

        _dsp_create_generic_scb(
            chip,
            scb_name,
            &mut pcm_reader_scb as *mut _ as *mut u32,
            dest,
            (*ins).null_algorithm,
            parent_scb,
            scb_child_type,
        )
    }
}

pub const GOF_PER_SEC: u32 = 200;

pub unsafe fn cs46xx_dsp_create_src_task_scb(
    chip: *mut snd_cs46xx,
    scb_name: *mut i8,
    rate: i32,
    src_buffer_addr: u16,
    src_delay_buffer_addr: u16,
    dest: u32,
    parent_scb: *mut dsp_scb_descriptor,
    scb_child_type: i32,
    pass_through: i32,
) -> *mut dsp_scb_descriptor {
    unsafe {
        let ins = (*chip).dsp_spos_instance;
        let scb: *mut dsp_scb_descriptor;
        let mut tmp1: u32;
        let tmp2: u32;
        let mut phiIncr: u32;
        let correctionPerGOF: u32;
        let correctionPerSec: u32;

        dev_dbg(
            (*(*chip).card).dev,
            c"dsp_spos: setting %s rate to %u\n".as_ptr(),
            scb_name,
            rate,
        );

        /*
         * Compute sample rate conversion values:
         * phiIncr = floor((Fs,in * 2^26) / Fs,out)
         * correctionPerGOF = floor((Fs,in * 2^26 - Fs,out * phiIncr) / GOF_PER_SEC)
         * correctionPerSec = remaining correction per second.
         */
        tmp1 = (rate as u32) << 16;
        phiIncr = tmp1 / 48000;
        tmp1 -= phiIncr * 48000;
        tmp1 <<= 10;
        phiIncr <<= 10;
        tmp2 = tmp1 / 48000;
        phiIncr += tmp2;
        tmp1 -= tmp2 * 48000;
        correctionPerGOF = tmp1 / GOF_PER_SEC;
        tmp1 -= correctionPerGOF * GOF_PER_SEC;
        correctionPerSec = tmp1;

        let mut src_task_scb = dsp_src_task_scb {
            a0_right: 0x0028,
            a0_left: 0x00c8,
            a1_right: 0x5555,
            a1_left: 0x0000,
            a2_right: 0x0000,
            a2_left: 0x0000,
            output_buf_ptr: src_buffer_addr,
            init: 1,
            correction_per_gof: correctionPerGOF,
            correction_per_sec: correctionPerSec,
            input_rs_config: RSCONFIG_SAMPLE_16STEREO + RSCONFIG_MODULO_32,
            reserved1: 0x0000,
            delay_buf_ptr: src_delay_buffer_addr,
            write_pos: 0x0,
            delay_buf_size: 0x080,
            read_pos: src_delay_buffer_addr + (24 * 4),
            next_scb_ptr: 0,
            sub_list_ptr: 0,
            entry_point: 0,
            spb_ptr: 0,
            output_rs_config: RSCONFIG_SAMPLE_16STEREO + RSCONFIG_MODULO_8,
            output_buf_ptr2: (src_buffer_addr as u32) << 0x10,
            phi_incr: phiIncr,
            volume: [
                0xffff - (*ins).dac_volume_right,
                0xffff - (*ins).dac_volume_left,
                0xffff - (*ins).dac_volume_right,
                0xffff - (*ins).dac_volume_left,
            ],
        };

        if (*ins).s16_up.is_null() {
            (*ins).s16_up =
                cs46xx_dsp_lookup_symbol(chip, c"S16_UPSRC".as_ptr() as *mut i8, SYMBOL_CODE);

            if (*ins).s16_up.is_null() {
                dev_err((*(*chip).card).dev, c"dsp_spos: symbol S16_UPSRC not found\n".as_ptr());
                return core::ptr::null_mut();
            }
        }

        /* clear buffers */
        _dsp_clear_sample_buffer(chip, src_buffer_addr as u32, 8);
        _dsp_clear_sample_buffer(chip, src_delay_buffer_addr as u32, 32);

        if pass_through != 0 {
            /* wont work with any other rate than the native DSP rate */
            snd_BUG_ON(rate != 48000);

            scb = cs46xx_dsp_create_generic_scb(
                chip,
                scb_name,
                &mut src_task_scb as *mut _ as *mut u32,
                dest,
                c"DMAREADER".as_ptr() as *mut i8,
                parent_scb,
                scb_child_type,
            );
        } else {
            scb = _dsp_create_generic_scb(
                chip,
                scb_name,
                &mut src_task_scb as *mut _ as *mut u32,
                dest,
                (*ins).s16_up,
                parent_scb,
                scb_child_type,
            );
        }

        scb
    }
}

/* #if 0 not used: cs46xx_dsp_create_filter_scb was disabled in the C source. */

pub unsafe fn cs46xx_dsp_create_mix_only_scb(
    chip: *mut snd_cs46xx,
    scb_name: *mut i8,
    mix_buffer_addr: u16,
    dest: u32,
    parent_scb: *mut dsp_scb_descriptor,
    scb_child_type: i32,
) -> *mut dsp_scb_descriptor {
    unsafe {
        let mut master_mix_scb = dsp_mix_only_scb {
            basic_req: [0, 0, mix_buffer_addr as u32, 0],
            sg_req: [0, 0, 0, 0, 0x00000080],
            next_scb_ptr: 0,
            sub_list_ptr: 0,
            entry_point: 0,
            spb_ptr: 0,
            strm_rs_config: RSCONFIG_SAMPLE_16STEREO + RSCONFIG_MODULO_32,
            strm_buf_ptr: ((mix_buffer_addr + (16 * 4)) as u32) << 0x10,
            frac_inc: 0,
            volume: [0x8000, 0x8000, 0x8000, 0x8000],
        };

        cs46xx_dsp_create_generic_scb(
            chip,
            scb_name,
            &mut master_mix_scb as *mut _ as *mut u32,
            dest,
            c"S16_MIX".as_ptr() as *mut i8,
            parent_scb,
            scb_child_type,
        )
    }
}

pub unsafe fn cs46xx_dsp_create_mix_to_ostream_scb(
    chip: *mut snd_cs46xx,
    scb_name: *mut i8,
    mix_buffer_addr: u16,
    writeback_spb: u16,
    dest: u32,
    parent_scb: *mut dsp_scb_descriptor,
    scb_child_type: i32,
) -> *mut dsp_scb_descriptor {
    unsafe {
        let mut mix2_ostream_scb = dsp_mix2_ostream_scb {
            basic_req: [
                DMA_RQ_C1_SOURCE_MOD64
                    + DMA_RQ_C1_DEST_ON_HOST
                    + DMA_RQ_C1_DEST_MOD1024
                    + DMA_RQ_C1_WRITEBACK_SRC_FLAG
                    + DMA_RQ_C1_WRITEBACK_DEST_FLAG
                    + 15,
                DMA_RQ_C2_AC_NONE + DMA_RQ_C2_SIGNAL_DEST_PINGPONG,
                CS46XX_DSP_CAPTURE_CHANNEL as u32,
                DMA_RQ_SD_SP_SAMPLE_ADDR + mix_buffer_addr as u32,
                0x0,
            ],
            sg_req: [0, 0, 0, 0, 0],
            next_scb_ptr: 0,
            sub_list_ptr: 0,
            entry_point: 0,
            spb_ptr: writeback_spb,
            strm_rs_config: RSCONFIG_DMA_ENABLE
                + (19 << RSCONFIG_MAX_DMA_SIZE_SHIFT)
                + ((dest >> 4) << RSCONFIG_STREAM_NUM_SHIFT)
                + RSCONFIG_DMA_TO_HOST
                + RSCONFIG_SAMPLE_16STEREO
                + RSCONFIG_MODULO_64,
            strm_buf_ptr: ((mix_buffer_addr + (32 * 4)) as u32) << 0x10,
            frac_inc: 1,
            reserved: 0,
            exp_vol_change_rate: 0x0001,
            scale_shift_count: 0x0080,
            volume: 0xFFFF,
            reserved2: 0,
        };

        cs46xx_dsp_create_generic_scb(
            chip,
            scb_name,
            &mut mix2_ostream_scb as *mut _ as *mut u32,
            dest,
            c"S16_MIX_TO_OSTREAM".as_ptr() as *mut i8,
            parent_scb,
            scb_child_type,
        )
    }
}

pub unsafe fn cs46xx_dsp_create_vari_decimate_scb(
    chip: *mut snd_cs46xx,
    scb_name: *mut i8,
    vari_buffer_addr0: u16,
    vari_buffer_addr1: u16,
    dest: u32,
    parent_scb: *mut dsp_scb_descriptor,
    scb_child_type: i32,
) -> *mut dsp_scb_descriptor {
    unsafe {
        let mut vari_decimate_scb = dsp_vari_decimate_scb {
            a0_right: 0x0028,
            a0_left: 0x00c8,
            a1_right: 0x5555,
            a1_left: 0x0000,
            a2_right: 0x0000,
            a2_left: 0x0000,
            vari_buffer_addr0,
            vari_buffer_addr1,
            decim_left: 0x0028,
            decim_right: 0x00c8,
            input_rs_config: RSCONFIG_SAMPLE_16STEREO + RSCONFIG_MODULO_256,
            filter_state: 0xFF800000,
            unused: 0,
            delay_size: 0x0080,
            delay_ptr: vari_buffer_addr1 + (25 * 4),
            next_scb_ptr: 0,
            sub_list_ptr: 0,
            entry_point: 0,
            spb_ptr: 0,
            output_rs_config: RSCONFIG_SAMPLE_16STEREO + RSCONFIG_MODULO_8,
            output_buf_ptr: (vari_buffer_addr0 as u32) << 0x10,
            phi_incr: 0x04000000,
            volume: [0x8000, 0x8000, 0xFFFF, 0xFFFF],
        };

        cs46xx_dsp_create_generic_scb(
            chip,
            scb_name,
            &mut vari_decimate_scb as *mut _ as *mut u32,
            dest,
            c"VARIDECIMATE".as_ptr() as *mut i8,
            parent_scb,
            scb_child_type,
        )
    }
}

unsafe fn cs46xx_dsp_create_pcm_serial_input_scb(
    chip: *mut snd_cs46xx,
    scb_name: *mut i8,
    dest: u32,
    input_scb: *mut dsp_scb_descriptor,
    parent_scb: *mut dsp_scb_descriptor,
    scb_child_type: i32,
) -> *mut dsp_scb_descriptor {
    unsafe {
        let mut pcm_serial_input_scb = dsp_pcm_serial_input_scb {
            basic_req: [0, 0, 0, 0],
            sg_req: [0, 0, 0, 0, 0],
            next_scb_ptr: 0,
            sub_list_ptr: 0,
            entry_point: 0,
            spb_ptr: 0,
            strm_rs_config: RSCONFIG_SAMPLE_16STEREO + RSCONFIG_MODULO_16,
            strm_buf_ptr: 0,
            frac_inc: 0,
            input_scb: (*input_scb).address,
            volume: [0x8000, 0x8000, 0x8000, 0x8000],
        };

        cs46xx_dsp_create_generic_scb(
            chip,
            scb_name,
            &mut pcm_serial_input_scb as *mut _ as *mut u32,
            dest,
            c"PCMSERIALINPUTTASK".as_ptr() as *mut i8,
            parent_scb,
            scb_child_type,
        )
    }
}

unsafe fn cs46xx_dsp_create_asynch_fg_tx_scb(
    chip: *mut snd_cs46xx,
    scb_name: *mut i8,
    dest: u32,
    hfg_scb_address: u16,
    asynch_buffer_address: u16,
    parent_scb: *mut dsp_scb_descriptor,
    scb_child_type: i32,
) -> *mut dsp_scb_descriptor {
    unsafe {
        let mut asynch_fg_tx_scb = dsp_asynch_fg_tx_scb {
            buf_mask: 0xfc00,
            buf_size: 0x03ff,
            max_delta: 0x0058,
            min_delta: 0x0028,
            unused0: 0,
            hfg_scb_address,
            current_delta: 0,
            consumer_adjust: 0,
            accum_phi: 0,
            unused1: 0,
            const_one_third: 0x2aab,
            unused2: [0, 0, 0],
            next_scb_ptr: 0,
            sub_list_ptr: 0,
            entry_point: 0,
            this_ptr: dest + AFGTxAccumPhi,
            strm_rs_config: RSCONFIG_SAMPLE_16STEREO + RSCONFIG_MODULO_256,
            strm_buf_ptr: (asynch_buffer_address as u32) << 0x10,
            phi_incr: 0x18000000,
            volume: [0x8000, 0x8000, 0x8000, 0x8000],
        };

        cs46xx_dsp_create_generic_scb(
            chip,
            scb_name,
            &mut asynch_fg_tx_scb as *mut _ as *mut u32,
            dest,
            c"ASYNCHFGTXCODE".as_ptr() as *mut i8,
            parent_scb,
            scb_child_type,
        )
    }
}

pub unsafe fn cs46xx_dsp_create_asynch_fg_rx_scb(
    chip: *mut snd_cs46xx,
    scb_name: *mut i8,
    dest: u32,
    hfg_scb_address: u16,
    asynch_buffer_address: u16,
    parent_scb: *mut dsp_scb_descriptor,
    scb_child_type: i32,
) -> *mut dsp_scb_descriptor {
    unsafe {
        let ins = (*chip).dsp_spos_instance;
        let mut asynch_fg_rx_scb = dsp_asynch_fg_rx_scb {
            buf_mask: 0xfe00,
            buf_size: 0x01ff,
            max_delta: 0x0064,
            min_delta: 0x001c,
            unused0: 0,
            hfg_scb_address,
            current_delta: 0,
            consumer_adjust: 0,
            unused1: [0, 0, 0, 0, 0],
            next_scb_ptr: 0,
            sub_list_ptr: 0,
            entry_point: 0,
            this_ptr: dest,
            strm_rs_config: RSCONFIG_MODULO_128 | RSCONFIG_SAMPLE_16STEREO,
            strm_buf_ptr: ((asynch_buffer_address + (16 * 4)) as u32) << 0x10,
            phi_incr: 0x18000000,
            volume: [
                0xffff - (*ins).spdif_input_volume_right,
                0xffff - (*ins).spdif_input_volume_left,
                0xffff - (*ins).spdif_input_volume_right,
                0xffff - (*ins).spdif_input_volume_left,
            ],
        };

        cs46xx_dsp_create_generic_scb(
            chip,
            scb_name,
            &mut asynch_fg_rx_scb as *mut _ as *mut u32,
            dest,
            c"ASYNCHFGRXCODE".as_ptr() as *mut i8,
            parent_scb,
            scb_child_type,
        )
    }
}

/* #if 0 not used: cs46xx_dsp_create_output_snoop_scb was disabled in C. */

pub unsafe fn cs46xx_dsp_create_spio_write_scb(
    chip: *mut snd_cs46xx,
    scb_name: *mut i8,
    dest: u32,
    parent_scb: *mut dsp_scb_descriptor,
    scb_child_type: i32,
) -> *mut dsp_scb_descriptor {
    unsafe {
        let mut spio_write_scb = dsp_spio_write_scb {
            spio_w_address2: 0,
            spio_w_address1: 0,
            spio_w_data1: 0,
            spio_w_data2: 0,
            spio_w_address4: 0,
            spio_w_address3: 0,
            spio_w_data3: 0,
            spio_w_data4: 0,
            spio_w_data_ptr: 0,
            unused1: 0,
            unused2: [0, 0],
            spio_w_child_ptr: 0,
            spio_w_sibling_ptr: 0,
            spio_w_this_ptr: 0,
            spio_w_entry_point: 0,
            unused3: [0, 0, 0, 0, 0],
        };

        cs46xx_dsp_create_generic_scb(
            chip,
            scb_name,
            &mut spio_write_scb as *mut _ as *mut u32,
            dest,
            c"SPIOWRITE".as_ptr() as *mut i8,
            parent_scb,
            scb_child_type,
        )
    }
}

pub unsafe fn cs46xx_dsp_create_magic_snoop_scb(
    chip: *mut snd_cs46xx,
    scb_name: *mut i8,
    dest: u32,
    snoop_buffer_address: u16,
    snoop_scb: *mut dsp_scb_descriptor,
    parent_scb: *mut dsp_scb_descriptor,
    scb_child_type: i32,
) -> *mut dsp_scb_descriptor {
    unsafe {
        let mut magic_snoop_scb = dsp_magic_snoop_task {
            i0: 0,
            i1: 0,
            snoop_buffer: (snoop_buffer_address as u32) << 0x10,
            unused2: 0,
            snoop_scb: (*snoop_scb).address,
            i3: 0,
            i4: 0,
            i5: 0,
            i6: 0,
            i7: 0,
            next_scb_ptr: 0,
            sub_list_ptr: 0,
            entry_point: 0,
            this_ptr: 0,
            strm_rs_config: RSCONFIG_SAMPLE_16STEREO + RSCONFIG_MODULO_64,
            strm_buf_ptr: (snoop_buffer_address as u32) << 0x10,
            frac_inc: 0,
            volume: [0x8000, 0x8000, 0xffff, 0xffff],
        };

        cs46xx_dsp_create_generic_scb(
            chip,
            scb_name,
            &mut magic_snoop_scb as *mut _ as *mut u32,
            dest,
            c"MAGICSNOOPTASK".as_ptr() as *mut i8,
            parent_scb,
            scb_child_type,
        )
    }
}

unsafe fn find_next_free_scb(
    chip: *mut snd_cs46xx,
    from: *mut dsp_scb_descriptor,
) -> *mut dsp_scb_descriptor {
    unsafe {
        let ins = (*chip).dsp_spos_instance;
        let mut scb = from;

        while (*scb).next_scb_ptr != (*ins).the_null_scb {
            if snd_BUG_ON((*scb).next_scb_ptr.is_null()) {
                return core::ptr::null_mut();
            }

            scb = (*scb).next_scb_ptr;
        }

        scb
    }
}

static pcm_reader_buffer_addr: [u32; DSP_MAX_PCM_CHANNELS as usize] = [
    0x0600, 0x1500, 0x1580, 0x1600, 0x1680, 0x1700, 0x1780, 0x1800,
    0x1880, 0x1900, 0x1980, 0x1A00, 0x1A80, 0x1B00, 0x1B80, 0x1C00,
    0x1C80, 0x1D00, 0x1D80, 0x1E00, 0x1E80, 0x1F00, 0x1F80, 0x2000,
    0x2080, 0x2100, 0x2180, 0x2200, 0x2280, 0x2300, 0x2380, 0x2400,
];

static src_output_buffer_addr: [u32; DSP_MAX_SRC_NR as usize] = [
    0x2B80, 0x2BA0, 0x2BC0, 0x2BE0, 0x2D00, 0x2D20, 0x2D40,
    0x2D60, 0x2D80, 0x2DA0, 0x2DC0, 0x2DE0, 0x2E00, 0x2E20,
];

static src_delay_buffer_addr: [u32; DSP_MAX_SRC_NR as usize] = [
    0x2480, 0x2500, 0x2580, 0x2600, 0x2680, 0x2700, 0x2780,
    0x2800, 0x2880, 0x2900, 0x2980, 0x2A00, 0x2A80, 0x2B00,
];

pub unsafe fn cs46xx_dsp_create_pcm_channel(
    chip: *mut snd_cs46xx,
    mut sample_rate: u32,
    private_data: *mut core::ffi::c_void,
    hw_dma_addr: u32,
    pcm_channel_id: i32,
) -> *mut dsp_pcm_channel_descriptor {
    unsafe {
        let ins = (*chip).dsp_spos_instance;
        let mut src_scb: *mut dsp_scb_descriptor = core::ptr::null_mut();
        let pcm_scb: *mut dsp_scb_descriptor;
        let mut mixer_scb: *mut dsp_scb_descriptor = core::ptr::null_mut();
        let mut src_parent_scb: *mut dsp_scb_descriptor;
        let mut scb_name = [0i8; DSP_MAX_SCB_NAME as usize];
        let mut i: i32;
        let mut pcm_index: i32 = -1;
        let insert_point: i32;
        let mut src_index: i32 = -1;
        let mut pass_through: i32 = 0;

        match pcm_channel_id {
            DSP_PCM_MAIN_CHANNEL => mixer_scb = (*ins).master_mix_scb,
            DSP_PCM_REAR_CHANNEL => mixer_scb = (*ins).rear_mix_scb,
            DSP_PCM_CENTER_LFE_CHANNEL => mixer_scb = (*ins).center_lfe_mix_scb,
            DSP_PCM_S71_CHANNEL => {
                /* TODO */
                snd_BUG();
            }
            DSP_IEC958_CHANNEL => {
                if snd_BUG_ON((*ins).asynch_tx_scb.is_null()) {
                    return core::ptr::null_mut();
                }
                mixer_scb = (*ins).asynch_tx_scb;
                if sample_rate == 48000 {
                    dev_dbg((*(*chip).card).dev, c"IEC958 pass through\n".as_ptr());
                    pass_through = 1;
                }
            }
            _ => {
                snd_BUG();
                return core::ptr::null_mut();
            }
        }

        if sample_rate == 0 {
            sample_rate = 44100;
        }

        i = 0;
        while i < DSP_MAX_PCM_CHANNELS && (pcm_index == -1 || src_scb.is_null()) {
            if i == CS46XX_DSP_CAPTURE_CHANNEL {
                i += 1;
                continue;
            }

            if (*ins).pcm_channels[i as usize].active != 0 {
                if src_scb.is_null()
                    && (*ins).pcm_channels[i as usize].sample_rate == sample_rate
                    && (*ins).pcm_channels[i as usize].mixer_scb == mixer_scb
                {
                    src_scb = (*ins).pcm_channels[i as usize].src_scb;
                    (*(*ins).pcm_channels[i as usize].src_scb).ref_count += 1;
                    src_index = (*ins).pcm_channels[i as usize].src_slot;
                }
            } else if pcm_index == -1 {
                pcm_index = i;
            }
            i += 1;
        }

        if pcm_index == -1 {
            dev_err((*(*chip).card).dev, c"dsp_spos: no free PCM channel\n".as_ptr());
            return core::ptr::null_mut();
        }

        if src_scb.is_null() {
            if (*ins).nsrc_scb >= DSP_MAX_SRC_NR {
                dev_err((*(*chip).card).dev, c"dsp_spos: too many SRC instances\n!".as_ptr());
                return core::ptr::null_mut();
            }

            i = 0;
            while i < DSP_MAX_SRC_NR {
                if (*ins).src_scb_slots[i as usize] == 0 {
                    src_index = i;
                    (*ins).src_scb_slots[i as usize] = 1;
                    break;
                }
                i += 1;
            }
            if snd_BUG_ON(src_index == -1) {
                return core::ptr::null_mut();
            }

            if (*mixer_scb).sub_list_ptr == (*ins).the_null_scb {
                src_parent_scb = mixer_scb;
                insert_point = SCB_ON_PARENT_SUBLIST_SCB;
            } else {
                src_parent_scb = find_next_free_scb(chip, (*mixer_scb).sub_list_ptr);
                insert_point = SCB_ON_PARENT_NEXT_SCB;
            }

            snprintf(
                scb_name.as_mut_ptr(),
                DSP_MAX_SCB_NAME,
                c"SrcTask_SCB%d".as_ptr(),
                src_index,
            );

            dev_dbg(
                (*(*chip).card).dev,
                c"dsp_spos: creating SRC \"%s\"\n".as_ptr(),
                scb_name.as_mut_ptr(),
            );
            src_scb = cs46xx_dsp_create_src_task_scb(
                chip,
                scb_name.as_mut_ptr(),
                sample_rate as i32,
                src_output_buffer_addr[src_index as usize] as u16,
                src_delay_buffer_addr[src_index as usize] as u16,
                0x400 + ((src_index as u32) * 0x10),
                src_parent_scb,
                insert_point,
                pass_through,
            );

            if src_scb.is_null() {
                dev_err((*(*chip).card).dev, c"dsp_spos: failed to create SRCtaskSCB\n".as_ptr());
                return core::ptr::null_mut();
            }

            (*ins).nsrc_scb += 1;
        }

        snprintf(
            scb_name.as_mut_ptr(),
            DSP_MAX_SCB_NAME,
            c"PCMReader_SCB%d".as_ptr(),
            pcm_index,
        );

        dev_dbg(
            (*(*chip).card).dev,
            c"dsp_spos: creating PCM \"%s\" (%d)\n".as_ptr(),
            scb_name.as_mut_ptr(),
            pcm_channel_id,
        );

        pcm_scb = cs46xx_dsp_create_pcm_reader_scb(
            chip,
            scb_name.as_mut_ptr(),
            pcm_reader_buffer_addr[pcm_index as usize] as u16,
            (pcm_index as u32 * 0x10) + 0x200,
            pcm_index,
            hw_dma_addr,
            core::ptr::null_mut(),
            0,
        );

        if pcm_scb.is_null() {
            dev_err((*(*chip).card).dev, c"dsp_spos: failed to create PCMreaderSCB\n".as_ptr());
            return core::ptr::null_mut();
        }

        guard_spinlock_irqsave(&mut (*chip).reg_lock);
        (*ins).pcm_channels[pcm_index as usize].sample_rate = sample_rate;
        (*ins).pcm_channels[pcm_index as usize].pcm_reader_scb = pcm_scb;
        (*ins).pcm_channels[pcm_index as usize].src_scb = src_scb;
        (*ins).pcm_channels[pcm_index as usize].unlinked = 1;
        (*ins).pcm_channels[pcm_index as usize].private_data = private_data;
        (*ins).pcm_channels[pcm_index as usize].src_slot = src_index;
        (*ins).pcm_channels[pcm_index as usize].active = 1;
        (*ins).pcm_channels[pcm_index as usize].pcm_slot = pcm_index;
        (*ins).pcm_channels[pcm_index as usize].mixer_scb = mixer_scb;
        (*ins).npcm_channels += 1;

        (*ins).pcm_channels.as_mut_ptr().add(pcm_index as usize)
    }
}

pub unsafe fn cs46xx_dsp_pcm_channel_set_period(
    chip: *mut snd_cs46xx,
    pcm_channel: *mut dsp_pcm_channel_descriptor,
    period_size: i32,
) -> i32 {
    unsafe {
        let mut temp = snd_cs46xx_peek(chip, (*(*pcm_channel).pcm_reader_scb).address << 2);
        temp &= !DMA_RQ_C1_SOURCE_SIZE_MASK;

        match period_size {
            2048 => temp |= DMA_RQ_C1_SOURCE_MOD1024,
            1024 => temp |= DMA_RQ_C1_SOURCE_MOD512,
            512 => temp |= DMA_RQ_C1_SOURCE_MOD256,
            256 => temp |= DMA_RQ_C1_SOURCE_MOD128,
            128 => temp |= DMA_RQ_C1_SOURCE_MOD64,
            64 => temp |= DMA_RQ_C1_SOURCE_MOD32,
            32 => temp |= DMA_RQ_C1_SOURCE_MOD16,
            _ => {
                dev_dbg(
                    (*(*chip).card).dev,
                    c"period size (%d) not supported by HW\n".as_ptr(),
                    period_size,
                );
                return -EINVAL;
            }
        }

        snd_cs46xx_poke(chip, (*(*pcm_channel).pcm_reader_scb).address << 2, temp);

        0
    }
}

pub unsafe fn cs46xx_dsp_pcm_ostream_set_period(
    chip: *mut snd_cs46xx,
    period_size: i32,
) -> i32 {
    unsafe {
        let mut temp = snd_cs46xx_peek(chip, WRITEBACK_SCB_ADDR << 2);
        temp &= !DMA_RQ_C1_DEST_SIZE_MASK;

        match period_size {
            2048 => temp |= DMA_RQ_C1_DEST_MOD1024,
            1024 => temp |= DMA_RQ_C1_DEST_MOD512,
            512 => temp |= DMA_RQ_C1_DEST_MOD256,
            256 => temp |= DMA_RQ_C1_DEST_MOD128,
            128 => temp |= DMA_RQ_C1_DEST_MOD64,
            64 => temp |= DMA_RQ_C1_DEST_MOD32,
            32 => temp |= DMA_RQ_C1_DEST_MOD16,
            _ => {
                dev_dbg(
                    (*(*chip).card).dev,
                    c"period size (%d) not supported by HW\n".as_ptr(),
                    period_size,
                );
                return -EINVAL;
            }
        }

        snd_cs46xx_poke(chip, WRITEBACK_SCB_ADDR << 2, temp);

        0
    }
}

pub unsafe fn cs46xx_dsp_destroy_pcm_channel(
    chip: *mut snd_cs46xx,
    pcm_channel: *mut dsp_pcm_channel_descriptor,
) {
    unsafe {
        let ins = (*chip).dsp_spos_instance;

        if snd_BUG_ON(
            (*pcm_channel).active == 0
                || (*ins).npcm_channels <= 0
                || (*(*pcm_channel).src_scb).ref_count <= 0,
        ) {
            return;
        }

        scoped_spinlock_irqsave(&mut (*chip).reg_lock, || {
            unsafe {
                (*pcm_channel).unlinked = 1;
                (*pcm_channel).active = 0;
                (*pcm_channel).private_data = core::ptr::null_mut();
                (*(*pcm_channel).src_scb).ref_count -= 1;
                (*ins).npcm_channels -= 1;
            }
        });

        cs46xx_dsp_remove_scb(chip, (*pcm_channel).pcm_reader_scb);

        if (*(*pcm_channel).src_scb).ref_count == 0 {
            cs46xx_dsp_remove_scb(chip, (*pcm_channel).src_scb);

            if snd_BUG_ON((*pcm_channel).src_slot < 0 || (*pcm_channel).src_slot >= DSP_MAX_SRC_NR) {
                return;
            }

            (*ins).src_scb_slots[(*pcm_channel).src_slot as usize] = 0;
            (*ins).nsrc_scb -= 1;
        }
    }
}

pub unsafe fn cs46xx_dsp_pcm_unlink(
    chip: *mut snd_cs46xx,
    pcm_channel: *mut dsp_pcm_channel_descriptor,
) -> i32 {
    unsafe {
        if snd_BUG_ON(
            (*pcm_channel).active == 0 || (*(*chip).dsp_spos_instance).npcm_channels <= 0,
        ) {
            return -EIO;
        }

        guard_spinlock_irqsave(&mut (*chip).reg_lock);
        if (*pcm_channel).unlinked != 0 {
            return -EIO;
        }

        (*pcm_channel).unlinked = 1;

        _dsp_unlink_scb(chip, (*pcm_channel).pcm_reader_scb);

        0
    }
}

pub unsafe fn cs46xx_dsp_pcm_link(
    chip: *mut snd_cs46xx,
    pcm_channel: *mut dsp_pcm_channel_descriptor,
) -> i32 {
    unsafe {
        let ins = (*chip).dsp_spos_instance;
        let parent_scb: *mut dsp_scb_descriptor;
        let src_scb = (*pcm_channel).src_scb;

        guard_spinlock_irqsave(&mut (*chip).reg_lock);

        if (*pcm_channel).unlinked == 0 {
            return -EIO;
        }

        parent_scb = src_scb;

        if (*src_scb).sub_list_ptr != (*ins).the_null_scb {
            (*(*src_scb).sub_list_ptr).parent_scb_ptr = (*pcm_channel).pcm_reader_scb;
            (*(*pcm_channel).pcm_reader_scb).next_scb_ptr = (*src_scb).sub_list_ptr;
        }

        (*src_scb).sub_list_ptr = (*pcm_channel).pcm_reader_scb;

        snd_BUG_ON(!(*(*pcm_channel).pcm_reader_scb).parent_scb_ptr.is_null());
        (*(*pcm_channel).pcm_reader_scb).parent_scb_ptr = parent_scb;

        /* update SCB entry in DSP RAM */
        cs46xx_dsp_spos_update_scb(chip, (*pcm_channel).pcm_reader_scb);

        /* update parent SCB entry */
        cs46xx_dsp_spos_update_scb(chip, parent_scb);

        (*pcm_channel).unlinked = 0;
        0
    }
}

pub unsafe fn cs46xx_add_record_source(
    chip: *mut snd_cs46xx,
    source: *mut dsp_scb_descriptor,
    addr: u16,
    scb_name: *mut i8,
) -> *mut dsp_scb_descriptor {
    unsafe {
        let ins = (*chip).dsp_spos_instance;
        let parent: *mut dsp_scb_descriptor;
        let pcm_input: *mut dsp_scb_descriptor;
        let insert_point: i32;

        if snd_BUG_ON((*ins).record_mixer_scb.is_null()) {
            return core::ptr::null_mut();
        }

        if (*(*ins).record_mixer_scb).sub_list_ptr != (*ins).the_null_scb {
            parent = find_next_free_scb(chip, (*(*ins).record_mixer_scb).sub_list_ptr);
            insert_point = SCB_ON_PARENT_NEXT_SCB;
        } else {
            parent = (*ins).record_mixer_scb;
            insert_point = SCB_ON_PARENT_SUBLIST_SCB;
        }

        pcm_input = cs46xx_dsp_create_pcm_serial_input_scb(
            chip,
            scb_name,
            addr as u32,
            source,
            parent,
            insert_point,
        );

        pcm_input
    }
}

pub unsafe fn cs46xx_src_unlink(
    chip: *mut snd_cs46xx,
    src: *mut dsp_scb_descriptor,
) -> i32 {
    unsafe {
        if snd_BUG_ON((*src).parent_scb_ptr.is_null()) {
            return -EINVAL;
        }

        /* mute SCB */
        cs46xx_dsp_scb_set_volume(chip, src, 0, 0);

        guard_spinlock_irqsave(&mut (*chip).reg_lock);
        _dsp_unlink_scb(chip, src);

        0
    }
}

pub unsafe fn cs46xx_src_link(
    chip: *mut snd_cs46xx,
    src: *mut dsp_scb_descriptor,
) -> i32 {
    unsafe {
        let ins = (*chip).dsp_spos_instance;
        let parent_scb: *mut dsp_scb_descriptor;

        if snd_BUG_ON(!(*src).parent_scb_ptr.is_null()) {
            return -EINVAL;
        }
        if snd_BUG_ON((*ins).master_mix_scb.is_null()) {
            return -EINVAL;
        }

        if (*(*ins).master_mix_scb).sub_list_ptr != (*ins).the_null_scb {
            parent_scb = find_next_free_scb(chip, (*(*ins).master_mix_scb).sub_list_ptr);
            (*parent_scb).next_scb_ptr = src;
        } else {
            parent_scb = (*ins).master_mix_scb;
            (*parent_scb).sub_list_ptr = src;
        }

        (*src).parent_scb_ptr = parent_scb;

        /* update entry in DSP RAM */
        cs46xx_dsp_spos_update_scb(chip, parent_scb);

        0
    }
}

pub unsafe fn cs46xx_dsp_enable_spdif_out(chip: *mut snd_cs46xx) -> i32 {
    unsafe {
        let ins = (*chip).dsp_spos_instance;

        if ((*ins).spdif_status_out & DSP_SPDIF_STATUS_HW_ENABLED) == 0 {
            cs46xx_dsp_enable_spdif_hw(chip);
        }

        /* dont touch anything if SPDIF is open */
        if ((*ins).spdif_status_out & DSP_SPDIF_STATUS_PLAYBACK_OPEN) != 0 {
            (*ins).spdif_status_out |= DSP_SPDIF_STATUS_OUTPUT_ENABLED;
            return -EBUSY;
        }

        if snd_BUG_ON(!(*ins).asynch_tx_scb.is_null()) {
            return -EINVAL;
        }
        if snd_BUG_ON((*(*ins).master_mix_scb).next_scb_ptr != (*ins).the_null_scb) {
            return -EINVAL;
        }

        /* reset output snooper sample buffer pointer */
        snd_cs46xx_poke(
            chip,
            ((*(*ins).ref_snoop_scb).address + 2) << 2,
            (OUTPUT_SNOOP_BUFFER + 0x10) << 0x10,
        );

        /* The asynch. transfer task */
        (*ins).asynch_tx_scb = cs46xx_dsp_create_asynch_fg_tx_scb(
            chip,
            c"AsynchFGTxSCB".as_ptr() as *mut i8,
            ASYNCTX_SCB_ADDR,
            SPDIFO_SCB_INST,
            SPDIFO_IP_OUTPUT_BUFFER1,
            (*ins).master_mix_scb,
            SCB_ON_PARENT_NEXT_SCB,
        );
        if (*ins).asynch_tx_scb.is_null() {
            return -ENOMEM;
        }

        (*ins).spdif_pcm_input_scb = cs46xx_dsp_create_pcm_serial_input_scb(
            chip,
            c"PCMSerialInput_II".as_ptr() as *mut i8,
            PCMSERIALINII_SCB_ADDR,
            (*ins).ref_snoop_scb,
            (*ins).asynch_tx_scb,
            SCB_ON_PARENT_SUBLIST_SCB,
        );

        if (*ins).spdif_pcm_input_scb.is_null() {
            return -ENOMEM;
        }

        /* monitor state */
        (*ins).spdif_status_out |= DSP_SPDIF_STATUS_OUTPUT_ENABLED;

        0
    }
}

pub unsafe fn cs46xx_dsp_disable_spdif_out(chip: *mut snd_cs46xx) -> i32 {
    unsafe {
        let ins = (*chip).dsp_spos_instance;

        /* dont touch anything if SPDIF is open */
        if ((*ins).spdif_status_out & DSP_SPDIF_STATUS_PLAYBACK_OPEN) != 0 {
            (*ins).spdif_status_out &= !DSP_SPDIF_STATUS_OUTPUT_ENABLED;
            return -EBUSY;
        }

        /* check integrety */
        if snd_BUG_ON((*ins).asynch_tx_scb.is_null()) {
            return -EINVAL;
        }
        if snd_BUG_ON((*ins).spdif_pcm_input_scb.is_null()) {
            return -EINVAL;
        }
        if snd_BUG_ON((*(*ins).master_mix_scb).next_scb_ptr != (*ins).asynch_tx_scb) {
            return -EINVAL;
        }
        if snd_BUG_ON((*(*ins).asynch_tx_scb).parent_scb_ptr != (*ins).master_mix_scb) {
            return -EINVAL;
        }

        cs46xx_dsp_remove_scb(chip, (*ins).spdif_pcm_input_scb);
        cs46xx_dsp_remove_scb(chip, (*ins).asynch_tx_scb);

        (*ins).spdif_pcm_input_scb = core::ptr::null_mut();
        (*ins).asynch_tx_scb = core::ptr::null_mut();

        /* clear buffer to prevent any undesired noise */
        _dsp_clear_sample_buffer(chip, SPDIFO_IP_OUTPUT_BUFFER1, 256);

        /* monitor state */
        (*ins).spdif_status_out &= !DSP_SPDIF_STATUS_OUTPUT_ENABLED;

        0
    }
}

pub unsafe fn cs46xx_iec958_pre_open(chip: *mut snd_cs46xx) -> i32 {
    unsafe {
        let ins = (*chip).dsp_spos_instance;

        if ((*ins).spdif_status_out & DSP_SPDIF_STATUS_OUTPUT_ENABLED) != 0 {
            /* remove AsynchFGTxSCB and PCMSerialInput_II */
            cs46xx_dsp_disable_spdif_out(chip);

            /* save state */
            (*ins).spdif_status_out |= DSP_SPDIF_STATUS_OUTPUT_ENABLED;
        }

        /* if not enabled already */
        if ((*ins).spdif_status_out & DSP_SPDIF_STATUS_HW_ENABLED) == 0 {
            cs46xx_dsp_enable_spdif_hw(chip);
        }

        /* Create the asynch. transfer task for playback */
        (*ins).asynch_tx_scb = cs46xx_dsp_create_asynch_fg_tx_scb(
            chip,
            c"AsynchFGTxSCB".as_ptr() as *mut i8,
            ASYNCTX_SCB_ADDR,
            SPDIFO_SCB_INST,
            SPDIFO_IP_OUTPUT_BUFFER1,
            (*ins).master_mix_scb,
            SCB_ON_PARENT_NEXT_SCB,
        );

        /* set spdif channel status value for streaming */
        cs46xx_poke_via_dsp(chip, SP_SPDOUT_CSUV, (*ins).spdif_csuv_stream);

        (*ins).spdif_status_out |= DSP_SPDIF_STATUS_PLAYBACK_OPEN;

        0
    }
}

pub unsafe fn cs46xx_iec958_post_close(chip: *mut snd_cs46xx) -> i32 {
    unsafe {
        let ins = (*chip).dsp_spos_instance;

        if snd_BUG_ON((*ins).asynch_tx_scb.is_null()) {
            return -EINVAL;
        }

        (*ins).spdif_status_out &= !DSP_SPDIF_STATUS_PLAYBACK_OPEN;

        /* restore settings */
        cs46xx_poke_via_dsp(chip, SP_SPDOUT_CSUV, (*ins).spdif_csuv_default);

        /* deallocate stuff */
        if !(*ins).spdif_pcm_input_scb.is_null() {
            cs46xx_dsp_remove_scb(chip, (*ins).spdif_pcm_input_scb);
            (*ins).spdif_pcm_input_scb = core::ptr::null_mut();
        }

        cs46xx_dsp_remove_scb(chip, (*ins).asynch_tx_scb);
        (*ins).asynch_tx_scb = core::ptr::null_mut();

        /* clear buffer to prevent any undesired noise */
        _dsp_clear_sample_buffer(chip, SPDIFO_IP_OUTPUT_BUFFER1, 256);

        /* restore state */
        if ((*ins).spdif_status_out & DSP_SPDIF_STATUS_OUTPUT_ENABLED) != 0 {
            cs46xx_dsp_enable_spdif_out(chip);
        }

        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
