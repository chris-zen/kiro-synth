use kiro_synth_engine::program::ParamBlock;

pub struct EnvGenParams {
  pub attack: ParamBlock,
  pub decay: ParamBlock,
  pub sustain: ParamBlock,
  pub release: ParamBlock,
  pub mode: ParamBlock,
  pub legato: ParamBlock,
  pub reset_to_zero: ParamBlock,
  pub dca_intensity: ParamBlock,
}

pub struct OscParams {
  pub shape: ParamBlock,
  pub amplitude: ParamBlock,
  pub octaves: ParamBlock,
  pub semitones: ParamBlock,
  pub cents: ParamBlock,
}

pub struct FilterParams {
  pub mode: ParamBlock,
  pub freq: ParamBlock,
  pub q: ParamBlock,
}

pub struct DcaParams {
  pub amplitude: ParamBlock,
  pub pan: ParamBlock,
}
