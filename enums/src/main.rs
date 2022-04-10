enum IpAddressKind {
    V4,
    V6
}

fn route(ip_kind: IpAddressKind) {

}
struct IpAddress {
    kind: IpAddressKind,
    address: String,
}
enum IpAddressEnum {
    V4(String),
    V6(String);
}
fn main() {
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;
    route(four);
    route(six);
    route(IpAddressKind::V4);
    route(IpAddressKind::V6);
    {
        let home = IpAddress {
            kind: IpAddressKind::V4,
            address: String::from("127.0.0.1")
        };
        let loopback = IpAddress {
            kind: IpAddressKind::V6,
            address: String::from("::1")
        };
    }
    {
        let home = IpAddressEnum::V4(String::from("127.0.0.1"));
        let loopback = IpAddressEnum::V6(String::from("::01"));
    }
    {
        enum IpAddressEnum{
            V4(u8,u8,u8,u8),
            V6(String)
        }
        let home = IpAddressEnum::V4(127,0,0,1);
        let loopback = IpAddressEnum::V6(String::from("::1"));
    }
    {
        enum Message {
            Quit,
            Move {x: i32, y: i32},
            Write(String),
            ChangeColor(i32,i32,i32,i32),
        }
        impl Message {
            fn call(&self) {
            }
        }
        let message_move = Message::Move {x : 12,y : 14};
        let some_number = Some(5);
        let some_strnig = Some(String::from("Alloha"));
        let absent_numbner :Option<i32> = None;
    }
}
