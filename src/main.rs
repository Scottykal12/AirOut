use fltk::{app, prelude::*, window::*, button::*, frame::*, enums::Color};
//use std::process::Command;

//static mut GLB_IW_OUT: &str = "none";

/*pub fn iw_list(){
    let iw_cmd = Command::new("iwconfig")
    .arg("|").arg("awk").arg("'/ESSID {print $1}'")
    .output()
    .expect("iwconfig failed");
    let iw_cmd_out = String::from_utf8(iw_cmd.stdout).unwrap(); 
    unsafe {
        GLB_IW_OUT = &String::from_utf8(iw_cmd.stdout).unwrap();
    }
}*/

fn interfaces() {  
   
    let mut int_wind = OverlayWindow::default()
    .with_size(1280, 800)
    .center_screen()
    .with_label("Interfaces");

        let mut iw_but = Button::default()
        .with_size(300, 75)
        .with_label("iwconfig")
        .center_x(&int_wind);
        /*unsafe {
            iw_but.set_callback(move |_| println!("{}", &GLB_IW_OUT));
        }*/

        let mut frame = Frame::default()
        .with_size(500,500)
        //.with_label(&iw_cmd_out)
        .center_x(&int_wind)
        .below_of(&iw_but, 0)
        .set_color(Color::Dark2);

    int_wind.end();
    int_wind.show();
}

fn ap_scan() {
    let mut ap_wind = OverlayWindow::default()
    .with_size(1280, 800)
    .center_screen()
    .with_label("AP Scan");

    ap_wind.end();
    ap_wind.show();
}

fn pac_cap() {
    let mut pc_wind = OverlayWindow::default()
    .with_size(1280, 800)
    .center_screen()
    .with_label("Packet Capture");

    pc_wind.end();
    pc_wind.show();
}

fn air_play() {
    let mut air_play_wind = OverlayWindow::default()
    .with_size(1280, 800)
    .center_screen()
    .with_label("Air Play");

    air_play_wind.end();
    air_play_wind.show();
}

fn air_crack() {
    let mut air_crack_wind = OverlayWindow::default()
    .with_size(1280, 800)
    .center_screen()
    .with_label("Air Crack");

    air_crack_wind.end();
    air_crack_wind.show();
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


    wind.end();
    wind.show();
    app.run().unwrap();
}