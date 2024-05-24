use std::fmt::Debug;

pub trait Category {
    type Object;
    type Morphism: Fn(&Self::Object) -> Self::Object;

    fn id(obj: &Self::Object) -> Self::Object;
    fn compose(morph_1: Self::Morphism, morph_2: Self::Morphism) -> Self::Morphism;
}

pub struct OpX<T: Clone> {
    basis: Vec<Vec<T>>,
    opensets: Vec<Vec<T>>,
}

impl<T: Clone + 'static> Category for OpX<T> {
    type Object = Vec<T>;
    type Morphism = Box<dyn Fn(&Self::Object) -> Self::Object + 'static>;

    fn id(obj: &Self::Object) -> Self::Object {
        obj.clone()
    }

    fn compose(morph_1: Self::Morphism, morph_2: Self::Morphism) -> Self::Morphism {
        Box::new(move |x| morph_2(&morph_1(x)))
    }
}

impl<T: Clone + 'static + Eq + Debug> OpX<T> {
    pub fn new(base: Vec<Vec<T>>) -> OpX<T> {
        OpX {
            basis: base.clone(),
            opensets: base,
        }
    }

    pub fn make_openset_from_basis(&mut self, union_idx: Vec<usize>) {
        let mut cumulative_vec: Vec<T> = Vec::new();
        for idx in union_idx {
            let base = self.basis[idx].clone();
            cumulative_vec.extend(base);
        }
        self.opensets.push(cumulative_vec);
    }

    pub fn inlcusion(&self, from: &Vec<T>, to: &Vec<T>) -> Option<<OpX<T> as Category>::Morphism> {
        if from.iter().all(|item| to.contains(item)) {
            let to_clone = to.clone();
            Some(Box::new(move |_x| to_clone.clone()))
        } else {
            None
        }
    }
}
