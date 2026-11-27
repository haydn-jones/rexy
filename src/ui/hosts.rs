//! The other servers the window lists, which home names: adding one, changing
//! one and taking one away. A host is typed in the palette as `rex hosts add`
//! takes it, its name and then the addresses that reach it.
use std::net::IpAddr;

use eframe::egui;
use url::Url;

use super::{
    App,
    close::asking,
    commands::title,
    palette::Listing,
    style::{Item, entry},
};
use crate::model::{Action, Host};

/// The command that adds a host, and the one that takes one away.
pub const ADD: &str = "client.host.add";
pub const REMOVE: &str = "client.host.remove";
/// The commands that turn the window to the next host, or the one before.
pub const NEXT: &str = "client.host.next";
pub const PREVIOUS: &str = "client.host.previous";

/// A host as it is typed: its name, then its addresses.
pub fn line(host: &Host) -> String {
    format!("{} {}", host.label, host.endpoints.join(" "))
}

/// The host that a line typed names, in place of the one it `was` if it is a
/// change to one. An address alone gives the host its name.
pub fn typed(line: &str, was: Option<&Host>) -> Option<Host> {
    let mut words = line.split_whitespace().map(str::to_owned);
    let first = words.next()?;
    let rest: Vec<String> = words.collect();
    let (label, endpoints) = match rest.is_empty() {
        true => (named(&first)?, vec![first]),
        false => (first, rest),
    };
    let id = was.map(|was| was.id.clone()).unwrap_or_default();
    Some(Host { id, label, endpoints, source: String::new() })
}

/// The name an address gives a host: its machine's, short of the domain it is
/// in. An address that names no machine, such as a socket's path, gives none.
fn named(address: &str) -> Option<String> {
    // Read as the addresses to connect to are, where one may come without a scheme.
    let url = match address.contains("://") {
        true => Url::parse(address),
        false => Url::parse(&format!("rex://{address}")),
    };
    let url = url.ok()?;
    // One in numbers is all name, and the longer kind of those is in brackets.
    let machine = url.host_str()?.trim_matches(['[', ']']);
    let short = machine.split('.').next().filter(|_| machine.parse::<IpAddr>().is_err());
    Some(short.unwrap_or(machine).to_owned()).filter(|name| !name.is_empty())
}

impl App {
    /// The menu of a server's name in the sidebar. One that home's list keeps
    /// may be changed there and taken away, where one its configuration
    /// declares is that file's. A host may be added from any.
    pub(super) fn host_menu(&mut self, name: &egui::Response, server: usize) {
        asking(name, 220.0, |ui| {
            let item = |label| Item { label, ..Default::default() };
            let host = self.servers[server].host.clone();
            // Home is where the window was pointed, and is in no list.
            if server > 0 {
                let (kept, why) = (host.source.is_empty(), format!("Declared in {}", host.source));
                let change = entry(ui, kept, item("Change Host…")).on_disabled_hover_text(&why);
                if change.clicked() {
                    self.list(ui.ctx(), Listing::Host(Some(host.clone())));
                }
                let remove = entry(ui, kept, item("Remove Host")).on_disabled_hover_text(&why);
                if remove.clicked() {
                    self.tell(0, Action::RemoveHost(host.id));
                    ui.close();
                }
                ui.separator();
            }
            if entry(ui, true, item(title(ADD))).clicked() {
                self.todo.push((ADD, None));
                ui.close();
            }
        });
    }
}

#[cfg(test)]
#[path = "../../tests/unit/ui_hosts.rs"]
mod tests;
