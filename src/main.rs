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
    object_A: CategoryObject,
    object_B: CategoryObject,
    arrow_a: CategoryArrow,
    arrow_b: CategoryArrow,
    arrow_f: CategoryArrow
}

impl Category for Two {
    type Object = CategoryObject;
    type Arrow = CategoryArrow;

    fn domain(&self, f: &CategoryArrow) -> &CategoryObject {
        match f {
            _ if f == &self.arrow_a => &self.object_A,
            _ if f == &self.arrow_b => &self.object_B,
            _ if f == &self.arrow_f => &self.object_A,
            _ => panic!(""),
        }
    }
    fn codomain(&self, f: &CategoryArrow) -> &CategoryObject {
        match f {
            _ if f == &self.arrow_a => &self.object_A,
            _ if f == &self.arrow_b => &self.object_B,
            _ if f == &self.arrow_f => &self.object_B,
            _ => panic!(""),
        }
    }
    fn identity(&self, o: &CategoryObject) -> &CategoryArrow {
        match o {
            _ if o == &self.object_A => &self.arrow_a,
            _ if o == &self.object_B => &self.arrow_b,
            _ => panic!(""),
        }
    }
    fn composition_internal<'a>(&self, f: &'a CategoryArrow, g: &'a CategoryArrow) -> &'a CategoryArrow {
        if f == g {
            f
        } else if f == &self.arrow_a && g == &self.arrow_f {
            g
        } else if f == &self.arrow_f && g == &self.arrow_b {
            f
        } else {
            panic!("")
        }
    }
    fn objects<'a>(&'a self) -> Box<Iterator<Item=&'a CategoryObject> + 'a> {
        Box::new([&self.object_A, &self.object_B].iter())
    }
    fn arrows<'a>(&'a self) -> Box<Iterator<Item=&'a CategoryArrow> + 'a> {
        Box::new(once(&self.arrow_a))
    }
}

fn main() {
    let _zero = Zero {};
    let _one = One::new();
    let _two = Two::default();

    println!("{:?}", "hello category!");
}
