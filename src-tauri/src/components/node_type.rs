use serde::Serialize;

#[derive(Serialize, Debug)]
enum NodeType {
    RemoteBranch,
    LocalBranch,
    RemoteTag,
    LocalTag,
}
