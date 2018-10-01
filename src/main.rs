macro_rules! cat {
    ([ $( $object:ident )* ]
     [ $( $arrow_name:ident : $domain:ident -> $codomain:ident )* ]
     [ $( $f1:ident;$f2:ident = $f3:ident )* ]) => ({
        let mut objects = vec![];
        let mut arrows = vec![];

        $(
            objects.push(NamedCategoryObject(stringify!($object)));
            let idx = objects.iter().count() - 1;
            arrows.push(NamedCategoryArrow {
                name: stringify!($object), domain: idx, codomain: idx, equals: vec![]
            });
        )*

        let mut equals_list: Vec<(&'static str, Vec<&'static str>)> = vec![];
        $( equals_list.push( (stringify!($f3),
                              vec![stringify!($f1), stringify!($f2)]) ); )*
        $(
            let dom_idx = objects.iter()
                                 .position(|x| x.0 == stringify!($domain))
                                 .unwrap();
            let cod_idx = objects.iter()
                                 .position(|x| x.0 == stringify!($codomain))
                                 .unwrap();
            let mut equals = vec![];

            for eqs in equals_list.iter() {
                if eqs.0 == stringify!($arrow_name) {
                    let before_idx = arrows.iter()
                                           .position(|x| x.name == eqs.1[0])
                                           .unwrap();
                    let after_idx = arrows.iter()
                                          .position(|x| x.name == eqs.1[1])
                                          .unwrap();
                    equals.push(vec![before_idx, after_idx]);
                }
            }
            arrows.push(NamedCategoryArrow {
                name: stringify!($arrow_name),
                domain: dom_idx,
                codomain: cod_idx,
                equals: equals,
            });
        )*
        SmallCat::<NamedCategoryObject, NamedCategoryArrow> { objects, arrows }
    })
}

macro_rules! cat3 {
    ($n:ident [ $( $object:ident )* ]) => {
        #[derive(Default)]
        struct $n<Object, Arrow> {
            objects: [Object; 3],
            arrows: [Arrow; 6],
        }

        impl<O: PartialEq + Eq + Hash + Default + Object + Debug + Clone, A: PartialEq + Debug + Arrow + Default + Clone> Category for $n<O, A> {
            type Object = O;
            type Arrow = A;

            fn domain(&self, f: &Self::Arrow) -> &Self::Object {
                match f {
                    _ if f == &self.arrows[0] => &self.objects[0],
                    _ if f == &self.arrows[1] => &self.objects[1],
                    _ if f == &self.arrows[2] => &self.objects[2],
                    _ if f == &self.arrows[3] => &self.objects[1],
                    _ if f == &self.arrows[4] => &self.objects[2],
                    _ if f == &self.arrows[5] => &self.objects[2],
                    _ => panic!(""),
                }
            }

            fn codomain(&self, f: &Self::Arrow) -> &Self::Object {
                match f {
                    _ if f == &self.arrows[0] => &self.objects[0],
                    _ if f == &self.arrows[1] => &self.objects[1],
                    _ if f == &self.arrows[2] => &self.objects[2],
                    _ if f == &self.arrows[3] => &self.objects[0],
                    _ if f == &self.arrows[4] => &self.objects[1],
                    _ if f == &self.arrows[5] => &self.objects[0],
                    _ => panic!(""),
                }
            }

            fn identity(&self, o: &Self::Object) -> &Self::Arrow {
                match o {
                    _ if o == &self.objects[0] => &self.arrows[0],
                    _ if o == &self.objects[1] => &self.arrows[1],
                    _ if o == &self.objects[2] => &self.arrows[2],
                    _ => panic!(""),
                }
            }

            fn composition_internal<'a>(&'a self, f: &'a Self::Arrow, g: &'a Self::Arrow) -> &'a Self::Arrow {
                if f == g {
                    f
                } else if f == &self.arrows[0] && g == &self.arrows[3] {
                    g
                } else if f == &self.arrows[3] && g == &self.arrows[1] {
                    f
                } else if f == &self.arrows[1] && g == &self.arrows[4] {
                    g
                } else if f == &self.arrows[4] && g == &self.arrows[2] {
                    f
                } else if f == &self.arrows[0] && g == &self.arrows[5] {
                    g
                } else if f == &self.arrows[5] && g == &self.arrows[2] {
                    f
                } else if f == &self.arrows[3] && g == &self.arrows[4] {
                    &self.arrows[5]
                } else {
                    panic!("")
                }
            }

            fn objects<'a>(&'a self) -> Box<Iterator<Item=&'a Self::Object> + 'a> {
                Box::new(self.objects.into_iter())
            }

            fn arrows<'a>(&'a self) -> Box<Iterator<Item=&'a Self::Arrow> + 'a> {
                Box::new(self.arrows.into_iter())
            }
        }
    }
}

