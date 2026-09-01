// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  ALSA card-level driver for Turtle Beach Wavefront cards
 *						(Maui,Tropez,Tropez+)
 *
 *  Copyright (c) 1997-1999 by Paul Barton-Davis <pbd@op.net>
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

MODULE_AUTHOR!("Paul Barton-Davis <pbd@op.net>");
MODULE_DESCRIPTION!("Turtle Beach Wavefront");
MODULE_LICENSE!("GPL");

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX;	    /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR;	    /* ID for this card */
static mut enable: [bool; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE;	    /* Enable this card */
/* CONFIG_PNP */
static mut isapnp: [bool; SNDRV_CARDS] = [true; SNDRV_CARDS];
static mut cs4232_pcm_port: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT;	/* PnP setup */
static mut cs4232_pcm_irq: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IRQ; /* 5,7,9,11,12,15 */
static mut cs4232_mpu_port: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT; /* PnP setup */
static mut cs4232_mpu_irq: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IRQ; /* 9,11,12,15 */
static mut ics2115_port: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT; /* PnP setup */
static mut ics2115_irq: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IRQ;    /* 2,9,11,12,15 */
static mut fm_port: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT;	    /* PnP setup */
static mut dma1: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_DMA;	    /* 0,1,3,5,6,7 */
static mut dma2: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_DMA;	    /* 0,1,3,5,6,7 */
static mut use_cs4232_midi: [bool; SNDRV_CARDS] = [false; SNDRV_CARDS];

module_param_array!(index, int, NULL, 0444);
MODULE_PARM_DESC!(index, "Index value for WaveFront soundcard.");
module_param_array!(id, charp, NULL, 0444);
MODULE_PARM_DESC!(id, "ID string for WaveFront soundcard.");
module_param_array!(enable, bool, NULL, 0444);
MODULE_PARM_DESC!(enable, "Enable WaveFront soundcard.");
/* CONFIG_PNP */
module_param_array!(isapnp, bool, NULL, 0444);
MODULE_PARM_DESC!(isapnp, "ISA PnP detection for WaveFront soundcards.");
module_param_hw_array!(cs4232_pcm_port, long, ioport, NULL, 0444);
MODULE_PARM_DESC!(cs4232_pcm_port, "Port # for CS4232 PCM interface.");
module_param_hw_array!(cs4232_pcm_irq, int, irq, NULL, 0444);
MODULE_PARM_DESC!(cs4232_pcm_irq, "IRQ # for CS4232 PCM interface.");
module_param_hw_array!(dma1, int, dma, NULL, 0444);
MODULE_PARM_DESC!(dma1, "DMA1 # for CS4232 PCM interface.");
module_param_hw_array!(dma2, int, dma, NULL, 0444);
MODULE_PARM_DESC!(dma2, "DMA2 # for CS4232 PCM interface.");
module_param_hw_array!(cs4232_mpu_port, long, ioport, NULL, 0444);
MODULE_PARM_DESC!(cs4232_mpu_port, "port # for CS4232 MPU-401 interface.");
module_param_hw_array!(cs4232_mpu_irq, int, irq, NULL, 0444);
MODULE_PARM_DESC!(cs4232_mpu_irq, "IRQ # for CS4232 MPU-401 interface.");
module_param_hw_array!(ics2115_irq, int, irq, NULL, 0444);
MODULE_PARM_DESC!(ics2115_irq, "IRQ # for ICS2115.");
module_param_hw_array!(ics2115_port, long, ioport, NULL, 0444);
MODULE_PARM_DESC!(ics2115_port, "Port # for ICS2115.");
module_param_hw_array!(fm_port, long, ioport, NULL, 0444);
MODULE_PARM_DESC!(fm_port, "FM port #.");
module_param_array!(use_cs4232_midi, bool, NULL, 0444);
MODULE_PARM_DESC!(use_cs4232_midi, "Use CS4232 MPU-401 interface (inaccessibly located inside your computer)");

/* CONFIG_PNP */
static mut isa_registered: c_int = 0;
static mut pnp_registered: c_int = 0;

static snd_wavefront_pnpids: [pnp_card_device_id; 3] = [
	pnp_card_device_id {
		/* Tropez */
		id: c"CSC7532".as_ptr(),
		devs: [
			pnp_device_id { id: c"CSC0000".as_ptr() },
			pnp_device_id { id: c"CSC0010".as_ptr() },
			pnp_device_id { id: c"PnPb006".as_ptr() },
			pnp_device_id { id: c"CSC0004".as_ptr() },
		],
	},
	pnp_card_device_id {
		/* Tropez+ */
		id: c"CSC7632".as_ptr(),
		devs: [
			pnp_device_id { id: c"CSC0000".as_ptr() },
			pnp_device_id { id: c"CSC0010".as_ptr() },
			pnp_device_id { id: c"PnPb006".as_ptr() },
			pnp_device_id { id: c"CSC0004".as_ptr() },
		],
	},
	pnp_card_device_id {
		id: c"".as_ptr(),
		devs: unsafe { core::mem::zeroed() },
	},
];

MODULE_DEVICE_TABLE!(pnp_card, snd_wavefront_pnpids);

unsafe fn snd_wavefront_pnp(
	dev: c_int,
	acard: *mut snd_wavefront_card_t,
	card: *mut pnp_card_link,
	id: *const pnp_card_device_id,
) -> c_int {
	let mut pdev: *mut pnp_dev;
	let mut err: c_int;

	/* Check for each logical device. */

	/* CS4232 chip (aka "windows sound system") is logical device 0 */

	(*acard).wss = pnp_request_card_device(card, (*id).devs[0].id, ptr::null_mut());
	if (*acard).wss.is_null() {
		return -EBUSY;
	}

	/* there is a game port at logical device 1, but we ignore it completely */

	/* the control interface is logical device 2, but we ignore it
	   completely. in fact, nobody even seems to know what it
	   does.
	*/

	/* Only configure the CS4232 MIDI interface if its been
	   specifically requested. It is logical device 3.
	*/

	if use_cs4232_midi[dev as usize] {
		(*acard).mpu = pnp_request_card_device(card, (*id).devs[2].id, ptr::null_mut());
		if (*acard).mpu.is_null() {
			return -EBUSY;
		}
	}

	/* The ICS2115 synth is logical device 4 */

	(*acard).synth = pnp_request_card_device(card, (*id).devs[3].id, ptr::null_mut());
	if (*acard).synth.is_null() {
		return -EBUSY;
	}

	/* PCM/FM initialization */

	pdev = (*acard).wss;

	/* An interesting note from the Tropez+ FAQ:

	   Q. [Ports] Why is the base address of the WSS I/O ports off by 4?

	   A. WSS I/O requires a block of 8 I/O addresses ("ports"). Of these, the first
	   4 are used to identify and configure the board. With the advent of PnP,
	   these first 4 addresses have become obsolete, and software applications
	   only use the last 4 addresses to control the codec chip. Therefore, the
	   base address setting "skips past" the 4 unused addresses.

	*/

	err = pnp_activate_dev(pdev);
	if err < 0 {
		dev_err!(&mut (*pdev).dev, "PnP WSS pnp configure failure\n");
		return err;
	}

	cs4232_pcm_port[dev as usize] = pnp_port_start(pdev, 0) as c_long;
	fm_port[dev as usize] = pnp_port_start(pdev, 1) as c_long;
	dma1[dev as usize] = pnp_dma(pdev, 0);
	dma2[dev as usize] = pnp_dma(pdev, 1);
	cs4232_pcm_irq[dev as usize] = pnp_irq(pdev, 0);

	/* Synth initialization */

	pdev = (*acard).synth;

	err = pnp_activate_dev(pdev);
	if err < 0 {
		dev_err!(&mut (*pdev).dev, "PnP ICS2115 pnp configure failure\n");
		return err;
	}

	ics2115_port[dev as usize] = pnp_port_start(pdev, 0) as c_long;
	ics2115_irq[dev as usize] = pnp_irq(pdev, 0);

	/* CS4232 MPU initialization. Configure this only if
	   explicitly requested, since its physically inaccessible and
	   consumes another IRQ.
	*/

	if use_cs4232_midi[dev as usize] {
		pdev = (*acard).mpu;

		err = pnp_activate_dev(pdev);
		if err < 0 {
			dev_err!(&mut (*pdev).dev, "PnP MPU401 pnp configure failure\n");
			cs4232_mpu_port[dev as usize] = SNDRV_AUTO_PORT;
		} else {
			cs4232_mpu_port[dev as usize] = pnp_port_start(pdev, 0) as c_long;
			cs4232_mpu_irq[dev as usize] = pnp_irq(pdev, 0);
		}

		dev_info!(
			&mut (*pdev).dev,
			"CS4232 MPU: port=0x%lx, irq=%i\n",
			cs4232_mpu_port[dev as usize],
			cs4232_mpu_irq[dev as usize]
		);
	}

	dev_dbg!(
		&mut (*pdev).dev,
		"CS4232: pcm port=0x%lx, fm port=0x%lx, dma1=%i, dma2=%i, irq=%i\nICS2115: port=0x%lx, irq=%i\n",
		cs4232_pcm_port[dev as usize],
		fm_port[dev as usize],
		dma1[dev as usize],
		dma2[dev as usize],
		cs4232_pcm_irq[dev as usize],
		ics2115_port[dev as usize],
		ics2115_irq[dev as usize]
	);

	0
}

unsafe extern "C" fn snd_wavefront_ics2115_interrupt(
	_irq: c_int,
	dev_id: *mut c_void,
) -> irqreturn_t {
	let mut acard: *mut snd_wavefront_card_t;

	acard = dev_id as *mut snd_wavefront_card_t;

	if acard.is_null() {
		return IRQ_NONE;
	}

	if (*acard).wavefront.interrupts_are_midi {
		snd_wavefront_midi_interrupt(acard);
	} else {
		snd_wavefront_internal_interrupt(acard);
	}
	IRQ_HANDLED
}

unsafe fn snd_wavefront_new_synth(
	card: *mut snd_card,
	hw_dev: c_int,
	acard: *mut snd_wavefront_card_t,
) -> *mut snd_hwdep {
	let mut wavefront_synth: *mut snd_hwdep = ptr::null_mut();

	if snd_wavefront_detect(acard) < 0 {
		return ptr::null_mut();
	}

	if snd_wavefront_start(&mut (*acard).wavefront) < 0 {
		return ptr::null_mut();
	}

	if snd_hwdep_new(card, c"WaveFront".as_ptr(), hw_dev, &mut wavefront_synth) < 0 {
		return ptr::null_mut();
	}
	strscpy(
		(*wavefront_synth).name.as_mut_ptr(),
		c"WaveFront (ICS2115) wavetable synthesizer".as_ptr(),
	);
	(*wavefront_synth).ops.open = Some(snd_wavefront_synth_open);
	(*wavefront_synth).ops.release = Some(snd_wavefront_synth_release);
	(*wavefront_synth).ops.ioctl = Some(snd_wavefront_synth_ioctl);

	wavefront_synth
}

unsafe fn snd_wavefront_new_fx(
	card: *mut snd_card,
	hw_dev: c_int,
	acard: *mut snd_wavefront_card_t,
	port: c_ulong,
) -> *mut snd_hwdep {
	let mut fx_processor: *mut snd_hwdep = ptr::null_mut();

	if snd_wavefront_fx_start(&mut (*acard).wavefront) != 0 {
		dev_err!((*card).dev, "cannot initialize YSS225 FX processor");
		return ptr::null_mut();
	}

	if snd_hwdep_new(card, c"YSS225".as_ptr(), hw_dev, &mut fx_processor) < 0 {
		return ptr::null_mut();
	}
	sprintf(
		(*fx_processor).name.as_mut_ptr(),
		c"YSS225 FX Processor at 0x%lx".as_ptr(),
		port,
	);
	(*fx_processor).ops.open = Some(snd_wavefront_fx_open);
	(*fx_processor).ops.release = Some(snd_wavefront_fx_release);
	(*fx_processor).ops.ioctl = Some(snd_wavefront_fx_ioctl);

	fx_processor
}

static mut internal_id: snd_wavefront_mpu_id = internal_mpu;
static mut external_id: snd_wavefront_mpu_id = external_mpu;

unsafe fn snd_wavefront_new_midi(
	card: *mut snd_card,
	midi_dev: c_int,
	acard: *mut snd_wavefront_card_t,
	port: c_ulong,
	mpu: snd_wavefront_mpu_id,
) -> *mut snd_rawmidi {
	let mut rmidi: *mut snd_rawmidi = ptr::null_mut();
	static mut first: c_int = 1;

	if first != 0 {
		first = 0;
		(*acard).wavefront.midi.base = port;
		if snd_wavefront_midi_start(acard) != 0 {
			dev_err!((*card).dev, "cannot initialize MIDI interface\n");
			return ptr::null_mut();
		}
	}

	if snd_rawmidi_new(card, c"WaveFront MIDI".as_ptr(), midi_dev, 1, 1, &mut rmidi) < 0 {
		return ptr::null_mut();
	}

	if mpu == internal_mpu {
		strscpy((*rmidi).name.as_mut_ptr(), c"WaveFront MIDI (Internal)".as_ptr());
		(*rmidi).private_data = &mut internal_id as *mut _ as *mut c_void;
	} else {
		strscpy((*rmidi).name.as_mut_ptr(), c"WaveFront MIDI (External)".as_ptr());
		(*rmidi).private_data = &mut external_id as *mut _ as *mut c_void;
	}

	snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT, &snd_wavefront_midi_output);
	snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT, &snd_wavefront_midi_input);

	(*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_OUTPUT
		| SNDRV_RAWMIDI_INFO_INPUT
		| SNDRV_RAWMIDI_INFO_DUPLEX;

	rmidi
}

