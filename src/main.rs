use std::cmp::PartialEq;
use std::iter::empty;

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

    fn objects() -> Box<Iterator<Item=Self::Object>>;
    fn arrows() -> Box<Iterator<Item=Self::Arrow>>;

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

struct Zero {}

impl Category for Zero {
    type Object = ();
    type Arrow = ();

    fn domain(&self, _f: &()) {}
    fn codomain(&self, _f: &()) {}
    fn identity(&self, _a: ()) {}
    fn composition_internal(&self, _f: &(), _g: &()) {}
    fn objects() -> Box<Iterator<Item=()>> { Box::new(empty()) }
    fn arrows() -> Box<Iterator<Item=()>> { Box::new(empty()) }
}

fn main() {
    let _zero = Zero {};
    println!("{:?}", "hello category!");
}
