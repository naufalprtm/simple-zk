impl pallet_simple_zk::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Hashing = BlakeTwo256;
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        // ... other pallets
        SimpleZK: pallet_simple_zk,
    }
);