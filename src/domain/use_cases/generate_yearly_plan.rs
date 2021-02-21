use std::collections::HashMap;

use super::super::models::{
    in_days::InDays,
    plot::Plot,
    sequence::Sequence,
    yearly_plan::{ PlotPlans, YearlyPlan },
    yearly_sequence::YearlySequence,
};

pub fn generate_yearly_plan<'a>(
    sequences: &'a Vec<Sequence>,
    plots: Vec<Plot>
) -> YearlyPlan<'a> {

    let mut plot_plans: PlotPlans = HashMap::new();

    for plot in plots {
        let ys = genereate_yearly_sequence(sequences);
        plot_plans.insert(plot, ys);
    }

    YearlyPlan::new(plot_plans)
}

fn genereate_yearly_sequence<'a>(
    sequences: &'a Vec<Sequence<'a>>,
) -> YearlySequence<'a> {
    static SEASON_DURATION: u32 = 200;

    let mut cumulated_duration = 0;
    let mut current_index = 0;
    let mut year_sequences: Vec<InDays<Sequence>> = vec![];
    let seq_count = sequences.len();

    while cumulated_duration < SEASON_DURATION {
        let ix = current_index % seq_count;
        year_sequences.push(
            InDays::new(
                cumulated_duration,
                &sequences[ix]
            )
        );
        cumulated_duration += sequences[ix].duration();
        current_index += 1;
    };

    YearlySequence::new(year_sequences).unwrap()
}
