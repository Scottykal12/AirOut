use fltk::{app::{self, delete_widget, App}, prelude::*, window::*, button::*, frame::*, enums::Color, output};
use std::{process::abort, process::Command, process::{Stdio, Output, Child}, str::{self, FromStr}, os::unix::process::CommandExt};

mod commands;

//static life && no mon

pub fn interfaces() {  
    let mut int_wind = OverlayWindow::default()
    .with_size(1280, 800)
    .center_screen()
    .with_label("Interfaces");

        let mut en_mon_int = Button::default()
        .with_size(300, 75)
        .with_label("Enable Monitor Mode")
        .center_x(&int_wind);

        let mut dis_mon_int = Button::default()
        .with_size(300, 75)
        .with_label("Disable Monitor Mode")
        .below_of(&en_mon_int, 10)
        .center_x(&int_wind);
        
        let mut frame_int = Frame::default()
        .with_size(300,10)
        .center_x(&int_wind)
        .below_of(&dis_mon_int, 0);

        let mut close_but = Button::default()
        .with_size(300, 75)
        .with_label("Close Window")
        .with_pos(970, 715);

        en_mon_int.set_callback( |_|{
            commands::mon_mode_on();
            //frame_int.with_label(&commands::get_interface());
        });

        dis_mon_int.set_callback( |_| {
            commands::mon_mode_off();
            //frame_int.with_label(&commands::get_interface());
        });

        loop {
            frame_int.with_label(&commands::get_interface());
        }

    int_wind.end();
    int_wind.make_resizable(true);
    int_wind.show();
    close_but.set_callback(move |_| int_wind.hide());
}

fn ap_scan() {
    let mut ap_wind = OverlayWindow::default()
    .with_size(1280, 800)
    .center_screen()
    .with_label("AP Scan");

    let mut close_but = Button::default()
    .with_size(300, 75)
    .with_label("Close Window")
    .with_pos(970, 715);

    ap_wind.end();
    ap_wind.show();
    close_but.set_callback(move |_| ap_wind.hide());
}

fn pac_cap() {
    let mut pc_wind = OverlayWindow::default()
    .with_size(1280, 800)
    .center_screen()
    .with_label("Packet Capture");

    let mut close_but = Button::default()
    .with_size(300, 75)
    .with_label("Close Window")
    .with_pos(970, 715);

    pc_wind.end();
    pc_wind.show();
    close_but.set_callback(move |_| pc_wind.hide());
}

fn air_play() {
    let mut air_play_wind = OverlayWindow::default()
    .with_size(1280, 800)
    .center_screen()
    .with_label("Air Play");
    
    let mut close_but = Button::default()
    .with_size(300, 75)
    .with_label("Close Window")
    .with_pos(970, 715);

    air_play_wind.end();
    air_play_wind.show();
    close_but.set_callback(move |_| air_play_wind.hide());
}

fn air_crack() {
    let mut air_crack_wind = OverlayWindow::default()
    .with_size(1280, 800)
    .center_screen()
    .with_label("Air Crack");

    let mut close_but = Button::default()
    .with_size(300, 75)
    .with_label("Close Window")
    .with_pos(970, 715);

    air_crack_wind.end();
    air_crack_wind.show();
    close_but.set_callback(move |_| air_crack_wind.hide());
}

fn main() {
    let app = app::App::default();


    let mut wind = Window::default()
    .with_size(1280, 800)
    .center_screen()
    .with_label("AirOut");

        let mut interfaces_but = Button::default()
        .with_size(300, 75)
        .with_label("Interfaces")
        .center_x(&wind);
        interfaces_but.set_callback(move |_| interfaces());

        let mut ap_scan_but = Button::default()
        .with_size(300, 75)
        .with_label("Access Point Scan")
        .center_x(&wind)
        .below_of(&interfaces_but, 10);
        ap_scan_but.set_callback(move |_| ap_scan());

        let mut pack_cap_but = Button::default()
        .with_size(300, 75)
        .with_label("Start/Stop Packet Capture")
        .center_x(&wind)
        .below_of(&ap_scan_but, 10);
        pack_cap_but.set_callback(move |_| pac_cap());

        let mut air_play_but = Button::default()
        .with_size(300, 75)
        .with_label("Disconnect Clients From AP")
        .center_x(&wind)
        .below_of(&pack_cap_but, 10);
        air_play_but.set_callback(move |_| air_play());

        let mut air_crack_but = Button::default()
        .with_size(300, 75)
        .with_label("Crack Password")
        .center_x(&wind)
        .below_of(&air_play_but, 10);
        air_crack_but.set_callback(move |_| air_crack());

        let mut close_but = Button::default()
        .with_size(300, 75)
        .with_label("Close")
        .with_pos(970, 715);

    wind.set_color(Color::Dark3);
    wind.end();
    wind.make_resizable(true);
    wind.show();
    close_but.set_callback(move |_| wind.hide());
    //close_but.set_callback(move |_| commands::mon_mode_off());
    app.run().unwrap();
}