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
) -> Option<repr_c::Vec<f32>>
{
    match fft.zip(input){
        Some((resolved_fft, resolved_input)) => Some(repr_c::Vec::from(resolved_fft.fwd.process_real(&resolved_input))),
        None => None
    }
}
// Signal functions
#[ffi_export]
pub extern fn dsprs_signal_new(data: Option<c_slice::Ref<'_, f32>>, sample_rate: usize) -> Option<repr_c::Box::<SignalHandle>> {
    match data {
        Some(data) => Some(repr_c::Box::new(SignalHandle { fwd: Signal::new(Vec::from(data.as_slice()), sample_rate) })),
        None => None
    }
}

#[ffi_export]
pub extern fn dsprs_signal_empty(sample_rate: u64) -> repr_c::Box::<SignalHandle> {
    repr_c::Box::new(SignalHandle { fwd: Signal::empty(sample_rate as usize) })
}


#[ffi_export]
pub extern fn dsprs_signal_free(to_free: Option<repr_c::Box::<SignalHandle>>) {
    drop(to_free);
}

#[ffi_export]
pub unsafe fn dsprs_signal_rescale(signal: Option<&SignalHandle>, amount: f32) -> Option<repr_c::Box::<SignalHandle>> {
    match signal{
        None => None,
        Some(valid_signal) => Some(repr_c::Box::new(SignalHandle{ fwd: Signal::rescale(&valid_signal.fwd, amount)}))
    }
}

// Forward FFT functions 

#[ffi_export]
pub extern fn dsprs_forward_fft_new(sample_size: u64) -> repr_c::Box::<ForwardFFTHandle> {
    repr_c::Box::new(ForwardFFTHandle {fwd: ForwardFFT::new(sample_size as usize)})
}


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


#[::safer_ffi::cfg_headers]
#[test]
fn generate_headers() -> ::std::io::Result<()>
{
    ::safer_ffi::headers::builder()
        .to_file("dsp_rs.h")?
        .generate()
}