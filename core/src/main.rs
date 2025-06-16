use std::{sync::{mpsc::{channel, Sender}, Arc, RwLock}, thread::{self, sleep}, time::{Duration}};

use types::{Event, State, EventType};

fn boot_heart_beat(
    tx: Sender<Event>
) {
    thread::spawn(move||{
        loop {
            let heart_beat_event = Event {
                event_type: EventType::HeartBeat
            };
            tx.send(heart_beat_event).unwrap();
            sleep(Duration::from_millis(1000));
        }
    });
}

fn boot_services_handle(
    tx: Sender<Event>,
    state: Arc<RwLock<State>>
) {
    /*thread::spawn(move || {
        ui::run_services_handle(
            tx,
            state
        );
    });*/
}

fn boot_automations_handle(
    tx: Sender<Event>,
    state: Arc<RwLock<State>>
) {
    /*thread::spawn(move || {
        ui::run_automations_handle(
            tx,
            state
        );
    });*/
}

fn boot_ai_handle(
    tx: Sender<Event>,
    state: Arc<RwLock<State>>
) {
    /*thread::spawn(move || {
        ui::run_ai_handle(
            tx,
            state
        );
    });*/
}

fn boot_ui_handle(
    tx: Sender<Event>
) {
    thread::spawn(move||{
        ui::run_ui_handle(
            tx
        );
    });
}

fn main() {
    let (tx, rx) = channel();
    let app_state = State {
        heart_beats_since_start: 0
    };
    let app_state: Arc<RwLock<State>> = Arc::new(RwLock::new(app_state));

    boot_heart_beat(tx.clone());
    boot_services_handle(tx.clone(), Arc::clone(&app_state));
    boot_automations_handle(tx.clone(), Arc::clone(&app_state));
    boot_ai_handle(tx.clone(), Arc::clone(&app_state));
    boot_ui_handle(tx.clone());

    loop {
        let event = rx.recv().unwrap();

        match event.event_type {
            EventType::HeartBeat => {
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
