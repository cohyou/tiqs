macro_rules! cat {
    ($n:ident [ $( $object:ident )* ]) => {
        #[derive(Default)]
        struct $n {
            objects: [NamedCategoryObject; 3],
            arrows: [NamedCategoryArrow; 6],
        }

        impl Category for $n {
            type Object = NamedCategoryObject;
            type Arrow = NamedCategoryArrow;

            fn domain(&self, f: &NamedCategoryArrow) -> &NamedCategoryObject {
                match f {
                    _ if f == &self.arrows[0] => &self.objects[0],
                    _ if f == &self.arrows[1] => &self.objects[1],
                    _ if f == &self.arrows[2] => &self.objects[0],
                    _ => panic!(""),
                }
            }

            fn codomain(&self, f: &NamedCategoryArrow) -> &NamedCategoryObject {
                match f {
                    _ if f == &self.arrows[0] => &self.objects[0],
                    _ if f == &self.arrows[1] => &self.objects[1],
                    _ if f == &self.arrows[2] => &self.objects[1],
                    _ => panic!(""),
                }
            }

            fn identity(&self, o: &NamedCategoryObject) -> &NamedCategoryArrow {
                match o {
                    _ if o == &self.objects[0] => &self.arrows[0],
                    _ if o == &self.objects[1] => &self.arrows[1],
                    _ => panic!(""),
                }
            }

            fn composition_internal<'a>(&self, f: &'a NamedCategoryArrow, g: &'a NamedCategoryArrow) -> &'a NamedCategoryArrow {
                if f == g {
                    f
                } else if f == &self.arrows[0] && g == &self.arrows[2] {
                    g
                } else if f == &self.arrows[2] && g == &self.arrows[1] {
                    f
                } else {
                    panic!("")
                }
            }

            fn objects<'a>(&'a self) -> Box<Iterator<Item=&'a NamedCategoryObject> + 'a> {
                Box::new(self.objects.into_iter())
            }

            fn arrows<'a>(&'a self) -> Box<Iterator<Item=&'a NamedCategoryArrow> + 'a> {
                Box::new(self.arrows.into_iter())
            }
        }
    }
}

use std::cmp::PartialEq;
use std::iter::empty;
use std::iter::once;

trait Category {
    type Object: PartialEq;
    type Arrow: PartialEq;

    fn domain(&self, f: &Self::Arrow) -> &Self::Object;
    fn codomain(&self, f: &Self::Arrow) -> &Self::Object;
    fn identity(&self, a: &Self::Object) -> &Self::Arrow;
    fn composition<'a>(&'a self, f: &'a Self::Arrow, g: &'a Self::Arrow) -> Option<&'a Self::Arrow> {
        if self.codomain(&f) != self.domain(&g) {
            None
        } else {
            Some(self.composition_internal(&f, &g))
        }
    }
    fn composition_internal<'a>(&'a self, f: &'a Self::Arrow, g: &'a Self::Arrow) -> &'a Self::Arrow;
    fn ci<'a>(&'a self, f: &'a Self::Arrow, g: &'a Self::Arrow) -> &'a Self::Arrow {
        self.composition_internal(f, g)
    }

    fn objects<'a>(&'a self) -> Box<Iterator<Item=&'a Self::Object> + 'a>;
    fn arrows<'a>(&'a self) -> Box<Iterator<Item=&'a Self::Arrow> + 'a>;

    fn associativity(&self, f: &Self::Arrow, g: &Self::Arrow, k: &Self::Arrow) -> bool {
        self.ci(&self.ci(&f, &g), &k) == self.ci(&f, &self.ci(&g, &k))
    }
    fn unit_law_domain(&self, o: &Self::Object, f: &Self::Arrow) -> bool {
        self.ci(&self.identity(&o), &f) == f
    }
    fn unit_law_codmain(&self, o: &Self::Object, f: &Self::Arrow) -> bool {
        self.ci(&f, &self.identity(&o)) == f
    }
}

struct Zero;

impl Category for Zero {
    type Object = ();
    type Arrow = ();

    fn domain(&self, _f: &()) -> &() { &() }
    fn codomain(&self, _f: &()) -> &() { &() }
    fn identity(&self, _a: &()) -> &() { &() }
    fn composition_internal<'a>(&self, _f: &'a (), _g: &'a ()) -> &'a () { &() }
    fn objects(&self) -> Box<Iterator<Item=&()>> { Box::new(empty()) }
    fn arrows(&self) -> Box<Iterator<Item=&()>> { Box::new(empty()) }
}

#[derive(PartialEq, Default)]
struct CategoryObject;

#[derive(PartialEq, Default)]
struct CategoryArrow;

struct One {
    object: CategoryObject,
    arrow: CategoryArrow
}

impl One {
    fn new() -> One {
        One { object: CategoryObject {}, arrow: CategoryArrow {} }
    }
}

impl Category for One {
    type Object = CategoryObject;
    type Arrow = CategoryArrow;

