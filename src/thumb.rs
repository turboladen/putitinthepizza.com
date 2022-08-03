use js_sys::Promise;
use wasm_bindgen::JsValue;
use web_sys::console;
use web_sys::HtmlMediaElement;
use yew::prelude::*;

pub(crate) struct Thumb {
    audio_ref: NodeRef,
}

impl Component for Thumb {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            audio_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PlayRequest => {
                let promise = self
                    .audio_ref
                    .cast::<HtmlMediaElement>()
                    .unwrap()
                    .play()
                    .unwrap();
                ctx.link().send_future(async {
                    match play_the_thing(promise).await {
                        Ok(_) => Msg::Success,
                        Err(_) => Msg::Failure,
                    }
                });

                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| {
            console::debug_1(&JsValue::from_str("hi"));
            Msg::PlayRequest
        });

        html! {
            <td>
                <audio id={ ctx.props().name.clone() } preload="auto" ref={self.audio_ref.clone()}>
                    <source src={ ctx.props().audio_file_path(AudioFileType::Mp3) } type="audio/mpeg" />
                    <source src={ ctx.props().audio_file_path(AudioFileType::Ogg) } type="audio/ogg" />
                    <p>
                        { "can't put it in the pizza.  you need an " }
                        <a href="http://thebrowsereview.com/html5/html5-audio-tag-and-format-support/">
                            { "html5 browser" }
                        </a>
                    </p>
                </audio>

                <img src={ctx.props().img_file_path()} onclick={onclick} />
                <p>{ &ctx.props().text }</p>
            </td>
        }
    }
}

async fn play_the_thing(promise: Promise) -> Result<JsValue, JsValue> {
    wasm_bindgen_futures::JsFuture::from(promise).await
}

pub(crate) enum Msg {
    PlayRequest,
    Success,
    Failure,
}

#[derive(PartialEq, Eq, Properties)]
pub(crate) struct Props {
    #[prop_or_default]
    pub(crate) text: String,

    #[prop_or_default]
    pub(crate) name: String,
}

impl Props {
    fn img_file_path(&self) -> String {
        format!("thumbs/{}.jpg", self.name)
    }

    fn audio_file_path(&self, file_type: AudioFileType) -> String {
        match file_type {
            AudioFileType::Ogg => format!("audio/{}.ogg", self.name),
            AudioFileType::Mp3 => format!("audio/{}.mp3", self.name),
        }
    }
}

enum AudioFileType {
    Ogg,
    Mp3,
}