unsafe fn snd_wavefront_card_new(
	pdev: *mut device,
	dev: c_int,
	cardp: *mut *mut snd_card,
) -> c_int {
	let mut card: *mut snd_card = ptr::null_mut();
	let mut acard: *mut snd_wavefront_card_t;
	let mut err: c_int;

	err = snd_devm_card_new(
		pdev,
		index[dev as usize],
		id[dev as usize],
		THIS_MODULE,
		core::mem::size_of::<snd_wavefront_card_t>(),
		&mut card,
	);
	if err < 0 {
		return err;
	}

	acard = (*card).private_data as *mut snd_wavefront_card_t;
	(*acard).wavefront.irq = -1;
	spin_lock_init(&mut (*acard).wavefront.irq_lock);
	init_waitqueue_head(&mut (*acard).wavefront.interrupt_sleeper);
	spin_lock_init(&mut (*acard).wavefront.midi.open);
	spin_lock_init(&mut (*acard).wavefront.midi.virtual_);
	(*acard).wavefront.card = card;

	*cardp = card;
	0
}

unsafe fn snd_wavefront_probe(card: *mut snd_card, dev: c_int) -> c_int {
	let mut acard: *mut snd_wavefront_card_t = (*card).private_data as *mut snd_wavefront_card_t;
	let mut chip: *mut snd_wss = ptr::null_mut();
	let mut wavefront_synth: *mut snd_hwdep;
	let mut ics2115_internal_rmidi: *mut snd_rawmidi = ptr::null_mut();
	let mut ics2115_external_rmidi: *mut snd_rawmidi = ptr::null_mut();
	let mut fx_processor: *mut snd_hwdep;
	let mut hw_dev: c_int = 0;
	let mut midi_dev: c_int = 0;
	let mut err: c_int;

	/* --------- PCM --------------- */

	err = snd_wss_create(
		card,
		cs4232_pcm_port[dev as usize],
		-1,
		cs4232_pcm_irq[dev as usize],
		dma1[dev as usize],
		dma2[dev as usize],
		WSS_HW_DETECT,
		0,
		&mut chip,
	);
	if err < 0 {
		dev_err!((*card).dev, "can't allocate WSS device\n");
		return err;
	}
	(*acard).chip = chip;

	err = snd_wss_pcm(chip, 0);
	if err < 0 {
		return err;
	}

	err = snd_wss_timer(chip, 0);
	if err < 0 {
		return err;
	}

	/* ---------- OPL3 synth --------- */

	if fm_port[dev as usize] > 0 && fm_port[dev as usize] != SNDRV_AUTO_PORT {
		let mut opl3: *mut snd_opl3 = ptr::null_mut();

		err = snd_opl3_create(
			card,
			fm_port[dev as usize],
			fm_port[dev as usize] + 2,
			OPL3_HW_OPL3_CS,
			0,
			&mut opl3,
		);
		if err < 0 {
			dev_err!((*card).dev, "can't allocate or detect OPL3 synth\n");
			return err;
		}

		err = snd_opl3_hwdep_new(opl3, hw_dev, 1, ptr::null_mut());
		if err < 0 {
			return err;
		}
		hw_dev += 1;
	}

	/* ------- ICS2115 Wavetable synth ------- */

	(*acard).wavefront.res_base =
		devm_request_region((*card).dev, ics2115_port[dev as usize], 16, c"ICS2115".as_ptr());
	if (*acard).wavefront.res_base.is_null() {
		dev_err!(
			(*card).dev,
			"unable to grab ICS2115 i/o region 0x%lx-0x%lx\n",
			ics2115_port[dev as usize],
			ics2115_port[dev as usize] + 16 - 1
		);
		return -EBUSY;
	}
	if devm_request_irq(
		(*card).dev,
		ics2115_irq[dev as usize],
		Some(snd_wavefront_ics2115_interrupt),
		0,
		c"ICS2115".as_ptr(),
		acard as *mut c_void,
	) != 0
	{
		dev_err!((*card).dev, "unable to use ICS2115 IRQ %d\n", ics2115_irq[dev as usize]);
		return -EBUSY;
	}

	(*acard).wavefront.irq = ics2115_irq[dev as usize];
	(*card).sync_irq = (*acard).wavefront.irq;
	(*acard).wavefront.base = ics2115_port[dev as usize];
	snd_wavefront_cache_firmware(&mut (*acard).wavefront);

	wavefront_synth = snd_wavefront_new_synth(card, hw_dev, acard);
	if wavefront_synth.is_null() {
		dev_err!((*card).dev, "can't create WaveFront synth device\n");
		return -ENOMEM;
	}

	strscpy((*wavefront_synth).name.as_mut_ptr(), c"ICS2115 Wavetable MIDI Synthesizer".as_ptr());
	(*wavefront_synth).iface = SNDRV_HWDEP_IFACE_ICS2115;
	hw_dev += 1;

	/* --------- Mixer ------------ */

	err = snd_wss_mixer(chip);
	if err < 0 {
		dev_err!((*card).dev, "can't allocate mixer device\n");
		return err;
	}

	/* -------- CS4232 MPU-401 interface -------- */

	if cs4232_mpu_port[dev as usize] > 0 && cs4232_mpu_port[dev as usize] != SNDRV_AUTO_PORT {
		err = snd_mpu401_uart_new(
			card,
			midi_dev,
			MPU401_HW_CS4232,
			cs4232_mpu_port[dev as usize],
			0,
			cs4232_mpu_irq[dev as usize],
			ptr::null_mut(),
		);
		if err < 0 {
			dev_err!((*card).dev, "can't allocate CS4232 MPU-401 device\n");
			return err;
		}
		midi_dev += 1;
	}

	/* ------ ICS2115 internal MIDI ------------ */

	if ics2115_port[dev as usize] > 0 && ics2115_port[dev as usize] != SNDRV_AUTO_PORT {
		ics2115_internal_rmidi = snd_wavefront_new_midi(
			card,
			midi_dev,
			acard,
			ics2115_port[dev as usize] as c_ulong,
			internal_mpu,
		);
		if ics2115_internal_rmidi.is_null() {
			dev_err!((*card).dev, "can't setup ICS2115 internal MIDI device\n");
			return -ENOMEM;
		}
		midi_dev += 1;
	}

	/* ------ ICS2115 external MIDI ------------ */

	if ics2115_port[dev as usize] > 0 && ics2115_port[dev as usize] != SNDRV_AUTO_PORT {
		ics2115_external_rmidi = snd_wavefront_new_midi(
			card,
			midi_dev,
			acard,
			ics2115_port[dev as usize] as c_ulong,
			external_mpu,
		);
		if ics2115_external_rmidi.is_null() {
			dev_err!((*card).dev, "can't setup ICS2115 external MIDI device\n");
			return -ENOMEM;
		}
		midi_dev += 1;
	}

	/* FX processor for Tropez+ */

	if (*acard).wavefront.has_fx {
		fx_processor = snd_wavefront_new_fx(
			card,
			hw_dev,
			acard,
			ics2115_port[dev as usize] as c_ulong,
		);
		if fx_processor.is_null() {
			dev_err!((*card).dev, "can't setup FX device\n");
			return -ENOMEM;
		}

		hw_dev += 1;

		strscpy((*card).driver.as_mut_ptr(), c"Tropez+".as_ptr());
		strscpy((*card).shortname.as_mut_ptr(), c"Turtle Beach Tropez+".as_ptr());
	} else {
		/* Need a way to distinguish between Maui and Tropez */
		strscpy((*card).driver.as_mut_ptr(), c"WaveFront".as_ptr());
		strscpy((*card).shortname.as_mut_ptr(), c"Turtle Beach WaveFront".as_ptr());
	}

	/* ----- Register the card --------- */

	/* Not safe to include "Turtle Beach" in longname, due to
	   length restrictions
	*/

	sprintf(
		(*card).longname.as_mut_ptr(),
		c"%s PCM 0x%lx irq %d dma %d".as_ptr(),
		(*card).driver.as_ptr(),
		(*chip).port,
		cs4232_pcm_irq[dev as usize],
		dma1[dev as usize],
	);

	if dma2[dev as usize] >= 0 && dma2[dev as usize] < 8 {
		sprintf(
			(*card).longname.as_mut_ptr().add(strlen((*card).longname.as_ptr())),
			c"&%d".as_ptr(),
			dma2[dev as usize],
		);
	}

	if cs4232_mpu_port[dev as usize] > 0 && cs4232_mpu_port[dev as usize] != SNDRV_AUTO_PORT {
		sprintf(
			(*card).longname.as_mut_ptr().add(strlen((*card).longname.as_ptr())),
			c" MPU-401 0x%lx irq %d".as_ptr(),
			cs4232_mpu_port[dev as usize],
			cs4232_mpu_irq[dev as usize],
		);
	}

	sprintf(
		(*card).longname.as_mut_ptr().add(strlen((*card).longname.as_ptr())),
		c" SYNTH 0x%lx irq %d".as_ptr(),
		ics2115_port[dev as usize],
		ics2115_irq[dev as usize],
	);

	snd_card_register(card)
}

