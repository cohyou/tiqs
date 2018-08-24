struct Category {
    objects: Vec<String>,
    arrows: Vec<String>
}

fn main() {
    let category = Category {
        objects: vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string()
        ],
        arrows: vec![
            "f: A -> B".to_string(),
            "g: A -> C".to_string(),
            "h: B -> C".to_string()
        ]
    };
}
