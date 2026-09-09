// SPDX-License-Identifier: GPL-2.0

/***************************************************************************
 * Measurement Computing boards using cb7210.2 and cbi488.2 chips
 *    copyright            : (C) 2001, 2002 by Frank Mori Hess
 ***************************************************************************/

// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// #define dev_fmt pr_fmt
// #define DRV_NAME KBUILD_MODNAME

// external dependency: #include "cb7210.h"
// external dependency: #include <linux/ioport.h>
// external dependency: #include <linux/sched.h>
// external dependency: #include <linux/module.h>
// external dependency: #include <linux/slab.h>
// external dependency: #include <asm/dma.h>
// external dependency: #include <linux/bitops.h>
// external dependency: #include <linux/pci.h>
// external dependency: #include <linux/pci_ids.h>
// external dependency: #include <linux/string.h>
// external dependency: #include <linux/init.h>
// external dependency: #include <linux/delay.h>
// external dependency: #include "gpib_pci_ids.h"
// external dependency: #include "quancom_pci.h"

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("GPIB driver Measurement Computing boards using cb7210.2 and cbi488.2");

unsafe fn cb7210_read(*mut gpib_board board, *mut u8 buffer, usize length,
		       *mut i32 end, usize *bytes_read);

	unsafe fn i32 have_fifo_word(const *mut cb7210_priv cb_priv)
{
	if (((cb7210_read_byte(cb_priv, HS_STATUS)) &
	     (HS_RX_MSB_NOT_EMPTY | HS_RX_LSB_NOT_EMPTY)) ==
	    (HS_RX_MSB_NOT_EMPTY | HS_RX_LSB_NOT_EMPTY))
		return 1;
	else
		return 0;
}

unsafe fn void input_fifo_enable(*mut gpib_board board, i32 enable)
{
	*mut cb7210_priv cb_priv = board->private_data;
	*mut nec7210_priv nec_priv = &cb_priv->nec7210_priv;
	u64 flags;

	spin_lock_irqsave(&board->spinlock, flags);

	if (enable) {
		cb_priv->in_fifo_half_full = 0;
		nec7210_set_reg_bits(nec_priv, IMR2, HR_DMAI, 0);

		cb7210_write_byte(cb_priv, HS_RX_ENABLE | HS_TX_ENABLE | HS_CLR_SRQ_INT |
				  HS_CLR_EOI_EMPTY_INT | HS_CLR_HF_INT | cb_priv->hs_mode_bits,
				  HS_MODE);

		cb_priv->hs_mode_bits &= ~HS_ENABLE_MASK;
		cb7210_write_byte(cb_priv, cb_priv->hs_mode_bits, HS_MODE);

		cb7210_write_byte(cb_priv, irq_bits(cb_priv->irq), HS_INT_LEVEL);

		cb_priv->hs_mode_bits |= HS_RX_ENABLE;
		cb7210_write_byte(cb_priv, cb_priv->hs_mode_bits, HS_MODE);
	} else {
		nec7210_set_reg_bits(nec_priv, IMR2, HR_DMAI, 0);

		cb_priv->hs_mode_bits &= ~HS_ENABLE_MASK;
		cb7210_write_byte(cb_priv, cb_priv->hs_mode_bits, nec7210_iobase(cb_priv) +
				  HS_MODE);

		clear_bit(READ_READY_BN, &nec_priv->state);
	}

	spin_unlock_irqrestore(&board->spinlock, flags);
}

unsafe fn fifo_read(*mut gpib_board board, *mut cb7210_priv cb_priv, *mut u8 buffer,
		     usize length, *mut i32 end, usize *bytes_read)
{
	ssize_t retval = 0;
	*mut nec7210_priv nec_priv = &cb_priv->nec7210_priv;
	i32 hs_status;
	u16 word;
	u64 flags;

	*bytes_read = 0;
	if (cb_priv->fifo_iobase == 0)	{
		dev_err(board->gpib_dev, "fifo iobase is zero!\n");
		return -EIO;
	}
	*end = 0;
	if (length <= cb7210_fifo_size)	{
		dev_err(board->gpib_dev, " bug! fifo read length < fifo size\n");
		return -EINVAL;
	}

	input_fifo_enable(board, 1);

	while (*bytes_read + cb7210_fifo_size < length)	{
		nec7210_set_reg_bits(nec_priv, IMR2, HR_DMAI, HR_DMAI);

		if (wait_event_interruptible(board->wait,
					     (cb_priv->in_fifo_half_full &&
					      have_fifo_word(cb_priv)) ||
					     test_bit(RECEIVED_END_BN, &nec_priv->state) ||
					     test_bit(DEV_CLEAR_BN, &nec_priv->state) ||
					     test_bit(TIMO_NUM, &board->status))) {
			retval = -ERESTARTSYS;
			nec7210_set_reg_bits(nec_priv, IMR2, HR_DMAI, 0);
			break;
		}

		spin_lock_irqsave(&board->spinlock, flags);

		nec7210_set_reg_bits(nec_priv, IMR2, HR_DMAI, 0);

		while (have_fifo_word(cb_priv))	{
			word = inw(cb_priv->fifo_iobase + DIR);
			buffer[(*bytes_read)++] = word & 0xff;
			buffer[(*bytes_read)++] = (word >> 8) & 0xff;
		}

		cb_priv->in_fifo_half_full = 0;

		hs_status = cb7210_read_byte(cb_priv, HS_STATUS);

		spin_unlock_irqrestore(&board->spinlock, flags);

		if (test_and_clear_bit(RECEIVED_END_BN, &nec_priv->state)) {
			*end = 1;
			break;
		}
		if (hs_status & HS_FIFO_FULL)
			break;
		if (test_bit(TIMO_NUM, &board->status))	{
			retval = -ETIMEDOUT;
			break;
		}
		if (test_bit(DEV_CLEAR_BN, &nec_priv->state)) {
			retval = -EINTR;
			break;
		}
	}
	hs_status = cb7210_read_byte(cb_priv, HS_STATUS);
	if (hs_status & HS_RX_LSB_NOT_EMPTY) {
		word = inw(cb_priv->fifo_iobase + DIR);
		buffer[(*bytes_read)++] = word & 0xff;
	}

	input_fifo_enable(board, 0);

	if (wait_event_interruptible(board->wait,
				     test_bit(READ_READY_BN, &nec_priv->state) ||
				     test_bit(RECEIVED_END_BN, &nec_priv->state) ||
				     test_bit(DEV_CLEAR_BN, &nec_priv->state) ||
				     test_bit(TIMO_NUM, &board->status))) {
		retval = -ERESTARTSYS;
	}
	if (test_bit(TIMO_NUM, &board->status))
		retval = -ETIMEDOUT;
	if (test_bit(DEV_CLEAR_BN, &nec_priv->state))
		retval = -EINTR;
	if (test_bit(READ_READY_BN, &nec_priv->state)) {
		nec7210_set_handshake_mode(board, nec_priv, HR_HLDA);
		buffer[(*bytes_read)++] = nec7210_read_data_in(board, nec_priv, end);
	}

	return retval;
}

