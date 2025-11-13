use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use protein_comparison_tool::eng::{GroceryItem, UxItem};

#[component]
fn Navbar() -> impl IntoView {
    view! {

        <div class="navbar-container">
            <div class="navbar">
                <ul class="navbar">
                    <li><a href="https://dan-codes-badly.com/">Home</a></li>
                    <li><a href="https://dan-codes-badly.com/career">Career</a></li>
                    <li><a href="https://dan-codes-badly.com/projects">Projects</a></li>
                    <li><a href="/protein-comparison-tool">Compare Protein</a></li>
                </ul>
            </div>
        </div>
    }
}

#[component]
fn Protein() -> impl IntoView {
    let (name, set_name) = signal("".to_string());
    let (protein, set_protein) = signal(0.0);
    let (calories, set_calories) = signal(0.0);
    let (cost, set_cost) = signal(0.0);
    let (servings, set_servings) = signal(1.0);
    let (grocery_items, set_grocery_items) = signal(vec![]);
    let (leanness, set_leanness) = signal(false);
    let (protein_per_dollar, set_protein_per_dollar) = signal(false);
    let ready = Memo::new(move |_| {
        if name.read().to_string().len() >= 2
            && protein.get() > 0.0
            && calories.get() > 0.0
            && cost.get() > 0.0
            && servings.get() >= 1.0
        {
            "Ready to Add \u{2705}".to_string()
        } else {
            "Invalid Values ⚠️".to_string()
        }
    });
    let sorted_leanness = Memo::new(move |_| {
        let mut _grocery: Vec<GroceryItem> = grocery_items.get();
        _grocery.sort_by(|a, b| a.leanness.cmp(&b.leanness));
        let _sorted: Vec<String> = _grocery
            .into_iter()
            .map(|item| format!("{}: {} kCal / {}g", item.name, item.calories, item.protein))
            .collect();
        _sorted
    });

    let sorted_protein_per_dollar = Memo::new(move |_| {
        let mut _grocery: Vec<GroceryItem> = grocery_items.get();
        _grocery.sort_by(|a, b| a.ppd.cmp(&b.ppd));
        let _sorted: Vec<String> = _grocery
            .into_iter()
            .map(|item| {
                format!(
                    "{}: {}g per $1",
                    item.name,
                    (item.ppd * 100.0).round() / 100.0
                )
            })
            .collect();
        _sorted
    });
    let sorted_grocery: Memo<Vec<String>> = Memo::new(move |_| {
        let mut _grocery: Vec<GroceryItem> = grocery_items.get();
        let mut _sorted: Vec<String>;
        if leanness.get() {
            _sorted = sorted_leanness.get();
        } else if protein_per_dollar.get() {
            _sorted = sorted_protein_per_dollar.get();
        } else {
            _sorted = _grocery
                .into_iter()
                .map(|item| {
                    format!(
                        "{}: {}(g) {}(kCal) {} servings",
                        item.name, item.protein, item.calories, item.servings
                    )
                })
                .collect();
        }
        _sorted
    });
    view! {
        <div class="main-container">
                <div class="flex-container">
                    <h1>"Compare Protein Sources"</h1>

                    <p style="font-style: italic; display: block">"Compare different sources of protein - animals, plant, and supplements - by leanness or cost. You can compare online deals to in-store prices."</p>

                    <div class="div-form">
                        <p class="input-status">{ready}</p>
                        <label for="name">"Item Label "
                            <div class="tooltip">" \u{24D8}"
                                <span class="tooltiptext">"A memorable label like 'Chicken', 'Protein Powder', etc. Must be 2+ characters long."</span>
                            </div>
                        </label>
                        <input class="" type="text" placeholder="(Chicken)" name="name" id="name"

                            on:input:target=move |ev| {
                                set_name.set(ev.target().value());
                            }

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
                        <input type="number" name="servings" id="servings" min=0.0 required value="1.0"
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
                                    if ready.get() == "Invalid Values ⚠️".to_string() {
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
                                }
                            >
                                "Clear"
                            </button>
                        </div>

                        <ul class="display-grocery-items">
                                <For
                                    each=move || sorted_grocery.get()
                                    key=|item| item.clone()
                                    children=move |item: String| {
                                        view! { <li>{format!("{}", item)}</li> }
                                    }
                                />
                        </ul>
                        <div class="input-form-buttons">
                            <h3 style="font-style: italic">"Sort By"</h3>
                            <input type="button" id="sort-leanness" class="form-button" on:click=move |_| {set_leanness.set(true); set_protein_per_dollar.set(true);} value="Leanness"/>
                            <input type="button" id="sort-protein-per-dollar" class="form-button" on:click=move |_| {set_leanness.set(false); set_protein_per_dollar.set(true);}value="Protein Per Dollar"/>
                            <input type="button" class="form-button clear-button" on:click=move |_| {set_grocery_items.write().clear()} value="Clear Items"/>
                        </div>
                    </div>
                </div>
            </div>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router base="/protein-comparison-tool">
            <Navbar/>
            <Routes fallback=|| view! {<h1>"WHOOPS! We Couldn't Find That Page"</h1><h3>(404 not found)</h3>}>
                <Route path=path!("/") view=Protein/>
            </Routes>
        </Router>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(App)
}
