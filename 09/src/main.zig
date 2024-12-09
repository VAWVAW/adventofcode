const std = @import("std");

const result_type = u64;
const disk_type = ?u15;

fn task_1(allocator: std.mem.Allocator, file_name: []const u8) !?result_type {
    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // read data
    const raw_data = try input.reader().readAllAlloc(allocator, 1 << 32);
    defer allocator.free(raw_data);
    const data = raw_data[0 .. raw_data.len - 1 :'\n'];

    var disk_size: u32 = 0;
    for (data) |*num| {
        num.* -= '0';
        disk_size += num.*;
    }

    var disk = try allocator.alloc(disk_type, disk_size);
    defer allocator.free(disk);

    var disk_offset: usize = 0;
    for (data, 0..) |num, i| {
        for (0..num) |_| {
            if (i % 2 == 1) {
                disk[disk_offset] = null;
            } else {
                disk[disk_offset] = @intCast(i / 2);
            }

            disk_offset += 1;
        }
    }

    // calculate result
    var offset_left: usize = 0;
    var offset_right: usize = disk.len - 1;

    while (true) {
        while (disk[offset_left] != null) {
            offset_left += 1;
        }
        while (disk[offset_right] == null) {
            offset_right -= 1;
        }

        if (offset_left > offset_right)
            break;

        disk[offset_left] = disk[offset_right];
        disk[offset_right] = null;
    }

    var result: result_type = 0;
    for (disk, 0..) |item, i| {
        if (item) |num| {
            result += num * i;
        }
    }

    return result;
}

fn task_2(allocator: std.mem.Allocator, file_name: []const u8) !?result_type {
    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // read data
    const raw_data = try input.reader().readAllAlloc(allocator, 1 << 32);
    defer allocator.free(raw_data);
    const data = raw_data[0 .. raw_data.len - 1 :'\n'];

    var disk_size: u32 = 0;
    for (data) |*num| {
        num.* -= '0';
        disk_size += num.*;
    }

    var disk = try allocator.alloc(disk_type, disk_size);
    defer allocator.free(disk);

    var disk_offset: usize = 0;
    for (data, 0..) |num, i| {
        for (0..num) |_| {
            if (i % 2 == 1) {
                disk[disk_offset] = null;
            } else {
                disk[disk_offset] = @intCast(i / 2);
            }

            disk_offset += 1;
        }
    }

    // calculate result
    var offset_right: isize = @intCast(disk.len - 1);

    while (offset_right >= 0) : (offset_right -= 1) {
        // find next file
        while (disk[@intCast(offset_right)] == null) {
            offset_right -= 1;
        }
        var len: usize = 0;
        const c = disk[@intCast(offset_right)];

        while (offset_right >= 0 and disk[@intCast(offset_right)] == c) {
            len += 1;
            offset_right -= 1;
        }
        offset_right += 1;

        // find free space
        var offset_left: usize = 0;
        while (offset_left < offset_right) : (offset_left += 1) {
            if (disk[offset_left] == null) {
                var free_len: usize = 1;
                while (disk[offset_left + free_len] == null and free_len < len) : (free_len += 1) {}

                if (len == free_len) {
                    for (0..len) |i| {
                        disk[offset_left + i] = disk[@as(usize, @intCast(offset_right)) + i];
                        disk[@as(usize, @intCast(offset_right)) + i] = null;
                    }
                    break;
                }
            }
        }
    }

    var result: result_type = 0;
    for (disk, 0..) |item, i| {
        if (item) |num| {
            result += num * i;
        }
    }

    return result;
}

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();

    // setup allocator
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    // read arguments
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    // execute
    var result: result_type = undefined;
    if (args.len < 2 or args[1][0] != '2') {
        result = (try task_1(allocator, "data.txt")).?;
    } else {
        result = (try task_2(allocator, "data.txt")).?;
    }

    try stdout.print("{d}\n", .{result});
}

test task_1 {
    try std.testing.expectEqual(1928, try task_1(std.testing.allocator, "data_test1.txt"));
}

test task_2 {
    try std.testing.expectEqual(2858, try task_2(std.testing.allocator, "data_test1.txt"));
}
