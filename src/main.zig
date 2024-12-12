const std = @import("std");
const tokenizer = @import("tokenizer/tokenizer.zig");

const allocator = std.heap.page_allocator;

pub fn err(message: []const u8) void {
    std.debug.print("Compilation failed:\n    {s}\n", .{message});
}

pub fn main() !void {
    const argv_count = std.os.argv.len;
    if (argv_count != 2) {
        err("Usage: zircon [file_name]");
        return;
    }

    const target = std.mem.span(std.os.argv[1]);
    const file = std.fs.cwd().openFile(target, .{}) catch {
        err("File not found.");
        return;
    };
    defer file.close();

    const file_size = (try file.stat()).size;

    const buffer = try allocator.alloc(u8, file_size);
    defer allocator.free(buffer);

    _ = try file.readAll(buffer);

    var toks = std.ArrayList(tokenizer.Token).init(allocator);
    defer toks.deinit();

    try tokenizer.scan_tokens(buffer, &toks);
    for (toks.items) |tok| {
        std.debug.print("{any}\n", .{tok});
    }
}