use std::cmp::PartialEq;
use std::iter::empty;
use std::iter::once;
use std::fmt::Debug;
use std::hash::Hash;

trait Arrow {
    fn name(&self) -> &'static str { "" }
}

trait Object {
    fn name(&self) -> &'static str { "" }
}

impl Arrow for () {
    fn name(&self) -> &'static str { "" }
}

impl Object for () {
    fn name(&self) -> &'static str { "" }
}

trait Category: Default {
    type Object: PartialEq + Eq + Hash + Clone + Debug + Object;
    type Arrow: PartialEq + Debug + Clone + Arrow;

    fn new(objects: Vec<Self::Object>, arrows: Vec<Self::Arrow>) -> Box<Self> { Default::default() }
    fn domain(&self, f: &Self::Arrow) -> &Self::Object;
    fn codomain(&self, f: &Self::Arrow) -> &Self::Object;
    fn identity(&self, a: &Self::Object) -> &Self::Arrow;
    fn composition<'a>(&'a mut self, f: &'a Self::Arrow, g: &'a Self::Arrow) -> Option<&'a Self::Arrow> {
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

#[derive(Default)]
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

#[derive(Default)]
struct One<Object, Arrow> {
    object: Object,
    arrow: Arrow
}

impl<O: PartialEq + Eq + Hash + Clone + Default + Debug + Object, A: PartialEq + Debug + Clone + Arrow + Default> Category for One<O, A> {
    type Object = O;
    type Arrow = A;

    fn domain(&self, _f: &Self::Arrow) -> &Self::Object { &self.object }
    fn codomain(&self, _f: &Self::Arrow) -> &Self::Object { &self.object }
    fn identity(&self, _a: &Self::Object) -> &Self::Arrow { &self.arrow }
    fn composition_internal<'a>(&'a self, _f: &'a Self::Arrow, _g: &'a Self::Arrow) -> &'a Self::Arrow { &self.arrow }
    fn objects<'a>(&'a self) -> Box<Iterator<Item=&'a Self::Object> + 'a> { Box::new(once(&self.object)) }
    fn arrows<'a>(&'a self) -> Box<Iterator<Item=&'a Self::Arrow> + 'a> { Box::new(once(&self.arrow)) }
}

#[derive(Default)]
struct Two<Object, Arrow> {
    objects: [Object; 2],
    arrows: [Arrow; 3],
}

impl<O: PartialEq + Eq + Hash + Clone + Default + Debug + Object, A: PartialEq + Debug + Clone + Arrow + Default> Category for Two<O, A> {
    type Object = O;
    type Arrow = A;

    fn domain(&self, f: &Self::Arrow) -> &Self::Object {
        match f {
            _ if f == &self.arrows[0] => &self.objects[0],
            _ if f == &self.arrows[1] => &self.objects[1],
            _ if f == &self.arrows[2] => &self.objects[0],
            _ => panic!(""),
        }
    }

    fn codomain(&self, f: &Self::Arrow) -> &Self::Object {
        match f {
            _ if f == &self.arrows[0] => &self.objects[0],
            _ if f == &self.arrows[1] => &self.objects[1],
            _ if f == &self.arrows[2] => &self.objects[1],
            _ => panic!(""),
        }
    }

    fn identity(&self, o: &Self::Object) -> &Self::Arrow {
        match o {
            _ if o == &self.objects[0] => &self.arrows[0],
            _ if o == &self.objects[1] => &self.arrows[1],
            _ => panic!(""),
        }
    }

