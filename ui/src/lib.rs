use std::{
    io,
    sync::{
        mpsc::Sender
    }
};

use types::{
    Event,
    EventType
};

fn stream_audio(

) {

}

fn detect_wake_word(

) {

}

pub fn run_ui_handle(
    tx: Sender<Event>
) {
    loop {
        let mut input_stream = String::new();
        io::stdin()
            .read_line(&mut input_stream)
            .expect("Failed to read line");

        let wake_word = "MIMO";
        if input_stream.contains(wake_word) {
            let input_event = Event {
                event_type: EventType::UserEvent
            };
            tx.send(input_event).unwrap();
        }
    }
}
