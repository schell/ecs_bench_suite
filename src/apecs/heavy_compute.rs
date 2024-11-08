use apecs::{anyhow, entities::*, join::*, storage::*, world::*, CanFetch, Write};
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
struct HeavyComputeData {
    positions: Write<VecStorage<Position>>,
    transforms: Write<VecStorage<Transform>>,
}

fn system(mut data: HeavyComputeData) -> anyhow::Result<()> {
    use cgmath::Transform;
    (&mut data.positions, &mut data.transforms).par_join()
        .for_each(|(pos, mat)| {
            for _ in 0..100 {
                mat.0 = mat.0.invert().unwrap();
            }
            pos.0 = mat.0.transform_vector(pos.0);
        });
    Ok(())
}

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let mut entities = Entities::default();
        let mut transforms: VecStorage<Transform> = VecStorage::new_with_capacity(1000);
        let mut positions: VecStorage<Position> = VecStorage::new_with_capacity(1000);
        let mut rotations: VecStorage<Rotation> = VecStorage::new_with_capacity(1000);
        let mut velocities: VecStorage<Velocity> = VecStorage::new_with_capacity(1000);
        (0..1000).for_each(|_| {
            let e = entities.create();
            transforms.insert(e.id(), Transform(Matrix4::<f32>::from_angle_x(Rad(1.2))));
            positions.insert(e.id(), Position(Vector3::unit_x()));
            rotations.insert(e.id(), Rotation(Vector3::unit_x()));
            velocities.insert(e.id(), Velocity(Vector3::unit_x()));
        });
        let mut world = World::default();
        world
            .with_resource(entities)
            .unwrap()
            .with_resource(transforms)
            .unwrap()
            .with_resource(positions)
            .unwrap()
            .with_resource(rotations)
            .unwrap()
            .with_resource(velocities)
            .unwrap()
            .with_system("heavy_compute", system);

        Self(world)
    }

    pub fn run(&mut self) {
        self.0.tick_with_context(None).unwrap();
    }
}
