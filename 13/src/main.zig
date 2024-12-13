const std = @import("std");

const result_type = isize;

fn task(allocator: std.mem.Allocator, file_name: []const u8, prize_offset: result_type) !?result_type {
    _ = allocator;

    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // read data
    var result: result_type = 0;
    var buf: [32]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |a_line| {
        var a_iter = std.mem.tokenizeAny(u8, a_line["Button A: X+".len..a_line.len], ", Y+");
        const a_diff_x = try std.fmt.parseInt(result_type, a_iter.next().?, 10);
        const a_diff_y = try std.fmt.parseInt(result_type, a_iter.next().?, 10);

        const b_line = (try input.reader().readUntilDelimiterOrEof(&buf, '\n')).?;
        var b_iter = std.mem.tokenizeAny(u8, b_line["Button B: X+".len..b_line.len], ", Y+");
        const b_diff_x = try std.fmt.parseInt(result_type, b_iter.next().?, 10);
        const b_diff_y = try std.fmt.parseInt(result_type, b_iter.next().?, 10);

        const prize_line = (try input.reader().readUntilDelimiterOrEof(&buf, '\n')).?;
        var prize_iter = std.mem.tokenizeAny(u8, prize_line["Prize: X=".len..prize_line.len], ", Y=");
        const prize_x = try std.fmt.parseInt(result_type, prize_iter.next().?, 10) + prize_offset;
        const prize_y = try std.fmt.parseInt(result_type, prize_iter.next().?, 10) + prize_offset;

        // ignore next line
        _ = input.reader().readUntilDelimiterOrEof(&buf, '\n') catch {};

        const a: result_type = std.math.divExact(result_type, prize_y * b_diff_x - prize_x * b_diff_y, a_diff_y * b_diff_x - a_diff_x * b_diff_y) catch continue;

        const b: result_type = std.math.divExact(result_type, prize_x - a_diff_x * a, b_diff_x) catch continue;

        try std.testing.expectEqual(prize_x, a * a_diff_x + b * b_diff_x);
        try std.testing.expectEqual(prize_y, a * a_diff_y + b * b_diff_y);

        result += 3 * a + b;
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
        result = (try task(allocator, "data.txt", 0)).?;
    } else {
        result = (try task(allocator, "data.txt", 10000000000000)).?;
    }

    try stdout.print("{d}\n", .{result});
}

test task {
    try std.testing.expectEqual(480, try task(std.testing.allocator, "data_test1.txt", 0));
}
