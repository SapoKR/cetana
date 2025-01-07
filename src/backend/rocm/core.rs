#[repr(C)]
struct rocmDeviceProp {
    name: [i8; 256],
    //uuid: hipUUID ?
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