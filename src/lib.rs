use std::collections::{HashMap, VecDeque};

enum Trace {
    Replace,
    Append,
    Remove,
    Keep,
}

struct AnnotTrace {
    trace: Trace,
    cost: f64,
}

impl AnnotTrace {
    fn new(cost: f64, trace: Trace) -> Self {
        Self { cost, trace }
    }
}

pub enum Merged<X, Y> {
    Replace(X, Y),
    Append(Y),
    Remove(X),
    Keep(X, Y),
}

impl<X: PartialEq, Y: PartialEq> PartialEq for Merged<X, Y> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Append(sy), Self::Append(oy)) => *sy == *oy,
            (Self::Keep(sx, sy), Self::Keep(ox, oy)) => *sx == *ox && *sy == *oy,
            (Self::Remove(sx), Self::Remove(ox)) => *sx == *ox,
            (Self::Replace(sx, sy), Self::Replace(ox, oy)) => *sx == *ox && *sy == *oy,
            _ => false,
        }
    }
}

pub fn constant_cost<T>(cost: f64) -> impl FnMut(&T) -> f64 {
    move |_| cost
}

pub fn merge<X, Y>(
    mut xs: VecDeque<X>,
    mut ys: VecDeque<Y>,
    mut is_same: impl FnMut(&X, &Y) -> bool,
    mut cost_of_replece: impl FnMut(&X, &Y) -> f64,
    mut cost_of_append: impl FnMut(&Y) -> f64,
    mut cost_of_remove: impl FnMut(&X) -> f64,
) -> VecDeque<Merged<X, Y>> {
    if xs.len() == 0 {
        return ys.into_iter().map(|y| Merged::Append(y)).collect();
    }

    if ys.len() == 0 {
        return xs.into_iter().map(|x| Merged::Remove(x)).collect();
    }

    let mut d: HashMap<[i32; 2], AnnotTrace> = HashMap::new();

    {
        {
            let mut i = 0;
            for x in &xs {
                d.insert(
                    [i, -1],
                    AnnotTrace::new(i as f64 * cost_of_remove(x), Trace::Remove),
                );
                i += 1;
            }
        }
        {
            let mut i = 0;
            for y in &ys {
                d.insert(
                    [-1, i],
                    AnnotTrace::new(i as f64 * cost_of_append(y), Trace::Append),
                );
                i += 1;
            }
        }
        d.insert([-1, -1], AnnotTrace::new(0.0, Trace::Keep));
    }

    {
        let mut xi = 0;
        for x in &xs {
            let mut yi = 0;
            for y in &ys {
                let mut is_keep = false;
                let replace = if is_same(&x, &y) {
                    is_keep = true;
                    d.get(&[xi - 1, yi - 1]).unwrap().cost
                } else {
                    d.get(&[xi - 1, yi - 1]).unwrap().cost + cost_of_replece(x, y)
                };
                let append = d.get(&[xi, yi - 1]).unwrap().cost + cost_of_append(y);
                let remove = d.get(&[xi - 1, yi]).unwrap().cost + cost_of_remove(x);

                if replace <= append && replace <= remove {
                    if is_keep {
                        d.insert([xi, yi], AnnotTrace::new(replace, Trace::Keep));
                    } else {
                        d.insert([xi, yi], AnnotTrace::new(replace, Trace::Replace));
                    }
                } else if append <= remove {
                    d.insert([xi, yi], AnnotTrace::new(append, Trace::Append));
                } else {
                    d.insert([xi, yi], AnnotTrace::new(remove, Trace::Remove));
                }
                yi += 1;
            }
            xi += 1;
        }

        d.remove(&[-1, -1]);
    }

    let res = {
        let mut res = VecDeque::new();
        let (mut xi, mut yi) = (xs.len() as i32 - 1, ys.len() as i32 - 1);
        while let Some(AnnotTrace { trace, .. }) = d.get(&[xi, yi]) {
            match trace {
                Trace::Replace => {
                    if let (Some(x), Some(y)) = (xs.pop_back(), ys.pop_back()) {
                        res.push_front(Merged::Replace(x, y));
                    }
                    xi -= 1;
                    yi -= 1;
                }
                Trace::Keep => {
                    if let (Some(x), Some(y)) = (xs.pop_back(), ys.pop_back()) {
                        res.push_front(Merged::Keep(x, y));
                    }
                    xi -= 1;
                    yi -= 1;
                }
                Trace::Append => {
                    if let Some(y) = ys.pop_back() {
                        res.push_front(Merged::Append(y));
                    }
                    yi -= 1;
                }
                Trace::Remove => {
                    if let Some(x) = xs.pop_back() {
                        res.push_front(Merged::Remove(x));
                    }
                    xi -= 1;
                }
            }
        }

        res
    };

    res
}
