const std = @import("std");

const result_type = u64;

fn task_1(allocator: std.mem.Allocator, file_name: []const u8) !?result_type {
    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // read data

    var result: result_type = 0;
    var values = std.ArrayList(result_type).init(allocator);
    defer values.deinit();

    var buf: [64]u8 = undefined;
    outer: while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| : (values.clearRetainingCapacity()) {
        var iter = std.mem.tokenizeAny(u8, line, &[_]u8{ ' ', ':' });

        const expected_value: result_type = try std.fmt.parseInt(result_type, iter.next().?, 10);

        while (iter.next()) |next| {
            try values.append(try std.fmt.parseInt(result_type, next, 10));
        }

        var i: usize = 0;
        while (i < (@as(usize, 1) << @as(u6, @intCast(values.items.len - 1)))) : (i += 1) {
            var partial_value: result_type = values.items[0];

            for (values.items[1..], 0..) |next, offset| {
                if ((i & (@as(usize, 1) << @as(u6, @intCast(offset)))) != 0) {
                    partial_value += next;
                } else {
                    partial_value *= next;
                }
            }

            if (partial_value == expected_value) {
                result += @intCast(expected_value);
                continue :outer;
            }
        }
    }

    return result;
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
        result = (try task_1(allocator, "data.txt")).?;
    } else {
        result = (try task_2(allocator, "data.txt")).?;
    }

    try stdout.print("{d}\n", .{result});
}

test task_1 {
    try std.testing.expectEqual(3749, try task_1(std.testing.allocator, "data_test1.txt"));
}

test task_2 {
    // try std.testing.expectEqual(6, try task_2(std.testing.allocator, "data_test2.txt"));
}
