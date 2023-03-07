use commands::{dump_air, get_interface};
use fltk::{
    app::{self, copy, delete_widget, App},
    button::*,
    draw,
    enums::*,
    frame::*,
    macros::menu,
    output,
    prelude::*,
    window::*,
};
use fltk_theme::{color_themes, widget_themes, ColorTheme, ThemeType, WidgetTheme};

mod commands;

const WIN_W: i32 = 900;
const WIN_H: i32 = 800;

const BUT_W: i32 = 300;
const BUT_H: i32 = 75;

fn interfaces() {
    let mut int_wind = OverlayWindow::default()
        .with_size(WIN_W, WIN_H)
        .center_screen()
        .with_label("Interfaces");

    let blank_frm = Frame::default().with_size(BUT_W, 20).center_x(&int_wind);

    let mut int_choice = fltk::menu::Choice::default()
        .with_size(BUT_W, 25)
        .with_label("Choose an Interface: ")
        .center_x(&int_wind)
        .below_of(&blank_frm, 0);

    for int in &commands::get_interface() {
        int_choice.add_choice(int);
    }

    // This seems to alway be the choice.
    int_choice.value();

    let mut en_mon_int = Button::default()
        .with_size(BUT_W, BUT_H)
        .with_label("Enable Monitor Mode")
        .below_of(&int_choice, 10);

    let mut dis_mon_int = Button::default()
        .with_size(BUT_W, BUT_H)
        .with_label("Disable Monitor Mode")
        .below_of(&en_mon_int, 10);

    let mut refresh_int = Button::default()
        .with_size(BUT_W, BUT_H)
        .with_label("Refresh interfaces")
        .below_of(&dis_mon_int, 10);

    let mut frame_int = Frame::default().with_size(300, 20);

    let mut close_but = Button::default()
        .with_size(BUT_W, BUT_H)
        .with_label("Close Window")
        .with_pos(WIN_W - BUT_W - 10, WIN_H - BUT_H - 10);


    // Can I refresh the interfaces after a choice?
    // may need to create a function to call

    let interface1 = int_choice.clone();
    let interface2 = int_choice.clone();
    en_mon_int.set_callback(move |_| match interface1.choice() {
        Some(choice) => commands::mon_mode_on(&choice),
        None => println!("No Interface Chosen"),
    });

    dis_mon_int.set_callback(move |_| match interface2.choice() {
        Some(choice) => commands::mon_mode_off(&choice),
        None => println!("No Interface Chosen"),
    });
    refresh_int.set_callback(move |_| {
        commands::get_interface();
        // snd_ref.send("refreshing");
        frame_int.set_label(&commands::get_interface()[0]);
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
    let mut ap_wind = OverlayWindow::default()
        .with_size(WIN_W, WIN_H)
        .center_screen()
        .with_label("AP Scan");

    let blank_frm = Frame::default().with_size(BUT_W, 20).center_x(&ap_wind);

    let mut int_choice = fltk::menu::Choice::default()
        .with_size(BUT_W, 25)
        .with_label("Choose an Interface: ")
        .center_x(&ap_wind)
        .below_of(&blank_frm, 0);

    for int in &commands::get_interface() {
        int_choice.add_choice(int);
    }

    int_choice.set_value(0);

    let mut scan_but = Button::default()
        .with_size(BUT_W, BUT_H)
        .center_x(&ap_wind)
        .below_of(&int_choice, 10)
        .with_label("Scan area");

    //can we get constant data stream data
    let mut frame_ap = Frame::default()
        .with_size(500, 500)
        .center_x(&ap_wind)
        .below_of(&scan_but, 10)
        .with_label("this is where i need data");

    // scan_but.set_callback(move |_| commands::dump_air());

    let mut close_but = Button::default()
        .with_size(BUT_W, BUT_H)
        .with_label("Close Window")
        .with_pos(WIN_W - BUT_W - 10, WIN_H - BUT_H - 10);

    ap_wind.end();
    ap_wind.show();
    close_but.set_callback(move |_| ap_wind.hide());
}

fn pac_cap() {
    let mut pc_wind = OverlayWindow::default()
        .with_size(WIN_W, WIN_H)
        .center_screen()
        .with_label("Packet Capture");

    let mut close_but = Button::default()
        .with_size(BUT_W, BUT_H)
        .with_label("Close Window")
        .with_pos(WIN_W - BUT_W - 10, WIN_H - BUT_H - 10);

    let mut choice_client = fltk::menu::Choice::default()
        .with_size(BUT_W, 25)
        .with_label("Client MAC: ")
        .center_of(&pc_wind);
    //.below_of(&scan_but, 10);

    choice_client.add_choice("qwer");

    let mut choice_ap = fltk::menu::Choice::default()
        .with_size(BUT_W, 25)
        .with_label("AP MAC: ")
        .center_of(&pc_wind)
        .below_of(&choice_client, 10);

    choice_ap.add_choice("qwer");

    let mut cap_hand_but = Button::default()
        .with_size(BUT_W, BUT_H)
        .center_of(&pc_wind)
        .below_of(&choice_ap, 10)
        .with_label("Start Capturing Handshake");

    //still need to handle file location
    let file = String::from("./handcap");

    cap_hand_but.set_callback(move |_| {
        commands::play_air(
            String::from(choice_ap.choice().unwrap()),
            String::from(choice_client.choice().unwrap()),
            file.clone(),
        )
    });

    pc_wind.end();
    pc_wind.show();
    close_but.set_callback(move |_| pc_wind.hide());
}

fn air_play() {
    let mut air_play_wind = OverlayWindow::default()
        .with_size(WIN_W, WIN_H)
        .center_screen()
        .with_label("Air Play");

    let mut close_but = Button::default()
        .with_size(BUT_W, BUT_H)
        .with_label("Close Window")
        .with_pos(WIN_W - BUT_W - 10, WIN_H - BUT_H - 10);

    air_play_wind.end();
    air_play_wind.show();
    close_but.set_callback(move |_| air_play_wind.hide());
}

fn air_crack() {
    let mut air_crack_wind = OverlayWindow::default()
        .with_size(WIN_W, WIN_H)
        .center_screen()
        .with_label("Air Crack");

    let mut close_but = Button::default()
        .with_size(WIN_W, WIN_H)
        .with_label("Close Window")
        .with_pos(WIN_W - BUT_W - 10, WIN_H - BUT_H - 10);

    air_crack_wind.end();
    air_crack_wind.show();
    close_but.set_callback(move |_| air_crack_wind.hide());
}

fn main() {
    println!("{:?}", get_interface());

    let app = app::App::default();
    let widget_theme = WidgetTheme::new(ThemeType::Dark);
    widget_theme.apply();

    let mut wind = Window::default()
        .with_size(WIN_W, WIN_H)
        .center_screen()
        .with_label("AirOut");

    let blank_frm = Frame::default().with_size(BUT_W, 20).center_x(&wind);

    // Show interfaces
    // let mut textbox = Frame::default().with_size(BUT_W, BUT_H);
    // thread::spawn(move || loop {
    //     textbox.with_label(&commands::get_interface()[1]);

    // });

    let mut interfaces_but = Button::default()
        .with_size(BUT_W, BUT_H)
        .with_label("Interfaces")
        .center_x(&wind)
        .below_of(&blank_frm, 10);

    interfaces_but.set_callback(move |_| interfaces());

    let mut ap_scan_but = Button::default()
        .with_size(BUT_W, BUT_H)
        .with_label("Access Point Scan")
        .center_x(&wind)
        .below_of(&interfaces_but, 10);
    ap_scan_but.set_callback(move |_| ap_scan());

    let mut pack_cap_but = Button::default()
        .with_size(BUT_W, BUT_H)
        .with_label("Start/Stop Packet Capture")
        .center_x(&wind)
        .below_of(&ap_scan_but, 10);
    pack_cap_but.set_callback(move |_| pac_cap());

    let mut air_play_but = Button::default()
        .with_size(BUT_W, BUT_H)
        .with_label("Disconnect Clients From AP")
        .center_x(&wind)
        .below_of(&pack_cap_but, 10);
    air_play_but.set_callback(move |_| air_play());

    let mut air_crack_but = Button::default()
        .with_size(BUT_W, BUT_H)
        .with_label("Crack Password")
        .center_x(&wind)
        .below_of(&air_play_but, 10);
    air_crack_but.set_callback(move |_| air_crack());

    let mut close_but = Button::default()
        .with_size(BUT_W, BUT_H)
        .with_label("Close")
        .with_pos(WIN_W - BUT_W - 10, WIN_H - BUT_H - 10);

    wind.end();
    wind.make_resizable(true);
    wind.show();
    close_but.set_callback(move |_| wind.hide());
    app.run().unwrap();
}
