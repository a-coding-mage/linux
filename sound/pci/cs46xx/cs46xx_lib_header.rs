/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  The driver for the Cirrus Logic's Sound Fusion CS46XX based soundcards
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

/*
 *  constants
 */

pub const CS46XX_BA0_SIZE: u32 = 0x1000;
pub const CS46XX_BA1_DATA0_SIZE: u32 = 0x3000;
pub const CS46XX_BA1_DATA1_SIZE: u32 = 0x3800;
pub const CS46XX_BA1_PRG_SIZE: u32 = 0x7000;
pub const CS46XX_BA1_REG_SIZE: u32 = 0x0100;

/* Original C condition: CONFIG_SND_CS46XX_NEW_DSP */
#[cfg(CONFIG_SND_CS46XX_NEW_DSP)]
pub const CS46XX_MIN_PERIOD_SIZE: u32 = 64;
#[cfg(CONFIG_SND_CS46XX_NEW_DSP)]
pub const CS46XX_MAX_PERIOD_SIZE: u32 = 1024 * 1024;
#[cfg(not(CONFIG_SND_CS46XX_NEW_DSP))]
pub const CS46XX_MIN_PERIOD_SIZE: u32 = 2048;
#[cfg(not(CONFIG_SND_CS46XX_NEW_DSP))]
pub const CS46XX_MAX_PERIOD_SIZE: u32 = 2048;

pub const CS46XX_FRAGS: u32 = 2;
/* #define CS46XX_BUFFER_SIZE CS46XX_MAX_PERIOD_SIZE * CS46XX_FRAGS */

pub const SCB_NO_PARENT: u32 = 0;
pub const SCB_ON_PARENT_NEXT_SCB: u32 = 1;
pub const SCB_ON_PARENT_SUBLIST_SCB: u32 = 2;

/* 3*1024 parameter, 3.5*1024 sample, 2*3.5*1024 code */
pub const BA1_DWORD_SIZE: u32 = 13 * 1024 + 512;
pub const BA1_MEMORY_COUNT: u32 = 3;

pub type u16 = core::ffi::c_ushort;
pub type u32 = core::ffi::c_uint;

