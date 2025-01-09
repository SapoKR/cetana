

#[link(name="rocm")]
extern "C" {
    fn hipMalloc(ptr: *mut *mut std::ffi::c_void, size: usize) -> i32;
    fn hipFree(ptr: *mut std::ffi::c_void) -> i32;
    fn hipMemcpy(
        dst: *mut std::ffi::c_void,
        src: *const std::ffi::c_void,
        sizeBytes: usize,
        kind: hipMemcpyKind,
    ) -> i32;
    fn hipMemset(ptr: *mut std::ffi::c_void, value: i32, sizeBytes: usize) -> i32;
    fn hipDeviceSynchronize() -> i32;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum hipMemcpyKind {
    hipMemcpyHostToHost,
    hipMemcpyHostToDevice,
    hipMemcpyDeviceToHost,
    hipMemcpyDeviceToDevice,
    hipMemcpyDefault,
    hipMemcpyDeviceToDeviceNoCU,
}

const HIP_SUCCESS: i32 = 0;