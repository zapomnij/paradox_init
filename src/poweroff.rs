fn main() {
    paradox_init::run_cmd(&"initctl send init_operate_halting poweroff".to_string()).unwrap();
}