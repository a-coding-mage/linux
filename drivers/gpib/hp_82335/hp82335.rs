// SPDX-License-Identifier: GPL-2.0

/***************************************************************************
 * copyright            : (C) 2002 by Frank Mori Hess                      *
 ***************************************************************************/

/*
 * should enable ATN interrupts (and update board->status on occurrence),
 * implement recovery from bus errors (if necessary)
 */

// C preprocessor configuration and Linux/GPIB includes are supplied by the
// surrounding kernel build; the referenced symbols remain external here.

// MODULE_LICENSE!("GPL");
// MODULE_DESCRIPTION!("GPIB driver for HP 82335 interface cards");

static mut HP82335_INTERFACE: gpib_interface = gpib_interface {
    name: "hp82335",
    attach: Some(hp82335_attach),
    detach: Some(hp82335_detach),
    read: Some(hp82335_read),
    write: Some(hp82335_write),
    command: Some(hp82335_command),
    request_system_control: Some(hp82335_request_system_control),
    take_control: Some(hp82335_take_control),
    go_to_standby: Some(hp82335_go_to_standby),
    interface_clear: Some(hp82335_interface_clear),
    remote_enable: Some(hp82335_remote_enable),
    enable_eos: Some(hp82335_enable_eos),
    disable_eos: Some(hp82335_disable_eos),
    parallel_poll: Some(hp82335_parallel_poll),
    parallel_poll_configure: Some(hp82335_parallel_poll_configure),
    parallel_poll_response: Some(hp82335_parallel_poll_response),
    local_parallel_poll_mode: None, // XXX
    line_status: Some(hp82335_line_status),
    update_status: Some(hp82335_update_status),
    primary_address: Some(hp82335_primary_address),
    secondary_address: Some(hp82335_secondary_address),
    serial_poll_response: Some(hp82335_serial_poll_response),
    serial_poll_status: Some(hp82335_serial_poll_status),
    t1_delay: Some(hp82335_t1_delay),
    return_to_local: Some(hp82335_return_to_local),
};

unsafe fn hp82335_read(board: *mut gpib_board, buffer: *mut u8, length: usize,
                       end: *mut i32, bytes_read: *mut usize) -> i32 {
    let priv_data = (*board).private_data as *mut hp82335_priv;
    tms9914_read(board, &mut (*priv_data).tms9914_priv, buffer, length, end, bytes_read)
}

unsafe fn hp82335_write(board: *mut gpib_board, buffer: *mut u8, length: usize,
                        send_eoi: i32, bytes_written: *mut usize) -> i32 {
    let priv_data = (*board).private_data as *mut hp82335_priv;
    tms9914_write(board, &mut (*priv_data).tms9914_priv, buffer, length, send_eoi, bytes_written)
}

unsafe fn hp82335_command(board: *mut gpib_board, buffer: *mut u8, length: usize,
                          bytes_written: *mut usize) -> i32 {
    let priv_data = (*board).private_data as *mut hp82335_priv;
    tms9914_command(board, &mut (*priv_data).tms9914_priv, buffer, length, bytes_written)
}

unsafe fn hp82335_take_control(board: *mut gpib_board, synchronous: i32) -> i32 {
    let priv_data = (*board).private_data as *mut hp82335_priv;
    tms9914_take_control(board, &mut (*priv_data).tms9914_priv, synchronous)
}

unsafe fn hp82335_go_to_standby(board: *mut gpib_board) -> i32 {
    let priv_data = (*board).private_data as *mut hp82335_priv;
    tms9914_go_to_standby(board, &mut (*priv_data).tms9914_priv)
}

unsafe fn hp82335_request_system_control(board: *mut gpib_board, request_control: i32) -> i32 {
    let priv_data = (*board).private_data as *mut hp82335_priv;
    tms9914_request_system_control(board, &mut (*priv_data).tms9914_priv, request_control)
}

unsafe fn hp82335_interface_clear(board: *mut gpib_board, assert_: i32) {
    let priv_data = (*board).private_data as *mut hp82335_priv;
    tms9914_interface_clear(board, &mut (*priv_data).tms9914_priv, assert_);
}

unsafe fn hp82335_remote_enable(board: *mut gpib_board, enable: i32) {
    let priv_data = (*board).private_data as *mut hp82335_priv;
    tms9914_remote_enable(board, &mut (*priv_data).tms9914_priv, enable);
}

