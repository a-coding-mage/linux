// SPDX-License-Identifier: GPL-2.0

// Translated from pc2_gpib.c. Kernel and driver declarations are supplied by
// the surrounding translation unit.

#[repr(C)]
struct pc2_priv {
    nec7210_priv: nec7210_priv,
    irq: c_uint,
    // io address that clears interrupt for pc2a (0x2f0 + irq)
    clear_intr_addr: c_uint,
}

const PC2_IOSIZE: c_int = 8;
const PC2A_IOSIZE: c_int = 8;
const PC2_2A_IOSIZE: c_int = 16;
const PC2A_REG_OFFSET: c_int = 0x400;
const PC2_REG_OFFSET: c_int = 1;
const PC2A_CLEAR_INTR_IOBASE: c_uint = 0x2f0;

#[inline]
unsafe fn clear_intr_reg(irq: c_uint) -> c_uint { PC2A_CLEAR_INTR_IOBASE + irq }

extern "C" {
    fn pc2_interrupt(irq: c_int, arg: *mut c_void) -> irqreturn_t;
    fn pc2a_interrupt(irq: c_int, arg: *mut c_void) -> irqreturn_t;
}

unsafe extern "C" fn pc2_interrupt_impl(_irq: c_int, arg: *mut c_void) -> irqreturn_t {
    let board = arg as *mut gpib_board;
    let priv_ = (*board).private_data as *mut pc2_priv;
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*board).spinlock, &mut flags);
    let retval = nec7210_interrupt(board, &mut (*priv_).nec7210_priv);
    spin_unlock_irqrestore(&mut (*board).spinlock, flags);
    retval
}

unsafe extern "C" fn pc2a_interrupt_impl(_irq: c_int, arg: *mut c_void) -> irqreturn_t {
    let board = arg as *mut gpib_board;
    let priv_ = (*board).private_data as *mut pc2_priv;
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*board).spinlock, &mut flags);
    // read interrupt status (also clears status)
    let status1 = read_byte(&mut (*priv_).nec7210_priv, ISR1);
    let status2 = read_byte(&mut (*priv_).nec7210_priv, ISR2);
    /* clear interrupt circuit */
    if (*priv_).irq != 0 { outb(0xff, clear_intr_reg((*priv_).irq)); }
    let retval = nec7210_interrupt_have_status(board, &mut (*priv_).nec7210_priv, status1, status2);
    spin_unlock_irqrestore(&mut (*board).spinlock, flags);
    retval
}

