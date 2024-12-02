#[derive(Serialize, Debug)]
enum NodeType {
    RemoteBranch,
    LocalBranch,
    RemoteTag,
    LocalTag,
}