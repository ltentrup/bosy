use crate::automata::CoBuchiAutomaton;
use crate::logic::Logic;

impl<L: Logic> CoBuchiAutomaton<L> {
    pub fn print_dot(&self) {
        dot::render(self, &mut std::io::stdout()).unwrap_or_else(|e| println!("{}", e));
    }
}

type Nd = usize;
type Ed = (usize, usize);

impl<'a, L: Logic> dot::Labeller<'a, Nd, Ed> for CoBuchiAutomaton<L> {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("UCW").unwrap()
    }
    fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
        dot::Id::new(format!("n{}", n)).unwrap()
    }
    fn node_label<'b>(&'b self, n: &Nd) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr(self.states[*n].name.as_ref().unwrap().into())
    }
    fn edge_label<'b>(&'b self, edge: &Ed) -> dot::LabelText<'b> {
        let guard = &self.transitions[&edge.0][&edge.1];
        dot::LabelText::LabelStr(format!("{}", guard).into())
    }
}

impl<'a, L: Logic> dot::GraphWalk<'a, Nd, Ed> for CoBuchiAutomaton<L> {
    fn nodes(&self) -> dot::Nodes<'a, Nd> {
        (0..self.states.len()).collect()
    }
    fn edges(&'a self) -> dot::Edges<'a, Ed> {
        self.transitions
            .iter()
            .flat_map(|(&source, outgoing)| {
                outgoing.iter().map(move |(&target, _)| (source, target))
            })
            .collect()
    }
    fn source(&self, e: &Ed) -> Nd {
        e.0
    }
    fn target(&self, e: &Ed) -> Nd {
        e.1
    }
}
