use bevy::prelude::*;
use std::{
    str::FromStr,
    sync::{mpsc, Mutex},
    thread,
};

use crate::{
    items::{list::Item, stack::ItemStack},
    player::{inventory::ui::UpdateSlotEvent, money::Money, Player},
};

#[derive(Event)]
pub struct CommandEvent {
    pub msg: String,
}

pub struct CommandsPlugin;
impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || loop {
            let mut buff = String::new();
            let stdin = std::io::stdin();
            stdin.read_line(&mut buff).unwrap();
            if let Some(msg) = buff.lines().collect::<Vec<_>>().first() {
                tx.send(msg.to_string()).unwrap();
            }
        });

        let rx_ref = Mutex::new(rx);

        let commands = move |mut event_writer: EventWriter<CommandEvent>| {
            if let Ok(msg) = rx_ref.lock().unwrap().try_recv() {
                event_writer.send(CommandEvent { msg })
            }
        };

        app.add_systems(Update, (commands, handle_commands))
            .add_event::<CommandEvent>();
    }
}

pub fn handle_commands(
    mut command_event: EventReader<CommandEvent>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
    mut update_slot_event: EventWriter<UpdateSlotEvent>,
) {
    let mut handle_command = |command: &CommandEvent| -> Result<(), String> {
        let command_data = command.msg.split(' ').collect::<Vec<_>>();
        let command_name = command_data[0];
        let args = &command_data[1..];

        match command_name {
            "give" => {
                let (_, mut player) = player_query
                    .get_single_mut()
                    .map_err(|_| "Player not found")?;

                let item_name = *args.first().ok_or("Item not found")?;

                let item = Item::from_str(item_name).map_err(|_| "Item not found")?;

                let count = if let Some(count_str) = args.get(1) {
                    let count: u16 = count_str
                        .parse()
                        .map_err(|_| "Second arg must be a number")?;
                    (count - 1) as u8
                } else {
                    0
                };

                player
                    .inventory
                    .push_item_stack(&mut Some(ItemStack { item, count }), &mut update_slot_event);

                Ok(())
            }

            "tp" => {
                let (mut transform, _) = player_query
                    .get_single_mut()
                    .map_err(|_| "Player not found")?;

                let poses = args
                    .iter()
                    .filter_map(|s| s.parse().ok())
                    .collect::<Vec<_>>();

                let [x, y] = poses.try_into().map_err(|_| "Please submit a valid pos")?;
                transform.translation = Vec2::new(x, y).extend(Player::EXTEND);

                Ok(())
            }

            "money" => {
                let (_, mut player) = player_query
                    .get_single_mut()
                    .map_err(|_| "Player not found")?;

                let action = args.first().ok_or("Please specify action")?;
                let amount = args
                    .get(1)
                    .ok_or("Please specify amount")?
                    .parse()
                    .map_err(|_| "Arg must be a valid int")?;

                match *action {
                    "add" => player.money += amount,
                    "remove" => {
                        if !player.money.try_remove(amount) {
                            player.money = Money::default();
                        }
                    }
                    _ => return Err("Must be a valid action (add, remove)".into()),
                }

                Ok(())
            }

            _ => Err(format!("Command not found {command_name}")),
        }
    };

    for command in command_event.read() {
        if let Err(err) = handle_command(command) {
            error!("{err}");
        }
    }
}
