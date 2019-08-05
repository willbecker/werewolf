pub struct Player {
    name: String,
    id: u16,
}

fn name_promt() -> String {
    let mut name = "noname".to_string();
    let promt: InputElement = document()
        .create_element("input")
        .unwrap()
        .try_into()
        .unwrap();
    promt.add_event_listener({
        move |event: KeyDownEvent| {
            if event.key() == "Enter" {
                
                let text: String = promt.raw_value();

            }
        }
}

impl Player {
    fn new() -> Player {
        let page = document().query_selector(".main").unwrap().unwrap();
        page.append_child(
            k
        )
        let name:
    }
}
