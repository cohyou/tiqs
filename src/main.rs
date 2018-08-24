use std::collections::HashMap;

type Arrows = HashMap<String, (String, String)>;

struct Category {
    objects: Vec<String>,
    arrows: Arrows
}

fn insert_arrow(arrows: &mut Arrows, name: &str, domain: &str, codomain: &str) {
    arrows.insert(name.to_string(), (domain.to_string(), codomain.to_string()));
}

fn main() {
    let mut arrows = HashMap::new();
    insert_arrow(&mut arrows, "f", "A", "B");
    insert_arrow(&mut arrows, "g", "A", "C");
    insert_arrow(&mut arrows, "h", "B", "C");

    let category = Category {
        objects: vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string()
        ],
        arrows: arrows
    };
}
