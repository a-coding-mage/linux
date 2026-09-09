// SPDX-License-Identifier: GPL-2.0

/***************************************************************************
 *   copyright            : (C) 2002 by Frank Mori Hess
 ***************************************************************************/

// pr_fmt(fmt) expands to KBUILD_MODNAME ": " fmt
// dev_fmt expands to pr_fmt
// DRV_NAME expands to KBUILD_MODNAME

// GPIB interrupt service routines

unsafe extern "C" fn cec_interrupt(irq: i32, arg: *mut core::ffi::c_void) -> irqreturn_t {
	let board = arg as *mut gpib_board;
	let priv_data = (*board).private_data as *mut cec_priv;
	let mut flags: c_ulong = 0;
	let retval: irqreturn_t;

	spin_lock_irqsave(&mut (*board).spinlock, &mut flags);
	retval = nec7210_interrupt(board, &mut (*priv_data).nec7210_priv);
	spin_unlock_irqrestore(&mut (*board).spinlock, flags);
	retval
}

const CEC_VENDOR_ID: u16 = 0x12fc;
const CEC_DEV_ID: u16 = 0x5cec;
const CEC_SUBID: u16 = 0x9050;

unsafe extern "C" fn cec_pci_attach(
	board: *mut gpib_board,
	config: *const gpib_board_config,
) -> i32;

unsafe extern "C" fn cec_pci_detach(board: *mut gpib_board);

// wrappers for interface functions
unsafe extern "C" fn cec_read(
	board: *mut gpib_board,
	buffer: *mut u8,
	length: usize,
	end: *mut i32,
	bytes_read: *mut usize,
) -> i32 {
	let priv_data = (*board).private_data as *mut cec_priv;
	nec7210_read(board, &mut (*priv_data).nec7210_priv, buffer, length, end, bytes_read)
}

unsafe extern "C" fn cec_write(
	board: *mut gpib_board,
	buffer: *mut u8,
	length: usize,
	send_eoi: i32,
	bytes_written: *mut usize,
) -> i32 {
	let priv_data = (*board).private_data as *mut cec_priv;
	nec7210_write(board, &mut (*priv_data).nec7210_priv, buffer, length, send_eoi, bytes_written)
}

unsafe extern "C" fn cec_command(
	board: *mut gpib_board,
	buffer: *mut u8,
	length: usize,
	bytes_written: *mut usize,
) -> i32 {
	let priv_data = (*board).private_data as *mut cec_priv;
	nec7210_command(board, &mut (*priv_data).nec7210_priv, buffer, length, bytes_written)
}

unsafe extern "C" fn cec_take_control(board: *mut gpib_board, synchronous: i32) -> i32 {
	let priv_data = (*board).private_data as *mut cec_priv;
	nec7210_take_control(board, &mut (*priv_data).nec7210_priv, synchronous)
}

unsafe extern "C" fn cec_go_to_standby(board: *mut gpib_board) -> i32 {
	let priv_data = (*board).private_data as *mut cec_priv;
	nec7210_go_to_standby(board, &mut (*priv_data).nec7210_priv)
}

unsafe extern "C" fn cec_request_system_control(board: *mut gpib_board, request_control: i32) -> i32 {
	let priv_data = (*board).private_data as *mut cec_priv;
	nec7210_request_system_control(board, &mut (*priv_data).nec7210_priv, request_control)
}

unsafe extern "C" fn cec_interface_clear(board: *mut gpib_board, assert_: i32) {
	let priv_data = (*board).private_data as *mut cec_priv;
	nec7210_interface_clear(board, &mut (*priv_data).nec7210_priv, assert_);
}

unsafe extern "C" fn cec_remote_enable(board: *mut gpib_board, enable: i32) {
	let priv_data = (*board).private_data as *mut cec_priv;
	nec7210_remote_enable(board, &mut (*priv_data).nec7210_priv, enable);
}

unsafe extern "C" fn cec_enable_eos(board: *mut gpib_board, eos_byte: u8, compare_8_bits: i32) -> i32 {
	let priv_data = (*board).private_data as *mut cec_priv;
	nec7210_enable_eos(board, &mut (*priv_data).nec7210_priv, eos_byte, compare_8_bits)
}

unsafe extern "C" fn cec_disable_eos(board: *mut gpib_board) {
	let priv_data = (*board).private_data as *mut cec_priv;
	nec7210_disable_eos(board, &mut (*priv_data).nec7210_priv);
}

unsafe extern "C" fn cec_update_status(board: *mut gpib_board, clear_mask: u32) -> u32 {
	let priv_data = (*board).private_data as *mut cec_priv;
	nec7210_update_status(board, &mut (*priv_data).nec7210_priv, clear_mask)
}

