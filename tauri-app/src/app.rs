use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use reqwest_wasm::{get, Client};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

async fn request_with_error() -> String {
    crate::binds::log("Requesting name from 127.0.0.1:8787");
    match get("http://127.0.0.1:8787/name").await {
        Ok(x) => {
            match x.text().await {
                Ok(y) => {
                    y
                },
                Err(e) => {
                    let error_msg = format!("Error: {}", e);
                    crate::binds::log(&error_msg);
                    String::from(error_msg)
                }
            }
        },
        Err(e) => { 
            let error_msg = format!("Error: {}", e);
            crate::binds::log(&error_msg);
            String::from(error_msg) 
        }
    }
}

async fn request_without_error() -> String {
    crate::binds::log("Requesting name from 127.0.0.1:8787");
    let builder = Client::builder().build();
    if builder.is_err() {
        let error_msg = format!("Builder Error: {}", builder.unwrap_err());
        crate::binds::log(&error_msg);
        return String::from(error_msg);
    }
    let request = builder.unwrap().get("http://127.0.0.1:8787/name_cors");
    match request.send().await {
        Ok(x) => {
            match x.text().await {
                Ok(y) => {
                    y
                },
                Err(e) => {
                    let error_msg = format!("Error: {}", e);
                    crate::binds::log(&error_msg);
                    String::from(error_msg)
                }
            }
        },
        Err(e) => { 
            let error_msg = format!("Error: {}", e);
            crate::binds::log(&error_msg);
            String::from(error_msg) 
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (greet_msg, set_greet_msg) = create_signal(String::new());

    let error = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            set_greet_msg.set(request_with_error().await);
        });
    };
    let r_without_err = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            set_greet_msg.set(request_without_error().await);
        });
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let new_msg = invoke("greet", JsValue::UNDEFINED).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };


    view! {
        <main class="container">
            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://docs.rs/leptos/" target="_blank">
                    <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
                </a>
            </div>

            <p>"Click on the Tauri and Leptos logos to learn more."</p>

            <p>
                "Recommended IDE setup: "
                <a href="https://code.visualstudio.com/" target="_blank">"VS Code"</a>
                " + "
                <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">"Tauri"</a>
                " + "
                <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">"rust-analyzer"</a>
            </p>

            <p><b>{ move || greet_msg.get() }</b></p>
            <form class="row" on:submit=error>
                <button type="err">"Cause Error"</button>
            </form>
            <form class="row" on:submit=r_without_err>
                <button type="submit">"Request w/o Error"</button>
            </form>
            <form class="row" on:submit=greet>
                <button type="submit">"Greet"</button>
            </form>
        </main>
    }
}