unsafe fn hp82335_enable_eos(board: *mut gpib_board, eos_byte: u8, compare_8_bits: i32) -> i32 {
    let priv_data = (*board).private_data as *mut hp82335_priv;
    tms9914_enable_eos(board, &mut (*priv_data).tms9914_priv, eos_byte, compare_8_bits)
}

unsafe fn hp82335_disable_eos(board: *mut gpib_board) {
    let priv_data = (*board).private_data as *mut hp82335_priv;
    tms9914_disable_eos(board, &mut (*priv_data).tms9914_priv);
}

unsafe fn hp82335_update_status(board: *mut gpib_board, clear_mask: u32) -> u32 {
    let priv_data = (*board).private_data as *mut hp82335_priv;
    tms9914_update_status(board, &mut (*priv_data).tms9914_priv, clear_mask)
}

unsafe fn hp82335_primary_address(board: *mut gpib_board, address: u32) -> i32 {
    let priv_data = (*board).private_data as *mut hp82335_priv;
    tms9914_primary_address(board, &mut (*priv_data).tms9914_priv, address)
}

unsafe fn hp82335_secondary_address(board: *mut gpib_board, address: u32, enable: i32) -> i32 {
    let priv_data = (*board).private_data as *mut hp82335_priv;
    tms9914_secondary_address(board, &mut (*priv_data).tms9914_priv, address, enable)
}

unsafe fn hp82335_parallel_poll(board: *mut gpib_board, result: *mut u8) -> i32 {
    let priv_data = (*board).private_data as *mut hp82335_priv;
    tms9914_parallel_poll(board, &mut (*priv_data).tms9914_priv, result)
}

unsafe fn hp82335_parallel_poll_configure(board: *mut gpib_board, config: u8) {
    let priv_data = (*board).private_data as *mut hp82335_priv;
    tms9914_parallel_poll_configure(board, &mut (*priv_data).tms9914_priv, config);
}

unsafe fn hp82335_parallel_poll_response(board: *mut gpib_board, ist: i32) {
    let priv_data = (*board).private_data as *mut hp82335_priv;
    tms9914_parallel_poll_response(board, &mut (*priv_data).tms9914_priv, ist);
}

unsafe fn hp82335_serial_poll_response(board: *mut gpib_board, status: u8) {
    let priv_data = (*board).private_data as *mut hp82335_priv;
    tms9914_serial_poll_response(board, &mut (*priv_data).tms9914_priv, status);
}

unsafe fn hp82335_serial_poll_status(board: *mut gpib_board) -> u8 {
    let priv_data = (*board).private_data as *mut hp82335_priv;
    tms9914_serial_poll_status(board, &mut (*priv_data).tms9914_priv)
}

unsafe fn hp82335_line_status(board: *const gpib_board) -> i32 {
    let priv_data = (*board).private_data as *mut hp82335_priv;
    tms9914_line_status(board, &mut (*priv_data).tms9914_priv)
}

unsafe fn hp82335_t1_delay(board: *mut gpib_board, nano_sec: u32) -> i32 {
    let priv_data = (*board).private_data as *mut hp82335_priv;
    tms9914_t1_delay(board, &mut (*priv_data).tms9914_priv, nano_sec)
}

unsafe fn hp82335_return_to_local(board: *mut gpib_board) {
    let priv_data = (*board).private_data as *mut hp82335_priv;
    tms9914_return_to_local(board, &mut (*priv_data).tms9914_priv);
}

unsafe fn hp82335_allocate_private(board: *mut gpib_board) -> i32 {
    (*board).private_data = kzalloc_obj::<hp82335_priv>();
    if (*board).private_data.is_null() { return -ENOMEM; }
    0
}

unsafe fn hp82335_free_private(board: *mut gpib_board) {
    kfree((*board).private_data);
    (*board).private_data = core::ptr::null_mut();
}

#[inline]
fn tms9914_to_hp82335_offset(register_num: u32) -> u32 { 0x1ff8u32.wrapping_add(register_num) }

unsafe fn hp82335_read_byte(priv_: *mut tms9914_priv, register_num: u32) -> u8 {
    tms9914_iomem_read_byte(priv_, tms9914_to_hp82335_offset(register_num))
}

