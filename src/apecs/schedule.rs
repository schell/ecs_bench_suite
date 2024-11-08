use apecs::{anyhow, entities::*, join::*, storage::*, world::*, CanFetch, Write};

struct A(f32);
struct B(f32);
struct C(f32);
struct D(f32);
struct E(f32);

#[derive(CanFetch)]
struct ABSystemData {
    a_store: Write<VecStorage<A>>,
    b_store: Write<VecStorage<B>>,
}

fn ab_system(mut data: ABSystemData) -> anyhow::Result<()> {
    for (_, a, b) in (&mut data.a_store, &mut data.b_store).join() {
        std::mem::swap(&mut a.0, &mut b.0);
    }

    Ok(())
}

#[derive(CanFetch)]
struct CDSystemData {
    c_store: Write<VecStorage<C>>,
    d_store: Write<VecStorage<D>>,
}

fn cd_system(mut data: CDSystemData) -> anyhow::Result<()> {
    for (_, c, d) in (&mut data.c_store, &mut data.d_store).join() {
        std::mem::swap(&mut c.0, &mut d.0);
    }

    Ok(())
}

#[derive(CanFetch)]
struct CESystemData {
    c_store: Write<VecStorage<C>>,
    e_store: Write<VecStorage<E>>,
}

fn ce_system(mut data: CESystemData) -> anyhow::Result<()> {
    for (_, c, e) in (&mut data.c_store, &mut data.e_store).join() {
        std::mem::swap(&mut c.0, &mut e.0);
    }

    Ok(())
}

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let mut entities = Entities::default();
        let mut a_store:VecStorage<A> = VecStorage::new_with_capacity(50000);
        let mut b_store:VecStorage<B> = VecStorage::new_with_capacity(40000);
        let mut c_store:VecStorage<C> = VecStorage::new_with_capacity(30000);
        let mut d_store:VecStorage<D> = VecStorage::new_with_capacity(10000);
        let mut e_store:VecStorage<E> = VecStorage::new_with_capacity(10000);

        (0..10000).for_each(|_| {
            let e = entities.create();
            a_store.insert(e.id(), A(0.0));
        });
        (0..10000).for_each(|_| {
            let e = entities.create();
            a_store.insert(e.id(), A(0.0));
            b_store.insert(e.id(), B(0.0));
        });

        (0..10000).for_each(|_| {
            let e = entities.create();
            a_store.insert(e.id(), A(0.0));
            b_store.insert(e.id(), B(0.0));
            c_store.insert(e.id(), C(0.0));
        });
        (0..10000).for_each(|_| {
            let e = entities.create();
            a_store.insert(e.id(), A(0.0));
            b_store.insert(e.id(), B(0.0));
            c_store.insert(e.id(), C(0.0));
            d_store.insert(e.id(), D(0.0));
        });
        (0..10000).for_each(|_| {
            let e = entities.create();
            a_store.insert(e.id(), A(0.0));
            b_store.insert(e.id(), B(0.0));
            c_store.insert(e.id(), C(0.0));
            e_store.insert(e.id(), E(0.0));
        });

        let mut world = World::default();
        world
            .with_resource(entities)
            .unwrap()
            .with_resource(a_store)
            .unwrap()
            .with_resource(b_store)
            .unwrap()
            .with_resource(c_store)
            .unwrap()
            .with_resource(d_store)
            .unwrap()
            .with_resource(e_store)
            .unwrap()
            .with_system("ab", ab_system)
            .with_system("cd", cd_system)
            .with_system("ce", ce_system);
            //.with_sync_systems_run_in_parallel(true);

        Self(world)
    }

    pub fn run(&mut self) {
        self.0.tick_with_context(None).unwrap()
    }
}