    fn domain(&self, _f: &CategoryArrow) -> &CategoryObject { &self.object }
    fn codomain(&self, _f: &CategoryArrow) -> &CategoryObject { &self.object }
    fn identity(&self, _a: &CategoryObject) -> &CategoryArrow { &self.arrow }
    fn composition_internal<'a>(&'a self, _f: &'a CategoryArrow, _g: &'a CategoryArrow) -> &'a CategoryArrow { &self.arrow }
    fn objects<'a>(&'a self) -> Box<Iterator<Item=&'a CategoryObject> + 'a> { Box::new(once(&self.object)) }
    fn arrows<'a>(&'a self) -> Box<Iterator<Item=&'a CategoryArrow> + 'a> { Box::new(once(&self.arrow)) }
}

#[derive(Default)]
struct Two {
    objects: [CategoryObject; 2],
    arrows: [CategoryArrow; 3],
}

impl Category for Two {
    type Object = CategoryObject;
    type Arrow = CategoryArrow;

    fn domain(&self, f: &CategoryArrow) -> &CategoryObject {
        match f {
            _ if f == &self.arrows[0] => &self.objects[0],
            _ if f == &self.arrows[1] => &self.objects[1],
            _ if f == &self.arrows[2] => &self.objects[0],
            _ => panic!(""),
        }
    }

    fn codomain(&self, f: &CategoryArrow) -> &CategoryObject {
        match f {
            _ if f == &self.arrows[0] => &self.objects[0],
            _ if f == &self.arrows[1] => &self.objects[1],
            _ if f == &self.arrows[2] => &self.objects[1],
            _ => panic!(""),
        }
    }

    fn identity(&self, o: &CategoryObject) -> &CategoryArrow {
        match o {
            _ if o == &self.objects[0] => &self.arrows[0],
            _ if o == &self.objects[1] => &self.arrows[1],
            _ => panic!(""),
        }
    }

    fn composition_internal<'a>(&self, f: &'a CategoryArrow, g: &'a CategoryArrow) -> &'a CategoryArrow {
        if f == g {
            f
        } else if f == &self.arrows[0] && g == &self.arrows[2] {
            g
        } else if f == &self.arrows[2] && g == &self.arrows[1] {
            f
        } else {
            panic!("")
        }
    }

    fn objects<'a>(&'a self) -> Box<Iterator<Item=&'a CategoryObject> + 'a> {
        Box::new(self.objects.into_iter())
    }

    fn arrows<'a>(&'a self) -> Box<Iterator<Item=&'a CategoryArrow> + 'a> {
        Box::new(self.arrows.into_iter())
    }
}

#[derive(PartialEq, Default)]
struct NamedCategoryObject(&'static str);

#[derive(PartialEq, Default)]
struct NamedCategoryArrow(&'static str);

let v = vec![];

cat!(Three [X Y Z]);

fn main() {
    let _zero = Zero {};
    let _one = One::new();
    let _two = Two::default();
    let _three = Three::default();

    println!("{:?}", "hello category!");

    // fn triangle(n: i32) -> i32 {
    //     let mut sum = 0;
    //     for i in 1..n+1 {
    //         sum += i;
    //     }
    //     sum
    // }
    // println!("{:?}", triangle(10));
    //
    // fn triangle2(n: i32) -> i32 {
    //     (1..n+1).fold(0, |sum, item| sum + item)
    // }
    // println!("{:?}", triangle2(10));
    //
    // println!("There's:");
    // let v = vec!["antimony", "arsenic", "aluminum", "selenium"];
    //
    // for element in &v {
    //     println!("{}", element);
    // }
    //
    // println!("There's:");
    // let v2 = vec!["antimony", "arsenic", "aluminum", "selenium"];
    // let mut iterator = (&v2).into_iter();
    // while let Some(element) = iterator.next() {
    //     println!("{}", element);
    // }

    let v = vec![4, 20, 12, 8, 6];
    let mut iterator = v.iter();
    assert_eq!(iterator.next(), Some(&4));
    assert_eq!(iterator.next(), Some(&20));
    assert_eq!(iterator.next(), Some(&12));
    assert_eq!(iterator.next(), Some(&8));
    assert_eq!(iterator.next(), Some(&6));
    assert_eq!(iterator.next(), None);


    use std::ffi::OsStr;
    use std::path::Path;

    let path = Path::new("C:/Users/JimB/Downloads/Fedora.iso");
    let mut iterator = path.iter();
    assert_eq!(iterator.next(), Some(OsStr::new("C:")));
    assert_eq!(iterator.next(), Some(OsStr::new("Users")));
    assert_eq!(iterator.next(), Some(OsStr::new("JimB")));


    // You should usually use HashSet,
    // but its iteration order is nondeterministic,
    // so BTreeSet works better in examples.

    use std::collections::BTreeSet;
    let mut favorites = BTreeSet::new();
    favorites.insert("Lucy in the Sky With Diamonds".to_string());
    favorites.insert("Liebesträume No. 3".to_string());

    let mut it = favorites.into_iter();
    assert_eq!(it.next(), Some("Liebesträume No. 3".to_string()));
    assert_eq!(it.next(), Some("Lucy in the Sky With Diamonds".to_string()));
    assert_eq!(it.next(), None);
}
