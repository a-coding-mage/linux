// SPDX-License-Identifier: GPL-2.0+
/* Command support for NI general purpose counters. */

// C dependencies supplied by the surrounding driver/kernel bindings.

unsafe fn ni_tio_configure_dma(counter: *mut ni_gpct, enable: bool, read: bool) {
    let counter_dev = (*counter).counter_dev;
    let cidx = (*counter).counter_index;
    let mut mask = GI_READ_ACKS_IRQ | GI_WRITE_ACKS_IRQ;
    let mut bits = 0;

    if enable {
        if read { bits |= GI_READ_ACKS_IRQ; } else { bits |= GI_WRITE_ACKS_IRQ; }
    }
    ni_tio_set_bits(counter, NITIO_INPUT_SEL_REG(cidx), mask, bits);
    match (*counter_dev).variant {
        ni_gpct_variant_e_series => {}
        ni_gpct_variant_m_series | ni_gpct_variant_660x => {
            mask = GI_DMA_ENABLE | GI_DMA_INT_ENA | GI_DMA_WRITE;
            bits = 0;
            if enable { bits |= GI_DMA_ENABLE | GI_DMA_INT_ENA; }
            if !read { bits |= GI_DMA_WRITE; }
            ni_tio_set_bits(counter, NITIO_DMA_CFG_REG(cidx), mask, bits);
        }
        _ => {}
    }
}

unsafe fn ni_tio_input_inttrig(_dev: *mut comedi_device, s: *mut comedi_subdevice, trig_num: u32) -> i32 {
    let counter = (*s).private as *mut ni_gpct;
    let cmd = &mut (*(*s).async_).cmd;
    let mut ret = 0;
    if trig_num != cmd.start_arg { return -EINVAL; }
    let mut flags = 0UL;
    spin_lock_irqsave(&mut (*counter).lock, &mut flags);
    if !(*counter).mite_chan.is_null() { mite_dma_arm((*counter).mite_chan); } else { ret = -EIO; }
    spin_unlock_irqrestore(&mut (*counter).lock, flags);
    if ret < 0 { return ret; }
    ret = ni_tio_arm(counter, true, NI_GPCT_ARM_IMMEDIATE);
    (*(*s).async_).inttrig = None;
    ret
}

unsafe fn ni_tio_input_cmd(s: *mut comedi_subdevice) -> i32 {
    let counter = (*s).private as *mut ni_gpct;
    let counter_dev = (*counter).counter_dev;
    let routing_tables = (*counter_dev).routing_tables;
    let cidx = (*counter).counter_index;
    let async_ = (*s).async_;
    let cmd = &mut (*async_).cmd;
    let mut ret = 0;
    comedi_buf_write_alloc(s, (*async_).prealloc_bufsz);
    (*counter).mite_chan.as_mut().unwrap().dir = COMEDI_INPUT;
    match (*counter_dev).variant {
        ni_gpct_variant_m_series | ni_gpct_variant_660x => mite_prep_dma((*counter).mite_chan, 32, 32),
        ni_gpct_variant_e_series => mite_prep_dma((*counter).mite_chan, 16, 32),
        _ => {}
    }
    ni_tio_set_bits(counter, NITIO_CMD_REG(cidx), GI_SAVE_TRACE, 0);
    ni_tio_configure_dma(counter, true, true);
    if cmd.start_src == TRIG_INT {
        (*async_).inttrig = Some(ni_tio_input_inttrig);
    } else {
        (*async_).inttrig = None;
        mite_dma_arm((*counter).mite_chan);
        if cmd.start_src == TRIG_NOW { ret = ni_tio_arm(counter, true, NI_GPCT_ARM_IMMEDIATE); }
        else if cmd.start_src == TRIG_EXT {
            let mut reg = CR_CHAN(cmd.start_arg);
            if reg >= NI_NAMES_BASE { reg = ni_get_reg_value(reg, NI_CtrArmStartTrigger(cidx), routing_tables); reg |= NI_GPCT_HW_ARM; }
            ret = ni_tio_arm(counter, true, reg);
        }
    }
    ret
}

unsafe fn ni_tio_output_cmd(s: *mut comedi_subdevice) -> i32 {
    let counter = (*s).private as *mut ni_gpct;
    dev_err((*(*counter).counter_dev).dev.class_dev, "output commands not yet implemented.\n");
    -ENOTSUPP
}

