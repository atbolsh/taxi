extern crate rand;
extern crate taxi;

#[macro_use]
extern crate criterion;

use rand::Isaac64Rng;

use criterion::Criterion;

use taxi::world::World;
use taxi::state::State;
use taxi::runner::{run_training_session, Probe};
use taxi::qlearner::QLearner;
use taxi::rmax::RMax;
use taxi::factoredrmax::FactoredRMax;

criterion_group!(trainers, qlearner, rmax, factored_rmax);
criterion_main!(trainers);

struct SessionData {
    world: World,
    probes: Vec<Probe>,
}

impl Default for SessionData {
    fn default() -> SessionData {
        // let world_str = "\
        //                  ┌───┬─────┐\n\
        //                  │R .│. . G│\n\
        //                  │   │     │\n\
        //                  │. .│. . .│\n\
        //                  │         │\n\
        //                  │. . . . .│\n\
        //                  │         │\n\
        //                  │.│. .│. .│\n\
        //                  │ │   │   │\n\
        //                  │Y│. .│B .│\n\
        //                  └─┴───┴───┘\n\
        //                  ";

        // let world = World::build_from_str(world_str).unwrap();

        // let probes = vec![
        //     Probe::new(State::build(&world, (2, 2), Some('Y'), 'R').unwrap(), 10),
        //     Probe::new(State::build(&world, (2, 2), Some('Y'), 'G').unwrap(), 14),
        //     Probe::new(State::build(&world, (2, 2), Some('Y'), 'B').unwrap(), 13),
        //     Probe::new(State::build(&world, (2, 2), Some('R'), 'B').unwrap(), 13),
        //     Probe::new(State::build(&world, (2, 2), Some('Y'), 'R').unwrap(), 6),
        //     Probe::new(State::build(&world, (2, 2), Some('B'), 'G').unwrap(), 13),
        // ];

        let world_str = "\
                         ┌─┬───┐\n\
                         │R│. G│\n\
                         │ │   │\n\
                         │. . .│\n\
                         │     │\n\
                         │Y B .│\n\
                         └─────┘\n\
                         ";

        let world = World::build_from_str(world_str).unwrap();

        let probes = vec![
            Probe::new(State::build(&world, (1, 1), Some('Y'), 'R').unwrap(), 4),
            Probe::new(State::build(&world, (1, 1), Some('Y'), 'G').unwrap(), 6),
            Probe::new(State::build(&world, (1, 1), Some('Y'), 'B').unwrap(), 3),
            Probe::new(State::build(&world, (1, 1), Some('R'), 'B').unwrap(), 5),
            Probe::new(State::build(&world, (1, 1), Some('G'), 'R').unwrap(), 6),
            Probe::new(State::build(&world, (1, 1), Some('B'), 'G').unwrap(), 4),
        ];

        SessionData { world, probes }
    }
}

fn qlearner(c: &mut Criterion) {
    let data = SessionData::default();
    let source_rng = Isaac64Rng::new_unseeded();

    c.bench_function("qmax", |b| {
        b.iter(|| {
            let mut qlearner = QLearner::new(&data.world, 0.1, 0.3, 0.6, false);
            let mut rng = source_rng;

            run_training_session(&data.world, &data.probes, 1, 100, &mut qlearner, &mut rng)
        })
    });
}

fn rmax(c: &mut Criterion) {
    let data = SessionData::default();
    let source_rng = Isaac64Rng::new_unseeded();

    c.bench_function("rmax", |b| {
        b.iter(|| {
            let mut rmax = RMax::new(&data.world, 0.3, 1.0, 1.0e-6);
            let mut rng = source_rng;

            run_training_session(&data.world, &data.probes, 1, 10, &mut rmax, &mut rng)
        })
    });
}

fn factored_rmax(c: &mut Criterion) {
    let data = SessionData::default();
    let source_rng = Isaac64Rng::new_unseeded();

    c.bench_function("factored_rmax", |b| {
        b.iter(|| {
            let mut factored_rmax = FactoredRMax::new(&data.world, 0.3, 1.0, 1.0e-6);
            let mut rng = source_rng;

            run_training_session(
                &data.world,
                &data.probes,
                1,
                10,
                &mut factored_rmax,
                &mut rng,
            )
        })
    });
}
