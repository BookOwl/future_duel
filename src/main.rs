use std::thread;
use std::time;

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
}

/// A struct implements the Player trait in order to take part in the duel.
trait Player {
    /// Returns the name of the player/bot.
    fn name(&self) -> String;
    /// Returns an Action to be performed.
    fn act(&mut self) -> Action;
    /// This method is called to inform the player of the opponent's actions.
    /// You don't have to implement this method, 
    /// the default implementation just ignores the opponent's action.
    fn perceive(&mut self, Action) -> () {
        // Do nothing.
        ()
    }
}

struct PlayerRunner {
    player: Box<Player>,
    ammo: i32,
    has_metal_shield_up: bool,
    has_thermal_shield_up: bool,
}
impl PlayerRunner {
    fn new(player: Box<Player>) -> PlayerRunner {
        PlayerRunner {
            player,
            ammo: 0,
            has_metal_shield_up: false,
            has_thermal_shield_up: false,
        }
    }
    fn act(&mut self) -> Result<Action, String> {
        let action = self.player.act();
        match action {
            Action::LoadAmmo => {
                self.ammo += 1;
                Ok(action)
            }
            Action::DefendBullet => {
                self.has_metal_shield_up = true;
                Ok(action)
            }
            Action::DefendPlasma => {
                self.has_thermal_shield_up = true;
                Ok(action)
            }
            Action::FireBullet => {
                self.ammo -= 1;
                if self.ammo < 0 {
                    Err(format!("{} blew up by trying to fire a bullet without having enough ammo!", self.player.name()))
                } else {
                    Ok(action)
                }
            }
            Action::FirePlasma => {
                self.ammo -= 2;
                if self.ammo < 0 {
                    Err(format!("{} blew up by trying to fire a plasma burst without having enough ammo!", self.player.name()))
                } else {
                    Ok(action)
                }
            }
        }
    }
    fn perceive(&mut self, opponent_action: Action) {
        self.player.perceive(opponent_action);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DuelResult {
    Player1Wins,
    Player2Wins,
    Tie,
}

struct Duel {
    player1: PlayerRunner,
    player2: PlayerRunner,
}
impl Duel {
    fn new(player1: PlayerRunner, player2: PlayerRunner) -> Duel {
        Duel { 
            player1,
            player2,
        }
    }
    fn run(&mut self) -> DuelResult {
        use Action::*;
        for _ in 0..100 {
            let player1_action = match self.player1.act() {
                Ok(a) => a,
                Err(msg) => {
                    println!("{}", msg);
                    return DuelResult::Player2Wins
                },
            };
            let player2_action = match self.player2.act() {
                Ok(a) => a,
                Err(msg) => {
                    println!("{}", msg);
                    return DuelResult::Player1Wins
                },
            };
        }
        // No one won after 100 turns, so the duel is a tie.
        DuelResult::Tie
    }
}

fn main() {
    println!("Hello, world!");
}
