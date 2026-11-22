use mirajazz::{error::MirajazzError, types::DeviceInput};

use crate::mappings::{KEY_COUNT};

pub fn process_input(input: u8, state: u8) -> Result<DeviceInput, MirajazzError> {
//    log::info!("Processing input: {}, {}", input, state);
    log::info!("RAW input={:#04x} state={:#04x}", input, state);

    match input {
//        (0..=6) | 0x25 | 0x30 | 0x31 => read_button_press(input, state),
//        0x90 | 0x91 | 0x50 | 0x51 | 0x60 | 0x61 => read_encoder_value(input),
//        0x33..=0x35 => read_encoder_press(input, state),
        0 | 1..=12 => read_button_press(input, state), // provisional
        _ => Err(MirajazzError::BadData),
    }
}

fn read_button_states(states: &[u8]) -> Vec<bool> {
    let mut bools = vec![];

    for i in 0..KEY_COUNT {
        bools.push(states[i + 1] != 0);
    }

    bools
}

fn read_button_press(input: u8, state: u8) -> Result<DeviceInput, MirajazzError> {
    let mut button_states = vec![0x01];
    button_states.extend(vec![0u8; KEY_COUNT + 1]);

    if input == 0 {
        return Ok(DeviceInput::ButtonStateChange(read_button_states(
            &button_states,
        )));
    }

    let pressed_index: usize = match input {
        // Six buttons with displays
//        (1..=6) => input as usize,
        // Three buttons without displays
//        0x25 => 7,
//        0x30 => 8,
//        0x31 => 9,
        1..=12 => input as usize,
        _ => return Err(MirajazzError::BadData),
    };

    button_states[pressed_index] = state;

    Ok(DeviceInput::ButtonStateChange(read_button_states(
        &button_states,
    )))
}


