fn defining() {
    enum IpAddr {
        V4([u8; 4]),
        V6(String)
    }
    impl std::fmt::Display for IpAddr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                IpAddr::V4(addr_v4) => write!(f, "{}", addr_v4.map(|x| x.to_string()).join(".")),
                IpAddr::V6(addr_v6) => write!(f, "{}", addr_v6),
            }
        }
    }
    let ip1 = IpAddr::V4([10, 180, 13, 96]);
    let ip2 = IpAddr::V6(String::from("2001:da8:8000:1:202:112:26:40"));
    println!("{}, {}", ip1, ip2);
    let x: Option<&str> = if true {None} else {Some("HellOwhatAs")};
    println!("{}", x.unwrap_or("None!!!"));
}

fn matchs() {

}

#[allow(dead_code)]
pub fn main() {
    defining();
    matchs();
}