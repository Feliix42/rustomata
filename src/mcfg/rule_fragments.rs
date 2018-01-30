use {PMCFGRule, VarT};
use integeriser::Integeriser;
use cs_representation::BracketContent;
use cs_representation::bracket_fragment::BracketFragment;
use dyck::Bracket;

#[derive(Debug)]
pub enum RuleFragment<'a, N, T, W> where N: 'a, T: 'a, W: 'a {
    Start(&'a PMCFGRule<N, T, W>, usize, Vec<&'a T>, (usize, usize)),
    Intermediate(&'a PMCFGRule<N, T, W>, usize, (usize, usize), Vec<&'a T>, (usize, usize)),
    End(&'a PMCFGRule<N, T, W>, usize, (usize, usize), Vec<&'a T>),
    Whole(&'a PMCFGRule<N, T, W>, usize, Vec<&'a T>)
}

pub struct FragmentIterator<'a, N: 'a, T: 'a, W: 'a>(&'a PMCFGRule<N, T, W>, usize, i64);

pub fn fragments<'a, N: 'a, T: 'a, W: 'a>(rule: &'a PMCFGRule<N, T, W>) -> FragmentIterator<'a, N, T, W> {
    FragmentIterator(rule, 0, -1)
}

impl<'a, N, T, W> Iterator for FragmentIterator<'a, N, T, W> {
    type Item = RuleFragment<'a, N, T, W>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.0.composition.composition.len() {
            return None;
        }

        let component = &self.0.composition.composition[self.1];
        let mut terminals = Vec::new();
        
        let start_var = if self.2 == -1 { None } else { match component[self.2 as usize] { VarT::Var(i, j) => Some((i, j)), _ => None } };
        self.2 += 1;

        for index in (self.2 as usize)..component.len() {
            match component[index] {
                VarT::T(ref t) => terminals.push(t),
                VarT::Var(i, j) => {
                    if let Some((i_,j_)) = start_var {
                        self.2 = index as i64;
                        return Some(Intermediate(self.0, self.1, (i_, j_), terminals, (i,j)));
                    } else {
                        self.2 = index as i64;
                        return Some(Start(self.0, self.1, terminals, (i,j)));
                    }
                }
            }
        }
        let comp = self.1;
        self.1 += 1;
        self.2 = -1;
        if let Some((i, j)) = start_var {
            return Some(End(self.0, comp, (i, j), terminals));
        } else {
            return Some(Whole(self.0, comp, terminals));
        }
    }
}

use self::RuleFragment::*;

impl<'a, N, T, W> RuleFragment<'a, N, T, W> where T: Clone + PartialEq, N: Clone + PartialEq {
    fn rule(&self) -> &'a PMCFGRule<N, T, W> {
        match *self {
            Start(r, _, _, _) | Intermediate(r, _, _, _, _) | End(r, _, _, _) | Whole(r, _, _) => r 
        }
    }

    pub fn bracket_word(&self, integerizer: &Integeriser<Item=PMCFGRule<N, T, W>>) -> BracketFragment<T> {
        let mut bracks = Vec::new();
        let r = integerizer.find_key(self.rule()).unwrap();
        
        match *self {
            Start(_, j, _, _) => bracks.push(Bracket::Open(BracketContent::Component(r, j))),
            Intermediate(_, _, (i, j), _, _) => bracks.push(Bracket::Close(BracketContent::Variable(r, i, j))),
            End(_, _, (i, j), _) => bracks.push(Bracket::Close(BracketContent::Variable(r, i, j))),
            Whole(_, j, _) => bracks.push(Bracket::Open(BracketContent::Component(r, j)))
        };

        for symbol in self.terminals() {
            bracks.push(Bracket::Open(BracketContent::Terminal((*symbol).clone())));
            bracks.push(Bracket::Close(BracketContent::Terminal((*symbol).clone())));
        }

        match *self {
            Start(_, _, _, (i,j)) => bracks.push(Bracket::Open(BracketContent::Variable(r, i, j))),
            Intermediate(_, _, _, _, (i, j)) => bracks.push(Bracket::Open(BracketContent::Variable(r, i, j))),
            End(_, j, _, _) => bracks.push(Bracket::Close(BracketContent::Component(r, j))),
            Whole(_, j, _) => bracks.push(Bracket::Close(BracketContent::Component(r, j)))
        };
        
        BracketFragment(bracks)
    }

    pub fn terminals(&self) -> &[&'a T] {
        match *self {
            Start(_, _, ref ts, _) 
            | Intermediate(_, _, _, ref ts, _) 
            | End(_, _, _, ref ts) 
            | Whole(_, _, ref ts) => ts
        }
    }

    pub fn from(&self) -> Bracket<(N, usize)> {
        match *self {
            Start(r, j, _, _) => Bracket::Open((r.head.clone(), j)),
            Intermediate(r, _, (i, j), _, _) => Bracket::Close((r.tail[i].clone(), j)),
            End(r, _, (i, j), _) => Bracket::Close((r.tail[i].clone(), j)),
            Whole(r, j, _) => Bracket::Open((r.head.clone(), j))
        }
    }

    pub fn to(&self) -> Bracket<(N, usize)> {
        match *self {
            Start(r, _, _, (i,j)) => Bracket::Open((r.tail[i].clone(), j)),
            Intermediate(r, _, _, _, (i, j)) => Bracket::Open((r.tail[i].clone(), j)),
            End(r, j, _, _) => Bracket::Close((r.head.clone(), j)),
            Whole(r, j, _) => Bracket::Close((r.head.clone(), j))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pmcfg::Composition;
    use log_domain::LogDomain;
    use num_traits::One;
    use integeriser::{Integeriser, HashIntegeriser};
    
    #[test]
    fn fragments() {
        let rule: PMCFGRule<usize, usize, LogDomain<f64>> = PMCFGRule{ 
            head: 1, 
            tail: vec![1], 
            composition: Composition{ composition: vec![vec![VarT::T(1), VarT::Var(0,0), VarT::T(2)]]}, 
            weight: LogDomain::one()
        };

        let mut int = HashIntegeriser::new();
        int.integerise(rule.clone());

        eprintln!("{:?}", super::fragments(&rule).map(|f| f.bracket_word(&int)).collect::<Vec<_>>());
    }
}