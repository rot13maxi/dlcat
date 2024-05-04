use bitcoin::opcodes::all::{OP_CHECKSIGVERIFY, OP_NOP4};
use bitcoin::{
    taproot::{TaprootBuilder, TaprootSpendInfo},
    Address, Network, ScriptBuf,
};
use dlc::secp256k1_zkp::{Secp256k1, XOnlyPublicKey};
use dlc_messages::contract_msgs::ContractDescriptor;

pub fn build_taproot_leafs(outcome: ContractDescriptor, key: XOnlyPublicKey) -> TaprootSpendInfo {
    let secp = Secp256k1::new();
    let mut builder = TaprootBuilder::new();
    match outcome {
        ContractDescriptor::EnumeratedContractDescriptor(enum_descriptor) => {
            for (index, _) in enum_descriptor.payouts.iter().enumerate() {
                let depth = (index / 2) as u8;
                let mut script = ScriptBuf::new();
                script.push_slice([]); //todo calculate this
                script.push_opcode(OP_NOP4);
                script.push_slice([]); //todo calculate this
                script.push_opcode(OP_CHECKSIGVERIFY);

                builder = builder.add_leaf(depth, script).unwrap();
            }
        }
        ContractDescriptor::NumericOutcomeContractDescriptor(_) => unimplemented!("not yet"),
    }

    builder.finalize(&secp, key).unwrap()
}

/// The collateral address for both parties to deposit to.
///
/// If they do not producs the same output key when building
/// the taproot tree, then the contract should not be funded.
#[allow(dead_code)]
fn build_collateral_address(info: TaprootSpendInfo) -> Address {
    let hash_match = info.output_key();
    Address::p2tr_tweaked(hash_match, Network::Regtest)
}
