// hide console window on Windows in release:
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

fn main() {
  let options = eframe::NativeOptions {
    transparent: true,
    ..Default::default()
  };
  
  eframe::run_native(
    "My egui App",
    options,
    Box::new(|_cc| Box::new(MyApp::default())),
  );
}

struct MyApp {
  name: String,
  age: u32,
}

impl Default for MyApp {
  fn default() -> Self {
    Self {
      name: "Arthur".to_owned(),
      age: 42,
    }
  }
}

impl eframe::App for MyApp {
  fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
    egui::Rgba::TRANSPARENT
  }
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    ctx.set_visuals(egui::Visuals::light());
    egui::SidePanel::left("my_left_panel")
      .frame(egui::Frame::none())
      .show(ctx, |ui| {
        ui.horizontal_wrapped(|ui| ui.label("Hello World!"));
      });
    egui::CentralPanel::default()
      .show(ctx, |ui| {
        ui.horizontal_wrapped(|ui| ui.label("Hello World!"));
        ui.heading("My egui Application");
        ui.horizontal(|ui| {
          ui.label("Your name: ");
          ui.text_edit_singleline(&mut self.name);
        });
        ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
        if ui.button("Click each year").clicked() {
          self.age += 1;
        }
        ui.label(format!("Hello '{}', age {}", self.name, self.age));
      });
  }
}

// use imap;

// fn main() {
//   let res = fetch();
//   println!("Res: {:?}", res);
// }

// fn build_ui() {
// }


// fn fetch() -> imap::error::Result<Option<String>> {

//   let client = imap::ClientBuilder::new("imap.fastmail.com", 993).native_tls()?;

//   // the client we have here is unauthenticated.
//   // to do anything useful with the e-mails, we need to log in
//   let mut imap_session = client
//   .login("paulrouget@fastmail.com", "jdprl8zq2rx39jwz")
//   .map_err(|e| e.0)?;


//   // we want to fetch the first email in the INBOX mailbox
//   imap_session.select("Notes")?;

//   // fetch message number 1 in this mailbox, along with its RFC822 field.
//   // RFC 822 dictates the format of the body of e-mails
//   let messages = imap_session.fetch("1:1024", "RFC822")?;
//   for message in messages.iter() {
//     // extract the message's body
//     let body = message.body().expect("message did not have a body!");
//     let body = std::str::from_utf8(body)
//     .expect("message was not valid utf-8")
//     .to_string();
//     println!("MESSAGE:\n{}", body);

//   }


//   // be nice to the server and log out
//   imap_session.logout()?;

//   Ok(None)
// }
