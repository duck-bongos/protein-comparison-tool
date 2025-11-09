use crate::dioxus_core::use_drop;
use dioxus::prelude::*;
use protein_comparison_tool::protein_calculator::ProteinCalculator;
use serde::{Deserialize, Serialize};
use theme::Theme;

#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]
enum Route {
    #[route("/")]
    ProteinCalc,
}

static CSS: Asset = asset!("/assets/main.css");
static FAVICON: Asset = asset!("/assets/favicon.ico");
static RUST: Asset = asset!(
    "/assets/rust-logo-png-transparent.png",
    ImageAssetOptions::new()
        .with_size(ImageSize::Manual {
            width: 50,
            height: 50
        })
        .with_format(ImageFormat::Png)
);
static DARK_RUST: Asset = asset!(
    "/assets/rust-logo-512x512.png",
    ImageAssetOptions::new()
        .with_size(ImageSize::Manual {
            width: 50,
            height: 50
        })
        .with_format(ImageFormat::Png)
);

fn RustLogo() -> Element {
    let is_dark = Theme::System == Theme::Dark;

    rsx! {
        if is_dark {
            a { href: "https://www.rust-lang.org/", target: "_blank",
                img { src: RUST }
            }
        } else {
            a { href: "https://www.rust-lang.org/", target: "_blank",
                img { src: DARK_RUST }
            }
        }
    }
}

static DIOXUS: Asset = asset!(
    "/assets/favicon.ico",
    ImageAssetOptions::new()
        .with_size(ImageSize::Manual {
            width: 50,
            height: 50
        })
        .with_format(ImageFormat::Png)
);

#[component]
pub fn TopNav() -> Element {
    rsx! {
        div { class: "navbar",
            ul { class: "navbar",
                li {
                    Link { class: "nav-item", to: "https://dan-codes-badly.com", "[ dan-codes-badly.com ]" }
                }
            }
        }

        Outlet::<Route> {}
    }
}

#[derive(Clone)]
struct TitleState(String);
#[component]
fn Title() -> Element {
    let title = use_context::<TitleState>();
    rsx! {
        h1 { "{title.0}" }
    }
}

fn Description() -> Element {
    use_drop(|| {
        tracing::debug!("Hiding Description");
    });

    rsx! {
        div { class: "instructions-container",

            div { class: "instructions",
                p {
                    "Thank you for using my Protein Comparison Tool! I wrote this to answer the two questions:"
                }
                ol {
                    li {
                        em { "Which protein is the leanest?" }
                    }
                    li {
                        em { "Which source has the most protein per dollar?" }
                    }
                }
                p {
                    "I find it useful at the grocery store comparing natural sources to protein bars and protein powders or shopping online for different protein powder sources in bulk. This app is powered entirely by your input and does not talk to any databases. We also don't save your data because frankly, it's not worth anything and certainly not worth figuring out how to add."
                }
            }
        }
    }
}

fn Instructions() -> Element {
    rsx! {
        div { class: "instructions-container",

            div { class: "instructions",
                p { "First we'll define everything, then we'll walk through instructions." }
                h3 { "Definitions" }
                ul {
                    li {
                        em { "Protein Source Label:" }
                        " An easy label for you to remember the item - \"chicken\", \"protein powder\", etc."
                    }
                    li {
                        em { "Protein Per Serving (g):" }
                        " The nutrition label quantity for protein. Often this is labeled per serving - enter tbat."
                    }
                    li {
                        em { "Calories Per Serving:" }
                        " The nutrition label quantity for protein. Often this is labeled per serving - enter tbat."
                    }
                    li {
                        em { "Total Cost:" }
                        " The total sticker price you will play for the entire item "
                    }
                    li {
                        em { "Total Servings:" }
                        " The total number of servings in the item"
                    }
                }
                h3 { "Instructions" }
                h4 { "Adding Values" }
                ol {
                    li { "Label the item something easy to remember" }
                    li {
                        "Input the amount of protein per serving - you can find this on the label or online."
                    }
                    li {
                        "Input the amount of calories per serving - you can find this on the label or online."
                    }
                    li { "Input the total cost of the item." }
                    li { "Input the total servings for the item." }
                    li { "Then hit 'add' and watch it show up below!" }
                }
                h4 { "Sorting Values" }
                p {
                    "When you add values, their leanness and unit cost are automatically calculated. You can sort them one of two ways:"
                }
                ol {
                    li {
                        em { "Leanness: " }
                        "Calories / grams of protein. Lower is leaner."
                    }
                    li {
                        em { "Protein Per Dollar: " }
                        "(Protein x servings) / Cost. Higher is often better."
                    }
                }
            }
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        div { class: "footer-container",
            div { class: "footer",
                div { class: "footer-ack", "Powered by" }
                RustLogo {}
                a { href: "https://dioxuslabs.com/", target: "_blank",
                    img { src: DIOXUS }
                }

                div { class: "footer-site-promotion" }
            }
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        TopNav {}
        h1 { "Dan Codes Badly" }
        p {
            "Hi there. My name is Dan. I write code, *badly. Most programming paradigms I'm not very experienced, but it felt funnier to say \"badly\". To combat this self-appointed moniker - I wrote this website to learn three things:"
        }
        ol {
            li { "How to deploy a website - it worked!" }
            li {
                "How to write in Rust - opted to use the "
                Link { to: "https://dioxuslabs.com/", "Dioxus framework."}
            }
            li {
                "How to put a utility I'll use on the internet. "
                Link { to: Route::ProteinCalc, "Check it out here!" }
            }
        }
        p {
            em {
                "*badly - I'm a Machine Learning Engineer by trade and write a lot of Python for data processing - ETL & Data Engineering, Machine Learning, even Software Engineering."
            }
        }
        Footer {}
    }
}

#[component]
fn ProteinCalc() -> Element {
    let mut open_explain: Signal<bool> = use_signal(|| false);
    let mut open_describe: Signal<bool> = use_signal(|| false);
    use_context_provider(|| TitleState("Compare Protein Sources".to_string()));
    rsx! {
        document::Stylesheet { href: CSS }
        document::Link { rel: "icon", href: "https://dan-codes-badly.com/assets/favicon_cyan/favicon.ico" }

        TopNav {}
        Title {}
        if open_explain() {
            div {

                button {
                    class: "toggle-instructions",
                    onclick: move |_| open_explain.toggle(),
                    "Go Back"
                }
            }
            Instructions {}
        } else if open_describe() {
            div {
                button {
                    class: "toggle-description",
                    onclick: move |_| open_describe.toggle(),
                    "Go Back"
                }

            }
            Description {}
        } else {
            button {
                class: "toggle-description",
                onclick: move |_| open_describe.toggle(),
                "Description"
            }
            button {
                class: "toggle-instructions",
                onclick: move |_| open_explain.toggle(),
                "Instructions"
            }
            ProteinCalculator {}
        }
        Footer {}
    }
}

// fn main() {
//     launch(|| {
//         rsx! {
//             Router::<Route> {}
//         }
//     });
// }

#[component]
fn App() -> Element {
    rsx! {

        Router::<Route> {}

    }
}

fn main() {
    // #[cfg(feature = "web")]
    // dioxus_web::launch::launch(
    //     || {
    //         rsx! { Router::<Route> {}}
    //     },
    //     vec![],
    //     dioxus_web::Config::new().hydrate(false),
    // );
    dioxus::launch(App);
}
