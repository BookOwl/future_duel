extern crate subprocess;
extern crate clap;
use clap::*;
use subprocess::{Exec, Redirection};
use std::thread;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex, Barrier};
use std::time;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::io::{Read, Write, BufRead, BufReader};

/// An Action represents the actions a player can make.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    /// LoadAmmo loads 1 ammo into the player's gun.
    LoadAmmo,
    /// FireBullet uses 1 ammo and fires a bullet at the other player.
    /// If the player has no ammo loaded, the gun explodes and the player dies.
    FireBullet,
    /// FireBullet uses 2 ammo and fires a plasma burst at the other player.
    /// If the player has 0 or 1 ammo loaded, the gun explodes and the player dies.
    FirePlasma,
    /// DefendBullet raises a metal shield that defends against bullets, but not plasma.
    DefendBullet,
    /// DefendPlasma raises a thermal shield that defends against plasma, but not bullets.
    DefendPlasma,
    Dead,
}

impl Action {
    fn from_byte(byte: u8) -> Result<Action, String> {
        match byte {
            0x30 => Ok(Action::LoadAmmo),
            0x31 => Ok(Action::FireBullet),
            0x32 => Ok(Action::FirePlasma),
            0x33 => Ok(Action::DefendBullet),
            0x34 => Ok(Action::DefendPlasma),
            _ => Err(format!("Invalid action: {}", byte)),
        }
    }
    fn to_byte(&self) -> u8 {
        match *self {
            Action::LoadAmmo => 0x30,
            Action::FireBullet => 0x31,
            Action::FirePlasma => 0x32,
            Action::DefendBullet => 0x33,
            Action::DefendPlasma => 0x34,
            Action::Dead => unreachable!(),
        }
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DuelResult {
    Player1Wins,
    Player2Wins,
    Tie,
}

struct BotState {
    ammo: i32,
    metal_shield: bool,
    thermal_shield: bool,
}

fn read_action(stdout: &mut std::fs::File) -> Action {
    let mut buf = [0; 1];
    stdout.read_exact(&mut buf);
    Action::from_byte(buf[0]).unwrap()
}

fn read_ready(stdout: &mut std::fs::File) {
    let mut buf = [0; 1];
    stdout.read_exact(&mut buf);
    assert_eq!(buf[0], 0x72, "Invalid starting byte!");
}

fn stream_stdout(mut out: std::fs::File) -> Receiver<Action> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        loop {
            tx.send(read_action(&mut out));
        }
    });
    rx
}

fn run_bot(name: String, barrier: Arc<Barrier>, action_sender: Sender<Action>, opponent_act_recv: Receiver<Action>) {
    thread::spawn(move || {
        let bot = Exec::cmd(name)
                    .stdin(Redirection::Pipe)
                    .stdout(Redirection::Pipe)
                    .popen()
                    .expect("Could not start bot1!");
        let mut bot_state = BotState {
            ammo: 0,
            metal_shield: false,
            thermal_shield: false,
        };
        let mut stdin = if let Some(ref f) = bot.stdin {
            f.try_clone().unwrap()
        } else {
            panic!("Couldn't get bot1 stdin!")
        };
        let mut stdout = if let Some(ref f) = bot.stdout {
            f.try_clone().unwrap()
        } else {
            panic!("Couldn't get bot1 stdin!")
        };
        read_ready(&mut stdout);
        barrier.wait();
        let actions = stream_stdout(stdout);
        loop {
            if let Ok(act) = actions.try_recv() {
                action_sender.send(act).unwrap();
            }
            if let Ok(act) = opponent_act_recv.try_recv() {
                match act {
                    Action::FireBullet => {
                        if !bot_state.metal_shield {
                            action_sender.send(Action::Dead).unwrap();
                        }
                    },
                    Action::FirePlasma=> {
                        if !bot_state.thermal_shield {
                            action_sender.send(Action::Dead).unwrap();
                        }
                    },
                    _ => (),
                }
                stdin.write_all(&[act.to_byte()]).unwrap();
            }
        }
    });
}

fn run_duel(bot1: String, bot2: String) -> DuelResult {
    let barrier = Arc::new(Barrier::new(3));
    let (bot1_act_tx, bot1_act_recv) = channel::<Action>();
    let (bot1_opponent_act_tx, bot1_opponent_act_recv) = channel::<Action>();
    let (bot2_act_tx, bot2_act_recv) = channel::<Action>();
    let (bot2_opponent_act_tx, bot2_opponent_act_recv) = channel::<Action>();
    run_bot(bot1, barrier.clone(), bot1_act_tx, bot1_opponent_act_recv);
    run_bot(bot2, barrier.clone(), bot2_act_tx, bot2_opponent_act_recv);
    barrier.wait();
    let start_time = time::Instant::now();
    loop {
        if let Ok(bot1_act) = bot1_act_recv.try_recv() {
            if bot1_act == Action::Dead {
                return DuelResult::Player2Wins
            }
            bot2_opponent_act_tx.send(bot1_act).unwrap();
        }
        if let Ok(bot2_act) = bot2_act_recv.try_recv() {
            if bot2_act == Action::Dead {
                return DuelResult::Player1Wins
            }
            bot1_opponent_act_tx.send(bot2_act).unwrap();
        }
        if start_time.elapsed().as_secs() >= 15 {
            return DuelResult::Tie;
        }
    }
}
fn main() {
    let matches = clap_app!(future_duel =>
        (version: "1.0")
        (author: "Matthew S.")
        (about: "Runs the future duel")
        (@arg BOT1: +required "Command to run bot 1")
        (@arg BOT2: +required "Command to run bot 2")
    ).get_matches();
    let bot1 = matches.value_of("BOT1").unwrap().to_owned();
    let bot2 = matches.value_of("BOT2").unwrap().to_owned();
    let result = run_duel(bot1.clone(), bot2.clone());
    println!("{}", match result {
        DuelResult::Player1Wins => "Player 1 won!",
        DuelResult::Player2Wins => "Player 2 won!",
        DuelResult::Tie => "It was a tie!"
    })
}
