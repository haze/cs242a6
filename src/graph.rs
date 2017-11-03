// There is no way I can know this works until I have someone give me the test code
// for the class. I doubt someone will share it with me since they are paying tuition
// but yeah, this is probably the furthest I can go.

pub trait Graph<T> {
    type Node;
    fn add_node(&mut self, x: T) -> Self::Node;

    // add_edge(G, x, y): adds the edge from the vertex x to the vertex y, if it is not there;

    // neighbors(G, x): lists all vertices y such that there is an edge from the vertex x to the vertex y;
    fn add_edge() {};
    fn neighbors(&self) -> Vec<Node> {
        self.
    };


    fn value(&self, n: Self::Node) -> &T {}
    fn connected(&self, n1: Self::Node, n2: Self::Node) -> bool;
}

