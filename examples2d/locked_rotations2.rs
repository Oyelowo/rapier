use rapier2d::prelude::*;
use rapier_testbed2d::Testbed;

// This shows a bug when a cylinder is in contact with a very large
// but very thin cuboid. In this case the EPA returns an incorrect
// contact normal, resulting in the cylinder falling through the floor.
pub fn init_world(testbed: &mut Testbed) {
    /*
     * World
     */
    let mut bodies = RigidBodySet::new();
    let mut colliders = ColliderSet::new();
    let impulse_joints = ImpulseJointSet::new();
    let multibody_joints = MultibodyJointSet::new();

    /*
     * The ground
     */
    let ground_size = 5.0;
    let ground_height = 0.1;

    let rigid_body = RigidBodyBuilder::new_static()
        .translation(vector![0.0, -ground_height])
        .build();
    let handle = bodies.insert(rigid_body);
    let collider = ColliderBuilder::cuboid(ground_size, ground_height).build();
    colliders.insert_with_parent(collider, handle, &mut bodies);

    /*
     * A rectangle that only rotate.
     */
    let rigid_body = RigidBodyBuilder::new_dynamic()
        .translation(vector![0.0, 3.0])
        .lock_translations()
        .build();
    let handle = bodies.insert(rigid_body);
    let collider = ColliderBuilder::cuboid(2.0, 0.6).build();
    colliders.insert_with_parent(collider, handle, &mut bodies);

    /*
     * A tilted capsule that cannot rotate.
     */
    let rigid_body = RigidBodyBuilder::new_dynamic()
        .translation(vector![0.0, 5.0])
        .rotation(1.0)
        .lock_rotations()
        .build();
    let handle = bodies.insert(rigid_body);
    let collider = ColliderBuilder::capsule_y(0.6, 0.4).build();
    colliders.insert_with_parent(collider, handle, &mut bodies);

    /*
     * Set up the testbed.
     */
    testbed.set_world(bodies, colliders, impulse_joints, multibody_joints);
    testbed.look_at(point![0.0, 0.0], 40.0);
}
