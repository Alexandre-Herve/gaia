mod domain;

use domain::models::{
    in_days::InDays,
    plot::Plot,
    sequence::Sequence,
    variety::Variety,
};

use domain::use_cases::{
    generate_yearly_plan::generate_yearly_plan
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
    let tomate = Variety::new(String::from("Tomate"), 120);
    let radis = Variety::new(String::from("Radis"), 14);
    let chou = Variety::new(String::from("Chou"), 60);

    // Sequences
    let s1 = Sequence::new(vec![
        InDays(0, &tomate),
        InDays(0, &radis),
        InDays(14, &radis),
    ]);
    let s2 = Sequence::new(vec![
        InDays(0, &chou),
        InDays(14, &radis),
        InDays(21, &radis),
    ]);

    // Yearly plan
    let sequences = vec![s1, s2];
    let plots = vec![plot1, plot2];

    let yearly_plan = generate_yearly_plan(
        &sequences,
        plots
    );

    println!("{:?}", yearly_plan);
}