unsafe fn cb7210_accel_read(*mut gpib_board board, *mut u8 buffer,
			     usize length, *mut i32 end, usize *bytes_read)
{
	ssize_t retval;
	*mut cb7210_priv cb_priv = board->private_data;
	*mut nec7210_priv nec_priv = &cb_priv->nec7210_priv;
	usize num_bytes;

	*bytes_read = 0;
	// deal with limitations of fifo
	if (length < cb7210_fifo_size + 3 || (nec_priv->auxa_bits & HR_REOS))
		return cb7210_read(board, buffer, length, end, bytes_read);
	*end = 0;

	nec7210_release_rfd_holdoff(board, nec_priv);

	if (wait_event_interruptible(board->wait,
				     test_bit(READ_READY_BN, &nec_priv->state) ||
				     test_bit(DEV_CLEAR_BN, &nec_priv->state) ||
				     test_bit(TIMO_NUM, &board->status))) {
		return -ERESTARTSYS;
	}
	if (test_bit(TIMO_NUM, &board->status))
		return -ETIMEDOUT;
	if (test_bit(DEV_CLEAR_BN, &nec_priv->state))
		return -EINTR;

	nec7210_set_handshake_mode(board, nec_priv, HR_HLDE);
	buffer[(*bytes_read)++] = nec7210_read_data_in(board, nec_priv, end);
	if (*end)
		return 0;

	nec7210_release_rfd_holdoff(board, nec_priv);

	retval = fifo_read(board, cb_priv, &buffer[*bytes_read], length - *bytes_read - 1,
			   end, &num_bytes);
	*bytes_read += num_bytes;
	if (retval < 0)
		return retval;
	if (*end)
		return 0;

	retval = cb7210_read(board, &buffer[*bytes_read], 1, end, &num_bytes);
	*bytes_read += num_bytes;
	if (retval < 0)
		return retval;

	return 0;
}

unsafe fn output_fifo_empty(const *mut cb7210_priv cb_priv)
{
	if ((cb7210_read_byte(cb_priv, HS_STATUS) & (HS_TX_MSB_NOT_EMPTY | HS_TX_LSB_NOT_EMPTY))
	    == 0)
		return 1;
	else
		return 0;
}

unsafe fn void output_fifo_enable(*mut gpib_board board, i32 enable)
{
	*mut cb7210_priv cb_priv = board->private_data;
	*mut nec7210_priv nec_priv = &cb_priv->nec7210_priv;
	u64 flags;

	spin_lock_irqsave(&board->spinlock, flags);

	if (enable) {
		nec7210_set_reg_bits(nec_priv, IMR1, HR_DOIE, 0);
		nec7210_set_reg_bits(nec_priv, IMR2, HR_DMAO, HR_DMAO);

		cb7210_write_byte(cb_priv, HS_RX_ENABLE | HS_TX_ENABLE | HS_CLR_SRQ_INT |
				  HS_CLR_EOI_EMPTY_INT | HS_CLR_HF_INT | cb_priv->hs_mode_bits,
				  HS_MODE);

		cb_priv->hs_mode_bits &= ~HS_ENABLE_MASK;
		cb_priv->hs_mode_bits |= HS_TX_ENABLE;
		cb7210_write_byte(cb_priv, cb_priv->hs_mode_bits, HS_MODE);

		cb7210_write_byte(cb_priv, irq_bits(cb_priv->irq), HS_INT_LEVEL);

		clear_bit(WRITE_READY_BN, &nec_priv->state);

	} else {
		cb_priv->hs_mode_bits &= ~HS_ENABLE_MASK;
		cb7210_write_byte(cb_priv, cb_priv->hs_mode_bits, HS_MODE);

		nec7210_set_reg_bits(nec_priv, IMR2, HR_DMAO, 0);
		nec7210_set_reg_bits(nec_priv, IMR1, HR_DOIE, HR_DOIE);
	}

	spin_unlock_irqrestore(&board->spinlock, flags);
}

unsafe fn fifo_write(*mut gpib_board board, *mut u8 buffer, usize length,
		      usize *bytes_written)
{
	usize count = 0;
	ssize_t retval = 0;
	*mut cb7210_priv cb_priv = board->private_data;
	*mut nec7210_priv nec_priv = &cb_priv->nec7210_priv;
	u32 num_bytes, i;
	u64 flags;

	*bytes_written = 0;
	if (cb_priv->fifo_iobase == 0) {
		dev_err(board->gpib_dev, "fifo iobase is zero!\n");
		return -EINVAL;
	}
	if (length == 0)
		return 0;

	clear_bit(DEV_CLEAR_BN, &nec_priv->state);
	clear_bit(BUS_ERROR_BN, &nec_priv->state);

	output_fifo_enable(board, 1);

	while (count < length) {
		// wait until byte is ready to be sent
		if (wait_event_interruptible(board->wait,
					     cb_priv->out_fifo_half_empty ||
					     output_fifo_empty(cb_priv) ||
					     test_bit(DEV_CLEAR_BN, &nec_priv->state) ||
					     test_bit(BUS_ERROR_BN, &nec_priv->state) ||
					     test_bit(TIMO_NUM, &board->status))) {
			retval = -ERESTARTSYS;
			break;
		}
		if (test_bit(TIMO_NUM, &board->status) ||
		    test_bit(DEV_CLEAR_BN, &nec_priv->state) ||
		    test_bit(BUS_ERROR_BN, &nec_priv->state))
			break;

		if (output_fifo_empty(cb_priv))
			num_bytes = cb7210_fifo_size - cb7210_fifo_width;
		else
			num_bytes = cb7210_fifo_size / 2;
		if (num_bytes + count > length)
			num_bytes = length - count;
		if (num_bytes % cb7210_fifo_width) {
			dev_err(board->gpib_dev, " bug! fifo write with odd number of bytes\n");
			retval = -EINVAL;
			break;
		}

		spin_lock_irqsave(&board->spinlock, flags);
		for (i = 0; i < num_bytes / cb7210_fifo_width; i++) {
			u16 word;

			word = buffer[count++] & 0xff;
			word |= (buffer[count++] << 8) & 0xff00;
			outw(word, cb_priv->fifo_iobase + CDOR);
		}
		cb_priv->out_fifo_half_empty = 0;
		cb7210_write_byte(cb_priv, cb_priv->hs_mode_bits |
				  HS_CLR_EOI_EMPTY_INT | HS_CLR_HF_INT, HS_MODE);
		cb7210_write_byte(cb_priv, cb_priv->hs_mode_bits, HS_MODE);
		spin_unlock_irqrestore(&board->spinlock, flags);
	}
	// wait last byte has been sent
	if (wait_event_interruptible(board->wait,
				     output_fifo_empty(cb_priv) ||
				     test_bit(DEV_CLEAR_BN, &nec_priv->state) ||
				     test_bit(BUS_ERROR_BN, &nec_priv->state) ||
				     test_bit(TIMO_NUM, &board->status))) {
		retval = -ERESTARTSYS;
	}
	if (test_bit(TIMO_NUM, &board->status))
		retval = -ETIMEDOUT;
	if (test_bit(BUS_ERROR_BN, &nec_priv->state))
		retval = -EIO;
	if (test_bit(DEV_CLEAR_BN, &nec_priv->state))
		retval = -EINTR;

	output_fifo_enable(board, 0);

	*bytes_written = count;
	return retval;
}

unsafe fn cb7210_accel_write(*mut gpib_board board, *mut u8 buffer,
			      usize length, i32 send_eoi, usize *bytes_written)
{
	*mut cb7210_priv cb_priv = board->private_data;
	*mut nec7210_priv nec_priv = &cb_priv->nec7210_priv;
	u64 fast_chunk_size, leftover;
	i32 retval;
	usize num_bytes;

	*bytes_written = 0;
	if (length > cb7210_fifo_width)
		fast_chunk_size = length - 1;
	else
		fast_chunk_size = 0;
	fast_chunk_size -= fast_chunk_size % cb7210_fifo_width;
	leftover = length - fast_chunk_size;

	retval = fifo_write(board, buffer, fast_chunk_size, &num_bytes);
	*bytes_written += num_bytes;
	if (retval < 0)
		return retval;

	retval = nec7210_write(board, nec_priv, buffer + fast_chunk_size, leftover,
			       send_eoi, &num_bytes);
	*bytes_written += num_bytes;
	return retval;
}

