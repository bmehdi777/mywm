use x11rb::connection::Connection;
use x11rb::errors::ReplyOrIdError;
use x11rb::protocol::{xproto::*, *};
use x11rb::COPY_DEPTH_FROM_PARENT;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = x11rb::connect(None).unwrap();
    let screen = &conn.setup().roots[screen_num];
    let root_window = screen.root;

    setup(&conn, root_window);
    conn.flush()?; // need to flush to send request to server
    print_info(&conn, screen_num);
    loop {
        let event = conn.wait_for_event()?;
        println!("Event: {:?}", event);
        match event {
            Event::MapRequest(e) => {
                println!("Debug event : {:?}", e);
                conn.map_window(e.window)?;
            }
            Event::KeyPress(e) => {
                println!("Key press {}", e.detail);
            }
            Event::ButtonPress(e) => {
                println!("Button pressed {}", e.detail);
            }
            Event::ClientMessage(message) => {
                println!("Message received {:?}", message);
            }
            _ => {}
        }
        conn.flush()?;
    }
}

fn print_info(conn: &impl Connection, screen_num: usize) {
    let screen = &conn.setup().roots[screen_num];
    println!();
    println!("Informations of screen {}:", screen.root);
    println!("  width.........: {}", screen.width_in_pixels);
    println!("  height........: {}", screen.height_in_pixels);
    println!("  white pixel...: {}", screen.white_pixel);
    println!("  black pixel...: {}", screen.black_pixel);
    println!();
}

fn setup(conn: &impl Connection, wid: u32) {
    let values = ChangeWindowAttributesAux::default()
        .event_mask(EventMask::SUBSTRUCTURE_NOTIFY | EventMask::SUBSTRUCTURE_REDIRECT | EventMask::KEY_PRESS);
    conn.change_window_attributes(wid, &values).unwrap();
    conn.grab_key(true, wid, ModMask::ANY, 26, GrabMode::ASYNC, GrabMode::ASYNC).unwrap();
}