unsafe extern "C" fn snd_wavefront_isa_match(
	pdev: *mut device,
	dev: c_uint,
) -> c_int {
	if !enable[dev as usize] {
		return 0;
	}
	/* CONFIG_PNP */
	if isapnp[dev as usize] {
		return 0;
	}
	if cs4232_pcm_port[dev as usize] == SNDRV_AUTO_PORT {
		dev_err!(pdev, "specify CS4232 port\n");
		return 0;
	}
	if ics2115_port[dev as usize] == SNDRV_AUTO_PORT {
		dev_err!(pdev, "specify ICS2115 port\n");
		return 0;
	}
	1
}

unsafe extern "C" fn snd_wavefront_isa_probe(
	pdev: *mut device,
	dev: c_uint,
) -> c_int {
	let mut card: *mut snd_card = ptr::null_mut();
	let mut err: c_int;

	err = snd_wavefront_card_new(pdev, dev as c_int, &mut card);
	if err < 0 {
		return err;
	}
	err = snd_wavefront_probe(card, dev as c_int);
	if err < 0 {
		return err;
	}

	dev_set_drvdata(pdev, card as *mut c_void);
	0
}

/* CONFIG_PM */
unsafe fn snd_wavefront_suspend(card: *mut snd_card) -> c_int {
	let acard: *mut snd_wavefront_card_t = (*card).private_data as *mut snd_wavefront_card_t;

	snd_wavefront_midi_suspend(acard);
	snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
	((*(*acard).chip).suspend.unwrap())((*acard).chip);
	0
}

