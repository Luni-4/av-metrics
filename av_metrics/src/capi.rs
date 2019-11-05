use std::ffi::CStr;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::ptr::null;

use crate::video::psnr::*;
use crate::video::*;

/// ciao ciao
#[no_mangle]
pub unsafe extern "C" fn get_you(ciao: i64) {}

/*#[inline(always)]
fn convert_c_string_into_path(c_buf: *const i8) -> PathBuf {
    let c_str = unsafe { CStr::from_ptr(c_buf) };
    Path::new(c_str.to_str().unwrap()).to_path_buf()
}

/// Simple Context
pub struct AvMetricsContext {
    proof: u64,
}

macro_rules! get_decs {
    ($metrics:ident, $ret:expr, $path1:ident, $path2:ident,
     $dec1: ident, $dec2:ident) => {
        if $path1.is_null() || $path2.is_null() {
            return $ret;
        }

        let path1 = convert_c_string_into_path($path1);
        let path2 = convert_c_string_into_path($path2);

        let mut file1 = File::open(path1).expect(concat!(
            "Error opening the first",
            stringify!($metrics),
            "video"
        ));
        let mut file2 = File::open(path2).expect(concat!(
            "Error opening the second",
            stringify!($metrics),
            "video"
        ));

        let mut $dec1 = y4m::Decoder::new(&mut file1).expect(concat!(
            "Failed to decode the first",
            stringify!($metrics),
            "y4m file"
        ));
        let mut $dec2 = y4m::Decoder::new(&mut file2).expect(concat!(
            "Failed to decode the second",
            stringify!($metrics),
            "y4m file"
        ));
    };
}

macro_rules! return_value {
    ($value:ident, ciede) => {
        if let Ok(ciede) = $value {
            ciede
        } else {
            -1.0
        }
    };
    ($class:ident, $_:ident) => {
        if let Ok(class) = $class {
            let boxed = Box::new(class);
            Box::into_raw(boxed)
        } else {
            null()
        }
    };
}

macro_rules! video_metrics {
    (ciede, f64, $func:ident) => {
        video_metrics!(
            concat!(
                "Calculate the `",
                stringify!($metrics),
                "` between two videos"
            ),
            "Returns the correct `ciede` value or `-1` on errors",
            ciede, ciede, f64, -1.0, $func
        );
    };
    ($metric:ident, $namespace:ident, $struct:ident, $func:ident) => {
        video_metrics!(
            concat!(
                "Calculate the `",
                stringify!($metrics),
                "` between two videos"
            ),
            concat!(
                "Returns either `NULL` or a newly allocated `",
                stringify!($struct),
                "`"
            ),
            $metric, $namespace, $struct, null(), $func, *const
        );
    };
    ($doc:expr, $doc1:expr,
     $metric:ident, $namespace:ident, $struct:ident,
     $ret:expr, $func:ident$(,)? $(*$const:ident)?) => {
        #[doc = $doc]
        #[doc = ""]
        #[doc = $doc1]
        #[no_mangle]
        pub unsafe extern fn $func(
            video1_path: *const i8,
            video2_path: *const i8,
            frame_limit: usize
        ) -> $(*$const)? $struct {

            get_decs!($metric, $ret, video1_path, video2_path, dec1, dec2);

            let mut limit: Option<usize> = None;
            if frame_limit > 0 {
                limit = Some(frame_limit);
            }

            let $metric = $namespace::$func(
                &mut dec1,
                &mut dec2,
                limit
            );

            return_value!($metric, $metric)

        }
    };
}

macro_rules! drop {
    ($metrics:ident, $drop:ident, $struct:ident) => {
           drop!(concat!("Free `", stringify!($struct), "`"),
               $metrics, $drop, $struct
           );
    };
    ($doc:expr, $metrics:ident, $drop:ident, $struct:ident) => {
        #[doc = $doc]
        #[no_mangle]
        pub unsafe extern fn $drop ($metrics: *const $struct) {
            std::mem::drop(Box::from_raw($metrics as *mut $struct));
        }
    };
}

macro_rules! frames {
    ($dec1:ident, $dec2:ident, $frame:ident, $type:ident,
     $metric:ident, $namespace:ident, $func:ident, $ret:expr) => {
        let frame1 = $dec1.read_specific_frame::<$type>($frame);
        let frame2 = $dec2.read_specific_frame::<$type>($frame);
        if let Ok(frame1) = frame1 {
            if let Ok(frame2) = frame2 {
                let $metric = $namespace::$func(&frame1, &frame2);
                return return_value!($metric, $metric);
            }
        }
        return $ret;
    };
}

macro_rules! frame_metrics {
    (ciede, f64, $func:ident) => {
        frame_metrics!(
            concat!(
                "Calculate the `",
                stringify!($metrics),
                "` between two frames"
            ),
            "Returns the correct `ciede` value or `-1` on errors",
            ciede, ciede, f64, -1.0, $func
        );
    };
    ($metric:ident, $namespace:ident, $struct:ident, $func:ident) => {
        frame_metrics!(
            concat!(
                "Calculate the `",
                stringify!($metrics),
                "` between two frames"
            ),
            concat!(
                "Returns either `NULL` or a newly allocated `",
                stringify!($struct),
                "`"
            ),
            $metric, $namespace, $struct, null(), $func, *const
        );
    };
    ($doc:expr, $doc1:expr,
     $metric:ident, $namespace:ident, $struct:ident,
     $ret:expr, $func:ident$(,)? $(*$const:ident)?) => {
        #[doc = $doc]
        #[doc = ""]
        #[doc = $doc1]
        #[no_mangle]
        pub unsafe extern fn $func(
            video1_path: *const i8,
            video2_path: *const i8,
            frame_number: usize
        ) -> $(*$const)? $struct {

            get_decs!($metric, $ret, video1_path, video2_path, dec1, dec2);

            if dec1.get_bit_depth() > 8 {
                frames!(
                    dec1,
                    dec2,
                    frame_number,
                    u16,
                    $metric,
                    $namespace,
                    $func,
                    $ret
                );
            } else {
                frames!(
                    dec1,
                    dec2,
                    frame_number,
                    u8,
                    $metric,
                    $namespace,
                    $func,
                    $ret
                );
            }
        }
    };
}

video_metrics!(psnr, psnr, PsnrResults, calculate_video_psnr);
drop!(psnr, drop_video_psnr, PsnrResults);
video_metrics!(psnr_hvs, psnr_hvs, PlanarMetrics, calculate_video_psnr_hvs);
drop!(psnr_hvs, drop_video_pnsr_hvs, PlanarMetrics);
video_metrics!(ssim, ssim, PlanarMetrics, calculate_video_ssim);
drop!(ssim, drop_video_ssim, PlanarMetrics);
video_metrics!(msssim, ssim, PlanarMetrics, calculate_video_msssim);
drop!(msssim, drop_video_msssim, PlanarMetrics);

frame_metrics!(psnr, psnr, PlanarMetrics, calculate_frame_psnr);
drop!(psnr, drop_frame_psnr, PlanarMetrics);
frame_metrics!(psnr_hvs, psnr_hvs, PlanarMetrics, calculate_frame_psnr_hvs);
drop!(psnr_hvs, drop_frame_psnr_hvs, PlanarMetrics);
frame_metrics!(ssim, ssim, PlanarMetrics, calculate_frame_ssim);
drop!(ssim, drop_frame_ssim, PlanarMetrics);
frame_metrics!(msssim, ssim, PlanarMetrics, calculate_frame_msssim);
drop!(msssim, drop_frame_msssim, PlanarMetrics);*/
