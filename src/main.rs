use qr_code_wrapper::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;

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

    if url.is_empty() {
        return html! {
            <>
                <p>{"QR code will be displayed below."}</p>
                <img src="public/placeholder.svg" alt="placeholder" />
            </>
        };
    }

    let base64_encoded_image_data =
        qr_code_wrapper::to_png_to_base64_str_from_str(&url, QrCodeEcc::Low, 256).unwrap();
    let img = format!("data:image/png;base64,{}", base64_encoded_image_data);
    // TODO: QRコードが表示されるまで loading 表示をしたい
    html! {
        <>
            <p>
                <img style="display: inline-block; width: 1.2em; height: 1.2em; vertical-align: middle;" src="public/icon-success.svg" alt="success" />
                {"QR code was generated."}
            </p>
            <img src={img} alt={&url} />
        </>
    }
}
