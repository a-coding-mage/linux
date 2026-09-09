extern "C" {
    pub fn vexpress_smp_init_ops() -> bool;
    pub fn vexpress_flags_set(data: u32);

    pub static vexpress_smp_dt_ops: smp_operations;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
