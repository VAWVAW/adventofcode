const std = @import("std");

const result_type = reg_type;

const reg_type = u64;
const A = 0;
const B = 1;
const C = 2;

fn getCombo(regs: [3]reg_type, operand: u8) reg_type {
    switch (operand) {
        '0'...'3' => return @intCast(operand - '0'),
        '4'...'6' => return regs[@intCast(operand - '4')],
        else => unreachable,
    }
}

const Action = union(enum) {
    output: reg_type,
    jump: usize,
    none: void,
};
fn executeInstruction(regs: *[3]reg_type, opcode: u8, operand: u8) Action {
    switch (opcode) {
        '0' => {
            // adv
            const denominator = std.math.pow(reg_type, 2, getCombo(regs.*, operand));
            regs[A] = @divTrunc(regs[A], denominator);
        },
        '1' => {
            // bxl
            regs[B] ^= operand - '0';
        },
        '2' => {
            // bst
            const op = getCombo(regs.*, operand);
            regs[B] = op & 0b111;
        },
        '3' => {
            // jnz
            if (regs[A] != 0) {
                return Action{ .jump = (operand - '0') };
            }
        },
        '4' => {
            // bxc
            regs[B] ^= regs[C];
        },
        '5' => {
            // out
            const op = getCombo(regs.*, operand);
            return Action{ .output = op & 0b111 };
        },
        '6' => {
            // bdv
            const denominator = std.math.pow(reg_type, 2, getCombo(regs.*, operand));
            regs[B] = @divTrunc(regs[A], denominator);
        },
        '7' => {
            // cdv
            const denominator = std.math.pow(reg_type, 2, getCombo(regs.*, operand));
            regs[C] = @divTrunc(regs[A], denominator);
        },
        else => unreachable,
    }
    return Action{ .none = {} };
}

test executeInstruction {
    var regs: [3]reg_type = .{ 0, 0, 0 };

    regs[C] = 9;
    try std.testing.expectEqual(Action{ .none = {} }, executeInstruction(&regs, '2', '6'));
    try std.testing.expectEqual(1, regs[B]);

    regs[B] = 29;
    try std.testing.expectEqual(Action{ .none = {} }, executeInstruction(&regs, '1', '7'));
    try std.testing.expectEqual(26, regs[B]);

    regs[B] = 2024;
    regs[C] = 43690;
    try std.testing.expectEqual(Action{ .none = {} }, executeInstruction(&regs, '4', '0'));
    try std.testing.expectEqual(44354, regs[B]);
}

fn execute(regs: *[3]reg_type, program: []const u8, writer: anytype) !void {
    var ip: usize = 0;
    while (ip < program.len) {
        const opcode = program[ip];
        const operand = program[ip + 2];

        switch (executeInstruction(regs, opcode, operand)) {
            Action.jump => |new_ip| {
                ip = new_ip * 2;
                continue;
            },
            Action.output => |out| {
                try std.fmt.formatInt(out, 10, .lower, .{}, writer);
                try writer.writeByte(',');
            },
            Action.none => {},
        }

        ip += 4;
    }
}

test execute {
    var regs: [3]reg_type = .{ 0, 0, 0 };
    var output = std.ArrayList(u8).init(std.testing.allocator);
    defer output.deinit();

    regs[A] = 10;
    try execute(&regs, "5,0,5,1,5,4", output.writer());
    try std.testing.expectEqualSlices(u8, "0,1,2,", output.items);
    output.clearRetainingCapacity();

    regs[A] = 2024;
    try execute(&regs, "0,1,5,4,3,0", output.writer());
    try std.testing.expectEqualSlices(u8, "4,2,5,6,7,7,7,7,3,1,0,", output.items);
    try std.testing.expectEqual(regs[A], 0);
    output.clearRetainingCapacity();
}

