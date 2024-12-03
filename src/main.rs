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

use thumb::*;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
    <>
    <header id="page_header">
        <h1>{ "put it in the pizza." }</h1>
        <p>{ "(click a pic)" }</p>
    </header>

    <article id="page_content">
        <table>
            <tr>
                <GimmePizza /> <AreYouReady /> <PIZZA /> <UmDidIHappenToSay />
            </tr>
            <tr>
                <FingerLickin /> <WhippedCream /> <FlyFlyPizzaPie /> <CaramelCoconutCream />
            </tr>
            <tr>
                <Spaghetti /> <Pasta /> <Uh /> <td></td>
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

fn main() {
    yew::Renderer::<App>::new().render();
}
