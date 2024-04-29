let targets = #{
    stm32f411ce: #{
        arch: "arm",
        rust_target: "thumbv7em-none-eabihf",
        probe_chip: "STM32F411CEUx",
        openocd_config: "stm32f4x"
    },

    esp32c3: #{
        arch: "riscv32",
        rust_target: "riscv32imc-unknown-none-elf",
        probe_chip: "esp32c3"
    },
};

let target = variable::get("chip");
let target_properties = targets.get(target);
for key in target_properties.keys() {
    variable::set(key, target_properties.get(key));
}
