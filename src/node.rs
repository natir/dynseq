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

    /// Get nucleotide in ascii representation
    pub fn get(&self, index: usize) -> core::option::Option<u8> {
        self.get_bits(index).map(cocktail::kmer::bit2nuc)
    }

    /// Get nucleotide in bits representation
    pub fn get_bits(&self, index: usize) -> core::option::Option<u64> {
        if index >= self.len() as usize {
            None
        } else {
            let corr_index = (self.len() - index as u64 - 1) * 2;

            Some((self.sequence() >> corr_index) & 0b11)
        }
    }

    /// Set nucleotide in ascii representation
    pub fn set(&mut self, index: usize, nuc: u8) -> core::option::Option<()> {
        self.set_bits(index, cocktail::kmer::nuc2bit(nuc))
    }

    /// Set nucleotide in bits representation
    pub fn set_bits(&mut self, index: usize, value: u64) -> core::option::Option<()> {
        if index >= self.len() as usize {
            None
        } else {
            let corr_index = (self.len() - index as u64 - 1) * 2;
            let mask_left = mask(self.len() as usize * 2, corr_index as usize + 2);
            let mask_right = mask(corr_index as usize, 0);
            let clean_mask = mask_left | (0b00 << corr_index) | mask_right;
            let mask = value << corr_index;

            self.set_sequence((self.sequence() & clean_mask) ^ mask);
            Some(())
        }
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
            let bin_index = (self.len() as usize * 2) - (index * 2);

            let begin =
                (self.sequence() & mask((self.len() * 2) as usize, bin_index)) << (2 * nucs.len());
            let end = self.sequence() & mask(bin_index, 0);
            let middle = cocktail::kmer::seq2bit(nucs) << bin_index;

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
    fn get_set() -> error::Result<()> {
        let mut rng = biotest::rand();
        let generator = biotest::Sequence::builder()
            .sequence_len(28)
            .build()
            .unwrap();
        let mut seq = vec![];
        generator.record(&mut seq, &mut rng).unwrap();

        let mut node = Node::new(&seq)?;

        for i in 0..28 {
            assert_eq!(node.get(i), seq.to_ascii_uppercase().get(i).copied());
        }

        for (i, nuc) in seq.iter().rev().enumerate() {
            assert!(node.set(i, *nuc).is_some());
        }

        seq.reverse();
        assert_eq!(node.seq(), seq.to_ascii_uppercase());

        assert!(node.get(28).is_none());
        assert!(node.set(28, b'N').is_none());

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
