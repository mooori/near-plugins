#[tokio::test]
async fn test_storage_prefix() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let wasm = workspaces::compile_project("./tests/contracts/access_controllable").await?;
    let contract = worker.dev_deploy(&wasm).await?;

    let res = contract.call("storage_prefix").args_json(()).view().await?;
    let storage_prefix = String::from_utf8(res.json::<Vec<u8>>()?)?;
    assert_eq!(storage_prefix, "__foo");

    Ok(())
}
