use crate::window::Window;
use crate::window;
use crate::signal::Signal;
use crate::spectrum::Spectrum;
use ::safer_ffi::prelude::*;
use crate::fft::ForwardFFT;

/// Struct used to derive a ForwardFFTHandle in C.
#[derive_ReprC]
#[ReprC::opaque]
pub struct ForwardFFTHandle {
    fwd: ForwardFFT,
}
/// Handle used to derive a SignalHandle in C
#[derive_ReprC]
#[ReprC::opaque]
pub struct SignalHandle {
    fwd: Signal,
}

/// Handle to keep a spectrum handle.
#[derive_ReprC]
#[ReprC::opaque]
pub struct SpectrumHandle {
    fwd: Spectrum,
}

#[derive_ReprC]
#[ReprC::opaque]
pub struct WindowHandle {
    fwd: Window
}


/// Get a new forward FFT handle
#[ffi_export]
pub extern fn forward_fft_handle_new(sample_size: u64) -> repr_c::Box<ForwardFFTHandle>
{
    repr_c::Box::new(ForwardFFTHandle { fwd: ForwardFFT::new(sample_size as usize) })
}
/// Free the handle
#[ffi_export]
pub extern fn forward_fft_handle_free(p: Option<repr_c::Box<ForwardFFTHandle>>)
{
    drop(p)
}
/// Calculate a real-valued FFT and return the values on 
#[ffi_export]
fn dsprs_forward_fft_process_real(
    fft: Option<&mut ForwardFFTHandle>,
    input: Option<c_slice::Ref<'_, f32>>,
    output: Option<c_slice::Mut<'_,f32>>
) -> bool
{
    if let Some(resolved_fft) = fft {
        if let Some((i,o)) = input.zip(output) {
            if i.len() == o.len() {
                let result = resolved_fft.fwd.process_real(&i);
                assert_eq!(o.len(), result.as_slice().len());
                o.as_slice().copy_from_slice(result.as_slice());
                return true;
            }
        }
    }
    return false;
}
// Signal functions
/// New signal from data
/// Params:
/// * data: a struct containing a pointer to floats, with their length.
/// * sample_rate: the sample rate of the sound sent.
#[ffi_export]
pub extern fn dsprs_signal_new(data: Option<c_slice::Ref<'_, f32>>, sample_rate: usize) -> Option<repr_c::Box::<SignalHandle>> {
    match data {
        Some(data) => Some(repr_c::Box::new(SignalHandle { fwd: Signal::new(Vec::from(data.as_slice()), sample_rate) })),
        None => None
    }
}
/// Create an empty signal
/// Params:
/// * sample_rate: the sample rate of the signal
#[ffi_export]
pub extern fn dsprs_signal_empty(sample_rate: u64) -> repr_c::Box::<SignalHandle> {
    repr_c::Box::new(SignalHandle { fwd: Signal::empty(sample_rate as usize) })
}

/// Free a signal
/// Params:
/// * to_free: Ptr from Rust with a signal.
#[ffi_export]
pub extern fn dsprs_signal_free(to_free: Option<repr_c::Box::<SignalHandle>>) {
    drop(to_free);
}
/// Re-scale a signal 
/// * signal: ptr to created signal
/// * amount: float to scale the signal to.
#[ffi_export]
pub unsafe fn dsprs_signal_rescale(signal: Option<&SignalHandle>, amount: f32) -> Option<repr_c::Box::<SignalHandle>> {
    match signal{
        None => None,
        Some(valid_signal) => Some(repr_c::Box::new(SignalHandle{ fwd: Signal::rescale(&valid_signal.fwd, amount)}))
    }
}

// Forward FFT functions 
/// Create a new Forward FFT planner
/// Params:
/// * fft_size: size of the FFT to plan.
#[ffi_export]
pub extern fn dsprs_forward_fft_new(fft_size: u64) -> repr_c::Box::<ForwardFFTHandle> {
    repr_c::Box::new(ForwardFFTHandle {fwd: ForwardFFT::new(fft_size as usize)})
}

/// Process an FFT with a signal
/// Params:
/// * fft: Ptr to previously created Forward FFT planer
/// * signal: Ptr to previously created signal
#[ffi_export]
pub extern fn dsprs_forward_fft_process(fft: Option<& mut ForwardFFTHandle>, signal: Option<&SignalHandle>) -> Option<repr_c::Box::<SpectrumHandle>> {
    match fft.zip(signal) {
        Some((resolved_fft, resolved_signal)) => Some(repr_c::Box::new(SpectrumHandle { fwd: resolved_fft.fwd.process(&resolved_signal.fwd)})),
        None => None,
    }
}
/// Window Functions

