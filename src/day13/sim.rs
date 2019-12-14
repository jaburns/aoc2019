use std::sync::mpsc::{Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::intcode::vm::{IntCodeMachine,RunResult};
use super::state::Game;

const FRAME_MILLIS: u64 = 8;

pub fn run(game_tape: &[i64], tick_tx: Sender<()>, shared_state: Arc<Mutex<Game>>) {
    let mut machine = IntCodeMachine::new(game_tape);

    machine.poke(0, 2);

    loop {
        match machine.run() {
            RunResult::Halted => break,

            RunResult::RequiresInput => {
                {
                    let game = shared_state.lock().unwrap();
                    machine.provide_input(game.get_best_joystick_dir());
                }

                if let Err(_) = tick_tx.send(()) {
                    break;
                }

                thread::sleep(Duration::from_millis(FRAME_MILLIS));
            },

            RunResult::ProvidingOutput(x) => {
                let y = machine.run_and_get_output().unwrap();
                let t = machine.run_and_get_output().unwrap();

                let mut game = shared_state.lock().unwrap();
                game.write_state(x, y, t);
            },
        }
    }
}