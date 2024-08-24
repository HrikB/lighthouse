use crate::historical_summary::HistoricalSummary;
use crate::test_utils::TestRandom;
use crate::*;
use compare_fields_derive::CompareFields;
use derivative::Derivative;
pub use eth_spec::*;
use metastruct::metastruct;
pub use milhouse::{interface::Interface, List, Vector};
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use superstruct::superstruct;
use test_random_derive::TestRandom;
use tree_hash::TreeHash;
use tree_hash_derive::TreeHash;

/// The state of the `BeaconChain` at some slot.
#[superstruct(
    variants(Base, Altair, Bellatrix, Capella, Deneb, Electra),
    variant_attributes(
        derive(
            Derivative,
            Debug,
            PartialEq,
            Serialize,
            Deserialize,
            Encode,
            Decode,
            TreeHash,
            TestRandom,
            CompareFields,
            arbitrary::Arbitrary,
        ),
        serde(bound = "E: EthSpec", deny_unknown_fields),
        arbitrary(bound = "E: EthSpec"),
        derivative(Clone),
    ),
    specific_variant_attributes(
        Base(metastruct(
            mappings(
                map_beacon_state2_base_fields(),
                map_beacon_state2_base_tree_list_fields(mutable, fallible, groups(tree_lists)),
                map_beacon_state2_base_tree_list_fields_immutable(groups(tree_lists)),
            ),
            bimappings(bimap_beacon_state2_base_tree_list_fields(
                other_type = "BeaconState2Base",
                self_mutable,
                fallible,
                groups(tree_lists)
            )),
            num_fields(all()),
        )),
        Altair(metastruct(
            mappings(
                map_beacon_state2_altair_fields(),
                map_beacon_state2_altair_tree_list_fields(mutable, fallible, groups(tree_lists)),
                map_beacon_state2_altair_tree_list_fields_immutable(groups(tree_lists)),
            ),
            bimappings(bimap_beacon_state2_altair_tree_list_fields(
                other_type = "BeaconState2Altair",
                self_mutable,
                fallible,
                groups(tree_lists)
            )),
            num_fields(all()),
        )),
        Bellatrix(metastruct(
            mappings(
                map_beacon_state2_bellatrix_fields(),
                map_beacon_state2_bellatrix_tree_list_fields(
                    mutable,
                    fallible,
                    groups(tree_lists)
                ),
                map_beacon_state2_bellatrix_tree_list_fields_immutable(groups(tree_lists)),
            ),
            bimappings(bimap_beacon_state2_bellatrix_tree_list_fields(
                other_type = "BeaconState2Bellatrix",
                self_mutable,
                fallible,
                groups(tree_lists)
            )),
            num_fields(all()),
        )),
        Capella(metastruct(
            mappings(
                map_beacon_state2_capella_fields(),
                map_beacon_state2_capella_tree_list_fields(mutable, fallible, groups(tree_lists)),
                map_beacon_state2_capella_tree_list_fields_immutable(groups(tree_lists)),
            ),
            bimappings(bimap_beacon_state2_capella_tree_list_fields(
                other_type = "BeaconState2Capella",
                self_mutable,
                fallible,
                groups(tree_lists)
            )),
            num_fields(all()),
        )),
        Deneb(metastruct(
            mappings(
                map_beacon_state2_deneb_fields(),
                map_beacon_state2_deneb_tree_list_fields(mutable, fallible, groups(tree_lists)),
                map_beacon_state2_deneb_tree_list_fields_immutable(groups(tree_lists)),
            ),
            bimappings(bimap_beacon_state2_deneb_tree_list_fields(
                other_type = "BeaconState2Deneb",
                self_mutable,
                fallible,
                groups(tree_lists)
            )),
            num_fields(all()),
        )),
        Electra(metastruct(
            mappings(
                map_beacon_state2_electra_fields(),
                map_beacon_state2_electra_tree_list_fields(mutable, fallible, groups(tree_lists)),
                map_beacon_state2_electra_tree_list_fields_immutable(groups(tree_lists)),
            ),
            bimappings(bimap_beacon_state2_electra_tree_list_fields(
                other_type = "BeaconState2Electra",
                self_mutable,
                fallible,
                groups(tree_lists)
            )),
            num_fields(all()),
        ))
    ),
    cast_error(ty = "Error", expr = "Error::IncorrectStateVariant"),
    partial_getter_error(ty = "Error", expr = "Error::IncorrectStateVariant"),
    map_ref_mut_into(BeaconStateRef)
)]
#[derive(
    Debug, PartialEq, Clone, Serialize, Deserialize, Encode, TreeHash, arbitrary::Arbitrary,
)]
#[serde(untagged)]
#[serde(bound = "E: EthSpec")]
#[arbitrary(bound = "E: EthSpec")]
#[tree_hash(enum_behaviour = "transparent")]
#[ssz(enum_behaviour = "transparent")]
pub struct BeaconState2<E>
where
    E: EthSpec,
{
    // Versioning
    #[superstruct(getter(copy))]
    #[metastruct(exclude_from(tree_lists))]
    #[serde(with = "serde_utils::quoted_u64")]
    pub genesis_time: u64,
    #[superstruct(getter(copy))]
    #[metastruct(exclude_from(tree_lists))]
    pub genesis_validators_root: Hash256,
    #[superstruct(getter(copy))]
    #[metastruct(exclude_from(tree_lists))]
    pub slot: Slot,
    #[superstruct(getter(copy))]
    #[metastruct(exclude_from(tree_lists))]
    pub fork: Fork,

    // History
    #[metastruct(exclude_from(tree_lists))]
    pub latest_block_header: BeaconBlockHeader,
    #[superstruct(getter(copy))]
    #[metastruct(exclude_from(tree_lists))]
    pub block_roots: Hash256,
    #[test_random(default)]
    #[compare_fields(as_iter)]
    pub state_roots: Vector<Hash256, E::SlotsPerHistoricalRoot>,
    // Frozen in Capella, replaced by historical_summaries
    #[test_random(default)]
    #[compare_fields(as_iter)]
    pub historical_roots: List<Hash256, E::HistoricalRootsLimit>,

    #[superstruct(getter(copy))]
    #[metastruct(exclude_from(tree_lists))]
    pub eth1_data: Hash256,
    #[superstruct(getter(copy))]
    #[metastruct(exclude_from(tree_lists))]
    pub eth1_data_votes: Hash256,
    #[superstruct(getter(copy))]
    #[metastruct(exclude_from(tree_lists))]
    #[serde(with = "serde_utils::quoted_u64")]
    pub eth1_deposit_index: u64,

    #[superstruct(getter(copy))]
    #[metastruct(exclude_from(tree_lists))]
    pub validators: Hash256,
    #[superstruct(getter(copy))]
    #[metastruct(exclude_from(tree_lists))]
    pub balances: Hash256,

    // Randomness
    #[superstruct(getter(copy))]
    #[metastruct(exclude_from(tree_lists))]
    pub randao_mixes: Hash256,

    #[superstruct(getter(copy))]
    #[metastruct(exclude_from(tree_lists))]
    pub slashings: Hash256,

    // Attestations (genesis fork only)
    #[superstruct(only(Base))]
    #[metastruct(exclude_from(tree_lists))]
    pub previous_epoch_attestations: Hash256,
    #[superstruct(only(Base))]
    #[metastruct(exclude_from(tree_lists))]
    pub current_epoch_attestations: Hash256,

    // Participation (Altair and later)
    #[superstruct(only(Altair, Bellatrix, Capella, Deneb, Electra))]
    #[metastruct(exclude_from(tree_lists))]
    pub previous_epoch_participation: Hash256,
    #[superstruct(only(Altair, Bellatrix, Capella, Deneb, Electra))]
    #[metastruct(exclude_from(tree_lists))]
    pub current_epoch_participation: Hash256,

    // Finality
    #[test_random(default)]
    #[metastruct(exclude_from(tree_lists))]
    pub justification_bits: Hash256,
    #[superstruct(getter(copy))]
    #[metastruct(exclude_from(tree_lists))]
    pub previous_justified_checkpoint: Hash256,
    #[superstruct(getter(copy))]
    #[metastruct(exclude_from(tree_lists))]
    pub current_justified_checkpoint: Hash256,
    #[superstruct(getter(copy))]
    #[metastruct(exclude_from(tree_lists))]
    pub finalized_checkpoint: Hash256,

    // Inactivity
    #[superstruct(only(Altair, Bellatrix, Capella, Deneb, Electra))]
    #[superstruct(getter(copy))]
    #[metastruct(exclude_from(tree_lists))]
    pub inactivity_scores: Hash256,

    // Light-client sync committees
    #[superstruct(only(Altair, Bellatrix, Capella, Deneb, Electra))]
    #[metastruct(exclude_from(tree_lists))]
    pub current_sync_committee: Hash256,
    #[superstruct(only(Altair, Bellatrix, Capella, Deneb, Electra))]
    #[metastruct(exclude_from(tree_lists))]
    pub next_sync_committee: Hash256,

    // Execution
    #[superstruct(
        only(Bellatrix),
        partial_getter(rename = "latest_execution_payload_header_bellatrix")
    )]
    #[metastruct(exclude_from(tree_lists))]
    pub latest_execution_payload_header: ExecutionPayloadHeaderBellatrix<E>,
    #[superstruct(
        only(Capella),
        partial_getter(rename = "latest_execution_payload_header_capella")
    )]
    #[metastruct(exclude_from(tree_lists))]
    pub latest_execution_payload_header: ExecutionPayloadHeaderCapella<E>,
    #[superstruct(
        only(Deneb),
        partial_getter(rename = "latest_execution_payload_header_deneb")
    )]
    #[metastruct(exclude_from(tree_lists))]
    pub latest_execution_payload_header: ExecutionPayloadHeaderDeneb<E>,
    #[superstruct(
        only(Electra),
        partial_getter(rename = "latest_execution_payload_header_electra")
    )]
    #[metastruct(exclude_from(tree_lists))]
    pub latest_execution_payload_header: ExecutionPayloadHeaderElectra<E>,

    // Capella
    #[superstruct(only(Capella, Deneb, Electra), partial_getter(copy))]
    #[serde(with = "serde_utils::quoted_u64")]
    #[metastruct(exclude_from(tree_lists))]
    pub next_withdrawal_index: u64,
    #[superstruct(only(Capella, Deneb, Electra), partial_getter(copy))]
    #[serde(with = "serde_utils::quoted_u64")]
    #[metastruct(exclude_from(tree_lists))]
    pub next_withdrawal_validator_index: u64,
    // Deep history valid from Capella onwards.
    #[superstruct(only(Capella, Deneb, Electra))]
    #[test_random(default)]
    pub historical_summaries: List<HistoricalSummary, E::HistoricalRootsLimit>,

    // Electra
    #[superstruct(only(Electra), partial_getter(copy))]
    #[metastruct(exclude_from(tree_lists))]
    #[serde(with = "serde_utils::quoted_u64")]
    pub deposit_receipts_start_index: u64,
    #[superstruct(only(Electra), partial_getter(copy))]
    #[metastruct(exclude_from(tree_lists))]
    #[serde(with = "serde_utils::quoted_u64")]
    pub deposit_balance_to_consume: u64,
    #[superstruct(only(Electra), partial_getter(copy))]
    #[metastruct(exclude_from(tree_lists))]
    #[serde(with = "serde_utils::quoted_u64")]
    pub exit_balance_to_consume: u64,
    #[superstruct(only(Electra), partial_getter(copy))]
    #[metastruct(exclude_from(tree_lists))]
    pub earliest_exit_epoch: Epoch,
    #[superstruct(only(Electra), partial_getter(copy))]
    #[metastruct(exclude_from(tree_lists))]
    #[serde(with = "serde_utils::quoted_u64")]
    pub consolidation_balance_to_consume: u64,
    #[superstruct(only(Electra), partial_getter(copy))]
    #[metastruct(exclude_from(tree_lists))]
    pub earliest_consolidation_epoch: Epoch,
    #[test_random(default)]
    #[superstruct(only(Electra))]
    pub pending_balance_deposits: List<PendingBalanceDeposit, E::PendingBalanceDepositsLimit>,
    #[test_random(default)]
    #[superstruct(only(Electra))]
    pub pending_partial_withdrawals:
        List<PendingPartialWithdrawal, E::PendingPartialWithdrawalsLimit>,
    #[test_random(default)]
    #[superstruct(only(Electra))]
    pub pending_consolidations: List<PendingConsolidation, E::PendingConsolidationsLimit>,
}

