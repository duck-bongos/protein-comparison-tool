use leptos::mount::mount_to_body;
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (name, set_name) = signal("".to_string());
    let (protein, set_protein) = signal(0.0);
    let (calories, set_calories) = signal(0.0);
    let (cost, set_cost) = signal(0.0);
    let (servings, set_servings) = signal(1.0);
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
            <p class="input-status">{ready}" "{name}": "{calories}" "{protein}" "{cost}" "{servings}</p>
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
                <input class="form-button" type="submit" value="Add"/>
                <input class="form-button clear-button" type="reset" value="Clear"/>
            </div>
            </div>
            <div class="input-form-buttons">
            <h3 style="font-style: italic">"Sort By"</h3>
            <input type="button" class="form-button" value="Lean-ness"/>
            <input type="button" class="form-button" value="Protein Per Dollar"/>
            <input type="button" class="form-button clear-button" value="Clear Items"/>
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