unsafe fn hp82335_write_byte(priv_: *mut tms9914_priv, data: u8, register_num: u32) {
    tms9914_iomem_write_byte(priv_, data, tms9914_to_hp82335_offset(register_num));
}

unsafe fn hp82335_clear_interrupt(hp_priv: *mut hp82335_priv) {
    let tms_priv = &mut (*hp_priv).tms9914_priv;
    writeb(0, tms_priv.mmiobase.add(HPREG_INTR_CLEAR as usize));
}

unsafe fn hp82335_attach(board: *mut gpib_board, config: *const gpib_board_config) -> i32 {
    let upper_iomem_base = (*config).ibbase + hp82335_rom_size;
    (*board).status = 0;
    let retval = hp82335_allocate_private(board);
    if retval != 0 { return retval; }
    let hp_priv = (*board).private_data as *mut hp82335_priv;
    let tms_priv = &mut (*hp_priv).tms9914_priv;
    tms_priv.read_byte = Some(hp82335_read_byte);
    tms_priv.write_byte = Some(hp82335_write_byte);
    tms_priv.offset = 1;
    match (*config).ibbase {
        0xc4000 | 0xc8000 | 0xcc000 | 0xd0000 | 0xd4000 | 0xd8000 | 0xdc000 |
        0xe0000 | 0xe4000 | 0xe8000 | 0xec000 | 0xf0000 | 0xf4000 | 0xf8000 | 0xfc000 => {}
        _ => { dev_err((*board).gpib_dev, "invalid base io address 0x%x\n", (*config).ibbase); return -EINVAL; }
    }
    if !request_mem_region(upper_iomem_base, hp82335_upper_iomem_size, "hp82335") {
        dev_err((*board).gpib_dev, "failed to allocate io memory region 0x%lx-0x%lx\n",
                upper_iomem_base, upper_iomem_base + hp82335_upper_iomem_size - 1);
        return -EBUSY;
    }
    (*hp_priv).raw_iobase = upper_iomem_base;
    tms_priv.mmiobase = ioremap(upper_iomem_base, hp82335_upper_iomem_size);
    let retval = request_irq((*config).ibirq, hp82335_interrupt, 0, DRV_NAME, board);
    if retval != 0 { dev_err((*board).gpib_dev, "can't request IRQ %d\n", (*config).ibirq); return retval; }
    (*hp_priv).irq = (*config).ibirq;
    tms9914_board_reset(tms_priv);
    hp82335_clear_interrupt(hp_priv);
    writeb(INTR_ENABLE, tms_priv.mmiobase.add(HPREG_CCR as usize));
    tms9914_online(board, tms_priv);
    0
}

unsafe fn hp82335_detach(board: *mut gpib_board) {
    let hp_priv = (*board).private_data as *mut hp82335_priv;
    if !hp_priv.is_null() {
        let tms_priv = &mut (*hp_priv).tms9914_priv;
        if (*hp_priv).irq != 0 { free_irq((*hp_priv).irq, board); }
        if !tms_priv.mmiobase.is_null() { writeb(0, tms_priv.mmiobase.add(HPREG_CCR as usize)); tms9914_board_reset(tms_priv); iounmap(tms_priv.mmiobase); }
        if (*hp_priv).raw_iobase != 0 { release_mem_region((*hp_priv).raw_iobase, hp82335_upper_iomem_size); }
    }
    hp82335_free_private(board);
}

unsafe fn hp82335_init_module() -> i32 { gpib_register_driver(&mut HP82335_INTERFACE, THIS_MODULE) }
unsafe fn hp82335_exit_module() { gpib_unregister_driver(&mut HP82335_INTERFACE); }

/* GPIB interrupt service routines */
unsafe fn hp82335_interrupt(_irq: i32, arg: *mut core::ffi::c_void) -> irqreturn_t {
    let board = arg as *mut gpib_board;
    let priv_ = (*board).private_data as *mut hp82335_priv;
    let status1 = read_byte(&mut (*priv_).tms9914_priv, ISR0);
    let status2 = read_byte(&mut (*priv_).tms9914_priv, ISR1);
    hp82335_clear_interrupt(priv_);
    tms9914_interrupt_have_status(board, &mut (*priv_).tms9914_priv, status1, status2)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
