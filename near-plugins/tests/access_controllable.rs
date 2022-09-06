use near_sdk::serde_json::json;

#[tokio::test]
async fn test_acl_storage_prefix() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let wasm = workspaces::compile_project("./tests/contracts/access_controllable").await?;
    let contract = worker.dev_deploy(&wasm).await?;

    let res = contract
        .call("acl_storage_prefix")
        .args_json(())
        .view()
        .await?;
    let storage_prefix = String::from_utf8(res.json::<Vec<u8>>()?)?;
    assert_eq!(storage_prefix, "__foo");

    Ok(())
}

#[tokio::test]
async fn test_acl_has_role() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let wasm = workspaces::compile_project("./tests/contracts/access_controllable").await?;
    let contract = worker.dev_deploy(&wasm).await?;
    let alice = worker.dev_create_account().await?;

    let res = contract
        .call("acl_has_role")
        .args_json(json!({
            "role": 0u8,
            "account_id": alice.id(),
        }))
        .view()
        .await?;
    assert_eq!(res.json::<bool>()?, false);

    Ok(())
}
