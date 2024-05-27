use x11rb::connection::Connection;
use x11rb::errors::ReplyOrIdError;
use x11rb::protocol::xproto::*;
use x11rb::COPY_DEPTH_FROM_PARENT;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = x11rb::connect(None).unwrap();
    let screen = &conn.setup().roots[screen_num];
    let win_id = conn.generate_id()?;
    conn.create_window(
        COPY_DEPTH_FROM_PARENT,
        win_id,
        screen.root,
        0,
        0,
        100,
        100,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::new().background_pixel(screen.white_pixel),
    )?;
    conn.map_window(win_id)?;
    setup(&conn, screen.root);
    conn.flush()?; // need to flush to send request to server
    print_info(&conn, screen_num);
    loop {
        let event = conn.wait_for_event()?;
        println!("Event: {:?}", event);
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

fn setup(conn: &impl Connection, wid: u32 ) {
    // XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT | XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY | XCB_EVENT_MASK_STRUCTURE_NOTIFY | XCB_EVENT_MASK_BUTTON_PRESS | XCB_EVENT_MASK_FOCUS_CHANGE
    // XCB_CW_EVENT_MASK ?
    let values = ChangeWindowAttributesAux::default().event_mask(EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY | EventMask::STRUCTURE_NOTIFY | EventMask::BUTTON_PRESS | EventMask::FOCUS_CHANGE | EventMask::KEY_PRESS);
    conn.change_window_attributes(wid, &values).unwrap();
}
