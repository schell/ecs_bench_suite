use std::task::Waker;

use apecs::{
    anyhow,
    entities::*,
    storage::{CanWriteStorage, VecStorage},
    world::World,
    join::Join,
    CanFetch, Facade, Read, Write,
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

#[derive(CanFetch)]
struct Data {
    es: Write<Entities>,
    ts: Write<VecStorage<Transform>>,
    ps: Write<VecStorage<Position>>,
    rs: Write<VecStorage<Rotation>>,
    vs: Write<VecStorage<Velocity>>,
}

#[derive(CanFetch)]
struct SimpleIterData {
    vs: Read<VecStorage<Velocity>>,
    ps: Write<VecStorage<Position>>,
}

async fn async_run(mut facade: Facade) -> anyhow::Result<()> {
    let SimpleIterData { vs, mut ps } = facade.fetch().await.unwrap();

    for (_, velocity, position) in (&vs, &mut ps).join() {
        position.0 += velocity.0;
    }

    Ok(())
}

fn sync_run(SimpleIterData { vs, mut ps }: SimpleIterData) -> anyhow::Result<()> {
    for (_, velocity, position) in (&vs, &mut ps).join() {
        position.0 += velocity.0;
    }

    Ok(())
}

pub struct Benchmark(World, std::task::Context<'static>);

pub enum Syncronicity {
    Async,
    Sync
}

static mut WAKER: Option<Waker> = None;

impl Benchmark {
    pub fn new(sync: Syncronicity) -> Self {
        unsafe {
            WAKER = Some(World::new_waker());
        };
        let mut world = World::default();
        world
            .with_default_resource::<Entities>().unwrap()
            .with_resource::<VecStorage<Transform>>(VecStorage::new_with_capacity(10000)).unwrap()
            .with_resource::<VecStorage<Position>>(VecStorage::new_with_capacity(10000)).unwrap()
            .with_resource::<VecStorage<Rotation>>(VecStorage::new_with_capacity(10000)).unwrap()
            .with_resource::<VecStorage<Velocity>>(VecStorage::new_with_capacity(10000)).unwrap();
        match &sync {
            Syncronicity::Async => world.with_async_system("simple_iter_async", async_run),
            Syncronicity::Sync => world.with_system("simple_iter", sync_run),
        };

        {
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

        Self(world, unsafe{std::task::Context::from_waker(WAKER.as_ref().unwrap())})
    }

    pub fn run(&mut self) {
        self.0.tick_with_context(Some(&mut self.1)).unwrap()
    }
}