unsafe fn ni_tio_cmd_setup(s: *mut comedi_subdevice) -> i32 {
    let cmd = &mut (*(*s).async_).cmd;
    let counter = (*s).private as *mut ni_gpct;
    let cidx = (*counter).counter_index;
    let routing_tables = (*(*counter).counter_dev).routing_tables;
    let mut set_gate_source = false;
    let mut gate_source = 0;
    if cmd.scan_begin_src == TRIG_EXT { set_gate_source = true; gate_source = cmd.scan_begin_arg; }
    else if cmd.convert_src == TRIG_EXT { set_gate_source = true; gate_source = cmd.convert_arg; }
    let mut retval = 0;
    if set_gate_source {
        if CR_CHAN(gate_source) >= NI_NAMES_BASE {
            let reg = ni_get_reg_value(CR_CHAN(gate_source), NI_CtrGate(cidx), routing_tables);
            if reg < 0 { return -EINVAL; }
            retval = ni_tio_set_gate_src_raw(counter, 0, reg);
        } else { retval = ni_tio_set_gate_src(counter, 0, gate_source); }
    }
    if cmd.flags & CMDF_WAKE_EOS != 0 { ni_tio_set_bits(counter, NITIO_INT_ENA_REG(cidx), GI_GATE_INTERRUPT_ENABLE(cidx), GI_GATE_INTERRUPT_ENABLE(cidx)); }
    retval
}

pub unsafe fn ni_tio_cmd(_dev: *mut comedi_device, s: *mut comedi_subdevice) -> i32 {
    let counter = (*s).private as *mut ni_gpct;
    let cmd = &(*(*s).async_).cmd;
    let mut retval = 0;
    let mut flags = 0UL;
    spin_lock_irqsave(&mut (*counter).lock, &mut flags);
    if (*counter).mite_chan.is_null() { retval = -EIO; }
    else { retval = ni_tio_cmd_setup(s); if retval == 0 { retval = if cmd.flags & CMDF_WRITE != 0 { ni_tio_output_cmd(s) } else { ni_tio_input_cmd(s) }; } }
    spin_unlock_irqrestore(&mut (*counter).lock, flags);
    retval
}

pub unsafe fn ni_tio_cmdtest(_dev: *mut comedi_device, s: *mut comedi_subdevice, cmd: *mut comedi_cmd) -> i32 {
    let counter = (*s).private as *mut ni_gpct;
    let cidx = (*counter).counter_index;
    let routing_tables = (*(*counter).counter_dev).routing_tables;
    let mut err = 0;
    let mut sources = TRIG_NOW | TRIG_INT | TRIG_OTHER;
    if ni_tio_counting_mode_registers_present((*counter).counter_dev) { sources |= TRIG_EXT; }
    err |= comedi_check_trigger_src(&mut (*cmd).start_src, sources);
    err |= comedi_check_trigger_src(&mut (*cmd).scan_begin_src, TRIG_FOLLOW | TRIG_EXT | TRIG_OTHER);
    err |= comedi_check_trigger_src(&mut (*cmd).convert_src, TRIG_NOW | TRIG_EXT | TRIG_OTHER);
    err |= comedi_check_trigger_src(&mut (*cmd).scan_end_src, TRIG_COUNT);
    err |= comedi_check_trigger_src(&mut (*cmd).stop_src, TRIG_NONE);
    if err != 0 { return 1; }
    err |= comedi_check_trigger_is_unique((*cmd).start_src);
    err |= comedi_check_trigger_is_unique((*cmd).scan_begin_src);
    err |= comedi_check_trigger_is_unique((*cmd).convert_src);
    if (*cmd).convert_src != TRIG_NOW && (*cmd).scan_begin_src != TRIG_FOLLOW { err |= -EINVAL; }
    if err != 0 { return 2; }
    match (*cmd).start_src { TRIG_NOW | TRIG_INT | TRIG_OTHER => err |= comedi_check_trigger_arg_is(&mut (*cmd).start_arg, 0), _ => {} }
    if (*cmd).scan_begin_src != TRIG_EXT { err |= comedi_check_trigger_arg_is(&mut (*cmd).scan_begin_arg, 0); } else { err |= ni_check_trigger_arg(CR_CHAN((*cmd).scan_begin_arg), NI_CtrGate(cidx), routing_tables); }
    if (*cmd).convert_src != TRIG_EXT { err |= comedi_check_trigger_arg_is(&mut (*cmd).convert_arg, 0); } else { err |= ni_check_trigger_arg(CR_CHAN((*cmd).convert_arg), NI_CtrGate(cidx), routing_tables); }
    err |= comedi_check_trigger_arg_is(&mut (*cmd).scan_end_arg, (*cmd).chanlist_len);
    err |= comedi_check_trigger_arg_is(&mut (*cmd).stop_arg, 0);
    if err != 0 { return 3; }
    0
}

