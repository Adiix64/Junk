use teloxide::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

mod command_handlers;

static OWNER_ID: i64 = 7660492768;

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();

    // Shared state for authorized users and access logs
    let authorized_users: Arc<Mutex<HashSet<i64>>> = Arc::new(Mutex::new(HashSet::new()));
    let access_logs: Arc<Mutex<HashMap<i64, String>>> = Arc::new(Mutex::new(HashMap::new()));

    // Add the owner by default
    authorized_users.lock().unwrap().insert(OWNER_ID);

    // Pass the state to the bot handler
    let handler = teloxide::repl_with_state(
        bot,
        (authorized_users, access_logs),
        command_handlers::handle_message,
    );
    handler.await;
}