unsafe fn snd_wavefront_resume(card: *mut snd_card) -> c_int {
	let acard: *mut snd_wavefront_card_t = (*card).private_data as *mut snd_wavefront_card_t;
	let mut err: c_int;

	((*(*acard).chip).resume.unwrap())((*acard).chip);
	err = snd_wavefront_resume_synth(acard);
	if err < 0 {
		return err;
	}
	snd_power_change_state(card, SNDRV_CTL_POWER_D0);
	0
}

unsafe extern "C" fn snd_wavefront_isa_suspend(
	dev: *mut device,
	_id: c_uint,
	_state: pm_message_t,
) -> c_int {
	snd_wavefront_suspend(dev_get_drvdata(dev) as *mut snd_card)
}

unsafe extern "C" fn snd_wavefront_isa_resume(dev: *mut device, _id: c_uint) -> c_int {
	snd_wavefront_resume(dev_get_drvdata(dev) as *mut snd_card)
}

const DEV_NAME: *const c_char = c"wavefront".as_ptr();

static mut snd_wavefront_driver: isa_driver = isa_driver {
	match_: Some(snd_wavefront_isa_match),
	probe: Some(snd_wavefront_isa_probe),
	/* CONFIG_PM */
	suspend: Some(snd_wavefront_isa_suspend),
	resume: Some(snd_wavefront_isa_resume),
	driver: device_driver {
		name: DEV_NAME,
	},
};