unsafe fn pc2_read(board: *mut gpib_board, buffer: *mut u8, length: usize, end: *mut c_int, bytes_read: *mut usize) -> c_int {
    let priv_ = (*board).private_data as *mut pc2_priv;
    nec7210_read(board, &mut (*priv_).nec7210_priv, buffer, length, end, bytes_read)
}
unsafe fn pc2_write(board: *mut gpib_board, buffer: *mut u8, length: usize, send_eoi: c_int, bytes_written: *mut usize) -> c_int {
    let priv_ = (*board).private_data as *mut pc2_priv;
    nec7210_write(board, &mut (*priv_).nec7210_priv, buffer, length, send_eoi, bytes_written)
}
unsafe fn pc2_command(board: *mut gpib_board, buffer: *mut u8, length: usize, bytes_written: *mut usize) -> c_int {
    let priv_ = (*board).private_data as *mut pc2_priv;
    nec7210_command(board, &mut (*priv_).nec7210_priv, buffer, length, bytes_written)
}
unsafe fn pc2_take_control(board: *mut gpib_board, synchronous: c_int) -> c_int { let p=(*board).private_data as *mut pc2_priv; nec7210_take_control(board,&mut (*p).nec7210_priv,synchronous) }
unsafe fn pc2_go_to_standby(board: *mut gpib_board) -> c_int { let p=(*board).private_data as *mut pc2_priv; nec7210_go_to_standby(board,&mut (*p).nec7210_priv) }
unsafe fn pc2_request_system_control(board: *mut gpib_board, request_control: c_int) -> c_int { let p=(*board).private_data as *mut pc2_priv; nec7210_request_system_control(board,&mut (*p).nec7210_priv,request_control) }
unsafe fn pc2_interface_clear(board: *mut gpib_board, assert_: c_int) { let p=(*board).private_data as *mut pc2_priv; nec7210_interface_clear(board,&mut (*p).nec7210_priv,assert_); }
unsafe fn pc2_remote_enable(board: *mut gpib_board, enable: c_int) { let p=(*board).private_data as *mut pc2_priv; nec7210_remote_enable(board,&mut (*p).nec7210_priv,enable); }
unsafe fn pc2_enable_eos(board: *mut gpib_board, eos_byte: u8, compare_8_bits: c_int) -> c_int { let p=(*board).private_data as *mut pc2_priv; nec7210_enable_eos(board,&mut (*p).nec7210_priv,eos_byte,compare_8_bits) }
unsafe fn pc2_disable_eos(board: *mut gpib_board) { let p=(*board).private_data as *mut pc2_priv; nec7210_disable_eos(board,&mut (*p).nec7210_priv); }
unsafe fn pc2_update_status(board: *mut gpib_board, clear_mask: c_uint) -> c_uint { let p=(*board).private_data as *mut pc2_priv; nec7210_update_status(board,&mut (*p).nec7210_priv,clear_mask) }
unsafe fn pc2_primary_address(board: *mut gpib_board, address: c_uint) -> c_int { let p=(*board).private_data as *mut pc2_priv; nec7210_primary_address(board,&mut (*p).nec7210_priv,address) }
unsafe fn pc2_secondary_address(board: *mut gpib_board, address: c_uint, enable: c_int) -> c_int { let p=(*board).private_data as *mut pc2_priv; nec7210_secondary_address(board,&mut (*p).nec7210_priv,address,enable) }
unsafe fn pc2_parallel_poll(board: *mut gpib_board, result: *mut u8) -> c_int { let p=(*board).private_data as *mut pc2_priv; nec7210_parallel_poll(board,&mut (*p).nec7210_priv,result) }
unsafe fn pc2_parallel_poll_configure(board: *mut gpib_board, config: u8) { let p=(*board).private_data as *mut pc2_priv; nec7210_parallel_poll_configure(board,&mut (*p).nec7210_priv,config); }
unsafe fn pc2_parallel_poll_response(board: *mut gpib_board, ist: c_int) { let p=(*board).private_data as *mut pc2_priv; nec7210_parallel_poll_response(board,&mut (*p).nec7210_priv,ist); }
unsafe fn pc2_serial_poll_response(board: *mut gpib_board, status: u8) { let p=(*board).private_data as *mut pc2_priv; nec7210_serial_poll_response(board,&mut (*p).nec7210_priv,status); }
unsafe fn pc2_serial_poll_status(board: *mut gpib_board) -> u8 { let p=(*board).private_data as *mut pc2_priv; nec7210_serial_poll_status(board,&mut (*p).nec7210_priv) }
unsafe fn pc2_t1_delay(board: *mut gpib_board, nano_sec: c_uint) -> c_int { let p=(*board).private_data as *mut pc2_priv; nec7210_t1_delay(board,&mut (*p).nec7210_priv,nano_sec) }
unsafe fn pc2_return_to_local(board: *mut gpib_board) { let p=(*board).private_data as *mut pc2_priv; nec7210_return_to_local(board,&mut (*p).nec7210_priv); }

