use extism_pdk::{info, plugin_fn, FnResult};
use hank_pdk::{Hank, PluginMetadata};
use hank_types::database::PreparedStatement;
use hank_types::message::Message;

#[plugin_fn]
pub fn plugin() -> FnResult<()> {
    let mut hank = Hank::new(PluginMetadata {
        name: "sample-rust-plugin",
        description: "A sample plugin to demonstrate some functionality.",
        version: "0.1.0",
        ..Default::default()
    });

    hank.register_install_handler(my_install);
    hank.register_initialize_handler(my_initialize);
    hank.register_message_handler(my_handle_message);
    hank.register_command_handler(my_handle_command);

    hank.start()
}

pub fn my_install() {
    let stmt = PreparedStatement {
        sql: "CREATE TABLE IF NOT EXISTS people (name TEXT, age INTEGER)".into(),
        ..Default::default()
    };
    Hank::db_query(stmt);
}

pub fn my_initialize() {
    info!("initializing...");
}

pub fn my_handle_message(message: Message) {
    info!("{}: {}", message.author_name, message.content);
}

pub fn my_handle_command(command: Message) {
    if command.content == "ping" {
        let response = Message {
            content: "Pong!".into(),
            ..command
        };
        Hank::send_message(response);
    }

    let people = Hank::db_query(PreparedStatement {
        sql: "SELECT * from people".into(),
        ..Default::default()
    });
    info!("{:?}", people);
}
