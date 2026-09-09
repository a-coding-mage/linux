/* Translated from smu_v11_0_i2c.c. */

const I2C_OK: u32 = 0;
const I2C_NAK_7B_ADDR_NOACK: u32 = 1;
const I2C_NAK_TXDATA_NOACK: u32 = 2;
const I2C_TIMEOUT: u32 = 4;
const I2C_SW_TIMEOUT: u32 = 8;
const I2C_ABORT: u32 = 0x10;
const I2C_X_RESTART: u32 = 1u32 << 31;
const I2C_SPEED_MODE_FAST: u32 = 2;
const T_I2C_POLL_US: u32 = 25;
const I2C_MAX_T_POLL_COUNT: i32 = 1000;

extern "C" {
    fn i2c_get_adapdata(control: *mut i2c_adapter) -> *mut amdgpu_smu_i2c_bus;
    fn rreg32_soc15(block: u32, instance: u32, reg: u32) -> u32;
    fn wreg32_soc15(block: u32, instance: u32, reg: u32, value: u32);
    fn reg_set_field(reg: u32, block: u32, field: u32, value: u32) -> u32;
    fn reg_get_field(reg: u32, block: u32, field: u32) -> u32;
    fn udelay(usecs: u32);
    fn time_after(a: u64, b: u64) -> bool;
    fn jiffies() -> u64;
    fn msecs_to_jiffies(msecs: u32) -> u64;
    fn drm_debug_enabled(flag: u32) -> bool;
    fn print_hex_dump(prefix: *const u8, data: *const u8, len: usize);
    fn drm_error(msg: *const u8);
    fn drm_warn(msg: *const u8);
    fn drm_debug_driver(msg: *const u8);
}

#[repr(C)] pub struct i2c_adapter { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_device { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_smu_i2c_bus { pub adev: *mut amdgpu_device }

const SMUIO: u32 = 0;
const mmSMUIO_PWRMGT: u32 = 0;
const mmCKSVII2C_IC_ENABLE: u32 = 0;
const mmCKSVII2C_IC_ENABLE_STATUS: u32 = 0;
const mmCKSVII2C_IC_CLR_INTR: u32 = 0;
const mmCKSVII2C_IC_CON: u32 = 0;
const mmCKSVII2C_IC_FS_SPKLEN: u32 = 0;
const mmCKSVII2C_IC_SS_SCL_HCNT: u32 = 0;
const mmCKSVII2C_IC_SS_SCL_LCNT: u32 = 0;
const mmCKSVII2C_IC_SDA_HOLD: u32 = 0;
const mmCKSVII2C_IC_TAR: u32 = 0;
const mmCKSVII2C_IC_STATUS: u32 = 0;
const mmCKSVII2C_IC_INTR_STAT: u32 = 0;
const mmCKSVII2C_IC_TX_ABRT_SOURCE: u32 = 0;
const mmCKSVII2C_IC_DATA_CMD: u32 = 0;
const mmCKSVII2C_IC_CLR_ACTIVITY: u32 = 0;
const I2C_M_STOP: u32 = 0x4000;

unsafe fn smu_v11_0_i2c_set_clock_gating(control: *mut i2c_adapter, en: bool) {
    let bus = &mut *i2c_get_adapdata(control);
    let reg = rreg32_soc15(SMUIO, 0, mmSMUIO_PWRMGT);
    let reg = reg_set_field(reg, 0, 0, if en { 1 } else { 0 });
    wreg32_soc15(SMUIO, 0, mmSMUIO_PWRMGT, reg);
}

unsafe fn smu_v11_0_i2c_enable(control: *mut i2c_adapter, enable: bool) -> i32 {
    let _bus = &mut *i2c_get_adapdata(control);
    wreg32_soc15(SMUIO, 0, mmCKSVII2C_IC_ENABLE, if enable { 1 } else { 0 });
    if !enable {
        let mut ii = I2C_MAX_T_POLL_COUNT;
        while ii > 0 {
            let en_stat = rreg32_soc15(SMUIO, 0, mmCKSVII2C_IC_ENABLE_STATUS);
            if reg_get_field(en_stat, 0, 0) != 0 { udelay(T_I2C_POLL_US); }
            else { return I2C_OK as i32; }
            ii -= 1;
        }
        return I2C_ABORT as i32;
    }
    I2C_OK as i32
}

