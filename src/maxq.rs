

use rand::Rng;

use state::State;
use actions::Actions;
use world::World;
use state_indexer::StateIndexer;

use runner::{Runner, Attempt};


#[derive(Debug, Clone)]
struct PrimitiveNode {
    action: Actions,
    values: Vec<f64>, // indexed by state_index
}

#[derive(Debug, Clone, Copy)]
enum CompoundNodeType {
    Root,
    Get,
    Put,
    Navigate(char),
}

#[derive(Debug, Clone)]
struct CompoundNode {
    compound_type: CompoundNodeType,
    children: Vec<usize>,
    completion: Vec<f64>, // indexed by state_index*children.len() + child_offset
}

#[derive(Debug, Clone)]
enum MaxNode {
    Primitive(PrimitiveNode),
    Compound(CompoundNode),
}


#[derive(Debug, Clone)]
pub struct MaxQ {
    alpha: f64,
    gamma: f64,
    epsilon: f64,

    state_indexer: StateIndexer,

    max_nodes: Vec<MaxNode>,
}

impl MaxQ {
    pub fn new(world: &World, alpha: f64, gamma: f64, epsilon: f64) -> MaxQ {

        let initial_q_value = world.max_reward() / (1.0 - gamma);

        let state_indexer = StateIndexer::new(world);
        let num_states = state_indexer.num_states();

        let num_navigates = world.num_fixed_positions();

        let start_get = 1;
        let start_put = start_get + 1;
        let start_navigates = start_put + 1; // put + drop-off
        let start_actions = start_navigates + num_navigates;

        let num_nodes = start_actions + Actions::NUM_ELEMENTS;

        let mut max_nodes = Vec::with_capacity(num_nodes);


        let root_node = CompoundNode {
            compound_type: CompoundNodeType::Root,
            children: vec![start_get, start_put],
            completion: vec![initial_q_value; num_states * 2],
        };

        max_nodes.push(MaxNode::Compound(root_node));

        assert!(max_nodes.len() == start_get);
        let get_node = {

            let num_children = 1 + num_navigates;
            let mut children = Vec::with_capacity(num_children);

            for i in 0..num_navigates {
                children.push(start_navigates + i);
            }

            children.push(start_actions + Actions::PickUp.to_index());

            CompoundNode {
                compound_type: CompoundNodeType::Get,
                children,
                completion: vec![initial_q_value; num_states * num_children],
            }
        };
        max_nodes.push(MaxNode::Compound(get_node));


        assert!(max_nodes.len() == start_put);
        let put_node = {
            let num_children = 1 + num_navigates;
            let mut children = Vec::with_capacity(num_children);
            children.push(start_actions + Actions::DropOff.to_index());

            for i in 0..num_navigates {
                children.push(start_navigates + i);
            }

            CompoundNode {
                compound_type: CompoundNodeType::Put,
                children,
                completion: vec![initial_q_value; num_states * num_children],
            }
        };
        max_nodes.push(MaxNode::Compound(put_node));

        assert!(max_nodes.len() == start_navigates);
        let movement_action_indices = vec![
            start_actions + Actions::North.to_index(),
            start_actions + Actions::South.to_index(),
            start_actions + Actions::East.to_index(),
            start_actions + Actions::West.to_index(),
        ];

        for fixed_position_index in 0..world.num_fixed_positions() {
            let id = world.get_fixed_id_from_index(fixed_position_index).unwrap();

            let children = movement_action_indices.clone();

            let navigate_node = CompoundNode {
                compound_type: CompoundNodeType::Navigate(id),
                children,
                completion: vec![initial_q_value; num_states * 4],
            };
            max_nodes.push(MaxNode::Compound(navigate_node));
        }

        assert!(max_nodes.len() == start_actions);
        for action_index in 0..Actions::NUM_ELEMENTS {
            let action = Actions::from_index(action_index).unwrap();

            max_nodes.push(MaxNode::Primitive(PrimitiveNode {
                action,
                values: vec![initial_q_value; num_states],
            }))
        }

        assert!(max_nodes.len() == num_nodes);

        MaxQ {
            alpha,
            gamma,
            epsilon,

            state_indexer,

            max_nodes,
        }
    }

