use cursive::views::{Dialog,TextView};
use cursive::event::Key;

fn main() {
    let mut siv = cursive::default();
    let cattext = "some text here";
    siv.add_layer(
            Dialog::around(TextView::new(cattext))
                .button("OK", |s| s.quit())
        );
    siv.add_global_callback(Key::Esc,|s|s.quit());
    siv.run();
}