impl<E: EthSpec> BeaconState2<E> {
    pub fn from_ref(state: &BeaconState<E>) -> Self {
        match state {
            BeaconState::Base(state) => BeaconState2::Base(BeaconState2Base {
                genesis_time: state.genesis_time,
                genesis_validators_root: state.genesis_validators_root,
                slot: state.slot,
                fork: state.fork,
                latest_block_header: state.latest_block_header.clone(),
                block_roots: state.block_roots.tree_hash_root(),
                state_roots: state.state_roots.clone(),
                historical_roots: state.historical_roots.clone(),
                eth1_data: state.eth1_data.tree_hash_root(),
                eth1_data_votes: state.eth1_data_votes.tree_hash_root(),
                eth1_deposit_index: state.eth1_deposit_index,
                validators: state.validators.tree_hash_root(),
                balances: state.balances.tree_hash_root(),
                randao_mixes: state.randao_mixes.tree_hash_root(),
                slashings: state.slashings.tree_hash_root(),
                previous_epoch_attestations: state.previous_epoch_attestations.tree_hash_root(),
                current_epoch_attestations: state.current_epoch_attestations.tree_hash_root(),
                justification_bits: state.justification_bits.tree_hash_root(),
                previous_justified_checkpoint: state.previous_justified_checkpoint.tree_hash_root(),
                current_justified_checkpoint: state.current_justified_checkpoint.tree_hash_root(),
                finalized_checkpoint: state.finalized_checkpoint.tree_hash_root(),
            }),
            BeaconState::Altair(state) => BeaconState2::Altair(BeaconState2Altair {
                genesis_time: state.genesis_time,
                genesis_validators_root: state.genesis_validators_root,
                slot: state.slot,
                fork: state.fork,
                latest_block_header: state.latest_block_header.clone(),
                block_roots: state.block_roots.tree_hash_root(),
                state_roots: state.state_roots.clone(),
                historical_roots: state.historical_roots.clone(),
                eth1_data: state.eth1_data.tree_hash_root(),
                eth1_data_votes: state.eth1_data_votes.tree_hash_root(),
                eth1_deposit_index: state.eth1_deposit_index,
                validators: state.validators.tree_hash_root(),
                balances: state.balances.tree_hash_root(),
                randao_mixes: state.randao_mixes.tree_hash_root(),
                slashings: state.slashings.tree_hash_root(),
                justification_bits: state.justification_bits.tree_hash_root(),
                previous_justified_checkpoint: state.previous_justified_checkpoint.tree_hash_root(),
                current_justified_checkpoint: state.current_justified_checkpoint.tree_hash_root(),
                finalized_checkpoint: state.finalized_checkpoint.tree_hash_root(),
                current_epoch_participation: state.current_epoch_participation.tree_hash_root(),
                previous_epoch_participation: state.previous_epoch_participation.tree_hash_root(),
                current_sync_committee: state.current_sync_committee.tree_hash_root(),
                next_sync_committee: state.next_sync_committee.tree_hash_root(),
                inactivity_scores: state.inactivity_scores.tree_hash_root(),
            }),
            BeaconState::Bellatrix(state) => BeaconState2::Bellatrix(BeaconState2Bellatrix {
                genesis_time: state.genesis_time,
                genesis_validators_root: state.genesis_validators_root,
                slot: state.slot,
                fork: state.fork,
                latest_block_header: state.latest_block_header.clone(),
                block_roots: state.block_roots.tree_hash_root(),
                state_roots: state.state_roots.clone(),
                historical_roots: state.historical_roots.clone(),
                eth1_data: state.eth1_data.tree_hash_root(),
                eth1_data_votes: state.eth1_data_votes.tree_hash_root(),
                eth1_deposit_index: state.eth1_deposit_index,
                validators: state.validators.tree_hash_root(),
                balances: state.balances.tree_hash_root(),
                randao_mixes: state.randao_mixes.tree_hash_root(),
                slashings: state.slashings.tree_hash_root(),
                justification_bits: state.justification_bits.tree_hash_root(),
                previous_justified_checkpoint: state.previous_justified_checkpoint.tree_hash_root(),
                current_justified_checkpoint: state.current_justified_checkpoint.tree_hash_root(),
                finalized_checkpoint: state.finalized_checkpoint.tree_hash_root(),
                current_epoch_participation: state.current_epoch_participation.tree_hash_root(),
                previous_epoch_participation: state.previous_epoch_participation.tree_hash_root(),
                current_sync_committee: state.current_sync_committee.tree_hash_root(),
                next_sync_committee: state.next_sync_committee.tree_hash_root(),
                inactivity_scores: state.inactivity_scores.tree_hash_root(),
                latest_execution_payload_header: state.latest_execution_payload_header.clone(),
            }),
            BeaconState::Capella(state) => BeaconState2::Capella(BeaconState2Capella {
                genesis_time: state.genesis_time,
                genesis_validators_root: state.genesis_validators_root,
                slot: state.slot,
                fork: state.fork,
                latest_block_header: state.latest_block_header.clone(),
                block_roots: state.block_roots.tree_hash_root(),
                state_roots: state.state_roots.clone(),
                historical_roots: state.historical_roots.clone(),
                eth1_data: state.eth1_data.tree_hash_root(),
                eth1_data_votes: state.eth1_data_votes.tree_hash_root(),
                eth1_deposit_index: state.eth1_deposit_index,
                validators: state.validators.tree_hash_root(),
                balances: state.balances.tree_hash_root(),
                randao_mixes: state.randao_mixes.tree_hash_root(),
                slashings: state.slashings.tree_hash_root(),
                justification_bits: state.justification_bits.tree_hash_root(),
                previous_justified_checkpoint: state.previous_justified_checkpoint.tree_hash_root(),
                current_justified_checkpoint: state.current_justified_checkpoint.tree_hash_root(),
                finalized_checkpoint: state.finalized_checkpoint.tree_hash_root(),
                current_epoch_participation: state.current_epoch_participation.tree_hash_root(),
                previous_epoch_participation: state.previous_epoch_participation.tree_hash_root(),
                current_sync_committee: state.current_sync_committee.tree_hash_root(),
                next_sync_committee: state.next_sync_committee.tree_hash_root(),
                inactivity_scores: state.inactivity_scores.tree_hash_root(),
                latest_execution_payload_header: state.latest_execution_payload_header.clone(),
                next_withdrawal_index: state.next_withdrawal_index,
                next_withdrawal_validator_index: state.next_withdrawal_validator_index,
                historical_summaries: state.historical_summaries.clone(),
            }),
            BeaconState::Deneb(state) => BeaconState2::Deneb(BeaconState2Deneb {
                genesis_time: state.genesis_time,
                genesis_validators_root: state.genesis_validators_root,
                slot: state.slot,
                fork: state.fork,
                latest_block_header: state.latest_block_header.clone(),
                block_roots: state.block_roots.tree_hash_root(),
                state_roots: state.state_roots.clone(),
                historical_roots: state.historical_roots.clone(),
                eth1_data: state.eth1_data.tree_hash_root(),
                eth1_data_votes: state.eth1_data_votes.tree_hash_root(),
                eth1_deposit_index: state.eth1_deposit_index,
                validators: state.validators.tree_hash_root(),
                balances: state.balances.tree_hash_root(),
                randao_mixes: state.randao_mixes.tree_hash_root(),
                slashings: state.slashings.tree_hash_root(),
                justification_bits: state.justification_bits.tree_hash_root(),
                previous_justified_checkpoint: state.previous_justified_checkpoint.tree_hash_root(),
                current_justified_checkpoint: state.current_justified_checkpoint.tree_hash_root(),
                finalized_checkpoint: state.finalized_checkpoint.tree_hash_root(),
                current_epoch_participation: state.current_epoch_participation.tree_hash_root(),
                previous_epoch_participation: state.previous_epoch_participation.tree_hash_root(),
                current_sync_committee: state.current_sync_committee.tree_hash_root(),
                next_sync_committee: state.next_sync_committee.tree_hash_root(),
                inactivity_scores: state.inactivity_scores.tree_hash_root(),
                latest_execution_payload_header: state.latest_execution_payload_header.clone(),
                next_withdrawal_index: state.next_withdrawal_index,
                next_withdrawal_validator_index: state.next_withdrawal_validator_index,
                historical_summaries: state.historical_summaries.clone(),
            }),
            BeaconState::Electra(state) => todo!(),
        }
    }
}
