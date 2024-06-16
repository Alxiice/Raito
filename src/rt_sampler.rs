/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Defines sampler to generate random values.
/// =====================================================

use crate::rt_types::*;

use rand::prelude::*;
use rand::Rng;

pub fn random_float() -> f32 {
    rand::thread_rng().gen()
}

pub fn random_float_range(x: f32, y: f32) -> f32 {
    rand::thread_rng().gen_range(x..y)
}

impl RtVec3 {
    pub fn random() -> RtVec3 {
        RtVec3::new(random_float(), random_float(), random_float())
    }

    pub fn random_range(min: f32, max: f32) -> RtVec3 {
        RtVec3::new(
            random_float_range(min,max), 
            random_float_range(min,max), 
            random_float_range(min,max)
        )
    }
}

pub struct RtSampler {
    seed: u32, 
    nsamples: i32, 
    ndim: i32
}

impl RtSampler {
    pub fn new(seed: u32, nsamples: i32, ndim: i32) -> RtSampler {
        RtSampler { seed, nsamples, ndim }
    }
}

// AiSamplerIterator(sampler, sg)

// // Call once before your sampling loop. 
// // The iterator guarantees a unique sequence of sample points based on the 
// // pixel location, subpixel sample, ray-tree depth, etc. 
// // However, creating an AtSamplerIterator multiple times in the same 
// // rendering state (e.g. multiple times in the same shader_evaluate method) 
// // will produce identical sampling patterns. 
// // The iterator will automatically switch to using a single sample 
// // if invoked "behind" another ray split (such as after a diffuse or glossy ray).

// AiSamplerGetSample (AtSamplerIterator *iterator, float *sample)

// // Get the next sample in an iterator.
// // Call this in a loop to obtain new samples until it returns false.

// AiSamplerGetSampleCount (const AtSamplerIterator *iterator)

// // Get the number of samples taken from this iterator.
// // Call this after the loop is done to see exactly how many samples were taken.

// AiSamplerGetSampleInvCount (const AtSamplerIterator *iterator)

// // Get the inverse sample count.
// // Call this after the loop is done to normalize your integral. 
// // This avoids calling AiSamplerGetSampleCount(), checking for 
// // 0 and inverting the result.
