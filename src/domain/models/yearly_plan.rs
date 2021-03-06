use std::collections::HashMap;

use super::{
    plot::Plot,
    yearly_sequence::YearlySequence,
};

pub type PlotPlans<'a> = HashMap<
    Plot,
    YearlySequence<'a>
>;

#[derive(Debug)]
pub struct YearlyPlan<'a> {
    plot_plans: PlotPlans<'a>
}

impl<'a> YearlyPlan<'a> {
    pub fn new(plot_plans: PlotPlans) -> YearlyPlan {
        YearlyPlan { plot_plans }
    }
}
