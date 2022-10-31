use fltk::{app, prelude::*, window::*, button::{*, self}, frame::*, enums::Color};
use std::process::Command;

pub fn iw_list(){
    let mut iw_cmd = Command::new("iwconfig")
    //.arg("|").arg("awk").arg("'/ESSID {print $1}'")
    .output()
    .expect("iwconfig failed");
    let iw_cmd_out = String::from_utf8(iw_cmd.stdout).unwrap(); 
    static glb_iw_out: String = iw_cmd_out;
}

fn interfaces() {  
   
    let mut int_wind = OverlayWindow::default()
    .with_size(1280, 800)
    .center_screen()
    .with_label("Interfaces");

        let mut iw_but = Button::default()
        .with_size(300, 75)
        .with_label("iwconfig")
        .center_x(&int_wind);
        iw_but.set_callback(move |_| println!("{}", &glb_iw_out));

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


    wind.end();
    wind.show();
    app.run().unwrap();
}