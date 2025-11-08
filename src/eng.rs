use core::panic;
use ordered_float::OrderedFloat;
use std::fmt::Display;

#[derive(Clone)]
pub struct UxItem<T: Into<f64>, U: Into<f64>, V: Into<f64>, W: Into<f64>> {
    pub protein: T,
    pub calories: U,
    pub cost: V,
    pub servings: W,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroceryItem {
    pub protein: OrderedFloat<f64>,
    pub calories: OrderedFloat<f64>,
    pub cost: OrderedFloat<f64>,
    pub servings: OrderedFloat<f64>,
    pub name: String,
    pub leanness: OrderedFloat<f64>, // lower is better
    pub ppd: OrderedFloat<f64>,      // protein per dollar
    pub lpd: OrderedFloat<f64>,      // leanness per dollar
}

impl<T, U, V, W> UxItem<T, U, V, W>
where
    f64: From<T>,
    f64: From<U>,
    f64: From<V>,
    f64: From<W>,
{
    pub fn to_grocery(self) -> GroceryItem {
        let _protein: OrderedFloat<f64> = OrderedFloat(self.protein.into());
        let _calories: OrderedFloat<f64> = OrderedFloat(self.calories.into());
        let _cost: OrderedFloat<f64> = OrderedFloat(self.cost.into());
        let _servings: OrderedFloat<f64> = OrderedFloat(self.servings.into());
        let _leanness: OrderedFloat<f64> = calc_leanness(&_protein, &_calories);
        let _ppd: OrderedFloat<f64> = calc_protein_per_dollar(&_protein, &_cost, &_servings);
        let _lpd: OrderedFloat<f64> =
            calc_leanness_per_dollar(&_protein, &_calories, &_cost, &_servings);

        GroceryItem {
            protein: _protein,
            calories: _calories,
            cost: _cost,
            servings: _servings,
            name: self.name,
            leanness: _leanness,
            ppd: _ppd,
            lpd: _lpd,
        }
    }
}

impl Display for GroceryItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} | P: {:.2} Cal: {:.2} Servings: {:.2} - ${:.2}",
            self.name, self.protein, self.calories, self.servings, self.cost,
        )
    }
}

pub fn calc_leanness(
    protein: &OrderedFloat<f64>,
    calories: &OrderedFloat<f64>,
) -> OrderedFloat<f64> {
    /* Lower is better. Sort ascending! */
    if *protein == 0.0 {
        return OrderedFloat(0.0);
    }

    let result = calories / protein;
    if result.is_infinite() {
        panic!("Result is infinite");
    } else if result.is_nan() {
        panic!("Result is not a number (NaN).");
    } else {
        result
    }
}

pub fn calc_protein_per_dollar(
    protein: &OrderedFloat<f64>,
    cost: &OrderedFloat<f64>,
    servings: &OrderedFloat<f64>,
) -> OrderedFloat<f64> {
    /* Higher is better. Sort descending! */
    if *cost <= OrderedFloat(0.0) {
        return OrderedFloat(0.0);
    }

    let result = (protein * servings) / cost;

    if result.is_infinite() {
        panic!("Result is infinite");
    } else if result.is_nan() {
        panic!("Result is not a number (NaN).");
    } else {
        result
    }
}

pub fn calc_leanness_per_dollar(
    protein: &OrderedFloat<f64>,
    calories: &OrderedFloat<f64>,
    cost: &OrderedFloat<f64>,
    servings: &OrderedFloat<f64>,
) -> OrderedFloat<f64> {
    /* Higher is better. Sort ascending! */
    if *cost == 0.0 {
        return OrderedFloat(0.0);
    }

    if *calories == 0.0 {
        return OrderedFloat(0.0);
    }

    let _leanness = calc_leanness(&protein, &calories);

    let result = (_leanness * servings) / cost;

    if result.is_infinite() {
        panic!("Result is infinite");
    } else if result.is_nan() {
        panic!("Result is not a number (NaN).");
    } else {
        result
    }
}
