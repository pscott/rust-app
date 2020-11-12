#![no_std]
#![no_main]

use nanos_ui::ui;
use nanos_sdk::io;
use libapp::handle_apdu;

#[no_mangle]
extern "C" fn sample_main() {
    let mut comm = io::Comm::new();

    loop {
        ui::SingleMessage::new("W e l c o m e").show();

        comm.io_exch(0x80);

        match handle_apdu(&mut comm) {
            Ok(()) => comm.set_status_word(io::StatusWords::OK),
            Err(sw) => comm.set_status_word(sw),
        }
    }
}