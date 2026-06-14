const std = @import("std");

export fn getMaxValue(arr: [*]const u32, len: usize) u32 {
    var max: u32 = arr[0];
    var i: usize = 1;
    while (i < len) : (i += 1) {
        if (arr[i] > max) {
            max = arr[i];
        }
    }
    return max;
}

export fn countingSort(arr: [*]u32, len: usize, exp: u32) void {
    var output: [1000]u32 = undefined;
    var count: [10]u32 = [_]u32{0} ** 10;
    var i: usize = 0;

    while (i < len) : (i += 1) {
        const index = (arr[i] / exp) % 10;
        count[index] += 1;
    }

    i = 1;
    while (i < 10) : (i += 1) {
        count[i] += count[i - 1];
    }

    i = len;
    while (i > 0) {
        i -= 1;
        const index = (arr[i] / exp) % 10;
        output[count[index] - 1] = arr[i];
        count[index] -= 1;
    }

    i = 0;
    while (i < len) : (i += 1) {
        arr[i] = output[i];
    }
}

export fn radixSort(ptr: usize, len: usize) void {
    const arr: [*]u32 = @ptrFromInt(ptr);
    const max = getMaxValue(arr, len);
    var exp: u32 = 1;
    
    while (max / exp > 0) : (exp *= 10) {
        countingSort(arr, len, exp);
    }
}

export fn allocate(size: usize) usize {
    const memory = std.heap.page_allocator.alloc(u8, size) catch unreachable;
    return @intFromPtr(memory.ptr);
}

export fn deallocate(ptr: usize, size: usize) void {
    const byte_ptr: [*]u8 = @ptrFromInt(ptr);
    const slice = byte_ptr[0..size];
    std.heap.page_allocator.free(slice);
}