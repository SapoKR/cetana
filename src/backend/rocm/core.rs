#[link(name = "rocm")]
extern "C" {
    fn hipGetDeviceProperties(prop: *mut rocmDeviceProp, device: i32) -> i32;
    fn hipDeviceGetName(name: *mut i8, len: i32, device: i32) -> i32;
    fn hipDeviceGetAttribute(value: *mut i32, attr: i32, device: i32) -> i32;
    fn hipMemGetInfo(free: *mut usize, total: *mut usize) -> i32;
    fn hipSetDevice(device: i32) -> i32;
    fn hipDeviceSynchronize() -> i32;
    fn hipGetDeviceCount(count: *mut i32) -> i32;
}

/*struct hipUUID {
    bytes: [i8, 16]
}*/

#[repr(C)]
struct rocmDeviceProp {
    name: [i8; 256],
    //uuid: hipUUID,
    luid: [i8; 8],
    //luidDeviceNodeMask ?
    totalGlobalMem: usize,
    sharedMemPerBlock: usize,
    regsPerBlock: i32,
    warpSize: i32,
    memPitch: usize,
    maxThreadsPerBlock: i32,
    maxThreadsDim: [i32; 3],
    maxGridSize: [i32; 3],
    clockRate: i32,
    totalConstMem: usize,
    major: i32,
    minor: i32,
    // ...
}

const HIP_SUCCESS: i32 = 0;
const HIP_DEVICE_ATTR_COMPUTE_CAPBILITY_MAJOR: i32 = 25
const HIP_DEVICE_ATTR_COMPUTE_CAPBILITY_MINOR: i32 = 63

impl RocmDevice {
    //pub fn new(device_id: i32) -> Result<Self,
}
