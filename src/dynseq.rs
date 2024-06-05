//! DynSeq object

/* std use */

/* crate use */
use petgraph::visit::NodeIndexable as _;

/* project use */
use crate::error;
use crate::node;

/// Sequence with good edition time
pub struct DynSeq {
    graph: petgraph::graph::DiGraph<node::Node, ()>,
    node_index2pos: Vec<usize>,
}

impl DynSeq {
    /// Create a new DynSeq
    pub fn new(seq: &[u8], node_length: usize) -> error::Result<DynSeq> {
        if node_length > 29 {
            return Err(error::Error::SeqToLargeForNode);
        }

        let mut graph = petgraph::graph::DiGraph::new();
        let node_index2pos = (0..(seq.len() / node_length + 1))
            .map(|x| x * node_length)
            .collect();

        for subseq in seq.chunks(node_length) {
            graph.add_node(node::Node::new(subseq).expect("unreacheable subseq have good size"));
        }

        for a in 0..(seq.len() / node_length) {
            graph.add_edge(graph.from_index(a), graph.from_index(a + 1), ());
        }

        Ok(Self {
            graph,
            node_index2pos,
        })
    }

    pub(crate) fn in_vec(&self) -> Vec<u8> {
        let mut seq = Vec::with_capacity(self.graph.node_count() * 29);

        let mut dfs = petgraph::visit::Dfs::new(&self.graph, self.graph.from_index(0));
        while let Some(node) = dfs.next(&self.graph) {
            if let Some(weight) = self.graph.node_weight(node) {
                seq.extend(weight.seq())
            }
        }

        seq
    }

    /// Get local a value
    pub fn get(&self, index: usize) -> core::option::Option<u8> {
        let node_after = self
            .node_index2pos
            .iter()
            .position(|&x| x > index)
            .unwrap_or(self.node_index2pos.len());

        let node_index = self.graph.from_index(node_after - 1);
        let index_corr = index - self.node_index2pos[node_after - 1];
        let data = self.graph.node_weight(node_index).unwrap();

        data.seq().get(index_corr).copied()
    }
}

impl std::convert::From<DynSeq> for Vec<u8> {
    fn from(dynseq: DynSeq) -> Self {
        dynseq.in_vec()
    }
}

#[cfg(test)]
mod tests {
    /* std use */

    /* crate use */
    use biotest::Format as _;

    /* project use */
    use super::*;

    #[test]
    fn dynseq2vec() -> error::Result<()> {
        let mut rng = biotest::rand();
        let generator = biotest::Sequence::builder()
            .sequence_len(100)
            .build()
            .unwrap();
        let mut seq = vec![];
        generator.record(&mut seq, &mut rng).unwrap();

        let dynseq = DynSeq::new(&seq, 24)?;
        let out: Vec<u8> = dynseq.into();
        assert_eq!(out, seq.to_ascii_uppercase());

        Ok(())
    }

    #[test]
    fn get() -> error::Result<()> {
        let mut rng = biotest::rand();
        let generator = biotest::Sequence::builder()
            .sequence_len(100)
            .build()
            .unwrap();
        let mut seq = vec![];
        generator.record(&mut seq, &mut rng).unwrap();

        let dynseq = DynSeq::new(&seq, 24)?;
        for i in 0..seq.len() {
            assert_eq!(seq.get(i).map(u8::to_ascii_uppercase), dynseq.get(i));
        }

        Ok(())
    }
}