unsafe extern "C" fn cec_primary_address(board: *mut gpib_board, address: u32) -> i32 {
	let priv_data = (*board).private_data as *mut cec_priv;
	nec7210_primary_address(board, &mut (*priv_data).nec7210_priv, address)
}

unsafe extern "C" fn cec_secondary_address(board: *mut gpib_board, address: u32, enable: i32) -> i32 {
	let priv_data = (*board).private_data as *mut cec_priv;
	nec7210_secondary_address(board, &mut (*priv_data).nec7210_priv, address, enable)
}

unsafe extern "C" fn cec_parallel_poll(board: *mut gpib_board, result: *mut u8) -> i32 {
	let priv_data = (*board).private_data as *mut cec_priv;
	nec7210_parallel_poll(board, &mut (*priv_data).nec7210_priv, result)
}

unsafe extern "C" fn cec_parallel_poll_configure(board: *mut gpib_board, config: u8) {
	let priv_data = (*board).private_data as *mut cec_priv;
	nec7210_parallel_poll_configure(board, &mut (*priv_data).nec7210_priv, config);
}

unsafe extern "C" fn cec_parallel_poll_response(board: *mut gpib_board, ist: i32) {
	let priv_data = (*board).private_data as *mut cec_priv;
	nec7210_parallel_poll_response(board, &mut (*priv_data).nec7210_priv, ist);
}

unsafe extern "C" fn cec_serial_poll_response(board: *mut gpib_board, status: u8) {
	let priv_data = (*board).private_data as *mut cec_priv;
	nec7210_serial_poll_response(board, &mut (*priv_data).nec7210_priv, status);
}

unsafe extern "C" fn cec_serial_poll_status(board: *mut gpib_board) -> u8 {
	let priv_data = (*board).private_data as *mut cec_priv;
	nec7210_serial_poll_status(board, &mut (*priv_data).nec7210_priv)
}

unsafe extern "C" fn cec_t1_delay(board: *mut gpib_board, nano_sec: u32) -> i32 {
	let priv_data = (*board).private_data as *mut cec_priv;
	nec7210_t1_delay(board, &mut (*priv_data).nec7210_priv, nano_sec)
}

unsafe extern "C" fn cec_return_to_local(board: *mut gpib_board) {
	let priv_data = (*board).private_data as *mut cec_priv;
	nec7210_return_to_local(board, &mut (*priv_data).nec7210_priv);
}

static mut cec_pci_interface: gpib_interface = gpib_interface {
	name: "cec_pci",
	attach: Some(cec_pci_attach),
	detach: Some(cec_pci_detach),
	read: Some(cec_read),
	write: Some(cec_write),
	command: Some(cec_command),
	take_control: Some(cec_take_control),
	go_to_standby: Some(cec_go_to_standby),
	request_system_control: Some(cec_request_system_control),
	interface_clear: Some(cec_interface_clear),
	remote_enable: Some(cec_remote_enable),
	enable_eos: Some(cec_enable_eos),
	disable_eos: Some(cec_disable_eos),
	parallel_poll: Some(cec_parallel_poll),
	parallel_poll_configure: Some(cec_parallel_poll_configure),
	parallel_poll_response: Some(cec_parallel_poll_response),
	local_parallel_poll_mode: None, // XXX
	line_status: None, // XXX
	update_status: Some(cec_update_status),
	primary_address: Some(cec_primary_address),
	secondary_address: Some(cec_secondary_address),
	serial_poll_response: Some(cec_serial_poll_response),
	serial_poll_status: Some(cec_serial_poll_status),
	t1_delay: Some(cec_t1_delay),
	return_to_local: Some(cec_return_to_local),
};

unsafe extern "C" fn cec_allocate_private(board: *mut gpib_board) -> i32 {
	let private_data = kzalloc_obj::<cec_priv>();
	(*board).private_data = private_data as *mut core::ffi::c_void;
	if (*board).private_data.is_null() {
		return -ENOMEM;
	}
	let priv_data = (*board).private_data as *mut cec_priv;
	init_nec7210_private(&mut (*priv_data).nec7210_priv);
	0
}

unsafe extern "C" fn cec_free_private(board: *mut gpib_board) {
	kfree((*board).private_data);
	(*board).private_data = core::ptr::null_mut();
}

unsafe extern "C" fn cec_generic_attach(board: *mut gpib_board) -> i32 {
	(*board).status = 0;
	let retval = cec_allocate_private(board);
	if retval != 0 {
		return retval;
	}
	let cec_priv = (*board).private_data as *mut cec_priv;
	let nec_priv = &mut (*cec_priv).nec7210_priv;
	(*nec_priv).read_byte = Some(nec7210_ioport_read_byte);
	(*nec_priv).write_byte = Some(nec7210_ioport_write_byte);
	(*nec_priv).offset = cec_reg_offset;
	(*nec_priv).type_ = NEC7210;
	0
}

