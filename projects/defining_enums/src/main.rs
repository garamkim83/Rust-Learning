enum IdAddrKind {
    v4,
    v6,
}

fn route(ip_kind::IdAddrKind){
    kind:IdAddrKind,
    address:String,
}

fn main() {
    let four = IdAddrKind::v4;
    let six = IdAddrKind::v6;
}
