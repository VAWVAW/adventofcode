const std = @import("std");

const result_type = i32;
fn task_1(allocator: std.mem.Allocator, file_name: []const u8, steps: result_type, width: result_type, heigth: result_type) !?result_type {
    _ = allocator;
    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // read data
    var ends = [_]result_type{0} ** 4;

    var buf: [32]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var token_iter = std.mem.tokenizeAny(u8, line, "p=, v\n");

        const x_start = try std.fmt.parseInt(result_type, token_iter.next().?, 10);
        const y_start = try std.fmt.parseInt(result_type, token_iter.next().?, 10);
        const x_diff = try std.fmt.parseInt(result_type, token_iter.next().?, 10);
        const y_diff = try std.fmt.parseInt(result_type, token_iter.next().?, 10);

        const x_end = @mod(x_start + steps * x_diff, width);
        const y_end = @mod(y_start + steps * y_diff, heigth);

        if (x_end == @divTrunc(width, 2) or y_end == @divTrunc(heigth, 2)) {
            continue;
        }

        var idx: usize = 0;
        if (x_end > @divTrunc(width, 2)) {
            idx += 1;
        }
        if (y_end > @divTrunc(heigth, 2)) {
            idx += 2;
        }
        ends[idx] += 1;
    }

    return ends[0] * ends[1] * ends[2] * ends[3];
}

fn task_2(allocator: std.mem.Allocator, file_name: []const u8) !?result_type {
    _ = allocator;
    _ = file_name;
    return error.NotImplemented;
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
        result = (try task_1(allocator, "data.txt", 100, 101, 103)).?;
    } else {
        result = (try task_2(allocator, "data.txt")).?;
    }

    try stdout.print("{d}\n", .{result});
}

test task_1 {
    try std.testing.expectEqual(12, try task_1(std.testing.allocator, "data_test1.txt", 100, 11, 7));
}

test task_2 {
    // try std.testing.expectEqual(80, try task_2(std.testing.allocator, "data_test1.txt"));
}