    fn composition_internal<'a>(&self, f: &'a Self::Arrow, g: &'a Self::Arrow) -> &'a Self::Arrow {
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

    fn objects<'a>(&'a self) -> Box<Iterator<Item=&'a Self::Object> + 'a> {
        Box::new(self.objects.into_iter())
    }

    fn arrows<'a>(&'a self) -> Box<Iterator<Item=&'a Self::Arrow> + 'a> {
        Box::new(self.arrows.into_iter())
    }
}

#[derive(PartialEq, Default, Debug, Hash, Eq, Clone)]
struct NamedCategoryObject(&'static str);

impl Object for NamedCategoryObject {
    fn name(&self) -> &'static str { self.0 }
}

#[derive(PartialEq, Default, Debug, Clone)]
struct NamedCategoryArrow {
    name: &'static str,
    domain: usize,
    codomain: usize,
    equals: Vec<Vec<usize>>,
}

impl Arrow for NamedCategoryArrow {
    fn name(&self) -> &'static str { self.name }
}

cat3!(Three [X Y Z]);
type One_ = One<CategoryObject, CategoryArrow>;
type Two_ = Two<CategoryObject, CategoryArrow>;
type Three_ = Three<CategoryObject, CategoryArrow>;

#[derive(Default, Debug)]
struct SmallCat<Object, Arrow> {
    objects: Vec<Object>,
    arrows: Vec<Arrow>,
}
type SmallCat_ = SmallCat<CategoryObject, NamedCategoryArrow>;

impl<O: PartialEq + Eq + Hash + Clone + Default + Debug + Object> Category for SmallCat<O, NamedCategoryArrow> {
    type Object = O;
    type Arrow = NamedCategoryArrow;

    fn new(objects: Vec<Self::Object>, arrows: Vec<Self::Arrow>) -> Box<Self> { Box::new(SmallCat { objects, arrows }) }
    fn domain(&self, f: &Self::Arrow) -> &Self::Object { &self.objects[f.domain] }
    fn codomain(&self, f: &Self::Arrow) -> &Self::Object { &self.objects[f.codomain] }
    fn identity(&self, o: &Self::Object) -> &Self::Arrow {
        let idx = self.objects.iter().position(|x| x == o).unwrap();
        for a in self.arrows.iter() {
            if a.codomain == idx && a.domain == idx {
                return a
            }
        }
        panic!("");
    }
    fn composition_internal<'a>(&'a self, f: &'a Self::Arrow, g: &'a Self::Arrow) -> &'a Self::Arrow {
        let f_idx = self.arrows.iter().position(|x| x == f).unwrap();
        let g_idx = self.arrows.iter().position(|x| x == g).unwrap();
        for arrow in self.arrows.iter() {
            for eq in arrow.equals.iter() {
                if eq[0] == f_idx && eq[1] == g_idx {
                    return arrow;
                }
            }
        }
        panic!("")
    }
    fn objects<'a>(&'a self) -> Box<Iterator<Item=&'a Self::Object> + 'a> {
        Box::new(self.objects.iter())
    }
    fn arrows<'a>(&'a self) -> Box<Iterator<Item=&'a Self::Arrow> + 'a> {
        Box::new(self.arrows.iter())
    }
}

trait Functor {
    type DOM: Category;
    type COD: Category;

    fn send_objects(object: <Self::DOM as Category>::Object) -> <Self::COD as Category>::Object;
    fn send_arrows(arrow: <Self::DOM as Category>::Arrow) -> <Self::COD as Category>::Arrow;
}

trait NaturalTransformation {
    type C: Category;
    type D: Category;
    type F: Functor<DOM=Self::C, COD=Self::D>;
    type G: Functor<DOM=Self::C, COD=Self::D>;

    fn components() -> Box<Iterator<Item=<Self::D as Category>::Arrow>>;
}

