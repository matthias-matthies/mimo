use std::{sync::{mpsc::{channel, Sender}, Arc, RwLock}, thread::{self, sleep}, time::{Duration}};

#[derive(Debug, Clone, Copy)]
enum EventType {
    SystemEvent,
    UserEvent,
    HeartBeat
}

#[derive(Debug, Clone, Copy)]
struct Event {
    event_type: EventType,
    debug: u128
}

#[derive(Debug)]
struct State {
    heart_beats_since_start: u32,
}

fn run_ui_handle(
    tx: Sender<Event>,
    app_state: Arc<RwLock<State>>
) {
    let mut i = 0;
    loop {
        let app_state_guard = app_state.read().unwrap();
        let test_event = Event {
            event_type: EventType::UserEvent,
            debug: i
        };
        if app_state_guard.heart_beats_since_start % 19 == 0 {
            tx.send(test_event).unwrap();
            sleep(Duration::from_millis(1000));
        }
        i = i + 1;
        drop(app_state_guard);
    }
}

fn main() {
    println!("Jan Tanner??");

    let (tx, rx) = channel();
    let app_state = State {
        heart_beats_since_start: 0
    };
    let app_state: Arc<RwLock<State>> = Arc::new(RwLock::new(app_state));

    let heart_beat = tx.clone();
    thread::spawn(move || {
        loop {
            let heart_beat_event = Event {
                event_type: EventType::HeartBeat,
                debug: 0
            };
            heart_beat.send(heart_beat_event).unwrap();
            sleep(Duration::from_millis(1000));
        }
    });

    // UI
    let ui_tx = tx.clone();
    let ui_app_state_reader = Arc::clone(&app_state);
    let ui_handle = thread::spawn(move || {
        run_ui_handle(
            ui_tx,
            ui_app_state_reader
        );
    });

    // Automations
    /*let automations_handle = thread::spawn(move || {
        run_service_handle(

        );
    });*/

    // Services
    /*let services_handle = thread::spawn(move || {
        run_service_handle(

        );
    });*/

    // AI
    /*let ai_handle = thread::spawn(move || {
        run_service_handle(

        );
    });*/

    loop {
        let event = rx.recv().unwrap();

        match event.event_type {
            EventType::HeartBeat => {
                println!("Beat");
                let mut app_state_guard = app_state.write().unwrap();
                app_state_guard.heart_beats_since_start = app_state_guard.heart_beats_since_start + 1;
            }
            EventType::SystemEvent => {
                let app_state_guard = app_state.read().unwrap();
                println!("SYSTEM: {:#?}, {:#?}", event, app_state_guard);
            }
            EventType::UserEvent => {
                let app_state_guard = app_state.read().unwrap();
                println!("USER: {:#?}, {:#?}", event, app_state_guard);
            }
        }
    }
}
