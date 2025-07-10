#[tokio::test]
async fn test_deploy() {
    // Mock signer/provider
    let mock_contract = /* mock Artifact */;
    let txid = deploy(mock_contract, MockSigner, MockProvider).await.unwrap();
    assert!(!txid.is_empty());
}

// Add more: call, verify, stream tests with mocks.
