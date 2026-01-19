pragma circom 2.1.5;

include "circomlib/poseidon.circom";

template HashLeftRight() {
    signal input left;
    signal input right;
    signal output out;

    component hasher = Poseidon(2);
    hasher.inputs[0] <== left;
    hasher.inputs[1] <== right;
    out <== hasher.out;
}

template MerkleProof(levels) {
    signal input leaf;
    signal input pathElements[levels];
    signal input pathIndex[levels]; // pathIndex = 0 (left), 1 (right)
    signal output root;

    component hashers[levels];
    signal hashes[levels + 1];
    hashes[0] <== leaf;

    for (var i = 0; i < levels; i++) {
        hashers[i] = HashLeftRight();

        // Conditional left/right based on pathIndex
        hashers[i].left <== (hashes[i] - pathElements[i]) * pathIndex[i] + pathElements[i] * (1 - pathIndex[i]);
        hashers[i].right <== (pathElements[i] - hashes[i]) * pathIndex[i] + hashes[i] * (1 - pathIndex[i]);

        hashes[i + 1] <== hashers[i].out;
    }

    root <== hashes[levels];
}

component main { public [root] } = MerkleProof(20);
