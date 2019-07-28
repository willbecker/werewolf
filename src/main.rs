use cursive::Cursive;
use cursive::views::*;
use cursive::traits::*;
use std::num::ParseIntError;
mod roles;


fn main() {
    let mut siv = Cursive::default();

    main_menu(&mut siv);

    fn promt_player_count(s: &mut Cursive) {
        //-> Result<u8, ParseIntError> {

        s.pop_layer();
        s.add_layer(Dialog::around(ListView::new()
                .child("Number of players", EditView::new()
                    .max_content_width(3)
                    .with_id("players")
                    .fixed_width(4)))
            .title("Werewolf")
            .button("Next", main_menu))
    }

    fn main_menu(s: &mut Cursive) {
        let mut player_count: Option<u8> = None;

        s.add_layer(Dialog::text("Main menu")
            .title("werewolf")
            .button("Start game", |s| s.add_layer(Dialog::info("No role settings.")))
            .button("Role settings", {
                match player_count {
                    None => promt_player_count,
                    Some(0...5) => promt_player_count,
                    _ => role_select, 
                }
            })
            .button("Quit", |s| s.quit()));
    }

    fn error(s: &mut Cursive) {
        s.add_layer(Dialog::around(DebugView::new()));
    }

    fn role_select(s: &mut Cursive) {

        let included = SelectView::<String>::new()
            .on_submit(delete_role)
            .with_id("included")
            .fixed_size((10, 5));
        
        let available = SelectView::<String>::new()
            .on_submit(add_to_included)
            .with_all_str(roles::AVAILABLE_ROLES.to_vec())
            .with_id("available")
            .fixed_size((10, 5));

        s.add_layer(Dialog::around(LinearLayout::horizontal()
            .child(LinearLayout::vertical()
                .child(TextView::new("Included"))
                .child(DummyView)
                .child(included))
            .child(TextView::new("||||||||||||")
                .center()
                .fixed_width(1))
            .child(LinearLayout::vertical()
                .child(TextView::new("Available"))
                .child(DummyView)
                .child(available)))
        .title("Role Select")
        .button("Back", |s| { 
            s.pop_layer();
        }));
    }

    fn delete_role(s: &mut Cursive, name: &String) {
    }

    fn add_to_included(s: &mut Cursive, name: &String) {
    }

    siv.run();
}
