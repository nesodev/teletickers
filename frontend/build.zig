const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.resolveTargetQuery(.{
        .cpu_arch = .wasm32,
        .os_tag = .freestanding,
    });

    const lib = b.addExecutable(.{
        .name = "radix_sort",
        .root_source_file = b.path("wasm/radix_sort.zig"),
        .target = target,
        .optimize = .ReleaseSmall,
    });

    lib.entry = .disabled;
    lib.rdynamic = true;

    const wasm_file = b.addInstallBinFile(lib.getEmittedBin(), "radix_sort.wasm");
    b.getInstallStep().dependOn(&wasm_file.step);
}