/*! \file */
/*******************************************
 *                                         *
 *  File auto-generated by `::safer_ffi`.  *
 *                                         *
 *  Do not manually edit this file.        *
 *                                         *
 *******************************************/

#ifndef __RUST_DSP__
#define __RUST_DSP__

#ifdef __cplusplus
extern "C" {
#endif


#include <stddef.h>
#include <stdint.h>

typedef struct ForwardFFTHandle ForwardFFTHandle_t;

/** \brief
 *  Get a new forward FFT handle
 */
ForwardFFTHandle_t * forward_fft_handle_new (
    uint64_t sample_size);

/** \brief
 *  Free the handle
 */
void forward_fft_handle_free (
    ForwardFFTHandle_t * p);

/** \brief
 *  `&'lt [T]` but with a guaranteed `#[repr(C)]` layout.
 * 
 *  # C layout (for some given type T)
 * 
 *  ```c
 *  typedef struct {
 *      // Cannot be NULL
 *      T * ptr;
 *      size_t len;
 *  } slice_T;
 *  ```
 * 
 *  # Nullable pointer?
 * 
 *  If you want to support the above typedef, but where the `ptr` field is
 *  allowed to be `NULL` (with the contents of `len` then being undefined)
 *  use the `Option< slice_ptr<_> >` type.
 */
typedef struct {

    float const * ptr;

    size_t len;

} slice_ref_float_t;

/** \brief
 *  `&'lt mut [T]` but with a guaranteed `#[repr(C)]` layout.
 * 
 *  # C layout (for some given type T)
 * 
 *  ```c
 *  typedef struct {
 *      // Cannot be NULL
 *      T * ptr;
 *      size_t len;
 *  } slice_T;
 *  ```
 * 
 *  # Nullable pointer?
 * 
 *  If you want to support the above typedef, but where the `ptr` field is
 *  allowed to be `NULL` (with the contents of `len` then being undefined)
 *  use the `Option< slice_ptr<_> >` type.
 */
typedef struct {

    float * ptr;

    size_t len;

} slice_mut_float_t;


#include <stdbool.h>

/** \brief
 *  Calculate a real-valued FFT and return the values on 
 */
bool dsprs_forward_fft_process_real (
    ForwardFFTHandle_t * fft,
    slice_ref_float_t input,
    slice_mut_float_t output);

typedef struct SignalHandle SignalHandle_t;

/** \brief
 *  New signal from data
 *  Params:
 *  * data: a struct containing a pointer to floats, with their length.
 *  * sample_rate: the sample rate of the sound sent.
 */
SignalHandle_t * dsprs_signal_new (
    slice_ref_float_t data,
    size_t sample_rate);

/** \brief
 *  Create an empty signal
 *  Params:
 *  * sample_rate: the sample rate of the signal
 */
SignalHandle_t * dsprs_signal_empty (
    uint64_t sample_rate);

/** \brief
 *  Free a signal
 *  Params:
 *  * to_free: Ptr from Rust with a signal.
 */
void dsprs_signal_free (
    SignalHandle_t * to_free);

/** \brief
 *  Re-scale a signal 
 *  * signal: ptr to created signal
 *  * amount: float to scale the signal to.
 */
SignalHandle_t * dsprs_signal_rescale (
    SignalHandle_t const * signal,
    float amount);

/** \brief
 *  Create a new Forward FFT planner
 *  Params:
 *  * fft_size: size of the FFT to plan.
 */
ForwardFFTHandle_t * dsprs_forward_fft_new (
    uint64_t fft_size);

typedef struct SpectrumHandle SpectrumHandle_t;

/** \brief
 *  Process an FFT with a signal
 *  Params:
 *  * fft: Ptr to previously created Forward FFT planer
 *  * signal: Ptr to previously created signal
 */
SpectrumHandle_t * dsprs_forward_fft_process (
    ForwardFFTHandle_t * fft,
    SignalHandle_t const * signal);

typedef struct WindowHandle WindowHandle_t;

/** \brief
 *  Window Functions
 *  Create a rectangular window handle
 */
WindowHandle_t * dsprs_rectangular_window (
    uint64_t width,
    uint64_t offset,
    uint64_t window_length);

/** \brief
 *  Create a triangular window handle
 */
WindowHandle_t * dsprs_triangular_window (
    uint64_t width,
    uint64_t offset,
    uint64_t window_length);

/** \brief
 *  Create a welch window handle
 */
WindowHandle_t * dsprs_welch_window (
    uint64_t width,
    uint64_t offset,
    uint64_t window_length);

/** \brief
 *  Create a sine window handle.
 */
WindowHandle_t * dsprs_sine_window (
    uint64_t width,
    uint64_t offset,
    uint64_t window_length);

/** \brief
 *  Create a hann window handle
 */
WindowHandle_t * dsprs_hann_window (
    uint64_t width,
    uint64_t offset,
    uint64_t window_length);

/** \brief
 *  Create a hamming window handle
 */
WindowHandle_t * dsprs_hamming_window (
    uint64_t width,
    uint64_t offset,
    uint64_t window_length);

/** \brief
 *  Create a blackman window handle
 */
WindowHandle_t * dsprs_blackman_window (
    uint64_t width,
    uint64_t offset,
    uint64_t window_length);

/** \brief
 *  Check for Window length.
 *  If it's null, returns -1, otherwise the window size.
 */
int64_t dsprs_window_len (
    WindowHandle_t const * window);

/** \brief
 *  Apply the window to the sample data
 */
bool dsprs_window_apply (
    WindowHandle_t const * window,
    slice_ref_float_t input,
    slice_mut_float_t output);

/** \brief
 *  Get the frequency of the fft component at index
 *  Params:
 *  * spectrum: ptr to the already created spectrum. 
 *  * index: Index of the FFT. Keep in mind that the calculation is ciclycal.
 *  Returns:
 *  Frequency in Hertz as a float if spectrum is not null, otherwise -1.0 
 */
float dsprs_spectrum_item_freq (
    SpectrumHandle_t const * spectrum,
    uint64_t index);

/** \brief
 *  Returns the maximum frequency in the spectrum
 *  Params:
 *  * spectrum: ptr to the already created spectrum. 
 *  Returns:
 *  Max freq in Hertz as a float if spectrum is not null, otherwise -1.0 
 */
float dsprs_spectrum_max_freq (
    SpectrumHandle_t const * spectrum);

/** \brief
 *  Returns the length of the spectrum data
 *  Params:
 *  * spectrum: ptr to the already created spectrum. 
 *  Returns:
 *  length of data (truncated to i64) if spectrum is not null, otherwise -1.0 
 */
int64_t dsprs_spectrum_len (
    SpectrumHandle_t const * handle);

/** \brief
 *  Returns the spectrum into a real valued array.
 *  Params:
 *  * spectrum: ptr to the already created spectrum. 
 *  * output: ptr to slice struct 
 *  Returns:
 *  true if copy was successful, false otherwise
 */
bool dsprs_spectrum_to_real (
    SpectrumHandle_t const * handle,
    slice_mut_float_t output);


#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* __RUST_DSP__ */
