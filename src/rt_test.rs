/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Simple module where I can test new features
///   easily.
/// =====================================================

use log::info;

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

fn test_xml() {
    let path = "/home/alice/Documents/PROJECTS/RaitoRender/Raito/tests/scenes/scene_000.xml";
    open_xml_scene(path);
}

fn test_buckets() {
    let path = "/datas/sonoleta/github/Raito/tests/scenes/scene_000.xml";
    let scene = open_xml_scene(path);
    if scene.is_none() {
        panic!("Test failed");
    }
    let scene = scene.unwrap();
    let mut result = RtRenderResult::new(RT_DEFAULT_WINDOW_WIDTH, RT_DEFAULT_WINDOW_HEIGHT);

    let now = std::time::Instant::now();
    RtRenderScene(scene, &mut result);
    info!("Render finished in {} sec", now.elapsed().as_secs_f64());
}


pub fn rt_test() {  
    // test_sampler();
    // test_xml();
    test_buckets()
}
