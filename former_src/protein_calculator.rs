use dioxus::prelude::*;

use crate::eng::{GroceryItem, UxItem};

#[derive(Clone)]
struct TitleState(String);
#[component]
fn Title() -> Element {
    let title = use_context::<TitleState>();
    rsx! {
        h1 { "{title.0}" }
    }
}

pub fn ProteinCalculator() -> Element {
    let mut name: Signal<String> = use_signal(|| String::from(""));
    let mut protein = use_signal(|| 0.0);
    let mut calories = use_signal(|| 0.0);
    let mut cost = use_signal(|| 0.0);
    let mut servings = use_signal(|| 1.0);
    let mut grocery_items: Signal<Vec<GroceryItem>> = use_signal(|| vec![]);
    let mut sort_label: Signal<String> = use_signal(|| String::from(""));
    let mut numerator: Signal<String> = use_signal(|| String::from(""));
    let mut denominator: Signal<String> = use_signal(|| String::from(""));
    let mut sort_label_descriptor: Signal<String> = use_signal(|| String::from(""));
    let mut leanness: Signal<bool> = use_signal(|| false);
    let mut protein_per_dollar: Signal<bool> = use_signal(|| false);
    let mut zero_warning: Signal<bool> = use_signal(|| false);

    rsx! {

        div { class: "flex-container",
            div { class: "div-form",
                h3 { "Input Factors" }
                div { class: "form-group",
                    div { class: "floating-label",
                        label { r#for: "name", "A Label like \n'Chicken', 'Protein Powder', etc:" }
                    }
                    input {
                        r#type: "text",
                        id: "name",
                        name: "name",
                        value: "{name}",
                        style: "max-width: 95%",
                        oninput: move |e| { name.set(e.value()) },
                    }
                }
                div { class: "form-group",
                    div { class: "floating-label",
                        label { r#for: "protein", "Protein per serving (g):" }
                    }
                    input {
                        r#type: "number",
                        id: "protein",
                        name: "protein",
                        value: protein,
                        style: "max-width: 95%",
                        oninput: move |e| {
                            let _is = e.value();
                            match _is.parse::<f64>() {
                                Ok(parsed_val) => {
                                    protein.set(parsed_val);
                                }
                                Err(_) => tracing::info!("Protein must be above 0"),
                            }
                        },
                    }
                }
                div { class: "form-group",
                    div { class: "floating-label",
                        label { r#for: "calories", "Calories per Serving:" }
                    }
                    input {
                        r#type: "number",
                        id: "calories",
                        name: "calories",
                        value: calories,
                        style: "max-width: 95%",
                        oninput: move |e| {
                            let _is = e.value();
                            match _is.parse::<f64>() {
                                Ok(parsed_val) => calories.set(parsed_val),
                                Err(_) => tracing::info!("Calories must be a number greater than 0."),
                            }
                        },
                    }
                }
                div { class: "form-group",
                    div { class: "floating-label",
                        label { r#for: "cost", "Total Cost:" }
                    }
                    input {
                        r#type: "number",
                        id: "cost",
                        name: "cost",
                        min: 0.01,
                        value: cost,
                        style: "max-width: 95%",
                        oninput: move |e| {
                            let _is = e.value();
                            match _is.parse::<f64>() {
                                Ok(parsed_val) => cost.set(parsed_val),
                                Err(_) => tracing::info!("Cost must be greater than 0."),
                            }
                        },
                    }
                }
                div { class: "form-group",

                    div { class: "floating-label",
                        label { r#for: "servings", "Total Servings:" }
                    }
                    input {
                        r#type: "number",
                        id: "servings",
                        name: "servings",
                        value: servings,
                        style: "max-width: 95%",
                        oninput: move |e| {
                            let _is = e.value();
                            match _is.parse::<f64>() {
                                Ok(parsed_val) => servings.set(parsed_val),
                                Err(_) => tracing::info!("Servings must be greater than 0."),
                            }
                        },
                    }
                }


                div { class: "input-form-buttons",
                    ErrorBoundary {
                        handle_error: |_| {
                            rsx! { "Oops we encountered an error! Please ensure none of the values are set to 0 and retry!" }
                        },
                        input {
                            class: "form-button",
                            r#type: "submit",
                            value: "Add",
                            onclick: move |_| {
                                let statuses = vec![protein(), calories(), cost(), servings()];
                                if statuses.iter().any(|&i| i == 0.0) {
                                    zero_warning.set(true);
                                } else {
                                    let _uxi = UxItem {
                                        name: name(),
                                        protein: protein(),
                                        calories: calories(),
                                        cost: cost(),
                                        servings: servings(),
                                    }
                                        .to_grocery();
                                    grocery_items.push(_uxi);
                                    name.set(String::new());
                                    protein.set(0.0);
                                    calories.set(0.0);
                                    cost.set(0.0);
                                    servings.set(1.0);
                                    zero_warning.set(false);
                                }
                            },
                        }
                    }


                    input {
                        class: "form-button clear-button",
                        r#type: "reset",
                        value: "Clear",
                        onclick: move |_| {
                            name.set(String::new());
                            protein.set(0.0);
                            calories.set(0.0);
                            cost.set(0.0);
                            servings.set(1.0);
                            zero_warning.set(false);
                        },
                    }
                }

                if zero_warning() {
                    div { class: "zero-warning", "Please ensure all values entered are non-zero!" }
                }
            }


            div { id: "protein-items", class: "div-form",
                h3 { "Protein Items" }
                div { class: "output-list",
                    ol {
                        if leanness() {
                            for item in grocery_items.read().iter() {

                                li {
                                    "{item.name}"
                                    br {}
                                    em { "{item.calories} kCal / {item.protein}g" }
                                }
                            }
                        } else if protein_per_dollar() {
                            for item in grocery_items.read().iter().rev() {

                                li {
                                    "{item.name}:"
                                    em { " {(item.ppd*100.0).round() / 100.0}g per $1" }
                                }
                            }
                        } else {
                            for item in grocery_items.read().iter() {

                                li {
                                    "{item.name}: "
                                    em { class: "display-item",
                                        "{item.protein*item.servings}g protein {item.calories*item.servings} kCal ${item.cost}"
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "input-form-buttons",

                    input {
                        class: "form-button",
                        r#type: "submit",
                        value: "Leanness",
                        onclick: move |_| {
                            tracing::info!("Leanness: {:?}", grocery_items());
                            grocery_items.write().sort_by(|a, b| a.leanness.cmp(&b.leanness));
                            sort_label.set(String::from("Leanness ="));
                            numerator.set(String::from("Calories"));
                            denominator.set(String::from("Protein"));
                            leanness.set(true);
                            protein_per_dollar.set(false);
                        },
                    }
                    input {
                        class: "form-button",
                        r#type: "submit",
                        value: "Protein per Dollar",
                        onclick: move |_| {
                            tracing::info!("Protein per Dollar: {:?}", grocery_items());
                            grocery_items.write().sort_by(|a, b| a.ppd.cmp(&b.ppd));
                            sort_label.set(String::from("Protein per Dollar ="));
                            numerator.set(String::from("Protein * Servings"));
                            denominator.set(String::from("Total Cost"));
                            sort_label_descriptor.set(String::from(""));
                            protein_per_dollar.set(true);
                            leanness.set(false);
                        },
                    }
                    input {
                        class: "form-button clear-button",
                        r#type: "reset",
                        value: "Clear Items",
                        onclick: move |_| {
                            grocery_items.set(vec![]);
                            numerator.set(String::from(""));
                            denominator.set(String::from(""));
                            sort_label_descriptor.set(String::from(""));
                            sort_label.set(String::from(""));
                            leanness.set(false);
                            protein_per_dollar.set(false);
                        },
                    }
                }
            
            }
        }
    }
}
