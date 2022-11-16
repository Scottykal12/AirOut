use fltk::{app::{self, delete_widget}, prelude::*, window::*, button::*, frame::*, enums::Color, output};
use std::{process::abort, process::Command, process::{Stdio, Output, Child}, str::{self, FromStr}, os::unix::process::CommandExt};


//static mut GLB_IW_OUT: &str = "none";

//https://stackoverflow.com/questions/73469520/how-to-pipe-commands-in-rust
//iw dev | grep Interface | awk -F ' ' '{print $2}'
//calculate_length
//-> impl  AsRef<str>

static mut ISMONMODE: bool = false;

fn mon_mode_on() {
    //put interface into monotor mode
    let monenabled_ret = String::from("Moniotor mode enabled");
    
    unsafe {
        if ISMONMODE == true {
            println!("{}", monenabled_ret);
            //return monenabled_ret;
        }else{
            //airmon-ng start $wint
            Command::new("pkexec")
            .arg("airmon-ng")
            .arg("start")
            .arg(&get_interface())
            .exec();
            
            ISMONMODE = true;

            println!("{}", monenabled_ret)

            //return monenabled_ret;
        }
    }
}

fn mon_mode_off() {
    //put interface into monotor mode
    let mondisabled_ret = String::from("Moniotor mode disabled");
    let mut mon_interface = get_interface();
    mon_interface.push_str("mon");

    unsafe {
        if ISMONMODE == false {
            println!("{}", mondisabled_ret);
            //return monenabled_ret;
        }else{
            //airmon-ng start $wint
            Command::new("pkexec")
            .arg("airmon-ng")
            .arg("stop")
            .arg(&mon_interface)
            .exec();
            
            ISMONMODE = false;

            println!("{}", mondisabled_ret);

            //return monenabled_ret;
        }
    }
}

//Don't stop new line!!!
fn get_interface() -> String {
    let iw_cmd = Command::new("iw")
    .arg("dev")
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();
    let grep_child = Command::new("grep")
    .arg("Interface")
    .stdin(Stdio::from(iw_cmd.stdout.unwrap()))
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();
    let awk_child = Command::new("awk")
    .arg("-F")
    .arg(" ")
    .arg("{print $2}")
    .stdin(Stdio::from(grep_child.stdout.unwrap()))
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();
    let xargs_child = Command::new("xargs")
    .arg("echo")
    .arg("-n")
    .stdin(Stdio::from(awk_child.stdout.unwrap()))
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();

    let raw_out = xargs_child.wait_with_output().unwrap();
    let read_out = str::from_utf8(&raw_out.stdout).unwrap();
    let interface_name = read_out.to_owned();
    print!("Your wireless interface is {}", read_out );
    return interface_name;
}

fn interfaces() {  
    let mut int_wind = OverlayWindow::default()
    .with_size(1280, 800)
    .center_screen()
    .with_label("Interfaces");

        let mut en_mon_int = Button::default()
        .with_size(300, 75)
        .with_label("Enable Monitor Mode")
        .center_x(&int_wind);

        en_mon_int.set_callback(move |_| mon_mode_on());

        let mut dis_mon_int = Button::default()
        .with_size(300, 75)
        .with_label("Disable Monitor Mode")
        .below_of(&en_mon_int, 10)
        .center_x(&int_wind);

        dis_mon_int.set_callback(move |_| mon_mode_off());
        
        let mut frame = Frame::default()
        .with_size(500,500)
        .with_label(&get_interface())
        .center_x(&int_wind)
        .below_of(&dis_mon_int, 0)
        .set_color(Color::Dark2);

        let mut close_but = Button::default()
        .with_size(300, 75)
        .with_label("Close Window")
        .with_pos(970, 715);

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
    app.run().unwrap();
}