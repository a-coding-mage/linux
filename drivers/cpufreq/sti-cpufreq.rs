// SPDX-License-Identifier: GPL-2.0-only
/*
 * Match running platform with pre-defined OPP values for CPUFreq
 *
 * Author: Ajit Pal Singh <ajitpal.singh@st.com>
 *         Lee Jones <lee.jones@linaro.org>
 *
 * Copyright (C) 2015 STMicroelectronics (R&D) Limited
 */

// Linux kernel dependencies supplied by other translation units.

const VERSION_ELEMENTS: usize = 3;
const MAX_PCODE_NAME_LEN: usize = 16;

const VERSION_SHIFT: u32 = 28;
const HW_INFO_INDEX: usize = 1;
const MAJOR_ID_INDEX: usize = 1;
const MINOR_ID_INDEX: usize = 2;
const DEFAULT_VERSION: i32 = 31;

const PCODE: usize = 0;
const SUBSTRATE: usize = 1;
const DVFS_MAX_REGFIELDS: usize = 2;

#[repr(C)]
struct StiCpufreqDdata {
    cpu: *mut device,
    syscfg_eng: *mut regmap,
    syscfg: *mut regmap,
}

static mut ddata: StiCpufreqDdata = StiCpufreqDdata {
    cpu: core::ptr::null_mut(),
    syscfg_eng: core::ptr::null_mut(),
    syscfg: core::ptr::null_mut(),
};

unsafe fn sti_cpufreq_fetch_major() -> i32 {
    let np = (*ddata.cpu).of_node;
    let dev = ddata.cpu;
    let mut major_offset: u32 = 0;
    let mut socid: u32 = 0;
    let ret = of_property_read_u32_index(np, "st,syscfg", MAJOR_ID_INDEX, &mut major_offset);
    if ret != 0 {
        dev_err(dev, "No major number offset provided in %pOF [%d]\n", np, ret);
        return ret;
    }
    let ret = regmap_read(ddata.syscfg, major_offset, &mut socid);
    if ret != 0 {
        dev_err(dev, "Failed to read major number from syscon [%d]\n", ret);
        return ret;
    }
    (((socid >> VERSION_SHIFT) & 0xf) + 1) as i32
}

unsafe fn sti_cpufreq_fetch_minor() -> i32 {
    let dev = ddata.cpu;
    let np = (*dev).of_node;
    let mut minor_offset: u32 = 0;
    let mut minid: u32 = 0;
    let ret = of_property_read_u32_index(np, "st,syscfg-eng", MINOR_ID_INDEX, &mut minor_offset);
    if ret != 0 {
        dev_err(dev, "No minor number offset provided %pOF [%d]\n", np, ret);
        return ret;
    }
    let ret = regmap_read(ddata.syscfg_eng, minor_offset, &mut minid);
    if ret != 0 {
        dev_err(dev, "Failed to read the minor number from syscon [%d]\n", ret);
        return ret;
    }
    (minid & 0xf) as i32
}

unsafe fn sti_cpufreq_fetch_regmap_field(
    reg_fields: *const reg_field,
    hw_info_offset: i32,
    field: usize,
) -> i32 {
    let mut reg_field = *reg_fields.add(field);
    let dev = ddata.cpu;
    let mut value: u32 = 0;
    reg_field.reg = hw_info_offset as u32;
    let regmap_field = devm_regmap_field_alloc(dev, ddata.syscfg_eng, reg_field);
    if IS_ERR(regmap_field) {
        dev_err(dev, "Failed to allocate reg field\n");
        return PTR_ERR(regmap_field);
    }
    let ret = regmap_field_read(regmap_field, &mut value);
    if ret != 0 {
        dev_err(dev, "Failed to read %s code\n", if field != 0 { "SUBSTRATE" } else { "PCODE" });
        return ret;
    }
    value as i32
}

static sti_stih407_dvfs_regfields: [reg_field; DVFS_MAX_REGFIELDS] = [
    REG_FIELD(0, 16, 19),
    REG_FIELD(0, 0, 2),
];

unsafe fn sti_cpufreq_match() -> *const reg_field {
    if of_machine_is_compatible("st,stih407") != 0
        || of_machine_is_compatible("st,stih410") != 0
        || of_machine_is_compatible("st,stih418") != 0
    {
        return sti_stih407_dvfs_regfields.as_ptr();
    }
    core::ptr::null()
}

