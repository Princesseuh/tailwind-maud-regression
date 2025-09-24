use maud::{html, Markup};

pub fn main() -> Markup {
    html! {
        header.px-8.py-4.text-black {
            "Hello, world!"
        }
    }
}
