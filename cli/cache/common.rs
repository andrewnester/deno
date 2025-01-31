// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.

use std::hash::Hasher;

/// A very fast insecure hasher that uses the xxHash algorithm.
#[derive(Default)]
pub struct FastInsecureHasher(twox_hash::XxHash64);

impl FastInsecureHasher {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn write_str(&mut self, text: &str) -> &mut Self {
    self.write(text.as_bytes());
    self
  }

  pub fn write(&mut self, bytes: &[u8]) -> &mut Self {
    self.0.write(bytes);
    self
  }

  pub fn write_u8(&mut self, value: u8) -> &mut Self {
    self.0.write_u8(value);
    self
  }

  pub fn write_u64(&mut self, value: u64) -> &mut Self {
    self.0.write_u64(value);
    self
  }

  pub fn write_hashable(
    &mut self,
    hashable: &impl std::hash::Hash,
  ) -> &mut Self {
    hashable.hash(&mut self.0);
    self
  }

  pub fn finish(&self) -> u64 {
    self.0.finish()
  }
}

/// Disable write-ahead-logging and tweak some other stuff.
/// We want to favor startup time over cache performance and
/// creating a WAL is expensive on startup.
pub static INITIAL_PRAGMAS: &str = "
  PRAGMA journal_mode=OFF;
  PRAGMA synchronous=NORMAL;
  PRAGMA temp_store=memory;
  PRAGMA page_size=4096;
  PRAGMA mmap_size=6000000;
  PRAGMA optimize;
";
