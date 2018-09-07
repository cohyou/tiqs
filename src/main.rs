macro_rules! cat {
    ($n:ident [ $( $object:ident )* ]) => {
        #[derive(Default)]
        struct $n<Object, Arrow> {
            objects: [Object; 3],
            arrows: [Arrow; 6],
        }

        impl<O: PartialEq, A: PartialEq> Category for $n<O, A> {
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

            fn composition_internal<'a>(&'a mut self, f: &'a Self::Arrow, g: &'a Self::Arrow) -> &'a Self::Arrow {
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

trait Category {
    type Object: PartialEq;
    type Arrow: PartialEq;

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
    fn composition_internal<'a>(&'a mut self, f: &'a Self::Arrow, g: &'a Self::Arrow) -> &'a Self::Arrow;
    fn ci<'a>(&'a mut self, f: &'a Self::Arrow, g: &'a Self::Arrow) -> &'a Self::Arrow {
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
    fn composition_internal<'a>(&mut self, _f: &'a (), _g: &'a ()) -> &'a () { &() }
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

impl<O: PartialEq, A: PartialEq> Category for One<O, A> {
    type Object = O;
    type Arrow = A;

    fn domain(&self, _f: &Self::Arrow) -> &Self::Object { &self.object }
    fn codomain(&self, _f: &Self::Arrow) -> &Self::Object { &self.object }
    fn identity(&self, _a: &Self::Object) -> &Self::Arrow { &self.arrow }
    fn composition_internal<'a>(&'a mut self, _f: &'a Self::Arrow, _g: &'a Self::Arrow) -> &'a Self::Arrow { &self.arrow }
    fn objects<'a>(&'a self) -> Box<Iterator<Item=&'a Self::Object> + 'a> { Box::new(once(&self.object)) }
    fn arrows<'a>(&'a self) -> Box<Iterator<Item=&'a Self::Arrow> + 'a> { Box::new(once(&self.arrow)) }
}

#[derive(Default)]
struct Two<Object, Arrow> {
    objects: [Object; 2],
    arrows: [Arrow; 3],
}

impl<O: PartialEq, A: PartialEq> Category for Two<O, A> {
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

    fn composition_internal<'a>(&mut self, f: &'a Self::Arrow, g: &'a Self::Arrow) -> &'a Self::Arrow {
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

#[derive(PartialEq, Default)]
struct NamedCategoryObject(&'static str);

#[derive(PartialEq)]
struct NamedCategoryArrow<'o, Object: 'o>(&'static str, &'o Object, &'o Object);

cat!(Three [X Y Z]);
type One_ = One<CategoryObject, CategoryArrow>;
type Two_ = Two<CategoryObject, CategoryArrow>;
type Three_ = Three<CategoryObject, CategoryArrow>;

#[derive(Default)]
struct SmallCat<Object, Category> {
    objects: Vec<Object>,
    arrows: Vec<Category>,
}
type SmallCat_ = SmallCat<CategoryObject, CategoryArrow>;

impl<'o, O: PartialEq> Category for SmallCat<O, NamedCategoryArrow<'o, O>> {
    type Object = O;
    type Arrow = NamedCategoryArrow<'o, O>;

    fn domain(&self, f: &Self::Arrow) -> &Self::Object { &f.1 }
    fn codomain(&self, f: &Self::Arrow) -> &Self::Object { &f.2 }
    fn identity(&self, o: &Self::Object) -> &Self::Arrow {
        for a in self.arrows.iter() {
            if a.1 == o && a.2 == o {
                return a
            }
        }
        panic!("");
    }
    fn composition_internal<'a>(&'a self, f: &'a Self::Arrow, g: &'a Self::Arrow) -> &'a Self::Arrow {
        let new_arrow = NamedCategoryArrow(&format!("{};{}", f.0, g.0), f.1, g.2);
        self.arrows.push(new_arrow);
        &self.arrows.last().unwrap()
    }
    fn objects<'a>(&'a self) -> Box<Iterator<Item=&'a Self::Object> + 'a> {
        Box::new(self.objects.iter())
    }
    fn arrows<'a>(&'a self) -> Box<Iterator<Item=&'a Self::Arrow> + 'a> {
        Box::new(self.arrows.iter())
    }
}

fn main() {
    let _zero = Zero {};
    let _one = One_::default();
    let _two = Two_::default();
    let _three = Three_::default();
    let _small_cat_zero = SmallCat_::default();
}
