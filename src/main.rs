use std::cmp::PartialEq;
use std::collections::HashSet;

trait Category {
    type Object: PartialEq;
    type Arrow: PartialEq;

    fn domain(&self, f: &Self::Arrow) -> Self::Object;
    fn codomain(&self, f: &Self::Arrow) -> Self::Object;
    fn identity(&self, a: Self::Object) -> Self::Arrow;
    fn composition(&self, f: Self::Arrow, g: Self::Arrow) -> Option<Self::Arrow> {
        if self.codomain(&f) != self.domain(&g) {
            None
        } else {
            Some(self.composition_internal(&f, &g))
        }
    }
    fn composition_internal(&self, f: &Self::Arrow, g: &Self::Arrow) -> Self::Arrow;
    fn ci(&self, f: &Self::Arrow, g: &Self::Arrow) -> Self::Arrow {
        self.composition_internal(f, g)
    }

    fn objects() -> HashSet<Self::Object>;
    fn arrows() -> HashSet<Self::Arrow>;

    fn associativity(&self, f: Self::Arrow, g: Self::Arrow, k: Self::Arrow) -> bool {
        self.ci(&self.ci(&f, &g), &k) == self.ci(&f, &self.ci(&g, &k))
    }
    fn unit_law_domain(&self, o: Self::Object, f: Self::Arrow) -> bool {
        self.ci(&self.identity(o), &f) == f
    }
    fn unit_law_codmain(&self, o: Self::Object, f: Self::Arrow) -> bool {
        self.ci(&f, &self.identity(o)) == f
    }
}

trait Functor {
    type C: Category;
    type D: Category;
    fn send_objects(&self, c: <<Self as Functor>::C as Category>::Object) -> <<Self as Functor>::D as Category>::Object;
    fn send_arrows(&self, c: <<Self as Functor>::C as Category>::Arrow) -> <<Self as Functor>::D as Category>::Arrow;
}

trait Foo {
    fn bar<'a>(&'a self) -> Box<Iterator<Item=&'a u8>>;
}

struct Baz {}

impl Foo for Baz {
    fn bar<'a>(&'a self) -> Box<Iterator<Item=&'a u8> + 'a> {
        let v = vec![1u8];
        Box::new(v.iter())
    }
}

// #[derive(PartialEq, Eq, Hash)]
// struct CategoryObject {}
//
// #[derive(PartialEq, Eq, Hash)]
// struct CategoryArrow {}
//
// struct Zero {}
//
// impl Category for Zero {
//     type Object = CategoryObject;
//     type Arrow = CategoryArrow;
//
//     fn domain(&self, _f: &CategoryArrow) -> CategoryObject { CategoryObject {} }
//     fn codomain(&self, _f: &CategoryArrow) -> CategoryObject { CategoryObject {} }
//     fn identity(&self, _a: CategoryObject) -> CategoryArrow { CategoryArrow {} }
//     fn composition_internal(&self, _f: &CategoryArrow, _g: &CategoryArrow) -> CategoryArrow { CategoryArrow {} }
//     fn objects() -> HashSet<CategoryObject> { HashSet::new() }
//     fn arrows() -> HashSet<CategoryArrow> { HashSet::new() }
// }

struct Zero {}

impl Category for Zero {
    type Object = ();
    type Arrow = ();

    fn domain(&self, _f: &()) {}
    fn codomain(&self, _f: &()) {}
    fn identity(&self, _a: ()) {}
    fn composition_internal(&self, _f: &(), _g: &()) {}
    fn objects() -> HashSet<()> { HashSet::new() }
    fn arrows() -> HashSet<()> { HashSet::new() }
}

fn main() {
    let zero = Zero {};
    println!("{:?}", "hello category!");
}
