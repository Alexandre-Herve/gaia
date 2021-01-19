mod domain;

use std::collections::HashMap;

use domain::models::{
    in_days::InDays,
    plot::Plot,
    sequence::Sequence,
    variety::Variety,
    yearly_plan::YearlyPlan,
    yearly_sequence::YearlySequence,
};

fn main() {
    // Garden
    let plot1 = Plot::new(
        String::from("1"),
        String::from("Parcelle du pré")
    );

    let plot2 = Plot::new(
        String::from("2"),
        String::from("Parcelle de l'étang")
    );

    // Varieties
    let tomate = Variety::new(String::from("Tomate"));
    let tomate = InDays::new(0, &tomate);

    let radis = Variety::new(String::from("Radis"));
    let radis = InDays::new(10, &radis);

    // Sequences
    let s1 = Sequence::new(vec![&tomate, &radis]);
    let s1 = InDays::new(0, &s1);

    let s2 = Sequence::new(vec![&radis, &radis]);
    let s2 = InDays::new(0, &s2);

    let yearly_sequence = YearlySequence::new(vec![&s1, &s2]);

    // Plan
    let mut plot_plans = HashMap::new();

    plot_plans.insert(plot1, &yearly_sequence);
    plot_plans.insert(plot2, &yearly_sequence);

    let yearly_plan = YearlyPlan::new(plot_plans);

    println!("{:?}", yearly_plan);
}