unsafe fn sti_cpufreq_set_opp_info() -> i32 {
    let dev = ddata.cpu;
    let np = (*dev).of_node;
    let reg_fields;
    let mut hw_info_offset: u32 = 0;
    let mut version: [u32; VERSION_ELEMENTS] = [0; VERSION_ELEMENTS];
    let mut pcode: i32;
    let mut substrate: i32;
    let mut major: i32;
    let mut minor: i32;
    let mut name = [0i8; MAX_PCODE_NAME_LEN];
    let config = dev_pm_opp_config {
        supported_hw: version.as_mut_ptr(),
        supported_hw_count: VERSION_ELEMENTS,
        prop_name: name.as_mut_ptr(),
    };

    reg_fields = sti_cpufreq_match();
    if reg_fields.is_null() {
        dev_err(dev, "This SoC doesn't support voltage scaling\n");
        return -ENODEV;
    }
    let ret = of_property_read_u32_index(np, "st,syscfg-eng", HW_INFO_INDEX, &mut hw_info_offset);
    if ret != 0 {
        dev_warn(dev, "Failed to read HW info offset from DT\n");
        substrate = DEFAULT_VERSION;
        pcode = 0;
    } else {
        pcode = sti_cpufreq_fetch_regmap_field(reg_fields, hw_info_offset as i32, PCODE);
        if pcode < 0 {
            dev_warn(dev, "Failed to obtain process code\n");
            pcode = 0;
        }
        substrate = sti_cpufreq_fetch_regmap_field(reg_fields, hw_info_offset as i32, SUBSTRATE);
        if substrate != 0 {
            dev_warn(dev, "Failed to obtain substrate code\n");
            substrate = DEFAULT_VERSION;
        }
    }
    major = sti_cpufreq_fetch_major();
    if major < 0 { dev_err(dev, "Failed to obtain major version\n"); major = DEFAULT_VERSION; }
    minor = sti_cpufreq_fetch_minor();
    if minor < 0 { dev_err(dev, "Failed to obtain minor version\n"); minor = DEFAULT_VERSION; }
    snprintf(name.as_mut_ptr(), MAX_PCODE_NAME_LEN, "pcode%d", pcode);
    version[0] = 1u32.wrapping_shl(major as u32);
    version[1] = 1u32.wrapping_shl(minor as u32);
    version[2] = 1u32.wrapping_shl(substrate as u32);
    let opp_token = dev_pm_opp_set_config(dev, &config);
    if opp_token < 0 { dev_err(dev, "Failed to set OPP config\n"); return opp_token; }
    dev_dbg(dev, "pcode: %d major: %d minor: %d substrate: %d\n", pcode, major, minor, substrate);
    dev_dbg(dev, "version[0]: %x version[1]: %x version[2]: %x\n", version[0], version[1], version[2]);
    0
}

unsafe fn sti_cpufreq_fetch_syscon_registers() -> i32 {
    let dev = ddata.cpu;
    let np = (*dev).of_node;
    ddata.syscfg = syscon_regmap_lookup_by_phandle(np, "st,syscfg");
    if IS_ERR(ddata.syscfg) { dev_err(dev, "\"st,syscfg\" not supplied\n"); return PTR_ERR(ddata.syscfg); }
    ddata.syscfg_eng = syscon_regmap_lookup_by_phandle(np, "st,syscfg-eng");
    if IS_ERR(ddata.syscfg_eng) { dev_err(dev, "\"st,syscfg-eng\" not supplied\n"); return PTR_ERR(ddata.syscfg_eng); }
    0
}

unsafe fn sti_cpufreq_init() -> i32 {
    if of_machine_is_compatible("st,stih407") == 0 && of_machine_is_compatible("st,stih410") == 0 && of_machine_is_compatible("st,stih418") == 0 { return -ENODEV; }
    ddata.cpu = get_cpu_device(0);
    if ddata.cpu.is_null() { dev_err(ddata.cpu, "Failed to get device for CPU0\n"); }
    else if !of_property_present((*ddata.cpu).of_node, "operating-points-v2") {
        dev_err(ddata.cpu, "OPP-v2 not supported\n");
    } else if sti_cpufreq_fetch_syscon_registers() == 0 && sti_cpufreq_set_opp_info() == 0 {
        platform_device_register_simple("cpufreq-dt", -1, core::ptr::null(), 0);
        return 0;
    }
    dev_err(ddata.cpu, "Not doing voltage scaling\n");
    platform_device_register_simple("cpufreq-dt", -1, core::ptr::null(), 0);
    0
}

// module_init(sti_cpufreq_init);

static sti_cpufreq_of_match: [of_device_id; 4] = [
    of_device_id { compatible: "st,stih407" },
    of_device_id { compatible: "st,stih410" },
    of_device_id { compatible: "st,stih418" },
    of_device_id { compatible: core::ptr::null() },
];

// MODULE_DEVICE_TABLE(of, sti_cpufreq_of_match);
// MODULE_DESCRIPTION("STMicroelectronics CPUFreq/OPP driver");
// MODULE_AUTHOR("Ajitpal Singh <ajitpal.singh@st.com>");
// MODULE_AUTHOR("Lee Jones <lee.jones@linaro.org>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
