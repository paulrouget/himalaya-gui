use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use anyhow::{Error, Result};
// FIXME: should be in lib
use himalaya::config::DeserializedConfig;
use himalaya_lib::BackendBuilder;
pub use himalaya_lib::{Envelope, Envelopes, Flag};
#[allow(unused_imports)]
use log::{error, info, warn};

pub type MailId = String;
pub type AccountId = String;
pub type MboxName = String;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MboxId {
  pub account: AccountId,
  pub name: MboxName,
}

pub type Mboxes = HashMap<AccountId, Vec<MboxName>>;

pub struct Server {
  pub to: Sender<ServerCmd>,
  pub from: Receiver<ServerEvent>,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum ServerCmd {
  GetAllEnvelopes(MboxName),
  GetMessageBody(MailId, MboxName),
  GetMboxes,
}

#[derive(Debug)]
pub enum ServerEvent {
  Envelopes((MboxId, Envelopes)),
  Mboxes(Vec<MboxName>),
  Body((MailId, String)),
  Error(Error),
}

pub fn accounts() -> Result<Vec<String>> {
  let config = DeserializedConfig::from_opt_path(None)?;
  let accounts = config.accounts.keys().cloned().collect();
  Ok(accounts)
}

pub fn run(ctx: egui::Context, account: String) -> (Sender<ServerCmd>, Receiver<ServerEvent>) {
  let (to_main, from_server) = channel();
  let (to_server, from_main) = channel();

  thread::spawn(move || {
    info!("Server thread spawn");

    let config = DeserializedConfig::from_opt_path(None).unwrap();
    let (account_config, backend_config) = config.to_configs(Some(&account)).unwrap();

    let mut backend = BackendBuilder::build(&account_config, &backend_config).unwrap();

    let mut db = HashMap::new();

    loop {
      let message = from_main.recv();

      info!("Got server command: {:?}", message);

      let message = match message {
        Ok(m) => m,
        Err(e) => {
          to_main.send(ServerEvent::Error(e.into())).expect("Main thread dead?");
          continue;
        },
      };

      let main_message = match message {
        // FIXME: should use 0 instead of 1024, but getting parsing error
        ServerCmd::GetAllEnvelopes(mbox) => {
          match backend.envelope_list(&mbox, 1024, 0) {
            Err(e) => to_main.send(ServerEvent::Error(e.into())),
            Ok(envelopes_as_vec) => {
              let envelopes_as_map: HashMap<String, Envelope> = envelopes_as_vec.clone().into_iter().map(|i| (i.id.clone(), i)).collect();
              db.insert(mbox.clone(), envelopes_as_map);
              let mbox = MboxId {
                account: account.clone(),
                name: mbox,
              };
              to_main.send(ServerEvent::Envelopes((mbox, envelopes_as_vec)))
            },
          }
        },
        ServerCmd::GetMessageBody(id, mbox) => {
          match backend.email_get(&mbox, &id) {
            Err(e) => to_main.send(ServerEvent::Error(e.into())),
            Ok(msg) => {
              let body = msg.to_readable_string("plain", vec![], &account_config).expect("Main thread dead?");
              to_main.send(ServerEvent::Body((id, body)))
            },
          }
        },
        ServerCmd::GetMboxes => {
          match backend.folder_list() {
            Err(e) => to_main.send(ServerEvent::Error(e.into())),
            Ok(msg) => {
              let mboxes: Vec<MboxName> = msg.folders.into_iter().filter(|f| f.name != "[Gmail]").map(|mbox| mbox.name).collect();
              to_main.send(ServerEvent::Mboxes(mboxes))
            },
          }
        },
      };

      if let Err(e) = main_message {
        error!("Communitcation with `main` failed: {}", e);
      }

      ctx.request_repaint();
    }
  });
  (to_server, from_server)
}