unsafe fn smu_v11_0_i2c_clear_status(control: *mut i2c_adapter) {
    let _bus = &mut *i2c_get_adapdata(control);
    rreg32_soc15(SMUIO, 0, mmCKSVII2C_IC_CLR_INTR);
}

unsafe fn smu_v11_0_i2c_configure(control: *mut i2c_adapter) {
    let _bus = &mut *i2c_get_adapdata(control);
    let mut reg = 0;
    reg = reg_set_field(reg, 0, 0, 1);
    reg = reg_set_field(reg, 0, 0, 1);
    reg = reg_set_field(reg, 0, 0, 0);
    reg = reg_set_field(reg, 0, 0, 0);
    reg = reg_set_field(reg, 0, 0, I2C_SPEED_MODE_FAST);
    reg = reg_set_field(reg, 0, 0, 1);
    wreg32_soc15(SMUIO, 0, mmCKSVII2C_IC_CON, reg);
}

unsafe fn smu_v11_0_i2c_set_clock(control: *mut i2c_adapter) {
    let _bus = &mut *i2c_get_adapdata(control);
    wreg32_soc15(SMUIO, 0, mmCKSVII2C_IC_FS_SPKLEN, 2);
    wreg32_soc15(SMUIO, 0, mmCKSVII2C_IC_SS_SCL_HCNT, 120);
    wreg32_soc15(SMUIO, 0, mmCKSVII2C_IC_SS_SCL_LCNT, 130);
    wreg32_soc15(SMUIO, 0, mmCKSVII2C_IC_SDA_HOLD, 20);
}

unsafe fn smu_v11_0_i2c_set_address(control: *mut i2c_adapter, address: u16) {
    let _bus = &mut *i2c_get_adapdata(control);
    wreg32_soc15(SMUIO, 0, mmCKSVII2C_IC_TAR, (address as u32) & 0x3ff);
}

unsafe fn smu_v11_0_i2c_poll_tx_status(control: *mut i2c_adapter) -> u32 {
    let mut ret = I2C_OK;
    let timeout_counter = jiffies() + msecs_to_jiffies(20);
    loop {
        if time_after(jiffies(), timeout_counter) { ret |= I2C_SW_TIMEOUT; break; }
        let reg = rreg32_soc15(SMUIO, 0, mmCKSVII2C_IC_STATUS);
        if reg_get_field(reg, 0, 0) != 0 { break; }
    }
    if ret != I2C_OK { return ret; }
    let reg = rreg32_soc15(SMUIO, 0, mmCKSVII2C_IC_INTR_STAT);
    if reg_get_field(reg, 0, 0) == 1 {
        let source = rreg32_soc15(SMUIO, 0, mmCKSVII2C_IC_TX_ABRT_SOURCE);
        if reg_get_field(source, 0, 0) == 1 { ret |= I2C_NAK_TXDATA_NOACK; }
        else if reg_get_field(source, 0, 0) == 1 { ret |= I2C_NAK_7B_ADDR_NOACK; }
        else { ret |= I2C_ABORT; }
        smu_v11_0_i2c_clear_status(control);
    }
    ret
}

unsafe fn smu_v11_0_i2c_poll_rx_status(control: *mut i2c_adapter) -> u32 {
    let mut ret = I2C_OK;
    let source = rreg32_soc15(SMUIO, 0, mmCKSVII2C_IC_TX_ABRT_SOURCE);
    if reg_get_field(source, 0, 0) == 1 { ret |= I2C_NAK_7B_ADDR_NOACK; smu_v11_0_i2c_clear_status(control); }
    else {
        let timeout_counter = jiffies() + msecs_to_jiffies(20);
        loop {
            if time_after(jiffies(), timeout_counter) { ret |= I2C_SW_TIMEOUT; break; }
            let status = rreg32_soc15(SMUIO, 0, mmCKSVII2C_IC_STATUS);
            if reg_get_field(status, 0, 0) != 0 { break; }
        }
    }
    ret
}

