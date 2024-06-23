use log::{error, info, warn};
/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Simple module where I can test new features
///   easily.
/// =====================================================

use raito::*;


fn test_sampler() {
    info!("Taking 3 samples of dim 2");
    let sampler = RtSampler::new(1, 3, 2);
    let mut it = RtSampleIterator::new(&sampler);
    let mut sample = vec![0.0; 2];
    while RtGetSample(&mut it, &mut sample) {
        let sample_item_0 = sample[0];
        let sample_item_1 = sample[1];
        info!("New sample : ({}, {})", sample_item_0, sample_item_1);
    }
    info!("Samples taken : {}", RtGetSampleCount(&it));
    info!("Weight : {}", RtSamplerGetSampleInvCount(&it));
}

pub fn rt_test() {  
    // test_sampler();
    let path = "/home/alice/Documents/PROJECTS/RaitoRender/Raito/tests/scenes/scene_000.xml";
    
    read_xml(&path);

    panic!("end");
}
