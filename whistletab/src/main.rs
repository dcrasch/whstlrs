use resvg::usvg;

fn main() {
    let fontdb = usvg::fontdb::Database::new();
    let filename = "contrib/starofthecountydown/starofthecountydown.svg";
    let opt = usvg::Options::default();
    let file_data = std::fs::read(filename).unwrap();
    let mut rtree = usvg::Tree::from_data(&file_data, &opt, &fontdb).unwrap();
    for node in rtree.root().children().iter() {
        match node {
            usvg::Node::Group(ref g) => {
                if !g.id().is_empty() {
                    println!("{:?}", g);
                }
            }
            _ => (),
        }
    }
}