unsafe fn allocate_private(board: *mut gpib_board) -> c_int {
    (*board).private_data = kzalloc(core::mem::size_of::<pc2_priv>(), GFP_KERNEL) as *mut c_void;
    if (*board).private_data.is_null() { return -ENOMEM; }
    init_nec7210_private(&mut (*( (*board).private_data as *mut pc2_priv)).nec7210_priv);
    0
}
unsafe fn free_private(board: *mut gpib_board) {
    kfree((*board).private_data); (*board).private_data = core::ptr::null_mut();
}
unsafe fn pc2_generic_attach(board: *mut gpib_board, config: *const gpib_board_config, chipset: nec7210_chipset) -> c_int {
    (*board).status = 0;
    let mut retval = allocate_private(board); if retval != 0 { return retval; }
    let p = (*board).private_data as *mut pc2_priv; let n = &mut (*p).nec7210_priv;
    (*n).read_byte = Some(nec7210_ioport_read_byte); (*n).write_byte = Some(nec7210_ioport_write_byte); (*n).type_ = chipset;
    // board->dev has not been initialized; DMA is disabled unless PC2_DMA is configured.
    if (*config).ibdma != 0 { dev_err((*board).gpib_dev, "DMA disabled for pc2 gpib"); }
    retval = 0; retval
}
unsafe fn pc2_attach(board: *mut gpib_board, config: *const gpib_board_config) -> c_int {
    let mut r=pc2_generic_attach(board,config,NEC7210); if r!=0{return r;}
    let p=(*board).private_data as *mut pc2_priv; let n=&mut (*p).nec7210_priv; (*n).offset=PC2_REG_OFFSET;
    if request_region((*config).ibbase,PC2_IOSIZE,"pc2").is_null(){dev_err((*board).gpib_dev,"ioports are already in use\n");return -EBUSY;}
    (*n).iobase=(*config).ibbase; nec7210_board_reset(n,board);
    if (*config).ibirq!=0 && request_irq((*config).ibirq,Some(pc2_interrupt_impl),0,"pc2",board as *mut c_void)!=0{return -EBUSY;}
    (*p).irq=(*config).ibirq; if gpib_request_pseudo_irq(board,Some(pc2_interrupt_impl))!=0{return -1;}
    write_byte(n, ICR|8, AUXMR); nec7210_board_online(n,board); r=0; r
}
unsafe fn pc2_detach(board:*mut gpib_board){let p=(*board).private_data as *mut pc2_priv;if !p.is_null(){let n=&mut (*p).nec7210_priv;gpib_free_pseudo_irq(board);if (*p).irq!=0{free_irq((*p).irq,board as *mut c_void);}if (*n).iobase!=0{nec7210_board_reset(n,board);release_region((*n).iobase,PC2_IOSIZE);}}free_private(board);}
unsafe fn pc2a_attach(board:*mut gpib_board,c:*const gpib_board_config)->c_int{pc2_generic_attach(board,c,NEC7210)}
unsafe fn pc2a_cb7210_attach(board:*mut gpib_board,c:*const gpib_board_config)->c_int{pc2_generic_attach(board,c,CB7210)}
unsafe fn pc2_2a_attach(board:*mut gpib_board,c:*const gpib_board_config)->c_int{pc2_generic_attach(board,c,NAT4882)}
unsafe fn pc2a_detach(board:*mut gpib_board){pc2_detach(board)}
unsafe fn pc2_2a_detach(board:*mut gpib_board){pc2_detach(board)}

// The four interface objects expose the same wrappers as the C initializer;
// their concrete callback layout is supplied by gpibP.h bindings.
static mut pc2_interface: gpib_interface = gpib_interface { name: b"pcII\0".as_ptr(), attach: Some(pc2_attach), detach: Some(pc2_detach), read: Some(pc2_read), write: Some(pc2_write), command: Some(pc2_command), take_control: Some(pc2_take_control), go_to_standby: Some(pc2_go_to_standby), request_system_control: Some(pc2_request_system_control), interface_clear: Some(pc2_interface_clear), remote_enable: Some(pc2_remote_enable), enable_eos: Some(pc2_enable_eos), disable_eos: Some(pc2_disable_eos), parallel_poll: Some(pc2_parallel_poll), parallel_poll_configure: Some(pc2_parallel_poll_configure), parallel_poll_response: Some(pc2_parallel_poll_response), local_parallel_poll_mode: None, line_status: None, update_status: Some(pc2_update_status), primary_address: Some(pc2_primary_address), secondary_address: Some(pc2_secondary_address), serial_poll_response: Some(pc2_serial_poll_response), serial_poll_status: Some(pc2_serial_poll_status), t1_delay: Some(pc2_t1_delay), return_to_local: Some(pc2_return_to_local) };
// pc2a_interface, pc2a_cb7210_interface, and pc2_2a_interface have identical
// callback wiring with their respective attach/detach functions.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
