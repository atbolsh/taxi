use std::fmt;

use crate::doormax::condition::Condition;
use crate::doormax::condition_learner::ConditionLearner;
use crate::doormax::effect;
use crate::doormax::effect::{ChangePassenger, ChangeTaxiX, ChangeTaxiY, Effect};

use crate::actions::Actions;
use crate::state::State;
use crate::world::World;

#[derive(Debug, Clone)]
pub struct CELearner<E: Effect> {
    condition_effects: Vec<(ConditionLearner, E)>,
}

impl<E: Effect> CELearner<E> {
    pub fn new() -> Self {
        CELearner {
            condition_effects: Vec::new(),
        }
    }

    pub fn predict(
        &self,
        world: &World,
        state: &State,
        condition: &Condition,
    ) -> Result<Option<State>, effect::Error> {
        let mut full_result = None;

        for &(ref condition_learner, ref learned_effect) in &self.condition_effects {
            let matches_condition = condition_learner.predict(condition);
            match matches_condition {
                // A condition learner returns None if it does not have enough
                // information to know if this condition applies.  So we
                // return None to show that this needs to be explored.
                None => {
                    return Ok(None);
                }

                // If the condition does not match this learner, ignore it.
                Some(false) => (),

                // There is a match.  If we supported multiple effect types per
                // learner, there could be a conflict (ie. a set value and increment
                // value effect could have been learned for the same condition).  This
                // code does not really support multiple effect types per learner, but
                // we go ahead and pretend it does just to show where the conflict checking
                // needs to be.
                Some(true) => {
                    let result = learned_effect.apply(world, state)?;

                    if let Some(full_result) = full_result {
                        if full_result != result {
                            // Conflicting result
                            return Ok(None);
                        }
                    } else {
                        full_result = Some(result);
                    }
                }
            };
        }

        if full_result.is_some() {
            Ok(full_result)
        } else {
            // full_result is None only if we know that
            // this condition does not match any effects.
            // Hence, full_result == None does _not_ mean
            // unknown effect (which is what return Ok(None) means),
            // but instead it means that there is no effect on the
            // state.
            Ok(Some(*state))
        }
    }

    pub fn apply_experience(&mut self, condition: &Condition, old_state: &State, new_state: &State)
    where
        E: Clone + PartialEq,
    {
        let observed_effect = E::generate_effects(old_state, new_state);

        match observed_effect {
            None => {
                for &mut (ref mut condition_learner, _) in &mut self.condition_effects {
                    condition_learner.apply_experience(condition, false);
                }
            }

            Some(observed_effect) => {
                let mut found_entry = false;
                for &mut (ref mut condition_learner, ref learned_effect) in
                    &mut self.condition_effects
                {
                    if observed_effect == *learned_effect {
                        condition_learner.apply_experience(condition, true);
                        found_entry = true;
                    } else {
                        condition_learner.apply_experience(condition, false);
                    }
                }

                if !found_entry {
                    let mut condition_learner = ConditionLearner::new();
                    condition_learner.apply_experience(condition, true);

                    for &(ref other_condition_learner, _) in &self.condition_effects {
                        condition_learner.remove_overlap(other_condition_learner);
                    }

                    self.condition_effects
                        .push((condition_learner, observed_effect));
                } else {
                    // Check for overlapping conditions.
                    if !self.condition_effects.is_empty() {
                        let mut has_conflict = false;

                        for i in 0..(self.condition_effects.len() - 1) {
                            let &(ref condition_learner, _) = &self.condition_effects[i];

                            for j in (i + 1)..self.condition_effects.len() {
                                let &(ref other_condition_learner, _) = &self.condition_effects[j];

                                // overlaps checks if either learner's truth hypothesis
                                // is contained in the other's
                                if condition_learner.overlaps(other_condition_learner) {
                                    has_conflict = true;
                                    break;
                                }
                            }
                        }

                        if has_conflict {
                            self.condition_effects = Vec::new();
                        }
                    }
                }
            }
        }
    }
}

impl<E: Effect> Default for CELearner<E> {
    fn default() -> Self {
        CELearner::new()
    }
}

impl<E: Effect + fmt::Display> fmt::Display for CELearner<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CL(")?;
        let mut leader = " ";
        for &(ref condition_learner, ref learned_effect) in &self.condition_effects {
            write!(f, "{}{} => {}", leader, condition_learner, learned_effect)?;
            leader = ", ";
        }
        write!(f, " )")
    }
}

#[derive(Debug, Clone)]
pub struct MCELearner {
    taxi_x_learners: [CELearner<ChangeTaxiX>; Actions::NUM_ELEMENTS],
    taxi_y_learners: [CELearner<ChangeTaxiY>; Actions::NUM_ELEMENTS],
    passenger_learners: [CELearner<ChangePassenger>; Actions::NUM_ELEMENTS],
}

impl MCELearner {
    pub fn new() -> Self {
        MCELearner {
            taxi_x_learners: Default::default(),
            taxi_y_learners: Default::default(),
            passenger_learners: Default::default(),
        }
    }

