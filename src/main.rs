use commands::ISMONMODE;
use fltk::{app::{self, delete_widget, App, copy}, prelude::*, window::*, button::*, frame::*, enums::*, output, macros::menu};
use std::{process::abort, process::Command, process::{Stdio, Output, Child}, str::{self, FromStr}, os::unix::{process::CommandExt}, sync::mpsc::{self, Sender}, thread, cmp::Reverse, io::BufRead, clone};


mod commands;

fn interfaces() {  
    // let (snd, rcv) = mpsc::channel();
    // let snd_en = snd.clone();
    // let snd_dis = snd.clone();
    // let snd_ref = snd.clone();
    let getint = &commands::get_interface().clone();
    let int_list: Vec<&str> = getint.split("\n").collect();


    let mut int_wind = OverlayWindow::default()
    .with_size(1280, 800)
    .center_screen()
    .with_label("Interfaces");

        let mut int_choice = fltk::menu::Choice::default()
        .with_size(300, 20)
        .with_label("Choose an Interface: ")
        .center_x(&int_wind);

        for int in &int_list {
            int_choice.add_choice(int);
        }

        int_choice.set_value(0);

        let mut en_mon_int = Button::default()
        .with_size(300, 75)
        .with_label("Enable Monitor Mode")
        .below_of(&int_choice, 10);

        let mut dis_mon_int = Button::default()
        .with_size(300, 75)
        .with_label("Disable Monitor Mode")
        .below_of(&en_mon_int, 10);

        let mut refresh_int = Button::default()
        .with_size(300, 75)
        .with_label("Refresh interfaces")
        .below_of(&dis_mon_int, 10);
        
        let mut frame_int = Frame::default()
        .with_size(300,20);

        let mut close_but = Button::default()
        .with_size(300, 75)
        .with_label("Close Window")
        .with_pos(970, 715);

        
        //This is ugly. There has to be a bettr way
        let interface1 = int_choice.choice().clone().unwrap();
        let interface2 = int_choice.choice().clone().unwrap();

        en_mon_int.set_callback(move |_| {
            commands::mon_mode_on(&interface1);
            commands::get_interface();
            // let snd_en = snd_en.clone();
            // thread::spawn(move || snd_en.send("enabling"));
        });

        dis_mon_int.set_callback(move |_| {
            commands::mon_mode_off(&interface2);
            commands::get_interface();
            // snd_dis.send("disabling");
        });
        refresh_int.set_callback(move |_| {
            commands::get_interface();
            // snd_ref.send("refreshing");
            frame_int.set_label(&commands::get_interface());
        });

        // while let Ok(msg) = rcv.try_recv() {
        //     frame_int.set_label(&commands::get_interface());
        //     println!("you got {}", msg);
        // }

    int_wind.end();
    int_wind.make_resizable(true);
    int_wind.show();
    close_but.set_callback(move |_| int_wind.hide());
}

fn ap_scan() {
    let getint = &commands::get_interface().clone();
    let int_list: Vec<&str> = getint.split("\n").collect();

    let mut ap_wind = OverlayWindow::default()
    .with_size(1280, 800)
    .center_screen()
    .with_label("AP Scan");

        let mut int_choice = fltk::menu::Choice::default()
        .with_size(300, 20)
        .with_label("Choose an Interface: ")
        .center_x(&ap_wind);

        for int in &int_list {  
            int_choice.add_choice(int);
        }

        let mut scan_but = Button::default()
        .with_size(300, 75)
        .center_x(&ap_wind)
        .below_of(&int_choice, 10)
        .with_label("Scan area");

        //can we get constant data stream data
        let mut frame_ap = Frame::default()
        .with_size(500, 500)
        .center_x(&ap_wind)
        .below_of(&scan_but, 10);

        scan_but.set_callback(move |_| commands::dump_air());

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

    let mut choice_client = fltk::menu::Choice::default()
    .with_size(300, 25)
    .with_label("Client MAC: ")
    .center_of(&pc_wind);
    //.below_of(&scan_but, 10);
    
    choice_client.add_choice("qwer");

    let mut choice_ap = fltk::menu::Choice::default()
    .with_size(300, 25)
    .with_label("AP MAC: ")
    .center_of(&pc_wind)
    .below_of(&choice_client, 10);
    
    choice_ap.add_choice("qwer");

    let mut cap_hand_but = Button::default()
    .with_size(300, 75)
    .center_of(&pc_wind)
    .below_of(&choice_ap, 10)
    .with_label("Start Capturing Handshake");

    //still need to handle file location
    let file = String::from("./handcap");

    cap_hand_but.set_callback(move |_| {
        commands::play_air(String::from(choice_ap.choice().unwrap()), String::from(choice_client.choice().unwrap()), file.clone())
    });

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
    app.run().unwrap();
}