use std::cmp::PartialEq;
use std::iter::empty;
use std::iter::once;

trait Category {
    type Object: PartialEq;
    type Arrow: PartialEq;

    fn domain(&self, f: &Self::Arrow) -> &Self::Object;
    fn codomain(&self, f: &Self::Arrow) -> &Self::Object;
    fn identity(&self, a: &Self::Object) -> &Self::Arrow;
    fn composition(&self, f: &Self::Arrow, g: &Self::Arrow) -> Option<&Self::Arrow> {
        if self.codomain(&f) != self.domain(&g) {
            None
        } else {
            Some(self.composition_internal(&f, &g))
        }
    }
    fn composition_internal(&self, f: &Self::Arrow, g: &Self::Arrow) -> &Self::Arrow;
    fn ci(&self, f: &Self::Arrow, g: &Self::Arrow) -> &Self::Arrow {
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
    fn composition_internal(&self, _f: &(), _g: &()) -> &() { &() }
    fn objects(&self) -> Box<Iterator<Item=&()>> { Box::new(empty()) }
    fn arrows(&self) -> Box<Iterator<Item=&()>> { Box::new(empty()) }
}

#[derive(PartialEq)]
struct CategoryObject;

#[derive(PartialEq)]
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
    fn composition_internal(&self, _f: &CategoryArrow, _g: &CategoryArrow) -> &CategoryArrow { &self.arrow }
    fn objects<'a>(&'a self) -> Box<Iterator<Item=&'a CategoryObject> + 'a> { Box::new(once(&self.object)) }
    fn arrows<'a>(&'a self) -> Box<Iterator<Item=&'a CategoryArrow> + 'a> { Box::new(once(&self.arrow)) }
}

fn main() {
    let _zero = Zero {};
    let _one = One::new();

    println!("{:?}", "hello category!");
}
