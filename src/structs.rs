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
    mod cc{
        pub struct Cuboid<T: Copy> {
            pub x: T,
            pub y: T,
            pub z: T
        }
        impl<T: Copy + std::ops::Mul<T, Output = T>> Cuboid<T> {
            pub fn new(x: T, y: T, z: T) -> Cuboid<T> {
                Cuboid { x, y, z }
            }
            pub fn volume(&self) -> T {
                self.x * self.y * self.z
            }
        }
        impl<T: std::fmt::Display + Copy> std::fmt::Display for Cuboid<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Cuboid<{}>{{x: {}, y: {}, z: {}}}", std::any::type_name::<T>(), self.x, self.y, self.z)
            }
        }
    }
    use cc::Cuboid;
    let x = Cuboid::new(10.3, 20.5, 30.);
    let y = Cuboid::new(1, 2, 3);
    println!("{}, {}, {}, {}, {}", x, x.volume(), x.x, x.y, x.z);
    println!("{}, {}", y, y.volume());
}

#[allow(dead_code)]
pub fn main() {
    defining_structs();
    example_structs();
}