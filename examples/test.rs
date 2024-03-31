use configer::Configer;

fn main() {
    let con = Configer::new("/home/kurumin/json");
    println!("{:#?}", con.read("pixiv-disks").unwrap()[con.hostname().unwrap()]);
}