pub unsafe fn ni_tio_cancel(counter: *mut ni_gpct) -> i32 {
    let cidx = (*counter).counter_index;
    ni_tio_arm(counter, false, 0);
    let mut flags = 0UL; spin_lock_irqsave(&mut (*counter).lock, &mut flags);
    if !(*counter).mite_chan.is_null() { mite_dma_disarm((*counter).mite_chan); }
    spin_unlock_irqrestore(&mut (*counter).lock, flags);
    ni_tio_configure_dma(counter, false, false);
    ni_tio_set_bits(counter, NITIO_INT_ENA_REG(cidx), GI_GATE_INTERRUPT_ENABLE(cidx), 0);
    0
}

unsafe fn should_ack_gate(counter: *mut ni_gpct) -> i32 {
    match (*(*counter).counter_dev).variant {
        ni_gpct_variant_m_series | ni_gpct_variant_660x => 1,
        ni_gpct_variant_e_series => { let mut flags=0UL; let mut ret=0; spin_lock_irqsave(&mut (*counter).lock,&mut flags); if (*counter).mite_chan.is_null() || (*(*counter).mite_chan).dir != COMEDI_INPUT || mite_done((*counter).mite_chan) { ret=1; } spin_unlock_irqrestore(&mut (*counter).lock,flags); ret },
        _ => 0,
    }
}

unsafe fn ni_tio_acknowledge_and_confirm(counter: *mut ni_gpct, gate_error: *mut i32, tc_error: *mut i32, perm_stale_data: *mut i32) {
    let cidx=(*counter).counter_index; let gxx_status=ni_tio_read(counter,NITIO_SHARED_STATUS_REG(cidx)); let gi_status=ni_tio_read(counter,NITIO_STATUS_REG(cidx)); let mut ack=0;
    if !gate_error.is_null(){*gate_error=0;} if !tc_error.is_null(){*tc_error=0;} if !perm_stale_data.is_null(){*perm_stale_data=0;}
    if gxx_status & GI_GATE_ERROR(cidx)!=0 { ack|=GI_GATE_ERROR_CONFIRM(cidx); if !gate_error.is_null() && (*(*counter).counter_dev).variant!=ni_gpct_variant_660x {*gate_error=1;} }
    if gxx_status & GI_TC_ERROR(cidx)!=0 { ack|=GI_TC_ERROR_CONFIRM(cidx); if !tc_error.is_null(){*tc_error=1;} }
    if gi_status & GI_TC != 0 { ack|=GI_TC_INTERRUPT_ACK; } if gi_status & GI_GATE_INTERRUPT != 0 && should_ack_gate(counter)!=0 { ack|=GI_GATE_INTERRUPT_ACK; }
    if ack!=0 { ni_tio_write(counter,ack,NITIO_INT_ACK_REG(cidx)); }
    if ni_tio_get_soft_copy(counter,NITIO_MODE_REG(cidx)) & GI_LOADING_ON_GATE != 0 && ni_tio_read(counter,NITIO_STATUS2_REG(cidx)) & GI_PERMANENT_STALE(cidx)!=0 { if !perm_stale_data.is_null(){*perm_stale_data=1;} }
}

pub unsafe fn ni_tio_acknowledge(counter:*mut ni_gpct){ni_tio_acknowledge_and_confirm(counter,std::ptr::null_mut(),std::ptr::null_mut(),std::ptr::null_mut());}
pub unsafe fn ni_tio_handle_interrupt(counter:*mut ni_gpct,s:*mut comedi_subdevice){let cidx=(*counter).counter_index;let mut ge=0;let mut te=0;let mut ps=0;ni_tio_acknowledge_and_confirm(counter,&mut ge,&mut te,&mut ps);if ge!=0{(*(*s).async_).events|=COMEDI_CB_OVERFLOW;}if ps!=0{(*(*s).async_).events|=COMEDI_CB_ERROR;}match (*(*counter).counter_dev).variant{ni_gpct_variant_m_series|ni_gpct_variant_660x=>if ni_tio_read(counter,NITIO_DMA_STATUS_REG(cidx))&GI_DRQ_ERROR!=0{(*(*s).async_).events|=COMEDI_CB_OVERFLOW;},_=>{}}let mut flags=0UL;spin_lock_irqsave(&mut (*counter).lock,&mut flags);if !(*counter).mite_chan.is_null(){mite_ack_linkc((*counter).mite_chan,s,true);}spin_unlock_irqrestore(&mut (*counter).lock,flags);}
pub unsafe fn ni_tio_set_mite_channel(counter:*mut ni_gpct,mite_chan:*mut mite_channel){let mut flags=0UL;spin_lock_irqsave(&mut (*counter).lock,&mut flags);(*counter).mite_chan=mite_chan;spin_unlock_irqrestore(&mut (*counter).lock,flags);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