/* CONFIG_PNP */
unsafe extern "C" fn snd_wavefront_pnp_detect(
	pcard: *mut pnp_card_link,
	pid: *const pnp_card_device_id,
) -> c_int {
	static mut dev: c_int = 0;
	let mut card: *mut snd_card = ptr::null_mut();
	let mut res: c_int;

	while dev < SNDRV_CARDS as c_int {
		if enable[dev as usize] && isapnp[dev as usize] {
			break;
		}
		dev += 1;
	}
	if dev >= SNDRV_CARDS as c_int {
		return -ENODEV;
	}

	res = snd_wavefront_card_new(&mut (*(*pcard).card).dev, dev, &mut card);
	if res < 0 {
		return res;
	}

	if snd_wavefront_pnp(dev, (*card).private_data as *mut snd_wavefront_card_t, pcard, pid) < 0 {
		if cs4232_pcm_port[dev as usize] == SNDRV_AUTO_PORT {
			dev_err!((*card).dev, "isapnp detection failed\n");
			return -ENODEV;
		}
	}

	res = snd_wavefront_probe(card, dev);
	if res < 0 {
		return res;
	}

	pnp_set_card_drvdata(pcard, card as *mut c_void);
	dev += 1;
	0
}

/* CONFIG_PM */
unsafe extern "C" fn snd_wavefront_pnpc_suspend(
	pcard: *mut pnp_card_link,
	_state: pm_message_t,
) -> c_int {
	snd_wavefront_suspend(pnp_get_card_drvdata(pcard) as *mut snd_card)
}

