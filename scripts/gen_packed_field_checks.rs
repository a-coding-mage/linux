// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2024, Intel Corporation

const MAX_PACKED_FIELD_SIZE: i32 = 50;

fn main() {
    /* The first macro doesn't need a 'do {} while(0)' loop */
    print!("#define CHECK_PACKED_FIELDS_1(fields) \\\n+");
    print!("\tCHECK_PACKED_FIELD(fields, 0)\n\n");

    /* Remaining macros require a do/while loop, and are implemented
     * recursively by calling the previous iteration's macro.
     */
    for i in 2..=MAX_PACKED_FIELD_SIZE {
        print!("#define CHECK_PACKED_FIELDS_{}(fields) do {{ \\\n+", i);
        print!("\tCHECK_PACKED_FIELDS_{}(fields); \\\n+", i - 1);
        print!("\tCHECK_PACKED_FIELD(fields, {}); \\\n+", i - 1);
        print!("}} while (0)\n\n");
    }

    print!("#define CHECK_PACKED_FIELDS(fields) \\\n+");

    for i in 1..=MAX_PACKED_FIELD_SIZE {
        print!(
            "\t__builtin_choose_expr(ARRAY_SIZE(fields) == {}, ({{ CHECK_PACKED_FIELDS_{}(fields); }}), \\\n+",
            i, i
        );
    }

    print!(
        "\t({{ BUILD_BUG_ON_MSG(1, \"CHECK_PACKED_FIELDS() must be regenerated to support array sizes larger than {}.\"); }}) \\\n+",
        MAX_PACKED_FIELD_SIZE
    );

    for _ in 1..=MAX_PACKED_FIELD_SIZE {
        print!(")");
    }

    println!();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