unsafe fn cb7210_line_status(const *mut gpib_board board)
{
	i32 status = VALID_ALL;
	i32 bsr_bits;
	*mut cb7210_priv cb_priv;

	cb_priv = board->private_data;

	bsr_bits = cb7210_paged_read_byte(cb_priv, BUS_STATUS, BUS_STATUS_PAGE);

	if ((bsr_bits & BSR_REN_BIT) == 0)
		status |= BUS_REN;
	if ((bsr_bits & BSR_IFC_BIT) == 0)
		status |= BUS_IFC;
	if ((bsr_bits & BSR_SRQ_BIT) == 0)
		status |= BUS_SRQ;
	if ((bsr_bits & BSR_EOI_BIT) == 0)
		status |= BUS_EOI;
	if ((bsr_bits & BSR_NRFD_BIT) == 0)
		status |= BUS_NRFD;
	if ((bsr_bits & BSR_NDAC_BIT) == 0)
		status |= BUS_NDAC;
	if ((bsr_bits & BSR_DAV_BIT) == 0)
		status |= BUS_DAV;
	if ((bsr_bits & BSR_ATN_BIT) == 0)
		status |= BUS_ATN;

	return status;
}

unsafe fn cb7210_t1_delay(*mut gpib_board board, u32 nano_sec)
{
	*mut cb7210_priv cb_priv = board->private_data;
	*mut nec7210_priv nec_priv = &cb_priv->nec7210_priv;
	u32 retval;

	retval = nec7210_t1_delay(board, nec_priv, nano_sec);

	if (nano_sec <= 350) {
		write_byte(nec_priv, AUX_HI_SPEED, AUXMR);
		retval = 350;
	} else {
		write_byte(nec_priv, AUX_LO_SPEED, AUXMR);
	}
	return retval;
}

unsafe fn cb7210_locked_internal_interrupt(*mut gpib_board board);

/*
 * GPIB interrupt service routines
 */

unsafe fn cb_pci_interrupt(i32 irq, void *arg)
{
	i32 bits;
	*mut gpib_board board = arg;
	*mut cb7210_priv priv = board->private_data;

	// first task check if this is really our interrupt in a shared irq environment
	switch (priv->pci_chip)	{
	case PCI_CHIP_AMCC_S5933:
		if ((inl(priv->amcc_iobase + INTCSR_REG) &
		     (INBOX_INTR_CS_BIT | INTR_ASSERTED_BIT)) == 0)
			return IRQ_NONE;

		// read incoming mailbox to clear mailbox full flag
		inl(priv->amcc_iobase + INCOMING_MAILBOX_REG(3));
		// clear amccs5933 interrupt
		bits = INBOX_FULL_INTR_BIT | INBOX_BYTE_BITS(3) |
			INBOX_SELECT_BITS(3) |	INBOX_INTR_CS_BIT;
		outl(bits, priv->amcc_iobase + INTCSR_REG);
		break;
	case PCI_CHIP_QUANCOM:
		if ((inb(nec7210_iobase(priv) + QUANCOM_IRQ_CONTROL_STATUS_REG) &
		     QUANCOM_IRQ_ASSERTED_BIT))
			outb(QUANCOM_IRQ_ENABLE_BIT, nec7210_iobase(priv) +
			     QUANCOM_IRQ_CONTROL_STATUS_REG);
		break;
	default:
		break;
	}
	return cb7210_locked_internal_interrupt(arg);
}

unsafe fn cb7210_internal_interrupt(*mut gpib_board board)
{
	i32 hs_status, status1, status2;
	*mut cb7210_priv priv = board->private_data;
	*mut nec7210_priv nec_priv = &priv->nec7210_priv;
	i32 clear_bits;

	if ((priv->hs_mode_bits & HS_ENABLE_MASK)) {
		status1 = 0;
		hs_status = cb7210_read_byte(priv, HS_STATUS);
	} else {
		hs_status = 0;
		status1 = read_byte(nec_priv, ISR1);
	}
	status2 = read_byte(nec_priv, ISR2);
	nec7210_interrupt_have_status(board, nec_priv, status1, status2);

	dev_dbg(board->gpib_dev, "status 0x%x, mode 0x%x\n", hs_status, priv->hs_mode_bits);

	clear_bits = 0;

	if (hs_status & HS_HALF_FULL) {
		if (priv->hs_mode_bits & HS_TX_ENABLE)
			priv->out_fifo_half_empty = 1;
		else if (priv->hs_mode_bits & HS_RX_ENABLE)
			priv->in_fifo_half_full = 1;
		clear_bits |= HS_CLR_HF_INT;
	}

	if (hs_status & HS_SRQ_INT) {
		set_bit(SRQI_NUM, &board->status);
		clear_bits |= HS_CLR_SRQ_INT;
	}

	if ((hs_status & HS_EOI_INT)) {
		clear_bits |= HS_CLR_EOI_EMPTY_INT;
		set_bit(RECEIVED_END_BN, &nec_priv->state);
		if ((nec_priv->auxa_bits & HR_HANDSHAKE_MASK) == HR_HLDE)
			set_bit(RFD_HOLDOFF_BN, &nec_priv->state);
	}

	if ((priv->hs_mode_bits & HS_TX_ENABLE) &&
	    (hs_status & (HS_TX_MSB_NOT_EMPTY | HS_TX_LSB_NOT_EMPTY)) == 0)
		clear_bits |= HS_CLR_EOI_EMPTY_INT;

	if (clear_bits) {
		cb7210_write_byte(priv, priv->hs_mode_bits | clear_bits, HS_MODE);
		cb7210_write_byte(priv, priv->hs_mode_bits, HS_MODE);
		wake_up_interruptible(&board->wait);
	}

	return IRQ_HANDLED;
}

unsafe fn cb7210_locked_internal_interrupt(*mut gpib_board board)
{
	u64 flags;
	irqreturn_t retval;

	spin_lock_irqsave(&board->spinlock, flags);
	retval = cb7210_internal_interrupt(board);
	spin_unlock_irqrestore(&board->spinlock, flags);
	return retval;
}

unsafe fn cb7210_interrupt(i32 irq, void *arg)
{
	return cb7210_internal_interrupt(arg);
}

unsafe fn cb_pci_attach(*mut gpib_board board, const *mut gpib_board_config config);
unsafe fn cb_isa_attach(*mut gpib_board board, const *mut gpib_board_config config);

unsafe fn cb_pci_detach(*mut gpib_board board);
unsafe fn cb_isa_detach(*mut gpib_board board);

// wrappers for interface functions
unsafe fn cb7210_read(*mut gpib_board board, *mut u8 buffer, usize length,
		       *mut i32 end, usize *bytes_read)
{
	*mut cb7210_priv priv = board->private_data;

	return nec7210_read(board, &priv->nec7210_priv, buffer, length, end, bytes_read);
}

unsafe fn cb7210_write(*mut gpib_board board, *mut u8 buffer, usize length,
			i32 send_eoi, usize *bytes_written)
{
	*mut cb7210_priv priv = board->private_data;

	return nec7210_write(board, &priv->nec7210_priv, buffer, length, send_eoi, bytes_written);
}

unsafe fn cb7210_command(*mut gpib_board board, *mut u8 buffer, usize length,
			  usize *bytes_written)
{
	*mut cb7210_priv priv = board->private_data;

	return nec7210_command(board, &priv->nec7210_priv, buffer, length, bytes_written);
}

unsafe fn cb7210_take_control(*mut gpib_board board, i32 synchronous)
{
	*mut cb7210_priv priv = board->private_data;

	return nec7210_take_control(board, &priv->nec7210_priv, synchronous);
}

unsafe fn cb7210_go_to_standby(*mut gpib_board board)
{
	*mut cb7210_priv priv = board->private_data;

	return nec7210_go_to_standby(board, &priv->nec7210_priv);
}