unsafe extern "C" fn snd_wavefront_pnpc_resume(pcard: *mut pnp_card_link) -> c_int {
	snd_wavefront_resume(pnp_get_card_drvdata(pcard) as *mut snd_card)
}

static mut wavefront_pnpc_driver: pnp_card_driver = pnp_card_driver {
	flags: PNP_DRIVER_RES_DISABLE,
	name: c"wavefront".as_ptr(),
	id_table: snd_wavefront_pnpids.as_ptr(),
	probe: Some(snd_wavefront_pnp_detect),
	/* CONFIG_PM */
	suspend: Some(snd_wavefront_pnpc_suspend),
	resume: Some(snd_wavefront_pnpc_resume),
};

unsafe fn alsa_card_wavefront_init() -> c_int {
	let mut err: c_int;

	err = isa_register_driver(&mut snd_wavefront_driver, SNDRV_CARDS as c_uint);
	/* CONFIG_PNP */
	if err == 0 {
		isa_registered = 1;
	}

	err = pnp_register_card_driver(&mut wavefront_pnpc_driver);
	if err == 0 {
		pnp_registered = 1;
	}

	if isa_registered != 0 {
		err = 0;
	}
	err
}

unsafe fn alsa_card_wavefront_exit() {
	/* CONFIG_PNP */
	if pnp_registered != 0 {
		pnp_unregister_card_driver(&mut wavefront_pnpc_driver);
	}
	if isa_registered != 0 {
		isa_unregister_driver(&mut snd_wavefront_driver);
	}
}

module_init!(alsa_card_wavefront_init);
module_exit!(alsa_card_wavefront_exit);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