fn main() {
    let _zero = Zero {};
    let _one = One_::default();
    let _two = Two_::default();
    let _three = Three_::default();

    let _small_cat_zero = SmallCat_::default();
    let _small_cat_one = SmallCat::<CategoryObject, CategoryArrow>::default();
    let _small_cat_two = SmallCat_ {
        objects: vec![CategoryObject::default(), CategoryObject::default()],
        arrows: vec![
            NamedCategoryArrow{ name: "a", codomain: 0, domain: 0, equals: vec![] },
            NamedCategoryArrow{ name: "b", codomain: 1, domain: 1, equals: vec![] },
            NamedCategoryArrow{ name: "f", codomain: 0, domain: 1, equals: vec![] },
        ]
    };
    let _small_cat_three = SmallCat_ {
        objects: vec![CategoryObject::default(), CategoryObject::default(), CategoryObject::default()],
        arrows: vec![
            NamedCategoryArrow{ name: "x", codomain: 0, domain: 0, equals: vec![] },
            NamedCategoryArrow{ name: "y", codomain: 1, domain: 1, equals: vec![] },
            NamedCategoryArrow{ name: "z", codomain: 2, domain: 2, equals: vec![] },
            NamedCategoryArrow{ name: "f", codomain: 0, domain: 1, equals: vec![] },
            NamedCategoryArrow{ name: "g", codomain: 1, domain: 2, equals: vec![] },
            NamedCategoryArrow{ name: "h", codomain: 0, domain: 2, equals: vec![vec![3, 4]] },
        ]
    };
    let _small_cat = cat!(
        [X Y Z]
        [f: X -> Y
         g: Y -> Z
         h: X -> Z]
        [f;g = h]
    );

    struct A<'a>(&'a str);
    let aaa = "a".to_string() + "b";
    let a = A(&aaa);
    println!("{:?}", a.0);


    // use std::rc::Rc;

    // enum Node<'a> {
    //     Link(Rc<Node<'a>>, Rc<Node<'a>>),
    //     Text(&'a str),
    //     Unit
    // };

    // let u = Rc::new(Node::Unit);
    // let t1 = Rc::new(Node::Text("a"));
    // let l1 = Rc::new(Node::Link(u.clone(), u.clone()));
    // let l2 = Rc::new(Node::Link(t1.clone(), u.clone()));
    
    // let t2 = Rc::new(Node::Text("b"));
    // let t3 = Rc::new(Node::Text("c"));
    // let l3 = Rc::new(Node::Link(t2.clone(), t3.clone()));

    let _struct_before = cat!(
        [Person String Integer]
        [first_name: Person -> String
         last_name: Person -> String
         age: Person -> Integer]
        []
    );

    let _struct_after1 = cat!(
        [Person1 String]
        [name: Person1 -> String]
        []
    );
    let _struct_after2 = cat!(
        [Person2 Integer]
        [age: Person2 -> Integer]
        []
    );
    
    use std::collections::HashMap;
    use std::collections::HashSet;

    fn split_scheme<C: Category + Default + Debug>(before: C, t: &'static str, f: fn(&'static str) -> &'static str) {
        let mut splitted: HashMap<&'static str, Vec<&C::Arrow>> = HashMap::new();

        for attribute in before.arrows()
            .filter(|a| 
                before.domain(a).name() == t && 
                before.codomain(a).name() != t) {

            let ss = f(attribute.name());
            if splitted.contains_key(ss) {
                let mut entry = splitted.get_mut(ss);
                entry.unwrap().push(attribute);
            } else {
                splitted.insert(ss, vec![attribute]);
            }            
        }

        println!("splitted: {:?}", splitted);

        for new_group in splitted.values() {
            let mut objects = HashSet::new();
            let mut arrows: Vec<&C::Arrow> = vec![];

            for arrow in new_group.iter() {
                objects.insert(before.domain(arrow));
                objects.insert(before.codomain(arrow));
                arrows.push(arrow.clone());
            }
            let objs = objects.into_iter().cloned().collect();
            println!("objs: {:?}", objs);
            let new_cat = C::new(objs, arrows.into_iter().cloned().collect());                

            println!("new_cat: {:?}", new_cat);
        }

        ()
    }

    split_scheme(_struct_before, "Person", |s| {
        if s.contains("name") {
            "name"
        } else {
            "age"
        }
    });
}
