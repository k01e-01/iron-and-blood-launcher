use anyhow::{Result, anyhow};
use std::io::Write;
use std::process::{Command, Stdio};
use gtk4::prelude::*;
use gtk4::{
  Application,
  ApplicationWindow, 
  Picture, 
  Overlay, 
  Box, 
  Orientation, 
  Button, 
  Align,
  CssProvider,

  gdk,
  gio,
  glib,
};

const APP_ID: &str = "com.iron_and_blood.launcher";

const RESOURCES: &[u8] = include_bytes!(
  concat!(env!("OUT_DIR"), "/resources.gresource")
);

macro_rules! res {
  ($a:expr) => {
    concat!("/com/iron_and_blood/launcher/", $a).into()
  };
}

fn make_button(name: &str) -> Button {
  Button::builder()
    .label(name)
    .height_request(100)
    .css_classes([name])
    .build()
}

fn action_revert() {
  println!("revert!");
}

fn action_docs() {
  unimplemented!();
}

fn action_discord() {
  unimplemented!();
}

fn action_launch() {
  unimplemented!();
}

fn error_dialog(err: &'static str) -> Result<()> {
  let mut child = Command::new("cmd")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()?;

  {
    let stdin = child.stdin.as_mut().expect("");
    stdin.write_all(err.as_bytes())?;
  }

  child.wait()?;
  Ok(())
}

fn main() -> Result<()> {
  let app: Application = Application::builder()
    .application_id(APP_ID)
    .build();

  let resource = gio::Resource::from_data(&glib::Bytes::from_static(RESOURCES))
    .expect("failed to load resource!");
  gio::resources_register(&resource);


  app.connect_startup(|_| {
    let provider = CssProvider::new();
    provider.load_from_resource(res!("assets/style.css"));

    gtk4::style_context_add_provider_for_display(
      &gdk::Display::default().expect("there should be a display!"),
      &provider,
      gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
  });

  app.connect_activate(|app| {
    let window = ApplicationWindow::builder()
      .application(app)
      .title("Napoleon Total War: Iron and Blood - Launcher")
      .default_width(800)
      .default_height(800)
      .resizable(false)
      .build();

    let overlay = Overlay::new();
    let bg = Picture::for_resource(res!("assets/bg.png"));
    overlay.set_child(Some(&bg));

    let logo = Picture::builder()
      .width_request(700)
      .halign(Align::Center)
      .valign(Align::Start)
      .margin_top(10)
      .build();

    logo.set_resource(res!("assets/logo.png"));

    overlay.add_overlay(&logo);


    let button_box = Box::builder()
      .width_request(700)
      .halign(Align::Center)
      .valign(Align::End)
      .margin_bottom(50)
      .homogeneous(true)
      .spacing(50)
      .orientation(Orientation::Horizontal)
      .build();

    let button_revert = make_button("Revert");
    let button_docs = make_button("Docs");
    let button_discord = make_button("Discord");
    let button_launch = make_button("Launch");

    button_revert.connect_clicked(|_| { action_revert() });
    button_docs.connect_clicked(|_| { action_docs() });
    button_discord.connect_clicked(|_| { action_discord() });
    button_launch.connect_clicked(|_| {action_launch() });

    button_box.append(&button_revert);
    button_box.append(&button_docs);
    button_box.append(&button_discord);
    button_box.append(&button_launch);

    overlay.add_overlay(&button_box);


    window.set_child(Some(&overlay));

    window.present();
  });  
  
  let result = std::panic::catch_unwind(|| {
    app.run();
  });

  match result {
    Ok(_) => Ok(()),
    Err(payload) => {
      if let Some(s) = payload.downcast_ref::<&str>() {
        println!("something bad happened: {}", s);
        error_dialog(s)?;
        Err(anyhow!(s.to_string()))
      } else {
        Err(anyhow!("something very bad happened!"))
      }
    }
  }
}
