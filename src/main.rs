#![deny(unused_extern_crates)]
#![warn(
    clippy::all,
    clippy::nursery,
    clippy::pedantic,
    future_incompatible,
    missing_copy_implementations,
    // missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_qualifications
)]
#[allow(clippy::redundant_pub_crate)]
mod thumb;

use crate::thumb::Thumb;
use yew::prelude::*;

const NAMES: [&str; 11] = [
    "gimme_pizza",
    "hey_are_you_ready_to_play",
    "p-i-z-z-a",
    "um_did_i_happen_to_say",
    "finger_lickin",
    "whipped_cream",
    "pizza_pie",
    "caramel",
    "spaghetti",
    "pasta",
    "uh",
];

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <>
        <header id="page_header">
            <h1>{ "put it in the pizza." }</h1>
            <p>{ "(click a pic)" }</p>
        </header>

        <article id="page_content">
            <table>
                <tr>
                    <Thumb name={ NAMES[0] } text="gimme pizza" />
                    <Thumb name={ NAMES[1] } text="are you ready?" />
                    <Thumb name={ NAMES[2] } text="p-i-z-z-a" />
                    <Thumb name={ NAMES[3] } text="did i happen to say?" />
                </tr>
                <tr>
                    <Thumb name={ NAMES[4] } text="finger lickin'" />
                    <Thumb name={ NAMES[5] } text="whipped cream pourin'" />
                    <Thumb name={ NAMES[6] } text="fly fly pizza pie" />
                    <Thumb name={ NAMES[7] } text="caramel coconut cream" />
                </tr>
                <tr>
                    <Thumb name={ NAMES[8] } text="1 2 3 4 5 spaghetti" />
                    <Thumb name={ NAMES[9] } text="pasta, fishsticks, ketchup, meatloaf" />
                    <Thumb name={ NAMES[10] } text="uhh... put it in the pizza" />
                    <td></td>
                </tr>
            </table>
        </article>

        <footer id="page_footer">
            { "inspired by" }
            <a href="http://www.youtube.com/watch?v=wusGIl3v044">{ "this." }</a>
        </footer>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
