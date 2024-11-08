use apecs::{
    anyhow,
    entities::*,
    world::World,
    storage::{CanWriteStorage, VecStorage},
    CanFetch, Write,
};
use cgmath::*;

#[derive(Copy, Clone)]
struct Transform(Matrix4<f32>);

#[derive(Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

pub struct Benchmark;

#[derive(CanFetch)]
struct Data {
    es: Write<Entities>,
    ts: Write<VecStorage<Transform>>,
    ps: Write<VecStorage<Position>>,
    rs: Write<VecStorage<Rotation>>,
    vs: Write<VecStorage<Velocity>>,
}

impl Benchmark {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&mut self) {
        let mut world = World::default();
        world
            .with_default_resource::<Entities>()
            .unwrap()
            .with_default_resource::<VecStorage<Transform>>()
            .unwrap()
            .with_default_resource::<VecStorage<Position>>()
            .unwrap()
            .with_default_resource::<VecStorage<Rotation>>()
            .unwrap()
            .with_default_resource::<VecStorage<Velocity>>()
            .unwrap();

        let Data {
            mut es,
            mut ts,
            mut ps,
            mut rs,
            mut vs,
        } = world.fetch().unwrap();

        (0..10000).for_each(|_| {
            let e = es.create();
            let id = e.id();
            ts.insert(id, Transform(Matrix4::<f32>::from_scale(1.0)));
            ps.insert(id, Position(Vector3::unit_x()));
            rs.insert(id, Rotation(Vector3::unit_x()));
            vs.insert(id, Velocity(Vector3::unit_x()));
        });
    }
}