#[repr(C)]
pub struct snd_cs46xx {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct dsp_spos_instance {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct dsp_module_desc {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct dsp_symbol_entry {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct dsp_scb_descriptor {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct dsp_pcm_channel_descriptor {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn writel(val: core::ffi::c_uint, addr: *mut core::ffi::c_void);
    pub fn readl(addr: *const core::ffi::c_void) -> core::ffi::c_uint;
}

/*
 *  common I/O routines
 */

pub unsafe fn snd_cs46xx_poke(chip: *mut snd_cs46xx, reg: core::ffi::c_ulong, val: core::ffi::c_uint) {
    let bank: core::ffi::c_uint = (reg >> 16) as core::ffi::c_uint;
    let offset: core::ffi::c_uint = (reg & 0xffff) as core::ffi::c_uint;

    /*
    if (bank == 0)
        printk(KERN_DEBUG "snd_cs46xx_poke: %04X - %08X\n",
               reg >> 2,val);
    */

    /*
     * Original C:
     * writel(val, chip->region.idx[bank+1].remap_addr + offset);
     *
     * This isolated header does not include the definition of struct snd_cs46xx,
     * so the field-path address computation cannot be expressed file-locally.
     */
    let _ = (chip, bank, offset, val);
    todo!("requires snd_cs46xx.region.idx[bank + 1].remap_addr layout from external dependency");
}

pub unsafe fn snd_cs46xx_peek(chip: *mut snd_cs46xx, reg: core::ffi::c_ulong) -> core::ffi::c_uint {
    let bank: core::ffi::c_uint = (reg >> 16) as core::ffi::c_uint;
    let offset: core::ffi::c_uint = (reg & 0xffff) as core::ffi::c_uint;

    /*
     * Original C:
     * return readl(chip->region.idx[bank+1].remap_addr + offset);
     *
     * This isolated header does not include the definition of struct snd_cs46xx,
     * so the field-path address computation cannot be expressed file-locally.
     */
    let _ = (chip, bank, offset);
    todo!("requires snd_cs46xx.region.idx[bank + 1].remap_addr layout from external dependency");
}

pub unsafe fn snd_cs46xx_pokeBA0(chip: *mut snd_cs46xx, offset: core::ffi::c_ulong, val: core::ffi::c_uint) {
    /*
     * Original C:
     * writel(val, chip->region.name.ba0.remap_addr + offset);
     *
     * This isolated header does not include the definition of struct snd_cs46xx,
     * so the field-path address computation cannot be expressed file-locally.
     */
    let _ = (chip, offset, val);
    todo!("requires snd_cs46xx.region.name.ba0.remap_addr layout from external dependency");
}

pub unsafe fn snd_cs46xx_peekBA0(chip: *mut snd_cs46xx, offset: core::ffi::c_ulong) -> core::ffi::c_uint {
    /*
     * Original C:
     * return readl(chip->region.name.ba0.remap_addr + offset);
     *
     * This isolated header does not include the definition of struct snd_cs46xx,
     * so the field-path address computation cannot be expressed file-locally.
     */
    let _ = (chip, offset);
    todo!("requires snd_cs46xx.region.name.ba0.remap_addr layout from external dependency");
}

unsafe extern "C" {
    pub fn cs46xx_dsp_spos_create(chip: *mut snd_cs46xx) -> *mut dsp_spos_instance;
    pub fn cs46xx_dsp_spos_destroy(chip: *mut snd_cs46xx);
    pub fn cs46xx_dsp_load_module(chip: *mut snd_cs46xx, module: *mut dsp_module_desc) -> core::ffi::c_int;

    /* Original C condition: CONFIG_PM_SLEEP */
    #[cfg(CONFIG_PM_SLEEP)]
    pub fn cs46xx_dsp_resume(chip: *mut snd_cs46xx) -> core::ffi::c_int;

    pub fn cs46xx_dsp_lookup_symbol(
        chip: *mut snd_cs46xx,
        symbol_name: *mut core::ffi::c_char,
        symbol_type: core::ffi::c_int,
    ) -> *mut dsp_symbol_entry;

    /* Original C condition: CONFIG_SND_PROC_FS */
    #[cfg(CONFIG_SND_PROC_FS)]
    pub fn cs46xx_dsp_proc_init(card: *mut snd_card, chip: *mut snd_cs46xx) -> core::ffi::c_int;
    #[cfg(CONFIG_SND_PROC_FS)]
    pub fn cs46xx_dsp_proc_done(chip: *mut snd_cs46xx) -> core::ffi::c_int;

    pub fn cs46xx_dsp_scb_and_task_init(chip: *mut snd_cs46xx) -> core::ffi::c_int;
    pub fn snd_cs46xx_download(
        chip: *mut snd_cs46xx,
        src: *mut u32,
        offset: core::ffi::c_ulong,
        len: core::ffi::c_ulong,
    ) -> core::ffi::c_int;
    pub fn snd_cs46xx_clear_BA1(
        chip: *mut snd_cs46xx,
        offset: core::ffi::c_ulong,
        len: core::ffi::c_ulong,
    ) -> core::ffi::c_int;
    pub fn cs46xx_dsp_enable_spdif_out(chip: *mut snd_cs46xx) -> core::ffi::c_int;
    pub fn cs46xx_dsp_enable_spdif_hw(chip: *mut snd_cs46xx) -> core::ffi::c_int;
    pub fn cs46xx_dsp_disable_spdif_out(chip: *mut snd_cs46xx) -> core::ffi::c_int;
    pub fn cs46xx_dsp_enable_spdif_in(chip: *mut snd_cs46xx) -> core::ffi::c_int;
    pub fn cs46xx_dsp_disable_spdif_in(chip: *mut snd_cs46xx) -> core::ffi::c_int;
    pub fn cs46xx_dsp_enable_pcm_capture(chip: *mut snd_cs46xx) -> core::ffi::c_int;
    pub fn cs46xx_dsp_disable_pcm_capture(chip: *mut snd_cs46xx) -> core::ffi::c_int;
    pub fn cs46xx_dsp_enable_adc_capture(chip: *mut snd_cs46xx) -> core::ffi::c_int;
    pub fn cs46xx_dsp_disable_adc_capture(chip: *mut snd_cs46xx) -> core::ffi::c_int;
    pub fn cs46xx_poke_via_dsp(chip: *mut snd_cs46xx, address: u32, data: u32) -> core::ffi::c_int;
    pub fn cs46xx_dsp_create_scb(
        chip: *mut snd_cs46xx,
        name: *mut core::ffi::c_char,
        scb_data: *mut u32,
        dest: u32,
    ) -> *mut dsp_scb_descriptor;

    /* Original C condition: CONFIG_SND_PROC_FS */
    #[cfg(CONFIG_SND_PROC_FS)]
    pub fn cs46xx_dsp_proc_free_scb_desc(scb: *mut dsp_scb_descriptor);
    #[cfg(CONFIG_SND_PROC_FS)]
    pub fn cs46xx_dsp_proc_register_scb_desc(chip: *mut snd_cs46xx, scb: *mut dsp_scb_descriptor);

    pub fn cs46xx_dsp_create_timing_master_scb(chip: *mut snd_cs46xx) -> *mut dsp_scb_descriptor;
    pub fn cs46xx_dsp_create_codec_out_scb(
        chip: *mut snd_cs46xx,
        codec_name: *mut core::ffi::c_char,
        channel_disp: u16,
        fifo_addr: u16,
        child_scb_addr: u16,
        dest: u32,
        parent_scb: *mut dsp_scb_descriptor,
        scb_child_type: core::ffi::c_int,
    ) -> *mut dsp_scb_descriptor;
    pub fn cs46xx_dsp_create_codec_in_scb(
        chip: *mut snd_cs46xx,
        codec_name: *mut core::ffi::c_char,
        channel_disp: u16,
        fifo_addr: u16,
        sample_buffer_addr: u16,
        dest: u32,
        parent_scb: *mut dsp_scb_descriptor,
        scb_child_type: core::ffi::c_int,
    ) -> *mut dsp_scb_descriptor;
    pub fn cs46xx_dsp_remove_scb(chip: *mut snd_cs46xx, scb: *mut dsp_scb_descriptor);
    pub fn cs46xx_dsp_create_src_task_scb(
        chip: *mut snd_cs46xx,
        scb_name: *mut core::ffi::c_char,
        sample_rate: core::ffi::c_int,
        src_buffer_addr: u16,
        src_delay_buffer_addr: u16,
        dest: u32,
        parent_scb: *mut dsp_scb_descriptor,
        scb_child_type: core::ffi::c_int,
        pass_through: core::ffi::c_int,
    ) -> *mut dsp_scb_descriptor;
    pub fn cs46xx_dsp_create_mix_only_scb(
        chip: *mut snd_cs46xx,
        scb_name: *mut core::ffi::c_char,
        mix_buffer_addr: u16,
        dest: u32,
        parent_scb: *mut dsp_scb_descriptor,
        scb_child_type: core::ffi::c_int,
    ) -> *mut dsp_scb_descriptor;
    pub fn cs46xx_dsp_create_vari_decimate_scb(
        chip: *mut snd_cs46xx,
        scb_name: *mut core::ffi::c_char,
        vari_buffer_addr0: u16,
        vari_buffer_addr1: u16,
        dest: u32,
        parent_scb: *mut dsp_scb_descriptor,
        scb_child_type: core::ffi::c_int,
    ) -> *mut dsp_scb_descriptor;
    pub fn cs46xx_dsp_create_asynch_fg_rx_scb(
        chip: *mut snd_cs46xx,
        scb_name: *mut core::ffi::c_char,
        dest: u32,
        hfg_scb_address: u16,
        asynch_buffer_address: u16,
        parent_scb: *mut dsp_scb_descriptor,
        scb_child_type: core::ffi::c_int,
    ) -> *mut dsp_scb_descriptor;
    pub fn cs46xx_dsp_create_spio_write_scb(
        chip: *mut snd_cs46xx,
        scb_name: *mut core::ffi::c_char,
        dest: u32,
        parent_scb: *mut dsp_scb_descriptor,
        scb_child_type: core::ffi::c_int,
    ) -> *mut dsp_scb_descriptor;
    pub fn cs46xx_dsp_create_mix_to_ostream_scb(
        chip: *mut snd_cs46xx,
        scb_name: *mut core::ffi::c_char,
        mix_buffer_addr: u16,
        writeback_spb: u16,
        dest: u32,
        parent_scb: *mut dsp_scb_descriptor,
        scb_child_type: core::ffi::c_int,
    ) -> *mut dsp_scb_descriptor;
    pub fn cs46xx_dsp_create_magic_snoop_scb(
        chip: *mut snd_cs46xx,
        scb_name: *mut core::ffi::c_char,
        dest: u32,
        snoop_buffer_address: u16,
        snoop_scb: *mut dsp_scb_descriptor,
        parent_scb: *mut dsp_scb_descriptor,
        scb_child_type: core::ffi::c_int,
    ) -> *mut dsp_scb_descriptor;
    pub fn cs46xx_dsp_create_pcm_channel(
        chip: *mut snd_cs46xx,
        sample_rate: u32,
        private_data: *mut core::ffi::c_void,
        hw_dma_addr: u32,
        pcm_channel_id: core::ffi::c_int,
    ) -> *mut dsp_pcm_channel_descriptor;
    pub fn cs46xx_dsp_destroy_pcm_channel(
        chip: *mut snd_cs46xx,
        pcm_channel: *mut dsp_pcm_channel_descriptor,
    );
    pub fn cs46xx_dsp_pcm_unlink(
        chip: *mut snd_cs46xx,
        pcm_channel: *mut dsp_pcm_channel_descriptor,
    ) -> core::ffi::c_int;
    pub fn cs46xx_dsp_pcm_link(
        chip: *mut snd_cs46xx,
        pcm_channel: *mut dsp_pcm_channel_descriptor,
    ) -> core::ffi::c_int;
    pub fn cs46xx_add_record_source(
        chip: *mut snd_cs46xx,
        source: *mut dsp_scb_descriptor,
        addr: u16,
        scb_name: *mut core::ffi::c_char,
    ) -> *mut dsp_scb_descriptor;
    pub fn cs46xx_src_unlink(chip: *mut snd_cs46xx, src: *mut dsp_scb_descriptor) -> core::ffi::c_int;
    pub fn cs46xx_src_link(chip: *mut snd_cs46xx, src: *mut dsp_scb_descriptor) -> core::ffi::c_int;
    pub fn cs46xx_iec958_pre_open(chip: *mut snd_cs46xx) -> core::ffi::c_int;
    pub fn cs46xx_iec958_post_close(chip: *mut snd_cs46xx) -> core::ffi::c_int;
    pub fn cs46xx_dsp_pcm_channel_set_period(
        chip: *mut snd_cs46xx,
        pcm_channel: *mut dsp_pcm_channel_descriptor,
        period_size: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn cs46xx_dsp_pcm_ostream_set_period(
        chip: *mut snd_cs46xx,
        period_size: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn cs46xx_dsp_set_dac_volume(chip: *mut snd_cs46xx, left: u16, right: u16) -> core::ffi::c_int;
    pub fn cs46xx_dsp_set_iec958_volume(chip: *mut snd_cs46xx, left: u16, right: u16) -> core::ffi::c_int;
}

#[cfg(not(CONFIG_SND_PROC_FS))]
pub unsafe fn cs46xx_dsp_proc_init(_card: *mut snd_card, _chip: *mut snd_cs46xx) {}

#[cfg(not(CONFIG_SND_PROC_FS))]
pub unsafe fn cs46xx_dsp_proc_done(_chip: *mut snd_cs46xx) {}

#[cfg(not(CONFIG_SND_PROC_FS))]
pub unsafe fn cs46xx_dsp_proc_free_scb_desc(_scb: *mut dsp_scb_descriptor) {}

#[cfg(not(CONFIG_SND_PROC_FS))]
pub unsafe fn cs46xx_dsp_proc_register_scb_desc(
    _chip: *mut snd_cs46xx,
    _scb: *mut dsp_scb_descriptor,
) {
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