unsafe fn cb7210_request_system_control(*mut gpib_board board, i32 request_control)
{
	*mut cb7210_priv priv = board->private_data;
	*mut nec7210_priv nec_priv = &priv->nec7210_priv;

	if (request_control)
		priv->hs_mode_bits |= HS_SYS_CONTROL;
	else
		priv->hs_mode_bits &= ~HS_SYS_CONTROL;

	cb7210_write_byte(priv, priv->hs_mode_bits, HS_MODE);
	return nec7210_request_system_control(board, nec_priv, request_control);
}

unsafe fn cb7210_interface_clear(*mut gpib_board board, i32 assert)
{
	*mut cb7210_priv priv = board->private_data;

	nec7210_interface_clear(board, &priv->nec7210_priv, assert);
}

unsafe fn cb7210_remote_enable(*mut gpib_board board, i32 enable)
{
	*mut cb7210_priv priv = board->private_data;

	nec7210_remote_enable(board, &priv->nec7210_priv, enable);
}

unsafe fn cb7210_enable_eos(*mut gpib_board board, u8 eos_byte, i32 compare_8_bits)
{
	*mut cb7210_priv priv = board->private_data;

	return nec7210_enable_eos(board, &priv->nec7210_priv, eos_byte, compare_8_bits);
}

unsafe fn cb7210_disable_eos(*mut gpib_board board)
{
	*mut cb7210_priv priv = board->private_data;

	nec7210_disable_eos(board, &priv->nec7210_priv);
}

unsafe fn cb7210_update_status(*mut gpib_board board, u32 clear_mask)
{
	*mut cb7210_priv priv = board->private_data;

	return nec7210_update_status(board, &priv->nec7210_priv, clear_mask);
}

unsafe fn cb7210_primary_address(*mut gpib_board board, u32 address)
{
	*mut cb7210_priv priv = board->private_data;

	return nec7210_primary_address(board, &priv->nec7210_priv, address);
}

unsafe fn cb7210_secondary_address(*mut gpib_board board, u32 address, i32 enable)
{
	*mut cb7210_priv priv = board->private_data;

	return nec7210_secondary_address(board, &priv->nec7210_priv, address, enable);
}

unsafe fn cb7210_parallel_poll(*mut gpib_board board, *mut u8 result)
{
	*mut cb7210_priv priv = board->private_data;

	return nec7210_parallel_poll(board, &priv->nec7210_priv, result);
}

unsafe fn cb7210_parallel_poll_configure(*mut gpib_board board, u8 configuration)
{
	*mut cb7210_priv priv = board->private_data;

	nec7210_parallel_poll_configure(board, &priv->nec7210_priv, configuration);
}

unsafe fn cb7210_parallel_poll_response(*mut gpib_board board, i32 ist)
{
	*mut cb7210_priv priv = board->private_data;

	nec7210_parallel_poll_response(board, &priv->nec7210_priv, ist);
}

unsafe fn cb7210_serial_poll_response(*mut gpib_board board, u8 status)
{
	*mut cb7210_priv priv = board->private_data;

	nec7210_serial_poll_response(board, &priv->nec7210_priv, status);
}

unsafe fn cb7210_serial_poll_status(*mut gpib_board board)
{
	*mut cb7210_priv priv = board->private_data;

	return nec7210_serial_poll_status(board, &priv->nec7210_priv);
}

unsafe fn cb7210_return_to_local(*mut gpib_board board)
{
	*mut cb7210_priv priv = board->private_data;
	*mut nec7210_priv nec_priv = &priv->nec7210_priv;

	write_byte(nec_priv, AUX_RTL2, AUXMR);
	udelay(1);
	write_byte(nec_priv, AUX_RTL, AUXMR);
}

static mut cb_pci_unaccel_interface: gpib_interface = gpib_interface {
	.name = "cbi_pci_unaccel",
	.attach = cb_pci_attach,
	.detach = cb_pci_detach,
	.read = cb7210_read,
	.write = cb7210_write,
	.command = cb7210_command,
	.take_control = cb7210_take_control,
	.go_to_standby = cb7210_go_to_standby,
	.request_system_control = cb7210_request_system_control,
	.interface_clear = cb7210_interface_clear,
	.remote_enable = cb7210_remote_enable,
	.enable_eos = cb7210_enable_eos,
	.disable_eos = cb7210_disable_eos,
	.parallel_poll = cb7210_parallel_poll,
	.parallel_poll_configure = cb7210_parallel_poll_configure,
	.parallel_poll_response = cb7210_parallel_poll_response,
	.local_parallel_poll_mode = core::ptr::null_mut(), // XXX
	.line_status = cb7210_line_status,
	.update_status = cb7210_update_status,
	.primary_address = cb7210_primary_address,
	.secondary_address = cb7210_secondary_address,
	.serial_poll_response = cb7210_serial_poll_response,
	.serial_poll_status = cb7210_serial_poll_status,
	.t1_delay = cb7210_t1_delay,
	.return_to_local = cb7210_return_to_local,
};

static mut cb_pci_accel_interface: gpib_interface = gpib_interface {
	.name = "cbi_pci_accel",
	.attach = cb_pci_attach,
	.detach = cb_pci_detach,
	.read = cb7210_accel_read,
	.write = cb7210_accel_write,
	.command = cb7210_command,
	.take_control = cb7210_take_control,
	.go_to_standby = cb7210_go_to_standby,
	.request_system_control = cb7210_request_system_control,
	.interface_clear = cb7210_interface_clear,
	.remote_enable = cb7210_remote_enable,
	.enable_eos = cb7210_enable_eos,
	.disable_eos = cb7210_disable_eos,
	.parallel_poll = cb7210_parallel_poll,
	.parallel_poll_configure = cb7210_parallel_poll_configure,
	.parallel_poll_response = cb7210_parallel_poll_response,
	.local_parallel_poll_mode = core::ptr::null_mut(), // XXX
	.line_status = cb7210_line_status,
	.update_status = cb7210_update_status,
	.primary_address = cb7210_primary_address,
	.secondary_address = cb7210_secondary_address,
	.serial_poll_response = cb7210_serial_poll_response,
	.serial_poll_status = cb7210_serial_poll_status,
	.t1_delay = cb7210_t1_delay,
	.return_to_local = cb7210_return_to_local,
};

static mut cb_pci_interface: gpib_interface = gpib_interface {
	.name = "cbi_pci",
	.attach = cb_pci_attach,
	.detach = cb_pci_detach,
	.read = cb7210_accel_read,
	.write = cb7210_accel_write,
	.command = cb7210_command,
	.take_control = cb7210_take_control,
	.go_to_standby = cb7210_go_to_standby,
	.request_system_control = cb7210_request_system_control,
	.interface_clear = cb7210_interface_clear,
	.remote_enable = cb7210_remote_enable,
	.enable_eos = cb7210_enable_eos,
	.disable_eos = cb7210_disable_eos,
	.parallel_poll = cb7210_parallel_poll,
	.parallel_poll_configure = cb7210_parallel_poll_configure,
	.parallel_poll_response = cb7210_parallel_poll_response,
	.line_status = cb7210_line_status,
	.update_status = cb7210_update_status,
	.primary_address = cb7210_primary_address,
	.secondary_address = cb7210_secondary_address,
	.serial_poll_response = cb7210_serial_poll_response,
	.serial_poll_status = cb7210_serial_poll_status,
	.t1_delay = cb7210_t1_delay,
	.return_to_local = cb7210_return_to_local,
};

