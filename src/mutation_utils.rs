use libafl::inputs::{HasBytesVec, Input};
use libafl::mutators::{MutationResult, MutatorsTuple};
use libafl::prelude::{
    tuple_list, BitFlipMutator, ByteAddMutator, ByteDecMutator, ByteFlipMutator, ByteIncMutator,
    ByteInterestingMutator, ByteNegMutator, ByteRandMutator, BytesCopyMutator, BytesExpandMutator,
    BytesInsertMutator, BytesRandInsertMutator, BytesRandSetMutator, BytesSetMutator,
    BytesSwapMutator, DwordAddMutator, DwordInterestingMutator, HasConstLen, Mutator, Prepend,
    QwordAddMutator, WordAddMutator, WordInterestingMutator,
};
use libafl::state::{HasMaxSize, HasRand, State};
use primitive_types::H160;
use rand::random;

pub fn byte_mutator<I, S>(state: &mut S, input: &mut I) -> MutationResult
where
    S: State + HasRand,
    I: HasBytesVec + Input,
{
    let mut mutations = tuple_list!(
        BitFlipMutator::new(),
        ByteFlipMutator::new(),
        ByteIncMutator::new(),
        ByteDecMutator::new(),
        ByteNegMutator::new(),
        ByteRandMutator::new(),
        ByteAddMutator::new(),
        WordAddMutator::new(),
        DwordAddMutator::new(),
        QwordAddMutator::new(),
        ByteInterestingMutator::new(),
        WordInterestingMutator::new(),
        DwordInterestingMutator::new(),
        BytesSetMutator::new(),
        BytesRandSetMutator::new(),
        BytesSwapMutator::new(),
    );
    mutations
        .get_and_mutate(random::<usize>() % mutations.len(), state, input, 0)
        .unwrap()
}

pub fn byte_mutator_with_expansion<I, S>(state: &mut S, input: &mut I) -> MutationResult
where
    S: State + HasRand + HasMaxSize,
    I: HasBytesVec + Input,
{
    let mut mutations = tuple_list!(
        BitFlipMutator::new(),
        ByteFlipMutator::new(),
        ByteIncMutator::new(),
        ByteDecMutator::new(),
        ByteNegMutator::new(),
        ByteRandMutator::new(),
        ByteAddMutator::new(),
        WordAddMutator::new(),
        DwordAddMutator::new(),
        QwordAddMutator::new(),
        ByteInterestingMutator::new(),
        WordInterestingMutator::new(),
        DwordInterestingMutator::new(),
        BytesExpandMutator::new(),
        BytesInsertMutator::new(),
        BytesRandInsertMutator::new(),
        BytesSetMutator::new(),
        BytesRandSetMutator::new(),
        BytesCopyMutator::new(),
        BytesSwapMutator::new(),
    );
    mutations
        .get_and_mutate(random::<usize>() % mutations.len(), state, input, 0)
        .unwrap()
}