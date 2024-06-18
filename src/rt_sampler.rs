/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Defines sampler to generate random values.
/// =====================================================

use crate::rt_types::*;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;


fn random_float() -> f32 {
    rand::thread_rng().gen()
}

fn random_float_range(x: f32, y: f32) -> f32 {
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


// ========================================
//  Sampler
// ========================================

pub struct RtSampler {
    // parameters
    seed: u64, 
    nsamples: u8, 
    ndim: u8,
}

impl RtSampler {
    pub fn new(seed: u64, nsamples: u8, ndim: u8) -> RtSampler {
        RtSampler { 
            seed, 
            nsamples, 
            ndim,
        }
    }
}


// ========================================
//  Sampler Iterator
// ========================================

pub struct RtSampleIterator<'a> {
    stop: bool,
    sampler: &'a RtSampler,

    // To iterate on
    rng: ChaCha8Rng,
    // Current state
    taken: u8
}

// From AiSamplerIterator :
// 
// Call once before your sampling loop. 
// The iterator guarantees a unique sequence of sample points based on the 
// pixel location, subpixel sample, ray-tree depth, etc. 
// However, creating an AtSamplerIterator multiple times in the same 
// rendering state (e.g. multiple times in the same shader_evaluate method) 
// will produce identical sampling patterns. 
// The iterator will automatically switch to using a single sample 
// if invoked "behind" another ray split (such as after a diffuse or glossy ray).
impl<'a> RtSampleIterator<'a> {
    pub fn new(sampler: &'a RtSampler) -> Self {
        Self { 
            stop: false, 
            sampler,  
            rng: ChaCha8Rng::seed_from_u64(sampler.seed), 
            taken: 0 }
    }

    fn take(&mut self) -> Vec<f32> {
        self.taken += 1;
        let res = vec![0.0; usize::from(self.sampler.ndim)];
        res.into_iter().map(|_| self.rng.gen()).collect()
    }
}

impl<'a> Iterator for RtSampleIterator<'a> {
    type Item = Vec<f32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop {
            return None
        }

        let sample = self.take();

        // Can we keep generating ?
        if self.taken >= self.sampler.nsamples {
            self.stop = true;
        }

        Some(sample)
    }
}


// ========================================
//  Utility functions
// ========================================

// from AiSamplerGetSample
//
// Get the next sample in an iterator.
// Call this in a loop to obtain new samples until it returns false.
pub fn RtGetSample(it: &mut RtSampleIterator<'_>, sample: &mut Vec<f32>) -> bool {
    let mut x: usize = 0;
    let res = (*it).next();
    if res.is_some() {
        for el in res.unwrap() {
            sample[x] = el;
            x += 1;
        }
        return true;
    } else {
        return false;
    }
}

// from AiSamplerGetSampleCount
//
// Get the number of samples taken from this iterator.
// Call this after the loop is done to see exactly how many samples were taken.
pub fn RtGetSampleCount(it: &RtSampleIterator<'_>) -> u8 {
    it.taken 
    // * it.sampler.nsamples
}

// from AiSamplerGetSampleInvCount
// 
// Get the inverse sample count.
// Call this after the loop is done to normalize your integral. 
// This avoids calling AiSamplerGetSampleCount(), checking for 
// 0 and inverting the result.
pub fn RtSamplerGetSampleInvCount(it: &RtSampleIterator<'_>) -> f32 {
    let nb_samples = RtGetSampleCount(it) as f32;
    if nb_samples == 0.0 { 1.0 } else { 1.0 / nb_samples }
}
