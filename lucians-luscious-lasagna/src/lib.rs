// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn expected_minutes_in_oven() -> i32 {
    40
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    let remaining_time = expected_minutes_in_oven() - actual_minutes_in_oven;
    println!(
        "calculate remaining minutes in oven given actual minutes in oven: {}",
        remaining_time
    );
    remaining_time
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    let time_layer_result = number_of_layers * 2;
    println!(
        "calculate preparation time in minutes for number of layers: {}",
        time_layer_result
    );
    time_layer_result
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    let curr_time = expected_minutes_in_oven() - remaining_minutes_in_oven(actual_minutes_in_oven);
    let total_cooking_time = preparation_time_in_minutes(number_of_layers) + curr_time;
    println!(
        "calculate elapsed time in minutes for number of layers {} and actual minutes in oven {}",
        number_of_layers,
        actual_minutes_in_oven
    );
    total_cooking_time
}