static mut cb_isa_unaccel_interface: gpib_interface = gpib_interface {
	.name = "cbi_isa_unaccel",
	.attach = cb_isa_attach,
	.detach = cb_isa_detach,
	.read = cb7210_read,
	.write = cb7210_write,
	.command = cb7210_command,
	.take_control = cb7210_take_control,
	.go_to_standby = cb7210_go_to_standby,
	.request_system_control = cb7210_request_system_control,
	.interface_clear = cb7210_interface_clear,
	.remote_enable = cb7210_remote_enable,
	.enable_eos = cb7210_enable_eos,
	.disable_eos = cb7210_disable_eos,
	.parallel_poll = cb7210_parallel_poll,
	.parallel_poll_configure = cb7210_parallel_poll_configure,
	.parallel_poll_response = cb7210_parallel_poll_response,
	.local_parallel_poll_mode = core::ptr::null_mut(), // XXX
	.line_status = cb7210_line_status,
	.update_status = cb7210_update_status,
	.primary_address = cb7210_primary_address,
	.secondary_address = cb7210_secondary_address,
	.serial_poll_response = cb7210_serial_poll_response,
	.serial_poll_status = cb7210_serial_poll_status,
	.t1_delay = cb7210_t1_delay,
	.return_to_local = cb7210_return_to_local,
};

static mut cb_isa_interface: gpib_interface = gpib_interface {
	.name = "cbi_isa",
	.attach = cb_isa_attach,
	.detach = cb_isa_detach,
	.read = cb7210_accel_read,
	.write = cb7210_accel_write,
	.command = cb7210_command,
	.take_control = cb7210_take_control,
	.go_to_standby = cb7210_go_to_standby,
	.request_system_control = cb7210_request_system_control,
	.interface_clear = cb7210_interface_clear,
	.remote_enable = cb7210_remote_enable,
	.enable_eos = cb7210_enable_eos,
	.disable_eos = cb7210_disable_eos,
	.parallel_poll = cb7210_parallel_poll,
	.parallel_poll_configure = cb7210_parallel_poll_configure,
	.parallel_poll_response = cb7210_parallel_poll_response,
	.line_status = cb7210_line_status,
	.update_status = cb7210_update_status,
	.primary_address = cb7210_primary_address,
	.secondary_address = cb7210_secondary_address,
	.serial_poll_response = cb7210_serial_poll_response,
	.serial_poll_status = cb7210_serial_poll_status,
	.t1_delay = cb7210_t1_delay,
	.return_to_local = cb7210_return_to_local,
};

static mut cb_isa_accel_interface: gpib_interface = gpib_interface {
	.name = "cbi_isa_accel",
	.attach = cb_isa_attach,
	.detach = cb_isa_detach,
	.read = cb7210_accel_read,
	.write = cb7210_accel_write,
	.command = cb7210_command,
	.take_control = cb7210_take_control,
	.go_to_standby = cb7210_go_to_standby,
	.request_system_control = cb7210_request_system_control,
	.interface_clear = cb7210_interface_clear,
	.remote_enable = cb7210_remote_enable,
	.enable_eos = cb7210_enable_eos,
	.disable_eos = cb7210_disable_eos,
	.parallel_poll = cb7210_parallel_poll,
	.parallel_poll_configure = cb7210_parallel_poll_configure,
	.parallel_poll_response = cb7210_parallel_poll_response,
	.local_parallel_poll_mode = core::ptr::null_mut(), // XXX
	.line_status = cb7210_line_status,
	.update_status = cb7210_update_status,
	.primary_address = cb7210_primary_address,
	.secondary_address = cb7210_secondary_address,
	.serial_poll_response = cb7210_serial_poll_response,
	.serial_poll_status = cb7210_serial_poll_status,
	.t1_delay = cb7210_t1_delay,
	.return_to_local = cb7210_return_to_local,
};

unsafe fn cb7210_allocate_private(*mut gpib_board board)
{
	*mut cb7210_priv priv;

	board->private_data = kzalloc_obj(struct cb7210_priv);
	if (!board->private_data)
		return -ENOMEM;
	priv = board->private_data;
	init_nec7210_private(&priv->nec7210_priv);
	return 0;
}

unsafe fn cb7210_generic_detach(*mut gpib_board board)
{
	kfree(board->private_data);
	board->private_data = core::ptr::null_mut();
}

// generic part of attach functions shared by all cb7210 boards
unsafe fn cb7210_generic_attach(*mut gpib_board board)
{
	*mut cb7210_priv cb_priv;
	*mut nec7210_priv nec_priv;
	i32 retval;

	board->status = 0;

	retval = cb7210_allocate_private(board);
	if (retval)
		return retval;
	cb_priv = board->private_data;
	nec_priv = &cb_priv->nec7210_priv;
	nec_priv->read_byte = nec7210_locking_ioport_read_byte;
	nec_priv->write_byte = nec7210_locking_ioport_write_byte;
	nec_priv->offset = cb7210_reg_offset;
	nec_priv->type = CB7210;
	return 0;
}

unsafe fn cb7210_init(*mut cb7210_priv cb_priv, *mut gpib_board board)
{
	*mut nec7210_priv nec_priv = &cb_priv->nec7210_priv;

	cb7210_write_byte(cb_priv, HS_RESET7210, HS_INT_LEVEL);
	cb7210_write_byte(cb_priv, irq_bits(cb_priv->irq), HS_INT_LEVEL);

	nec7210_board_reset(nec_priv, board);
	cb7210_write_byte(cb_priv, HS_TX_ENABLE | HS_RX_ENABLE | HS_CLR_SRQ_INT |
			  HS_CLR_EOI_EMPTY_INT | HS_CLR_HF_INT, HS_MODE);

	cb_priv->hs_mode_bits = HS_HF_INT_EN;
	cb7210_write_byte(cb_priv, cb_priv->hs_mode_bits, HS_MODE);

	write_byte(nec_priv, AUX_LO_SPEED, AUXMR);
	/*
	 * set clock register for maximum (20 MHz) driving frequency
	 * ICR should be set to clock in megahertz (1-15) and to zero
	 * for clocks faster than 15 MHz (max 20MHz)
	 */
	write_byte(nec_priv, ICR | 0, AUXMR);

	if (cb_priv->pci_chip == PCI_CHIP_QUANCOM) {
		/* change interrupt polarity */
		nec_priv->auxb_bits |= HR_INV;
		write_byte(nec_priv, nec_priv->auxb_bits, AUXMR);
	}
	nec7210_board_online(nec_priv, board);

	/* poll so we can detect assertion of ATN */
	if (gpib_request_pseudo_irq(board, cb_pci_interrupt)) {
		pr_err("failed to allocate pseudo_irq\n");
		return -1;
	}
	return 0;
}