    fn evaluate_max_node(
        &self,
        node_index: usize,
        world: &World,
        state: &State,
        state_index: usize,
    ) -> Option<(f64, usize, usize)> {
        match self.max_nodes[node_index] {
            MaxNode::Primitive(ref primitive) => Some(
                (primitive.values[state_index], node_index, 0),
            ),
            MaxNode::Compound(ref compound) => {
                if !self.terminal_state(compound.compound_type, world, state) {

                    let num_children = compound.children.len();

                    let mut highest_q = None;
                    let mut best_index = node_index;
                    let mut best_offset = 0;
                    let mut best_value = 0.0;

                    for (offset, child_index) in compound.children.iter().enumerate() {
                        if let Some((max_value, _, _)) =
                            self.evaluate_max_node(*child_index, world, state, state_index)
                        {

                            let q = Some(
                                max_value +
                                    compound.completion[state_index * num_children + offset],
                            );

                            if q > highest_q {
                                highest_q = q;
                                best_index = *child_index;
                                best_offset = offset;
                                best_value = max_value;
                            }
                        }
                    }

                    if highest_q != None {
                        Some((best_value, best_index, best_offset))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }

    fn evaluate_max_node_action(
        &self,
        node_index: usize,
        world: &World,
        state: &State,
        state_index: usize,
    ) -> Option<(f64, Actions)> {
        match self.max_nodes[node_index] {
            MaxNode::Primitive(ref primitive) => Some(
                (primitive.values[state_index], primitive.action),
            ),
            MaxNode::Compound(ref compound) => {
                if !self.terminal_state(compound.compound_type, world, state) {
                    let num_children = compound.children.len();

                    let mut highest_q = None;
                    let mut best_action = Actions::PickUp;
                    let mut best_value = 0.0;

                    for (offset, child_index) in compound.children.iter().enumerate() {

                        if let Some((max_value, max_action)) =
                            self.evaluate_max_node_action(*child_index, world, state, state_index)
                        {
                            let q = Some(
                                max_value +
                                    compound.completion[state_index * num_children + offset],
                            );

                            if q > highest_q {
                                highest_q = q;
                                best_action = max_action;
                                best_value = max_value;
                            }
                        }
                    }

                    if highest_q != None {
                        Some((best_value, best_action))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }

    fn terminal_state(&self, node_type: CompoundNodeType, world: &World, state: &State) -> bool {
        match node_type {
            CompoundNodeType::Root => state.at_destination(),
            CompoundNodeType::Get => state.get_passenger() == None,
            CompoundNodeType::Put => state.at_destination(),
            CompoundNodeType::Navigate(id) => {
                Some(state.get_taxi()) == world.get_fixed_position(id)
            }
        }
    }

    fn maxq_q<R: Rng>(
        &mut self,
        node_index: usize,
        world: &World,
        mut state: &mut State,
        max_steps: usize,
        mut rng: &mut R,
    ) -> Vec<usize> {

        let mut seq = Vec::new();

        // Store a copy of the compound node type so that we do not need to hold
        // onto a reference of self.max_nodes[node_index].  This is necessary
        // so that we can recursively call this function and mutate self.
        let compound_node_type = match self.max_nodes[node_index] {
            MaxNode::Primitive(_) => None,
            MaxNode::Compound(ref compound) => Some(compound.compound_type),
        };

        if let Some(node_type) = compound_node_type {
            while !self.terminal_state(node_type, world, state) && seq.len() < max_steps {

                // println!(
                //     "step {} node {}\n{}",
                //     seq.len(),
                //     node_index,
                //     state.display(world)
                // );

                if let Some(state_index) = self.state_indexer.get_index(world, state) {

                    let selected_action: Option<(usize, usize)> =
                        if let MaxNode::Compound(ref compound) = self.max_nodes[node_index] {
                            let nongreedy_roll = rng.gen_range(0.0f64, 1.0f64);

                            if nongreedy_roll < self.epsilon {
                                let mut non_terminal_children =
                                    Vec::with_capacity(compound.children.len());
                                for (child_offset, child_index) in
                                    compound.children.iter().enumerate()
                                {
                                    match self.max_nodes[*child_index] {
                                        MaxNode::Primitive(_) => {
                                            non_terminal_children.push((*child_index, child_offset))
                                        }
                                        MaxNode::Compound(ref compound) => {
                                            if !self.terminal_state(
                                                compound.compound_type,
                                                world,
                                                state,
                                            )
                                            {
                                                non_terminal_children.push(
                                                    (*child_index, child_offset),
                                                );
                                            }
                                        }
                                    }
                                }

                                rng.choose(&non_terminal_children).map(|v| *v)
                            } else {
                                if let Some((_, child_index, child_offset)) =
                                    self.evaluate_max_node(node_index, world, state, state_index)
                                {
                                    Some((child_index, child_offset))
                                } else {
                                    None
                                }
                            }
                        } else {
                            None
                        };


                    if let Some((child_index, child_offset)) = selected_action {

                        // match self.max_nodes[child_index] {
                        //     MaxNode::Primitive(ref primitive) => {
                        //         println!(
                        //             "Selecting child {} - primitive action {}",
                        //             child_index,
                        //             primitive.action
                        //         )
                        //     }
                        //     MaxNode::Compound(ref compound) => {
                        //         println!(
                        //             "Selecting child {} - {:?}",
                        //             child_index,
                        //             compound.compound_type
                        //         )
                        //     }
                        // };

                        let mut child_seq = self.maxq_q::<R>(
                            child_index,
                            world,
                            &mut state,
                            max_steps - seq.len(),
                            &mut rng,
                        );

                        if let Some(result_state_index) =
                            self.state_indexer.get_index(world, state)
                        {

                            if let Some((best_value, _, best_offset)) =
                                self.evaluate_max_node(node_index, world, state, result_state_index)
                            {

                                if let MaxNode::Compound(ref mut compound) =
                                    self.max_nodes[node_index]
                                {

                                    let num_children = compound.children.len();

                                    let best_q = best_value +
                                        compound.completion[result_state_index * num_children +
                                                                best_offset];

                                    let mut accum_gamma = self.gamma;
                                    for si in child_seq.iter().rev() {

                                        compound.completion[si * num_children + child_offset] *=
                                            1.0 - self.alpha;
                                        compound.completion[si * num_children + child_offset] +=
                                            self.alpha * accum_gamma * best_q;
                                        accum_gamma *= self.gamma;
                                    }
                                } else {
                                    panic!("Failed to unwrap compound node {}.", node_index);
                                }
                            }
                        }

                        seq.append(&mut child_seq);
                    }
                }
            }

        // println!("Terminating node {}", node_index);

        } else {
            // Primitive node type
            if let Some(state_index) = self.state_indexer.get_index(world, state) {

                if let MaxNode::Primitive(ref mut primitive) = self.max_nodes[node_index] {

                    let reward = state.apply_action(world, primitive.action);

                    primitive.values[state_index] *= 1.0 - self.alpha;
                    primitive.values[state_index] += self.alpha * reward;
                    seq.push(state_index);
                } else {
                    panic!("Failed to unwrap primitive node {}.", node_index);
                }
            }
        }

        // assert!(
        //     seq.len() > 0,
        //     "Failed to append to sequence while evaluating node {}, max_nodes = {:?}",
        //     node_index,
        //     self.max_nodes
        // );

        seq
    }
}


impl Runner for MaxQ {
    fn learn<R: Rng>(
        &mut self,
        world: &World,
        mut state: State,
        max_steps: usize,
        mut rng: &mut R,
    ) -> Option<usize> {

        let seq = self.maxq_q(0, world, &mut state, max_steps, &mut rng);

        if let Some(final_state_index) = seq.last() {
            if let Some(final_state) = self.state_indexer.get_state(world, *final_state_index) {
                if final_state.at_destination() {
                    return Some(seq.len());
                }
            }
        }

        None
    }

    fn attempt<R: Rng>(
        &self,
        world: &World,
        mut state: State,
        max_steps: usize,
        mut _rng: &mut R,
    ) -> Attempt {

        let mut attempt = Attempt::new(state, max_steps);

        for _ in 0..max_steps {
            if state.at_destination() {
                break;
            }

            if let Some(state_index) = self.state_indexer.get_index(world, &state) {
                if let Some((_, next_action)) =
                    self.evaluate_max_node_action(0, world, &state, state_index)
                {
                    attempt.step(next_action);
                    state.apply_action(world, next_action);
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if state.at_destination() {
            attempt.succeeded()
        }

        attempt
    }

    fn solves<R: Rng>(
        &self,
        world: &World,
        mut state: State,
        max_steps: usize,
        mut _rng: &mut R,
    ) -> bool {
        for _ in 0..max_steps {
            if state.at_destination() {
                return true;
            }

            if let Some(state_index) = self.state_indexer.get_index(world, &state) {
                if let Some((_, next_action)) =
                    self.evaluate_max_node_action(0, world, &state, state_index)
                {
                    state.apply_action(world, next_action);
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        state.at_destination()
    }
}