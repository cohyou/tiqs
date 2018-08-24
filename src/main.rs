use std::fmt::Display;
use std::collections::HashMap;

type Arrows = HashMap<String, (String, String)>;

struct Category {
    objects: Vec<String>,
    arrows: Arrows
}

impl Category {
    fn has_arrow(&self, domain: &str, codomain: &str) -> bool {
        for (dom, cod) in self.arrows.values() {
            if dom == domain && cod == codomain {
                return true
            }
        }
        false
    }

    fn is_isomorphism(&self, object1: &str, object2: &str) -> bool {
        self.has_arrow(object1, object2) && self.has_arrow(object2, object1)
    }
}

impl Display for Category {
    fn fmt(&self, dest: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut arrow_texts = vec![];
        for (k, v) in &self.arrows {
            arrow_texts.push(format!("{}: {} -> {}", k, v.0, v.1));
        }
        write!(dest, "objects: {:?} \n arrows: {:?}", self.objects, arrow_texts)
    }
}

fn insert_arrow(arrows: &mut Arrows, name: &str, domain: &str, codomain: &str) {
    arrows.insert(name.to_string(), (domain.to_string(), codomain.to_string()));
}

fn check_isomorphism(category: Category, object1: &str, object2: &str) {
    println!("{} and {} are isomorphism: {:?}", object1, object2, category.is_isomorphism(object1, object2));
}

fn new_category(objects: Vec<&str>, arrow_names: Vec<(&str, &str, &str)>) -> Category {
    let mut arrows = HashMap::new();
    for arrow in &arrow_names {
        insert_arrow(&mut arrows, arrow.0, arrow.1, arrow.2);
    }
    let category = Category { objects: objects.iter().map(|s| s.to_string()).collect(), arrows: arrows };
    println!("made {}", category);
    category
}

fn main() {
    let arrows1 = vec![
        ("f", "A", "B"),
        ("g", "A", "C"),
        ("h", "B", "C")
    ];
    let category1 = new_category(vec!["A", "B", "C"], arrows1);
    check_isomorphism(category1, "A", "B");

    let arrows2 = vec![
        ("f", "A", "B"),
        ("g", "B", "A")
    ];
    let category2 = new_category(vec!["A", "B"], arrows2);
    check_isomorphism(category2, "A", "B");
}
