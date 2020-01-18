use crate::oscillators::osc_pitch_shift::OscPitchShift;
use crate::oscillators::osc_waveform::OscWaveform;
use num_traits::Float;

#[derive(Debug)]
pub struct PitchedOscillator<F: Float> {
  sample_rate: F,
  waveform: OscWaveform<F>,
  pitch_freq: F,
  pitch_shift: OscPitchShift<F>,
  pitch_multiplier: F,
  modulo: F,
  phase_inc: F,
  amplitude: F,
  amp_mod: F,
}

// TODO follow an invalidation strategy for setters
impl<F> PitchedOscillator<F>
where
  F: Float,
{
  pub fn new(sample_rate: F, waveform: OscWaveform<F>, pitch_freq: F) -> Self {
    let pitch_shift = OscPitchShift::default();
    let pitch_multiplier = pitch_shift.multiplier();
    let modulo = waveform.initial_modulo();

    let mut pitched_oscillator = Self {
      sample_rate,
      waveform,
      pitch_freq,
      pitch_shift,
      pitch_multiplier,
      modulo,
      phase_inc: F::zero(),
      amplitude: F::one(),
      amp_mod: F::zero(),
    };
    pitched_oscillator.update_phase_inc();
    pitched_oscillator
  }

  /// Set the sample rate
  pub fn set_sample_rate(&mut self, sample_rate: F) {
    self.sample_rate = sample_rate;
  }

  /// Set the waveform
  pub fn set_waveform(&mut self, waveform: OscWaveform<F>) {
    self.waveform = waveform;
    self.update_phase_inc();
    self.modulo = self.waveform.initial_modulo();
  }

  /// Set the pitch frequency
  pub fn set_pitch_frequency(&mut self, pitch_freq: F) {
    self.pitch_freq = pitch_freq;
    self.update_phase_inc();
  }

  /// Set the shift for the octaves
  pub fn set_octaves(&mut self, octaves: F) {
    self.pitch_shift.set_octaves(octaves);
    self.pitch_multiplier = self.pitch_shift.multiplier();
    self.update_phase_inc();
  }

  /// Set the semitones shift
  pub fn set_semitones(&mut self, semitones: F) {
    self.pitch_shift.set_semitones(semitones);
    self.pitch_multiplier = self.pitch_shift.multiplier();
    self.update_phase_inc();
  }

  /// Set the shift for the cents
  pub fn set_cents(&mut self, cents: F) {
    self.pitch_shift.set_cents(cents);
    self.pitch_multiplier = self.pitch_shift.multiplier();
    self.update_phase_inc();
  }

  /// Set the pitch bend
  pub fn set_pitch_bend(&mut self, pitch_bend: F) {
    self.pitch_shift.set_pitch_bend(pitch_bend);
    self.pitch_multiplier = self.pitch_shift.multiplier();
    self.update_phase_inc();
  }

  /// Set the frequency modulation
  pub fn set_frequency_modulation(&mut self, freq_mod: F) {
    self.pitch_shift.set_modulation(freq_mod);
    self.pitch_multiplier = self.pitch_shift.multiplier();
    self.update_phase_inc();
  }

  /// Set amplitude
  pub fn set_amplitude(&mut self, amplitude: F) {
    self.amplitude = amplitude;
  }

  /// Set amplitude modulation
  pub fn set_amplitude_modulation(&mut self, amp_mod: F) {
    self.amp_mod = amp_mod;
  }

  /// Generate the next value
  pub fn generate(&mut self) -> F {
//    println!("{:?} {:?}", self.modulo.to_f32().unwrap(), self.phase_inc.to_f32().unwrap());
    let wf = self.waveform.generate(self.modulo, self.phase_inc);
    self.update_modulo();
    wf * self.amplitude + self.amp_mod
  }

  fn update_phase_inc(&mut self) {
    let freq = self.pitch_freq * self.pitch_multiplier;
    self.phase_inc = freq / self.sample_rate;
  }

  fn update_modulo(&mut self) {
    self.modulo = self.modulo + self.phase_inc;
    if self.modulo < F::zero() {
      self.modulo = self.modulo + F::one();
    } else if self.modulo >= F::one() {
      self.modulo = self.modulo - F::one();
    }
  }
}