fn task_1(allocator: std.mem.Allocator, file_name: []const u8) !?std.ArrayList(u8) {
    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // read data
    var regs: [3]reg_type = undefined;

    var buf: [32]u8 = undefined;
    var i: usize = 0;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        if (line.len == 0)
            break;

        const val = try std.fmt.parseInt(reg_type, line["Register A: ".len..line.len], 10);
        regs[i] = val;
        i += 1;
    }

    // skip to program
    _ = try input.reader().read(buf[0.."Program: ".len]);

    const program = try input.reader().readAllAlloc(allocator, 1 << 16);
    defer allocator.free(program);

    // execute program
    var result = std.ArrayList(u8).init(allocator);
    errdefer result.deinit();

    try execute(&regs, program, result.writer());

    _ = result.pop();

    return result;
}

fn execute2(regs: *[3]reg_type, program: []const u8, expected: []const u8) !bool {
    var offset: usize = 0;

    var ip: usize = 0;
    while (ip < program.len) {
        const opcode = program[ip];
        const operand = program[ip + 2];

        switch (executeInstruction(regs, opcode, operand)) {
            Action.jump => |new_ip| {
                ip = new_ip * 2;
                continue;
            },
            Action.output => |out| {
                if (out + '0' != expected[offset]) {
                    return false;
                }
                offset += 2;
            },
            Action.none => {},
        }

        ip += 4;
    }

    return offset >= expected.len;
}

fn task_2(allocator: std.mem.Allocator, file_name: []const u8) !?result_type {
    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // read data
    var regs: [3]reg_type = undefined;

    var buf: [32]u8 = undefined;
    var i: usize = 0;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        if (line.len == 0)
            break;

        const val = try std.fmt.parseInt(reg_type, line["Register A: ".len..line.len], 10);
        regs[i] = val;
        i += 1;
    }

    // skip to program
    _ = try input.reader().read(buf[0.."Program: ".len]);

    const program = try input.reader().readAllAlloc(allocator, 1 << 16);
    defer allocator.free(program);

    // execute program
    var possible = std.AutoArrayHashMap(reg_type, void).init(allocator);
    defer possible.deinit();
    var old_possible = std.AutoArrayHashMap(reg_type, void).init(allocator);
    defer old_possible.deinit();

    try possible.put(0, {});

    for (0..(program.len) / 2) |match_len| {
        std.mem.swap(std.AutoArrayHashMap(reg_type, void), &possible, &old_possible);
        possible.clearRetainingCapacity();
        const expected = program[(program.len - 2 * (match_len + 1))..program.len];

        for (old_possible.keys()) |old_a| {
            for (0..8) |next_a| {
                const cur_a = (old_a << 3) + @as(reg_type, @intCast(next_a));

                var current_regs = regs;
                current_regs[A] = cur_a;

                if (try execute2(&current_regs, program, expected)) {
                    try possible.put(cur_a, {});
                }
            }
        }
    }

    var min = possible.keys()[0];
    for (possible.keys()) |k| {
        min = @min(min, k);
    }
    return min;
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
    if (args.len < 2 or args[1][0] != '2') {
        var result: std.ArrayList(u8) = undefined;
        result = (try task_1(allocator, "data.txt")).?;
        defer result.deinit();

        try stdout.print("{s}\n", .{result.items});
    } else {
        var result: result_type = undefined;
        result = (try task_2(allocator, "data.txt")).?;
        try stdout.print("{d}\n", .{result});
    }
}

test task_1 {
    const result = (try task_1(std.testing.allocator, "data_test1.txt")).?;
    defer result.deinit();

    try std.testing.expectEqualSlices(u8, "4,6,3,5,6,3,5,2,1,0", result.items);

    const result2 = (try task_1(std.testing.allocator, "data_test2.txt")).?;
    defer result2.deinit();

    try std.testing.expectEqualSlices(u8, "0,3,5,4,3,0", result2.items);
}

test task_2 {
    try std.testing.expectEqual(117440, try task_2(std.testing.allocator, "data_test2.txt"));
}