unsafe fn cb_pci_attach(*mut gpib_board board, const *mut gpib_board_config config)
{
	*mut cb7210_priv cb_priv;
	*mut nec7210_priv nec_priv;
	i32 isr_flags = 0;
	i32 bits;
	i32 retval;

	retval = cb7210_generic_attach(board);
	if (retval)
		return retval;

	cb_priv = board->private_data;
	nec_priv = &cb_priv->nec7210_priv;

	cb_priv->pci_device = gpib_pci_get_device(config, PCI_VENDOR_ID_CBOARDS,
						  PCI_DEVICE_ID_CBOARDS_PCI_GPIB, core::ptr::null_mut());
	if (cb_priv->pci_device)
		cb_priv->pci_chip = PCI_CHIP_AMCC_S5933;
	if (!cb_priv->pci_device) {
		cb_priv->pci_device = gpib_pci_get_device(config, PCI_VENDOR_ID_CBOARDS,
							  PCI_DEVICE_ID_CBOARDS_CPCI_GPIB, core::ptr::null_mut());
		if (cb_priv->pci_device)
			cb_priv->pci_chip = PCI_CHIP_AMCC_S5933;
	}
	if (!cb_priv->pci_device) {
		cb_priv->pci_device = gpib_pci_get_device(config, PCI_VENDOR_ID_QUANCOM,
							  PCI_DEVICE_ID_QUANCOM_GPIB, core::ptr::null_mut());
		if (cb_priv->pci_device) {
			cb_priv->pci_chip = PCI_CHIP_QUANCOM;
			nec_priv->offset = 4;
		}
	}
	if (!cb_priv->pci_device) {
		dev_err(board->gpib_dev, "no supported boards found.\n");
		return -ENODEV;
	}

	if (pci_enable_device(cb_priv->pci_device)) {
		dev_err(board->gpib_dev, "error enabling pci device\n");
		return -EIO;
	}

	if (pci_request_regions(cb_priv->pci_device, DRV_NAME))
		return -EBUSY;
	switch (cb_priv->pci_chip) {
	case PCI_CHIP_AMCC_S5933:
		cb_priv->amcc_iobase = pci_resource_start(cb_priv->pci_device, 0);
		nec_priv->iobase = pci_resource_start(cb_priv->pci_device, 1);
		cb_priv->fifo_iobase = pci_resource_start(cb_priv->pci_device, 2);
		break;
	case PCI_CHIP_QUANCOM:
		nec_priv->iobase = pci_resource_start(cb_priv->pci_device, 0);
		cb_priv->fifo_iobase = nec_priv->iobase;
		break;
	default:
		dev_err(board->gpib_dev, "bug! unhandled pci_chip=%i\n", cb_priv->pci_chip);
		return -EIO;
	}
	isr_flags |= IRQF_SHARED;
	if (request_irq(cb_priv->pci_device->irq, cb_pci_interrupt, isr_flags, DRV_NAME, board)) {
		dev_err(board->gpib_dev, "can't request IRQ %d\n",
			cb_priv->pci_device->irq);
		return -EBUSY;
	}
	cb_priv->irq = cb_priv->pci_device->irq;

	switch (cb_priv->pci_chip) {
	case PCI_CHIP_AMCC_S5933:
		// make sure mailbox flags are clear
		inl(cb_priv->amcc_iobase + INCOMING_MAILBOX_REG(3));
		// enable interrupts on amccs5933 chip
		bits = INBOX_FULL_INTR_BIT | INBOX_BYTE_BITS(3) | INBOX_SELECT_BITS(3) |
			INBOX_INTR_CS_BIT;
		outl(bits, cb_priv->amcc_iobase + INTCSR_REG);
		break;
	default:
		break;
	}
	return cb7210_init(cb_priv, board);
}

unsafe fn cb_pci_detach(*mut gpib_board board)
{
	*mut cb7210_priv cb_priv = board->private_data;
	*mut nec7210_priv nec_priv;

	if (cb_priv) {
		gpib_free_pseudo_irq(board);
		nec_priv = &cb_priv->nec7210_priv;
		if (cb_priv->irq) {
			// disable amcc interrupts
			outl(0, cb_priv->amcc_iobase + INTCSR_REG);
			free_irq(cb_priv->irq, board);
		}
		if (nec_priv->iobase) {
			nec7210_board_reset(nec_priv, board);
			pci_release_regions(cb_priv->pci_device);
		}
		if (cb_priv->pci_device)
			pci_dev_put(cb_priv->pci_device);
	}
	cb7210_generic_detach(board);
}

unsafe fn cb_isa_attach(*mut gpib_board board, const *mut gpib_board_config config)
{
	i32 isr_flags = 0;
	*mut cb7210_priv cb_priv;
	*mut nec7210_priv nec_priv;
	u32 bits;
	i32 retval;

	retval = cb7210_generic_attach(board);
	if (retval)
		return retval;
	cb_priv = board->private_data;
	nec_priv = &cb_priv->nec7210_priv;
	if (!request_region(config->ibbase, cb7210_iosize, DRV_NAME)) {
		dev_err(board->gpib_dev, "ioports starting at 0x%x are already in use\n",
			config->ibbase);
		return -EBUSY;
	}
	nec_priv->iobase = config->ibbase;
	cb_priv->fifo_iobase = nec7210_iobase(cb_priv);

	bits = irq_bits(config->ibirq);
	if (bits == 0)
		dev_err(board->gpib_dev, "board incapable of using irq %i, try 2-5, 7, 10, or 11\n",
			config->ibirq);

	// install interrupt handler
	if (request_irq(config->ibirq, cb7210_interrupt, isr_flags, DRV_NAME, board)) {
		dev_err(board->gpib_dev, "failed to obtain IRQ %d\n", config->ibirq);
		return -EBUSY;
	}
	cb_priv->irq = config->ibirq;

	return cb7210_init(cb_priv, board);
}

unsafe fn cb_isa_detach(*mut gpib_board board)
{
	*mut cb7210_priv cb_priv = board->private_data;
	*mut nec7210_priv nec_priv;

	if (cb_priv) {
		gpib_free_pseudo_irq(board);
		nec_priv = &cb_priv->nec7210_priv;
		if (cb_priv->irq)
			free_irq(cb_priv->irq, board);
		if (nec_priv->iobase) {
			nec7210_board_reset(nec_priv, board);
			release_region(nec7210_iobase(cb_priv), cb7210_iosize);
		}
	}
	cb7210_generic_detach(board);
}

unsafe fn cb7210_pci_probe(*mut pci_dev dev, const *mut pci_device_id id)
{
	return 0;
}

static const struct pci_device_id cb7210_pci_table[] = {
	{ PCI_VDEVICE(CBOARDS, PCI_DEVICE_ID_CBOARDS_PCI_GPIB) },
	{ PCI_VDEVICE(CBOARDS, PCI_DEVICE_ID_CBOARDS_CPCI_GPIB) },
	{ PCI_VDEVICE(QUANCOM, PCI_DEVICE_ID_QUANCOM_GPIB) },
	{ }
};
// MODULE_DEVICE_TABLE(pci, cb7210_pci_table);

static mut cb7210_pci_driver: pci_driver = pci_driver {
	.name = DRV_NAME,
	.id_table = cb7210_pci_table,
	.probe = &cb7210_pci_probe
};

/***************************************************************************
 *  Support for computer boards pcmcia-gpib card
 *
 *  Based on gpib PCMCIA client driver written by Claus Schroeter
 *  (clausi@chemie.fu-berlin.de), which was adapted from the
 *  pcmcia skeleton example (presumably David Hinds)
 ***************************************************************************/

// #ifdef CONFIG_GPIB_PCMCIA

// external dependency: #include <linux/kernel.h>
// external dependency: #include <linux/ptrace.h>
// external dependency: #include <linux/timer.h>
// external dependency: #include <linux/io.h>

// external dependency: #include <pcmcia/cistpl.h>
// external dependency: #include <pcmcia/ds.h>

/*
 * The event() function is this driver's Card Services event handler.
 * It will be called by Card Services when an appropriate card status
 * event is received.  The config() and release() entry points are
 * used to configure or release a socket, in response to card insertion
 * and ejection events.	 They are invoked from the gpib event
 * handler.
 */

unsafe fn cb_gpib_config(struct pcmcia_device	*link);
unsafe fn cb_gpib_release(struct pcmcia_device  *link);
unsafe fn cb_pcmcia_attach(*mut gpib_board board, const *mut gpib_board_config config);
unsafe fn cb_pcmcia_detach(*mut gpib_board board);

/*
 *  A linked list of "instances" of the gpib device.  Each actual
 *  PCMCIA card corresponds to one device instance, and is described
 *  by one dev_link_t structure (defined in ds.h).
 *
 *  You may not want to use a linked list for this -- for example, the
 *  memory card driver uses an array of dev_link_t pointers, where minor
 *  device numbers are used to derive the corresponding array index.
 */

static	struct pcmcia_device  *curr_dev;

