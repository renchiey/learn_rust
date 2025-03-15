enum IpAddr {
    V4(String),
    V6(String),
}

impl IpAddr {
    fn get_addr(&self) -> &str {
        match self {
            IpAddr::V4(s) => &s[..],
            _ => " ",
        }
    }
}

pub fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home address: {}", home.get_addr());

}

fn describe_v6_ipaddr(add: IpAddr) -> Option<bool> {
    // pattern matching syntax
    if let IpAddr::V6(a) = add {
        a
    } else {
        return None;
    };

    // equivalent 
    // let IpAddr::V6(a) = add else {
    //     return None;
    // }
    // 
    // also equivalent
    // match add {
    //     IpAddr::V6(a) => Some(true),
    //     _ => None
    // }

    Some(true)
}
