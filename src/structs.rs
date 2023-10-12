fn defining_structs() {
    {
        struct Point3d<T> {x: T, y: T, z: T}
        impl<T: std::fmt::Display> std::fmt::Display for Point3d<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Point3d<{}>{{x: {}, y: {}, z: {}}}", std::any::type_name::<T>(), self.x, self.y, self.z)
            }
        }
        let p1 = Point3d {x: 0, y: 0, z: 0};
        let p2 = Point3d {y: 200, ..p1};
        println!("{}, {}", p1, p2);
    }
    {
        struct Point3d<T>(T, T, T);
        impl<T: std::fmt::Display> std::fmt::Display for Point3d<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Point3d<{}>({}, {}, {})", std::any::type_name::<T>(), self.0, self.1, self.2)
            }
        }
        let p1 = Point3d(0, 0, 0);
        let p2 = Point3d(0, 100, 0);
        println!("{}, {}", p1, p2);
    }
    {
        struct Point1d;
        let p = Point1d;
        impl std::fmt::Display for Point1d {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", std::any::type_name::<Point1d>())
            }
        }
        println!("{}", p);
    }
}

fn example_structs() {
    // struct cuboid<T> {
    //     x: T, y: T, z: T
    // }
    // impl<T> cuboid<T> where for<'a> T: std::ops::Mul<&'a T, Output = T>
    // {
    //     fn new(x: T, y: T, z: T) -> cuboid<T> {
    //         cuboid {x, y, z}
    //     }
    //     fn volume(&self) {
    //         let c = (&self.x * &self.y);
    //     }
    // }
}

#[allow(dead_code)]
pub fn main() {
    defining_structs();
    example_structs();
}