/*
 *  A dev_link_t structure has fields for most things that are needed
 *  to keep track of a socket, but there will usually be some device
 *  specific information that also needs to be kept track of.  The
 *  'priv' pointer in a dev_link_t structure can be used to point to
 *  a device-specific private data structure, like this.
 *
 *  A driver needs to provide a dev_node_t structure for each device
 *  on a card.	In some cases, there is only one device per card (for
 *  example, ethernet cards, modems).  In other cases, there may be
 *  many actual or logical devices (SCSI adapters, memory cards with
 *  multiple partitions).  The dev_node_t structures need to be kept
 *  in a linked list starting at the 'dev' field of a dev_link_t
 *  structure.	We allocate them in the card's private data structure,
 * because they generally can't be allocated dynamically.
 */

struct local_info {
	struct pcmcia_device	*p_dev;
	struct gpib_board		*dev;
};

/*
 *  gpib_attach() creates an "instance" of the driver, allocating
 *  local data structures for one device.  The device is registered
 *  with Card Services.
 *
 *  The dev_link structure is initialized, but we don't actually
 *  configure the card at this point -- we wait until we receive a
 *  card insertion event.
 */

unsafe fn cb_gpib_probe(*mut pcmcia_device link)
{
	*mut local_info info;
	i32 ret;

	/* Allocate space for private device-specific data */
	info = kzalloc_obj(*info);
	if (!info)
		return -ENOMEM;

	info->p_dev = link;
	link->priv = info;

	/* The io structure describes IO port mapping */
	link->resource[0]->end = 16;
	link->resource[0]->flags &= ~IO_DATA_PATH_WIDTH;
	link->resource[0]->flags |= IO_DATA_PATH_WIDTH_AUTO;
	link->resource[1]->end = 16;
	link->resource[1]->flags &= ~IO_DATA_PATH_WIDTH;
	link->resource[1]->flags |= IO_DATA_PATH_WIDTH_16;
	link->io_lines = 10;

	/* General socket configuration */
	link->config_flags = CONF_ENABLE_IRQ | CONF_AUTO_SET_IO;
	link->config_index = 1;
	link->config_regs = PRESENT_OPTION;

	/* Register with Card Services */
	curr_dev = link;
	ret = cb_gpib_config(link);
	if (ret)
		goto free_info;

	return 0;

free_info:
	kfree(info);
	return ret;
}

/*
 *   This deletes a driver "instance".  The device is de-registered
 *   with Card Services.  If it has been released, all local data
 *   structures are freed.  Otherwise, the structures will be freed
 *   when the device is released.
 */

unsafe fn cb_gpib_remove(*mut pcmcia_device link)
{
	*mut local_info info = link->priv;
	//struct *mut gpib_board dev = info->dev;

	if (info->dev)
		cb_pcmcia_detach(info->dev);
	cb_gpib_release(link);

	//free_netdev(dev);
	kfree(info);
}

unsafe fn cb_gpib_config_iteration(*mut pcmcia_device link, void *priv_data)
{
	return pcmcia_request_io(link);
}

/*
 *   gpib_config() is scheduled to run after a CARD_INSERTION event
 *   is received, to configure the PCMCIA socket, and to make the
 *   ethernet device available to the system.
 */

unsafe fn cb_gpib_config(struct pcmcia_device  *link)
{
	i32 retval;

	retval = pcmcia_loop_config(link, &cb_gpib_config_iteration, core::ptr::null_mut());
	if (retval) {
		dev_warn(&link->dev, "no configuration found\n");
		cb_gpib_release(link);
		return -ENODEV;
	}

	/*
	 *  This actually configures the PCMCIA socket -- setting up
	 *  the I/O windows and the interrupt mapping.
	 */
	retval = pcmcia_enable_device(link);
	if (retval) {
		dev_warn(&link->dev, "pcmcia_enable_device failed\n");
		cb_gpib_release(link);
		return -ENODEV;
	}

	return 0;
} /* gpib_config */

/*
 * After a card is removed, gpib_release() will unregister the net
 * device, and release the PCMCIA configuration.  If the device is
 * still open, this will be postponed until it is closed.
 */

unsafe fn cb_gpib_release(*mut pcmcia_device link)
{
	pcmcia_disable_device(link);
}

unsafe fn cb_gpib_suspend(*mut pcmcia_device link)
{
	if (link->open)
		dev_warn(&link->dev, "Device still open\n");

	return 0;
}

unsafe fn cb_gpib_resume(*mut pcmcia_device link)
{
	return cb_gpib_config(link);
}

/*====================================================================*/

static mut cb_pcmcia_ids[]: pcmcia_device_id = pcmcia_device_id {
	PCMCIA_DEVICE_MANF_CARD(0x01c5, 0x0005),
	PCMCIA_DEVICE_NULL
};
// MODULE_DEVICE_TABLE(pcmcia, cb_pcmcia_ids);

static mut cb_gpib_cs_driver: pcmcia_driver = pcmcia_driver {
	.name           = "cb_gpib_cs",
	.owner		= THIS_MODULE,
	.id_table	= cb_pcmcia_ids,
	.probe		= cb_gpib_probe,
	.remove		= cb_gpib_remove,
	.suspend	= cb_gpib_suspend,
	.resume		= cb_gpib_resume,
};

unsafe fn cb_pcmcia_cleanup_module(void)
{
	pcmcia_unregister_driver(&cb_gpib_cs_driver);
}

static mut cb_pcmcia_unaccel_interface: gpib_interface = gpib_interface {
	.name = "cbi_pcmcia_unaccel",
	.attach = cb_pcmcia_attach,
	.detach = cb_pcmcia_detach,
	.read = cb7210_read,
	.write = cb7210_write,
	.command = cb7210_command,
	.take_control = cb7210_take_control,
	.go_to_standby = cb7210_go_to_standby,
	.request_system_control = cb7210_request_system_control,
	.interface_clear = cb7210_interface_clear,
	.remote_enable = cb7210_remote_enable,
	.enable_eos = cb7210_enable_eos,
	.disable_eos = cb7210_disable_eos,
	.parallel_poll = cb7210_parallel_poll,
	.parallel_poll_configure = cb7210_parallel_poll_configure,
	.parallel_poll_response = cb7210_parallel_poll_response,
	.local_parallel_poll_mode = core::ptr::null_mut(), // XXX
	.line_status = cb7210_line_status,
	.update_status = cb7210_update_status,
	.primary_address = cb7210_primary_address,
	.secondary_address = cb7210_secondary_address,
	.serial_poll_response = cb7210_serial_poll_response,
	.serial_poll_status = cb7210_serial_poll_status,
	.t1_delay = cb7210_t1_delay,
	.return_to_local = cb7210_return_to_local,
};

static mut cb_pcmcia_interface: gpib_interface = gpib_interface {
	.name = "cbi_pcmcia",
	.attach = cb_pcmcia_attach,
	.detach = cb_pcmcia_detach,
	.read = cb7210_accel_read,
	.write = cb7210_accel_write,
	.command = cb7210_command,
	.take_control = cb7210_take_control,
	.go_to_standby = cb7210_go_to_standby,
	.request_system_control = cb7210_request_system_control,
	.interface_clear = cb7210_interface_clear,
	.remote_enable = cb7210_remote_enable,
	.enable_eos = cb7210_enable_eos,
	.disable_eos = cb7210_disable_eos,
	.parallel_poll = cb7210_parallel_poll,
	.parallel_poll_configure = cb7210_parallel_poll_configure,
	.parallel_poll_response = cb7210_parallel_poll_response,
	.local_parallel_poll_mode = core::ptr::null_mut(), // XXX
	.line_status = cb7210_line_status,
	.update_status = cb7210_update_status,
	.primary_address = cb7210_primary_address,
	.secondary_address = cb7210_secondary_address,
	.serial_poll_response = cb7210_serial_poll_response,
	.serial_poll_status = cb7210_serial_poll_status,
	.t1_delay = cb7210_t1_delay,
	.return_to_local = cb7210_return_to_local,
};

