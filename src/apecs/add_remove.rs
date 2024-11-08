use apecs::{
    entities::*,
    storage::*,
    Write, world::World,
};

pub type Storage<T> = BTreeStorage<T>;

// TODO: Remove clone constraint
#[derive(Clone)]
struct A(f32);

#[derive(Clone)]
struct B(f32);

pub struct Benchmark(World, Vec<Entity>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();
        world
            .with_default_resource::<Entities>()
            .unwrap()
            .with_resource::<Storage<A>>(Storage::default())
            .unwrap()
            .with_resource::<Storage<B>>(Storage::default())
            .unwrap();

        let entities = {
            let (mut entities, mut a_storage): (Write<Entities>, Write<Storage<A>>) =
                world.fetch().unwrap();
            (0..10000)
                .map(|_| {
                    let e = entities.create();
                    let _ = a_storage.insert(e.id(), A(0.0));
                    e
                })
                .collect()
        };

        Self(world, entities)
    }

    pub fn run(&mut self) {
        let mut b_storage = self.0.fetch::<Write<Storage<B>>>().unwrap();
        for entity in &self.1 {
            let _ = b_storage.insert(entity.id(), B(0.0));
        }

        for entity in &self.1 {
            b_storage.remove(entity.id());
        }
    }
}
