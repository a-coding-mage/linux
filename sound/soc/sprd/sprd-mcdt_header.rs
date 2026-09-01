// SPDX-License-Identifier: GPL-2.0

// Header guard removed: __SPRD_MCDT_H.

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sprd_mcdt_channel_type {
    SPRD_MCDT_DAC_CHAN,
    SPRD_MCDT_ADC_CHAN,
    SPRD_MCDT_UNKNOWN_CHAN,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sprd_mcdt_dma_chan {
    SPRD_MCDT_DMA_CH0,
    SPRD_MCDT_DMA_CH1,
    SPRD_MCDT_DMA_CH2,
    SPRD_MCDT_DMA_CH3,
    SPRD_MCDT_DMA_CH4,
}

#[repr(C)]
pub struct sprd_mcdt_chan_callback {
    pub notify: Option<unsafe extern "C" fn(data: *mut core::ffi::c_void)>,
    pub data: *mut core::ffi::c_void,
}

/**
 * struct sprd_mcdt_chan - this struct represents a single channel instance
 * @mcdt: the mcdt controller
 * @id: channel id
 * @fifo_phys: channel fifo physical address which is used for DMA transfer
 * @type: channel type
 * @cb: channel fifo interrupt's callback interface to notify the fifo events
 * @dma_enable: indicate if use DMA mode to transfer data
 * @int_enable: indicate if use interrupt mode to notify users to read or
 * write data manually
 * @list: used to link into the global list
 *
 * Note: users should not modify any members of this structure.
 */
#[repr(C)]
pub struct sprd_mcdt_chan {
    pub mcdt: *mut sprd_mcdt_dev,
    pub id: u8,
    pub fifo_phys: core::ffi::c_ulong,
    pub type_: sprd_mcdt_channel_type,
    pub dma_chan: sprd_mcdt_dma_chan,
    pub cb: *mut sprd_mcdt_chan_callback,
    pub dma_enable: bool,
    pub int_enable: bool,
    pub list: list_head,
}

// C conditional preserved from:
// #if IS_ENABLED(CONFIG_SND_SOC_SPRD_MCDT)
#[cfg(CONFIG_SND_SOC_SPRD_MCDT)]
unsafe extern "C" {
    pub fn sprd_mcdt_request_chan(
        channel: u8,
        type_: sprd_mcdt_channel_type,
    ) -> *mut sprd_mcdt_chan;
    pub fn sprd_mcdt_free_chan(chan: *mut sprd_mcdt_chan);

    pub fn sprd_mcdt_chan_write(
        chan: *mut sprd_mcdt_chan,
        tx_buf: *mut core::ffi::c_char,
        size: u32,
    ) -> core::ffi::c_int;
    pub fn sprd_mcdt_chan_read(
        chan: *mut sprd_mcdt_chan,
        rx_buf: *mut core::ffi::c_char,
        size: u32,
    ) -> core::ffi::c_int;
    pub fn sprd_mcdt_chan_int_enable(
        chan: *mut sprd_mcdt_chan,
        water_mark: u32,
        cb: *mut sprd_mcdt_chan_callback,
    ) -> core::ffi::c_int;
    pub fn sprd_mcdt_chan_int_disable(chan: *mut sprd_mcdt_chan);

    pub fn sprd_mcdt_chan_dma_enable(
        chan: *mut sprd_mcdt_chan,
        dma_chan: sprd_mcdt_dma_chan,
        water_mark: u32,
    ) -> core::ffi::c_int;
    pub fn sprd_mcdt_chan_dma_disable(chan: *mut sprd_mcdt_chan);
}

// C conditional preserved from:
// #else
#[cfg(not(CONFIG_SND_SOC_SPRD_MCDT))]
#[no_mangle]
pub unsafe extern "C" fn sprd_mcdt_request_chan(
    _channel: u8,
    _type: sprd_mcdt_channel_type,
) -> *mut sprd_mcdt_chan {
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_SND_SOC_SPRD_MCDT))]
#[no_mangle]
pub unsafe extern "C" fn sprd_mcdt_free_chan(_chan: *mut sprd_mcdt_chan) {}

#[cfg(not(CONFIG_SND_SOC_SPRD_MCDT))]
#[no_mangle]
pub unsafe extern "C" fn sprd_mcdt_chan_write(
    _chan: *mut sprd_mcdt_chan,
    _tx_buf: *mut core::ffi::c_char,
    _size: u32,
) -> core::ffi::c_int {
    -EINVAL
}

#[cfg(not(CONFIG_SND_SOC_SPRD_MCDT))]
#[no_mangle]
pub unsafe extern "C" fn sprd_mcdt_chan_read(
    _chan: *mut sprd_mcdt_chan,
    _rx_buf: *mut core::ffi::c_char,
    _size: u32,
) -> core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_SND_SOC_SPRD_MCDT))]
#[no_mangle]
pub unsafe extern "C" fn sprd_mcdt_chan_int_enable(
    _chan: *mut sprd_mcdt_chan,
    _water_mark: u32,
    _cb: *mut sprd_mcdt_chan_callback,
) -> core::ffi::c_int {
    -EINVAL
}

#[cfg(not(CONFIG_SND_SOC_SPRD_MCDT))]
#[no_mangle]
pub unsafe extern "C" fn sprd_mcdt_chan_int_disable(_chan: *mut sprd_mcdt_chan) {}

#[cfg(not(CONFIG_SND_SOC_SPRD_MCDT))]
#[no_mangle]
pub unsafe extern "C" fn sprd_mcdt_chan_dma_enable(
    _chan: *mut sprd_mcdt_chan,
    _dma_chan: sprd_mcdt_dma_chan,
    _water_mark: u32,
) -> core::ffi::c_int {
    -EINVAL
}

#[cfg(not(CONFIG_SND_SOC_SPRD_MCDT))]
#[no_mangle]
pub unsafe extern "C" fn sprd_mcdt_chan_dma_disable(_chan: *mut sprd_mcdt_chan) {}

// C conditional preserved from:
// #endif

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