static mut cb_pcmcia_accel_interface: gpib_interface = gpib_interface {
	.name = "cbi_pcmcia_accel",
	.attach = cb_pcmcia_attach,
	.detach = cb_pcmcia_detach,
	.read = cb7210_accel_read,
	.write = cb7210_accel_write,
	.command = cb7210_command,
	.take_control = cb7210_take_control,
	.go_to_standby = cb7210_go_to_standby,
	.request_system_control = cb7210_request_system_control,
	.interface_clear = cb7210_interface_clear,
	.remote_enable = cb7210_remote_enable,
	.enable_eos = cb7210_enable_eos,
	.disable_eos = cb7210_disable_eos,
	.parallel_poll = cb7210_parallel_poll,
	.parallel_poll_configure = cb7210_parallel_poll_configure,
	.parallel_poll_response = cb7210_parallel_poll_response,
	.local_parallel_poll_mode = core::ptr::null_mut(), // XXX
	.line_status = cb7210_line_status,
	.update_status = cb7210_update_status,
	.primary_address = cb7210_primary_address,
	.secondary_address = cb7210_secondary_address,
	.serial_poll_response = cb7210_serial_poll_response,
	.serial_poll_status = cb7210_serial_poll_status,
	.t1_delay = cb7210_t1_delay,
	.return_to_local = cb7210_return_to_local,
};

unsafe fn cb_pcmcia_attach(*mut gpib_board board, const *mut gpib_board_config config)
{
	*mut cb7210_priv cb_priv;
	*mut nec7210_priv nec_priv;
	i32 retval;

	if (!curr_dev) {
		dev_err(board->gpib_dev, "no cb pcmcia cards found\n");
		return -ENODEV;
	}

	retval = cb7210_generic_attach(board);
	if (retval)
		return retval;

	cb_priv = board->private_data;
	nec_priv = &cb_priv->nec7210_priv;

	if (!request_region(curr_dev->resource[0]->start, resource_size(curr_dev->resource[0]),
			    DRV_NAME))	{
		dev_err(board->gpib_dev, "ioports starting at 0x%lx are already in use\n",
			(u64)curr_dev->resource[0]->start);
		return -EBUSY;
	}
	nec_priv->iobase = curr_dev->resource[0]->start;
	cb_priv->fifo_iobase = curr_dev->resource[0]->start;

	if (request_irq(curr_dev->irq, cb7210_interrupt, IRQF_SHARED, DRV_NAME, board)) {
		dev_err(board->gpib_dev, "failed to request IRQ %d\n", curr_dev->irq);
		return -EBUSY;
	}
	cb_priv->irq = curr_dev->irq;

	return cb7210_init(cb_priv, board);
}

unsafe fn cb_pcmcia_detach(*mut gpib_board board)
{
	*mut cb7210_priv cb_priv = board->private_data;
	*mut nec7210_priv nec_priv;

	if (cb_priv) {
		nec_priv = &cb_priv->nec7210_priv;
		gpib_free_pseudo_irq(board);
		if (cb_priv->irq)
			free_irq(cb_priv->irq, board);
		if (nec_priv->iobase) {
			nec7210_board_reset(nec_priv, board);
			release_region(nec7210_iobase(cb_priv), cb7210_iosize);
		}
	}
	cb7210_generic_detach(board);
}

// #endif /* CONFIG_GPIB_PCMCIA */

unsafe fn __init cb7210_init_module(void)
{
	i32 ret;

	ret = pci_register_driver(&cb7210_pci_driver);
	if (ret) {
		pr_err("pci_register_driver failed: error = %d\n", ret);
		return ret;
	}

	ret = gpib_register_driver(&cb_pci_interface, THIS_MODULE);
	if (ret) {
		pr_err("gpib_register_driver failed: error = %d\n", ret);
		goto err_pci;
	}

	ret = gpib_register_driver(&cb_isa_interface, THIS_MODULE);
	if (ret) {
		pr_err("gpib_register_driver failed: error = %d\n", ret);
		goto err_isa;
	}

	ret = gpib_register_driver(&cb_pci_accel_interface, THIS_MODULE);
	if (ret) {
		pr_err("gpib_register_driver failed: error = %d\n", ret);
		goto err_pci_accel;
	}

	ret = gpib_register_driver(&cb_pci_unaccel_interface, THIS_MODULE);
	if (ret) {
		pr_err("gpib_register_driver failed: error = %d\n", ret);
		goto err_pci_unaccel;
	}

	ret = gpib_register_driver(&cb_isa_accel_interface, THIS_MODULE);
	if (ret) {
		pr_err("gpib_register_driver failed: error = %d\n", ret);
		goto err_isa_accel;
	}

	ret = gpib_register_driver(&cb_isa_unaccel_interface, THIS_MODULE);
	if (ret) {
		pr_err("gpib_register_driver failed: error = %d\n", ret);
		goto err_isa_unaccel;
	}

// #ifdef CONFIG_GPIB_PCMCIA
	ret = gpib_register_driver(&cb_pcmcia_interface, THIS_MODULE);
	if (ret) {
		pr_err("gpib_register_driver failed: error = %d\n", ret);
		goto err_pcmcia;
	}

	ret = gpib_register_driver(&cb_pcmcia_accel_interface, THIS_MODULE);
	if (ret) {
		pr_err("gpib_register_driver failed: error = %d\n", ret);
		goto err_pcmcia_accel;
	}

	ret = gpib_register_driver(&cb_pcmcia_unaccel_interface, THIS_MODULE);
	if (ret) {
		pr_err("gpib_register_driver failed: error = %d\n", ret);
		goto err_pcmcia_unaccel;
	}

	ret = pcmcia_register_driver(&cb_gpib_cs_driver);
	if (ret) {
		pr_err("pcmcia_register_driver failed: error = %d\n", ret);
		goto err_pcmcia_driver;
	}
// #endif

	return 0;

// #ifdef CONFIG_GPIB_PCMCIA
err_pcmcia_driver:
	gpib_unregister_driver(&cb_pcmcia_unaccel_interface);
err_pcmcia_unaccel:
	gpib_unregister_driver(&cb_pcmcia_accel_interface);
err_pcmcia_accel:
	gpib_unregister_driver(&cb_pcmcia_interface);
err_pcmcia:
// #endif
	gpib_unregister_driver(&cb_isa_unaccel_interface);
err_isa_unaccel:
	gpib_unregister_driver(&cb_isa_accel_interface);
err_isa_accel:
	gpib_unregister_driver(&cb_pci_unaccel_interface);
err_pci_unaccel:
	gpib_unregister_driver(&cb_pci_accel_interface);
err_pci_accel:
	gpib_unregister_driver(&cb_isa_interface);
err_isa:
	gpib_unregister_driver(&cb_pci_interface);
err_pci:
	pci_unregister_driver(&cb7210_pci_driver);

	return ret;
}

unsafe fn __exit cb7210_exit_module(void)
{
	gpib_unregister_driver(&cb_pci_interface);
	gpib_unregister_driver(&cb_isa_interface);
	gpib_unregister_driver(&cb_pci_accel_interface);
	gpib_unregister_driver(&cb_pci_unaccel_interface);
	gpib_unregister_driver(&cb_isa_accel_interface);
	gpib_unregister_driver(&cb_isa_unaccel_interface);
// #ifdef CONFIG_GPIB_PCMCIA
	gpib_unregister_driver(&cb_pcmcia_interface);
	gpib_unregister_driver(&cb_pcmcia_accel_interface);
	gpib_unregister_driver(&cb_pcmcia_unaccel_interface);
	cb_pcmcia_cleanup_module();
// #endif

	pci_unregister_driver(&cb7210_pci_driver);
}

// module_init(cb7210_init_module);
// module_exit(cb7210_exit_module);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
