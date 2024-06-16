
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
  pub kafka_brokers: String,
}

impl Default for Metadata {
  fn default() -> Self {
    Self { kafka_brokers: "localhost:9092".to_string() }
  }
}