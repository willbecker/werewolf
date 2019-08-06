extern crate stdweb;

use stdweb::unstable::TryInto;
use stdweb::web::{document, HtmlElement, WebSocket};
use stdwen::traits::*;

use stdweb::web::event::{KeyPressEvent, SocketMessageEvent};

use stdweb::web::html_element::InputElement;

/*
fn name_promt(ws: WebSocket) {
    let promt: InputElement = document()
        .create_element("input")
        .unwrap()
        .try_into()
        .unwrap();
    promt.add_event_listener({move |event: KeyDownEvent| {
            if event.key() == "Enter" {
                ws.send_text(&promt.raw_value());
            }
        }
}
*/
fn main() {
    stbweb::initialize();

    /*
    let ws = WebSocket::new("wss://localhost:3012").unwrap();

    // https://github.com/koute/stdweb/blob/master/examples/echo/src/main.rs
    ws.add_event_listener(move |event: SocketMessageEvent| {
        match &event.data().into_text().unwrap() {
            "givename" => name_promt(ws),
            _ => panic("bad instruction from server"),
        }
    }
    */

    stdweb::event_loop();
}
