@group(0)
@binding(0)
var<storage, read_write> buffer: array<array<f32, 3>>;

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    buffer[global_id.x] = test(buffer[global_id.x]);
}

fn test(pix: array<f32, 3>) -> array<f32, 3>{

    var r = pix[0];
    var g = pix[1];
    var b = pix[2];
    var r1 = r + 1.32;
    var b1 = b + 8.342342;
    var g1 = g - 0.00032;

    var test: array<f32, 3> = array<f32, 3>(r1, g1, b1);
    return test;

}

// testing func
fn xyz(rgb: array<f32, 3>) -> array<f32, 3> {
    //let rgb = pq(pix);
    let xyz = array<f32, 3>(
            rgb[0] * 0.4124 + rgb[1] * 0.3576 + rgb[2] * 0.1805, //this will be x
            rgb[0] * 0.2126 + rgb[1] * 0.7152 + rgb[2] * 0.0722, //this will be y
            rgb[0] * 0.0193 + rgb[1] * 0.1192 + rgb[2] * 0.9505  //this will be z
    );
    return xyz;
}

//fn eotf_ST2084(x: f32) -> f32 {
//    if x > 0.0 {
//    //adapted from colour science repo
//    var m1 = 2610 / 4096 * (1 / 4);
//    var m2 = 2523 / 4096 * 128;
//    var c1 = 3424 / 4096;
//    var c2 = 2413 / 4096 * 32;
//    var c3 = 2392 / 4096 * 32;
//    var lp = 10000;
//
//    let xpow = powf(x, 1.0 / m2);
//    let num = (xpow - c1).max(0.0);
//    let den = c3.mul_add(-xpow, c2).max(f32::EPSILON);
//    powf(num / den, 1.0 / m1);
//
//    return y - e;
//
//    } else {
//        return 0.0
//    }
//}

//fn pq(rgb: array<f32, 3>) -> array<f32, 3>{
//    var r = eotf_ST2084(rgb[0]);
//    var g = eotf_ST2084(rgb[1]);
//    var b = eotf_ST2084(rgb[2]);
//
//    var test: array<f32, 3> = array<f32, 3>(r, g, b);
//    return test;
//
//}