unsafe fn smu_v11_0_i2c_transmit(control: *mut i2c_adapter, address: u16, data: *mut u8, mut numbytes: u32, i2c_flag: u32) -> u32 {
    let mut bytes_sent = 0;
    let mut ret = I2C_OK;
    smu_v11_0_i2c_set_address(control, address);
    smu_v11_0_i2c_enable(control, true);
    smu_v11_0_i2c_clear_status(control);
    let timeout_counter = jiffies() + msecs_to_jiffies(20);
    while numbytes > 0 {
        let mut reg = rreg32_soc15(SMUIO, 0, mmCKSVII2C_IC_STATUS);
        if reg_get_field(reg, 0, 0) == 0 {
            if time_after(jiffies(), timeout_counter) { ret |= I2C_SW_TIMEOUT; break; }
        } else {
            reg = reg_set_field(reg, 0, 0, *data.add(bytes_sent as usize) as u32);
            if numbytes == 1 && (i2c_flag & I2C_M_STOP) != 0 { reg = reg_set_field(reg, 0, 0, 1); }
            if bytes_sent == 0 && (i2c_flag & I2C_X_RESTART) != 0 { reg = reg_set_field(reg, 0, 0, 1); }
            reg = reg_set_field(reg, 0, 0, 0);
            wreg32_soc15(SMUIO, 0, mmCKSVII2C_IC_DATA_CMD, reg);
            bytes_sent += 1; numbytes -= 1;
        }
    }
    if ret == I2C_OK { ret = smu_v11_0_i2c_poll_tx_status(control); }
    ret
}

unsafe fn smu_v11_0_i2c_receive(control: *mut i2c_adapter, address: u16, data: *mut u8, mut numbytes: u32, i2c_flag: u32) -> u32 {
    let mut bytes_received = 0;
    let mut ret = I2C_OK;
    smu_v11_0_i2c_set_address(control, address);
    smu_v11_0_i2c_enable(control, true);
    while numbytes > 0 {
        let mut reg = 0;
        smu_v11_0_i2c_clear_status(control);
        reg = reg_set_field(reg, 0, 0, 0);
        reg = reg_set_field(reg, 0, 0, 1);
        if numbytes == 1 && (i2c_flag & I2C_M_STOP) != 0 { reg = reg_set_field(reg, 0, 0, 1); }
        if bytes_received == 0 && (i2c_flag & I2C_X_RESTART) != 0 { reg = reg_set_field(reg, 0, 0, 1); }
        wreg32_soc15(SMUIO, 0, mmCKSVII2C_IC_DATA_CMD, reg);
        ret = smu_v11_0_i2c_poll_rx_status(control);
        if ret != I2C_OK { break; }
        reg = rreg32_soc15(SMUIO, 0, mmCKSVII2C_IC_DATA_CMD);
        *data.add(bytes_received as usize) = reg_get_field(reg, 0, 0) as u8;
        bytes_received += 1; numbytes -= 1;
    }
    ret
}

unsafe fn smu_v11_0_i2c_abort(control: *mut i2c_adapter) {
    let mut reg = reg_set_field(0, 0, 0, 1);
    wreg32_soc15(SMUIO, 0, mmCKSVII2C_IC_ENABLE, reg);
    reg = reg_set_field(reg, 0, 0, 1);
    wreg32_soc15(SMUIO, 0, mmCKSVII2C_IC_ENABLE, reg);
}

unsafe fn smu_v11_0_i2c_activity_done(control: *mut i2c_adapter) -> bool {
    const IDLE_TIMEOUT: u32 = 1024;
    let enable_status = rreg32_soc15(SMUIO, 0, mmCKSVII2C_IC_ENABLE_STATUS);
    let enable = rreg32_soc15(SMUIO, 0, mmCKSVII2C_IC_ENABLE);
    if reg_get_field(enable, 0, 0) == 0 && reg_get_field(enable_status, 0, 0) == 1 { smu_v11_0_i2c_abort(control); }
    else if reg_get_field(enable, 0, 0) == 0 { return true; }
    let mut timeout_count = 0;
    loop {
        let activity = rreg32_soc15(SMUIO, 0, mmCKSVII2C_IC_CLR_ACTIVITY);
        if reg_get_field(activity, 0, 0) == 0 { return true; }
        timeout_count += 1;
        if timeout_count >= IDLE_TIMEOUT { return false; }
    }
}

unsafe fn smu_v11_0_i2c_init(control: *mut i2c_adapter) {
    smu_v11_0_i2c_set_clock_gating(control, false);
    if !smu_v11_0_i2c_activity_done(control) { drm_warn(b"I2C busy !\0".as_ptr()); }
    if smu_v11_0_i2c_enable(control, false) != I2C_OK as i32 { smu_v11_0_i2c_abort(control); }
    smu_v11_0_i2c_configure(control);
    smu_v11_0_i2c_set_clock(control);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
