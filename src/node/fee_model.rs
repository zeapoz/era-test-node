use std::fmt::Debug;
use zksync_node_fee_model::BatchFeeModelInputProvider;
use zksync_types::fee_model::{FeeModelConfigV2, FeeParams, FeeParamsV2};
use zksync_types::L1_GAS_PER_PUBDATA_BYTE;

use super::{DEFAULT_L2_GAS_PRICE, L1_GAS_PRICE};

#[derive(Debug, Clone, PartialEq)]
pub struct TestNodeFeeInputProvider {
    pub l1_gas_price: u64,
    pub l1_pubdata_price: u64,
    pub l2_gas_price: u64,
    pub compute_overhead_part: f64,
    pub pubdata_overhead_part: f64,
    pub batch_overhead_l1_gas: u64,
    pub max_gas_per_batch: u64,
    pub max_pubdata_per_batch: u64,
}

impl TestNodeFeeInputProvider {
    pub fn get_fee_model_config(&self) -> FeeModelConfigV2 {
        FeeModelConfigV2 {
            minimal_l2_gas_price: self.l2_gas_price,
            compute_overhead_part: self.compute_overhead_part,
            pubdata_overhead_part: self.pubdata_overhead_part,
            batch_overhead_l1_gas: self.batch_overhead_l1_gas,
            max_gas_per_batch: self.max_gas_per_batch,
            max_pubdata_per_batch: self.max_pubdata_per_batch,
        }
    }
}

impl BatchFeeModelInputProvider for TestNodeFeeInputProvider {
    fn get_fee_model_params(&self) -> FeeParams {
        // TODO: consider using old fee model for the olds blocks, when forking
        FeeParams::V2(FeeParamsV2 {
            config: self.get_fee_model_config(),
            l1_gas_price: self.l1_gas_price,
            l1_pubdata_price: self.l1_pubdata_price,
        })
    }
}

impl From<FeeParams> for TestNodeFeeInputProvider {
    fn from(value: FeeParams) -> Self {
        match value {
            FeeParams::V1(_) => todo!(),
            FeeParams::V2(value) => Self {
                l1_gas_price: value.l1_gas_price,
                l1_pubdata_price: value.l1_pubdata_price,
                l2_gas_price: value.config.minimal_l2_gas_price,
                compute_overhead_part: value.config.compute_overhead_part,
                pubdata_overhead_part: value.config.pubdata_overhead_part,
                batch_overhead_l1_gas: value.config.batch_overhead_l1_gas,
                max_gas_per_batch: value.config.max_gas_per_batch,
                max_pubdata_per_batch: value.config.max_pubdata_per_batch,
            },
        }
    }
}

impl Default for TestNodeFeeInputProvider {
    fn default() -> Self {
        Self {
            l1_gas_price: L1_GAS_PRICE,
            l1_pubdata_price: L1_GAS_PRICE * L1_GAS_PER_PUBDATA_BYTE as u64,
            l2_gas_price: DEFAULT_L2_GAS_PRICE,
            compute_overhead_part: 0.0,
            pubdata_overhead_part: 1.0,
            batch_overhead_l1_gas: 800000,
            max_gas_per_batch: 200000000,
            max_pubdata_per_batch: 100000,
        }
    }
}
