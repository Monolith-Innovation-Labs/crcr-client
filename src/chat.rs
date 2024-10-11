use std::fmt::Debug;
use std::fs;
use slint::ComponentHandle;
use tokio::sync::mpsc;
use crcr_command_derive::Command;
use crcr_command::{parse_command, Command};
use crate::{AppWindow, Connection};

#[derive(Command, Debug)]
pub struct Money(pub u32);

#[derive(Command, Debug)]
pub struct Handshake(pub u16);

pub fn connect_chat_component(ui: &AppWindow) {
    let (pda_tx, mut pda_rx) = mpsc::channel(32);
    //let (irc_tx, mut irc_rx) = mpsc::channel::<T>(32);
    let (stop_tx, mut stop_rx) = mpsc::channel::<()>(1);

    ui.set_chat_irc_status(Connection::Disconnected);
    ui.set_chat_pda_status(Connection::Disconnected);

    spawn_pda_loop(pda_tx.clone());
    handle_ui_updates(ui, pda_rx);
}

fn spawn_pda_loop(tx: mpsc::Sender<Connection>) {
    let input  = "./gamedata/configs/crc_output.txt";
    let output  = "./gamedata/configs/crc_input.txt";

    tokio::spawn(async move {
        if let Err(err) = tx.send(Connection::Connecting).await {
            println!("Error: {}", err);
        }

        loop {
            let data = fs::read_to_string(input).unwrap();

            for line in data.split('\n') {
                if line.is_empty() {
                    continue;
                }

                if let Ok(Handshake(_)) = parse_command(line.trim()) {
                    if let Err(err) = tx.send(Connection::Connected).await {
                        println!("Error: {}", err);
                    }
                }
            }
        }
    });
}

fn handle_ui_updates(ui: &AppWindow, mut rx: mpsc::Receiver<Connection>) {
    let handle_weak = ui.as_weak();

    tokio::spawn(async move {
        while let Some(status) = rx.recv().await {
            let handle_copy = handle_weak.clone();

            slint::invoke_from_event_loop(move || {
                if let Some(handle) = handle_copy.upgrade() {
                    handle.set_chat_irc_status(status);
                }
            }).unwrap();
        }
    });
}
