use leptos::mount::mount_to_body;
use leptos::prelude::*;
use protein_comparison_tool::eng::{GroceryItem, UxItem};

// let mut grocery_items: Signal<Vec<GroceryItem>> = use_signal(|| vec![]);
// let mut sort_label: Signal<String> = use_signal(|| String::from(""));
// let mut numerator: Signal<String> = use_signal(|| String::from(""));
// let mut denominator: Signal<String> = use_signal(|| String::from(""));
// let mut sort_label_descriptor: Signal<String> = use_signal(|| String::from(""));
// let mut leanness: Signal<bool> = use_signal(|| false);
// let mut protein_per_dollar: Signal<bool> = use_signal(|| false);
// let mut zero_warning: Signal<bool> = use_signal(|| false);

#[component]
fn App() -> impl IntoView {
    let (name, set_name) = signal("".to_string());
    let (protein, set_protein) = signal(0.0);
    let (calories, set_calories) = signal(0.0);
    let (cost, set_cost) = signal(0.0);
    let (servings, set_servings) = signal(1.0);
    let (zero_warning, set_zero_warning) = signal(false);
    let (grocery_items, set_grocery_items) = signal(vec![]);
    let ready = Memo::new(move |_| {
        if name.read().to_string().len() >= 2
            && protein.get() > 0.0
            && calories.get() > 0.0
            && cost.get() > 0.0
            && servings.get() >= 1.0
        {
            "Ready to Add \u{2705}".to_string()
        } else {
            "Invalid Values \u{274C}".to_string()
        }
    });

    view! {
        <div class="main-container">
            <div class="flex-container">
                <h1>"Compare Protein Sources"</h1>

                <p style="font-style: italic; display: block">"Compare different sources of protein - animals, plant, and supplements - by leanness or cost. You can compare online deals to in-store prices."</p>

                <div class="div-form">
                    // <p class="input-status">{ready}" "{name}": "{calories}" "{protein}" "{cost}" "{servings}</p>
                    <p class="input-status">{ready}</p>
                    <label for="name">"Item Label "
                        <div class="tooltip">" \u{24D8}"
                            <span class="tooltiptext">"A memorable label like 'Chicken', 'Protein Powder', etc. Must be 2+ characters long."</span>
                        </div>
                    </label>
                    <input class="" type="text" placeholder="(Chicken)" name="name" id="name"
                        // adding :target gives us typed access to the element
                        // that is the target of the event that fires
                        on:input:target=move |ev| {
                            // .value() returns the current value of an HTML input element
                            set_name.set(ev.target().value());
                        }
                        // the `prop:` syntax lets you update a DOM property,
                        // rather than an attribute.
                        prop:value=name
                    />

                    <label for="protein">"Protein (g)"</label>
                    <input type="number" name="protein" id="protein" required min=0.0
                        on:change=move |ev: leptos::ev::Event| {
                            let value_str = event_target_value(&ev);
                            if let Ok(num) = value_str.parse::<f64>() {
                                set_protein.set(num);
                            }
                        }
                    prop:value=protein/>
                    <label for="calories">"Calories"</label>
                    <input type="number" name="calories" id="calories" required min=0.0
                        on:change=move |ev: leptos::ev::Event| {
                            let value_str = event_target_value(&ev);
                            if let Ok(num) = value_str.parse::<f64>() {
                                set_calories.set(num);
                            }
                        }
                    prop:value=calories/>
                    <label for="cost">"Cost"</label>
                    <input type="number" name="cost" id="cost" min=0.0 required
                        on:change=move |ev: leptos::ev::Event| {
                            let value_str = event_target_value(&ev);
                            if let Ok(num) = value_str.parse::<f64>() {
                                set_cost.set(num);
                            }
                        }
                    prop:value=cost/>
                    <label for="servings">"Servings"</label>
                    <input type="number" name="servings" id="servings" min=1 required value="1.0"
                        on:change=move |ev: leptos::ev::Event| {
                            let value_str = event_target_value(&ev);
                            if let Ok(num) = value_str.parse::<f64>() {
                                set_servings.set(num);
                            }
                        }

                    prop:value=servings/>

                    <div class="input-form-buttons">
                        <button
                            on:click=move |_| {
                                if ready.get() == "Invalid Values \u{274C}".to_string() {
                                    println!("Not OK!");
                                }
                                else {

                                    let _uxi = UxItem {
                                        name: name.read().to_string(),
                                        protein: protein.get(),
                                        calories: calories.get(),
                                        cost: cost.get(),
                                        servings: servings.get(),
                                    }.to_grocery();
                                    set_grocery_items.write().push(_uxi);
                                    set_name.set("".to_string());
                                    set_protein.set(0.0);
                                    set_calories.set(0.0);
                                    set_cost.set(0.0);
                                    set_servings.set(1.0);
                                    set_zero_warning.set(false);
                                }
                            }

                        >
                            "Add"

                        </button>
                        <button
                            on:click=move |_| {
                                set_name.set("".to_string());
                                set_protein.set(0.0);
                                set_calories.set(0.0);
                                set_cost.set(0.0);
                                set_servings.set(1.0);
                                set_zero_warning.set(false);
                            }
                        >
                            "Clear"
                        </button>
                    </div>

                    <ul class="display-grocery-items">
                            <For
                                each=move || grocery_items.get()
                                key=|item| item.name.clone()
                                children=move |item: GroceryItem| {
                                    view! { <li>{format!("{}", item)}</li> }
                                }
                            />
                    </ul>
                    <div class="input-form-buttons">
                        <h3 style="font-style: italic">"Sort By"</h3>
                        <input type="button" class="form-button" on:click=move |_| {set_grocery_items.write().sort_by(|a, b| a.leanness.cmp(&b.leanness));} value="Lean-ness"/>
                        <input type="button" class="form-button" on:click=move |_| {set_grocery_items.write().sort_by(|a, b| a.ppd.cmp(&b.ppd));}value="Protein Per Dollar"/>
                        <input type="button" class="form-button clear-button" on:click=move |_| {set_grocery_items.write().clear()} value="Clear Items"/>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(App)
}
