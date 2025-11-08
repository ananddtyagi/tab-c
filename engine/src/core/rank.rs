use crate::ipc::protocol::CompleteRequest;

pub fn rerank(_request: &CompleteRequest, candidates: &mut [String]) {
    candidates.sort_by(|a, b| a.len().cmp(&b.len()));
}