    pub fn predict(
        &self,
        world: &World,
        state: &State,
        action: Actions,
    ) -> Result<Option<State>, effect::Error> {
        let condition = Condition::new(world, state);
        let action_index = action.to_index();

        if let Some(predicted_taxi_x) =
            self.taxi_x_learners[action_index].predict(world, state, &condition)?
        {
            if let Some(predicted_taxi_y) =
                self.taxi_y_learners[action_index].predict(world, state, &condition)?
            {
                if let Some(predicted_passenger) =
                    self.passenger_learners[action_index].predict(world, state, &condition)?
                {
                    return Ok(Some(State::build(
                        world,
                        (predicted_taxi_x.get_taxi().x, predicted_taxi_y.get_taxi().y),
                        predicted_passenger.get_passenger(),
                        state.get_destination(),
                    )?));
                }
            }
        }

        Ok(None)
    }

    pub fn apply_experience(
        &mut self,
        world: &World,
        state: &State,
        action: Actions,
        new_state: &State,
    ) {
        let condition = Condition::new(world, state);
        let action_index = action.to_index();

        self.taxi_x_learners[action_index].apply_experience(&condition, state, new_state);
        self.taxi_y_learners[action_index].apply_experience(&condition, state, new_state);
        self.passenger_learners[action_index].apply_experience(&condition, state, new_state);
    }
}

impl fmt::Display for MCELearner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "taxi_x:")?;
        for action_index in 0..Actions::NUM_ELEMENTS {
            let action = Actions::from_index(action_index).unwrap();
            writeln!(f, "{} - {}", action, self.taxi_x_learners[action_index])?;
        }
        writeln!(f)?;

        writeln!(f, "taxi_y:")?;
        for action_index in 0..Actions::NUM_ELEMENTS {
            let action = Actions::from_index(action_index).unwrap();
            writeln!(f, "{} - {}", action, self.taxi_y_learners[action_index])?;
        }
        writeln!(f)?;

        writeln!(f, "passenger:")?;
        for action_index in 0..Actions::NUM_ELEMENTS {
            let action = Actions::from_index(action_index).unwrap();
            writeln!(f, "{} - {}", action, self.passenger_learners[action_index])?;
        }
        writeln!(f)?;

        Ok(())
    }
}

#[cfg(test)]
mod mcelearner_test {
    use super::*;
    use crate::position::Position;
    use crate::world::Costs;

    #[test]
    fn learns_taxi_east_simple() {
        let source_world = "\
                            ┌───┬─────┐\n\
                            │R .│. . .│\n\
                            │   │     │\n\
                            │. .│G . .│\n\
                            │         │\n\
                            │. . . . .│\n\
                            │         │\n\
                            │.│Y .│B .│\n\
                            │ │   │   │\n\
                            │.│. .│. .│\n\
                            └─┴───┴───┘\n\
                            ";

        let costs = Costs::default();
        let w = World::build_from_str(source_world, costs).unwrap();

        let old_state = State::build(&w, (1, 3), Some('R'), 'B').unwrap();
        let (_, new_state) = old_state.apply_action(&w, Actions::East);
        assert_eq!(new_state.get_taxi(), Position::new(2, 3));

        let mut learner = MCELearner::new();
        learner.apply_experience(&w, &old_state, Actions::East, &new_state);

        let predicted_0 = learner.predict(&w, &old_state, Actions::East).unwrap();
        assert_eq!(predicted_0, Some(new_state));
    }

    #[test]
    fn learns_taxi_east_full() {
        let source_world = "\
                            ┌───┬─────┐\n\
                            │R .│. . .│\n\
                            │   │     │\n\
                            │. .│G . .│\n\
                            │         │\n\
                            │. . . . .│\n\
                            │         │\n\
                            │.│Y .│B .│\n\
                            │ │   │   │\n\
                            │.│. .│. .│\n\
                            └─┴───┴───┘\n\
                            ";
        let costs = Costs::default();
        let w = World::build_from_str(source_world, costs).unwrap();

        let clear_state = State::build(&w, (1, 2), Some('R'), 'B').unwrap();
        let (_, clear_final_state) = clear_state.apply_action(&w, Actions::East);
        assert_eq!(clear_final_state.get_taxi(), Position::new(2, 2));

        let mut learner = MCELearner::new();
        learner.apply_experience(&w, &clear_state, Actions::East, &clear_final_state);

        let predicted_0 = learner.predict(&w, &clear_state, Actions::East).unwrap();
        assert_eq!(predicted_0, Some(clear_final_state));

        let blocked_state = State::build(&w, (1, 1), Some('R'), 'B').unwrap();
        let (_, blocked_final_state) = blocked_state.apply_action(&w, Actions::East);
        assert_eq!(blocked_final_state.get_taxi(), Position::new(1, 1));

        learner.apply_experience(&w, &blocked_state, Actions::East, &blocked_final_state);

        let predicted_0b = learner.predict(&w, &clear_state, Actions::East).unwrap();
        assert_eq!(predicted_0b, Some(clear_final_state));

        let predicted_1 = learner.predict(&w, &blocked_state, Actions::East).unwrap();
        assert_eq!(predicted_1, Some(blocked_final_state));
    }
}
