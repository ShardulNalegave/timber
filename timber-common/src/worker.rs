
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerIntroduction {
  pub kind: WorkerKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkerKind {
  Shipper,
  Transformer,
  Archiver,
  Ranger,
}