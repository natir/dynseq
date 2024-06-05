//! Define node in graph

/* std use */

/* crate use */

/* local use */
use crate::error;

bitfield::bitfield! {
    /// DynBinSeq node, first 5 bite is length of sequence, other bytes cocktail representation of sequence.
    pub struct Node(u64);
    impl Debug;
    /// Length of sequence
    length, set_length: 4, 0;
    /// Sequence in cocktail representation
    sequence, set_sequence: 63, 5;
}

impl Node {
    /// Create a new node
    pub fn new(seq: &[u8]) -> error::Result<Self> {
        if seq.len() > 29 {
            Err(error::Error::SeqToLargeForNode)
        } else {
            let mut obj = Node(0);

            obj.set_length(seq.len() as u64);
            obj.set_sequence(cocktail::kmer::seq2bit(seq));

            Ok(obj)
        }
    }

    /// Get length of sequence
    pub fn len(&self) -> u64 {
        self.length()
    }

    /// Return true if node is_empty
    pub fn is_empty(&self) -> bool {
        self.length() == 0
    }

    /// Get sequence in cocktail representation
    pub fn bits(&self) -> u64 {
        self.sequence()
    }

    /// Get sequence in ascii representation
    pub fn seq(&self) -> Vec<u8> {
        cocktail::kmer::kmer2seq(self.sequence(), self.len() as u8)
    }

    /// Add nucleotide at end
    pub fn push_back(&mut self, nuc: u8) -> error::Result<()> {
        if self.len() > 28 {
            Err(error::Error::SeqToLargeForNode)
        } else {
            let value = self.sequence() << 2 | cocktail::kmer::nuc2bit(nuc);
            self.set_sequence(value);
            self.set_length(self.length() + 1);

            Ok(())
        }
    }

    /// Add nucleotide at begin
    pub fn push_front(&mut self, nuc: u8) -> error::Result<()> {
        if self.len() > 28 {
            Err(error::Error::SeqToLargeForNode)
        } else {
            let value = cocktail::kmer::nuc2bit(nuc) << (self.length() * 2) | self.sequence();
            self.set_sequence(value);
            self.set_length(self.length() + 1);

            Ok(())
        }
    }

    /// Insert a nucleotide at position
    pub fn insert(&mut self, index: usize, nucs: &[u8]) -> error::Result<()> {
        if self.len() as usize + nucs.len() > 29 {
            Err(error::Error::SeqToLargeForNode)
        } else {
            println!("{} {}", index, self.len() * 2);

            let bin_index = (self.len() as usize * 2) - (index * 2);

            let begin =
                (self.sequence() & mask((self.len() * 2) as usize, bin_index)) << (2 * nucs.len());
            let end = self.sequence() & mask(bin_index, 0);
            let middle = cocktail::kmer::seq2bit(nucs) << bin_index;

            println!(
                "{:058b}",
                mask((self.len() * 2) as usize, bin_index) << (2 * nucs.len())
            );
            println!("{:058b}", mask(bin_index, 0));
            println!("{:058b}", middle);

            self.set_sequence(begin | middle | end);
            self.set_length(self.len() + nucs.len() as u64);
            Ok(())
        }
    }
}

fn mask(msb: usize, lsb: usize) -> u64 {
    let len = msb - lsb;
    let mask: u64 = (1 << len) - 1;

    mask << lsb
}

#[cfg(test)]
mod tests {
    /* std use */

    /* crate format */
    use biotest::Format as _;

    /* local use */
    use super::*;

    #[test]
    fn create_node() -> error::Result<()> {
        let mut rng = biotest::rand();
        let generator = biotest::Sequence::builder()
            .sequence_len(29)
            .build()
            .unwrap();
        let mut seq = vec![];
        generator.record(&mut seq, &mut rng).unwrap();

        let node = Node::new(&seq)?;

        assert!(!node.is_empty());
        assert_eq!(node.len(), 29);
        assert_eq!(node.bits(), 153899156765281607);
        assert_eq!(node.seq(), seq.to_ascii_uppercase());

        seq.push(b'C');
        assert!(matches!(
            Node::new(&seq),
            Err(error::Error::SeqToLargeForNode)
        ));

        Ok(())
    }

    #[test]
    fn edit() -> error::Result<()> {
        let mut rng = biotest::rand();
        let generator = biotest::Sequence::builder()
            .sequence_len(25)
            .build()
            .unwrap();
        let mut seq = vec![];
        generator.record(&mut seq, &mut rng).unwrap();

        let mut node = Node::new(&seq)?;
        assert_eq!(node.seq(), seq.to_ascii_uppercase());

        node.push_back(b'A')?;
        seq.push(b'A');
        assert_eq!(node.seq(), seq.to_ascii_uppercase());

        node.push_front(b'G')?;
        seq.insert(0, b'G');
        assert_eq!(node.seq(), seq.to_ascii_uppercase());

        node.insert(10, b"GC")?;
        seq.insert(10, b'G');
        seq.insert(11, b'C');
        assert_eq!(node.seq(), seq.to_ascii_uppercase());

        assert!(matches!(
            node.push_back(b'C'),
            Err(error::Error::SeqToLargeForNode)
        ));

        assert!(matches!(
            node.push_front(b'C'),
            Err(error::Error::SeqToLargeForNode)
        ));

        assert!(matches!(
            node.insert(10, b"GC"),
            Err(error::Error::SeqToLargeForNode)
        ));

        Ok(())
    }
}
