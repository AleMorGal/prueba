use std::any::{Any, TypeId};
struct Nodo{
	content: Box<dyn Any>,
}

struct ListaDLigada{
	contenido: Nodo,
	nodoIzq: Box<Nodo>,
	nodoDer: Box<Nodo>,
}

fn main() {
	let c = Nodo{content:Box::new(5)};
	let lista = ListaDLigada{contenido:c, nodoDer:Box::<Nodo>, nodoIzq:Box::<Nodo>};
}