unsafe extern "C" fn cec_init(cec_priv: *mut cec_priv, board: *const gpib_board) {
	let nec_priv = &mut (*cec_priv).nec7210_priv;
	nec7210_board_reset(nec_priv, board);
	// set internal counter register for 8 MHz input clock
	write_byte(nec_priv, ICR | 8, AUXMR);
	nec7210_board_online(nec_priv, board);
}

unsafe extern "C" fn cec_pci_attach(board: *mut gpib_board, config: *const gpib_board_config) -> i32 {
	let retval = cec_generic_attach(board);
	if retval != 0 {
		return retval;
	}
	let cec_priv = (*board).private_data as *mut cec_priv;
	let nec_priv = &mut (*cec_priv).nec7210_priv;
	let mut isr_flags = 0;

	(*cec_priv).pci_device = core::ptr::null_mut();
	while {
		(*cec_priv).pci_device = gpib_pci_get_device(config, CEC_VENDOR_ID, CEC_DEV_ID, (*cec_priv).pci_device);
		!(*cec_priv).pci_device.is_null()
	} {
		// check for board with plx9050 controller
		if (*(*cec_priv).pci_device).subsystem_device == CEC_SUBID {
			break;
		}
	}
	if (*cec_priv).pci_device.is_null() {
		dev_err((*board).gpib_dev, "no cec PCI board found\n");
		return -ENODEV;
	}
	if pci_enable_device((*cec_priv).pci_device) != 0 {
		dev_err((*board).gpib_dev, "error enabling pci device\n");
		return -EIO;
	}
	if pci_request_regions((*cec_priv).pci_device, "cec-gpib") != 0 {
		return -EBUSY;
	}
	(*cec_priv).plx_iobase = pci_resource_start((*cec_priv).pci_device, 1);
	(*nec_priv).iobase = pci_resource_start((*cec_priv).pci_device, 3);
	isr_flags |= IRQF_SHARED;
	if request_irq((*(*cec_priv).pci_device).irq, Some(cec_interrupt), isr_flags, DRV_NAME, board) != 0 {
		dev_err((*board).gpib_dev, "failed to obtain IRQ %d\n", (*(*cec_priv).pci_device).irq);
		return -EBUSY;
	}
	(*cec_priv).irq = (*(*cec_priv).pci_device).irq;
	if gpib_request_pseudo_irq(board, Some(cec_interrupt)) != 0 {
		dev_err((*board).gpib_dev, "failed to allocate pseudo irq\n");
		return -1;
	}
	cec_init(cec_priv, board);
	outl(PLX9050_LINTR1_EN_BIT | PLX9050_LINTR1_POLARITY_BIT | PLX9050_PCI_INTR_EN_BIT,
		(*cec_priv).plx_iobase + PLX9050_INTCSR_REG);
	0
}

unsafe extern "C" fn cec_pci_detach(board: *mut gpib_board) {
	let cec_priv = (*board).private_data as *mut cec_priv;
	if !cec_priv.is_null() {
		let nec_priv = &mut (*cec_priv).nec7210_priv;
		gpib_free_pseudo_irq(board);
		if (*cec_priv).irq != 0 {
			// disable plx9050 interrupts
			outl(0, (*cec_priv).plx_iobase + PLX9050_INTCSR_REG);
			free_irq((*cec_priv).irq, board);
		}
		if (*nec_priv).iobase != 0 {
			nec7210_board_reset(nec_priv, board);
			pci_release_regions((*cec_priv).pci_device);
		}
		if !(*cec_priv).pci_device.is_null() {
			pci_dev_put((*cec_priv).pci_device);
		}
	}
	cec_free_private(board);
}

unsafe extern "C" fn cec_pci_probe(_dev: *mut pci_dev, _id: *const pci_device_id) -> i32 {
	0
}

static cec_pci_table: [pci_device_id; 2] = [
	pci_device_sub(CEC_VENDOR_ID, CEC_DEV_ID, PCI_ANY_ID, CEC_SUBID),
	pci_device_id {},
];

static mut cec_pci_driver: pci_driver = pci_driver {
	name: DRV_NAME,
	id_table: cec_pci_table.as_ptr(),
	probe: Some(cec_pci_probe),
};

unsafe extern "C" fn cec_init_module() -> i32 {
	let mut result = pci_register_driver(&mut cec_pci_driver);
	if result != 0 {
		pr_err("pci_register_driver failed: error = %d\n", result);
		return result;
	}
	result = gpib_register_driver(&mut cec_pci_interface, THIS_MODULE);
	if result != 0 {
		pr_err("gpib_register_driver failed: error = %d\n", result);
		return result;
	}
	0
}

unsafe extern "C" fn cec_exit_module() {
	gpib_unregister_driver(&mut cec_pci_interface);
	pci_unregister_driver(&mut cec_pci_driver);
}

// module_init(cec_init_module)
// module_exit(cec_exit_module)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
