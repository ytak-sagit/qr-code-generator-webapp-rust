use base64_wrapper::{engine::general_purpose::STANDARD, Engine};
use qr_code_wrapper::{to_png_to_vec_from_str, QrCodeEcc};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let input_ref = use_node_ref();

    let url_updator = use_state(|| "".to_string());

    let onsubmit = {
        let input_ref = input_ref.clone();
        let url_updator = url_updator.clone();
        move |e: SubmitEvent| {
            e.prevent_default();
            url_updator.set(input_ref.cast::<HtmlInputElement>().unwrap().value());
        }
    };

    html! {
        <main>
            <form {onsubmit}>
                <label>
                    {"Input URL:"}
                    // TODO: バリデーション
                    <input
                        ref={input_ref.clone()}
                        type="url"
                        placeholder="https://example.com"
                        required=true
                        maxlength=2048
                    />
                </label>
                <button type="submit">{"Generate QR Code"}</button>
            </form>
            <div>
                <QrCodeImage url={(*url_updator).clone()} />
            </div>
        </main>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct QrCodeProps {
    url: AttrValue,
}

#[function_component(QrCodeImage)]
fn qr_code_image(props: &QrCodeProps) -> Html {
    let QrCodeProps { url } = props.clone();
    let clipboard = use_clipboard();

    if url.is_empty() {
        return html! {
            <>
                <p>{"QR code will be displayed below."}</p>
                <img src="public/placeholder.svg" alt="placeholder" />
            </>
        };
    }

    let png_image_data = to_png_to_vec_from_str(&url, QrCodeEcc::Low, 256).unwrap();
    let onclick = {
        let png_image_data = png_image_data.clone();
        let clipboard = clipboard.clone();
        move |_| {
            let png_image_data = png_image_data.clone();
            clipboard.write(png_image_data, Some("image/png".to_owned()));
            // TODO: スナックバーのような形式でメッセージ表示したい
            let window = web_sys::window().unwrap();
            window.confirm_with_message("Copied!").unwrap();
        }
    };

    let base64_encoded_image_data = STANDARD.encode(png_image_data.clone());
    let img = format!("data:image/png;base64,{}", base64_encoded_image_data);
    // TODO: QRコードが表示されるまで loading 表示をしたい
    html! {
        <>
            <p>
                <img style="display: inline-block; width: 1.2em; height: 1.2em; vertical-align: middle;" src="public/icon-success.svg" alt="success" />
                {"QR code was generated."}
            </p>
            <img src={img} alt={&url} />
            // TODO: アイコンボタンにしたい
            // <img onclick={onclick} style="display: inline-block; height: 1lh; vertical-align: middle; cursor: pointer;" src="public/icon-clipboard.svg" alt="copy to clipboard" />
            <button onclick={onclick}>{"Copy to Clipboard"}</button>
        </>
    }
}
