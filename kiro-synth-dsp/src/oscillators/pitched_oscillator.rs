use crate::float::Float;
use crate::oscillators::clamp_modulo;
use crate::oscillators::osc_pitch_shift::OscPitchShift;
use crate::oscillators::osc_waveform::OscWaveform;

#[derive(Debug)]
pub struct PitchedOscillator<F: Float> {
  waveform: OscWaveform<F>,
  pitch_freq: F,
  pitch_shift: OscPitchShift<F>,
  amplitude: F,
  amp_mod: F,

  modulo: F,
  phase_inc: F,
  phase_inc_invalidated: bool,
  inv_sample_rate: F,
}

impl<F: Float> PitchedOscillator<F> {
  // FIXME do not require the waveform (and maybe the pitch_freq) in the constructor
  pub fn new(sample_rate: F, waveform: OscWaveform<F>, pitch_freq: F) -> Self {
    let pitch_shift = OscPitchShift::default();
    let modulo = waveform.initial_modulo();

    PitchedOscillator {
      waveform,
      pitch_freq,
      pitch_shift,
      amplitude: F::one(),
      amp_mod: F::zero(),

      modulo,
      phase_inc: F::zero(),
      phase_inc_invalidated: true,
      inv_sample_rate: sample_rate.recip(),
    }
  }

  /// Set the waveform
  pub fn set_waveform(&mut self, waveform: OscWaveform<F>) {
    self.waveform = waveform;
    self.modulo = self.waveform.initial_modulo();
    // FIXME figure out how to avoid clips after changing the waveform and the module
    // self.phase_inc_invalidated = true; // TODO really necessary ???
  }

  /// Set the pitch frequency
  pub fn set_pitch_frequency(&mut self, pitch_freq: F) {
    self.pitch_freq = pitch_freq;
    self.phase_inc_invalidated = true;
  }

  /// Get the pitch frequency
  pub fn get_pitch_frequency(&self) -> F {
    self.pitch_freq
  }

  /// Set the shift for the octaves
  pub fn set_octaves(&mut self, octaves: F) {
    self.pitch_shift.set_octaves(octaves);
    self.phase_inc_invalidated = true;
  }

  /// Get the shift for the octaves
  pub fn get_octaves(&self) -> F {
    self.pitch_shift.get_octaves()
  }

  /// Set the semitones shift
  pub fn set_semitones(&mut self, semitones: F) {
    self.pitch_shift.set_semitones(semitones);
    self.phase_inc_invalidated = true;
  }

  /// Get the semitones shift
  pub fn get_semitones(&self) -> F {
    self.pitch_shift.get_semitones()
  }

  /// Set the shift for the cents
  pub fn set_cents(&mut self, cents: F) {
    self.pitch_shift.set_cents(cents);
    self.phase_inc_invalidated = true;
  }

  /// Get the shift for the cents
  pub fn get_cents(&self) -> F {
    self.pitch_shift.get_cents()
  }

  /// Set the pitch bend
  pub fn set_pitch_bend(&mut self, pitch_bend: F) {
    self.pitch_shift.set_pitch_bend(pitch_bend);
    self.phase_inc_invalidated = true;
  }

  /// Get the pitch bend
  pub fn get_pitch_bend(&self) -> F {
    self.pitch_shift.get_pitch_bend()
  }

  /// Set the frequency modulation
  pub fn set_frequency_modulation(&mut self, freq_mod: F) {
    self.pitch_shift.set_modulation(freq_mod);
    self.phase_inc_invalidated = true;
  }

  /// Get the frequency modulation
  pub fn get_frequency_modulation(&self) -> F {
    self.pitch_shift.get_modulation()
  }

  /// Set amplitude
  pub fn set_amplitude(&mut self, amplitude: F) {
    self.amplitude = amplitude;
  }

  /// Get amplitude
  pub fn get_amplitude(&self) -> F {
    self.amplitude
  }

  /// Set amplitude modulation
  pub fn set_amplitude_modulation(&mut self, amp_mod: F) {
    self.amp_mod = amp_mod;
  }

  /// Get amplitude modulation
  pub fn get_amplitude_modulation(&self) -> F {
    self.amp_mod
  }

  /// Set the sample rate
  pub fn set_sample_rate(&mut self, sample_rate: F) {
    self.inv_sample_rate = sample_rate.recip();
    self.phase_inc_invalidated = true;
  }

  // Reset the oscillator
  pub fn reset(&mut self) {
    self.modulo = self.waveform.initial_modulo();
  }

  /// Generate the next value
  pub fn generate(&mut self) -> F {
    if self.phase_inc_invalidated {
      self.update_phase_inc();
    }

    let signal = self.waveform.generate(self.modulo, self.phase_inc);
    self.modulo = clamp_modulo(self.modulo + self.phase_inc);
    signal * (self.amplitude + self.amp_mod)
  }

  fn update_phase_inc(&mut self) {
    let freq = self.pitch_freq * self.pitch_shift.multiplier();
    self.phase_inc = freq * self.inv_sample_rate;
  }
}
