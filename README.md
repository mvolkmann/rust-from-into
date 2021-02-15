# rust-from-into

This demonstrates using the Rust traits
`std::convert::From` and `std::convert::Into`.

It uses the types `Point2D` (a point with x and y coordinates)
and `Point3D` (a point with x, y, and z coordinates).

It implements the following type conversions.

- Point2D -> Point3D sets z to zero.
- Point2D -> f64 calculates the distance from the origin.
- Point3D -> Point2D discards the z coordinate.
- Point3D -> f64 calculates the distance from the origin.