/// Create a rectangular window handle
#[ffi_export]
pub extern fn dsprs_rectangular_window(width: u64, offset: u64, window_length: u64) -> repr_c::Box::<WindowHandle> {
    repr_c::Box::new(WindowHandle {fwd: window::rectangular(width as usize, offset as usize, window_length as usize)})
}
/// Create a triangular window handle
#[ffi_export]
pub extern fn dsprs_triangular_window(width: u64, offset: u64, window_length: u64) -> repr_c::Box::<WindowHandle> {
    repr_c::Box::new(WindowHandle {fwd: window::triangular(width as usize, offset as usize, window_length as usize)})
}
/// Create a welch window handle
#[ffi_export]
pub extern fn dsprs_welch_window(width: u64, offset: u64, window_length: u64) -> repr_c::Box::<WindowHandle> {
    repr_c::Box::new(WindowHandle {fwd: window::welch(width as usize, offset as usize, window_length as usize)})
}
/// Create a sine window handle.
#[ffi_export]
pub extern fn dsprs_sine_window(width: u64, offset: u64, window_length: u64) -> repr_c::Box::<WindowHandle> {
    repr_c::Box::new(WindowHandle {fwd: window::sine(width as usize, offset as usize, window_length as usize)})
}
/// Create a hann window handle
#[ffi_export]
pub extern fn dsprs_hann_window(width: u64, offset: u64, window_length: u64) -> repr_c::Box::<WindowHandle> {
    repr_c::Box::new(WindowHandle {fwd: window::hann(width as usize, offset as usize, window_length as usize)})
}
/// Create a hamming window handle
#[ffi_export]
pub extern fn dsprs_hamming_window(width: u64, offset: u64, window_length: u64) -> repr_c::Box::<WindowHandle> {
    repr_c::Box::new(WindowHandle {fwd: window::hamming(width as usize, offset as usize, window_length as usize)})
}
/// Create a blackman window handle
#[ffi_export]
pub extern fn dsprs_blackman_window(width: u64, offset: u64, window_length: u64) -> repr_c::Box::<WindowHandle> {
    repr_c::Box::new(WindowHandle {fwd: window::blackman(width as usize, offset as usize, window_length as usize)})
}
/// Check for Window length.
/// If it's null, returns -1, otherwise the window size.
#[ffi_export]
pub extern fn dsprs_window_len(window: Option<&WindowHandle>) -> i64{
    if let Some(w) = window {
        return w.fwd.len() as i64;
    }
    return -1;
}
/// Apply the window to the sample data
#[ffi_export]
pub extern fn dsprs_window_apply(window:Option<&WindowHandle>, input :Option<c_slice::Ref<'_, f32>>, output: Option<c_slice::Mut<'_, f32>>) -> bool {
    if let Some(w) = window {
        match input.zip(output){
            Some((i, o)) => {
                if i.len() == w.fwd.len(){
                    w.fwd.apply(&i,o.as_slice());
                    return true;
                }
                return false;
            },
            None => false
        }
    } else {
        false
    }
    
}

// Spectrum functions

/// Get the frequency of the fft component at index
/// Params:
/// * spectrum: ptr to the already created spectrum. 
/// * index: Index of the FFT. Keep in mind that the calculation is ciclycal.
/// Returns:
/// Frequency in Hertz as a float if spectrum is not null, otherwise -1.0 
#[ffi_export]
pub extern fn dsprs_spectrum_item_freq(spectrum: Option<&SpectrumHandle>, index: u64) -> f32{
    if let Some(s) = spectrum {
        s.fwd.item_freq(index as usize)
    } else {
        -1.0
    }
}
/// Returns the maximum frequency in the spectrum
/// Params:
/// * spectrum: ptr to the already created spectrum. 
/// Returns:
/// Max freq in Hertz as a float if spectrum is not null, otherwise -1.0 
#[ffi_export]
pub extern fn dsprs_spectrum_max_freq(spectrum: Option<&SpectrumHandle>) -> f32 {
    if let Some(s) = spectrum {
        s.fwd.max_freq()
    } else {
        -1.0
    }
}

/// Returns the length of the spectrum data
/// Params:
/// * spectrum: ptr to the already created spectrum. 
/// Returns:
/// length of data (truncated to i64) if spectrum is not null, otherwise -1.0 
#[ffi_export]
pub extern fn dsprs_spectrum_len(handle:Option<&SpectrumHandle>) -> i64 {
    if let Some(spectrum) = handle {
        spectrum.fwd.len() as i64
    } else {
        -1
    }
}

/// Returns the spectrum into a real valued array.
/// Params:
/// * spectrum: ptr to the already created spectrum. 
/// * output: ptr to slice struct 
/// Returns:
/// true if copy was successful, false otherwise
#[ffi_export]
pub extern fn dsprs_spectrum_to_real(handle: Option<&SpectrumHandle>, output: Option<c_slice::Mut<'_, f32>>) -> bool {
    if let Some(spectrum) = handle {
        let size = spectrum.fwd.len();
        if let Some(out) = output {
            if out.len() == size {
                // Copy the values from the output into the raw array.
                let fft_result = spectrum.fwd.to_real();
                out.as_slice().copy_from_slice(fft_result.as_slice());
                return true;
            }
        }
    }
    return false;
}

#[::safer_ffi::cfg_headers]
#[test]
fn generate_headers() -> ::std::io::Result<()>
{
    ::safer_ffi::headers::builder()
        .to_file("dsp_rs.h")?
        .generate()
}